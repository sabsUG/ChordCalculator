#![allow(warnings)]
use chordcalc::lex;
use chordcalc::ast;
use chordcalc::parse;

use std::process::Command;
use std::fs;
use std::path::Path;


#[test]
fn all_cases(){
    let mut total = 0;
    let mut passed = 0;
    let mut failed = 0;
    
    for entry in std::fs::read_dir("tests/cases").unwrap(){
        let path = entry.unwrap().path();

        let filename = path.file_name().unwrap().to_string_lossy();

        run_test(path.to_str().unwrap(), &filename, &mut total, &mut passed, &mut failed);
    }
    println!(
        "\nSummary: {} total | \x1b[32m{} passed\x1b[0m | \x1b[31m{} failed\x1b[0m\n",
        total, passed, failed
    );

}

fn run_test(input_file: &str, file_name: &str, total: &mut i32, passed: &mut i32, failed: &mut i32){
    *total +=1;

    let input_text = fs::read_to_string(input_file)
        .expect("Failed to read input file");

    let tokens = lex::tokenize(&input_text);

    match parse::parse_song(&tokens) {
        Ok(song) => {
            println!("\x1b[32m✅ PASS: {}\x1b[0m", file_name);
            println!("\n==============================================================");
            //println!("{:#?}", song);
            *passed += 1;
        }
        Err(err) => {
            println!("\x1b[31m❌ FAIL: {}\x1b[0m", file_name);
            //eprintln!("\nParse error: {} at {:?}", err.msg, err.span);
            parse::show_error_span(&input_text, &err.span);
            *failed +=1;
        }
    }

}