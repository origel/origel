ARCH?=x86_64

# Kernel variables
KTARGET=$(ARCH)-unknown-none
KBUILD=build/kernel
KRUSTC=./krustc.sh
KRUSTCFLAGS=--target $(KTARGET).json -C opt-level=2 -C debuginfo=0 -C soft-float
KRUSTDOC=./krustdoc.sh
KCARGO=RUSTC="$(KRUSTC)" RUSTDOC="$(KRUSTDOC)" cargo
KCARGOFLAGS=--target $(KTARGET).json --release -- -C soft-float

# Default targets
.PHONY: all

all:$(KBUILD)/kernel

clean:
	rm -rf build	

# Kernel recipes
$(KBUILD)/libcore.rlib: rust/src/libcore/lib.rs
	mkdir -p $(KBUILD)
	$(KRUSTC) $(KRUSTCFLAGS) -o $@ $<

$(KBUILD)/librand.rlib: rust/src/librand/lib.rs $(KBUILD)/libcore.rlib
	$(KRUSTC) $(KRUSTCFLAGS) -o $@ $<

$(KBUILD)/liballoc.rlib: rust/src/liballoc/lib.rs $(KBUILD)/libcore.rlib
	$(KRUSTC) $(KRUSTCFLAGS) -o $@ $<

$(KBUILD)/librustc_unicode.rlib: rust/src/librustc_unicode/lib.rs $(KBUILD)/libcore.rlib
	$(KRUSTC) $(KRUSTCFLAGS) -o $@ $<

$(KBUILD)/libcollections.rlib: rust/src/libcollections/lib.rs $(KBUILD)/libcore.rlib $(KBUILD)/liballoc.rlib $(KBUILD)/librustc_unicode.rlib
	$(KRUSTC) $(KRUSTCFLAGS) -o $@ $<

$(KBUILD)/librustc_bitflags.rlib: rust/src/librustc_bitflags/lib.rs $(KBUILD)/libcore.rlib $(KBUILD)/liballoc.rlib $(KBUILD)/librustc_unicode.rlib
	$(KRUSTC) $(KRUSTCFLAGS) -o $@ $<

$(KBUILD)/libkernel.a: kernel/** $(KBUILD)/libcore.rlib $(KBUILD)/liballoc.rlib $(KBUILD)/libcollections.rlib initfs/src/initfs.rs
	$(KCARGO) rustc $(KCARGOFLAGS) -C lto -o $@

$(KBUILD)/kernel: $(KBUILD)/libkernel.a
	$(LD) $(LDFLAGS) -z max-page-size=0x1000 -T arch/$(ARCH)/src/linker.ld -o $@ $<

kernel: $(KBUILD)/kernel