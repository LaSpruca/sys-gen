use std::{env, fs};
use std::path::Path;
use sys_gen::generate;
use sys_gen::validate;

fn main() {
    let sys_yaml_path = std::env::var("SYS_YAML_PATH").unwrap_or("../sys.yaml".to_string());
    let file = fs::read_to_string("../sys.yaml").unwrap();
    let def = sys_gen::parse_file(&file).unwrap();

    validate(&def).unwrap();

    let (rs, js, ts) = generate(def);

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lib.rs");

    fs::write(
        &dest_path,
        rs.as_str(),
    ).unwrap();

    fs::write(
        "index.js",
        js.as_str(),
    ).unwrap();

    fs::write(
        "index.d.ts",
        ts.as_str(),
    ).unwrap();

    println!("cargo:rerun-if-changed=../sys.yaml");
    #[cfg(debug_assertions)]
    println!("cargo:rerun-i=../sys.yaml");
}