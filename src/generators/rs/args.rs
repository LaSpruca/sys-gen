use std::process::exit;

pub fn generate_arg(index: usize, arg: &String, rs_type: &String) -> String {
    match rs_type.as_str() {
        "usize" => num_type_from_arg(index, arg, rs_type),
        "[u8]" => byte_from_arg(index, arg),
        "string" => string_from_arg(index, arg),
        _ => {
            eprintln!("Converter for {rs_type} as arg is not implemented");
            exit(1);
        }
    }
}

fn string_from_arg(index: usize, arg: &str) -> String {
    format!("\tlet {arg} = {{
\t\tlet mut temp = cx.argument::<Handle<JsString>>({index})?.value(&mut cx);     
\t\t
\t}}")
}

fn byte_from_arg(index: usize, arg: &str) -> String {
    format!(
        "\tlet mut {arg} = vec![];
\tfor a in cx.argument::<Handle<JsArray>>({index})?.to_vec(&mut cx).iter() {{
\t\tlet k: Handle<JsNumber> = a.downcast_or_throw(&mut cx)?;
\t\t{arg}.push(k.value(&mut cx) as u8);
\t}}"
    )
}

fn num_type_from_arg(index: usize, arg: &String, rs_type: &String) -> String {
    format!("\tlet {arg} = cx.argument::<Handle<JsNumber>>({index})?.value(&mut cx) as {rs_type};")
}
