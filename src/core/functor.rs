use crate::core::functor_mapping::FunctorMappings;
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::{CategoryTrait, NCategoryError};
use crate::core::traits::functor_trait::FunctorTrait;

pub struct Functor<
    'a,
    Id: Identifier,
    SourceCategory: CategoryTrait<'a>,
    TargetCategory: CategoryTrait<'a>,
> {
    id: Id,
    source_category: &'a SourceCategory,
    target_category: &'a TargetCategory,
    mappings: FunctorMappings<'a, Id, SourceCategory, TargetCategory>,
}

impl<'a, Id, SourceCategory, TargetCategory> Functor<'a, Id, SourceCategory, TargetCategory>
where
    SourceCategory: CategoryTrait<'a>,
    TargetCategory: CategoryTrait<'a>,
    Id: Identifier + 'a,
{
    pub fn new(
        id: Id,
        source_category: &'a SourceCategory,
        target_category: &'a TargetCategory,
        mappings: FunctorMappings<'a, Id, SourceCategory, TargetCategory>,
    ) -> Self {
        let functor = Functor {
            id,
            source_category,
            target_category,
            mappings,
        };
        functor
            .validate_level()
            .expect("Functor level validation failed");
        functor
    }
}

impl<'a, Id, SourceCategory, TargetCategory> FunctorTrait<'a>
    for Functor<'a, Id, SourceCategory, TargetCategory>
where
    SourceCategory: CategoryTrait<'a>,
    TargetCategory: CategoryTrait<'a>,
    Id: Identifier + 'a,
{
    type SourceCategory = SourceCategory;
    type TargetCategory = TargetCategory;
    type Identifier = Id;

    fn functor_id(&self) -> &Self::Identifier {
        &self.id
    }

    fn source_category(&self) -> &Self::SourceCategory {
        self.source_category
    }

    fn target_category(&self) -> &Self::TargetCategory {
        self.target_category
    }

    fn mappings(
        &self,
    ) -> Result<
        &FunctorMappings<'a, Self::Identifier, Self::SourceCategory, Self::TargetCategory>,
        NCategoryError,
    > {
        Ok(&self.mappings)
    }

    fn morphisms(
        &self,
    ) -> Result<
        Vec<
            &'a super::traits::arrow_trait::DynArrowTraitType<
                'a,
                Self::Identifier,
                Self::SourceCategory,
                Self::TargetCategory,
            >,
        >,
        NCategoryError,
    > {
        todo!()
    }
}
