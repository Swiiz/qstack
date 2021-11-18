//TODO: Implement real error handling

use std::{collections::HashMap, fs::File, io::Read};
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Right,
    Left,
}
impl Direction {
    fn other(&self) -> Direction {
        match self {
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }

    fn move_pc(&self, pc: &mut usize) {
        match self {
            Direction::Right => *pc += 1,
            Direction::Left => *pc -= 1,
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
enum Tokens<'a> {
    Push(i64),
    Pop,
    Swap,
    Dup,
    Rot,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Dump,
    Print,
    Jump(Direction),
    FunStart,
    FunEnd,
    Identifier(&'a str),
}

fn parse_file<'a>(code: &'a str) -> Vec<Tokens<'a>> {
    let mut tokens = Vec::new();
    Regex::new(r"\n").unwrap().split(code).filter(|l| !l.starts_with(";")).for_each(|l| {
        for tok in Regex::new(r"\s+").unwrap().split(l).filter(|s| !s.is_empty()).collect::<Vec<&str>>() {
            match tok {
                "!" => tokens.push(Tokens::Pop),
                "$" => tokens.push(Tokens::Swap),
                "&" => tokens.push(Tokens::Dup),
                "@" => tokens.push(Tokens::Rot),
                "+" => tokens.push(Tokens::Add),
                "-" => tokens.push(Tokens::Sub),
                "*" => tokens.push(Tokens::Mul),
                "%" => tokens.push(Tokens::Mod),
                "^" => tokens.push(Tokens::Pow),
                "/" => tokens.push(Tokens::Div),
                "." => tokens.push(Tokens::Dump),
                ":" => tokens.push(Tokens::Print),
                "{" => tokens.push(Tokens::FunStart),
                "}" => tokens.push(Tokens::FunEnd),
                ">" => tokens.push(Tokens::Jump(Direction::Right)),
                "<" => tokens.push(Tokens::Jump(Direction::Left)),
                str=> {
                    match str.parse::<i64>() {
                        Ok(v) => tokens.push(Tokens::Push(v)), // Number
                        _ => {
                            match str.starts_with("*") {
                                true => str
                                        .chars()
                                        .into_iter()
                                        .skip(1)
                                        .for_each(|c| tokens.push(Tokens::Push(c as i64))), // Char
                                false => tokens.push(Tokens::Identifier(str)), // Identifier
                            }
                        }, 
                    }
                }
            };
        }
    });
    tokens
}

fn simulate_program(program: Vec<Tokens>, debug: bool) {
    let mut stack: Vec<i64> = Vec::new();
    let mut swap = 0;
    let mut funs: HashMap<&str, usize> = HashMap::new();
    let mut fun_stack: Vec<usize> = Vec::new();
    let mut pc = 0;
    while pc != program.len() {
        match &program[pc] {
            Tokens::Push(x) => {
                stack.push(*x);
            },
            Tokens::Pop => {
                stack.pop();
            },
            Tokens::Swap => {
                let a = stack.pop().unwrap();
                stack.push(swap);
                swap = a;
            },
            Tokens::Dup => {
                stack.push(stack[stack.len()-1]);
            },
            Tokens::Rot => {
                stack.rotate_right(1);
            },
            Tokens::Add => {
                let x = stack.pop();
                let y = stack.pop();
                stack.push(x.unwrap() + y.unwrap());
            },
            Tokens::Sub => {
                let x = stack.pop();
                let y = stack.pop();
                stack.push(y.unwrap() - x.unwrap());
            },
            Tokens::Mul => {
                let x = stack.pop();
                let y = stack.pop();
                stack.push(x.unwrap() * y.unwrap());
            },
            Tokens::Div => {
                let x = stack.pop();
                let y = stack.pop();
                stack.push(x.unwrap() / y.unwrap());
            },
            Tokens::Mod => {
                let x = stack.pop();
                let y = stack.pop();
                stack.push(y.unwrap() % x.unwrap());
            },
            Tokens::Pow => {
                let x = stack.pop();
                let y = stack.pop();
                stack.push(y.unwrap().pow(x.unwrap() as u32));
            },
            Tokens::Dump => {
                println!("{:?}", stack.pop().unwrap());
            },
            Tokens::Print => {
                print!("{}", stack.pop().unwrap() as u8 as char);
            },
            Tokens::Jump(dir) => {
                if stack.pop().unwrap() > 0 {
                    let mut sub = -1;
                    while program[pc] != Tokens::Jump(dir.other()) || sub != 0 {
                        if program[pc] == Tokens::Jump(*dir) {
                            sub += 1;
                        }else if program[pc] == Tokens::Jump(dir.other()) {
                            sub -= 1;
                        }
                        dir.move_pc(&mut pc);
                    }
                }
            },
            Tokens::Identifier(name) => {
                if program.len() > pc + 1 && program[pc + 1] == Tokens::FunStart {
                    funs.insert(name, pc);
                    while program[pc] != Tokens::FunEnd {
                        pc += 1;
                    }
                } else {
                    if funs.contains_key(name) {
                        fun_stack.push(pc);
                        pc = funs[name];
                    }else {
                        println!("Error: function identifer {} not found", name);
                        break;
                    }
                }
            },
            Tokens::FunStart => {},
            Tokens::FunEnd => {
                pc = fun_stack.pop().unwrap();
            }
        };
        if debug {
            println!("[DEBUG] {:?} ${} {:?}", program[pc], swap, stack);
        }
        pc += 1;
    }
}

fn main() {
    let mut filepath = std::env::args().nth(1).expect("No file given");
    if !filepath.ends_with(".qs") {
        filepath.push_str(".qs");
    }
    println!("{}:", filepath);
    let mut file = File::open(filepath).expect("Invalid file path.");
    let mut code = String::new();
    file.read_to_string(&mut code).expect("Failed to read the provided file.");
    let program = parse_file(&code);
    let mut debug = false;
    match std::env::args().nth(2) {
        Some(d) => {
            if d == "debug" {
                debug = true;
            }
        },
        _ => {}
    }
    if debug {
        println!("[DEBUG] Tokens: {:?}", program);
    }
    simulate_program(program, debug);
}
