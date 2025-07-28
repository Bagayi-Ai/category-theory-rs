use crate::core::ncategory::NCategory;
use crate::core::nfunctor::NFunctor;

pub struct GenericNFunctor<'a, Category: NCategory<'a>> {
    value: Category::Identifier,
}


impl <'a, Category: NCategory<'a>> NFunctor<'a> for GenericNFunctor<'a, Category> {
    type Category = Category;

    fn id(&self) -> &<Self::Category as NCategory<'a>>::Identifier {
        todo!()
    }

    fn source_category(&self) -> &Self::Category {
        todo!()
    }

    fn target_category(&self) -> &Self::Category {
        todo!()
    }
}