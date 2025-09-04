use crate::core::errors::Errors;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use crate::core::traits::functor_trait::FunctorTrait;
use std::rc::Rc;

pub trait MorphismTrait<Object: CategoryTrait>: ArrowTrait<Object, Object> {
    fn functor(&self) -> Result<&Rc<impl FunctorTrait<Object, Object>>, Errors>;
}
