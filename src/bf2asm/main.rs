use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::fs;
use std::path::Path;

type Result<T> = std::result::Result<T, BFError>;

const OPT: bool = true;

#[derive(Debug)]
enum BFError {
    Argument(String),
    IO(io::Error),
    Parser(String),
}

impl fmt::Display for BFError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BFError::Argument(ref s) => write!(f, "ArgumentError: {}", s),
            BFError::IO(ref e) => write!(f, "IOError: {}", e.description()),
            BFError::Parser(ref s) => write!(f, "ParserError: {}", s),
        }
    }
}

impl From<io::Error> for BFError {
    fn from(err: io::Error) -> BFError {
        BFError::IO(err)
    }
}

#[derive(Debug)]
enum BFToken {
    ModValue(i32),
    ModPointer(i32),
    Loop(Vec<BFToken>),
    Input,
    Output,
}

fn parse(file_path: &str) -> Result<Vec<BFToken>> {
    fn push(stack: &mut Vec<Vec<BFToken>>, token: BFToken) -> Result<()> {
        let peeked_vec = stack
            .last_mut()
            .ok_or_else(|| BFError::Parser("Mooh".to_string()))?;

        if OPT {
            if let Some(prev) = peeked_vec.last_mut() {
                match (&prev, &token) {
                    (&&mut BFToken::ModValue(i), &BFToken::ModValue(j)) => {
                        *prev = BFToken::ModValue(i + j);
                        return Ok(());
                    }
                    (&&mut BFToken::ModPointer(i), &BFToken::ModPointer(j)) => {
                        *prev = BFToken::ModPointer(i + j);
                        return Ok(());
                    }
                    (_, _) => {}
                }
            }
        }

        peeked_vec.push(token);
        Ok(())
    }

    let path = Path::new(file_path);
    let file = BufReader::new(File::open(&path)?);

    let mut stack: Vec<Vec<BFToken>> = Vec::new();
    stack.push(Vec::new());

    for line in file.lines() {
        for character in line?.chars() {
            let token = match character {
                '+' => BFToken::ModValue(1),
                '-' => BFToken::ModValue(-1),
                '>' => BFToken::ModPointer(1),
                '<' => BFToken::ModPointer(-1),
                '.' => BFToken::Output,
                ',' => BFToken::Input,
                '[' => {
                    stack.push(Vec::new());
                    continue;
                }
                ']' => {
                    let loop_body = stack
                        .pop()
                        .ok_or_else(|| BFError::Parser("Unbalanced number of [ or ]".into()))?;
                    BFToken::Loop(loop_body)
                }
                _ => continue,
            };
            push(&mut stack, token)?;
        }
    }
    if stack.len() != 1 {
        return Err(BFError::Parser("Unbalanced number of [ or ]".into()));
    }
    Ok(stack.pop().unwrap())
}

fn generate_recursive(tokens: Vec<BFToken>, mut level: u32) -> (String, u32) {
    let mut buffer = String::new();

    for token in tokens {
        match token {
            BFToken::Input => unimplemented!(),
            BFToken::Output => {
                buffer.push_str("    stdb r1, CONSOLEIO_START\n");
            }

            BFToken::Loop(loop_tokens) => {
                let label = format!("label_{}", level);

                buffer.push_str(&format!("{}_before:\n", label));
                buffer.push_str("    cmpi r1, 0\n");
                buffer.push_str(&format!("    breq {}_after\n", label));

                level += 1;
                let (code, new_level) = generate_recursive(loop_tokens, level);
                level = new_level;
                buffer.push_str(&code);
                buffer.push_str(&format!("    jmp {}_before\n", label));
                buffer.push_str(&format!("{}_after:\n", label));
            }

            BFToken::ModPointer(value) => {
                buffer.push_str("    stb r1, r0\n");

                if value > 0 {
                    buffer.push_str(&format!("    addi r0, r0, {}\n", value));
                } else {
                    buffer.push_str(&format!("    subi r0, r0, {}\n", -value));
                }

                buffer.push_str("    ldb r1, r0\n");

            }

            BFToken::ModValue(value) => {
                if value > 0 {
                    buffer.push_str(&format!("    addi r1, r1, {}\n", value));
                } else {
                    buffer.push_str(&format!("    subi r1, r1, {}\n", -value));
                }
            }

        }
    }

    (buffer, level)
}

fn generate(tokens: Vec<BFToken>) -> String {
    let mut buffer = String::new();
    buffer.push_str("    ldi r0, PROGRAM_END\n");
    let (result, _) = generate_recursive(tokens, 0);
    buffer.push_str(&result);
    buffer.push_str("    halt\n");
    buffer

}

fn parse_commandline_args() -> Result<String> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => Ok(args[1].to_string()),
        0 | 1 => Err(BFError::Argument("Not enough arguments!".to_string())),
        _ => Err(BFError::Argument("Too many arguments!".to_string())),
    }
}

fn do_it() -> Result<()> {
    let filename = parse_commandline_args()?;
    let program = parse(filename.as_str())?;

//    println!("{:#?}", program);

    let code = generate(program);
    print!("{}", &code);

    fs::write("output.asm", code).expect("Failed to write file");

    Ok(())
}

fn main() {
    match do_it() {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}
