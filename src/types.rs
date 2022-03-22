use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyscallsDef {
    pub calls: Vec<Call>,
    pub enums: HashMap<String, HashMap<String, usize>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Call {
    pub n: usize,
    pub name: String,
    pub params: Vec<(String, String)>,
    pub args: Vec<(TypeValue, String)>,
    #[serde(rename = "return")]
    pub return_type: String,
    #[serde(default)]
    pub effect: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeValue {
    #[serde(rename = "val")]
    Value,
    #[serde(rename = "ptr")]
    Ptr,
    #[serde(rename = "pio")]
    PtrIO,
    #[serde(rename = "len")]
    Len,
}
