use crate::core::identifier::Identifier;
use crate::core::ncategory::NCategory;
use crate::core::nfunctor::NFunctor;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

pub struct FunctorMappings<'a, Id, SourceCategory, TargetCategory>
where
    SourceCategory: NCategory<'a>,
    TargetCategory: NCategory<'a>,
    Id: Identifier,
{
    SourceCategory: &'a SourceCategory,
    TargetCategory: &'a TargetCategory,
    MorphismMapping: HashMap<
        &'a <SourceCategory as NCategory<'a>>::Morphism,
        &'a <TargetCategory as NCategory<'a>>::Morphism,
    >,
    FunctorMappings: HashMap<
        &'a <SourceCategory as NCategory<'a>>::Object,
        &'a dyn NFunctor<
            'a,
            Identifier = Id,
            SourceCategory = <SourceCategory as NCategory<'a>>::BaseCategory,
            TargetCategory = <TargetCategory as NCategory<'a>>::BaseCategory,
        >,
    >,
}

impl<'a, SourceCategory, TargetCategory, Id> Debug
    for FunctorMappings<'a, Id, SourceCategory, TargetCategory>
where
    SourceCategory: NCategory<'a>,
    TargetCategory: NCategory<'a>,
    Id: Identifier,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctorMappings")
            .field("SourceCategory", &self.SourceCategory)
            .field("TargetCategory", &self.TargetCategory)
            .finish()
    }
}

impl<'a, SourceCategory, TargetCategory, Id> PartialEq
    for FunctorMappings<'a, Id, SourceCategory, TargetCategory>
where
    SourceCategory: NCategory<'a>,
    TargetCategory: NCategory<'a>,
    Id: Identifier,
{
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl<'a, SourceCategory, TargetCategory, Id> FunctorMappings<'a, Id, SourceCategory, TargetCategory>
where
    SourceCategory: NCategory<'a>,
    TargetCategory: NCategory<'a>,
    Id: Identifier,
{
    pub fn new() -> Self {
        todo!()
    }

    pub fn get_morphism_mapping(
        &self,
    ) -> &HashMap<
        &'a <SourceCategory as NCategory<'a>>::Morphism,
        &'a <TargetCategory as NCategory<'a>>::Morphism,
    > {
        &self.MorphismMapping
    }

    pub fn get_functor_mappings(
        &self,
    ) -> &HashMap<
        &'a <SourceCategory as NCategory<'a>>::Object,
        &'a dyn NFunctor<
            'a,
            Identifier = Id,
            SourceCategory = <SourceCategory as NCategory<'a>>::BaseCategory,
            TargetCategory = <TargetCategory as NCategory<'a>>::BaseCategory,
        >,
    > {
        &self.FunctorMappings
    }
}
