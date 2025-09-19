use crate::core::arrow::Morphism;
use crate::core::dynamic_category::DynamicCategory;
use crate::core::object_id::ObjectId;
use crate::core::traits::category_trait::{CategoryFromObjects, CategoryTrait};
use std::collections::HashMap;
use std::sync::Arc;

#[tokio::test]
pub async fn test_base_scenario() {
    // empty category
    let mut category = DynamicCategory::new();

    let objects = category.get_all_objects().await;
    assert!(objects.is_ok());
    let objects = objects.unwrap();
    assert_eq!(objects.len(), 0);

    let morphisms = category.get_all_morphisms().await;
    assert!(morphisms.is_ok());
    let morphisms = morphisms.unwrap();
    assert_eq!(morphisms.len(), 0);

    // let level = category.level();
    // assert_eq!(level, 0);

    // now add a set of a, b, c
    let object_a = DynamicCategory::from_objects(vec!["a", "b", "c"])
        .await
        .unwrap();
    let inner_objects = object_a.get_all_objects().await;
    assert!(inner_objects.is_ok());
    let inner_objects = inner_objects.unwrap();
    assert_eq!(inner_objects.len(), 3);
    assert!(
        inner_objects
            .iter()
            .any(|o| o.equal_to(&DynamicCategory::new_with_id(ObjectId::Str(
                "a".to_string()
            ))))
    );
    assert!(
        inner_objects
            .iter()
            .any(|o| o.equal_to(&<&str as Into<DynamicCategory>>::into("b")))
    );
    assert!(
        inner_objects
            .iter()
            .any(|o| o.equal_to(&<&str as Into<DynamicCategory>>::into("c")))
    );
    // assert_eq!(object_a.level(), 1);
    let object_a = Arc::new(object_a);

    category.add_object(object_a.clone()).await.unwrap();

    // now our category should have one object
    let objects = category.get_all_objects().await;
    assert!(objects.is_ok());
    let objects = objects.unwrap();
    assert_eq!(objects.len(), 1);
    let first_object = objects.iter().next().unwrap();
    // assert_eq!(first_object.level(), 1);
    assert!(first_object.equal_to(&*object_a));
    assert!(first_object.equal_to(&*object_a));

    // object with number 1,2,3
    let object_num = DynamicCategory::from_objects(vec![1, 2, 3]).await.unwrap();
    let inner_objects = object_num.get_all_objects().await;
    assert!(inner_objects.is_ok());
    let inner_objects = inner_objects.unwrap();
    assert_eq!(inner_objects.len(), 3);
    assert!(
        inner_objects
            .iter()
            .any(|o| o.equal_to(&DynamicCategory::new_with_id(ObjectId::Int(1))))
    );
    assert!(
        inner_objects
            .iter()
            .any(|o| o.equal_to(&DynamicCategory::new_with_id(ObjectId::Int(2))))
    );
    assert!(
        inner_objects
            .iter()
            .any(|o| o.equal_to(&DynamicCategory::new_with_id(ObjectId::Int(3))))
    );

    // now add the object_num to the category
    let object_num = Arc::new(object_num);
    category.add_object(object_num.clone()).await.unwrap();
    let objects = category.get_all_objects().await;
    assert!(objects.is_ok());
    let objects = objects.unwrap();
    assert_eq!(objects.len(), 2);
    // get object where id is the same as object_a and confirm they are the same
    let expected_obj_num = objects.iter().find(|o| o.equal_to(&*object_num));
    assert!(expected_obj_num.is_some());
    let expected_obj_num = expected_obj_num.unwrap();
    assert!(expected_obj_num.equal_to(&*object_num));
    // confirm object_a is also in the category
    let expected_obj_a = objects.iter().find(|o| o.equal_to(&*object_a));
    assert!(expected_obj_a.is_some());
    let expected_obj_a = expected_obj_a.unwrap();
    assert!(expected_obj_a.equal_to(&*object_a));
    assert!(objects.iter().any(|o| o.equal_to(&*object_a)));
    assert!(objects.iter().any(|o| o.equal_to(&*object_num)));

    // now add a functor from object_a to object_num
    // a -> 1, b -> 2, c -> 3
    // then use it to create a morphism in the category
    let functor = HashMap::from([
        (
            object_a
                .get_identity_morphism(&DynamicCategory::new_with_id(ObjectId::Str(
                    "a".to_string(),
                )))
                .await
                .unwrap()
                .clone(),
            object_num
                .get_identity_morphism(&DynamicCategory::new_with_id(ObjectId::Int(1)))
                .await
                .unwrap()
                .clone(),
        ),
        (
            object_a
                .get_identity_morphism(&DynamicCategory::new_with_id(ObjectId::Str(
                    "b".to_string(),
                )))
                .await
                .unwrap()
                .clone(),
            object_num
                .get_identity_morphism(&DynamicCategory::new_with_id(ObjectId::Int(2)))
                .await
                .unwrap()
                .clone(),
        ),
        (
            object_a
                .get_identity_morphism(&DynamicCategory::new_with_id(ObjectId::Str(
                    "c".to_string(),
                )))
                .await
                .unwrap()
                .clone(),
            object_num
                .get_identity_morphism(&DynamicCategory::new_with_id(ObjectId::Int(3)))
                .await
                .unwrap()
                .clone(),
        ),
    ]);
    // assert!(functor.is_ok());
    // let functor = functor.unwrap();

    // let functor = Arc::new(functor);

    let morphism_a_num = Arc::new(Morphism::new(
        "morphism_a_num".into(),
        object_a.clone(),
        object_num.clone(),
        functor,
    ));
    // assert!(morphism_a_num.is_ok());
    // let morphism_a_num = morphism_a_num.unwrap();

    category.add_morphism(morphism_a_num.clone()).await.unwrap();

    // create another functor from object_a to object_num
    // a -> 3, b -> 2, c -> 1
    let functor_2 = HashMap::from([
        (
            object_a
                .get_identity_morphism(&DynamicCategory::new_with_id(ObjectId::Str(
                    "a".to_string(),
                )))
                .await
                .unwrap()
                .clone(),
            object_num
                .get_identity_morphism(&DynamicCategory::new_with_id(ObjectId::Int(3)))
                .await
                .unwrap()
                .clone(),
        ),
        (
            object_a
                .get_identity_morphism(&DynamicCategory::new_with_id(ObjectId::Str(
                    "b".to_string(),
                )))
                .await
                .unwrap()
                .clone(),
            object_num
                .get_identity_morphism(&DynamicCategory::new_with_id(ObjectId::Int(2)))
                .await
                .unwrap()
                .clone(),
        ),
        (
            object_a
                .get_identity_morphism(&DynamicCategory::new_with_id(ObjectId::Str(
                    "c".to_string(),
                )))
                .await
                .unwrap()
                .clone(),
            object_num
                .get_identity_morphism(&DynamicCategory::new_with_id(ObjectId::Int(1)))
                .await
                .unwrap()
                .clone(),
        ),
    ]);

    let morphism_a_num_2 = Arc::new(Morphism::new(
        "morphism_a_num_2".into(),
        object_a.clone(),
        object_num.clone(),
        functor_2.clone(),
    ));
    // assert!(morphism_a_num_2.is_ok());
    // let morphism_a_num_2 = morphism_a_num_2.unwrap();
    category.add_morphism(morphism_a_num_2).await.unwrap();

    let category = Arc::new(category);

    // now create a category of the functor category
    let mut functor_category = DynamicCategory::new_with_id("FunctorCategory".into());
    functor_category.add_object(Arc::new(
        DynamicCategory::functor_to_category(morphism_a_num.get_functor().unwrap().clone())
            .expect("Expecting category"),
    ));
}
