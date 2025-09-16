use crate::core::arrow::{Arrow, Functor, Morphism};
use crate::core::base_category::*;
use crate::core::discrete_category::DiscreteCategory;
use crate::core::dynamic_category::DynamicCategory;
use crate::core::identifier::Identifier;
use crate::core::object_id::ObjectId;
use crate::core::tests::ncategory_test_helper::*;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use std::collections::HashMap;
use std::rc::Rc;

fn generate_identifier() -> String {
    String::generate()
}

fn generate_object() -> Rc<DiscreteCategory> {
    let random_string = random_string(5);
    let mut object = DiscreteCategory::new();
    object
        .add_object(Rc::new(DiscreteCategory::new_with_id(ObjectId::Str(
            random_string,
        ))))
        .unwrap();
    Rc::new(object)
}

#[test]
pub fn test_base_scenarios() {
    let mut category: BaseCategory<DiscreteCategory> = BaseCategory::new();
    // add object 1
    let object1 = generate_object();

    category.add_object(object1.clone()).unwrap();
    // check identity morphism
    let cell = category.get_object_morphisms(&*object1);
    assert!(cell.is_ok());
    let cell = cell.unwrap();
    assert_eq!(cell.len(), 1);
    let cell = cell.first().unwrap();
    assert!(cell.source_object().equal_to(&*object1));
    assert!(cell.target_object().equal_to(&*object1));

    // TODO: implement comparison of the object assert_eq!(category.get_object(&object1_id).unwrap(), &object);

    // check identity morphism
    let cell = category.get_object_morphisms(&*object1);
    assert!(cell.is_ok());
    let cell = cell.unwrap();
    assert_eq!(cell.len(), 1);
    let cell = cell.first().unwrap();
    assert!(cell.source_object().equal_to(&*object1));
    assert!(cell.target_object().equal_to(&*object1));

    // TODO: implement comparison of the object assert_eq!(category.get_object(&object1_id).unwrap(), &object);

    // add object 2
    let object2 = generate_object();
    category.add_object(object2.clone()).unwrap();

    // check identity morphism
    let cells = category.get_object_morphisms(&*object2);
    assert!(cells.is_ok());
    let cells = cells.unwrap();
    assert_eq!(cells.len(), 1);
    let cell = cells.first().unwrap();
    assert!(cell.source_object().equal_to(&*object2));
    assert!(cell.target_object().equal_to(&*object2));

    // add object 3
    let object3 = generate_object();
    category.add_object(object3.clone());

    // check identity morphism
    let cells = category.get_object_morphisms(&*object3);
    assert!(cells.is_ok());
    let cells = cells.unwrap();
    assert_eq!(cells.len(), 1);
    let cell = cells.first().unwrap();
    assert!(cell.source_object().equal_to(&*object3));
    assert!(cell.target_object().equal_to(&*object3));

    // // now add a cell between object1 and object2
    // let cell_id = generate_identifier();
    // let cell = Arrow::new(cell_id.clone(), &object1, &object2);
    // category.add_morphism(cell).unwrap();
    //
    // let cell = category.get_moprhism(&cell_id).unwrap();
    // assert_eq!(cell.source_object(), &object1);
    // assert_eq!(cell.target_object(), &object2);

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

#[test]
pub fn test_identity_cell_tree() {
    // Discrete category A with a, b, c as objects
    let mut discreteCategoryALower = Rc::new(DiscreteCategory::new_with_id(ObjectId::Str(
        "alphabet_lower".to_string(),
    )));
    let object_a: Rc<DiscreteCategory> = Rc::new("a".into());
    let object_b: Rc<DiscreteCategory> = Rc::new("b".into());
    let object_c: Rc<DiscreteCategory> = Rc::new("c".into());
    Rc::get_mut(&mut discreteCategoryALower)
        .unwrap()
        .add_object(object_a.clone())
        .unwrap();
    Rc::get_mut(&mut discreteCategoryALower)
        .unwrap()
        .add_object(object_b.clone())
        .unwrap();
    Rc::get_mut(&mut discreteCategoryALower)
        .unwrap()
        .add_object(object_c.clone())
        .unwrap();

    // Discrete category A with a, b, c as objects
    let mut discreteCategoryAUpper = Rc::new(DiscreteCategory::new_with_id(ObjectId::Str(
        "alphabet_upper".to_string(),
    )));
    let object_A: Rc<DiscreteCategory> = Rc::new("A".into());
    let object_B: Rc<DiscreteCategory> = Rc::new("B".into());
    let object_C: Rc<DiscreteCategory> = Rc::new("C".into());
    Rc::get_mut(&mut discreteCategoryAUpper)
        .unwrap()
        .add_object(object_A.clone())
        .unwrap();
    Rc::get_mut(&mut discreteCategoryAUpper)
        .unwrap()
        .add_object(object_B.clone())
        .unwrap();
    Rc::get_mut(&mut discreteCategoryAUpper)
        .unwrap()
        .add_object(object_C.clone())
        .unwrap();

    let mut discreteCategoryANumber = Rc::new(DiscreteCategory::new());
    let object_1: Rc<DiscreteCategory> = Rc::new(1.into());
    let object_2: Rc<DiscreteCategory> = Rc::new(2.into());
    let object_3: Rc<DiscreteCategory> = Rc::new(3.into());
    Rc::get_mut(&mut discreteCategoryANumber)
        .unwrap()
        .add_object(object_1.clone())
        .unwrap();
    Rc::get_mut(&mut discreteCategoryANumber)
        .unwrap()
        .add_object(object_2.clone())
        .unwrap();
    Rc::get_mut(&mut discreteCategoryANumber)
        .unwrap()
        .add_object(object_3.clone())
        .unwrap();

    let lower_to_numer_mappings: HashMap<
        Rc<Morphism<DiscreteCategory>>,
        Rc<Morphism<DiscreteCategory>>,
    > = HashMap::from([
        // a to 1
        (
            discreteCategoryALower
                .get_identity_morphism(&*object_a)
                .unwrap()
                .clone(),
            discreteCategoryANumber
                .get_identity_morphism(&*object_1)
                .unwrap()
                .clone(),
        ),
        // b to 2
        (
            discreteCategoryALower
                .get_identity_morphism(&*object_b)
                .unwrap()
                .clone(),
            discreteCategoryANumber
                .get_identity_morphism(&*object_2)
                .unwrap()
                .clone(),
        ),
        // c to 3
        (
            discreteCategoryALower
                .get_identity_morphism(&*object_c)
                .unwrap()
                .clone(),
            discreteCategoryANumber
                .get_identity_morphism(&*object_3)
                .unwrap()
                .clone(),
        ),
    ]);
    // create a functor from lower to number
    let functor_lower_to_number = Functor::new(
        "functor_lower_to_number".to_string(),
        discreteCategoryALower.clone(),
        discreteCategoryANumber.clone(),
        lower_to_numer_mappings,
    );

    let number_to_upper_mappings: HashMap<
        Rc<Morphism<DiscreteCategory>>,
        Rc<Morphism<DiscreteCategory>>,
    > = HashMap::from([
        // 1 to a
        (
            discreteCategoryANumber
                .get_identity_morphism(&*object_1)
                .unwrap()
                .clone(),
            discreteCategoryALower
                .get_identity_morphism(&*object_a)
                .unwrap()
                .clone(),
        ),
        // b to 2
        (
            discreteCategoryANumber
                .get_identity_morphism(&*object_2)
                .unwrap()
                .clone(),
            discreteCategoryALower
                .get_identity_morphism(&*object_b)
                .unwrap()
                .clone(),
        ),
        // c to c
        (
            discreteCategoryANumber
                .get_identity_morphism(&*object_3)
                .unwrap()
                .clone(),
            discreteCategoryALower
                .get_identity_morphism(&*object_c)
                .unwrap()
                .clone(),
        ),
    ]);

    // create a functor1 from lower to upper
    let lower_to_upper_mappings: HashMap<
        Rc<Morphism<DiscreteCategory>>,
        Rc<Morphism<DiscreteCategory>>,
    > = HashMap::from([
        // a to A
        (
            discreteCategoryALower
                .get_identity_morphism(&*object_a)
                .unwrap()
                .clone(),
            discreteCategoryAUpper
                .get_identity_morphism(&*object_A)
                .unwrap()
                .clone(),
        ),
        // b to B
        (
            discreteCategoryALower
                .get_identity_morphism(&*object_b)
                .unwrap()
                .clone(),
            discreteCategoryAUpper
                .get_identity_morphism(&*object_B)
                .unwrap()
                .clone(),
        ),
        // c to C
        (
            discreteCategoryALower
                .get_identity_morphism(&*object_c)
                .unwrap()
                .clone(),
            discreteCategoryAUpper
                .get_identity_morphism(&*object_C)
                .unwrap()
                .clone(),
        ),
    ]);

    let mut setCategoryAlphabet = BaseCategory::new();

    // Add the discrete category A as an object in Set category alphabet
    setCategoryAlphabet
        .add_object(discreteCategoryALower.clone())
        .unwrap();

    // Add the discrete category A as an object in Set category alphabet
    setCategoryAlphabet
        .add_object(discreteCategoryAUpper.clone())
        .unwrap();

    let identity_cell = setCategoryAlphabet
        .get_identity_morphism(&*discreteCategoryALower)
        .unwrap();
    assert!(
        identity_cell
            .source_object()
            .equal_to(&*discreteCategoryALower)
    );
    assert!(
        identity_cell
            .target_object()
            .equal_to(&*discreteCategoryALower)
    );

    // now add morphism from lower to upper
    let morphism = Rc::new(Morphism::new(
        "lower_to_upper".to_string(),
        discreteCategoryALower.clone(),
        discreteCategoryAUpper.clone(),
        lower_to_upper_mappings,
    ));
    setCategoryAlphabet.add_morphism(morphism).unwrap();

    // let product_endo_functor = apply_endofunctor(
    //     "product_endo_functor".to_string(),
    //     discreteCategoryALower.clone(),
    //     discreteCategoryANumber.clone(),
    // )
    // .unwrap();
    // // create a morphism from lower to upper using the functor
    // let morphism2 = Rc::new(Morphism::new(
    //     "lower_to_upper".to_string(),
    //     (*product_endo_functor.source_object()).clone(),
    //     (*product_endo_functor.target_object()).clone(),
    //     product_endo_functor,
    // ));
    //
    // // add the morphism to the set category alphabet
    // setCategoryAlphabet.add_morphism(morphism2).unwrap();

    // add a new object that is a result of

    // assert!(morphism.validate_composition().is_ok());
    // assert!(morphism.validate_commutation(&morphism).is_ok());
    //
    // assert!(functor_lower_to_upper.validate_composition().is_ok());
    // assert!(functor_lower_to_upper.validate_commutation(&functor_lower_to_upper).is_ok());
    // assert!(functor_lower_to_upper.validate_mappings().is_ok());

    // let actual_cell_tree = setCategoryAlphabet.get_cell_tree(identity_cell).unwrap();

    // expected cell tree
    // all the cells of the discrete category A remain as children of the identity cell
    // let expected_cell_tree = CellTree::new_with_children(
    //     identity_cell.id(),
    //     identity_cell.source_object_id(),
    //     identity_cell.target_object_id(),
    //     vec![
    //         CellTree::new(
    //             &object_a,
    //             &object_a,
    //             &object_a
    //         ),
    //         CellTree::new(
    //             &object_b,
    //             &object_b,
    //             &object_b
    //         ),
    //         CellTree::new(
    //             &object_c,
    //             &object_c,
    //             &object_c
    //         )
    //     ]
    // );
    //
    // assert_eq!(actual_cell_tree, expected_cell_tree);
    //

    // // Add the discrete category A as an object in Set category alphabet
    // setCategoryAlphabet.add_object(
    //     &discreteCategoryAUpper).unwrap();
    //
    // let identity_cell = setCategoryAlphabet.get_identity_cell(&discreteCategoryAUpper).unwrap();
    // assert_eq!(identity_cell.source_object_id(), NCategory::category_id(&discreteCategoryAUpper));
    // assert_eq!(identity_cell.target_object_id(), NCategory::category_id(&discreteCategoryAUpper));
    //
    // let actual_cell_tree = setCategoryAlphabet.get_cell_tree(identity_cell).unwrap();
    //
    // let expected_cell_tree = CellTree::new_with_children(
    //     identity_cell.id(),
    //     identity_cell.source_object_id(),
    //     identity_cell.target_object_id(),
    //     vec![
    //         CellTree::new(
    //             &object_A,
    //             &object_A,
    //             &object_A
    //         ),
    //         CellTree::new(
    //             &object_B,
    //             &object_B,
    //             &object_B
    //         ),
    //         CellTree::new(
    //             &object_C,
    //             &object_C,
    //             &object_C
    //         )
    //     ]
    // );
    //
    // assert_eq!(actual_cell_tree, expected_cell_tree);
    //
    // // now add a cell between the two objects
    // let cell_id = generate_identifier();
    // let cell = GenericNCell::new(
    //     cell_id.clone(),
    //     NCategory::category_id(&discreteCategoryALower).clone(),
    //     NCategory::category_id(&discreteCategoryAUpper).clone(),
    //     "lower to upper".to_string());
    // setCategoryAlphabet.add_cell(cell).unwrap();
    //
    // // add another cell between the two objects
    // // but its invalid semantically from lower to upper
    // let cell_id_lower_upper_reverse = generate_identifier();
    // let cell_lower_upper_reverse = GenericNCell::new(
    //     cell_id_lower_upper_reverse.clone(),
    //     NCategory::category_id(&discreteCategoryAUpper).clone(),
    //     NCategory::category_id(&discreteCategoryALower).clone(),
    //     "lower to upper reverse".to_string());
    // setCategoryAlphabet.add_cell(cell_lower_upper_reverse).unwrap();
    //
    //
    // let cell_lower_upper = setCategoryAlphabet.get_cell(&cell_id).unwrap();
    // assert_eq!(cell_lower_upper.source_object_id(),  NCategory::category_id(&discreteCategoryALower));
    // assert_eq!(cell_lower_upper.target_object_id(),  NCategory::category_id(&discreteCategoryAUpper));
    //
    // let cell_lower_upper_reverse = setCategoryAlphabet.get_cell(&cell_id_lower_upper_reverse).unwrap();
    // assert_eq!(cell_lower_upper_reverse.source_object_id(),  NCategory::category_id(&discreteCategoryAUpper));
    // assert_eq!(cell_lower_upper_reverse.target_object_id(),  NCategory::category_id(&discreteCategoryALower));
    //
    // // now get cell tree of the cell
    // let actual_cell_tree = setCategoryAlphabet.get_cell_tree(cell_lower_upper).unwrap();
    //
    // let functor_1 = "functor_1".to_string();
    // let functor_2 = "functor_2".to_string();
    // let expected_cell_tree = CellTree::new_with_children(
    //     cell_lower_upper.id(),
    //     cell_lower_upper.source_object_id(),
    //     cell_lower_upper.target_object_id(),
    //     vec![
    //         CellTree::new(
    //             &functor_1,
    //             &object_a,
    //             &object_A
    //         ),
    //         CellTree::new(
    //             &functor_1,
    //             &object_b,
    //             &object_B
    //         ),
    //         CellTree::new(
    //             &functor_1,
    //             &object_c,
    //             &object_C
    //         )
    //     ]
    // );
    //
    // assert_eq!(actual_cell_tree, expected_cell_tree);
    //
    // // now get cell tree of the reverse cell
    // let actual_cell_tree_reverse = setCategoryAlphabet.get_cell_tree(cell_lower_upper_reverse).unwrap();
    //
    // let expected_cell_tree_reverse = CellTree::new_with_children(
    //     cell_lower_upper_reverse.id(),
    //     cell_lower_upper_reverse.source_object_id(),
    //     cell_lower_upper_reverse.target_object_id(),
    //     vec![
    //         CellTree::new(
    //             &functor_2,
    //             &object_a,
    //             &object_C
    //         ),
    //         CellTree::new(
    //             &functor_2,
    //             &object_b,
    //             &object_B
    //         ),
    //         CellTree::new(
    //             &functor_2,
    //             &object_c,
    //             &object_A
    //         )
    //     ]
    // );
}

#[test]
pub fn test_nested_category() {
    // start with discrete category A with a, b, c as objects
    let mut categoryAlphaLower: Rc<DiscreteCategory> = Rc::new(vec!["a", "b", "c"].into());

    let categoryAlphaUpper: Rc<DiscreteCategory> = Rc::new(vec!["A", "B", "C"].into());

    let categoryAlphaNumber: Rc<DiscreteCategory> = Rc::new(vec![1, 2, 3].into());

    Rc::get_mut(&mut categoryAlphaLower)
        .unwrap()
        .add_object(categoryAlphaUpper.clone())
        .unwrap();

    let lower_to_number_mappings: HashMap<
        Rc<Morphism<DiscreteCategory>>,
        Rc<Morphism<DiscreteCategory>>,
    > = HashMap::from([
        // a to 1
        (
            categoryAlphaLower
                .get_identity_morphism(&DiscreteCategory::new_with_id(ObjectId::Str(
                    "a".to_string(),
                )))
                .unwrap()
                .clone(),
            categoryAlphaNumber
                .get_identity_morphism(&DiscreteCategory::new_with_id(ObjectId::Int(1)))
                .unwrap()
                .clone(),
        ),
        // b to 2
        (
            categoryAlphaLower
                .get_identity_morphism(&DiscreteCategory::new_with_id(ObjectId::Str(
                    "b".to_string(),
                )))
                .unwrap()
                .clone(),
            categoryAlphaNumber
                .get_identity_morphism(&DiscreteCategory::new_with_id(ObjectId::Int(2)))
                .unwrap()
                .clone(),
        ),
        // c to 3
        (
            categoryAlphaLower
                .get_identity_morphism(&DiscreteCategory::new_with_id(ObjectId::Str(
                    "c".to_string(),
                )))
                .unwrap()
                .clone(),
            categoryAlphaNumber
                .get_identity_morphism(&DiscreteCategory::new_with_id(ObjectId::Int(3)))
                .unwrap()
                .clone(),
        ),
    ]);

    // create a functor from lower to number
    let functor_lower_to_number = Rc::new(Functor::new(
        "functor_lower_to_number".to_string(),
        categoryAlphaLower.clone(),
        categoryAlphaNumber.clone(),
        lower_to_number_mappings,
    ));

    let number_to_upper_mappings: HashMap<
        Rc<Morphism<DiscreteCategory>>,
        Rc<Morphism<DiscreteCategory>>,
    > = HashMap::from([
        // 1 to a
        (
            categoryAlphaNumber
                .get_identity_morphism(&DiscreteCategory::new_with_id(ObjectId::Int(1)))
                .unwrap()
                .clone(),
            categoryAlphaLower
                .get_identity_morphism(&DiscreteCategory::new_with_id(ObjectId::Str(
                    "a".to_string(),
                )))
                .unwrap()
                .clone(),
        ),
        // 2 to b
        (
            categoryAlphaNumber
                .get_identity_morphism(&DiscreteCategory::new_with_id(ObjectId::Int(2)))
                .unwrap()
                .clone(),
            categoryAlphaLower
                .get_identity_morphism(&DiscreteCategory::new_with_id(ObjectId::Str(
                    "b".to_string(),
                )))
                .unwrap()
                .clone(),
        ),
        // 3 to c
        (
            categoryAlphaNumber
                .get_identity_morphism(&DiscreteCategory::new_with_id(ObjectId::Int(3)))
                .unwrap()
                .clone(),
            categoryAlphaLower
                .get_identity_morphism(&DiscreteCategory::new_with_id(ObjectId::Str(
                    "c".to_string(),
                )))
                .unwrap()
                .clone(),
        ),
    ]);

    let functor_number_to_upper = Functor::new(
        "functor_number_to_upper".to_string(),
        categoryAlphaNumber.clone(),
        categoryAlphaUpper.clone(),
        number_to_upper_mappings,
    );

    // create a functor1 from lower to upper
    let lower_to_upper_mappings: HashMap<
        Rc<Morphism<DiscreteCategory>>,
        Rc<Morphism<DiscreteCategory>>,
    > = HashMap::from([
        // a to A
        (
            categoryAlphaLower
                .get_identity_morphism(&DiscreteCategory::new_with_id(ObjectId::Str(
                    "a".to_string(),
                )))
                .unwrap()
                .clone(),
            categoryAlphaUpper
                .get_identity_morphism(&DiscreteCategory::new_with_id(ObjectId::Str(
                    "A".to_string(),
                )))
                .unwrap()
                .clone(),
        ),
        // b to B
        (
            categoryAlphaLower
                .get_identity_morphism(&DiscreteCategory::new_with_id(ObjectId::Str(
                    "b".to_string(),
                )))
                .unwrap()
                .clone(),
            categoryAlphaUpper
                .get_identity_morphism(&DiscreteCategory::new_with_id(ObjectId::Str(
                    "B".to_string(),
                )))
                .unwrap()
                .clone(),
        ),
        // c to C
        (
            categoryAlphaLower
                .get_identity_morphism(&DiscreteCategory::new_with_id(ObjectId::Str(
                    "c".to_string(),
                )))
                .unwrap()
                .clone(),
            categoryAlphaUpper
                .get_identity_morphism(&DiscreteCategory::new_with_id(ObjectId::Str(
                    "C".to_string(),
                )))
                .unwrap()
                .clone(),
        ),
    ]);

    let functor_lower_to_upper = Rc::new(Functor::new(
        "functor_1".to_string(),
        categoryAlphaLower.clone(),
        categoryAlphaUpper.clone(),
        lower_to_upper_mappings,
    ));

    // functor from lower to upper in reverse
    let lower_to_upper_reverse_mappings: HashMap<
        Rc<Morphism<DiscreteCategory>>,
        Rc<Morphism<DiscreteCategory>>,
    > = HashMap::from([
        // a to C
        (
            categoryAlphaLower
                .get_identity_morphism(&DiscreteCategory::new_with_id(ObjectId::Str(
                    "a".to_string(),
                )))
                .unwrap()
                .clone(),
            categoryAlphaUpper
                .get_identity_morphism(&DiscreteCategory::new_with_id(ObjectId::Str(
                    "C".to_string(),
                )))
                .unwrap()
                .clone(),
        ),
        // b to B
        (
            categoryAlphaLower
                .get_identity_morphism(&DiscreteCategory::new_with_id(ObjectId::Str(
                    "b".to_string(),
                )))
                .unwrap()
                .clone(),
            categoryAlphaUpper
                .get_identity_morphism(&DiscreteCategory::new_with_id(ObjectId::Str(
                    "B".to_string(),
                )))
                .unwrap()
                .clone(),
        ),
        // c to A
        (
            categoryAlphaLower
                .get_identity_morphism(&DiscreteCategory::new_with_id(ObjectId::Str(
                    "c".to_string(),
                )))
                .unwrap()
                .clone(),
            categoryAlphaUpper
                .get_identity_morphism(&DiscreteCategory::new_with_id(ObjectId::Str(
                    "A".to_string(),
                )))
                .unwrap()
                .clone(),
        ),
    ]);

    let mut setCategoryAlphabet: BaseCategory<DiscreteCategory> = BaseCategory::new();
    // Add the discrete category A as an object in Set category alphabet
    setCategoryAlphabet
        .add_object(categoryAlphaLower.clone())
        .unwrap();
    // Add the discrete category A as an object in Set category alphabet
    setCategoryAlphabet
        .add_object(categoryAlphaUpper.clone())
        .unwrap();
    let setCategoryAlphabet = Rc::new(setCategoryAlphabet);

    // category of numbers
    let mut setCategoryNumber: BaseCategory<DiscreteCategory> = BaseCategory::new();
    // Add the discrete category A as an object in Set category number
    setCategoryNumber
        .add_object(categoryAlphaNumber.clone())
        .unwrap();

    let setCategoryAlphabetNumber = Rc::new(setCategoryNumber);

    // lets create a functor category
    // where objects are functors and morphisms are natural transformations
    // let mut functorCategory = BaseCategory::new();
    //
    // functorCategory
    //     .add_object(functor_lower_to_upper.clone())
    //     .unwrap();
    // functorCategory
    //     .add_object(functor_lower_to_upper_reverse.clone())
    //     .unwrap();
    //
    // // now creating a morphism from lower to upper functor which is a natural transformation
    // let natural_transformation_morphism = Morphism::new(
    //     "natural_transformation_lower_to_upper".to_string(),
    //     functor_lower_to_upper.clone(),
    //     functor_lower_to_upper_reverse.clone(),
    //     Rc::new(Functor::new(
    //         "natural_transformation_lower_to_upper".to_string(),
    //         functor_lower_to_upper.clone(),
    //         functor_lower_to_upper_reverse.clone(),
    //         HashMap::new(),
    //     )),
    // );
    // let natural_transformation_morphism = Rc::new(natural_transformation_morphism);
    //
    // functorCategory
    //     .add_morphism(natural_transformation_morphism)
    //     .unwrap();
}
