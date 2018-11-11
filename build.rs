use std::process::{Command, exit};

fn main() {
    let ret = Command::new("python3").arg("generate_instructions.py").status().unwrap();
    exit(ret.code().unwrap());
}