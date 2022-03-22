mod generators;
mod types;

use std::fs::File;

use generators::generate;
use serde_yaml::from_reader;
use types::SyscallsDef;

fn main() {
    let x = File::open("sys.yaml").unwrap();
    let result: SyscallsDef = from_reader(x).unwrap();

    generate(result);
}
