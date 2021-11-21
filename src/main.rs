mod preprocessor;
mod lexer;
mod simulator;

//TODO: Implement proper and complete error handling

fn main() {
    let mut filepath = std::env::args().nth(1).expect("No file given");
    if !filepath.ends_with(".qs") {
        filepath.push_str(".qs");
    }
    println!("Applying preprocessor to {}...", filepath);
    let code = preprocessor::process_sources(&filepath);
    println!("Tokenizing the code...");
    let program = lexer::tokenize(&code);
    let mut debug = false;
    match std::env::args().nth(2) {
        Some(d) => {
            if d == "debug" {
                debug = true;
            }
        },
        _ => {}
    }
    if !debug {
        println!("Debug mode disabled, to enable it write \'debug\' as the first argument")
    }else {
        println!("[DEBUG] Tokens: {:?}", program);
    }
    println!("Simulating the program...");
    simulator::simulate_program(&program, debug);
}
