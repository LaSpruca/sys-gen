use std::collections::HashMap;
use std::str::Chars;

use super::first_upper;
use crate::{generators::get_from_type_map, types::Call};

pub fn generate_ts_enum(enums: &HashMap<String, HashMap<String, usize>>) -> String {
    enums
        .iter()
        .map(|(name, fields)| {
            format!(
                "export declare enum {name} {{\n{}\n}}",
                fields
                    .iter()
                    .map(|(name, value)| format!("\t{name} = {value}"))
                    .collect::<Vec<String>>()
                    .join(",\n")
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn generate_ts(def: &Call, type_map: &HashMap<String, String>) -> String {
    let Call {
        name,
        params,
        return_type,
        ..
    } = def.to_owned();

    format!(
        "export declare function sys{}({}): {};",
        first_upper(name.as_str()),
        params
            .iter()
            .map(|(name, rs_type)| format!(
                "{name}: {}",
                get_ts_return_type(rs_type, type_map)
            ))
            .collect::<Vec<String>>()
            .join(", "),
        get_ts_return_type(&return_type, type_map)
    )
}

fn get_ts_return_type(rs_type: &String, type_map: &HashMap<String, String>) -> String {
    if rs_type.starts_with("$") {
        rs_type
            .chars()
            .skip(1)
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join("")
    } else {
        get_from_type_map(type_map, &rs_type)
    }
}
