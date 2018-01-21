use model::chunk::Chunk;

#[derive(Debug)]
enum TrackDivision {
    Msb0 { ticks_per_quarter_note: u16 },
    Msb1 { frames_per_second: u8,
           ticks_per_frame: u8 },
}

#[derive(Debug)]
pub struct Header {
    format: u16,
    tracks: u16,
    division: TrackDivision,
}

// Parses the given chunk as a header chunk
pub fn parse_initial_header(chunk: &Chunk) -> Option<Header> {
    if chunk.is_header() && chunk.length() >= 6 && chunk.data().len() >= 6 {
        let d = chunk.data();
        let format   = ((d[0] as u16) << 8) + (d[1] as u16);
        let tracks   = ((d[2] as u16) << 8) + (d[3] as u16);
        let division = ((d[4] as u16) << 8) + (d[5] as u16);
        let division = parse_division(division);
        Some(Header { format, tracks, division })
    }
    else {
        None
    }
}

fn parse_division(raw_div: u16) -> TrackDivision {
    let msb = raw_div >> 15;
    if msb == 1 {
        TrackDivision::Msb1 {
            frames_per_second: ((raw_div >> 8) as i8).wrapping_neg() as u8,
            ticks_per_frame: raw_div as u8,
        }
    }
    else {
        TrackDivision::Msb0 {
            ticks_per_quarter_note: raw_div
        }
    }
}

pub fn is_valid_format(header: &Header) -> bool {
    match header.format { 0 | 1 | 2 => true, _ => false}
}

pub fn parse_chunks(chunks: &Vec<Chunk>) {

}
