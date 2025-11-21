# 🎵 Chord Calculator

A Rust-based **parser and chord analysis engine** that validates songs using a custom grammar and computes the **pitch classes** that make up each chord. It also generates a **pitch-class histogram table** similar to the specification from the Extra Credit Project.

---

## 🧠 Overview

This project takes a text file containing a sequence of musical chords, separated into bars. It then:

1. **Tokenizes** the input (`lex.rs`)
2. **Parses** it using a recursive-descent parser (`parse.rs`)
3. **Validates** the structure according to the grammar
4. **Interprets** each chord using:
   - Chord qualities (Table I)
   - Suspensions (Table II)
   - Extensions (Table III)
   - Additions, alterations, and omissions
5. Computes each chord’s **pitch-class set** (0–11)
6. Produces a **pitch-class histogram table** matching the project specification.


---

## 🎼 Chord Calculator Capabilities

### ✔️ Root Interpretation  
Resolves every chord’s root into its numerical pitch class (0–11).

### ✔️ Quality & Suspensions  
Handles major, minor, augmented, diminished, power chords, unison, and suspensions (`sus2`, `sus4`, `sus24`).

### ✔️ Extensions & Additions  
Supports 6, 7, 9, 11, 13 (including raised/lowered forms), plus parenthesized additions that skip the implied 7th.

### ✔️ Alterations  
Handles `#` and `b` extensions such as `#11` or `b9`.

### ✔️ Omissions  
Chords may remove the 3rd or 5th using `no3`, `no5`, or `no35`.

### ✔️ Slash Chords / Inversions  
Processes bass-note modifiers like `G/B` and includes the bass pitch class when required.

### ✔️ Pitch-Class Histogram  
Generates the standard histogram table with:
- Header `0 1 2 ... A B`
- One row per chord
- `*` marking the pitch classes present
- Totals at the bottom

---

## 🧪 Testing

### Unit tests include:
- Chord parsing validation  
- Pitch-class correctness  
- Grammar conformance  
- Various chord types and combinations

### Integration tests include:
- Entire song parsing  
- Full pitch-class extraction  
- Total histogram correctness  



## 🚀 Usage
To run the project:

```bash
cargo run --release <path_to_input_file>
```

To run the test suite: 
```bash
cargo test --release -- --nocapture
```

## Project Structure

src/
  lex.rs        # tokenizer
  parse.rs      # recursive descent parser
  ast.rs        # abstract syntax tree structures
  calc.rs       # pitch-class calculations
  table.rs      # histogram and table printing

tests/
  integration_tests.rs
  cases/

