use std::hash::Hash;
use std::fmt::Debug;

use crate::core::ncategory::{NCategory};

pub trait CategoryLevel {
    fn level() -> usize;
}

#[derive(Clone)]
struct Cell<ObjectId> {
    from: ObjectId,
    to: ObjectId,
    name: String,
}


struct GenericNCategory<ObjectId, Object, CellId, BaseCategory: NCategory<Object = Cell<ObjectId>>> {
    base_category: BaseCategory,
    _phantom: std::marker::PhantomData<(Object, CellId)>
}

impl<ObjectId, Object, CellId, BaseCategory: NCategory> NCategory for GenericNCategory<ObjectId, Object, CellId, BaseCategory>
where
    ObjectId: Clone + PartialEq + Eq + Hash + Debug,
    CellId: Clone + PartialEq + Eq + Hash + Debug,
    BaseCategory: NCategory<Object = Cell<ObjectId>>,
{
    type Object = Object;
    type ObjectId = ObjectId;
    type CellId = CellId;
    type Cell = Cell<ObjectId>;

    type BaseCategory = BaseCategory;

    fn source(&self, cell_id: &Self::CellId) -> &Self::ObjectId {
        // Placeholder implementation
        unimplemented!()
    }

    fn target(&self, cell_id: &Self::CellId) -> &Self::ObjectId {
        // Placeholder implementation
        unimplemented!()
    }

    fn add_object(&mut self, o: Self::Object) -> &Self::ObjectId {
        todo!()
    }

    fn add_object_with_id(&mut self, objectId: Self::ObjectId, object: Self::Object) {
        todo!()
    }

    fn add_cell(&mut self, m: Self::Cell) {
        todo!()
    }

    fn get_object(&self, id: &Self::ObjectId) -> Option<&Self::Object> {
        todo!()
    }

    fn get_object_cells(&self, objectId: &Self::ObjectId) -> Vec<&Self::CellId> {
        todo!()
    }

    fn get_cell(&self, id: &Self::CellId) -> Option<&Self::Cell> {
        todo!()
    }


    fn commute(left: &Self::CellId, right: &Self::CellId) -> bool {
        // Placeholder implementation
        unimplemented!()
    }

    fn base_category(&self) -> &Self::BaseCategory {
        &self.base_category
    }
}
