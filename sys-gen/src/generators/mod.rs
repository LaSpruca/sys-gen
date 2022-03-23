mod js;
mod rs;
mod ts;

use std::collections::HashMap;

use crate::types::SyscallsDef;

use crate::generators::js::generate_js;
use crate::generators::ts::generate_ts_enum;
use common_macros::hash_map;
use rs::generate_rs;
use ts::generate_ts;

pub fn generate(def: SyscallsDef) -> (String, String, String) {
    println!("--------- Generating code ---------");

    let type_reg: HashMap<String, String> = hash_map! {
        "usize" => "number",
        "[u8]" => "number[]",
        "String" => "string",
    }
    .iter()
    .map(|(a, b)| (a.to_string(), b.to_string()))
    .collect();

    let js_file = generate_js(&def.enums);

    let mut ts_file = generate_ts_enum(&def.enums);

    ts_file += "\n\n";

    let mut rs_body = "use std::arch::asm;
use neon::prelude::*;"
        .to_string();

    let mut rs_main = "#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {"
        .to_string();

    for call in def.calls.iter() {
        println!("[INFO] Generating syscall {}", &call.name);
        ts_file += &format!("{}\n", generate_ts(call, &type_reg));
        rs_body += &format!("\n\n{}", generate_rs(call));
        rs_main += &format!(
            "\n\tcx.export_function(\"sys{}\", sys_{})?;",
            first_upper(call.name.as_str()),
            &call.name
        );
    }

    rs_main += "\n\tOk(())\n}";

    let rs_file = format!("{rs_body}\n\n{rs_main}");

    println!("------- Generation complete -------");

    (rs_file, js_file, ts_file)
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
