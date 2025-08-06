use crate::core::discrete_category::DiscreteCategory;
use crate::core::identifier::Identifier;
use crate::core::traits::category_trait::CategoryTrait;
use crate::core::traits::functor_trait::FunctorTrait;
use crate::core::traits::morphism_trait::MorphismTrait;
use crate::core::unit::unit_functor::UNIT_FUNCTOR_STRING;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

pub struct FunctorMappings<'a, Id, SourceCategory, TargetCategory>
where
    SourceCategory: CategoryTrait<'a>,
    TargetCategory: CategoryTrait<'a>,
    Id: Identifier,
{
    morphism_mappings: HashMap<
        &'a <SourceCategory as CategoryTrait<'a>>::Morphism,
        &'a <TargetCategory as CategoryTrait<'a>>::Morphism,
    >,
    functor_mappings: HashMap<
        &'a <SourceCategory as CategoryTrait<'a>>::Object,
        &'a dyn FunctorTrait<
            'a,
            Identifier = Id,
            SourceCategory = <SourceCategory as CategoryTrait<'a>>::Object,
            TargetCategory = <TargetCategory as CategoryTrait<'a>>::Object,
        >,
    >,
}

impl<'a, SourceCategory, TargetCategory, Id> Debug
    for FunctorMappings<'a, Id, SourceCategory, TargetCategory>
where
    SourceCategory: CategoryTrait<'a>,
    TargetCategory: CategoryTrait<'a>,
    Id: Identifier,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("functor_mappings").finish()
    }
}

impl<'a, SourceCategory, TargetCategory, Id> PartialEq
    for FunctorMappings<'a, Id, SourceCategory, TargetCategory>
where
    SourceCategory: CategoryTrait<'a>,
    TargetCategory: CategoryTrait<'a>,
    Id: Identifier,
{
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl<'a, SourceCategory, TargetCategory, Id> FunctorMappings<'a, Id, SourceCategory, TargetCategory>
where
    SourceCategory: CategoryTrait<'a>,
    TargetCategory: CategoryTrait<'a>,
    Id: Identifier,
{
    pub fn new() -> Self {
        todo!()
    }

    pub fn get_morphism_mapping(
        &self,
    ) -> &HashMap<
        &'a <SourceCategory as CategoryTrait<'a>>::Morphism,
        &'a <TargetCategory as CategoryTrait<'a>>::Morphism,
    > {
        &self.morphism_mappings
    }

    pub fn get_functor_mappings(
        &self,
    ) -> &HashMap<
        &'a <SourceCategory as CategoryTrait<'a>>::Object,
        &'a dyn FunctorTrait<
            'a,
            Identifier = Id,
            SourceCategory = <SourceCategory as CategoryTrait<'a>>::Object,
            TargetCategory = <TargetCategory as CategoryTrait<'a>>::Object,
        >,
    > {
        &self.functor_mappings
    }
}

impl<
    'a,
    SourceObject: Eq + Clone + Identifier + Display + 'a,
    TargetObject: Eq + Clone + Identifier + Display + 'a,
>
    From<
        Vec<(
            &'a DiscreteCategory<SourceObject>,
            &'a DiscreteCategory<TargetObject>,
        )>,
    >
    for FunctorMappings<'a, String, DiscreteCategory<SourceObject>, DiscreteCategory<TargetObject>>
{
    fn from(
        value: Vec<(
            &'a DiscreteCategory<SourceObject>,
            &'a DiscreteCategory<TargetObject>,
        )>,
    ) -> Self {
        let mut morphism_mappings = HashMap::new();
        let mut functor_mappings: HashMap<
            &'a <DiscreteCategory<SourceObject> as CategoryTrait<'a>>::Object,
            &'a dyn FunctorTrait<
                'a,
                Identifier = String,
                SourceCategory = <DiscreteCategory<SourceObject> as CategoryTrait<'a>>::Object,
                TargetCategory = <DiscreteCategory<TargetObject> as CategoryTrait<'a>>::Object,
            >,
        > = HashMap::new();

        for (source_morphism, target_morphism) in value {
            // Assuming the morphisms are identity morphisms in discrete categories

            morphism_mappings.insert(source_morphism, target_morphism);
            functor_mappings.insert(source_morphism.source_object(), &UNIT_FUNCTOR_STRING);
        }

        FunctorMappings {
            morphism_mappings,
            functor_mappings,
        }
    }
}
