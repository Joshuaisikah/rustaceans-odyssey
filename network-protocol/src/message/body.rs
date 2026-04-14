use super::header::Header;
use super::types::MessageType;

// ─── Message — header + payload ──────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub header:  Header,
    pub payload: Vec<u8>,
}

impl Message {
    /// Construct a v1 message; `payload_length` is set automatically.
    pub fn new(msg_type: MessageType, sequence: u32, payload: Vec<u8>) -> Self {
        todo!()
    }

    /// Encode as header bytes followed by payload bytes.
    pub fn encode(&self) -> Vec<u8> {
        todo!()
    }

    /// Decode from a byte slice.
    /// Returns None if data is incomplete or the header is invalid.
    pub fn decode(buf: &[u8]) -> Option<Self> {
        todo!()
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sets_version_and_payload_length() {
        let payload = b"hello".to_vec();
        let msg     = Message::new(MessageType::Request, 1, payload.clone());
        assert_eq!(msg.header.version, 1);
        assert_eq!(msg.header.payload_length, payload.len() as u32);
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let original = Message::new(MessageType::Heartbeat, 7, (0u8..32).collect());
        let bytes    = original.encode();
        assert_eq!(Message::decode(&bytes), Some(original));
    }

    #[test]
    fn test_empty_payload_roundtrip() {
        let msg   = Message::new(MessageType::Heartbeat, 0, vec![]);
        let bytes = msg.encode();
        let dec   = Message::decode(&bytes).unwrap();
        assert!(dec.payload.is_empty());
    }

    #[test]
    fn test_decode_truncated_is_none() {
        let msg   = Message::new(MessageType::Request, 1, b"full payload".to_vec());
        let bytes = msg.encode();
        assert_eq!(Message::decode(&bytes[..Header::SIZE]), None);
    }

    #[test]
    fn test_encode_length_matches_payload() {
        let payload = b"testing".to_vec();
        let msg     = Message::new(MessageType::Response, 99, payload.clone());
        let bytes   = msg.encode();
        assert_eq!(bytes.len(), Header::SIZE + payload.len());
    }
}
