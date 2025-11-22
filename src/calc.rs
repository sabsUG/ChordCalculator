use crate::ast::*;

pub fn analyze_song(song: &Song) {
    for bar in song.bars.iter() {
        //println!("Bar {}:", i + 1);
        for item in &bar.items {
            if let BarItem::Chord(ch) = item {
                let _pcs = chord_to_pitch_classes(ch);
                //println!("  {:?} -> {:?}", ch, pcs);
            }
        }
    }
}

// ---------------------------------------------------------
// Main chord evaluation
// ---------------------------------------------------------

pub fn chord_to_pitch_classes(chord: &Chord) -> Vec<u8> {
    let mut desc = chord.description.clone();

    // Handle “5” quality as power chord if no explicit qual was given
    if let Some(d) = &mut desc {
        if d.qual.is_none() {
            if let Some(q) = &d.qnum {
                if q.n == Some(5) {
                    d.qual = Some(Qual::Five);
                }
            }
        }
    }

    // 1. Start with base triad intervals (Table I)
    let mut intervals = base_intervals(desc.as_ref().and_then(|d| d.qual));

    // 2. Apply extensions (#, 6, 7, 9, 11, 13) (Table III)
    if let Some(d) = &desc {
        intervals = apply_qnum(intervals, d.qnum.as_ref());

        // 3. Additions ((9), (#11), etc.)
        intervals = apply_add(intervals, d.add.as_ref());

        // 4. Suspensions (sus2, sus4, sus24)
        intervals = apply_sus(intervals, d.sus.as_ref());

        // 5. Omissions (no3, no5, no35)
        intervals = apply_omit(intervals, d.omit.as_ref());
    }

    // Convert to pitch classes
    let root_pc = note_to_pc(&chord.root);
    let mut pcs = to_pitch_classes(root_pc, &intervals);

    // Inversion: add bass if needed
    if let Some(bass) = &chord.bass {
        let bass_pc = note_to_pc(bass);
        if !pcs.contains(&bass_pc) {
            pcs.insert(0, bass_pc);
        }
    }

    pcs
}

// ---------------------------------------------------------
// ROOT → pitch class
// ---------------------------------------------------------

fn note_to_pc(note: &Note) -> u8 {
    let base = match note.letter {
        Letter::C => 0,
        Letter::D => 2,
        Letter::E => 4,
        Letter::F => 5,
        Letter::G => 7,
        Letter::A => 9,
        Letter::B => 11,
    };

    match note.acc {
        Some(Accidental::Sharp) => (base + 1) % 12,
        Some(Accidental::Flat) => (base + 11) % 12,
        None => base,
    }
}

// ---------------------------------------------------------
// QUALITIES
// ---------------------------------------------------------

fn base_intervals(qual: Option<Qual>) -> Vec<u8> {
    match qual {
        Some(Qual::Minus) => vec![0, 3, 7],  // minor
        Some(Qual::Plus) => vec![0, 4, 8],   // augmented
        Some(Qual::LowerO) => vec![0, 3, 6], // diminished
        Some(Qual::Five) => vec![0, 7],      // power chord
        Some(Qual::One) => vec![0],          // unison
        _ => vec![0, 4, 7],                  // major
    }
}

// ---------------------------------------------------------
// EXTENSIONS
// ---------------------------------------------------------

fn apply_qnum(mut intervals: Vec<u8>, qnum: Option<&Qnum>) -> Vec<u8> {
    if let Some(qn) = qnum {
        // n = 6 or 7
        if let Some(n) = qn.n {
            match n {
                6 => intervals.push(9), // sixth
                7 => {
                    // Seventh
                    let seventh = if qn.hat { 11 } else { 10 };
                    intervals.push(seventh);
                }
                9 => {
                    intervals.push(10); // b7
                    intervals.push(2); // 9th
                }
                13 => {
                    intervals.push(10); // b7
                    intervals.push(9); // 13th
                }
                _ => {}
            }
        }

        // Extensions (9, 11, 13)
        if let Some(ext) = &qn.ext {
            let seventh = if qn.hat { 11 } else { 10 };
            intervals.push(seventh);

            match ext {
                Ext::Nine => intervals.push(2),
                Ext::Eleven => intervals.push(5),
                Ext::Thirteen => intervals.push(9),
            }
        }

        // Remove 7th when parentheses-numbered extension
        // is present — handled in apply_add
    }

    intervals.sort();
    intervals.dedup();
    intervals
}

// ---------------------------------------------------------
// ADDITIONS
// ---------------------------------------------------------

fn apply_add(mut intervals: Vec<u8>, add: Option<&Add>) -> Vec<u8> {
    if let Some(a) = add {
        match a {
            Add::Acc5(acc) => {
                let mut fifth = 7;
                if let Some(Accidental::Sharp) = acc {
                    fifth = 8;
                }
                if let Some(Accidental::Flat) = acc {
                    fifth = 6;
                }
                intervals.push(fifth);
            }

            Add::AccExt(acc, ext) => {
                let (mut iv, _include_seventh) = match ext {
                    Ext::Nine => (2, false),
                    Ext::Eleven => (5, false),
                    Ext::Thirteen => (9, false),
                };

                // apply accidental
                if let Some(Accidental::Sharp) = acc {
                    iv = (iv + 1) % 12;
                }
                if let Some(Accidental::Flat) = acc {
                    iv = (iv + 11) % 12;
                }

                intervals.push(iv);
            }
        }
    }

    intervals.sort();
    intervals.dedup();
    intervals
}

// ---------------------------------------------------------
//  SUSPENSIONS
// ---------------------------------------------------------

fn apply_sus(mut intervals: Vec<u8>, sus: Option<&Sus>) -> Vec<u8> {
    if let Some(s) = sus {
        // remove 3rd
        intervals.retain(|&i| i != 3 && i != 4);

        match s {
            Sus::Sus2 => intervals.push(2),
            Sus::Sus4 => intervals.push(5),
            Sus::Sus24 => {
                intervals.push(2);
                intervals.push(5);
            }
        }
    }

    intervals.sort();
    intervals.dedup();
    intervals
}

// ---------------------------------------------------------
// OMISSIONS
// ---------------------------------------------------------

fn apply_omit(mut intervals: Vec<u8>, omit: Option<&Omit>) -> Vec<u8> {
    if let Some(o) = omit {
        match o {
            Omit::No3 => intervals.retain(|&i| i != 3 && i != 4),
            Omit::No5 => intervals.retain(|&i| i != 7),
            Omit::No35 => intervals.retain(|&i| i != 3 && i != 4 && i != 7),
        }
    }
    intervals
}

// ---------------------------------------------------------
// CONVERSION TO PITCH CLASSES
// ---------------------------------------------------------

fn to_pitch_classes(root_pc: u8, intervals: &[u8]) -> Vec<u8> {
    let mut pcs: Vec<u8> = intervals.iter().map(|i| (root_pc + i) % 12).collect();

    pcs.sort();
    pcs.dedup();
    pcs
}
