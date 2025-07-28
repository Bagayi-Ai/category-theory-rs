use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use uuid::Uuid;

pub trait ObjectId: Clone + Eq + Hash + Debug + Display {
    type Id: Eq + Hash + Clone + Debug;

    fn id(&self) -> &Self::Id;

}

pub trait Identifier: Clone + Eq + Hash + Debug + Display + ObjectId {
    fn generate() -> Self;
}


impl ObjectId for String {
    type Id = String;

    fn id(&self) -> &Self::Id {
        self
    }
}

impl Identifier for String {
    fn generate() -> Self {
        Uuid::new_v4().to_string()
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
    type Id = ();

    fn id(&self) -> &Self::Id {
        todo!()
    }
}