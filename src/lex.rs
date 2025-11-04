#![allow(warnings)]
//to track the position of each token in the input
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub lo: usize, //lower bound
    pub hi: usize, //upper bound
}

#[derive(Debug, Clone, PartialEq, Eq)] //inherit from interfaces custom operators to interact easily with enum and structs
pub enum TokKind {
    Num(u16),         // e.g. 4, 11, 13 (used for meters or chord numbers)
    NoteLetter(char), // {A, B, C, D, E, F, G}
    Sharp,
    Flat, //<acc> which can be # or b
    Dash,
    Plus,
    LowerO, //<qual> "-", "+", "o"
    Caret,  //<qnum> "^"
    LParen,
    RParen, //<add> "(", ")"
    Sus2,
    Sus4,
    Sus24, //<sus> "sus2", "sus4", "sus24"
    No3,
    No5,
    No35,          //<omit> "no3", "no5", "no35"
    Slash,         //<bass> "/" or for <meter>
    EOF,           //End of file
    Bar,           // <bar> "|"
    NC,            //<chords> NC
    Percentage,    //<chords> %
    Unknown(char), //any unrecognized character (for error reporting)
}

//Each token consists of its kind, text and position span
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokKind,
    pub text: String,
    pub span: Span,
}

//Turn input text into Vec<Token>
pub fn tokenize(src: &str) -> Vec<Token> {
    //collect tokens into this vector
    let mut toks = vec![];

    //Convert input string to bytes
    let bytes = src.as_bytes();
    let mut i = 0usize; //current index
    let mut pos = 0usize;

    //helper to push the token into the Vector
    let mut push = |kind: TokKind, start: usize, end: usize, pos: &mut usize| {
        let len = end - start;
        toks.push(Token {
            kind,
            text: src[start..end].to_string(),
            span: Span {
                lo: *pos,
                hi: *pos + len,
            },
        });
        *pos += len;
    };

    // helpers to identify the character
    let peek = |idx: usize| -> Option<u8> {
        if idx < bytes.len() {
            Some(bytes[idx])
        } else {
            None
        }
    };
    let is_space = |b: u8| matches!(b, b' ' | b'\t' | b'\r' | b'\n');
    let is_digit = |b: u8| (b'0'..=b'9').contains(&b);
    //let is_note_letter = |b: u8| (b'A'..=b'G').contains(&b);
    let starts_with_at = |idx: usize, s: &str| -> bool {
        let end = idx + s.len();
        end <= bytes.len() && &src[idx..end] == s
    };

    while i < bytes.len() {
        //skip whitespace
        while i < bytes.len() && is_space(bytes[i]) {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }

        let start = i;
        let b = bytes[i];

        match b {
            // digits -> one numeric token (7, 11, 13, etc)
            b'0'..=b'9' => {
                i += 1;
                while i < bytes.len() && is_digit(bytes[i]) {
                    i += 1;
                }
                let val = src[start..i].parse::<u16>().unwrap_or(0);
                push(TokKind::Num(val), start, i, &mut pos);
            }

            //Note letters
            b'A'..=b'G' => {
                i += 1;
                push(TokKind::NoteLetter(b as char), start, i, &mut pos);
            }

            //single-char fixed symbols
            b'/' => {
                i += 1;
                push(TokKind::Slash, start, i, &mut pos);
            }
            b'%' => {
                i += 1;
                push(TokKind::Percentage, start, i, &mut pos);
            }
            b'#' => {
                i += 1;
                push(TokKind::Sharp, start, i, &mut pos);
            }
            b'b' => {
                i += 1;
                push(TokKind::Flat, start, i, &mut pos);
            }
            b'-' => {
                i += 1;
                push(TokKind::Dash, start, i, &mut pos);
            }
            b'+' => {
                i += 1;
                push(TokKind::Plus, start, i, &mut pos);
            }
            b'o' => {
                i += 1;
                push(TokKind::LowerO, start, i, &mut pos);
            }
            b'^' => {
                i += 1;
                push(TokKind::Caret, start, i, &mut pos);
            }
            b'(' => {
                i += 1;
                push(TokKind::LParen, start, i, &mut pos);
            }
            b')' => {
                i += 1;
                push(TokKind::RParen, start, i, &mut pos);
            }

            b'|' => {
                if matches!(peek(i + 1), Some(b'|')) {
                    i += 2;
                } else {
                    i += 1;
                }
                push(TokKind::Bar, start, i, &mut pos);
            }

            //keywords NC / sus / no
            b'N' => {
                if matches!(peek(i + 1), Some(b'C')) {
                    i += 2;
                    push(TokKind::NC, start, i, &mut pos);
                } else {
                    i += 1;
                    push(TokKind::Unknown('N'), start, i, &mut pos);
                }
            }

            b's' => {
                if starts_with_at(i, "sus24") {
                    i += 5;
                    push(TokKind::Sus24, start, i, &mut pos);
                } else if starts_with_at(i, "sus2") {
                    i += 4;
                    push(TokKind::Sus2, start, i, &mut pos);
                } else if starts_with_at(i, "sus4") {
                    i += 4;
                    push(TokKind::Sus4, start, i, &mut pos);
                } else {
                    i += 1;
                    push(TokKind::Unknown('s'), start, i, &mut pos);
                }
            }

            b'n' => {
                if starts_with_at(i, "no35") {
                    i += 4;
                    push(TokKind::No35, start, i, &mut pos);
                } else if starts_with_at(i, "no3") {
                    i += 3;
                    push(TokKind::No3, start, i, &mut pos);
                } else if starts_with_at(i, "no5") {
                    i += 3;
                    push(TokKind::No5, start, i, &mut pos);
                } else {
                    i += 1;
                    push(TokKind::Unknown('n'), start, i, &mut pos);
                }
            }

            //anything else -> unknown
            _ => {
                i += 1;
                push(TokKind::Unknown(b as char), start, i, &mut pos);
            }
        }
    }
    let end_of_file = pos;
    toks.push(Token {
        kind: TokKind::EOF,
        text: String::new(),
        span: Span {
            lo: end_of_file,
            hi: end_of_file,
        },
    });
    toks
}
