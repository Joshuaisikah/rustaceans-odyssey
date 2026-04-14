// ─── network-protocol ────────────────────────────────────────────────────────
//
// A custom binary protocol with length-prefixed framing.
//
// Crate layout
// ┌────────────────────────────────────────────────────────┐
// │ message/          Wire-format types                    │
// │   types.rs     →  MessageType (u8 discriminant)       │
// │   header.rs    →  Header (10-byte fixed prefix)       │
// │   body.rs      →  Message (header + payload)          │
// ├────────────────────────────────────────────────────────┤
// │ codec/            Serialization / framing              │
// │   frame.rs     →  FrameCodec (length-prefix framing)  │
// └────────────────────────────────────────────────────────┘

pub mod codec;
pub mod message;

pub use codec::FrameCodec;
pub use message::{Header, Message, MessageType};
