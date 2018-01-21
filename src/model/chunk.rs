use std::fmt::Display;
use std::fmt;

// Class representing the chunks of MIDI data
#[derive(Clone, Debug)]
pub struct Chunk {
    type_: [u8; 4],
    length: u32,
    data: Vec<u8>,
}

impl Chunk {
    pub fn new(type_: [u8; 4], length: u32, data: Vec<u8>) -> Chunk {
        Chunk {
            type_,
            length,
            data
        }
    }

    // Returns if this chunk is the passed in type
    pub fn of_type(&self, type_: [char; 4]) -> bool {
        self.type_ == Self::type_from_chars(type_)
    }

    // Returns if this is a header chunk
    pub fn is_header(&self) -> bool {
        self.of_type(['M', 'T', 'h', 'd'])
    }

    // Returns if this is a track chunk
    pub fn is_track(&self) -> bool {
        self.of_type(['M', 'T', 'r', 'k'])
    }

    // Returns the length of this chunk
    pub fn length(&self) -> u32 {
        self.length
    }

    // Returns the data stored in this chunk
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    // Returns a midi Chunk type header from an array of chars
    fn type_from_chars(chars: [char; 4]) -> [u8; 4] {
        [chars[0] as u8, chars[1] as u8, chars[2] as u8, chars[3] as u8]
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let t = self.type_;
        write!(f,
               "[{}{}{}{}]: length: {} bytes",
                t[0] as char,
                t[1] as char,
                t[2] as char,
                t[3] as char,
                self.length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_types() {
        let chunk = Chunk { type_ : [77,
                                     84,
                                     114,
                                     107],
                            length: 0,
                            data: Vec::new()};
        assert!(chunk.of_type(['M', 'T', 'r', 'k']));
        assert!(!chunk.of_type(['M', 'T', 'h', 'd']));
        assert!(chunk.is_track());
        assert!(!chunk.is_header());
    }
}
