mod js;
mod rs;

use std::collections::HashMap;

use crate::types::SyscallsDef;

use common_macros::hash_map;
use js::generate_js;
use rs::generate_rs;

pub fn generate(def: SyscallsDef) {
    let type_reg: HashMap<String, String> = hash_map! {
        "usize" => "number",
        "[u8]" => "number[]",
        "String" => "string",
    }
    .iter()
    .map(|(a, b)| (a.to_string(), b.to_string()))
    .collect();

    let mut js_file = String::new();
    let mut rs_body = "use std::arch::asm;
use neon::prelude::*;"
        .to_string();

    let mut rs_main = "#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {"
        .to_string();

    for call in def.calls.iter() {
        println!("Generating syscall {}", &call.name);
        js_file += &format!("{}\n", generate_js(call, &type_reg));
        rs_body += &format!("\n\n{}", generate_rs(call, &type_reg));
        rs_main += &format!(
            "\n\tcx.export_function(\"sys{}\", sys_{})?;",
            first_upper(call.name.as_str()),
            &call.name
        );
    }

    rs_main += "\n\tOk(())\n}";

    let rs_file = format!("{rs_body}\n\n{rs_main}");

    println!("--------------------------------- index.d.ts ---------------------------------");
    println!("{js_file}\n");
    println!("---------------------------------   lib.rs   ---------------------------------");
    println!("{rs_file}");
}

fn first_upper(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn get_from_type_map(type_map: &HashMap<String, String>, name: &String) -> String {
    type_map
        .get(name)
        .and_then(|x| Some(x.to_string()))
        .unwrap_or("unknown".to_string())
}
