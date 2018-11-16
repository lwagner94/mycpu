from string import Template
import yaml


TEMPLATE = """
// AUTOMATICALLY GENERATED, DO NOT EDIT!
use std::mem::transmute;

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum Instruction {
$instr
}

impl Into<u8> for Instruction {
    fn into(self: Self) -> u8 {
        self as u8
    }
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Instruction {
        unsafe {transmute(value)}
    }
}
"""


if __name__ == "__main__":
    with open("instructions.yaml") as f:
        instructions = yaml.safe_load(f)

    instr = ""
    instr_to_code = ""
    code_to_instr = ""

    for instruction in instructions:
        name = instruction["name"]
        code = hex(instruction["code"])

        instr += f"    {name},\n"

    t = Template(TEMPLATE)

    with open("src/common/generated/instruction.rs", "w") as f:
        f.write(t.substitute(**locals()))



