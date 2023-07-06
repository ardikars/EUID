use std::{env, str::FromStr};

use euid::EUID;

fn help() {
    println!("Usage: euid_toys [COMMAND] [ARG1] [ARG2]");
    println!("Examples:");
    println!("  euid_toys create");
    println!("  euid_toys create_with_extension 123");
    println!("  euid_toys create_batch 10:");
    println!("  euid_toys create_with_extension_batch 123 10:");
    println!("  euid_toys from 123897324");
    println!("  euid_toys from_string C90FS3R3Z3J80V6BBZF0NM7SRKV");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 || args.len() == 1 {
        help();
    } else if args.len() == 2 {
        if "create" == args[1] {
            println!("{}", EUID::create().unwrap_or_default());
        } else {
            help();
        }
    } else if args.len() == 3 {
        let cmd = args[1].as_str().clone();
        match cmd {
            "create_with_extension" => {
                let ext_str = args[2].as_str().clone();
                let ext = u16::from_str_radix(ext_str, 10);
                match ext {
                    Ok(n) => {
                        if n > 32767 {
                            println!("{}", "extension overflow");
                        } else {
                            println!("{}", EUID::create_with_extension(n).unwrap_or_default());
                        }
                    }
                    Err(err) => {
                        println!("{:?}", err);
                    }
                }
            }
            "create_batch" => {
                let n_str = args[2].as_str().clone();
                let n = u16::from_str_radix(n_str, 10);
                match n {
                    Ok(v) => {
                        let mut euid: EUID = EUID::create().unwrap_or_default();
                        println!("{}", euid);
                        for _ in 1..v {
                            euid = euid.next().unwrap_or_default();
                            println!("{}", euid);
                        }
                    }
                    Err(err) => {
                        println!("{:?}", err);
                    }
                }
            }
            "from_string" => {
                let encoded = args[2].as_str().clone();
                let euid = EUID::from_str(encoded);
                match euid {
                    Ok(v) => {
                        println!("{}", v);
                    }
                    Err(err) => {
                        println!("{:?}", err);
                    }
                }
            }
            "from" => {
                let encoded = args[2].as_str().clone();
                let n = u128::from_str_radix(encoded, 10);
                match n {
                    Ok(euid) => {
                        let v: EUID = EUID::from(euid);
                        println!("{}", v);
                    }
                    Err(err) => {
                        println!("{:?}", err);
                    }
                }
            }
            _ => help(),
        };
    } else if args.len() == 4 {
        if "create_with_extension_batch" == args[1] {
            let n_str = args[3].as_str().clone();
            let n = u16::from_str_radix(n_str, 10);
            match n {
                Ok(v) => {
                    let ext_str = args[2].as_str().clone();
                    let ext = u16::from_str_radix(ext_str, 10);
                    match ext {
                        Ok(n) => {
                            if n > 32767 {
                                println!("{}", "extension overflow");
                            } else {
                                let mut euid: EUID =
                                    EUID::create_with_extension(n).unwrap_or_default();
                                println!("{}", euid);
                                for _ in 1..v {
                                    euid = euid.next().unwrap_or_default();
                                    println!("{}", euid);
                                }
                            }
                        }
                        Err(err) => {
                            println!("{:?}", err);
                        }
                    }
                }
                Err(err) => {
                    println!("{:?}", err);
                }
            }
        } else {
            help();
        }
    } else {
        help();
    }
}
