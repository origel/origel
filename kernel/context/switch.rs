use core::sync::atomic::Ordering;

use arch;
use context::{contexts, Context, Status, CONTEXT_ID};
use syscall;

/// Switch to the next context
///
/// # Safety
///
/// Do not call this while holding locks!
pub unsafe fn switch() -> bool {
    use core::ops::DerefMut;

    // Set the global lock to avoid the unsafe operations below from causing issues
    while arch::context::CONTEXT_SWITCH_LOCK.compare_and_swap(false, true, Ordering::SeqCst) {
        arch::interrupt::pause();
    }

    let cpu_id = ::cpu_id();

    let from_ptr;
    let mut to_ptr = 0 as *mut Context;
    let mut to_sig = None;
    {
        let contexts = contexts();
        {
            let context_lock = contexts.current().expect("context::switch: not inside of context");
            let mut context = context_lock.write();
            from_ptr = context.deref_mut() as *mut Context;
        }

        let check_context = |context: &mut Context| -> bool {
            if context.cpu_id == None && cpu_id == 0 {
                context.cpu_id = Some(cpu_id);
                // println!("{}: take {} {}", cpu_id, context.id, ::core::str::from_utf8_unchecked(&context.name.lock()));
            }

            if context.status == Status::Blocked && ! context.pending.is_empty() {
                context.unblock();
            }

            if context.status == Status::Blocked && context.wake.is_some() {
                let wake = context.wake.expect("context::switch: wake not set");

                let current = arch::time::monotonic();
                if current.0 > wake.0 || (current.0 == wake.0 && current.1 >= wake.1) {
                    context.unblock();
                }
            }

            if context.cpu_id == Some(cpu_id) {
                if context.status == Status::Runnable && ! context.running {
                    return true;
                }
            }

            false
        };

        for (pid, context_lock) in contexts.iter() {
            if *pid > (*from_ptr).id {
                let mut context = context_lock.write();
                if check_context(&mut context) {
                    to_ptr = context.deref_mut() as *mut Context;
                    to_sig = context.pending.pop_front();
                    break;
                }
            }
        }

        if to_ptr as usize == 0 {
            for (pid, context_lock) in contexts.iter() {
                if *pid < (*from_ptr).id {
                    let mut context = context_lock.write();
                    if check_context(&mut context) {
                        to_ptr = context.deref_mut() as *mut Context;
                        to_sig = context.pending.pop_front();
                        break;
                    }
                }
            }
        }
    };

    if to_ptr as usize == 0 {
        // Unset global lock if no context found
        arch::context::CONTEXT_SWITCH_LOCK.store(false, Ordering::SeqCst);
        return false;
    }

    (&mut *from_ptr).running = false;
    (&mut *to_ptr).running = true;
    if let Some(ref stack) = (*to_ptr).kstack {
        arch::gdt::TSS.rsp[0] = (stack.as_ptr() as usize + stack.len() - 256) as u64;
    }
    CONTEXT_ID.store((&mut *to_ptr).id, Ordering::SeqCst);

    // Unset global lock before switch, as arch is only usable by the current CPU at this time
    arch::context::CONTEXT_SWITCH_LOCK.store(false, Ordering::SeqCst);

    if let Some(sig) = to_sig {
        println!("Handle {}", sig);
        (&mut *to_ptr).arch.signal_stack(signal_handler, sig);
    }

    (&mut *from_ptr).arch.switch_to(&mut (&mut *to_ptr).arch);

    true
}

extern fn signal_handler(signal: usize) {
    println!("Signal handler: {}", signal);
    syscall::exit(signal);
}
