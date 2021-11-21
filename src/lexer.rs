use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Right,
    Left,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Tokens<'a> {
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

pub fn tokenize<'a>(code: &'a str) -> Vec<Tokens<'a>> {
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