use std::fs::File;
use std::io;
use std::io::prelude::*;

use model::chunk::Chunk;

/// Class for parsing a midi file
pub struct MidiParser {
    chunks: Vec<Chunk>,
}

impl MidiParser {
    /// Creates a new MidiParser from a file object
    pub fn new(midi_file: File) -> Option<MidiParser> {
        if let Ok(chunks) = Self::read_chunks(midi_file) {
            Some(MidiParser {chunks})
        }
        else {
            None
        }
    }

    // Returns a vector of the header Chunks
    pub fn header_chunks(&self) -> Vec<&Chunk> {
        self.chunks.iter()
                   .filter(|chunk| chunk.of_type(['M', 'T', 'h', 'd']))
                   .collect()
    }

    // Returns a vector of the track Chunks
    pub fn track_chunks(&self) -> Vec<&Chunk> {
        self.chunks.iter()
                   .filter(|chunk| chunk.of_type(['M', 'T', 'r', 'k']))
                   .collect()
    }

    pub fn chunks(&self) -> &Vec<Chunk> {
        &self.chunks
    }

    /// Reads the file as chunks
    fn read_chunks(f: File) -> Result<Vec<Chunk>, io::Error>  {
        let mut bytes = f.bytes();
        let mut chunks = Vec::new();

        'read_loop : loop {
            let mut chunk_type: [u8; 4] = [0, 0, 0, 0];
            for i in 0..4 {
                match bytes.next() {
                    Some(byte) => { chunk_type[i] = byte?; },
                    None => { break 'read_loop; },
                };
            };

            let mut len: u32 = 0;
            for i in 0..4 {
                match bytes.next() {
                    Some(byte) => { len += (byte? as u32) << (24 - i * 8); },
                    None => { break 'read_loop; },
                };
            };

            let mut byte_array = Vec::new();
            for _ in 0..len {
                match bytes.next() {
                    Some(byte) => { byte_array.push(byte?); },
                    None => { break 'read_loop; },
                };
            };

            chunks.push(Chunk::new(chunk_type, len, byte_array));
        };

        Ok(chunks)
    }
}
