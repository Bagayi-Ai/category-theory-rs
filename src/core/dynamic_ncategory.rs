use crate::core::ncategory::{NCategory, NCategoryError};

#[derive(Clone, Debug)]
struct CellDynamic<T> {
    from: T,
    to: T,
    name: String,
    children: Vec<CellDynamic<T>>,
}

struct DynamicNCategory<T> {
    _phantom: std::marker::PhantomData<T>,
}


impl <T: Clone + PartialEq> NCategory for DynamicNCategory<T>
where
    T: Clone + PartialEq,
{
    type Object = T;
    type ObjectId = ();
    type CellId = ();
    type Cell = CellDynamic<Self::ObjectId>;
    type BaseCategory = Self;

    fn source(&self, cell_id: &Self::CellId) -> Result<&Self::ObjectId, NCategoryError> {
        todo!()
    }

    fn target(&self, cell_id: &Self::CellId) -> Result<&Self::ObjectId, NCategoryError> {
        todo!()
    }

    fn add_object(&mut self, object: Self::Object) -> Result<Self::ObjectId, NCategoryError> {
        todo!()
    }

    fn add_object_with_id(&mut self, objectId: Self::ObjectId, object: Self::Object) -> Result<(), NCategoryError> {
        todo!()
    }

    fn add_cell(&mut self, cell: Self::Cell) -> Result<Self::CellId, NCategoryError> {
        todo!()
    }

    fn get_object(&self, objectId: &Self::ObjectId) -> Result<&Self::Object, NCategoryError> {
        todo!()
    }

    fn get_object_cells(&self, objectId: &Self::ObjectId) -> Result<Vec<&Self::CellId>, NCategoryError> {
        todo!()
    }

    fn get_cell(&self, cell_id: &Self::CellId) -> Result<&Self::Cell, NCategoryError> {
        todo!()
    }

    fn commute(left_cell_id: &Self::CellId, right_cell_id: &Self::CellId) -> Result<bool, NCategoryError> {
        todo!()
    }

    fn base_object(&self, object_id: &Self::ObjectId) -> Result<&Self::BaseCategory, NCategoryError> {
        todo!()
    }
}
