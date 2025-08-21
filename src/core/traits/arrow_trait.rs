use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::traits::category_trait::CategoryTrait;
use std::rc::Rc;

pub trait ArrowTrait {
    type SourceObject: CategoryTrait;

    type TargetObject: CategoryTrait;

    fn source_object(&self) -> &Rc<Self::SourceObject>;

    fn target_object(&self) -> &Rc<Self::TargetObject>;

    fn is_identity(&self) -> bool;

    fn compose(
        &self,
        other: &impl ArrowTrait,
    ) -> Result<
        impl ArrowTrait<SourceObject = Self::SourceObject, TargetObject = Self::TargetObject>,
        Errors,
    >;

    // for handling composition of arrows
    // for single arrow just return itself
    fn arrows(
        &self,
    ) -> Vec<&impl ArrowTrait<SourceObject = Self::SourceObject, TargetObject = Self::TargetObject>>;

    fn validate_composition(&self) -> Result<(), Errors> {
        todo!()
    }

    fn validate_commutation(&self, other: &impl ArrowTrait) -> Result<(), Errors> {
        todo!()
    }

    fn is_isomorphism(&self) -> bool {
        todo!()
    }
}
