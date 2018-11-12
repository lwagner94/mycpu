from string import Template
import yaml

match_template = """ 
        "$keyword" if line.tokens.len() == $if_guard => DecodedInstruction::new(
            Instruction::$name,
            $reg1,
            $reg2,
            $reg3,
            $operand),"""

R1 = "match_register_name(line.tokens[1].token.as_str())?"
R2 = "match_register_name(line.tokens[2].token.as_str())?"
R3 = "match_register_name(line.tokens[3].token.as_str())?"
OP = "parse_numeric_literal(line.tokens[2].token.as_str())?"

if __name__ == "__main__":
    with open("instructions.yaml") as f:
        instructions = yaml.safe_load(f)

    with open("src/assembler/matcher.rs.template") as f:
        template = f.read()
    cases = ""

    for instruction in instructions:
        name = instruction["name"]
        keyword = instruction["keyword"]
        regs = instruction["regs"]
        op = instruction["op"]

        if_guard = regs + op + 1

        reg1 = R1 if regs >= 1 else "0"
        reg2 = R2 if regs >= 2 else "0"
        reg3 = R3 if regs >= 3 else "0"
        operand = OP if op == 1 else "0"

        t = Template(match_template)
        cases += t.substitute(**locals())

    t = Template(template)

    with open("src/assembler/generated/matcher.rs", "w") as f:
        f.write(t.substitute(**locals()))



