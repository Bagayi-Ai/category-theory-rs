use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::fmt::Debug;

use crate::core::morphism::{Morphism, UnitMorphism};
use crate::core::identifier::Identifier;
use crate::core::morphism_tree::MorphismMappingTree;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NCategoryError {
    CellAlreadyExists,
    CellNotFound,
    OnlyIdentityCellDiscreteCategory,
    InvalidCellComposition,
    InvalidCellCommutation,
    ObjectNotFound,
    InvalidObjectId,
    InvalidObjectMapping,
    InvalidCellMapping,
    NoObjectsInCategory,
    InvalidCategory,
    InvalidFunctorLevelMissmatch,
    InvalidFunctor,
}


pub trait NCategory<'a> : Debug
where
    Self::BaseCategory: NCategory<'a, Identifier = Self::Identifier>,
{
    type Identifier: Identifier;
    type Object: 'a + Eq + Debug;
    type Morphism: Morphism<'a, Category = Self>;
    type BaseCategory: NCategory<'a>;


    fn level(&self) -> usize where Self: Sized {
        Self::nested_level()
    }

    fn category_id(&self) -> &Self::Identifier;

    fn add_object(&mut self, object: Self::Object) -> Result<(), NCategoryError>;

    fn add_moprhism(&mut self, cell: Self::Morphism) -> Result<Self::Identifier, NCategoryError>;

    fn get_object(&self, object_id: &Self::Identifier) -> Result<Self::Object, NCategoryError>;

    fn get_identity_morphism(
        &self,
        object: Self::Object,
    ) -> Result<&Self::Morphism, NCategoryError>;

    fn get_all_objects(&self) -> Result<HashSet<Self::Object>, NCategoryError>;

    fn get_all_morphisms(&self) -> Result<HashSet<&Self::Morphism>, NCategoryError>;

    fn get_object_morphisms(
        &self,
        object_id: Self::Object,
    ) -> Result<Vec<&Self::Morphism>, NCategoryError>;

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

    fn get_cell(&self, cell_id: &Self::Identifier) -> Result<&Self::Morphism, NCategoryError>;

    fn get_morphism_tree(&self, cell_id: &Self::Morphism) -> Result<MorphismMappingTree<'a, Self::Identifier, Self, Self>, NCategoryError> where Self: Sized
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
    fn cells_commute(
        &self,
        left_cells: Vec<&Self::Morphism>,
        right_cells: Vec<&Self::Morphism>,
    ) -> Result<bool, NCategoryError> {

        self.validate_commutation(left_cells, right_cells)?;


        Ok(true)
    }

    fn validate_commutation(&self,
                            left_cells: Vec<&Self::Morphism>,
                            right_cells: Vec<&Self::Morphism>) -> Result<(), NCategoryError>
    {
        // source and target of left cells id should be same with right cells
        let left_source_object = left_cells.first().ok_or_else(|| NCategoryError::InvalidCellCommutation)?.source_object();
        let right_source_object = right_cells.first().ok_or_else(|| NCategoryError::InvalidCellCommutation)?.source_object();

        if left_source_object != right_source_object {
            return Err(NCategoryError::InvalidCellComposition);
        }

        let left_target_object = left_cells.first().ok_or_else(|| NCategoryError::InvalidCellCommutation)?.target_object();
        let right_target_object = right_cells.first().ok_or_else(|| NCategoryError::InvalidCellCommutation)?.target_object();

        if left_target_object != right_target_object {
            return Err(NCategoryError::InvalidCellComposition);
        }

        // confirm composition is correct
        self.validate_composition(left_cells)?;
        self.validate_composition(right_cells)?;

        Ok(())
    }

    fn validate_composition(&self, cells: Vec<&Self::Morphism>) -> Result<(), NCategoryError>
    {
        if cells.is_empty() {
            return Err(NCategoryError::InvalidCellComposition)
        }

        // composition of only once cell is always valid
        if cells.len() <= 1 {
            return Ok(());
        }
        // target of first cell needs to be the source of subsequent cell
        let mut target_object = cells.first()
            .ok_or_else(|| NCategoryError::InvalidCellComposition)?.target_object();

        for cell in &cells[1..] {
            if cell.source_object() != target_object {
                return Err(NCategoryError::InvalidCellComposition);
            }
            target_object = cell.target_object();
        }
        Ok(())
    }


    fn is_zero_category(&self) -> bool {
        false
    }


    fn base_object(&self, object_id: &Self::Identifier) -> Result<&Self::BaseCategory, NCategoryError>;

    fn nested_level() -> usize
    where
        Self: Sized,
    {
        1 + <Self::BaseCategory as NCategory>::nested_level()
    }

}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitCategory<T: Identifier> {
    _phantom: std::marker::PhantomData<T>,
}

impl <'a, T: Identifier> NCategory<'a> for UnitCategory<T> {
    type Identifier = T;

    type Object = ();

    type Morphism = UnitMorphism<T>;

    type BaseCategory = UnitCategory<T>;

    fn category_id(&self) -> &Self::Identifier {
        todo!()
    }

    fn add_object(&mut self, object: Self::Object) -> Result<(), NCategoryError> {
        todo!()
    }

    fn add_moprhism(&mut self, cell: Self::Morphism) -> Result<Self::Identifier, NCategoryError> {
        todo!()
    }

    fn get_object(&self, object_id: &Self::Identifier) -> Result<Self::Object, NCategoryError> {
        todo!()
    }

    fn get_identity_morphism(
        &self,
        object_id: Self::Object,
    ) -> Result<&Self::Morphism, NCategoryError> {
        todo!()
    }

    fn get_all_objects(&self) -> Result<HashSet<Self::Object>, NCategoryError> {
        todo!()
    }

    fn get_all_morphisms(&self) -> Result<HashSet<&Self::Morphism>, NCategoryError> {
        todo!()
    }

    fn get_object_morphisms(
        &self,
        object_id: Self::Object,
    ) -> Result<Vec<&Self::Morphism>, NCategoryError> {
        todo!()
    }

    fn get_cell(&self, cell_id: &Self::Identifier) -> Result<&Self::Morphism, NCategoryError> {
        todo!()
    }

    fn base_object(&self, object_id: &Self::Identifier) -> Result<&Self::BaseCategory, NCategoryError> {
        todo!()
    }

    fn nested_level() -> usize { 0 }
}