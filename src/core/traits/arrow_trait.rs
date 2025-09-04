use crate::core::errors::Errors;
use crate::core::traits::category_trait::CategoryTrait;
use std::rc::Rc;

pub trait ArrowTrait<SourceObject: CategoryTrait, TargetObject: CategoryTrait> {
    fn source_object(&self) -> &Rc<SourceObject>;

    fn target_object(&self) -> &Rc<TargetObject>;

    fn is_identity(&self) -> bool;

    fn compose(
        &self,
        other: &impl ArrowTrait<SourceObject, TargetObject>,
    ) -> Result<Rc<impl ArrowTrait<SourceObject, TargetObject>>, Errors>;

    // for handling composition of arrows
    // for single arrow just return itself
    fn arrows(&self) -> Vec<&impl ArrowTrait<SourceObject, TargetObject>>;

    fn validate_composition(&self) -> Result<(), Errors> {
        todo!()
    }

    fn validate_commutation(
        &self,
        other: &impl ArrowTrait<SourceObject, TargetObject>,
    ) -> Result<(), Errors> {
        todo!()
    }

    fn is_isomorphism(&self) -> bool {
        todo!()
    }
}
