use rand::{distributions::Alphanumeric, Rng};
use crate::core::ncategory::{NCategory};
use crate::core::identifier::{Identifier, ObjectId};
use crate::core::ncell::NCell;

pub trait NCategoryTestHelper<'a> {
    type Category: NCategory<'a>;

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

    fn generate_object(&mut self) -> <Self::Category as NCategory<'a>>::Object;

    fn expected_nested_level(&self) -> isize;

    // fn test_basic_object_cell(&mut self) {
    //     let object1_id = self.generate_identifier();
    //     let object1 = self.generate_object();
    //     let object2_id = self.generate_identifier();
    //     {
    //         let category = self.get_mut_category();
    //         category.add_object_with_id(object1_id, object1).unwrap();
    //     }
    //     {
    //         let category = self.get_category();
    //         // assert!(category.get_object(&object1_id).is_ok());
    //         //
    //         // // check identity morphism
    //         // let cell_ids = category.get_object_cells(&object1_id);
    //         // assert!(cell_ids.is_ok());
    //         // let cell_ids = cell_ids.unwrap();
    //         // assert_eq!(cell_ids.len(), 1);
    //         // let cell_id = cell_ids.first().unwrap();
    //         // assert_eq!(category.get_cell(cell_id).unwrap().source_object().object_id(), &object1_id);
    //         // assert_eq!(category.get_cell(cell_id).unwrap().target_object().object_id(), &object1_id);
    //         //
    //         // // check object 2 does not exist yet
    //         // assert!(!category.get_object(&object2_id).is_ok());
    //     }
    // }
}


pub fn random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}


// pub fn basic_object_cell_test<'a, CategoryTestHelper: NCategoryTestHelper<'a>>(
//     category_test_helper: &'a mut CategoryTestHelper,
// ) {
//     let object1_id = category_test_helper.generate_identifier();
//     let object1 = category_test_helper.generate_object();
//     let object2_id = category_test_helper.generate_identifier();
//
//     {
//         // Mutable borrow with short lifetime
//         let category = category_test_helper.get_mut_category();
//         category.add_object_with_id(object1_id.clone(), object1.clone()).unwrap();
//         category.get_object(&<CategoryTestHelper::Category as NCategory>::Identifier::generate()).unwrap();
//     } // Mutable borrow ends here
//
//     // {
//     //     // Immutable borrow after mutable borrow ends
//     //     let category = category_test_helper.get_category();
//     //     assert!(category.get_object(&object2_id).is_ok());
//     // }
// }

pub fn basic_object_cell_test<'a, CategoryTestHelper: NCategoryTestHelper<'a>>(mut category_test_helper: CategoryTestHelper){
    // add object 1
    let object1_id = category_test_helper.generate_identifier();
    let object1 = category_test_helper.generate_object();
    let object2_id = category_test_helper.generate_identifier();
    {
        let category = category_test_helper.get_mut_category();
        category.add_object_with_id(object1_id.clone(), object1).unwrap();
        assert!(category.get_object(&object1_id).is_ok());

        // check identity morphism
        let cell_ids = category.get_object_cells(&object1_id);
        assert!(cell_ids.is_ok());
        let cell_ids = cell_ids.unwrap();
        assert_eq!(cell_ids.len(), 1);
        let cell_id = cell_ids.first().unwrap();
        assert_eq!(category.get_cell(cell_id).unwrap().source_object().object_id(), &object1_id);
        assert_eq!(category.get_cell(cell_id).unwrap().target_object().object_id(), &object1_id);

        // TODO: implement comparison of the object assert_eq!(category.get_object(&object1_id).unwrap(), &object);

        // check object 2 does not exist yet
        assert!(!category.get_object(&object2_id).is_ok());
    }

    // add object 2
    let object2 = category_test_helper.generate_object();
    {
        let category = category_test_helper.get_mut_category();
        category.add_object_with_id(object2_id.clone(), object2).unwrap();
        assert!(category.get_object(&object2_id).is_ok());

        // check identity morphism
        let cell_ids = category.get_object_cells(&object2_id);
        assert!(cell_ids.is_ok());
        let cell_ids = cell_ids.unwrap();
        assert_eq!(cell_ids.len(), 1);
        let cell_id = cell_ids.first().unwrap();
        assert_eq!(category.get_cell(cell_id).unwrap().source_object().object_id(), &object2_id);
        assert_eq!(category.get_cell(cell_id).unwrap().target_object().object_id(), &object2_id);
    }

    // add object 3 without id
    let object3_id= {
        let object3 = category_test_helper.generate_object();
        let category = category_test_helper.get_mut_category();
        let object_id = category.add_object(object3);
        object_id
    };
    assert!(object3_id.is_ok());
    let object3_id = object3_id.unwrap().clone();
    // check object 3 exists
    {
        let category = category_test_helper.get_category();
        assert!(category.get_object(&object3_id).is_ok());

        // check identity morphism
        let cell_ids = category.get_object_cells(&object3_id);
        assert!(cell_ids.is_ok());
        let cell_ids = cell_ids.unwrap();
        assert_eq!(cell_ids.len(), 1);
        let cell = category.get_cell(cell_ids.first().unwrap()).unwrap();
        assert_eq!(cell.source_object().object_id(), &object3_id);
        assert_eq!(cell.target_object().object_id(), &object3_id);
    }

    let nested_level = <<CategoryTestHelper as NCategoryTestHelper>::Category as NCategory>::nested_level();
    assert_eq!(nested_level, category_test_helper.expected_nested_level());

    if nested_level <= 1{
        // Todo : Add tests for DiscreteCategory later
        return;
    }

    {
        // now we add two objects and a cell between them
        let cell_id = category_test_helper.generate_cell();
        let category = category_test_helper.get_category();
        let cell = category.get_cell(&cell_id).unwrap();
        let source_id = cell.source_object().object_id();
        let target_id = cell.source_object().object_id();
        assert!(category.get_object(source_id).is_ok());
        assert!(category.get_object(target_id).is_ok());

    }

    let (commuting_cell1, commuting_cell2) = {
        // now we generate two commuting cells
        category_test_helper.generate_commuting_cell()
    };
    {
        // check that the two commuting cells are indeed commuting
        let commute_result = category_test_helper.get_category().cells_commute(
            commuting_cell1.iter().collect(),
            commuting_cell2.iter().collect()
        );
        assert!(commute_result.is_ok());
        let commute_result = commute_result.unwrap();
        assert!(commute_result);
    }

    {
        // now we test for the non-commuting cells
        let (commuting_cell1, commuting_cell2) = category_test_helper.generate_non_commuting_cell();
        let commute_result = category_test_helper.get_category().cells_commute(
            commuting_cell1.iter().collect(),
            commuting_cell2.iter().collect()
        );
        assert!(commute_result.is_ok());
        let commute_result = commute_result.unwrap();
        assert!(!commute_result);
    }

}
