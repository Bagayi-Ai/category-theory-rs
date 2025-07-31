use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::fmt::Debug;

use crate::core::ncell::{NCell};
use crate::core::identifier::Identifier;
use crate::core::cell_tree::CellTree;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NCategoryError {
    CellAlreadyExists,
    CellNotFound,
    OnlyIdentityCellDiscreteCategory,
    InvalidCellComposition,
    InvalidCellCommutation,
    ObjectNotFound,
    InvalidObjectMapping,
    InvalidCellMapping,
    NoObjectsInCategory,
    InvalidCategory,
}


pub trait NCategory<'a>
where
    Self::BaseCategory: NCategory<'a, Identifier = Self::Identifier>,
{
    type Identifier: Identifier;
    type Object: 'a;
    type Cell: NCell<Identifier = Self::Identifier>;
    type BaseCategory: NCategory<'a>;

    fn id(&self) -> &Self::Identifier;

    fn add_object(&mut self, object: Self::Object) -> Result<Self::Identifier, NCategoryError>;

    fn add_object_with_id(&mut self, object_id: Self::Identifier, object: Self::Object) -> Result<(), NCategoryError>;

    fn add_cell(&mut self, cell: Self::Cell) -> Result<Self::Identifier, NCategoryError>;

    fn get_object(&self, object_id: &Self::Identifier) -> Result<Self::Object, NCategoryError>;

    fn get_identity_cell(
        &self,
        object_id: &Self::Identifier,
    ) -> Result<&Self::Cell, NCategoryError>;

    fn get_all_objects(&self) -> Result<HashSet<Self::Object>, NCategoryError>;

    fn get_all_cells(&self) -> Result<HashSet<&Self::Cell>, NCategoryError>;

    fn get_object_cells(
        &self,
        object_id: &Self::Identifier,
    ) -> Result<Vec<&Self::Cell>, NCategoryError>;

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

    fn get_cell(&self, cell_id: &Self::Identifier) -> Result<&Self::Cell, NCategoryError>;

    fn get_cell_source_object(&self, cell: &Self::Cell) -> Result<Self::Object, NCategoryError> {
        self.get_object(cell.source_object_id())
            .map_err(|_| NCategoryError::ObjectNotFound)
    }

    fn get_cell_target_object(&self, cell: &Self::Cell) -> Result<Self::Object, NCategoryError> {
        self.get_object(cell.target_object_id())
            .map_err(|_| NCategoryError::ObjectNotFound)
    }

    fn get_cell_tree(&self, cell_id: &Self::Identifier) -> Result<CellTree<Self::Identifier>, NCategoryError>
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
        left_cell_id: Vec<&Self::Identifier>,
        right_cell_id: Vec<&Self::Identifier>,
    ) -> Result<bool, NCategoryError> {

        self.validate_commutation(left_cell_id, right_cell_id)?;


        Ok(true)
    }

    fn validate_commutation(&self,
                            left_cell_ids: Vec<&Self::Identifier>,
                            right_cell_ids: Vec<&Self::Identifier>) -> Result<(), NCategoryError>
    {
        // source and target of left cells id should be same with right cells
        let left_source_object_id = self.get_cell(&left_cell_ids[0])?.source_object_id();
        let right_source_object_id = self.get_cell(&right_cell_ids[0])?.source_object_id();

        if left_source_object_id != right_source_object_id {
            return Err(NCategoryError::InvalidCellComposition);
        }

        let left_target_object_id = self.get_cell(&left_cell_ids[left_cell_ids.len() - 1])?.target_object_id();
        let right_target_object_id = self.get_cell(&right_cell_ids[right_cell_ids.len() - 1])?.target_object_id();

        if left_target_object_id != right_target_object_id {
            return Err(NCategoryError::InvalidCellComposition);
        }

        // confirm composition is correct
        self.validate_composition(left_cell_ids)?;
        self.validate_composition(right_cell_ids)?;

        Ok(())
    }

    fn validate_composition(&self, cell_ids: Vec<&Self::Identifier>) -> Result<(), NCategoryError>
    {
        if cell_ids.is_empty() {
            return Err(NCategoryError::InvalidCellComposition)
        }

        // composition of only once cell is always valid
        if cell_ids.len() <= 1 {
            return Ok(());
        }
        // target of first cell needs to be the source of subsequent cell
        let mut target_object_id = self.get_cell(&cell_ids[0])?.target_object_id();

        for cell_id in &cell_ids[1..] {
            let cell = self.get_cell(cell_id)?;

            if cell.source_object_id() != target_object_id {
                return Err(NCategoryError::InvalidCellComposition);
            }

            target_object_id = cell.target_object_id();
        }


        Ok(())
    }


    fn is_zero_category(&self) -> bool {
        false
    }


    fn base_object(&self, object_id: &Self::Identifier) -> Result<&Self::BaseCategory, NCategoryError>;

    fn nested_level() -> isize
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

    type Cell = Self;

    type BaseCategory = UnitCategory<T>;

    fn id(&self) -> &Self::Identifier {
        todo!()
    }

    fn add_object(&mut self, object: Self::Object) -> Result<Self::Identifier, NCategoryError> {
        todo!()
    }

    fn add_object_with_id(&mut self, object_id: Self::Identifier, object: Self::Object) -> Result<(), NCategoryError> {
        todo!()
    }

    fn add_cell(&mut self, cell: Self::Cell) -> Result<Self::Identifier, NCategoryError> {
        todo!()
    }

    fn get_object(&self, object_id: &Self::Identifier) -> Result<Self::Object, NCategoryError> {
        todo!()
    }

    fn get_identity_cell(
        &self,
        object_id: &Self::Identifier,
    ) -> Result<&Self::Cell, NCategoryError> {
        todo!()
    }

    fn get_all_objects(&self) -> Result<HashSet<Self::Object>, NCategoryError> {
        todo!()
    }

    fn get_all_cells(&self) -> Result<HashSet<&Self::Cell>, NCategoryError> {
        todo!()
    }

    fn get_object_cells(
        &self,
        object_id: &Self::Identifier,
    ) -> Result<Vec<&Self::Cell>, NCategoryError> {
        todo!()
    }

    fn get_cell(&self, cell_id: &Self::Identifier) -> Result<&Self::Cell, NCategoryError> {
        todo!()
    }

    fn base_object(&self, object_id: &Self::Identifier) -> Result<&Self::BaseCategory, NCategoryError> {
        todo!()
    }

    fn nested_level() -> isize { 0 }
}