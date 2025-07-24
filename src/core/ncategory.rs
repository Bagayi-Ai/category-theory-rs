use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::fmt::Debug;

use crate::core::ncell::{NCell};
use crate::core::identifier::Identifier;


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
}

pub enum RecursiveCellMap<CellId> {
    Leaf,
    Node(
        HashMap<
            CellId, // source cell id
            (
                CellId, // target cell id
                // children maps strating all over from the target object of the cell
                Option<Vec<RecursiveCellMap<CellId>>>,
            )
        >
    ),
}


pub trait NCategory
where
    Self::BaseCategory: NCategory<Identifier = Self::Identifier>,
{
    type Identifier: Identifier;
    type Object;
    type Cell: NCell<Category = Self>;
    type BaseCategory: NCategory;

    fn id(&self) -> &Self::Identifier;

    fn add_object(&mut self, object: Self::Object) -> Result<Self::Identifier, NCategoryError>;

    fn add_object_with_id(&mut self, object_id: Self::Identifier, object: Self::Object) -> Result<(), NCategoryError>;

    fn add_cell(&mut self, cell: Self::Cell) -> Result<Self::Identifier, NCategoryError>;

    fn get_object(&self, object_id: &Self::Identifier) -> Result<&Self::Object, NCategoryError>;

    fn get_identity_cell(
        &self,
        object_id: &Self::Identifier,
    ) -> Result<&Self::Identifier, NCategoryError>;

    fn get_all_objects(&self) -> Result<HashSet<&Self::Identifier>, NCategoryError>;

    fn get_all_cells(&self) -> Result<HashSet<&Self::Identifier>, NCategoryError>;

    fn get_object_cells(
        &self,
        object_id: &Self::Identifier,
    ) -> Result<Vec<&Self::Identifier>, NCategoryError>;

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

impl <T: Identifier> NCategory for UnitCategory<T> {
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

    fn get_object(&self, object_id: &Self::Identifier) -> Result<&Self::Object, NCategoryError> {
        todo!()
    }

    fn get_identity_cell(
        &self,
        object_id: &Self::Identifier,
    ) -> Result<&Self::Identifier, NCategoryError> {
        todo!()
    }

    fn get_all_objects(&self) -> Result<HashSet<&Self::Identifier>, NCategoryError> {
        todo!()
    }

    fn get_all_cells(&self) -> Result<HashSet<&Self::Identifier>, NCategoryError> {
        todo!()
    }

    fn get_object_cells(
        &self,
        object_id: &Self::Identifier,
    ) -> Result<Vec<&Self::Identifier>, NCategoryError> {
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