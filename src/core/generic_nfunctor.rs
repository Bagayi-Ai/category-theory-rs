use crate::core::identifier::Identifier;
use crate::core::ncategory::NCategory;
use crate::core::nfunctor::NFunctor;

pub struct GenericNFunctor<'a, Id: Identifier, SourceCategory: NCategory<'a>,  TargetCategory: NCategory<'a>>{
    id: Id,
    source_category: &'a SourceCategory,
    target_category: &'a TargetCategory,
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
    ) -> Self {
        GenericNFunctor {
            id,
            source_category,
            target_category,
        }
    }
}


impl <'a, Id, SourceCategory, TargetCategory> NFunctor<'a> for GenericNFunctor<'a, Id, SourceCategory, TargetCategory>
where
    SourceCategory: NCategory<'a, Identifier = Id>,
    TargetCategory: NCategory<'a, Identifier = Id>,
    Id: Identifier,
{
    type Identifier = Id;
    type SourceCategory = SourceCategory;
    type TargetCategory = TargetCategory;

    fn functor_id(&self) -> &Self::Identifier {
        todo!()
    }

    fn source_category(&self) -> &Self::SourceCategory {
        todo!()
    }

    fn target_category(&self) -> &Self::TargetCategory {
        todo!()
    }
}