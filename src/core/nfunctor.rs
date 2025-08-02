use crate::core::identifier::Identifier;
use crate::core::ncategory::{NCategory, NCategoryError, UnitCategory};
use std::collections::HashMap;

// pub struct FunctorMapping<'a, SourceCategory: NCategory<'a>, TargetCategory: NCategory<'a>> {
//     source_cell: &'a <SourceCategory as NCategory<'a>>::Cell,
//     target_cell: &'a <TargetCategory as NCategory<'a>>::Cell,
//     children: Vec<
//         Box<
//             dyn FunctorMappingTrait<
//                 'a,
//                 SourceCategory = <SourceCategory as NCategory<'a>>::BaseCategory,
//                 TargetCategory = <TargetCategory as NCategory<'a>>::BaseCategory
//             >
//         >
//     >
// }
//
// pub trait FunctorMappingTrait<'a> {
//     type SourceCategory: NCategory<'a>;
//     type TargetCategory: NCategory<'a>;
//
//     fn source_cell(&self) -> &'a <Self::SourceCategory as NCategory<'a>>::Cell;
//     fn target_cell(&self) -> &'a <Self::TargetCategory as NCategory<'a>>::Cell;
// }

pub struct Mapping<'a, SourceCategory, TargetCategory, Id>
where
    SourceCategory: NCategory<'a>,
    TargetCategory: NCategory<'a>,
    Id: Identifier,
{
    pub target_cell: &'a <TargetCategory as NCategory<'a>>::Morphism,
    pub base_functor: &'a dyn NFunctor<
        'a,
        Identifier = Id,
        SourceCategory = <SourceCategory as NCategory<'a>>::BaseCategory,
        TargetCategory = <TargetCategory as NCategory<'a>>::BaseCategory,
    >,
}

pub struct FunctorMappings<'a, SourceCategory, TargetCategory, Id>
where
    SourceCategory: NCategory<'a>,
    TargetCategory: NCategory<'a>,
    Id: Identifier,
{
    pub mappings: HashMap<
        &'a <SourceCategory as NCategory<'a>>::Morphism,
        Mapping<'a, SourceCategory, TargetCategory, Id>,
    >,
}

impl<'a, SourceCategory, TargetCategory, Id> FunctorMappings<'a, SourceCategory, TargetCategory, Id>
where
    SourceCategory: NCategory<'a>,
    TargetCategory: NCategory<'a>,
    Id: Identifier,
{
    pub fn new() -> Self {
        Self {
            mappings: HashMap::new(),
        }
    }
}

pub trait NFunctor<'a> {
    type SourceCategory: NCategory<'a>;
    type TargetCategory: NCategory<'a>;

    type Identifier: Identifier;

    fn functor_id(&self) -> &Self::Identifier;

    fn source_category(&self) -> &Self::SourceCategory;

    fn target_category(&self) -> &Self::TargetCategory;

    // for each mapping of a cell,
    // there is a corresponding functor mapping of its base category
    // to base category of the target category
    fn mappings(
        &self,
    ) -> Result<
        FunctorMappings<'a, Self::SourceCategory, Self::TargetCategory, Self::Identifier>,
        NCategoryError,
    >;

    fn validate_level(&self) -> Result<(), NCategoryError> {
        if self.source_category().level() != self.target_category().level() {
            return Err(NCategoryError::InvalidFunctorLevelMissmatch);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitFunctor<
    'a,
    T: Identifier,
    SourceCategory: NCategory<'a>,
    TargetCategory: NCategory<'a>,
> {
    _phantom: std::marker::PhantomData<&'a T>,
    _phantom2: std::marker::PhantomData<SourceCategory>,
    _phantom3: std::marker::PhantomData<TargetCategory>,
}
impl<'a, T, SourceCategory, TargetCategory> NFunctor<'a>
    for UnitFunctor<'a, T, SourceCategory, TargetCategory>
where
    T: Identifier,
    SourceCategory: NCategory<'a>,
    TargetCategory: NCategory<'a>,
{
    type Identifier = T;
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

    fn mappings(
        &self,
    ) -> Result<
        FunctorMappings<'a, Self::SourceCategory, Self::TargetCategory, Self::Identifier>,
        NCategoryError,
    > {
        todo!()
    }
}
