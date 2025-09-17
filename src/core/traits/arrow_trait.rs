use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::traits::category_trait::{CategorySubObjectAlias, CategoryTrait};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

pub trait ArrowTrait<SourceObject: CategoryTrait, TargetObject: CategoryTrait>: Eq + Hash {
    fn source_object(&self) -> &Rc<SourceObject>;

    fn target_object(&self) -> &Rc<TargetObject>;

    fn new_instance(
        source: Rc<SourceObject>,
        target: Rc<TargetObject>,
        id: &str,
        mappings: HashMap<Rc<SourceObject::Morphism>, Rc<TargetObject::Morphism>>,
    ) -> Self
    where
        Self: Sized;

    fn new(
        id: String,
        source: Rc<SourceObject>,
        target: Rc<TargetObject>,
        mappings: HashMap<Rc<SourceObject::Morphism>, Rc<TargetObject::Morphism>>,
    ) -> Self
    where
        Self: Sized;

    fn new_with_mappings(
        source_object: Rc<SourceObject>,
        target_object: Rc<TargetObject>,
        mappings: HashMap<Rc<SourceObject::Morphism>, Rc<TargetObject::Morphism>>,
    ) -> Self
    where
        Self: Sized,
    {
        Self::new(String::generate(), source_object, target_object, mappings)
    }

    fn is_identity(&self) -> bool;

    fn arrow_id(&self) -> &String;

    fn compose(
        &self,
        other: &impl ArrowTrait<SourceObject, TargetObject>,
    ) -> Result<Rc<impl ArrowTrait<SourceObject, TargetObject>>, Errors>;

    // for handling composition of arrows
    // for single arrow just return itself
    fn arrows(&self) -> Vec<&impl ArrowTrait<SourceObject, TargetObject>>;

    fn arrow_mappings(&self) -> &HashMap<Rc<SourceObject::Morphism>, Rc<TargetObject::Morphism>>;

    fn validate_composition(&self) -> Result<(), Errors> {
        todo!()
    }

    fn validate_commutation(
        &self,
        other: &impl ArrowTrait<SourceObject, TargetObject>,
    ) -> Result<(), Errors> {
        todo!()
    }

    fn validate_mappings(&self) -> Result<(), Errors>;

    fn is_isomorphism(&self) -> bool {
        todo!()
    }
}
