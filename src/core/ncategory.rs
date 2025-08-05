use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

use crate::core::identifier::Identifier;
use crate::core::morphism::Morphism;
use crate::core::morphism_tree::MorphismMappingTree;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NCategoryError {
    MorphismAlreadyExists,
    MorphismNotFound,
    OnlyIdentityMorphismDiscreteCategory,
    InvalidMorphismComposition,
    InvalidMorphismCommutation,
    ObjectNotFound,
    InvalidObjectId,
    InvalidObjectMapping,
    InvalidCellMapping,
    NoObjectsInCategory,
    InvalidCategory,
    InvalidFunctorLevelMissmatch,
    InvalidFunctor,
    InvalidFunctorMappings,
    InvalidBaseFunctor,
    CannotAddObjectInDiscreteCategoryOnlyIdentityMorphism,
}

pub trait NCategory<'a>: Debug {
    type Identifier: Identifier;
    type Object: 'a + Eq + Debug + Hash + NCategory<'a>;
    type Arrow: Morphism<'a, Object = Self::Object, Identifier = Self::Identifier>;

    fn level(&self) -> usize
    where
        Self: Sized,
    {
        Self::nested_level()
    }

    fn category_id(&self) -> &Self::Identifier;

    fn add_object(&mut self, object: &'a Self::Object) -> Result<(), NCategoryError>;

    fn add_arrow(
        &mut self,
        arrow: Self::Arrow,
    ) -> Result<Self::Identifier, NCategoryError>;

    fn get_identity_arrow(
        &self,
        object_id: &Self::Identifier,
    ) -> Result<&Self::Arrow, NCategoryError>;

    fn get_all_object_ids(&self) -> Result<HashSet<&Self::Identifier>, NCategoryError>;

    fn get_all_arrows(&self) -> Result<HashSet<&Self::Arrow>, NCategoryError>;

    fn get_object_arrows(
        &self,
        object_id: &Self::Identifier,
    ) -> Result<Vec<&Self::Arrow>, NCategoryError>;

    fn get_object_targets(
        &self,
        object_id: &Self::Identifier,
    ) -> Result<Vec<&Self::Identifier>, NCategoryError> {
        // self.get_object_cells(object_id)
        //     .unwrap()
        //     .iter()
        //     .map(|cell_id| self.target(cell_id))
        //     .collect()
        todo!()
    }

    fn get_arrow(
        &self,
        arrow_id: &Self::Identifier,
    ) -> Result<&Self::Arrow, NCategoryError>;

    fn get_morphism_tree(
        &self,
        cell_id: &Self::Arrow,
    ) -> Result<MorphismMappingTree<'a, Self::Identifier, Self, Self>, NCategoryError>
    where
        Self: Sized,
    {
        /*
        Cell tree is a recursive structure that represents the hierarchy of cells and mapping
        of objects.
        */

        // let cell = self.get_cell(cell_id)?;
        //
        // let cell_tree = CellTree::new(
        //     cell.id(),
        //     cell.source_object_id(),
        //     cell.target_object_id()
        // );
        //
        // // Now take map all the cells in the base of source object
        // let source_base_objects = self.base_object(cell.source_object_id())?;
        // let all_source_base_cells = source_base_objects.get_all_cells()?;
        //
        //
        // Ok(cell_tree)
        todo!()
    }
    fn arrow_commute(
        &self,
        left_morphisms: Vec<&Self::Arrow>,
        right_morphisms: Vec<&Self::Arrow>,
    ) -> Result<bool, NCategoryError> {
        self.validate_arrow_commutation(left_morphisms, right_morphisms)?;

        Ok(true)
    }

    fn validate_arrow_commutation(
        &self,
        left_morphisms: Vec<&Self::Arrow>,
        right_morphisms: Vec<&Self::Arrow>,
    ) -> Result<(), NCategoryError> {
        // source and target of left cells id should be same with right cells
        let left_source_object = left_morphisms
            .first()
            .ok_or(NCategoryError::InvalidMorphismCommutation)?
            .source_object();
        let right_source_object = right_morphisms
            .first()
            .ok_or(NCategoryError::InvalidMorphismCommutation)?
            .source_object();

        if left_source_object != right_source_object {
            return Err(NCategoryError::InvalidMorphismComposition);
        }

        let left_target_object = left_morphisms
            .first()
            .ok_or(NCategoryError::InvalidMorphismCommutation)?
            .target_object();
        let right_target_object = right_morphisms
            .first()
            .ok_or(NCategoryError::InvalidMorphismCommutation)?
            .target_object();

        if left_target_object != right_target_object {
            return Err(NCategoryError::InvalidMorphismComposition);
        }

        // confirm composition is correct
        self.validate_arrow_composition(left_morphisms)?;
        self.validate_arrow_composition(right_morphisms)?;

        Ok(())
    }

    fn validate_arrow_composition(
        &self,
        morphims: Vec<&Self::Arrow>,
    ) -> Result<(), NCategoryError> {
        if morphims.is_empty() {
            return Err(NCategoryError::InvalidMorphismComposition);
        }

        // composition of only once cell is always valid
        if morphims.len() <= 1 {
            return Ok(());
        }
        // target of first cell needs to be the source of subsequent cell
        let mut target_object = morphims
            .first()
            .ok_or(NCategoryError::InvalidMorphismComposition)?
            .target_object();

        for morphism in &morphims[1..] {
            if morphism.source_object() != target_object {
                return Err(NCategoryError::InvalidMorphismComposition);
            }
            target_object = morphism.target_object();
        }
        Ok(())
    }

    fn is_zero_category(&self) -> bool {
        false
    }

    fn nested_level() -> usize
    where
        Self: Sized,
    {
        1 + <Self::Object as NCategory>::nested_level()
    }
}
