use std::process::exit;

pub fn generate_effect(effect: &Vec<String>, params: &Vec<(String, String)>) -> String {
    if effect.is_empty() {
        return String::new();
    }

    effect
        .iter()
        .map(|effect| {
            let index = params.iter().position(|(f, _)| f == effect).unwrap();
            (effect.to_owned(), params[index].clone().1, index)
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
        "\tprintln!(\"{{{name}:?}}\");
\tlet {name}_js: Handle<JsArray> = cx.argument({index})?;
\tfor (index, val) in {name}.iter().enumerate() {{
\t\tlet num = cx.number(index as f64);
\t\tlet other_num = cx.number((*val) as f64);
\t\t{name}_js.set(&mut cx,  num, other_num)?;
\t}}"
    )
}
