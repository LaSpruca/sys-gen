use std::collections::HashMap;
use common_macros::hash_map;
use thiserror::Error;
use crate::types::{Call, SyscallsDef};

#[derive(Debug, Error)]
pub enum Error {
    #[error("In the enum {enum_name}, the variants {} share the value {value}", .variants.join(", "))]
    EnumVariantWithSameValue { enum_name: String, variants: Vec<String>, value: usize },

    #[error("In call: {call_name}, param: {param_name}: Type {param_type} is not recognised")]
    CallUnknownParamType { call_name: String, param_name: String, param_type: String },

    #[error("In call: {call_name}, unable to use {arg_name} as arg, because it is not listed as a param")]
    CallUnknownArg { call_name: String, arg_name: String },

    #[error("In call: {call_name}, return type {return_type} is not recognised")]
    CallUnknownReturnType { call_name: String, return_type: String },

    #[error("In call: {call_name}, unable to effect {effect_name}, because it is not listed as a param")]
    CallUnknownEffect { call_name: String, effect_name: String },

    #[error("The calls {} all have the same number, {number}", .calls.join(", "))]
    CallMultipleSameNumber { number: usize, calls: Vec<String> },
}

pub fn validate(def: &SyscallsDef) -> Result<(), Vec<Error>> {
    println!("--------- Validating code ---------");

    let errors = vec![];

    println!("[INFO] Validating enums");
    validate_enums(&def.enums);
    println!("[INFO] Validated calls");

    let mut types = vec![
        "usize".to_string(),
        "[u8]".to_string(),
        "String".to_string()
    ];

    types.append(&mut def.enums.keys().map(ToOwned::to_owned).collect());

    validate_calls(&def.calls, &types);

    println!("------- Validation complete -------");

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub fn validate_calls(calls: &Vec<Call>, types: &Vec<String>) -> Vec<Error> {
    let mut errors = vec![];
    let mut values: HashMap<usize, Vec<String>> = hash_map! {};

    for call in calls {
        let mut param_names = vec![];

        for (name, param_type) in call.params.iter() {
            param_names.push(name.to_string());

            // Type check params
            if !types.contains(param_type) {
                errors.push(Error::CallUnknownParamType {
                    call_name: call.name.to_string(),
                    param_type: param_type.to_string(),
                    param_name: name.to_string()
                });
            }

            // Check effects
            for effect in call.effect.iter() {
                if !param_names.contains(effect) {
                    errors.push(Error::CallUnknownEffect {
                        call_name: call.name.to_string(),
                        effect_name: effect.to_string()
                    });
                }
            }

            // Check args
            for (_, arg) in call.args.iter() {
                if !param_names.contains(arg) {
                    errors.push(Error::CallUnknownArg {
                        call_name: call.name.to_string(),
                        arg_name: arg.to_string()
                    });
                }
            }

            // Check return
            if !types.contains(&call.return_type) {
                errors.push(Error::CallUnknownReturnType {
                    return_type: call.return_type.to_string(),
                    call_name: call.name.to_string()
                })
            }

            // Check for syscalls with duplicate numbers
            if values.contains_key(&call.n) {
                let fields = values.get_mut(&call.n).unwrap();
                fields.push(call.name.to_string());
            } else {
                values.insert(call.n, vec![call.name.to_string()]);
            }
        }
    }

    errors.append(&mut values.iter().filter_map(|(n, calls)| {
        if calls.len() != 1 {
            Some(Error::CallMultipleSameNumber {
                number: *n,
                calls: calls.clone(),
            })
        } else {
            None
        }
    }).collect());


    errors
}

pub fn validate_enums(enums: &HashMap<String, HashMap<String, usize>>) -> Vec<Error> {
    let mut errors = vec![];

    for (name, fields) in enums.iter() {
        let mut values: HashMap<usize, Vec<String>> = hash_map! {};

        for (field_name, field_value) in fields {
            if values.contains_key(field_value) {
                let fields = values.get_mut(field_value).unwrap();
                fields.push(field_name.to_string());
            } else {
                values.insert(*field_value, vec![field_name.to_string()]);
            }
        }

        errors.append(&mut values.iter().filter_map(|(value, variants)| {
            if fields.len() != 1 {
                Some(Error::EnumVariantWithSameValue {
                    variants: variants.clone(),
                    enum_name: name.clone(),
                    value: value.clone()
                })
            } else {
                None
            }
        }).collect())
    }

    return errors;
}
