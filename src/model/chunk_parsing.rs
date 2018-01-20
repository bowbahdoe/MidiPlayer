use model::chunk::Chunk;

#[derive(Debug)]
pub struct Header {
    format: u16,
    tracks: u16,
    division: u16,
}

// Parses the given chunk
pub fn parse_initial_header(chunk: &Chunk) -> Option<Header> {
    if chunk.is_header() && chunk.length() >= 6 && chunk.data().len() >= 6 {
        let d = chunk.data();
        Some(Header { format:   ((d[0] as u16) << 8) + (d[1] as u16),
                      tracks:   ((d[2] as u16) << 8) + (d[3] as u16),
                      division: ((d[4] as u16) << 8) + (d[5] as u16), })
    }
    else {
        None
    }
}
