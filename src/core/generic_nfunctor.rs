use crate::core::ncategory::NCategory;
use crate::core::nfunctor::NFunctor;

pub struct GenericNFunctor<Category: NCategory> {
    value: Category::Identifier,
}


impl <Category: NCategory> NFunctor for GenericNFunctor<Category> {
    type Category = Category;

    fn id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn source_category_id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn target_category_id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }
}