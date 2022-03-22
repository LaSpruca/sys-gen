use std::collections::HashMap;

use super::first_upper;
use crate::{types::Call, generators::get_from_type_map};

pub fn generate_js(def: &Call, type_map: &HashMap<String, String>) -> String {
    let Call {
        name,
        params,
        return_type,
        ..
    } = def.to_owned();

    format!(
        "export declare function sys{}({}): {return_type};",
        first_upper(name.as_str()),
        params
            .iter()
            .map(|(name, rs_type)| format!(
                "{name}: {}",
                get_from_type_map(type_map, rs_type)
            ))
            .collect::<Vec<String>>()
            .join(", ")
    )
}
