mod generators;
mod types;
mod validator;

use std::env::{args, current_dir};
use std::fs::{copy, create_dir_all, write, File};
use std::process::{Command, Stdio};

use generators::generate;
use serde_yaml::from_reader;
use types::SyscallsDef;
use uuid::Uuid;

fn main() {
    let path = args().skip(1).next().unwrap_or("sys.yaml".to_string());
    let x = File::open(&path).unwrap();
    let result: SyscallsDef = from_reader(x).unwrap();

    let (rs, js, ts) = generate(result);

    let temp_id = Uuid::new_v4().to_string();
    let mut temp_dir = std::env::temp_dir();

    temp_dir.push(temp_id);

    create_dir_all(&temp_dir).expect("Could not create temp dir");

    let mut cargo = temp_dir.clone();
    cargo.push("Cargo.toml");
    write(cargo, include_str!("project_files/Cargo.toml")).expect("Could not create cargo.toml");

    let mut main = temp_dir.clone();
    main.push("src");
    create_dir_all(&main).expect("Could not create src dir in temp folder");
    main.push("lib.rs");
    write(main, rs).expect("Could not write lib.rs");

    let mut compile_command = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(&temp_dir)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    compile_command.wait().expect("Could not compile crate");

    let mut cwd = current_dir().unwrap();
    cwd.push("syscall");
    create_dir_all(&cwd).expect("Could not create syscall dir for artifacts");

    let mut node_old = temp_dir.clone();
    node_old.push("target");
    node_old.push("release");
    node_old.push("libsyscall.so");

    let mut new_node = cwd.clone();
    new_node.push("index.node");

    copy(&node_old, &new_node).expect("Could not copy output");

    let mut package = cwd.clone();
    package.push("package.json");
    write(package, include_str!("project_files/package.json"))
        .expect("Could not create package.json");

    let mut indexdts = cwd.clone();
    indexdts.push("index.d.ts");
    write(indexdts, ts).expect("Could not write index.d.ts file");

    let mut indexjs = cwd.clone();
    indexjs.push("index.js");
    write(indexjs, js).expect("Could not write index.js file");

    println!("DONE!");
    println!("Now run `npm i ./syscall` to install the package");
}
