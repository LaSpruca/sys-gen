use std::{collections::HashMap, process::exit};

pub fn generate_effect(effect: &Vec<String>, params: &HashMap<String, String>) -> String {
    if effect.is_empty() {
        return String::new();
    }

    effect
        .iter()
        .filter(|effect| params.contains_key(effect.to_owned()))
        .map(|effect| {
            (
                effect.to_owned(),
                params.get(effect).unwrap(),
                params
                    .keys()
                    .map(ToOwned::to_owned)
                    .position(|f| f == effect.to_owned())
                    .unwrap(),
            )
        })
        .map(|(name, rs_type, index)| match rs_type.as_str() {
            "[u8]" => effect_u8_arr(name, index),
            _ => {
                eprintln!("Effects on type `{rs_type}` aren't implemented");
                exit(1);
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn effect_u8_arr(name: String, index: usize) -> String {
    format!(
        "\tlet {name}_js: Handle<JsArray> = cx.argument({index})?;  
\tfor (index, val) in {name}.iter().enumerate() {{
\t\tlet num = cx.number(index as f64);
\t\tlet other_num = cx.number((*val) as f64);
\t\t{name}_js.set(&mut cx,  num, other_num)?;
\t}}"
    )
}
