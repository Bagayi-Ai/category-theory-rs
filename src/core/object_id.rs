use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::Add;

#[derive(Debug, Clone, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum ObjectId {
    Int(i32),
    Str(String),
    Bool(bool),
}

impl Add for ObjectId {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        use ObjectId::*;
        match (self, other) {
            (Int(a), Int(b)) => Int(a + b),
            (Str(a), Str(b)) => Str(a + &b),
            (Bool(a), Bool(b)) => Bool(a || b),
            _ => panic!("Cannot add different types of ObjectId"),
        }
    }
}

impl PartialEq for ObjectId {
    fn eq(&self, other: &Self) -> bool {
        use ObjectId::*;
        match (self, other) {
            (Int(a), Int(b)) => a == b,
            (Str(a), Str(b)) => a == b,
            (Bool(a), Bool(b)) => a == b,
            _ => false,
        }
    }
}

impl Display for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectId::Int(v) => write!(f, "{}", v),
            ObjectId::Str(v) => write!(f, "{}", v),
            ObjectId::Bool(v) => write!(f, "{}", v),
        }
    }
}

impl PartialEq<str> for ObjectId {
    fn eq(&self, other: &str) -> bool {
        matches!(self, ObjectId::Str(v) if v == other)
    }
}
impl PartialEq<&str> for ObjectId {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<i32> for ObjectId {
    fn eq(&self, other: &i32) -> bool {
        matches!(self, ObjectId::Int(v) if v == other)
    }
}

// Optional: enables 1 == dynamic_value
impl PartialEq<ObjectId> for i32 {
    fn eq(&self, other: &ObjectId) -> bool {
        other == self
    }
}

impl From<i32> for ObjectId {
    fn from(value: i32) -> Self {
        ObjectId::Int(value)
    }
}

impl From<String> for ObjectId {
    fn from(value: String) -> Self {
        ObjectId::Str(value)
    }
}

impl From<&str> for ObjectId {
    fn from(value: &str) -> Self {
        ObjectId::Str(value.to_string())
    }
}
