use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::functor_trait::FunctorTrait;
use std::collections::HashSet;

pub trait CategoryTrait<'a> {
    type Identifier: Identifier;
    type Object: 'a + CategoryTrait<'a>;
    type Morphism: ArrowTrait<
            'a,
            SourceObject = Self::Object,
            TargetObject = Self::Object,
            Identifier = Self::Identifier,
        >;

    fn level(&self) -> usize
    where
        Self: Sized,
    {
        Self::nested_level()
    }

    fn category_id(&self) -> &Self::Identifier;

    fn add_object(&mut self, object: &'a Self::Object) -> Result<(), Errors>;

    fn add_morphism(&mut self, morphism: Self::Morphism) -> Result<Self::Identifier, Errors>;

    fn get_identity_morphism(
        &self,
        object_id: &Self::Identifier,
    ) -> Result<&Self::Morphism, Errors>;

    fn get_all_object_ids(&self) -> Result<HashSet<&Self::Identifier>, Errors>;

    fn get_all_morphisms(&self) -> Result<HashSet<&Self::Morphism>, Errors>;

    fn get_object_morphisms(
        &self,
        object_id: &Self::Identifier,
    ) -> Result<Vec<&Self::Morphism>, Errors>;

    fn get_object_targets(
        &self,
        object_id: &Self::Identifier,
    ) -> Result<Vec<&Self::Identifier>, Errors> {
        // self.get_object_cells(object_id)
        //     .unwrap()
        //     .iter()
        //     .map(|cell_id| self.target(cell_id))
        //     .collect()
        todo!()
    }

    fn get_moprhism(&self, morphism_id: &Self::Identifier) -> Result<&Self::Morphism, Errors>;

    fn morphism_commute(
        &self,
        left_morphisms: Vec<&Self::Morphism>,
        right_morphisms: Vec<&Self::Morphism>,
    ) -> Result<bool, Errors> {
        self.validate_morphisms_commutation(left_morphisms, right_morphisms)?;

        Ok(true)
    }

    fn validate_morphisms_commutation(
        &self,
        left_morphisms: Vec<&Self::Morphism>,
        right_morphisms: Vec<&Self::Morphism>,
    ) -> Result<(), Errors> {
        // // source and target of left cells id should be same with right cells
        // let left_source_object = left_morphisms
        //     .first()
        //     .ok_or(NCategoryError::InvalidMorphismCommutation)?
        //     .source_object();
        // let right_source_object = right_morphisms
        //     .first()
        //     .ok_or(NCategoryError::InvalidMorphismCommutation)?
        //     .source_object();
        //
        // if left_source_object != right_source_object {
        //     return Err(NCategoryError::InvalidMorphismComposition);
        // }
        //
        // let left_target_object = left_morphisms
        //     .first()
        //     .ok_or(NCategoryError::InvalidMorphismCommutation)?
        //     .target_object();
        // let right_target_object = right_morphisms
        //     .first()
        //     .ok_or(NCategoryError::InvalidMorphismCommutation)?
        //     .target_object();
        //
        // if left_target_object != right_target_object {
        //     return Err(NCategoryError::InvalidMorphismComposition);
        // }
        //
        // // confirm composition is correct
        // self.validate_morphisms_composition(left_morphisms)?;
        // self.validate_morphisms_composition(right_morphisms)?;
        //
        // Ok(())
        todo!()
    }

    fn validate_morphisms_composition(&self, morphims: Vec<&Self::Morphism>) -> Result<(), Errors> {
        // if morphims.is_empty() {
        //     return Err(NCategoryError::InvalidMorphismComposition);
        // }
        //
        // // composition of only once cell is always valid
        // if morphims.len() <= 1 {
        //     return Ok(());
        // }
        // // target of first cell needs to be the source of subsequent cell
        // let mut target_object = morphims
        //     .first()
        //     .ok_or(NCategoryError::InvalidMorphismComposition)?
        //     .target_object();
        //
        // for morphism in &morphims[1..] {
        //     if morphism.source_object() != target_object {
        //         return Err(NCategoryError::InvalidMorphismComposition);
        //     }
        //     target_object = morphism.target_object();
        // }
        // Ok(())
        todo!()
    }

    fn is_zero_category(&self) -> bool {
        false
    }

    fn nested_level() -> usize
    where
        Self: Sized,
    {
        1 + <Self::Object as CategoryTrait>::nested_level()
    }
}

pub type ChildObjectAlias<'a, Object> = <Object as CategoryTrait<'a>>::Object;
pub type MorphismAlias<'a, Category> = <Category as CategoryTrait<'a>>::Morphism;
