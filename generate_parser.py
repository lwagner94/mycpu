from string import Template
import yaml


TEMPLATE = """
// AUTOMATICALLY GENERATED, DO NOT EDIT!

#[derive(Debug)]
pub enum Instruction {
$instr
}

impl Into<u8> for Instruction {
    fn into(self: Self) -> u8 {
        match self {
$instr_to_code
        }
    }
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Instruction {
        match value {
$code_to_instr
            _ => Instruction::Invalid
        }
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

        instr_to_code += f"            Instruction::{name} => {code},\n"

        code_to_instr += f"            {code} => Instruction::{name},\n"

    t = Template(TEMPLATE)

    with open("src/common/generated/instruction.rs", "w") as f:
        f.write(t.substitute(**locals()))



