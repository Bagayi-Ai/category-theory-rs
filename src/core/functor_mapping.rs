use crate::core::discrete_category::DiscreteCategory;
use crate::core::identifier::Identifier;
use crate::core::traits::morphism_trait::MorphismTrait;
use crate::core::ncategory::NCategory;
use crate::core::nfunctor::NFunctor;
use crate::core::unit::unit_functor::UNIT_FUNCTOR_STRING;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

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
            &'a <DiscreteCategory<SourceObject> as NCategory<'a>>::Object,
            &'a dyn NFunctor<
                'a,
                Identifier = String,
                SourceCategory = <DiscreteCategory<SourceObject> as NCategory<'a>>::Object,
                TargetCategory = <DiscreteCategory<TargetObject> as NCategory<'a>>::Object,
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
