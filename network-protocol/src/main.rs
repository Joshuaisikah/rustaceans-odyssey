// ─── network-protocol — integration demo ─────────────────────────────────────
//
// Run with:  cargo run -p network-protocol

use network_protocol::{FrameCodec, Message, MessageType};

fn main() {
    println!("=== network-protocol integration demo ===\n");

    demo_message_encode_decode();
    demo_frame_codec();
    demo_multi_message_stream();

    println!("\nAll demos completed.");
}

// ── Demo 1: Message encode/decode ─────────────────────────────────────────────
fn demo_message_encode_decode() {
    println!("[ Demo 1 ] Message encode / decode");

    let payload  = b"Hello, protocol!".to_vec();
    let original = Message::new(MessageType::Request, 1, payload.clone());
    let bytes    = original.encode();

    println!("  encoded {} bytes (header={} + payload={})",
        bytes.len(), 10, payload.len());

    let decoded = Message::decode(&bytes).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.header.version, 1);
    assert_eq!(decoded.header.sequence, 1);
    assert_eq!(decoded.payload, payload);
    println!("  round-trip verified  ✓");

    // Truncation must return None.
    assert_eq!(Message::decode(&bytes[..5]), None);
    println!("  truncation returns None  ✓");
}

// ── Demo 2: FrameCodec ────────────────────────────────────────────────────────
fn demo_frame_codec() {
    println!("[ Demo 2 ] FrameCodec");

    let msg   = Message::new(MessageType::Response, 42, b"frame body".to_vec());
    let frame = FrameCodec::encode(&msg);

    let prefix_len = u32::from_be_bytes([frame[0], frame[1], frame[2], frame[3]]) as usize;
    println!("  frame size={}, prefix says inner={}", frame.len(), prefix_len);
    assert_eq!(prefix_len, frame.len() - 4);

    let (decoded, consumed) = FrameCodec::decode(&frame).expect("frame decode failed");
    assert_eq!(decoded,  msg);
    assert_eq!(consumed, frame.len());
    println!("  frame round-trip verified, consumed={consumed} bytes  ✓");
}

// ── Demo 3: streaming multiple messages ───────────────────────────────────────
fn demo_multi_message_stream() {
    println!("[ Demo 3 ] streaming multiple framed messages");

    let messages = vec![
        Message::new(MessageType::Request,   1, b"query".to_vec()),
        Message::new(MessageType::Response,  1, b"result".to_vec()),
        Message::new(MessageType::Heartbeat, 0, vec![]),
        Message::new(MessageType::Error,     2, b"not found".to_vec()),
    ];

    // Simulate sending all messages into one buffer (like a TCP stream).
    let mut stream: Vec<u8> = Vec::new();
    for msg in &messages {
        stream.extend(FrameCodec::encode(msg));
    }
    println!("  stream total bytes: {}", stream.len());

    // Simulate receiving: decode one message at a time.
    let mut cursor = 0;
    let mut received = Vec::new();
    while cursor < stream.len() {
        let (msg, consumed) = FrameCodec::decode(&stream[cursor..])
            .expect("incomplete frame in stream");
        received.push(msg);
        cursor += consumed;
    }

    assert_eq!(received.len(), messages.len());
    for (i, (got, want)) in received.iter().zip(messages.iter()).enumerate() {
        assert_eq!(got, want, "message {i} mismatch");
    }
    println!("  all {} messages decoded correctly  ✓", received.len());
}
