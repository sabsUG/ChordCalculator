mod ast;
mod calc;
mod lex;
mod parse;
mod table;

fn main() {
    let path = std::env::args().nth(1).expect("usage: chordcalc <file>"); //expects 2 arguments, the second being the file to process
    let src = std::fs::read_to_string(&path).expect("read file"); //read file

    let tokens = lex::tokenize(&src);
    if cfg!(debug_assertions) {
        println!("=== TOKENS ===");
        for tok in &tokens {
            println!("{:#?}", tok);
        }
    }

    match parse::parse_song(&tokens) {
        Ok(song) => {
            println!("This is a valid song");
            if cfg!(debug_assertions) {
                println!("\n=== AST ===");
                println!("{:#?}", song);
            }
            println!("\n=== Pitch Classes ===");
            calc::analyze_song(&song);

            table::print_pitch_table(&song);
        }
        Err(err) => {
            eprintln!("\nParse error: {} at {:?}", err.msg, err.span);
            parse::show_error_span(&src, &err.span);
        }
    }
}
