use std::fmt::Debug;
use std::hash::Hash;

pub trait GenericObjectIdTrait: Eq + Hash + Clone + Debug {
    fn new() -> Self;
}