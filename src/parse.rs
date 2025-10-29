#![allow(warnings)]
use crate::lex::{Token, TokKind, Span};
use TokKind::*;
use crate::ast::*;
use crate::ast::Bar as AstBar;
use std::cell::Cell;


// Global or thread-local indentation tracker
thread_local! {
    static INDENT: Cell<usize> = Cell::new(0);
}

// Pretty indentation string
fn indent() -> String {
    INDENT.with(|lvl| "  ".repeat(lvl.get()))
}

// Helper: print enter/exit trace for parse functions
fn trace_enter(name: &str) {
    if cfg!(debug_assertions) {
        INDENT.with(|lvl| {
            println!("{}→ Enter {}", indent(), name);
            lvl.set(lvl.get() + 1);
        });
    }
}

fn trace_exit(name: &str) {
    if cfg!(debug_assertions) {
        INDENT.with(|lvl| {
            lvl.set(lvl.get().saturating_sub(1));
            println!("{}← Exit {}", indent(), name);
        });
    }
}


#[derive(Debug, Clone)]
pub struct ParseError {pub msg: String, pub span: Span}
type PResult<T> = Result<T, ParseError>;

pub fn parse_song(tokens: &[Token]) -> PResult<Song> {
    let mut p = Parser { toks: tokens, pos: 0};
    p.parse_song()
}
pub fn show_error_span(src: &str, span: &Span) {
    let lo = span.lo as usize;
    let hi = span.hi as usize;
    let snippet = &src[lo.min(src.len()) .. hi.min(src.len())];
    println!("Span {:?} → {:?}", span, snippet);
}
struct Parser<'a> { toks: &'a [Token], pos:usize }

impl <'a> Parser<'a> {
    //Small helpers
    fn peek(&self) -> Token { self.toks.get(self.pos).unwrap().clone() }
    fn is_at_end(&self) -> bool { matches!(self.peek().kind, EOF) }
    fn advance(&mut self) {
        if !self.is_at_end() {
            if cfg!(debug_assertions) {
                println!("{}[advance] {:?}", indent(), self.peek().kind); 
            }
            self.pos+= 1; 
        }
    }
    fn expect(&mut self, want:TokKind, what: &str) -> PResult<()> {
        if self.peek().kind == want {self.advance(); Ok(()) }
        else { Err(ParseError{ msg: what.into(), span: self.peek().span.clone() }) }
    }

    //song ::= bar {bar} "|"
    fn parse_song(&mut self) -> PResult<Song> {
        trace_enter("parse_song");
        let mut bars = Vec::new();

        if self.is_at_end(){return Ok(Song {bars}); }
        bars.push(self.parse_bar()?);

        while self.peek().kind == TokKind::Bar {
            self.advance();
            if !matches!(self.peek().kind, Bar | EOF){
                bars.push(self.parse_bar()?);
            }
        }
        trace_exit("parse_song");
        Ok(Song {bars })
    }

    // bar := [meter] chords "|"
    fn parse_bar(&mut self) -> PResult<AstBar>{
        trace_enter("parse_bar");
        let meter = self.parse_meter_opt()?;
        let mut items = Vec::new();

        while !matches!(self.peek().kind, TokKind::Bar | EOF) {
            items.push(self.parse_bar_item()?);
        }
        trace_exit("parse_bar");
        Ok(AstBar { meter, items})
    }

    //meter ::= numerator "/" denominator
    fn parse_meter_opt(&mut self) -> PResult<Option<Meter>>{
        trace_enter("parse_meter_opt");
        if let Num(_) = self.peek().kind {
            let num = self.read_num_in(1..=15, "invalid numerator (1..=15")?;
            self.expect(Slash, "expected '/' in meter")?;
            let den = self.read_num_set(&[1, 2, 4, 8, 16], "invalid denominator (1,2,4,8,16)")?;
            trace_exit("parse_meter_opt");
            Ok(Some(Meter{ numerator: num as u16, denominator: den as u16 }))
        } else {
            trace_exit("parse_meter_opt");
            Ok(None)
        }
    }

    fn read_num_in(&mut self, range: std::ops::RangeInclusive<u16>, msg: &str) -> PResult<u16> {
        let t = self.peek();
        if let Num(n) = t.kind {
            if range.contains(&n) {self.advance(); Ok(n) }
            else { Err(ParseError{ msg: msg.into(), span: t.span.clone() })}
        } else { Err(ParseError{ msg: "expected number".into(), span: t.span.clone() })}
    }

    fn read_num_set(&mut self, set: &[u16], msg: &str) -> PResult<u16> {
        let t = self.peek();
        if let Num(n) = t.kind {
            if set.contains(&n) { self.advance(); Ok(n) }
            else { Err(ParseError{ msg: msg.into(), span: t.span.clone() }) }
        } else { Err(ParseError{ msg: "expected number".into(), span: t.span.clone() }) }
    }

    //chords ::= NC | % | chord {chord}
    fn parse_bar_item(&mut self) -> PResult<BarItem> {
        trace_enter("parse_bar_item");
        match self.peek().kind {
            NC => {
                self.advance(); 
                trace_exit("parse_bar_item");
                Ok(BarItem::NC) 
            }
            Percentage => {
                self.advance(); 
                trace_exit("parse_bar_item");
                Ok(BarItem::Repeat)}
            _ => {
                trace_exit("parse_bar_item");
                Ok(BarItem::Chord(self.parse_chord()? ))
            },
        }
    }

    //chord ::= root [description] [bass] , and root := note, and bass ::= note "/"
    fn parse_chord(&mut self) -> PResult<Chord> {
        trace_enter("parse_chord");
        let root = self.parse_note()?;
        let description = self.parse_description_opt()?;
        let bass = if self.peek().kind == Slash {
            self.advance();
            Some(self.parse_note()?)
        } else {None};

        if let Some(desc) = &description  {
            if desc.qual.is_some() && desc.sus.is_some(){
                return Err(ParseError {msg: "qual and sus cannot coexist".into(), span: self.peek().span.clone() });
            }
        }
        trace_exit("parse_chord");
        Ok(Chord{root, description, bass})
    }

    //note ::= letter [acc]
    fn parse_note(&mut self) -> PResult<Note> {
        trace_enter("parse_note");
        let letter = self.parse_letter()?;
        let acc = self.parse_acc_opt()?;
        trace_exit("parse_note");
        Ok(Note {letter, acc})
    }

    fn parse_letter(&mut self) -> PResult<Letter> {
        trace_enter("parse_letter");
        let t = self.peek();
        let l = match t.kind {
            NoteLetter('A') => Letter::A,
            NoteLetter('B') => Letter::B,
            NoteLetter('C') => Letter::C,
            NoteLetter('D') => Letter::D,
            NoteLetter('E') => Letter::E,
            NoteLetter('F') => Letter::F,
            NoteLetter('G') => Letter::G,
            _ => return Err (ParseError{
                msg: "expected note letter A..G".into(),
                span: t.span.clone(),
            })
        };
        self.advance();
        trace_exit("parse_letter");
        Ok(l)
    }

    fn parse_acc_opt(&mut self) -> PResult<Option<Accidental>> {
        trace_enter("parse_acc_opt");
        let acc = self.peek();
        let a = match acc.kind {
            Sharp => {self.advance(); Some(Accidental::Sharp)},
            Flat => {self.advance(); Some(Accidental::Flat)},
            _ => None
        };
        trace_exit("parse_acc_opt");
        Ok(a)
    }

    //description ::= [qual] [qnum] [add] [sus] [omit]
    fn parse_description_opt(&mut self) -> PResult<Option<Description>> {
        trace_enter("parse_description_opt");
        let t = self.peek();
        // Only start a description if the next token can begin one
        if !matches!(
            t.kind,
            TokKind::Dash
                | TokKind::Plus
                | TokKind::LowerO
                | TokKind::Num(_)
                | TokKind::Caret
                | TokKind::LParen
                | TokKind::Sus2
                | TokKind::Sus4
                | TokKind::Sus24
                | TokKind::No3
                | TokKind::No5
                | TokKind::No35
        ) {
            trace_exit("parse_description_opt");
            return Ok(None);
        }
        let qual = self.parse_qual_opt()?;
        let qnum = self.parse_qnum_opt()?;
        let add = self.parse_add_opt()?;
        let sus = self.parse_sus_opt()?;
        let omit = self.parse_omit_opt()?;

        if qual.is_none() && qnum.is_none() && add.is_none() && sus.is_none() && omit.is_none(){
            trace_exit("parse_description_opt");
            return Ok(None);
        }
        trace_exit("parse_description_opt");
        Ok(Some (Description {qual, qnum, add, sus, omit}))
    }

    //qual ::= "-" | "+" | "o" | "5" | "1"
    fn parse_qual_opt(&mut self) -> PResult<Option<Qual>> {
        trace_enter("parse_qual_opt");
        let qual = self.peek();
        let q = match qual.kind {
            TokKind::Dash => {self.advance(); Some(Qual::Minus)},
            TokKind::Plus => {self.advance(); Some(Qual::Plus)},
            TokKind::LowerO => {self.advance(); Some(Qual::LowerO)},
            TokKind::Num(5) => {self.advance(); Some(Qual::Five)},
            TokKind::Num(1) => {self.advance(); Some(Qual::One)},
            _ => None
        };
        trace_exit("parse_qual_opt");
        Ok(q)
    }

    //qnum ::= "6" | [" ^ "] "7" | [" ^ "] ext
    fn parse_qnum_opt(&mut self) -> PResult<Option<Qnum>> {
        trace_enter("parse_qnum_opt");
        let t = self.peek();
        let mut hat = false;
    
        // Case "6"
        if matches!(t.kind, TokKind::Num(6)) {
            self.advance();
            trace_exit("parse_qnum_opt");
            return Ok(Some(Qnum { hat, n: Some(6), ext: None }));
        }
    
        // Case starts with '^'
        if matches!(t.kind, TokKind::Caret) {
            // Peek ahead before consuming
            let next = self.toks.get(self.pos + 1).cloned();
    
            // If there’s no next or it's not 7/9/11/13 -> do not advance
            if let Some(next) = next {
                match next.kind {
                    TokKind::Num(7) => {
                        self.advance(); // consume '^'
                        self.advance(); // consume 7
                        trace_exit("parse_qnum_opt");
                        return Ok(Some(Qnum { hat: true, n: Some(7), ext: None }));
                    }
                    TokKind::Num(9) | TokKind::Num(11) | TokKind::Num(13) => {
                        self.advance(); // consume '^'
                        let ext = self.parse_ext()?;
                        trace_exit("parse_qnum_opt");
                        return Ok(Some(Qnum { hat: true, n: None, ext: Some(ext) }));
                    }
                    _ => {
                        trace_exit("parse_qnum_opt");
                        return Ok(None)
                    }, // leave position unchanged
                }
            }
            trace_exit("parse_qnum_opt");
            return Ok(None); // '^' at EOF
        }
        else{
            if matches!(t.kind, TokKind::Num(7)) {
                self.advance(); //consume 7
                return Ok(Some(Qnum { hat: false, n: Some(7), ext: None }));
            }
            else if matches!(t.kind, TokKind::Num(9)){
                self.advance(); //consume 9
                return Ok(Some(Qnum { hat: false, n: Some(9), ext: None }));
            }
            else if matches!(t.kind, TokKind::Num(11)){
                self.advance(); //consume 11
                return Ok(Some(Qnum { hat: false, n: Some(11), ext: None }));
            }
            else if matches!(t.kind, TokKind::Num(13)){
                self.advance(); //consume 13
                return Ok(Some(Qnum { hat: false, n: Some(13), ext: None }));
            }
        }
        trace_exit("parse_qnum_opt");
        Ok(None)
    }

    // add := alt | "(" alt ")"
    // alt := [acc] "5" | [acc] ext
    fn parse_add_opt(&mut self) -> PResult<Option<Add>> {
        trace_enter("parse_add_opt");
        if matches!(self.peek().kind, TokKind::LParen){
            self.advance();
            let alt = self.parse_alt()?;
            self.expect(TokKind::RParen, "expected ')' after alt")?;
            trace_exit("parse_add_opt");
            return Ok(Some(alt))
        };

        if (matches!(self.peek().kind, TokKind::Flat) || 
           matches!(self.peek().kind, TokKind::Sharp) ||
           matches!(self.peek().kind, TokKind::Num(5)) ||
           matches!(self.peek().kind, TokKind::Num(9)) ||
           matches!(self.peek().kind, TokKind::Num(11)) || 
           matches!(self.peek().kind, TokKind::Num(13))) {
            
            let alt = self.parse_alt()?;
            trace_exit("parse_add_opt");
            return Ok(Some(alt));
        }

        trace_exit("parse_add_opt");
        return Ok(None);  
    }

    //alt := [acc] "5" | [acc] ext
    fn parse_alt(&mut self) -> PResult<Add> {
        trace_enter("parse_alt");
        let acc = self.parse_acc_opt()?;
        
        let t = self.peek();

        if matches!(t.kind, TokKind::Num(5)){
            self.advance();
            trace_exit("parse_alt");
            return Ok(Add::Acc5(acc));
        }
        
        if matches!(t.kind, TokKind::Num(9)) || matches!(t.kind, TokKind::Num(11)) || matches!(t.kind, TokKind::Num(13)){
            let ext = self.parse_ext()?;
            trace_exit("parse_alt");
            return Ok(Add::AccExt(acc, ext));
        }  
        return Err(ParseError{msg: "Expected 5 or ext after acc".into(), span: t.span.clone()});
    }

    //ext := "9" | "11" | "13"
    fn parse_ext(&mut self) -> PResult<Ext> {
        trace_enter("parse_ext");
        let t = self.peek();

        let n = match t.kind {
            TokKind::Num(9) => {self.advance(); Ext::Nine},
            TokKind::Num(11) => {self.advance(); Ext::Eleven},
            TokKind::Num(13) => {self.advance(); Ext::Thirteen},
            _ => return Err (ParseError{
                msg: "expected 9, 11 or 13".into(),
                span: t.span.clone(),
            })
        };
        trace_exit("parse_ext");
        Ok(n)
    }

    //sus := sus2 | sus4 | sus24
    fn parse_sus_opt(&mut self) -> PResult<Option<Sus>> {
        trace_enter("parse_sus_opt");
        let t = self.peek();

        let sus = match t.kind {
            TokKind::Sus2 => {self.advance(); Some(Sus::Sus2)},
            TokKind::Sus4 => {self.advance(); Some(Sus::Sus4)},
            TokKind::Sus24 => {self.advance(); Some(Sus::Sus24)},
            _ => None,
        };
        trace_exit("parse_sus_opt");
        Ok(sus)
    }

    //omit := "no3" | "no5" | "no35"
    fn parse_omit_opt(&mut self) -> PResult<Option<Omit>> {
        trace_enter("parse_omit_opt");
        let t = self.peek();
        
        let om = match t.kind {
            TokKind::No3 => {self.advance(); Some(Omit::No3)},
            TokKind::No5 => {self.advance(); Some(Omit::No5)},
            TokKind::No35 => {self.advance(); Some(Omit::No35)},
            _ => None,
        };
        trace_exit("parse_omit_opt");
        Ok(om)
    }

}