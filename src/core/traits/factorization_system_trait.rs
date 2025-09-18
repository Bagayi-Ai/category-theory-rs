use crate::core::errors::Errors;
use crate::core::traits::category_trait::CategoryTrait;
use std::sync::Arc;

pub trait FactorizationSystemTrait: CategoryTrait {
    // in a factorization system for each morphism there are two morphisms
    // such that the original morphism factors through them
    // i.e. f = g . h, where g is the morphism from the first factorization and h is the morphism from the second factorization
    // this means that
    fn morphism_factors(
        &self,
        morphism: &Self::Morphism,
    ) -> Result<&(Arc<Self::Morphism>, Arc<Self::Morphism>), Errors>;
}
