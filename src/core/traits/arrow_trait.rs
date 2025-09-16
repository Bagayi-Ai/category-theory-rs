use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use crate::core::errors::Errors;
use crate::core::traits::category_trait::{CategorySubObjectAlias, CategoryTrait};
use std::rc::Rc;

pub trait ArrowTrait<SourceObject: CategoryTrait, TargetObject: CategoryTrait>: Eq + Hash
{
    fn source_object(&self) -> &Rc<SourceObject>;

    fn target_object(&self) -> &Rc<TargetObject>;

    fn is_identity(&self) -> bool;

    fn arrow_id(&self) -> &String;

    fn compose(
        &self,
        other: &impl ArrowTrait<SourceObject, TargetObject>,
    ) -> Result<Rc<impl ArrowTrait<SourceObject, TargetObject>>, Errors>;

    // for handling composition of arrows
    // for single arrow just return itself
    fn arrows(&self) -> Vec<&impl ArrowTrait<SourceObject, TargetObject>>;

    fn arrow_mappings(
        &self,
    ) -> &HashMap<
        Rc<SourceObject::Morphism>,
        Rc<TargetObject::Morphism>,
    >;

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
