use crate::message::Message;

// ─── FrameCodec — length-prefixed framing ────────────────────────────────────
//
// Wire format:  [ u32 BE length (4 bytes) ][ message bytes ]
//
// Hint:
//   encode → Message::encode(), prepend 4-byte big-endian length of that result.
//   decode → read prefix, validate remaining length, call Message::decode().

pub struct FrameCodec;

impl FrameCodec {
    /// Wrap `msg` in a 4-byte length-prefixed frame.
    pub fn encode(msg: &Message) -> Vec<u8> {
        todo!()
    }

    /// Decode the next framed message from `buf`.
    /// Returns `Some((message, bytes_consumed))` or `None` if data is incomplete.
    pub fn decode(buf: &[u8]) -> Option<(Message, usize)> {
        todo!()
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::{Message, MessageType};

    fn msg(seq: u32, payload: &[u8]) -> Message {
        Message::new(MessageType::Request, seq, payload.to_vec())
    }

    #[test]
    fn test_encode_has_4_byte_prefix() {
        let frame = FrameCodec::encode(&msg(1, b"data"));
        let declared = u32::from_be_bytes([frame[0], frame[1], frame[2], frame[3]]) as usize;
        assert_eq!(declared, frame.len() - 4);
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let original        = msg(99, b"oops");
        let frame           = FrameCodec::encode(&original);
        let (decoded, used) = FrameCodec::decode(&frame).unwrap();
        assert_eq!(decoded, original);
        assert_eq!(used, frame.len());
    }

    #[test]
    fn test_decode_incomplete_prefix_is_none() {
        assert_eq!(FrameCodec::decode(&[0, 0]),    None);
        assert_eq!(FrameCodec::decode(&[]),         None);
    }

    #[test]
    fn test_decode_missing_body_is_none() {
        // prefix says 10 bytes but nothing follows
        assert_eq!(FrameCodec::decode(&[0, 0, 0, 10]), None);
    }

    #[test]
    fn test_sequential_decode() {
        let m1 = msg(1, b"first");
        let m2 = msg(2, b"second");
        let mut stream = FrameCodec::encode(&m1);
        stream.extend(FrameCodec::encode(&m2));

        let (d1, n1) = FrameCodec::decode(&stream).unwrap();
        let (d2, n2) = FrameCodec::decode(&stream[n1..]).unwrap();
        assert_eq!(d1, m1);
        assert_eq!(d2, m2);
        assert_eq!(n1 + n2, stream.len());
    }
}
