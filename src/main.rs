mod lex;
mod ast;
mod parse;

fn main() {
    let path = std::env::args().nth(1).expect("usage: chordcalc <file>"); //expects 2 arguments, the second being the file to process
    let src = std::fs::read_to_string(&path).expect("read file"); //read file
    let tokens = lex::tokenize(&src);

    /* 
    match parse::parse_song(&tokens) {
        Ok(song) => println!("Parsed {} bars successfully", songs.bars.len()),
        Err(diags) => {
            for e in diags {eprintln!("{e}");}
            std::process::exit(1);
        }
    }
    */

    for tok in tokens {
        println!("{:#?}", tok);
    }
}
