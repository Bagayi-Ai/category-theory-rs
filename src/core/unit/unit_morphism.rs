use crate::core::identifier::Identifier;
use crate::core::traits::morphism_trait::MorphismTrait;
use crate::core::unit::unit_category::UnitCategory;
use crate::core::unit::unit_functor::UnitFunctor;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitMorphism<T: Identifier> {
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T: Identifier + 'a> MorphismTrait<'a> for UnitMorphism<T> {
    type Object = UnitCategory;
    type Identifier = T;

    type Functor = UnitFunctor<T>;

    fn cell_id(&self) -> &Self::Identifier {
        todo!()
    }

    fn source_object(&self) -> &Self::Object {
        todo!()
    }

    fn target_object(&self) -> &Self::Object {
        todo!()
    }

    fn is_identity(&self) -> bool {
        todo!()
    }

    fn functor(&self) -> &Self::Functor {
        todo!()
    }
}
