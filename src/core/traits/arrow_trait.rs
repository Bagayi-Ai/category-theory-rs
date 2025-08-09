use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::traits::category_trait::CategoryTrait;

pub trait ArrowTrait<'a> {
    type SourceObject: CategoryTrait<'a>;

    type TargetObject: CategoryTrait<'a>;

    type Identifier: Identifier;

    fn arrow_id(&self) -> &Self::Identifier;

    fn source_object(&self) -> &Self::SourceObject;

    fn target_object(&self) -> &Self::TargetObject;

    fn is_identity(&self) -> bool;

    // for handling composition of arrows
    // for single arrow just return itself
    fn arrows(
        &self,
    ) -> Vec<
        &impl ArrowTrait<
            'a,
            SourceObject = Self::SourceObject,
            TargetObject = Self::TargetObject,
            Identifier = Self::Identifier,
        >,
    >;

    fn validate_composition(&self) -> Result<(), Errors> {
        todo!()
    }

    fn validate_commutation(
        &self,
        other: &impl ArrowTrait<
            'a,
            SourceObject = Self::SourceObject,
            TargetObject = Self::TargetObject,
            Identifier = Self::Identifier,
        >,
    ) -> Result<(), Errors> {
        todo!()
    }

    fn is_isomorphism(&self) -> bool {
        todo!()
    }
}
