use std::{fs::File, io::Read, os::raw};

/// The preprocessor activate in between # and the next # or # and line break.
#[derive(Debug)]
enum LiteralContent {
    /// A string literal.
    String(String),
    /// A number literal.
    Number(i64),
}

#[derive(Debug)]
enum PreprocessorToken {
    Identifier(String),
    If, Else, Repeat, End,
    Assignment,
    Literal(LiteralContent),
    Include(String),
    Plus, Minus, Multiply, Divide, Modulo, Power,
    Equal, NotEqual, Less, LessEqual, Greater, GreaterEqual, And, Or, Not,
}

/// Take the entrypoint .qs filepath.
pub fn process_sources<'a>(filepath: &'a str) -> String {
    let mut file = File::open(filepath).expect("Invalid file path.");
    let mut code = String::new();
    file.read_to_string(&mut code).expect("Failed to read the provided file.");
    let segments = tokenize(&code);
    println!("{:?}", segments);
    code
}

#[derive(Debug)]
enum CodeSegment<'a> {
    RawCode(&'a str),
    PreprocessorCode(Vec<PreprocessorToken>),
}

fn char_at(code: &str, index: usize) -> char {
    code.chars().nth(index).unwrap()
}

/// Is the character something that delimitate the end of a token;
fn is_token_break(code: &str, index: usize) -> bool {
    let c = char_at(code, index);
    c == ' ' || c == '\t' || c == '\n' || c == '\r' || c == '\0' || index == code.len() - 1
}

//TODO: Ability to escape \" et \' in strings
fn recognize_token(i: &mut usize, code: &str) -> PreprocessorToken {
    let mut raw = String::new();
    let mut local_index = 0;
    loop {
        raw.push(char_at(code, *i));

        if is_token_break(code, *i)
        && (
            (
                !raw.starts_with("\"")
            &&  !raw.starts_with("'")
            &&  !raw.starts_with("include")
            ) || (
                (
                    char_at(code, *i) == '"' || char_at(code, *i) == '\''
                ) && (
                    local_index != 0
                )
            )
        ) {
            break;
        }
        local_index += 1;
    }
    match raw.as_str() {
        "if" => PreprocessorToken::If,
        "else" => PreprocessorToken::Else,
        "repeat" => PreprocessorToken::Repeat,
        "end" => PreprocessorToken::End,
        "=" => PreprocessorToken::Assignment,
        "+" => PreprocessorToken::Plus,
        "-" => PreprocessorToken::Minus,
        "*" => PreprocessorToken::Multiply,
        "/" => PreprocessorToken::Divide,
        "%" => PreprocessorToken::Modulo,
        "^" => PreprocessorToken::Power,
        "==" => PreprocessorToken::Equal,
        "!=" => PreprocessorToken::NotEqual,
        "!" => PreprocessorToken::Not,
        "<" => PreprocessorToken::Less,
        "<=" => PreprocessorToken::LessEqual,
        ">" => PreprocessorToken::Greater,
        ">=" => PreprocessorToken::GreaterEqual,
        "&&" => PreprocessorToken::And,
        "||" => PreprocessorToken::Or,
        str => {
            if str.starts_with("include") {
                let mut parts = str.split(&['"', '\''][..]);
                PreprocessorToken::Include(parts.nth(1).unwrap().to_string())
            }else {
                match str.parse::<i64>() {
                    Ok(v) => PreprocessorToken::Literal(LiteralContent::Number(v)), // Number
                    _ => {
                        match str.starts_with("\"") || str.starts_with("\'") {
                            true => PreprocessorToken::Literal(LiteralContent::String(
                                str.chars()
                                    .into_iter()
                                    .skip(1)
                                    .take_while(|c| *c != '"' && *c != '\'')
                                    .collect::<String>(),
                            )), // String
                            false => PreprocessorToken::Identifier(str.to_string()), // Identifier
                        }
                    }
                }
            }
        }
    }
}

fn tokenize<'a>(code: &'a str) -> Vec<CodeSegment<'a>> {
    let mut segments = Vec::new();
    let mut tokens = Vec::new();
    let mut rawcode_index = (0, 0);
    let mut inside_preprocessor = false;

    for mut i in 0..code.chars().count() {
        if char_at(code, i) == '#' || char_at(code, i) == '\n' || i == code.len() - 1 {
            if inside_preprocessor {
                inside_preprocessor = false;
                segments.push(CodeSegment::PreprocessorCode(tokens));
                tokens = Vec::new();
                rawcode_index.1 = i;
            }
        }
        if char_at(code, i) == '#' && !inside_preprocessor {
            inside_preprocessor = true;
            segments.push(CodeSegment::RawCode(&code[rawcode_index.0..rawcode_index.1]));
        }else if inside_preprocessor {
            if is_token_break(code, i) {
                loop {
                    i += 1;
                    if is_token_break(code, i) {
                        break;
                    }
                }    
            }else {
                tokens.push(recognize_token(&mut i, code));
            }
        }
        if !inside_preprocessor && i == code.len() - 1 {
            rawcode_index.1 = i;
            segments.push(CodeSegment::RawCode(&code[rawcode_index.0..rawcode_index.1]));
        }
    }
                

    segments
}