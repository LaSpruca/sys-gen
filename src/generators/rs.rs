mod args;
mod rs_return;
mod effect;

use crate::{
    generators::{get_from_type_map, rs::{rs_return::generate_return, effect::generate_effect}},
    types::{Call, TypeValue},
};
use args::generate_arg;
use std::collections::HashMap;

pub fn generate_rs(call: &Call, type_map: &HashMap<String, String>) -> String {
    let Call {
        args,
        params,
        n,
        name,
        return_type,
        effect
    } = call.to_owned();

    format!(
        "fn sys_{name}(mut cx: FunctionContext) -> JsResult<{}> {{
{}\n{}\n{}\n{}\n}}",
        get_from_type_map(type_map, &return_type),
        params
            .iter()
            .enumerate()
            .map(|(index, (arg, rs_type))| generate_arg(index, arg, rs_type))
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
{0}asm!("syscall", 
{0}{0}in("rax") {n},
{1},
{0}{0}lateout("rax"), ret);"#,
        "\t",
        args.iter()
            .enumerate()
            .map(|(index, (accessor, name))|    
                match accessor {
                    TypeValue::Value => format!("\t\tin(\"{}\") {name}", get_reg(index)),
                    TypeValue::Ptr => format!("\t\tin(\"{}\") {name}.as_ptr()", get_reg(index)),
                    TypeValue::Len => format!("\t\tin(\"{}\") {name}.len()", get_reg(index)),
                    TypeValue::PtrIO => format!("\t\tinout(\"{}\") {name}.as_ptr()", get_reg(index)),
                }
            )
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
