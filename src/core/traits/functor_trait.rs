use crate::core::errors::Errors;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use crate::core::traits::category_trait::MorphismAlias;
use std::collections::HashMap;
use std::rc::Rc;

pub trait FunctorTrait: ArrowTrait {
    fn validate_mappings(&self) -> Result<(), Errors> {
        /*
        Functor should validate that all objects in the source category
        have a corresponding object in the target category.

        And that all morphisms in the source category are mapped to morphisms in the target category,
        such that they commute with morphism in the target category.
        i.e        for each morphism f: A -> B in the source category,
        there exists a morphism f': F(A) -> F(B) in the target category such that
        F(B) ∘ F(f) = F(f') ∘ F(A)
         */

        // start with checking if all objects in the source category have a corresponding object in the target category
        let mapping = self.arrow_mappings();
        for source_object in self.source_object().get_all_objects()? {
            let identity_morphism = self
                .source_object()
                .get_identity_morphism(source_object)?;

            // a -> F(a)
            let mapped_identity_morphism =
                mapping
                    .get(identity_morphism)
                    .ok_or(Errors::InvalidFunctor(
                        "No functor found for identity morphism".to_string(),
                    ))?;

            // now get the hom-set for the source object
            let hom_set_x = self.source_object().get_hom_set_x(source_object)?;

            for morphism in hom_set_x {
                if morphism.is_identity() {
                    // just check its identity mapping
                    if !mapping.contains_key(morphism) {
                        return Err(Errors::InvalidFunctor(
                            "No functor found for identity morphism".to_string(),
                        ));
                    }
                } else {
                    // F(a) -> F(b)
                    let target_morphism = mapping.get(morphism).ok_or(Errors::InvalidFunctor(
                        "No functor found for morphism".to_string(),
                    ))?;
                    // now we check the commutation condition
                    // from the source object to the target object to the mapped target object
                    // a -> F(a) -> F(b)
                    let first_path = mapped_identity_morphism.compose(&**target_morphism)?;

                    let identity_of_target = self
                        .source_object()
                        .get_identity_morphism(&**morphism.target_object())?;
                    // b -> F(b)
                    let mapped_identity_target_morphism =
                        mapping
                            .get(identity_of_target)
                            .ok_or(Errors::InvalidFunctor(
                                "No functor found for identity morphism in target".to_string(),
                            ))?;
                    // a -> b -> F(b)
                    let second_path = morphism.compose(&**mapped_identity_target_morphism)?;

                    first_path.validate_commutation(&second_path)?;
                }
            }
        }
        Ok(())
    }

    fn arrow_mappings(
        &self,
    ) -> &HashMap<Rc<MorphismAlias<Self::SourceObject>>, Rc<MorphismAlias<Self::TargetObject>>>;
}
