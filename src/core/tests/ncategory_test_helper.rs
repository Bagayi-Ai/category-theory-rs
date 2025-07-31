use rand::{distributions::Alphanumeric, Rng};
use crate::core::discrete_category::DiscreteCategory;
use crate::core::ncategory::{NCategory};
use crate::core::identifier::Identifier;
use crate::core::ncell::NCell;
use crate::core::tests::test_generic_ncategory::GenericCategory1TestHelper;

pub trait NCategoryTestHelper<'a> {
    type Category: NCategory<'a>;
    type CategoryObject;

    fn get_category(&self) -> &Self::Category;
    fn get_mut_category(&mut self) -> &mut Self::Category;
    fn generate_cell(&mut self) -> <Self::Category as NCategory<'a>>::Identifier;

    fn generate_identifier(&self) -> <Self::Category as NCategory<'a>>::Identifier {
        <Self::Category as NCategory>::Identifier::generate()
    }

    fn generate_commuting_cell(
        &mut self
    ) -> (Vec<<Self::Category as NCategory<'a>>::Identifier>, Vec<<Self::Category as NCategory<'a>>::Identifier>);

    fn generate_non_commuting_cell(
        &mut self
    ) -> (Vec<<Self::Category as NCategory<'a>>::Identifier>, Vec<<Self::Category as NCategory<'a>>::Identifier>);

    fn generate_object(&self) -> Self::CategoryObject;

    fn expected_nested_level(&self) -> isize;

}


pub fn random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub fn basic_object_cell_test<'a, F, CategoryTestHelper>(mut category_test_helper_factory: F)
where
    F: FnOnce() -> CategoryTestHelper,
    CategoryTestHelper: NCategoryTestHelper<'a> + 'a,
    // <<CategoryTestHelper as NCategoryTestHelper<'a>>::Category as NCategory<'a>>::Object: From<<CategoryTestHelper as NCategoryTestHelper<'a>>::CategoryObject>
{
    // let mut generated_objects = Vec::new();
    let mut category_test_helper = category_test_helper_factory();
    // let mut category_test_helper = GenericCategory1TestHelper::new(&mut generated_objects);
    // add object 1
    let object1_id = category_test_helper.generate_identifier();
    let object1 = category_test_helper.generate_object();
    let object2_id = category_test_helper.generate_identifier();
    {
        let category = category_test_helper.get_mut_category();
        // category.add_object_with_id(object1_id.clone(), object1).unwrap();
        assert!(category.get_object(&object1_id).is_ok());

        // // check identity morphism
        // let cell = category.get_object_cells(&object1_id);
        // assert!(cell.is_ok());
        // let cell = cell.unwrap();
        // assert_eq!(cell.len(), 1);
        // let cell = cell.first().unwrap();
        // assert_eq!(cell.source_object_id(), &object1_id);
        // assert_eq!(cell.target_object_id(), &object1_id);
        //
        // // TODO: implement comparison of the object assert_eq!(category.get_object(&object1_id).unwrap(), &object);
        //
        // // check object 2 does not exist yet
        // assert!(!category.get_object(&object2_id).is_ok());
    }

    // // add object 2
    // let object2_index = category_test_helper.generate_object_index();
    // let object2 = category_test_helper.get_generated_objects(object2_index);
    // {
    //     let category = category_test_helper.get_mut_category();
    //     category.add_object_with_id(object2_id.clone(), &object2).unwrap();
    //     assert!(category.get_object(&object2_id).is_ok());
    //
    //     // check identity morphism
    //     let cells = category.get_object_cells(&object2_id);
    //     assert!(cells.is_ok());
    //     let cells = cells.unwrap();
    //     assert_eq!(cells.len(), 1);
    //     let cell = cells.first().unwrap();
    //     assert_eq!(cell.source_object_id(), &object2_id);
    //     assert_eq!(cell.target_object_id(), &object2_id);
    // }
    //
    // add object 3 without id
    // let object3_id = {
    //     let object3 = category_test_helper.generate_object();
    //     let category = category_test_helper.get_mut_category();
    //     let object_id = category.add_object(&object3);
    //     object_id.clone()
    // };
    // assert!(object3_id.is_ok());
    // let object3_id = object3_id.unwrap();
    // // check object 3 exists
    // {
    //     let category = category_test_helper.get_category();
    //     assert!(category.get_object(&object3_id).is_ok());
    //
    //     // check identity morphism
    //     let cells = category.get_object_cells(&object3_id);
    //     assert!(cells.is_ok());
    //     let cells = cells.unwrap();
    //     assert_eq!(cells.len(), 1);
    //     let cell = cells.first().unwrap();
    //     assert_eq!(cell.source_object_id(), &object3_id);
    //     assert_eq!(cell.target_object_id(), &object3_id);
    // }
    //
    // let nested_level = <<CategoryTestHelper as NCategoryTestHelper>::Category as NCategory>::nested_level();
    // assert_eq!(nested_level, category_test_helper.expected_nested_level());
    //
    // if nested_level <= 1{
    //     // Todo : Add tests for DiscreteCategory later
    //     return;
    // }
    //
    // {
    //     // now we add two objects and a cell between them
    //     let cell_id = category_test_helper.generate_cell();
    //     let category = category_test_helper.get_category();
    //     let cell = category.get_cell(&cell_id).unwrap();
    //     let source_id = cell.source_object_id();
    //     let target_id = cell.target_object_id();
    //     assert!(category.get_object(source_id).is_ok());
    //     assert!(category.get_object(target_id).is_ok());
    // }
    //
    // {
    //     // now we test for the commuting cells
    //     let (commuting_cell1, commuting_cell2) = category_test_helper.generate_commuting_cell();
    //     let commute_result = category_test_helper.get_category().cells_commute(
    //         commuting_cell1.iter().collect(),
    //         commuting_cell2.iter().collect()
    //     );
    //     assert!(commute_result.is_ok());
    //     let commute_result = commute_result.unwrap();
    //     assert!(commute_result);
    // }

    // {
    //     // now we test for the non-commuting cells
    //     let (commuting_cell1, commuting_cell2) = category_test_helper.generate_non_commuting_cell();
    //     let commute_result = category_test_helper.get_category().cells_commute(
    //         commuting_cell1.iter().collect(),
    //         commuting_cell2.iter().collect()
    //     );
    //     assert!(commute_result.is_ok());
    //     let commute_result = commute_result.unwrap();
    //     assert!(!commute_result);
    // }

}
