use crate::core::ncategory::{NCategory, UnitCategory};
use crate::core::identifier::{Identifier};

pub trait NFunctor<'a>
{
    type SourceCategory: NCategory<'a>;
    type TargetCategory: NCategory<'a>;

    type Identifier: Identifier;

    fn functor_id(&self) -> &Self::Identifier;

    fn source_category(&self) -> &Self::SourceCategory;

    fn target_category(&self) -> &Self::TargetCategory;

}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitFunctor<'a, T: Identifier, SourceCategory: NCategory<'a>, TargetCategory: NCategory<'a>> {
    _phantom: std::marker::PhantomData<&'a T>,
    _phantom2: std::marker::PhantomData<SourceCategory>,
    _phantom3: std::marker::PhantomData<TargetCategory>,
}
impl <'a, T, SourceCategory, TargetCategory> NFunctor<'a> for UnitFunctor<'a, T, SourceCategory, TargetCategory>
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
}