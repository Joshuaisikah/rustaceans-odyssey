// ─── Value — the atomic unit of data ─────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Text(String),
    Boolean(bool),
    Null,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{i}"),
            Value::Float(v)   => write!(f, "{v}"),
            Value::Text(s)    => write!(f, "{s}"),
            Value::Boolean(b) => write!(f, "{b}"),
            Value::Null       => write!(f, "NULL"),
        }
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_integer() { assert_eq!(Value::Integer(42).to_string(), "42"); }

    #[test]
    fn test_display_null() { assert_eq!(Value::Null.to_string(), "NULL"); }

    #[test]
    fn test_display_boolean() {
        assert_eq!(Value::Boolean(true).to_string(),  "true");
        assert_eq!(Value::Boolean(false).to_string(), "false");
    }

    #[test]
    fn test_equality() {
        assert_eq!(Value::Integer(1), Value::Integer(1));
        assert_ne!(Value::Integer(1), Value::Integer(2));
        assert_ne!(Value::Integer(1), Value::Null);
    }

    #[test]
    fn test_clone() {
        let v = Value::Text("hello".into());
        assert_eq!(v.clone(), v);
    }
}
