use crate::core::discrete_category::DiscreteCategory;
use crate::core::functor_mapping::FunctorMappings;
use crate::core::identifier::Identifier;
use crate::core::ncategory::{NCategory, NCategoryError};
use crate::core::nfunctor::NFunctor;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use uuid::Uuid;

pub struct GenericNFunctor<
    'a,
    Id: Identifier,
    SourceCategory: NCategory<'a>,
    TargetCategory: NCategory<'a>,
> {
    id: Id,
    source_category: &'a SourceCategory,
    target_category: &'a TargetCategory,
    mappings: FunctorMappings<'a, Id, SourceCategory, TargetCategory>,
}

impl<'a, Id, SourceCategory, TargetCategory> GenericNFunctor<'a, Id, SourceCategory, TargetCategory>
where
    SourceCategory: NCategory<'a>,
    TargetCategory: NCategory<'a>,
    Id: Identifier + 'a,
{
    pub fn new(
        id: Id,
        source_category: &'a SourceCategory,
        target_category: &'a TargetCategory,
        mappings: FunctorMappings<'a, Id, SourceCategory, TargetCategory>,
    ) -> Self {
        let functor = GenericNFunctor {
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

impl<'a, Id, SourceCategory, TargetCategory> NFunctor<'a>
    for GenericNFunctor<'a, Id, SourceCategory, TargetCategory>
where
    SourceCategory: NCategory<'a>,
    TargetCategory: NCategory<'a>,
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
}
