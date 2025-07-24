use std::fmt::{Debug, Display};
use std::hash::Hash;
use uuid::Uuid;

pub trait Identifier: Clone + Eq + Hash + Debug + Display {
    type Id: Eq + Hash + Clone + Debug;

    fn id(&self) -> &Self::Id;

    fn generate() -> Self;
}


impl Identifier for String {
    type Id = String;

    fn id(&self) -> &Self::Id {
        self
    }

    fn generate() -> Self::Id {
        Uuid::new_v4().to_string()
    }
}