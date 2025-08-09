use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::traits::category_trait::CategoryTrait;
use crate::core::traits::functor_trait::FunctorTrait;

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
        &dyn ArrowTrait<
            'a,
            SourceObject = Self::SourceObject,
            TargetObject = Self::TargetObject,
            Identifier = Self::Identifier,
        >,
    >;
}
