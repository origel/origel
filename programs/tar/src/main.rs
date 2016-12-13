#![deny(warnings)]

extern crate tar;

use std::{env, process};
use std::io::{stdin, stdout, stderr, copy, Result, Read, Write};
use std::fs::{self, File};
use std::os::unix::fs::OpenOptionsExt;

use tar::{Archive, Builder, EntryType};

fn create_inner<T: Write>(input: &str, ar: &mut Builder<T>) -> Result<()> {
    if try!(fs::metadata(input)).is_dir() {
        for entry_result in try!(fs::read_dir(input)) {
            let entry = try!(entry_result);
            if try!(fs::metadata(entry.path())).is_dir() {
                try!(create_inner(entry.path().to_str().unwrap(), ar));
            } else {
                println!("{}", entry.path().display());
                try!(ar.append_path(entry.path()));
            }
        }
    } else {
        println!("{}", input);
        try!(ar.append_path(input));
    }

    Ok(())
}

fn create(input: &str, tar: &str) -> Result<()> {
    if tar == "-" {
        create_inner(input, &mut Builder::new(stdout()))
    } else {
        create_inner(input, &mut Builder::new(try!(File::create(tar))))
    }
}

fn list_inner<T: Read>(ar: &mut Archive<T>) -> Result<()> {
    for entry_result in try!(ar.entries()) {
        let entry = try!(entry_result);
        let path = try!(entry.path());
        println!("{}", path.display());
    }

    Ok(())
}

fn list(tar: &str) -> Result<()> {
    if tar == "-" {
        list_inner(&mut Archive::new(stdin()))
    } else {
        list_inner(&mut Archive::new(try!(File::open(tar))))
    }
}

fn extract_inner<T: Read>(ar: &mut Archive<T>) -> Result<()> {
    for entry_result in try!(ar.entries()) {
        let mut entry = try!(entry_result);
        match entry.header().entry_type() {
            EntryType::Regular => {
                let mut file = {
                    let path = try!(entry.path());
                    if let Some(parent) = path.parent() {
                        try!(fs::create_dir_all(parent));
                    }
                    try!(
                        fs::OpenOptions::new()
                            .read(true)
                            .write(true)
                            .truncate(true)
                            .create(true)
                            .mode(entry.header().mode().unwrap_or(644))
                            .open(path)
                    )
                };
                try!(copy(&mut entry, &mut file));
            },
            EntryType::Directory => {
                try!(fs::create_dir_all(try!(entry.path())));
            },
            other => {
                panic!("Unsupported entry type {:?}", other);
            }
        }
    }

    Ok(())
}

fn extract(tar: &str) -> Result<()> {
    if tar == "-" {
        extract_inner(&mut Archive::new(stdin()))
    } else {
        extract_inner(&mut Archive::new(try!(File::open(tar))))
    }
}

fn main() {
    let mut args = env::args().skip(1);
    if let Some(op) = args.next() {
        match op.as_str() {
            "c" => if let Some(input) = args.next() {
                if let Err(err) = create(&input, "-") {
                    write!(stderr(), "tar: create: failed: {}\n", err).unwrap();
                    process::exit(1);
                }
            } else {
                write!(stderr(), "tar: create: no input specified: {}\n", op).unwrap();
                process::exit(1);
            },
            "cf" => if let Some(tar) = args.next() {
                if let Some(input) = args.next() {
                    if let Err(err) = create(&input, &tar) {
                        write!(stderr(), "tar: create: failed: {}\n", err).unwrap();
                        process::exit(1);
                    }
                } else {
                    write!(stderr(), "tar: create: no input specified: {}\n", op).unwrap();
                    process::exit(1);
                }
            } else {
                write!(stderr(), "tar: create: no tarfile specified: {}\n", op).unwrap();
                process::exit(1);
            },
            "t" | "tf" => {
                let tar = args.next().unwrap_or("-".to_string());
                if let Err(err) = list(&tar) {
                    write!(stderr(), "tar: list: failed: {}\n", err).unwrap();
                    process::exit(1);
                }
            },
            "x" | "xf" => {
                let tar = args.next().unwrap_or("-".to_string());
                if let Err(err) = extract(&tar) {
                    write!(stderr(), "tar: extract: failed: {}\n", err).unwrap();
                    process::exit(1);
                }
            },
            _ => {
                write!(stderr(), "tar: {}: unknown operation\n", op).unwrap();
                write!(stderr(), "tar: need to specify c[f] (create), t[f] (list), or x[f] (extract)\n").unwrap();
                process::exit(1);
            }
        }
    } else {
        write!(stderr(), "tar: no operation\n").unwrap();
        write!(stderr(), "tar: need to specify cf (create), tf (list), or xf (extract)\n").unwrap();
        process::exit(1);
    }
}
