#![allow(warnings)]
use chordcalc::ast;
use chordcalc::calc::chord_to_pitch_classes;
use chordcalc::lex;
use chordcalc::parse;

use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn all_cases() {
    let mut total = 0;
    let mut passed = 0;
    let mut failed = 0;

    for entry in std::fs::read_dir("tests/cases").unwrap() {
        let path = entry.unwrap().path();

        let filename = path.file_name().unwrap().to_string_lossy();

        run_test(
            path.to_str().unwrap(),
            &filename,
            &mut total,
            &mut passed,
            &mut failed,
        );
    }
    println!(
        "\nSummary: {} total | \x1b[32m{} passed\x1b[0m | \x1b[31m{} failed\x1b[0m\n",
        total, passed, failed
    );
}

fn run_test(
    input_file: &str,
    file_name: &str,
    total: &mut i32,
    passed: &mut i32,
    failed: &mut i32,
) {
    *total += 1;

    let input_text = fs::read_to_string(input_file).expect("Failed to read input file");

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
            *failed += 1;
        }
    }
}

#[test]
fn test_pitch_classes_for_selected_chords() {
    let input_text = "Gsus2 | Bsus2 | E-(9) | G^7 | B-(9) | A13 | G9#11 | Dno3";
    let tokens = lex::tokenize(&input_text);

    let song = parse::parse_song(&tokens).unwrap();

    let mut chords = Vec::new();

    for bar in &song.bars {
        for item in &bar.items {
            if let chordcalc::ast::BarItem::Chord(ch) = item {
                chords.push(ch.clone());
            }
        }
    }

    //Gsus2 = 2, 7, 9
    assert_eq!(chord_to_pitch_classes(&chords[0]), pc(&[2, 7, 9]));

    // Bsus2 = 1, 6, 11
    assert_eq!(chord_to_pitch_classes(&chords[1]), pc(&[1, 6, 11]));

    // E-(9) = 4, 6, 7, 11
    assert_eq!(chord_to_pitch_classes(&chords[2]), pc(&[4, 6, 7, 11]));

    // G^7 = [2,5,6,7,11]
    assert_eq!(chord_to_pitch_classes(&chords[3]), pc(&[2, 5, 6, 7, 11]));

    // B-(9) = 1, 2, 6, 11
    assert_eq!(chord_to_pitch_classes(&chords[4]), pc(&[1, 2, 6, 11]));

    // A13 = [1,4,9]
    assert_eq!(chord_to_pitch_classes(&chords[5]), pc(&[1, 4, 9]));

    // G9#11 = [0,2,7,11]
    assert_eq!(chord_to_pitch_classes(&chords[6]), pc(&[0, 2, 7, 11]));

    // Dno3 = [2, 9]
    assert_eq!(chord_to_pitch_classes(&chords[7]), pc(&[2, 9]));

    println!("\x1b[32m✅ PASS: pitch-classes {}\x1b[0m", input_text);
    println!("\n==============================================================");
}

fn pc(indices: &[u8]) -> Vec<u8> {
    let mut v = indices.to_vec();
    v.sort();
    v
}
