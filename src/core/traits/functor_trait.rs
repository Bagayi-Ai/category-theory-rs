use crate::core::functor_mapping::FunctorMappings;
use crate::core::identifier::Identifier;
use crate::core::traits::category_trait::{CategoryTrait, NCategoryError};
use crate::core::traits::arrow_trait::{DynArrowTraitType, ArrowTrait};

pub trait FunctorTrait<'a>: 'a {
    type SourceCategory: CategoryTrait<'a>;
    type TargetCategory: CategoryTrait<'a>;

    type Identifier: Identifier;

    fn functor_id(&self) -> &Self::Identifier;

    fn source_category(&self) -> &Self::SourceCategory;

    fn target_category(&self) -> &Self::TargetCategory;

    fn morphisms(
        &self,
    ) -> Result<
        Vec<
            &'a DynArrowTraitType<
                'a,
                Self::SourceCategory,
                Self::TargetCategory,
                Self::Identifier,
            >,
        >,
        NCategoryError,
    >;

    // for each mapping of a cell,
    // there is a corresponding functor mapping of its base category
    // to base category of the target category
    fn mappings(
        &self,
    ) -> Result<
        &FunctorMappings<'a, Self::Identifier, Self::SourceCategory, Self::TargetCategory>,
        NCategoryError,
    >;

    fn validate_level(&self) -> Result<(), NCategoryError> {
        if self.source_category().level() != self.target_category().level() {
            return Err(NCategoryError::InvalidFunctorLevelMissmatch);
        }
        Ok(())
    }

    fn validate_mappings(&self) -> Result<(), NCategoryError> {
        // let functor_mapping = self.mappings()?;
        // // should map all morphisms in source category to target category
        // if functor_mapping.get_morphism_mapping().len()
        //     != self.source_category().get_all_morphisms()?.len()
        // {
        //     return Err(NCategoryError::InvalidFunctorMappings);
        // }
        //
        // // check each mapping in source category is mapped to a target category morphism
        // let target_morphisms = self.target_category().get_all_morphisms()?;
        // for source_morphism in self.source_category().get_all_morphisms()? {
        //     let mapped_morphism = functor_mapping
        //         .get_morphism_mapping()
        //         .get(source_morphism)
        //         .ok_or(NCategoryError::InvalidFunctorMappings)?;
        //     if !target_morphisms.contains(mapped_morphism) {
        //         return Err(NCategoryError::InvalidFunctor);
        //     }
        //
        //     // if its a identity morphism, we should have equivalent functor mapping
        //     if source_morphism.is_identity() {
        //         // there should be a functor mapping this object to its target object
        //         let functor = functor_mapping
        //             .get_functor_mappings()
        //             .get(&source_morphism.source_object())
        //             .ok_or(NCategoryError::InvalidFunctorMappings)?;
        //
        //         // now confirm that the functor target object is the same as this morphism's target object
        //         mapped_morphism.target_object();
        //     }
        //     // // now check the base functor has mapping from base category of source to base category of target
        //     // let base_functor = &mapping.base_functor;
        //     // if base_functor.source_category().category_id() != self.source_category().base_object().category_id(){
        //     //     return Err(NCategoryError::InvalidBaseFunctor);
        //     // }
        //     // if mapping.base_functor.source_category().get_all_morphisms()?.contains(source_cell) {
        //     //     continue;
        //     // } else {
        //     //     return Err(NCategoryError::InvalidFunctor);
        //     // }
        // }
        // Ok(())
        todo!()
    }
}
