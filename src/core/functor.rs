use crate::core::identifier::Identifier;
use crate::core::traits::category_trait::{CategoryTrait, MorphismAlias};
use crate::core::traits::functor_trait::FunctorTrait;
use std::collections::HashMap;

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Functor<'a, Id, SourceCategory, TargetCategory>
where
    Id: Identifier,
    SourceCategory: CategoryTrait<'a>,
    TargetCategory: CategoryTrait<'a>,
{
    id: Id,
    source_category: &'a SourceCategory,
    target_category: &'a TargetCategory,
    mappings: HashMap<&'a MorphismAlias<'a, SourceCategory>, &'a MorphismAlias<'a, TargetCategory>>,
}

impl<'a, Id, SourceCategory, TargetCategory> Functor<'a, Id, SourceCategory, TargetCategory>
where
    Id: Identifier,
    SourceCategory: CategoryTrait<'a>,
    TargetCategory: CategoryTrait<'a>,
{
    pub fn new(
        id: Id,
        source_category: &'a SourceCategory,
        target_category: &'a TargetCategory,
        mappings: HashMap<
            &'a MorphismAlias<'a, SourceCategory>,
            &'a MorphismAlias<'a, TargetCategory>,
        >,
    ) -> Self {
        Functor {
            id,
            source_category,
            target_category,
            mappings,
        }
    }
}

impl<'a, Id, SourceCategory, TargetCategory> FunctorTrait<'a, Id, SourceCategory, TargetCategory>
    for Functor<'a, Id, SourceCategory, TargetCategory>
where
    Id: Identifier,
    SourceCategory: CategoryTrait<'a>,
    TargetCategory: CategoryTrait<'a>,
{
    fn functor_id(&self) -> &Id {
        &self.id
    }

    fn source_category(&self) -> &'a SourceCategory {
        self.source_category
    }

    fn target_category(&self) -> &'a TargetCategory {
        self.target_category
    }

    fn arrow_mappings(
        &self,
    ) -> &HashMap<&MorphismAlias<'a, SourceCategory>, &MorphismAlias<'a, TargetCategory>> {
        // self.mappings
        //     .iter()
        //     .map(|m| {
        //         m as &dyn ArrowMappingTrait<
        //             'a,
        //             SourceArrow = <SourceCategory as CategoryTrait<'a>>::Morphism,
        //             TargetArrow = <TargetCategory as CategoryTrait<'a>>::Morphism,
        //         >
        //     })
        //     .collect()
        todo!()
    }
}
