// ─── network-protocol: integration tests ─────────────────────────────────────
//
// Black-box tests through the public API.
// Exercises the full pipeline: MessageType → Header → Message → FrameCodec.

use network_protocol::{FrameCodec, Header, Message, MessageType};

// ── MessageType ───────────────────────────────────────────────────────────────

#[test]
fn test_all_message_types_roundtrip_through_u8() {
    for mt in [
        MessageType::Request,
        MessageType::Response,
        MessageType::Heartbeat,
        MessageType::Error,
    ] {
        let byte = mt.as_u8();
        assert_ne!(byte, 0, "discriminants must be non-zero");
        assert_eq!(MessageType::from_u8(byte), Some(mt));
    }
}

#[test]
fn test_unknown_u8_returns_none() {
    assert_eq!(MessageType::from_u8(0),   None);
    assert_eq!(MessageType::from_u8(255), None);
}

// ── Header encode / decode ────────────────────────────────────────────────────

#[test]
fn test_header_encodes_to_exactly_10_bytes() {
    let h = Header {
        version:        1,
        message_type:   MessageType::Request,
        sequence:       42,
        payload_length: 128,
    };
    let mut buf = Vec::new();
    h.encode(&mut buf);
    assert_eq!(buf.len(), Header::SIZE);
}

#[test]
fn test_header_encode_decode_roundtrip() {
    let original = Header {
        version:        1,
        message_type:   MessageType::Response,
        sequence:       0xDEAD_BEEF,
        payload_length: 512,
    };
    let mut buf = Vec::new();
    original.encode(&mut buf);
    assert_eq!(Header::decode(&buf), Some(original));
}

#[test]
fn test_header_decode_too_short_is_none() {
    assert_eq!(Header::decode(&[1, 2, 3]), None);
    assert_eq!(Header::decode(&[]),        None);
}

#[test]
fn test_header_big_endian_byte_order() {
    let h = Header {
        version:        1,
        message_type:   MessageType::Request,
        sequence:       0x0102_0304,
        payload_length: 0x0A0B_0C0D,
    };
    let mut buf = Vec::new();
    h.encode(&mut buf);
    assert_eq!(buf[0], 1); // version
    assert_eq!(&buf[2..6],  &[0x01, 0x02, 0x03, 0x04]); // sequence BE
    assert_eq!(&buf[6..10], &[0x0A, 0x0B, 0x0C, 0x0D]); // payload_length BE
}

// ── Message encode / decode ───────────────────────────────────────────────────

#[test]
fn test_message_new_sets_version_and_length() {
    let payload = b"hello".to_vec();
    let msg     = Message::new(MessageType::Request, 1, payload.clone());
    assert_eq!(msg.header.version, 1);
    assert_eq!(msg.header.payload_length, payload.len() as u32);
}

#[test]
fn test_message_encode_decode_roundtrip() {
    let original = Message::new(MessageType::Heartbeat, 7, (0u8..32).collect());
    let bytes    = original.encode();
    assert_eq!(Message::decode(&bytes), Some(original));
}

#[test]
fn test_message_empty_payload_roundtrip() {
    let msg   = Message::new(MessageType::Heartbeat, 0, vec![]);
    let bytes = msg.encode();
    let dec   = Message::decode(&bytes).unwrap();
    assert!(dec.payload.is_empty());
}

#[test]
fn test_message_decode_truncated_is_none() {
    let msg   = Message::new(MessageType::Request, 1, b"payload".to_vec());
    let bytes = msg.encode();
    assert_eq!(Message::decode(&bytes[..Header::SIZE]), None);
}

#[test]
fn test_message_encode_length_is_header_plus_payload() {
    let payload = b"test data".to_vec();
    let msg     = Message::new(MessageType::Response, 5, payload.clone());
    let bytes   = msg.encode();
    assert_eq!(bytes.len(), Header::SIZE + payload.len());
}

// All four message types survive encode→decode.
#[test]
fn test_all_message_types_survive_encode_decode() {
    for mt in [
        MessageType::Request,
        MessageType::Response,
        MessageType::Heartbeat,
        MessageType::Error,
    ] {
        let original = Message::new(mt, 100, b"body".to_vec());
        let decoded  = Message::decode(&original.encode()).unwrap();
        assert_eq!(decoded, original);
    }
}

// ── FrameCodec ────────────────────────────────────────────────────────────────

#[test]
fn test_frame_encode_has_4_byte_length_prefix() {
    let msg   = Message::new(MessageType::Request, 1, b"data".to_vec());
    let frame = FrameCodec::encode(&msg);
    let declared = u32::from_be_bytes([frame[0], frame[1], frame[2], frame[3]]) as usize;
    assert_eq!(declared, frame.len() - 4);
}

#[test]
fn test_frame_encode_decode_roundtrip() {
    let original = Message::new(MessageType::Response, 42, b"response body".to_vec());
    let frame    = FrameCodec::encode(&original);
    let (decoded, consumed) = FrameCodec::decode(&frame).unwrap();
    assert_eq!(decoded, original);
    assert_eq!(consumed, frame.len());
}

#[test]
fn test_frame_decode_incomplete_returns_none() {
    assert_eq!(FrameCodec::decode(&[]),          None);
    assert_eq!(FrameCodec::decode(&[0, 0]),      None);
    assert_eq!(FrameCodec::decode(&[0, 0, 0, 10]), None); // prefix only, no body
}

// Encode two messages into a single byte stream and decode them sequentially.
#[test]
fn test_frame_sequential_decode_of_stream() {
    let m1 = Message::new(MessageType::Request,   1, b"first".to_vec());
    let m2 = Message::new(MessageType::Response,  2, b"second".to_vec());
    let m3 = Message::new(MessageType::Heartbeat, 3, vec![]);

    let mut stream = FrameCodec::encode(&m1);
    stream.extend(FrameCodec::encode(&m2));
    stream.extend(FrameCodec::encode(&m3));

    let (d1, n1) = FrameCodec::decode(&stream).unwrap();
    let (d2, n2) = FrameCodec::decode(&stream[n1..]).unwrap();
    let (d3, n3) = FrameCodec::decode(&stream[n1 + n2..]).unwrap();

    assert_eq!(d1, m1);
    assert_eq!(d2, m2);
    assert_eq!(d3, m3);
    assert_eq!(n1 + n2 + n3, stream.len());
}

// Frame with large payload survives codec.
#[test]
fn test_frame_large_payload_roundtrip() {
    let payload: Vec<u8> = (0u8..=255).cycle().take(4096).collect();
    let original         = Message::new(MessageType::Request, 999, payload);
    let frame            = FrameCodec::encode(&original);
    let (decoded, _)     = FrameCodec::decode(&frame).unwrap();
    assert_eq!(decoded, original);
}
