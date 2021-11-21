use std::collections::HashMap;

use crate::lexer::{Direction, Tokens};

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

pub fn simulate_program(program: &Vec<Tokens>, debug: bool) {
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