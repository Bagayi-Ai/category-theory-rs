use std::fmt::Display;

#[derive(Debug, Clone, Eq, Hash)]
pub enum DynamicValue {
    Int(i32),
    Str(String),
    Bool(bool),
}

impl PartialEq for DynamicValue {
    fn eq(&self, other: &Self) -> bool {
        use DynamicValue::*;
        match (self, other) {
            (Int(a), Int(b)) => a == b,
            (Str(a), Str(b)) => a == b,
            (Bool(a), Bool(b)) => a == b,
            _ => false,
        }
    }
}

impl Display for DynamicValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DynamicValue::Int(v) => write!(f, "{}", v),
            DynamicValue::Str(v) => write!(f, "{}", v),
            DynamicValue::Bool(v) => write!(f, "{}", v),
        }
    }
}

impl PartialEq<str> for DynamicValue {
    fn eq(&self, other: &str) -> bool {
        matches!(self, DynamicValue::Str(v) if v == other)
    }
}
impl PartialEq<&str> for DynamicValue {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<i32> for DynamicValue {
    fn eq(&self, other: &i32) -> bool {
        matches!(self, DynamicValue::Int(v) if v == other)
    }
}

// Optional: enables 1 == dynamic_value
impl PartialEq<DynamicValue> for i32 {
    fn eq(&self, other: &DynamicValue) -> bool {
        other == self
    }
}

impl From<i32> for DynamicValue {
    fn from(value: i32) -> Self {
        DynamicValue::Int(value)
    }
}

impl From<String> for DynamicValue {
    fn from(value: String) -> Self {
        DynamicValue::Str(value)
    }
}

impl From<&str> for DynamicValue {
    fn from(value: &str) -> Self {
        DynamicValue::Str(value.to_string())
    }
}
