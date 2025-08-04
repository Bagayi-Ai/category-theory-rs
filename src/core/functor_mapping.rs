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

pub struct FunctorMappings<'a, Id, SourceCategory, TargetCategory>
where
    SourceCategory: NCategory<'a>,
    TargetCategory: NCategory<'a>,
    Id: Identifier,
{
    MorphismMapping: HashMap<
        &'a <SourceCategory as NCategory<'a>>::Morphism,
        &'a <TargetCategory as NCategory<'a>>::Morphism,
    >,
    FunctorMappings: HashMap<
        &'a <SourceCategory as NCategory<'a>>::Object,
        Arc<
            dyn NFunctor<
                    'a,
                    Identifier = Id,
                    SourceCategory = <SourceCategory as NCategory<'a>>::Object,
                    TargetCategory = <TargetCategory as NCategory<'a>>::Object,
                >,
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
        f.debug_struct("FunctorMappings").finish()
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
        Arc<
            dyn NFunctor<
                    'a,
                    Identifier = Id,
                    SourceCategory = <SourceCategory as NCategory<'a>>::Object,
                    TargetCategory = <TargetCategory as NCategory<'a>>::Object,
                >,
        >,
    > {
        &self.FunctorMappings
    }
}

impl<
    'a,
    SourceObject: Eq + Clone + Identifier + Display + 'a,
    TargetObject: Eq + Clone + Identifier + Display + 'a,
>
    From<
        Vec<(
            &'a UnitCategory<SourceObject>,
            &'a UnitCategory<TargetObject>,
        )>,
    >
    for FunctorMappings<'a, String, DiscreteCategory<SourceObject>, DiscreteCategory<TargetObject>>
{
    fn from(
        value: Vec<(
            &'a UnitCategory<SourceObject>,
            &'a UnitCategory<TargetObject>,
        )>,
    ) -> Self {
        let mut morphism_mapping = HashMap::new();
        let mut functor_mappings = HashMap::new();

        let unit_functor: Arc<
            dyn NFunctor<
                    'a,
                    Identifier = String,
                    SourceCategory = UnitCategory<SourceObject>,
                    TargetCategory = UnitCategory<TargetObject>,
                >,
        > = Arc::new(UnitFunctor::<
            String,
            UnitCategory<SourceObject>,
            UnitCategory<TargetObject>,
        >::new());
        for (source_morphism, target_morphism) in value {
            morphism_mapping.insert(source_morphism, target_morphism);
            functor_mappings.insert(source_morphism, Arc::clone(&unit_functor));
        }

        FunctorMappings {
            MorphismMapping: morphism_mapping,
            FunctorMappings: functor_mappings,
        }
    }
}
