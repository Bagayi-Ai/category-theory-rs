use crate::core::discrete_category::DiscreteCategory;
use crate::core::identifier::Identifier;
use crate::core::ncategory::NCategory;
use crate::core::nfunctor::NFunctor;
use crate::core::unit::unit_category::UnitCategory;
use crate::core::unit::unit_functor::UnitFunctor;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;
use std::sync::Arc;
use crate::core::morphism::Morphism;

pub struct FunctorMappings<'a, Id, SourceCategory, TargetCategory>
where
    SourceCategory: NCategory<'a>,
    TargetCategory: NCategory<'a>,
    Id: Identifier,
{
    morphism_mappings: HashMap<
        &'a <SourceCategory as NCategory<'a>>::Morphism,
        &'a <TargetCategory as NCategory<'a>>::Morphism,
    >,
    functor_mappings: HashMap<
        &'a <SourceCategory as NCategory<'a>>::Object,
        &'a dyn NFunctor<
                    'a,
                    Identifier = Id,
                    SourceCategory = <SourceCategory as NCategory<'a>>::Object,
                    TargetCategory = <TargetCategory as NCategory<'a>>::Object,
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
        f.debug_struct("functor_mappings").finish()
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
        &self.morphism_mappings
    }

    pub fn get_functor_mappings(
        &self,
    ) -> &HashMap<
        &'a <SourceCategory as NCategory<'a>>::Object,
        &'a dyn NFunctor<
            'a,
            Identifier = Id,
            SourceCategory = <SourceCategory as NCategory<'a>>::Object,
            TargetCategory = <TargetCategory as NCategory<'a>>::Object,
        >,
    > {
        &self.functor_mappings
    }

    pub fn from_vec(
        base_functor: &'a dyn NFunctor<
            'a,
            Identifier = Id,
            SourceCategory = <SourceCategory as NCategory<'a>>::Object,
            TargetCategory = <TargetCategory as NCategory<'a>>::Object,
        >,
        value: Vec<(
            &'a <SourceCategory as NCategory<'a>>::Morphism,
            &'a <TargetCategory as NCategory<'a>>::Morphism,
        )>,
    ) -> Self {
        let mut morphism_mapping = HashMap::new();
        let mut functor_mappings = HashMap::new();

        for (source_morphism, target_morphism) in value {
            morphism_mapping.insert(source_morphism, target_morphism);
            functor_mappings.insert(source_morphism.source_object(), base_functor);
        }

        FunctorMappings {
            morphism_mappings: morphism_mapping,
            functor_mappings: functor_mappings,
        }
    }
}
