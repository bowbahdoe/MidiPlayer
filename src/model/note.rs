use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub enum Of { A, B, C, D, E, F, G }

#[derive(Debug, PartialEq)]
pub enum Accidental {
    Natural,
    Sharp,
    Flat
}

pub type Octave = u8;

#[derive(Debug)]
pub struct Note {
    note: Of,
    accidental: Accidental,
    octave: Octave
}

impl Note {
    pub fn new(note: Of, accidental: Accidental, octave: Octave) -> Note {
        Note {
            note, accidental, octave
        }
    }
    
    /// Converts the note to its midi int representation using the table
    /// at this site. Assumes that the note was created such that its value is a valid 7 bit unsigned integer
    /// http://www.electronics.dit.ie/staff/tscarff/Music_technology/midi/midi_note_numbers_for_octaves.htm
    pub fn to_int(&self) -> u8 {
        use self::Of::{A, B, C, D, E, F, G};
        let mut val = match self.note {
            A => 9,
            B => 11,
            C => 0,
            D => 2,
            E => 4,
            F => 5,
            G => 7,
        };

        val += 12 * self.octave;

        if self.accidental == Accidental::Sharp {
            val += 1;
        }
        else if self.accidental == Accidental::Flat {
            val -= 1;
        }

        val as u8
    }

    fn valid_octave(octave: Octave) -> bool {
        match octave {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 => true,
            _ => false
        }
    }
}

impl PartialEq for Note {
    fn eq(&self, other: &Note) -> bool {
        self.to_int() == other.to_int()
    }
}

pub struct NoteEvent {
    pub note: Note,
    pub length: u32,
}
