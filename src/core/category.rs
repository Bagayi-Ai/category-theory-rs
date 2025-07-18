use std::collections::{HashSet};
use std::hash::Hash;

pub trait CategoryObjectIdTrait: Eq + Hash + Clone {
    type IdType: Eq + Hash + Clone;

    fn new() -> Self;

}

pub struct CategoryObject<Id, Value>
where
    Id: CategoryObjectIdTrait,
{
    pub id: Id,
    pub value: Value,
}

impl<Id, Value> CategoryObject<Id, Value>
where
    Id: CategoryObjectIdTrait,
    Value: Eq + Hash + Clone,
{
    pub fn new(id: Id, value: Value) -> Self {
        CategoryObject { id, value: value }
    }
}

pub trait MorphismIdTrait: Eq + Hash + Clone {
    type IdType: Eq + Hash + Clone;

    fn new(id: Self::IdType) -> Self;

    fn id(&self) -> &Self::IdType;

    fn compose(&self, other: &Self) -> Self;

    fn decompose(&self) -> Option<(Self, Self)>;

    fn is_composite(&self) -> bool {
        self.decompose().is_some()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Morphism<MorphismId: MorphismIdTrait, CategoryObjectId: Eq + Hash + Clone> {
    pub id: MorphismId,
    pub source_id: CategoryObjectId,
    pub target_id: CategoryObjectId,
    pub is_identity: bool,
}

impl<MorphismId: MorphismIdTrait, CategoryObjectId: Eq + Hash + Clone>
    Morphism<MorphismId, CategoryObjectId>
{
    pub fn new(
        id: MorphismId,
        source_id: CategoryObjectId,
        target_id: CategoryObjectId,
        is_identity: bool,
    ) -> Self {
        Morphism {
            id,
            source_id,
            target_id,
            is_identity,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CategoryError<CategoryObjectId, MorphismId> {
    ObjectNotFound(CategoryObjectId),
    MorphismNotFound(MorphismId),
    InvalidComposition,
}

pub trait Category {
    type CategoryObjectId: CategoryObjectIdTrait;

    type CategoryObjectValue: Eq + Hash + Clone;
    type MorphismId: MorphismIdTrait;

    fn get_object(
        &self,
        id: &Self::CategoryObjectId,
    ) -> Option<&CategoryObject<Self::CategoryObjectId, Self::CategoryObjectValue>>;

    fn add_object_with_id(&mut self, id: Self::CategoryObjectId, object: Self::CategoryObjectValue);

    fn add_object(&mut self, object: Self::CategoryObjectValue);

    fn get_object_morphisms(
        &self,
        id: &Self::CategoryObjectId,
    ) -> HashSet<&Morphism<Self::MorphismId, Self::CategoryObjectId>>;

    fn get_object_targets(
        &self,
        id: &Self::CategoryObjectId,
    ) -> HashSet<&Self::CategoryObjectId>;

    fn get_object_sources(
        &self,
        id: &Self::CategoryObjectId,
    ) -> HashSet<&Self::CategoryObjectId>;

    fn get_morphism(
        &self,
        id: &Self::MorphismId,
    ) -> Option<&Morphism<Self::MorphismId, Self::CategoryObjectId>>;

    fn add_morphism(
        &mut self,
        id: Self::MorphismId,
        source_id: Self::CategoryObjectId,
        target_id: Self::CategoryObjectId,
    ) -> Result<(), CategoryError<Self::CategoryObjectId, Self::MorphismId>>;

    fn morphisms_commute(
        &self,
        left: &Self::MorphismId,
        right: &Self::MorphismId,
    ) -> bool;
}
