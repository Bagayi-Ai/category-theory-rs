use crate::core::errors::Errors;
use crate::core::morphism::Morphism;
use crate::core::traits::category_trait::CategoryTrait;
use std::rc::Rc;

pub trait FactorizationSystemTrait: CategoryTrait {
    // in a factorization system for each morphism there are two morphisms
    // such that the original morphism factors through them
    // i.e. f = g . h, where g is the morphism from the first factorization and h is the morphism from the second factorization
    // this means that
    fn morphism_factors(
        &self,
        morphism: &Morphism<Self::Object>,
    ) -> Result<&(Rc<Morphism<Self::Object>>, Rc<Morphism<Self::Object>>), Errors>;
}
