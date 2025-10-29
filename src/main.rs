mod lex;
mod ast;
mod parse;

fn main() {
    let path = std::env::args().nth(1).expect("usage: chordcalc <file>"); //expects 2 arguments, the second being the file to process
    let src = std::fs::read_to_string(&path).expect("read file"); //read file
    
    let tokens = lex::tokenize(&src);
    println!("=== TOKENS ===");
    for tok in &tokens {
        println!("{:#?}", tok);
    }

    
    match parse::parse_song(&tokens) {
        Ok(song) => {
            println!("\n=== AST ===");
            println!("{:#?}", song);
        }
        Err(err) => {
            eprintln!("\nParse error: {} at {:?}", err.msg, err.span);
            parse::show_error_span(&src, &err.span);
        }
    }
    
}
