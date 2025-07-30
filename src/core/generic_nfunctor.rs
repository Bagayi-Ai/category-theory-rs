use crate::core::identifier::Identifier;
use crate::core::ncategory::NCategory;
use crate::core::nfunctor::NFunctor;

pub struct GenericNFunctor<Id: Identifier> {
    value: Id
}


impl <Id: Identifier> NFunctor for GenericNFunctor<Id> {
    type Identifier = Id;

    fn id(&self) -> &Self::Identifier {
        todo!()
    }

    fn source_category_id(&self) -> &Self::Identifier {
        todo!()
    }

    fn target_category_id(&self) -> &Self::Identifier {
        todo!()
    }
}