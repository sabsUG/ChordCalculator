use crate::ast::*;

pub fn analyze_song(song: &Song) {
    for (i, bar) in song.bars.iter().enumerate() {
        println!("Bar {}:", i+1);
        for item in &bar.items {
            if let BarItem::Chord(ch) = item {
                let pitch_classes = chord_to_pitch_classes(ch);
                println!("  {:?} -> {:?}", ch, pitch_classes);
            }
        }
    }
}

pub fn chord_to_pitch_classes(chord: &Chord) -> Vec<u8> {
    let root_pc = note_to_pc(&chord.root);
    let mut intervals = base_intervals(chord.description.as_ref().and_then(|d| d.qual));

    if let Some(desc) = &chord.description {
        intervals = apply_qnum(intervals, desc.qnum.as_ref());
        intervals = apply_add(intervals, desc.add.as_ref());
        intervals = apply_sus(intervals, desc.sus.as_ref());
        intervals = apply_omit(intervals, desc.omit.as_ref());
    }

    let mut pitch_classes = to_pitch_classes(root_pc, &intervals);

    if let Some(bass) = &chord.bass {
        let bass_pc = note_to_pc(bass);
        pitch_classes.insert(0, bass_pc);
    }

    pitch_classes
}

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
        Some(Accidental::Sharp) => (base+1) % 12,
        Some(Accidental::Flat) => (base + 11) %12,
        None => base,
    }
}

fn base_intervals(qual: Option<Qual>) -> Vec<u8> {
    match qual {
        Some(Qual::Minus) => vec![0, 3, 7],
        Some(Qual::Plus) => vec![0, 4, 8],
        Some(Qual::LowerO) => vec![0, 3, 6],
        Some(Qual::Five) => vec![0, 7],
        Some(Qual::One) => vec![0],
        _=> vec![0, 4, 7], //default major
    }
}

fn apply_qnum(mut intervals: Vec<u8>, qnum: Option<&Qnum>) -> Vec<u8> {
    if let Some(qn) = qnum {
        if let Some(n) = qn.n {
            match n {
                6 => intervals.push(9),
                7 => intervals.push(10),
                _=> {}
            }
        }
        if qn.hat {
            intervals.push(11);
        }
        if let Some(ext) = &qn.ext {
            match ext {
                Ext::Nine => intervals.push(2),
                Ext::Eleven => intervals.push(5),
                Ext::Thirteen => intervals.push(9),
            }
        }
    }
    intervals.sort();
    intervals.dedup();
    intervals
}

fn apply_add(mut intervals: Vec<u8>, add: Option<&Add>) -> Vec<u8> {
    if let Some(a) = add {
        match a {
            Add::Acc5(_) => intervals.push(7),
            Add::AccExt(_, ext) => match ext {
                Ext::Nine => intervals.push(2),
                Ext::Eleven => intervals.push(5),
                Ext::Thirteen => intervals.push(9),
            },
        }
    }
    intervals.sort();
    intervals.dedup();
    intervals
}

fn apply_sus(mut intervals: Vec<u8>, sus: Option<&Sus>) -> Vec<u8> {
    if sus.is_none() { return intervals; }

    intervals.retain( |&i| i != 3 && i != 4);

    match sus.unwrap() {
        Sus::Sus2 => intervals.push(2),
        Sus::Sus4 => intervals.push(5),
        Sus::Sus24 => { intervals.push(2); intervals.push(5); },
    }

    intervals.sort();
    intervals.dedup();
    intervals
}

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

fn to_pitch_classes(root_pc: u8, intervals: &[u8]) -> Vec<u8> {
    let mut pcs: Vec<u8> = intervals.iter().map(|i| (root_pc+i) % 12).collect();
    pcs.sort();
    pcs.dedup();
    pcs
}