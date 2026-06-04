pub const MAGIC: &[u8] = b"MIGNO";

pub struct Header {
    pub version: u16,
    pub nonce: [u8; 24],
}
