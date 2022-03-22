use std::process::exit;

pub fn generate_return(return_type: &String) -> String {
    match return_type.as_str() {
        "usize" => "\tOk(cx.number(ret as f64))".to_string(),
        _ => {
            eprintln!("Converter for {return_type} as return is not implemented");
            exit(1);
        }
    }
}
