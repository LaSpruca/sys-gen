use std::collections::HashMap;
use std::fmt::format;

pub fn generate_js(enums: &HashMap<String, HashMap<String, usize>>) -> String {
    format!(
        r#"{};

module.exports = {{
{},
{t}...require("./index.node")
}};"#,
        enums
            .iter()
            .map(|(name, fields)| format!(
                "var {name};\n\
            (function({name}) {{\n\
            {}\n\
            }})(Yes || (Yes = {{}}))",
                generate_body(name, fields)
            ))
            .collect::<Vec<String>>()
            .join(";\n"),
        enums
            .iter()
            .map(|(name, _)| format!("\t{name}"))
            .collect::<Vec<String>>()
            .join(", "),
        t = "\t"
    )
}

fn generate_body(name: &String, fields: &HashMap<String, usize>) -> String {
    fields
        .iter()
        .map(|(field_name, value)| {
            format!("\t{name}[{name}[\"{field_name}\"] = {value}] = \"{field_name}\"")
        })
        .collect::<Vec<String>>()
        .join("\n")
}
