#![deny(warnings)]

extern crate extra;

use std::env::args;
use std::io::{self, Write};
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use extra::option::OptionalExt;

static LONG_HELP: &'static str = /* @MANSTART{rem} */ r#"
NAME
    rem - set a count-down.

SYNOPSIS
    rem [-h | --help] [-m N | --minutes N] [-H N | --hours N] [-s N | --seconds N]
        [-M N | --milliseconds N] [-n | --len] [-b | --blink]

DESCRIPTION
    This utility lets you set a count-down with a progress bar. The options can be given in
    combination, adding together the durations given.

OPTIONS
    --help
        Print this manual page.

    -h
        Print short help page.

    -m N
    --minutes N
        Wait N minutes.

    -H N
    --hours N
        Wait N hours.

    -s N
    --seconds N
        Wait N seconds.

    -M N
    --milliseconds N
        Wait N milliseconds.

    -n N
    --len N
        Set the length of the progress bar to N.

    -b
    --blink
        Blink with a red banner when done.

AUTHOR
    This program was written by Ticki for Redox OS. Bugs, issues, or feature requests should be
    reported in the Github repository, 'redox-os/extrautils'.

COPYRIGHT
    Copyright (c) 2016 Ticki

    Permission is hereby granted, free of charge, to any person obtaining a copy of this software
    and associated documentation files (the "Software"), to deal in the Software without
    restriction, including without limitation the rights to use, copy, modify, merge, publish,
    distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
    Software is furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all copies or
    substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
    BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
    NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
    DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
"#; /* @MANEND */

static SHORT_HELP: &'static str = r#"
    rem - set a count-down.

    Options (use --help for extended list):
    -m N          => Wait N minutes.
    -H N          => Wait N hours.
    -s N          => Wait N seconds.
    -M N          => Wait N milliseconds.
    -n N          => N character progress bar.
    -b            => Blink when done.
"#;

fn main() {
    let mut args = args().skip(1);
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut stderr = io::stderr();

    let mut ms = 0u64;
    let mut len = 20;
    let mut blink = false;

    // Loop over the arguments.
    loop {
        let arg = if let Some(x) = args.next() {
            x
        } else {
            break;
        };

        match arg.as_str() {
            "--help" => {
                // Print help.
                stdout.write(LONG_HELP.as_bytes()).try(&mut stderr);
                return;
            },
            "-h" => {
                // Print help.
                stdout.write(SHORT_HELP.as_bytes()).try(&mut stderr);
                return;
            },
            "-n" | "--len" => len = args.next().fail("no number after -n.", &mut stderr).parse().try(&mut stderr),
            "-b" | "--blink" => blink = true,
            t => {
                // Find number input.
                let num: u64 = args.next().unwrap_or_else(|| {
                    stderr.write(b"error: incorrectly formatted number. \
                                   Please input a positive integer.\n").try(&mut stderr);
                    stderr.flush().try(&mut stderr);
                    exit(1);
                }).parse().try(&mut stderr);
                ms += num * match t {
                    "-m" | "--minutes" => 1000 * 60,
                    "-H" | "--hours" => 1000 * 60 * 60,
                    "-s" | "--seconds" => 1000,
                    "-M" | "--milliseconds" => 1,
                    _ => {
                        // Unknown argument.
                        stderr.write(b"error: unknown argument, ").try(&mut stderr);
                        stderr.write(t.as_bytes()).try(&mut stderr);
                        stderr.write(b".\n").try(&mut stderr);
                        stderr.flush().try(&mut stderr);
                        exit(1);
                    },

                };
            },
        }
    }

    // Default to help page.
    if ms == 0 {
        stdout.write(SHORT_HELP.as_bytes()).try(&mut stderr);
        return;
    }

    // Hide the cursor.
    stdout.write(b"\x1b[?25l").try(&mut stderr);
    // Draw the empty progress bar.
    for _ in 0..len + 1 {
        stdout.write(b" ").try(&mut stderr);
    }
    stdout.write(b"]").try(&mut stderr);

    stdout.write(b"\r[").try(&mut stderr);

    // As time goes, update the progress bar.
    for _ in 0..len {
        stdout.write(b"#").try(&mut stderr);
        stdout.flush().try(&mut stderr);
        // Sleep.
        sleep(Duration::from_millis(ms / len));
    }


    if blink {
        // This will print a blinking red banner.
        for _ in 0..13 {
            // Set drawing mode to red background.
            stdout.write(b"\x1b[41m").try(&mut stderr);
            // Clear the current line, rendering the background red.
            stdout.write(b"\x1b[2K").try(&mut stderr);
            // Flush.
            stdout.flush().try(&mut stderr);
            // Sleep.
            sleep(Duration::from_millis(200));

            // Clear the drawing mode.
            stdout.write(b"\x1b[0m").try(&mut stderr);
            // Clear the background.
            stdout.write(b"\x1b[2K").try(&mut stderr);
            // Flush.
            stdout.flush().try(&mut stderr);
            sleep(Duration::from_millis(200));

            // Repeat...
        }
    }

    // Show the cursor again.
    stdout.write(b"\x1b[?25h").try(&mut stderr);
    stdout.write(b"\n").try(&mut stderr);
}
