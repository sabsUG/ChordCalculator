# 🎵 Chord Calculator

A Rust-based parser and chord analysis tool that verifies and interprets chord progressions according to a defined EBNF grammar.

---

## 🧠 Overview
This project reads a song file containing chord progressions, tokenizes it, and parses it using a recursive descent parser.  
If the file conforms to the grammar, it produces an Abstract Syntax Tree (AST) and validates the input.

---

## ⚙️ Features
- Lexical analysis and tokenization (`lex.rs`)
- Parser and AST construction (`parse.rs`)
- Grammar defined in EBNF
- Error spans and reporting
- Test automation for multiple input files

---

## 🚀 Usage
To run the project:

```bash
cargo run --release <path_to_input_file>
```

To run the test suite: 
```bash
cargo test --release -- --nocapture
```

TBD: small pipeline in github with docker container build, and tests stages