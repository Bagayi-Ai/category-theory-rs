use std::collections::HashMap;

use crate::core::identifier::Identifier;
use crate::core::ncategory::{NCategory, NCategoryError};
use crate::core::nfunctor::NFunctor;

pub struct GenericNFunctor<'a, Id: Identifier, SourceCategory: NCategory<'a>,  TargetCategory: NCategory<'a>>{
    id: Id,
    source_category: &'a SourceCategory,
    target_category: &'a TargetCategory,
    mappings: HashMap<
        &'a <SourceCategory as NCategory<'a>>::Cell,
        &'a <TargetCategory as NCategory<'a>>::Cell>,
}

impl<'a, Id, SourceCategory, TargetCategory> GenericNFunctor<'a, Id, SourceCategory, TargetCategory>
where
    SourceCategory: NCategory<'a, Identifier = Id>,
    TargetCategory: NCategory<'a, Identifier = Id>,
    Id: Identifier,
{
    pub fn new(
        id: Id,
        source_category: &'a SourceCategory,
        target_category: &'a TargetCategory,
        mappings: HashMap<
            &'a <SourceCategory as NCategory<'a>>::Cell,
            &'a <TargetCategory as NCategory<'a>>::Cell>,
    ) -> Self {
        let functor = GenericNFunctor {
            id,
            source_category,
            target_category,
            mappings,
        };
        functor.validate_level().expect("Functor level validation failed");
        functor
    }
}


impl <'a, Id, SourceCategory, TargetCategory> NFunctor<'a> for GenericNFunctor<'a, Id, SourceCategory, TargetCategory>
where
    SourceCategory: NCategory<'a, Identifier = Id>,
    TargetCategory: NCategory<'a, Identifier = Id>,
    Id: Identifier,
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
    
    fn mappings(&self) -> Result<
        super::nfunctor::FunctorMappings<'a, Self::SourceCategory, Self::TargetCategory, Self::Identifier>,
        NCategoryError> {
        todo!()
    }

}