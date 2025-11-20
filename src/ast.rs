#![allow(warnings)]

use std::fmt;

impl fmt::Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Letter::A => "A",
                Letter::B => "B",
                Letter::C => "C",
                Letter::D => "D",
                Letter::E => "E",
                Letter::F => "F",
                Letter::G => "G",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Song {
    pub bars: Vec<Bar>,
}

#[derive(Debug, Clone)]
pub struct Bar {
    /// Optional per-bar meter, e.g., 4/4. Keep here if meter can change per bar.
    pub meter: Option<Meter>,
    /// Sequence of bar items (NC, %, or a single chord).
    pub items: Vec<BarItem>,
}

#[derive(Debug, Clone)]
pub struct Meter {
    pub numerator: u16,   // 1..=15
    pub denominator: u16, // {1,2,4,8,16}
}

#[derive(Debug, Clone)]
pub enum BarItem {
    NC,           // "NC"
    Repeat,       // "%"
    Chord(Chord), // chord
}

#[derive(Debug, Clone)]
pub struct Chord {
    pub root: Note,                       // required
    pub description: Option<Description>, // [qual][qnum][add][sus][omit]
    pub bass: Option<Note>,               // optional "/ <note>"
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Letter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Accidental {
    Sharp,
    Flat,
} // '#', 'b'

#[derive(Debug, Clone)]
pub struct Note {
    pub letter: Letter,          // A..G
    pub acc: Option<Accidental>, // # or b
}

/// Description bundles optionals. Enforce "qual & sus cannot coexist" in the parser.
#[derive(Debug, Clone)]
pub struct Description {
    pub qual: Option<Qual>, // "-", "+", "o", "5", "1"
    pub qnum: Option<Qnum>, // e.g., 6, 7, ^6, ^7, maybe with ext
    pub add: Option<Add>,   // "(" ... ")" or without parens
    pub sus: Option<Sus>,   // sus2/sus4/sus24
    pub omit: Option<Omit>, // no3/no5/no35
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Qual {
    Minus,
    Plus,
    LowerO,
    Five,
    One,
} // -, +, o, 5, 1

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ext {
    Nine,
    Eleven,
    Thirteen,
} // 9, 11, 13

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Qnum {
    pub hat: bool,        // was there a "^"?
    pub n: Option<u8>,    // 6 or 7 (extend later if needed)
    pub ext: Option<Ext>, // optional 9/11/13
}

/// add ::= alt | "(" alt ")"
/// alt ::= [acc] "5" | [acc] ext
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Add {
    Acc5(Option<Accidental>),        // (5), (b5), (#5)
    AccExt(Option<Accidental>, Ext), // (9),(11),(13) with optional accidental
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sus {
    Sus2,
    Sus4,
    Sus24,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Omit {
    No3,
    No5,
    No35,
}
