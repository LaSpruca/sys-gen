mod validator;
pub mod types;
mod generators;

pub use generators::generate;
pub use validator::validate;

use serde_yaml::from_str;
use crate::types::SyscallsDef;
use serde_yaml::Result;

pub fn parse_file(source: &str) -> Result<SyscallsDef> {
    from_str(source)
}
