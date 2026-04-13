// ─── Message types ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum MessageType {
    Request  = 1,
    Response = 2,
    Heartbeat = 3,
    Error    = 4,
}

impl MessageType {
    pub fn from_u8(val: u8) -> Option<Self> {
        todo!()
    }

    pub fn as_u8(&self) -> u8 {
        todo!()
    }
}

// ─── Header  (10 bytes: version[1] + type[1] + sequence[4] + length[4]) ──────

#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub version: u8,
    pub message_type: MessageType,
    pub sequence: u32,
    pub payload_length: u32,
}

impl Header {
    pub const SIZE: usize = 10;

    /// Append the 10-byte big-endian binary representation to `buf`.
    pub fn encode(&self, buf: &mut Vec<u8>) {
        todo!()
    }

    /// Parse a header from the first 10 bytes of `buf`. Returns `None` if
    /// the slice is too short or the message-type byte is unknown.
    pub fn decode(buf: &[u8]) -> Option<Self> {
        todo!()
    }
}

// ─── Message ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub header: Header,
    pub payload: Vec<u8>,
}

impl Message {
    /// Construct a v1 message with `payload_length` set automatically.
    pub fn new(msg_type: MessageType, sequence: u32, payload: Vec<u8>) -> Self {
        todo!()
    }

    /// Encode as header bytes followed by payload bytes.
    pub fn encode(&self) -> Vec<u8> {
        todo!()
    }

    /// Decode from a byte slice.  Returns `None` if the data is incomplete or
    /// the header is invalid.
    pub fn decode(buf: &[u8]) -> Option<Self> {
        todo!()
    }
}

// ─── Frame codec  (4-byte big-endian length prefix + message bytes) ──────────

pub struct FrameCodec;

impl FrameCodec {
    /// Wrap an encoded message with a 4-byte length prefix.
    pub fn encode(msg: &Message) -> Vec<u8> {
        todo!()
    }

    /// Decode the next framed message from `buf`.
    /// Returns `Some((message, bytes_consumed))` or `None` if data is
    /// incomplete.
    pub fn decode(buf: &[u8]) -> Option<(Message, usize)> {
        todo!()
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // MessageType round-trips through u8.
    #[test]
    fn test_message_type_as_u8_roundtrip() {
        for mt in [
            MessageType::Request,
            MessageType::Response,
            MessageType::Heartbeat,
            MessageType::Error,
        ] {
            let byte = mt.as_u8();
            assert_eq!(MessageType::from_u8(byte), Some(mt));
        }
    }

    // An unknown byte must return None.
    #[test]
    fn test_message_type_unknown_byte_is_none() {
        assert_eq!(MessageType::from_u8(0), None);
        assert_eq!(MessageType::from_u8(255), None);
    }

    // Header encodes to exactly 10 bytes in big-endian order.
    #[test]
    fn test_header_encode_size_and_byte_order() {
        let h = Header {
            version: 1,
            message_type: MessageType::Request,
            sequence: 0x0102_0304,
            payload_length: 0x0A0B_0C0D,
        };
        let mut buf = Vec::new();
        h.encode(&mut buf);

        assert_eq!(buf.len(), Header::SIZE);
        assert_eq!(buf[0], 1);                // version
        assert_eq!(buf[1], MessageType::Request.as_u8()); // type
        assert_eq!(&buf[2..6], &[0x01, 0x02, 0x03, 0x04]); // sequence big-endian
        assert_eq!(&buf[6..10], &[0x0A, 0x0B, 0x0C, 0x0D]); // length big-endian
    }

    // Header decode round-trips with encode.
    #[test]
    fn test_header_encode_decode_roundtrip() {
        let original = Header {
            version: 2,
            message_type: MessageType::Response,
            sequence: 42,
            payload_length: 100,
        };
        let mut buf = Vec::new();
        original.encode(&mut buf);
        let decoded = Header::decode(&buf).expect("decode failed");
        assert_eq!(decoded, original);
    }

    // Header decode returns None when the buffer is too short.
    #[test]
    fn test_header_decode_returns_none_on_short_buffer() {
        assert_eq!(Header::decode(&[1, 2, 3]), None);
        assert_eq!(Header::decode(&[]), None);
    }

    // Message::new sets payload_length automatically.
    #[test]
    fn test_message_new_sets_payload_length() {
        let payload = b"hello".to_vec();
        let msg = Message::new(MessageType::Request, 1, payload.clone());
        assert_eq!(msg.header.payload_length, payload.len() as u32);
        assert_eq!(msg.header.version, 1);
    }

    // Message encode/decode round-trip preserves all fields.
    #[test]
    fn test_message_encode_decode_roundtrip() {
        let payload: Vec<u8> = (0..32).collect();
        let original = Message::new(MessageType::Heartbeat, 7, payload);
        let bytes = original.encode();
        let decoded = Message::decode(&bytes).expect("decode failed");
        assert_eq!(decoded, original);
    }

    // Message with empty payload works correctly.
    #[test]
    fn test_message_with_empty_payload() {
        let msg = Message::new(MessageType::Heartbeat, 0, vec![]);
        let bytes = msg.encode();
        let decoded = Message::decode(&bytes).unwrap();
        assert!(decoded.payload.is_empty());
    }

    // Message decode returns None when bytes are incomplete.
    #[test]
    fn test_message_decode_returns_none_on_truncated_data() {
        let msg = Message::new(MessageType::Request, 1, b"full payload".to_vec());
        let bytes = msg.encode();
        // Provide only the header (truncate the payload)
        let truncated = &bytes[..Header::SIZE];
        assert_eq!(Message::decode(truncated), None);
    }

    // FrameCodec encode produces a 4-byte length prefix then the message.
    #[test]
    fn test_frame_encode_has_length_prefix() {
        let msg = Message::new(MessageType::Request, 1, b"data".to_vec());
        let frame = FrameCodec::encode(&msg);
        let inner_len = u32::from_be_bytes([frame[0], frame[1], frame[2], frame[3]]) as usize;
        assert_eq!(inner_len, frame.len() - 4);
    }

    // FrameCodec decode round-trip.
    #[test]
    fn test_frame_encode_decode_roundtrip() {
        let msg = Message::new(MessageType::Error, 99, b"oops".to_vec());
        let frame = FrameCodec::encode(&msg);
        let (decoded, consumed) = FrameCodec::decode(&frame).expect("decode failed");
        assert_eq!(decoded, msg);
        assert_eq!(consumed, frame.len());
    }

    // FrameCodec decode returns None when the buffer is too short.
    #[test]
    fn test_frame_decode_none_on_incomplete_data() {
        assert_eq!(FrameCodec::decode(&[0, 0, 0, 10]), None); // says 10 bytes but none follow
        assert_eq!(FrameCodec::decode(&[0, 0]), None);        // even the length prefix is truncated
    }

    // Multiple frames can be decoded sequentially from a buffer.
    #[test]
    fn test_frame_multiple_sequential_messages() {
        let m1 = Message::new(MessageType::Request,  1, b"first".to_vec());
        let m2 = Message::new(MessageType::Response, 2, b"second".to_vec());

        let mut buf = FrameCodec::encode(&m1);
        buf.extend(FrameCodec::encode(&m2));

        let (d1, n1) = FrameCodec::decode(&buf).unwrap();
        let (d2, n2) = FrameCodec::decode(&buf[n1..]).unwrap();

        assert_eq!(d1, m1);
        assert_eq!(d2, m2);
        assert_eq!(n1 + n2, buf.len());
    }
}
