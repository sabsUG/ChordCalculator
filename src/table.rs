use crate::calc::chord_to_pitch_classes;
use crate::ast::{Song, BarItem, Chord};

pub fn print_pitch_table(song: &Song) {
    let headers = ["0","1","2","3","4","5","6","7","8","9","A","B"];
    let mut totals = [0u32; 12];
    let mut chord_index = 1;

    println!("    {}", headers.join(" "));
    println!("    {}", "- ".repeat(12));

   for bar in &song.bars {
        for item in &bar.items {
            match item {
                BarItem::Chord(ch) => {
                    let pcs = chord_to_pitch_classes(ch);
                    let mut row = vec![" "; 12];

                    for p in pcs {
                        let idx = (p % 12) as usize;
                        row[idx] = "*";
                        totals[idx] += 1;
                    }

                    let name = chord_to_string(ch);
                    println!("{:>3}. {}  {}", chord_index, row.join(" "), name);
                    chord_index += 1;
                }
                BarItem::Repeat | BarItem::NC => continue, 
            }
        }
    }

    println!("    {}", "- ".repeat(12));
    print!("    ");
    for t in totals.iter() {
        print!("{:>2} ", t);
    }
    println!();
}

fn chord_to_string(ch: &Chord) -> String {
    let mut s = ch.root.letter.to_string();

    if let Some(acc) = &ch.root.acc {
        match acc {
            crate::ast::Accidental::Sharp => s.push('#'),
            crate::ast::Accidental::Flat => s.push('b'),
        }
    }

    if let Some(desc) = &ch.description {
        if let Some(q) = &desc.qual {
            use crate::ast::Qual::*;
            match q {
                Minus => s.push('-'),
                Plus => s.push('+'),
                LowerO => s.push('o'),
                Five => s.push('5'),
                One => s.push('1'),
            }
        }
        if let Some(qn) = &desc.qnum {
            if qn.hat { s.push('^'); }
            if let Some(n) = qn.n { s.push_str(&n.to_string()); }
            if let Some(ext) = &qn.ext {
                use crate::ast::Ext::*;
                match ext {
                    Nine => s.push_str("9"),
                    Eleven => s.push_str("11"),
                    Thirteen => s.push_str("13"),
                }
            }
        }
    }

    if let Some(bass) = &ch.bass {
        s.push('/');
        s.push_str(&bass.letter.to_string());
        if let Some(acc) = &bass.acc {
            match acc {
                crate::ast::Accidental::Sharp => s.push('#'),
                crate::ast::Accidental::Flat => s.push('b'),
            }
        }
    }

    s
}
