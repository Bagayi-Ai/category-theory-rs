use std::collections::HashSet;
use std::hash::Hash;
use uuid::Uuid;

use crate::core::ncategory::{NCategory};

#[derive(Clone)]
struct Cell<ObjectId> {
    from: ObjectId,
    to: ObjectId,
    name: String,
}

struct GenericNCategory<ObjectId, Object, CellId, H: NCategory<Object = Cell<ObjectId>>> {
    _phantom: std::marker::PhantomData<(Object, H, CellId)>
}

impl<ObjectId, Object, CellId, H: NCategory> NCategory for GenericNCategory<ObjectId, Object, CellId, H>
where
    ObjectId: Clone + PartialEq + Eq + Hash,
    CellId: Clone + PartialEq + Eq + Hash,
    H: NCategory<Object = Cell<ObjectId>>,
{
    type Object = Object;
    type ObjectId = ObjectId;
    type CellId = CellId;
    type Cell = Cell<ObjectId>;

    type Higher = H;

    fn source(m: &Self::Cell) -> &Self::ObjectId {
        // Placeholder implementation
        unimplemented!()
    }

    fn target(m: &Self::Cell) -> &Self::ObjectId {
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

    fn get_cell(&self, id: &Self::CellId) -> Option<&Self::Cell> {
        todo!()
    }


    fn commute(left: &Self::CellId, right: &Self::CellId) -> bool {
        // Placeholder implementation
        unimplemented!()
    }
}
