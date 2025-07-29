use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use uuid::Uuid;

pub trait ObjectId: Clone + Eq + Hash + Debug + Display {
    type Id: Eq + Hash + Clone + Debug;

    fn object_id(&self) -> &Self::Id;

    fn generate() -> Self;

}

pub trait Identifier: Clone + Eq + Hash + Debug + Display + ObjectId {
    fn generateadad() -> Self;
}


impl ObjectId for String {
    type Id = String;

    fn object_id(&self) -> &Self::Id {
        self
    }

    fn generate() -> Self {
        Uuid::new_v4().to_string()
    }
}

impl Identifier for String {
    fn generateadad() -> Self {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitObject <T: Display> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Display + Debug> Display for UnitObject<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl <T: Clone + Eq + Hash + Debug + Display> ObjectId for UnitObject<T> {
    type Id = T;

    fn object_id(&self) -> &Self::Id {
        todo!()
    }

    fn generate() -> Self {
        todo!()
    }
}