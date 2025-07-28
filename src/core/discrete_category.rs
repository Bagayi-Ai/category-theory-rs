use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use crate::core::identifier::Identifier;
use crate::core::ncategory::{NCategory, NCategoryError, UnitCategory};
use crate::core::ncell::NCell;
use crate::core::nfunctor::UnitFunctor;

pub struct DiscreteCategory<T> {
    category_id: T,
    // TODO: Find a way of avoiding storing identity cells
    // as we could derive them from the objects.
    cells: Option<HashMap<T ,Self>>

}

impl<T: Eq + Clone + Debug + Hash + Identifier> DiscreteCategory<T> {
    pub fn new() -> Self {
        DiscreteCategory {
            category_id: T::generate(),
            cells: Some(HashMap::new()),
        }
    }

    pub fn new_with_id(category_id: T) -> Self {
        DiscreteCategory {
            category_id,
            cells: None
        }
    }
}

impl<'a, T: Eq + Clone + Hash + Debug + Identifier + 'a> NCategory<'a > for DiscreteCategory<T>
{
    type Identifier = T;
    type Object = T;
    type Cell = Self;
    type BaseCategory = UnitCategory<T>;

    fn id(&self) -> &Self::Identifier {
        &self.category_id
    }

    fn add_object(&mut self, object: Self::Object) -> Result<&Self::Identifier, NCategoryError> {
        let cell = Self::new_with_id(object.clone());
        if let Some(cells) = &mut self.cells {
            let a = cells.insert(object.clone(), cell);
        } else {
            self.cells = Some(HashMap::new());
            self.cells.as_mut().unwrap().insert(object.clone(), cell);
        }

        let cell = self.cells.as_ref().unwrap().get(&object).unwrap();
        Ok(&NCell::id(cell))
    }

    fn add_object_with_id(&mut self, object_id: Self::Identifier, _object: Self::Object) -> Result<&Self::Identifier, NCategoryError> {
        self.add_object(object_id)
    }

    fn add_cell(&mut self, _cell: Self::Cell) -> Result<&Self::Identifier, NCategoryError> {
        Err(NCategoryError::OnlyIdentityCellDiscreteCategory)
    }

    fn get_object(&self, object_id: &Self::Identifier) -> Result<&Self::Object, NCategoryError> {
        if let Some(cells) = &self.cells {
            if let Some((key, _value)) = cells.get_key_value(object_id) {
                return Ok(key);
            }
        }
        Err(NCategoryError::ObjectNotFound)
    }

    fn get_identity_cell(&self, object_id: &Self::Identifier) -> Result<&Self::Identifier, NCategoryError> {
        self.get_object(object_id)
    }

    fn get_all_objects(&self) -> Result<HashSet<&Self::Identifier>, NCategoryError> {
        if let Some(cells) = &self.cells {
            Ok(cells.keys().collect())
        } else {
            Err(NCategoryError::NoObjectsInCategory)
        }
    }

    fn get_all_cells(&self) -> Result<HashSet<&Self::Identifier>, NCategoryError> {
        // In discrete the cells are only identity cells
        Ok(self.get_all_objects()?)
    }

    fn get_object_cells(&self, object_id: &Self::Identifier) -> Result<Vec<&Self::Identifier>, NCategoryError> {
        // only cell in discrete category is the identity cell.
        Ok(vec![self.get_identity_cell(object_id)?])
    }

    fn get_cell(&self, cell_id: &Self::Identifier) -> Result<&Self::Cell, NCategoryError> {
        if let Some(cells) = &self.cells {
            if let Some(cell) = cells.get(cell_id) {
                return Ok(cell);
            }
        }
        Err(NCategoryError::CellNotFound)
    }

    fn base_object(&self, _object_id: &Self::Identifier) -> Result<&Self::BaseCategory, NCategoryError> {
        todo!()
    }
}

impl<'a, T: Eq + Clone + Hash + Debug + Identifier + 'a> NCell<'a> for DiscreteCategory<T> {
    type Category = Self;
    type Functor = UnitFunctor<T>;

    fn id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn source_object(&self) -> &<Self::Category as NCategory>::Object {
        todo!()
    }

    fn target_object(&self) -> &<Self::Category as NCategory>::Object {
        todo!()
    }

    fn functor(&self) -> &Self::Functor {
        todo!()
    }
}

impl <T: Eq + Clone + Hash + Debug + Identifier> From<T> for DiscreteCategory<T>
{
    fn from(object: T) -> Self {
        let mut category = DiscreteCategory::new();
        category.add_object(object).unwrap();
        category
    }
}

impl <T: Eq + Clone + Hash + Debug + Identifier> From<Vec<T>> for DiscreteCategory<T>
{
    fn from(objects: Vec<T>) -> Self {
        let mut category = DiscreteCategory::new();
        for object in objects {
            category.add_object(object).unwrap();
        }
        category
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::ncategory_test_helper::*;

    struct GenericCategory0TestHelper {
        category: DiscreteCategory<String>,
    }

    impl GenericCategory0TestHelper {
        pub fn new() -> Self {
            GenericCategory0TestHelper {
                category: DiscreteCategory::new(),
            }
        }
    }

    impl NCategoryTestHelper<'_> for GenericCategory0TestHelper {
        type Category = DiscreteCategory<String>;

        fn get_category(&self) -> &Self::Category {
            &self.category
        }

        fn get_mut_category(&mut self) -> &mut Self::Category {
            &mut self.category
        }

        fn generate_cell(&mut self) -> <Self::Category as NCategory>::Identifier {
            todo!()
        }

        fn generate_commuting_cell(&mut self) -> (Vec<<Self::Category as NCategory>::Identifier>, Vec<<Self::Category as NCategory>::Identifier>) {
            todo!()
        }

        fn generate_non_commuting_cell(&mut self) -> (Vec<<Self::Category as NCategory>::Identifier>, Vec<<Self::Category as NCategory>::Identifier>) {
            todo!()
        }


        fn generate_object(&mut self) -> <Self::Category as NCategory>::Object {
            random_string(5)
        }

        fn expected_nested_level(&self) -> isize {
            1
        }

    }
    #[test]
    pub fn test_base_scenarios() {
        let category_test_helper = GenericCategory0TestHelper::new();
        basic_object_cell_test(category_test_helper);
    }
}
