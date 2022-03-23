mod generators;
mod types;
mod validator;

use std::env::{args, current_dir};
use std::fs::{copy, create_dir_all, write, File};
use std::process::{Command, exit, Stdio};

use generators::generate;
use serde_yaml::from_reader;
use types::SyscallsDef;
use uuid::Uuid;
use crate::validator::validate;

fn main() {
    let path = args().skip(1).next().unwrap_or("sys.yaml".to_string());
    let x = File::open(&path).unwrap();
    let result: SyscallsDef = from_reader(x).unwrap();

    if let Err(errors) = validate(&result) {
        for error in errors.iter() {
            eprintln!("[ERROR] {error}");
        }

        eprintln!("[INFO] Aborting due to {} previous errors", errors.len());
        exit(1);
    }

    let (rs, js, ts) = generate(result);

    println!("--------- Writing outputs --------");

    let mut cwd = current_dir().unwrap();
    cwd.push("./generated");
    create_dir_all(&cwd).expect("Could not create syscall dir for artifacts");

    let mut librs = cwd.clone();
    librs.push("lib.rs");
    write(librs, rs)
        .expect("Could not write lib.rs");

    let mut indexdts = cwd.clone();
    indexdts.push("index.d.ts");
    write(indexdts, ts).expect("Could not write index.d.ts file");

    let mut indexjs = cwd.clone();
    indexjs.push("index.js");
    write(indexjs, js).expect("Could not write index.js file");

    println!("------ Generated NPM package ------");

    println!("[INFO] DONE!");
    println!("[INFO] Now run `npm i ./syscall` to install the package");
}
