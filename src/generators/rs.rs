mod args;
mod effect;
mod rs_return;

use crate::{
    generators::rs::{effect::generate_effect, rs_return::generate_return},
    types::{Call, TypeValue},
};
use args::generate_arg;
use std::process::exit;

pub fn generate_rs(call: &Call) -> String {
    let Call {
        args,
        params,
        n,
        name,
        return_type,
        effect,
    } = call.to_owned();

    format!(
        "fn sys_{name}(mut cx: FunctionContext) -> JsResult<{}> {{
{}\n{}\n{}\n{}\n}}",
        match return_type.as_str() {
            "usize" => "JsNumber".to_string(),
            _ => {
                eprintln!("Cannot use `{return_type}` for return type");
                exit(1);
            }
        },
        params
            .iter()
            .enumerate()
            .map(|(index, (arg, rs_type))| generate_arg(
                index,
                arg,
                if rs_type.starts_with("$") {
                    "usize".to_string()
                } else {
                    rs_type.to_string()
                }
            ))
            .collect::<Vec<String>>()
            .join("\n"),
        generate_syscall(n, &args),
        generate_effect(&effect, &params),
        generate_return(&return_type)
    )
}

fn generate_syscall(n: usize, args: &Vec<(TypeValue, String)>) -> String {
    format!(
        r#"{0}let ret: usize;
{0}unsafe {{
{0}{0}asm!("syscall",
{0}{0}{0}in("rax") {n},
{1},
{0}{0}{0}lateout("rax") ret);
{0}}}"#,
        "\t",
        args.iter()
            .enumerate()
            .map(|(index, (accessor, name))| match accessor {
                TypeValue::Value => format!("\t\t\tin(\"{}\") {name}", get_reg(index)),
                TypeValue::Ptr => format!("\t\t\tin(\"{}\") {name}.as_ptr()", get_reg(index)),
                TypeValue::Len => format!("\t\t\tin(\"{}\") {name}.len()", get_reg(index)),
                TypeValue::PtrIO => format!("\t\t\tinout(\"{}\") {name}.as_ptr()", get_reg(index)),
            })
            .collect::<Vec<String>>()
            .join(",\n")
    )
}

#[inline(always)]
fn get_reg(index: usize) -> &'static str {
    match index {
        0 => "rdi",
        1 => "rsi",
        2 => "rdx",
        3 => "r10",
        4 => "r8",
        5 => "r9",
        _ => unreachable!(),
    }
}
