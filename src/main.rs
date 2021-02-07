#[macro_use]
extern crate clap;

use procfs::process::{Process, FDTarget};
use std::fs;

fn main() {
    let yaml = load_yaml!("option.yml");
    let args = clap::App::from_yaml(yaml).get_matches();

    let pid = args.value_of("pid").unwrap().parse::<i32>().unwrap_or_else(|e| {
        eprintln!("pid parse error: {}", e);
        std::process::exit(1);
    });

    let proc = Process::new(pid).unwrap().fd().unwrap();
    let pathes = proc.iter().map(|i| &i.target).filter_map(|t| {
        match t { 
            FDTarget::Path(path) => Some(path),
            _ => None,
        }
    }).filter(|path| {
        if args.is_present("regular") {
            path.is_file()
        } else {
            true
        }
    }).collect::<Vec<_>>();

    for path in pathes {
        if args.is_present("copy") {
            if path.as_path().is_file() {
                match fs::copy(path, path.file_name().unwrap().to_str().unwrap()) {
                    Ok(_) => println!("copied! {} to {}", path.to_str().unwrap(), path.file_name().unwrap().to_str().unwrap()),
                    Err(e) => eprintln!("Error when copying {}: {}", path.to_str().unwrap(), e),
                }
            } else {
                println!("{} is not an existing regular file", path.to_str().unwrap());
            }
        } else {
            println!("{}", path.as_path().to_str().unwrap());
        }
    }
}
