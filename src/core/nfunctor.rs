use crate::core::ncategory::{NCategory};
use crate::core::identifier::Identifier;

pub trait NFunctor<Id: Identifier, Category: NCategory<Identifier = Id>> {
    type SourceCategory: NCategory<Identifier = Id>;
    type TargetCategory: NCategory<Identifier = Id>;

    fn id(&self) -> &Id;

    fn source_category(&self) -> &Self::SourceCategory;

    fn target_category(&self) -> &Self::TargetCategory;
    
}