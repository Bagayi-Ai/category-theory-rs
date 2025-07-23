// use std::collections::HashMap;
// use super::*;
// use crate::core::tests::ncategory_test_helper::*;
// use crate::core::discrete_category::Category0;
// use crate::core::ncategory::NCategory;
// use crate::core::uuid_id::UuidCategoryObjectId;
//
// type Category0String = Category0<String>;
//
// struct GenericCategory1TestHelper {
//     category: crate::core::generic_ncategory::GenericNCategory<UuidCategoryObjectId, Category0String>,
// }
//
// impl GenericCategory1TestHelper {
//     pub fn new() -> Self {
//         GenericCategory1TestHelper {
//             category: crate::core::generic_ncategory::GenericNCategory {
//                 objects: HashMap::new(),
//                 object_mapping: HashMap::new(),
//                 cells: HashMap::new(),
//             },
//         }
//     }
// }
//
// impl NCategoryTestHelper for GenericCategory1TestHelper {
//     type Category = crate::core::generic_ncategory::GenericNCategory<UuidCategoryObjectId, Category0String>;
//
//     fn get_category(&self) -> &Self::Category {
//         &self.category
//     }
//
//     fn get_mut_category(&mut self) -> &mut Self::Category {
//         &mut self.category
//     }
//
//     fn generate_object_id(&self) -> crate::core::ncategory::ObjectId {
//         UuidCategoryObjectId::new()
//     }
//
//     fn generate_cell_id(&self) -> crate::core::ncategory::CellId {
//         UuidCategoryObjectId::new()
//     }
//
//     fn generate_cell(&mut self) -> crate::core::ncategory::CellId {
//         let object1 = self.generate_object();
//         let object2 = self.generate_object();
//         let object1_id = self.get_mut_category().add_object(object1).unwrap();
//         let object2_id = self.get_mut_category().add_object(object2).unwrap();
//         let cell_id = self.generate_cell_id();
//         let cell = crate::core::generic_ncategory::Cell {
//             id: cell_id.clone(),
//             from: object1_id,
//             to: object2_id,
//             name: "test_cell".to_string(),
//             _phantom: std::marker::PhantomData,
//         };
//         self.get_mut_category().add_cell(cell).unwrap();
//         cell_id
//     }
//
//     fn generate_commuting_cell(&mut self) -> (Vec<crate::core::ncategory::CellId>, Vec<crate::core::ncategory::CellId>) {
//         // have 3 Category0 objects A, B, C
//         // A will contain {a,b,c} as objects
//         // B will contain {1, 2, 3, 4, 5} as objects
//         // C will contain {x, y, z} as objects
//         let objectA = Category0::from(
//             vec!["a", "b", "c"]
//                 .into_iter()
//                 .map(|s| s.to_string())
//                 .collect::<Vec<String>>());
//         let objectA_id = self.get_mut_category().add_object(objectA).unwrap();
//         let objectB = Category0::from(
//             vec!["1", "2", "3", "4", "5"]
//                 .into_iter()
//                 .map(|s| s.to_string())
//                 .collect::<Vec<String>>());
//         let objectB_id = self.get_mut_category().add_object(objectB).unwrap();
//         let objectC = Category0::from(
//             vec!["x", "y", "z"]
//                 .into_iter()
//                 .map(|s| s.to_string())
//                 .collect::<Vec<String>>());
//         let objectC_id = self.get_mut_category().add_object(objectC).unwrap();
//
//         // We add three Cells that are commuting
//         // A -> B, B -> C, C -> A
//         let cell1_id = self.generate_cell_id();
//         let cell1 = crate::core::generic_ncategory::Cell {
//             id: cell1_id.clone(),
//             from: objectA_id.clone(),
//             to: objectB_id.clone(),
//             name: "cell_A_to_B".to_string(),
//             _phantom: std::marker::PhantomData,
//         };
//         self.get_mut_category().add_cell(cell1).unwrap();
//
//         let cell2_id = self.generate_cell_id();
//         let cell2 = crate::core::generic_ncategory::Cell {
//             id: cell2_id.clone(),
//             from: objectB_id.clone(),
//             to: objectC_id.clone(),
//             name: "cell_B_to_C".to_string(),
//             _phantom: std::marker::PhantomData,
//         };
//         self.get_mut_category().add_cell(cell2).unwrap();
//
//         let cell3_id = self.generate_cell_id();
//         let cell3 = crate::core::generic_ncategory::Cell {
//             id: cell3_id.clone(),
//             from: objectC_id.clone(),
//             to: objectA_id.clone(),
//             name: "cell_C_to_A".to_string(),
//             _phantom: std::marker::PhantomData,
//         };
//         self.get_mut_category().add_cell(cell3).unwrap();
//
//         (
//             vec![cell1_id, cell2_id],
//             vec![cell3_id],
//         )
//     }
//
//     fn generate_non_commuting_cell(&mut self) -> (Vec<crate::core::ncategory::CellId>, Vec<crate::core::ncategory::CellId>) {
//         // have 3 Category0 objects D, E, F
//         // D will contain {da, db, dc} as objects
//         // E will contain {11, 12, 13, 14, 15} as objects
//         // F will contain {xa, yb, zc} as objects
//         let objectD = Category0::from(
//             vec!["da", "db", "dc"]
//                 .into_iter()
//                 .map(|s| s.to_string())
//                 .collect::<Vec<String>>());
//         let objectD_id = self.get_mut_category().add_object(objectD).unwrap();
//         let objectE = Category0::from(
//             vec!["11", "12", "13", "14", "15"]
//                 .into_iter()
//                 .map(|s| s.to_string())
//                 .collect::<Vec<String>>());
//         let objectE_id = self.get_mut_category().add_object(objectE).unwrap();
//         let objectF = Category0::from(
//             vec!["xa", "yb", "zc"]
//                 .into_iter()
//                 .map(|s| s.to_string())
//                 .collect::<Vec<String>>());
//         let objectF_id = self.get_mut_category().add_object(objectF).unwrap();
//
//         // We add three Cells that are commuting
//         // D -> E, E -> F, D -> F
//         let cell1_id = self.generate_cell_id();
//         let cell1 = crate::core::generic_ncategory::Cell {
//             id: cell1_id.clone(),
//             from: objectD_id.clone(),
//             to: objectE_id.clone(),
//             name: "cell_D_to_E".to_string(),
//             _phantom: std::marker::PhantomData,
//         };
//         self.get_mut_category().add_cell(cell1).unwrap();
//
//         let cell2_id = self.generate_cell_id();
//         let cell2 = crate::core::generic_ncategory::Cell {
//             id: cell2_id.clone(),
//             from: objectE_id.clone(),
//             to: objectF_id.clone(),
//             name: "cell_E_to_F".to_string(),
//             _phantom: std::marker::PhantomData,
//         };
//         self.get_mut_category().add_cell(cell2).unwrap();
//
//         let cell3_id = self.generate_cell_id();
//         let cell3 = crate::core::generic_ncategory::Cell {
//             id: cell3_id.clone(),
//             from: objectD_id.clone(),
//             to: objectF_id.clone(),
//             name: "cell_D_to_F".to_string(),
//             _phantom: std::marker::PhantomData,
//         };
//         self.get_mut_category().add_cell(cell3).unwrap();
//
//         (
//             vec![cell1_id, cell2_id],
//             vec![cell3_id],
//         )
//     }
//
//     fn generate_object(&mut self) -> crate::core::ncategory::Object {
//         let random_string = random_string(5);
//         let mut object = Category0::new();
//         object.add_object(random_string).unwrap();
//         object
//     }
//
//     fn expected_category_level(&self) -> isize {
//         1
//     }
//
// }
// #[test]
// pub fn test_base_scenarios() {
//     let category_test_helper = GenericCategory1TestHelper::new();
//     basic_object_cell_test(category_test_helper);
// }