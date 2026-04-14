// ─── MessageType ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum MessageType {
    Request   = 1,
    Response  = 2,
    Heartbeat = 3,
    Error     = 4,
}

impl MessageType {
    /// Map a discriminant byte to a MessageType.  Returns None if unknown.
    pub fn from_u8(val: u8) -> Option<Self> {
        todo!()
    }

    pub fn as_u8(&self) -> u8 {
        todo!()
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip_all_variants() {
        for mt in [
            MessageType::Request,
            MessageType::Response,
            MessageType::Heartbeat,
            MessageType::Error,
        ] {
            assert_eq!(MessageType::from_u8(mt.as_u8()), Some(mt));
        }
    }

    #[test]
    fn test_unknown_byte_is_none() {
        assert_eq!(MessageType::from_u8(0),   None);
        assert_eq!(MessageType::from_u8(255), None);
    }

    #[test]
    fn test_discriminants_are_nonzero() {
        for mt in [
            MessageType::Request,
            MessageType::Response,
            MessageType::Heartbeat,
            MessageType::Error,
        ] {
            assert_ne!(mt.as_u8(), 0);
        }
    }
}
