use super::types::MessageType;

// ─── Header — 10-byte wire prefix ────────────────────────────────────────────
//
// Byte layout (big-endian):
//   [0]     version        u8
//   [1]     message_type   u8
//   [2..6]  sequence       u32 BE
//   [6..10] payload_length u32 BE

#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub version:        u8,
    pub message_type:   MessageType,
    pub sequence:       u32,
    pub payload_length: u32,
}

impl Header {
    pub const SIZE: usize = 10;

    /// Append the 10-byte big-endian encoding to `buf`.
    pub fn encode(&self, buf: &mut Vec<u8>) {
        todo!()
    }

    /// Parse from the first 10 bytes of `buf`.
    /// Returns None if the slice is too short or the type byte is unknown.
    pub fn decode(buf: &[u8]) -> Option<Self> {
        todo!()
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Header {
        Header {
            version:        1,
            message_type:   MessageType::Request,
            sequence:       0x0102_0304,
            payload_length: 0x0A0B_0C0D,
        }
    }

    #[test]
    fn test_encode_produces_exactly_10_bytes() {
        let mut buf = Vec::new();
        sample().encode(&mut buf);
        assert_eq!(buf.len(), Header::SIZE);
    }

    #[test]
    fn test_encode_byte_order() {
        let mut buf = Vec::new();
        sample().encode(&mut buf);
        assert_eq!(buf[0], 1);   // version
        assert_eq!(&buf[2..6],  &[0x01, 0x02, 0x03, 0x04]); // sequence BE
        assert_eq!(&buf[6..10], &[0x0A, 0x0B, 0x0C, 0x0D]); // length BE
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let original = sample();
        let mut buf  = Vec::new();
        original.encode(&mut buf);
        assert_eq!(Header::decode(&buf), Some(original));
    }

    #[test]
    fn test_decode_short_buffer_is_none() {
        assert_eq!(Header::decode(&[1, 2, 3]), None);
        assert_eq!(Header::decode(&[]),        None);
    }

    #[test]
    fn test_decode_unknown_type_byte_is_none() {
        let mut buf = Vec::new();
        sample().encode(&mut buf);
        buf[1] = 0xFF; // corrupt the type byte
        assert_eq!(Header::decode(&buf), None);
    }
}
