#![deny(warnings)]

extern crate extra;

use std::env::args;
use std::io::{self, Write, Read};

use extra::option::OptionalExt;
use extra::io::fail;

static HELP: &'static str = /* @MANSTART{cur} */ r#"
NAME
    cur - freely move you cursor using H, J, K, and L (Vi-bindings).

SYNOPSIS
    cur [-h | --help]

DESCRIPTION
    This utility will let you navigate the terminal cursor using standard Vi bindings (h, j, k, and
    l) by using ANSI escape codes.

    In combination with other tools, this can be used as a very simple pager.

OPTIONS
    -h
    --help
        Print this manual page.

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

fn csi<T: Write>(stdout: &mut T, other: &str) {
    // Print CSI (control sequence introducer)
    if let Ok(2) = stdout.write("\x1b[".as_bytes()) {
        let _ = stdout.write(other.as_bytes());
    }
}

fn main() {
    let args = args().skip(1);
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut stderr = io::stderr();

    for i in args {
        match i.as_str() {
            // Print the help page.
            "-h" | "--help" => {
                stdout.write(HELP.as_bytes()).try(&mut stderr);
            },
            // This argument is unknown.
            _ => fail("unknown argument.", &mut stderr),
        }
    }

    loop {
        // We read one byte at a time from stdin.
        let mut input = [0];
        let _ = stdin.read(&mut input);

        // Output the right escape code to stdout.
        match input[0] {
            b'k' => csi(&mut stdout, "A"),
            b'j' => csi(&mut stdout, "B"),
            b'l' => csi(&mut stdout, "C"),
            b'h' => csi(&mut stdout, "D"),
            b'q' => break,
            _ => {},
        }

        // Flush it.
        let _ = stdout.flush();
    }
}
