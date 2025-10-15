/*
Endo functor maps objects and morphisms of a category to itself.
This is a specific case of a functor where the source and target categories are the same.

For our category framework since we always have a definite set of objects and morphisms

Endo functor will create a new category with the objects and morphisms of the original category.
and the new objects and morphisms mapped to the new values.

For instance if category A has objects {a, b} and morphisms {f: a -> b, g: b -> a},
an endo functor F would map these to a new category B with objects {F(a), F(b)} and morphisms {F(f): F(a) -> F(b), F(g): F(b) -> F(a)}.
This is useful for creating new categories that are derived from the original category, such as the category of sets or the category of groups.
 */
use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::object_id::ObjectId;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::{CategoryFromObjects, CategoryTrait};
use crate::core::traits::functor_trait::FunctorTrait;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

pub async fn apply_product<Category: CategoryTrait + Eq + Hash>(
    category: &mut Category,
    source_object: &Arc<Category::Object>,
    fixed_object: Arc<Category::Object>,
) -> Result<
    (
        Arc<Category::Object>, // the new object created by the product
        HashMap<Arc<Category::Morphism>, Arc<Category::Morphism>>, // mapping of morphisms from source category to product target category
    ),
    Errors>
where
    <<<Category as CategoryTrait>::Object as CategoryTrait>::Object as CategoryTrait>::Object:
        From<ObjectId>, // <SourCategory as CategoryTrait>::Object: CategoryTrait<Identifier = String>,
{
    // we take a product of the source category and the fixed category
    // and map the objects and morphisms of the source category to the target category
    let mut target_object = Category::Object::new().await?;

    let fixed_objects = fixed_object.get_all_objects().await?;

    let mut source_object_mapping = HashMap::new();

    // first map the objects from the source category to the target category
    for source_sub_identity_morphism in source_object.get_all_identity_morphisms().await? {
        for fixed_sub_object in &fixed_objects {
            // product object (_) * fixed_object
            let new_value = (*source_sub_identity_morphism.source_object().clone())
                .category_id()
                .to_owned()
                + (*fixed_sub_object).clone().category_id().clone();
            let new_category =
                <Category::Object as CategoryTrait>::Object::from_object(new_value).await?;
            let new_category_rc = Arc::new(new_category);

            let target_sub_identity_morphism =
                target_object.add_object(new_category_rc.clone()).await?;
            source_object_mapping
                .entry(source_sub_identity_morphism)
                .or_insert_with(Vec::new)
                .push(target_sub_identity_morphism);
        }
    }

    let source_object_mapped_product = Arc::new(target_object);
    category
        .add_object(source_object_mapped_product.clone())
        .await?;

    // should map other related object
    let mut morphism_mapping = HashMap::new();

    let object_morphisms = category
        .get_object_morphisms(&*source_object)
        .await?
        .into_iter()
        .map(|m| m.clone())
        .collect::<Vec<_>>();

    for morphism in object_morphisms {
        dbg!(&morphism);
        if morphism.is_identity() {
            let identity_mapped_object = category
                .get_identity_morphism(&*source_object_mapped_product)
                .await?;
            morphism_mapping.insert(morphism.clone(), identity_mapped_object.clone());
            continue;
        }
        let arrow_mapping = morphism.arrow_mappings();
        let mut new_mapping = HashMap::new();
        let mut target_object = Category::Object::new().await?;
        for (source_sub_morphism, target_sub_morphism) in arrow_mapping.into_iter().flatten() {
            let mut mapped_objects = Vec::new();
            for fixed_sub_object in &fixed_objects {
                // product object (_) * fixed_object
                let new_value = (*target_sub_morphism.source_object().clone())
                    .category_id()
                    .to_owned()
                    + (*fixed_sub_object).clone().category_id().clone();
                let new_category =
                    <Category::Object as CategoryTrait>::Object::from_object(new_value).await?;
                let new_category_rc = Arc::new(new_category);
                target_object.add_object(new_category_rc.clone()).await?;
                let mapped_morphism = target_object
                    .get_identity_morphism(&*new_category_rc)
                    .await?;
                mapped_objects.push(mapped_morphism.clone());
            }

            if let Some(source_mapped_morphism) = source_object_mapping.get(source_sub_morphism) {
                for (index, source_mapped_object) in source_mapped_morphism.iter().enumerate() {
                    let target_mapped_morphism = &mapped_objects[index];
                    new_mapping.insert(
                        source_mapped_object.clone().clone(),
                        target_mapped_morphism.clone(),
                    );
                }
            } else {
                return Err(Errors::InvalidFunctor("Invalid mapping".to_string()));
            }
        }
        let target_object = Arc::new(target_object);

        category.add_object(target_object.clone()).await?;

        let new_morphism = Arc::new(Category::Morphism::new(
            String::generate(),
            source_object_mapped_product.clone(),
            target_object.clone(),
            new_mapping,
        ));
        category.add_morphism(new_morphism.clone()).await?;

        morphism_mapping.insert(morphism.clone(), new_morphism.clone());
    }
    Ok((source_object_mapped_product, morphism_mapping))
}


#[cfg(test)]
mod test {
    use std::sync::Arc;
    use surrealdb::sql::Base;
    use crate::core::arrow::Morphism;
    use crate::core::base_category::BaseCategory;
    use crate::core::discrete_category::DiscreteCategory;
    use crate::core::functor::Functor;
    use crate::core::traits::arrow_trait::ArrowTrait;
    use crate::core::traits::category_trait::{CategoryFromObjects, CategoryTrait};

    #[tokio::test]
    async fn test_apply_product() {
        // start with a category with 3 objects
        // one with objects a, b
        // another one with object 1, 2.
        // apply product to get a new category with objects (a,1), (a,2), (b,1), (b,2)
        // and any morphisms from the original categories mapped to the new category

        let category_abc: Arc<BaseCategory<DiscreteCategory>>= Arc::new(
            BaseCategory::from_objects(vec!["a", "b", "c"]).await.unwrap());

        let category_ABC: Arc<BaseCategory<DiscreteCategory>>= Arc::new(
            BaseCategory::from_objects(vec!["A", "B", "C"]).await.unwrap());

        let category_123: Arc<BaseCategory<DiscreteCategory>>= Arc::new(
            BaseCategory::from_objects(vec!["1", "2", "3"]).await.unwrap());

        // functor from category_a to category_A
        let mapping_a_to_A = vec![
            (category_abc.get_identity_morphism(&"a".into()).await.unwrap().clone(),
             category_ABC.get_identity_morphism(&"A".into()).await.unwrap().clone()),

            (category_abc.get_identity_morphism(&"b".into()).await.unwrap().clone(),
             category_ABC.get_identity_morphism(&"B".into()).await.unwrap().clone()),

            (category_abc.get_identity_morphism(&"c".into()).await.unwrap().clone(),
             category_ABC.get_identity_morphism(&"C".into()).await.unwrap().clone()),

        ].into_iter().collect();
        let functor_a_to_A = Arc::new(Functor::new(
            "A -> B".to_string(),
            category_abc.clone(),
            category_ABC.clone(),
            mapping_a_to_A,
        ));

        // create a new category that has 4 objects
        let mut category = BaseCategory::<BaseCategory<DiscreteCategory>>::new();
        category.add_object(category_abc.clone()).await.unwrap();
        category.add_object(category_ABC.clone()).await.unwrap();
        category.add_object(category_123.clone()).await.unwrap();

        let category_mno: Arc<BaseCategory<DiscreteCategory>>= Arc::new(
            BaseCategory::from_objects(vec!["m", "n", "o"]).await.unwrap());
        category.add_object(category_mno.clone()).await.unwrap();

        // add morphism from category_abc to category_ABC
        let morphism_a_to_A = Arc::new(Morphism::new(
            "f".to_string(),
            category_abc.clone(),
            category_ABC.clone(),
            Some(functor_a_to_A),
        ));
        category.add_morphism(morphism_a_to_A.clone()).await.unwrap();

        // now applying product to category_abc with fixed category_123
        // should give us a new category with objects (a,1), (a,2), (b,1), (b,2)
        // and two functors from category_abc to the new category
        // and from category_ABC to the new category.
        let result = super::apply_product(
            &mut category,
            &category_abc,
            category_123.clone(),
        ).await;
        assert!(result.is_ok());
        let (product_object, morphism_mapping) = result.clone().unwrap();
        print!("Result: {:#?}", product_object);
        dbg!(&result);

        let all_objects = product_object.get_all_objects().await.unwrap();
        assert_eq!(all_objects.len(), 9);

        let expected_objects = vec![
            "a1", "a2", "a3",
            "b1", "b2", "b3",
            "c1", "c2", "c3",
        ];
        for obj in &expected_objects {
            assert!(all_objects.iter().any(|o| o.category_id() == obj));
        }

        // there is a morphism from category_abc to category_ABC
        // so there should be a morphism from product_object_abc to product_object_ABC
        let category_abc_morphism = category.get_hom_set(&category_abc, &category_ABC).await.unwrap();

        // sanity check should only be one morphism
        assert_eq!(category_abc_morphism.len(), 1);

        let morphism_abc_to_ABC = category_abc_morphism.iter().next().unwrap();

        // now check where that morphism is mapped in the product.
        assert!(morphism_mapping.contains_key(*morphism_abc_to_ABC));

        let mapped_morphism = morphism_mapping.get(*morphism_abc_to_ABC).unwrap();
        let mapped_target= mapped_morphism.target_object();

        let mapped_target_objects = mapped_target.get_all_objects().await.unwrap();
        assert_eq!(mapped_target_objects.len(), 9);

        let expected_mapped_target_objects = vec![
            "A1", "A2", "A3",
            "B1", "B2", "B3",
            "C1", "C2", "C3",
        ];
        for obj in &expected_mapped_target_objects {
            assert!(mapped_target_objects.iter().any(|o| o.category_id() == obj));
        }


        // now confirm the arrow mapping from product_object abc to product_object ABC
        let arrow_mappings = mapped_morphism.arrow_mappings();
        assert!(arrow_mappings.is_some());
        let arrow_mappings = arrow_mappings.unwrap();
        // should have 9 mappings
        assert_eq!(arrow_mappings.len(), 9);
        // for each a1 to A1 etc

        for (index, value) in expected_objects.iter().enumerate() {
            let source_object_identity_morphism
                = product_object.get_identity_morphism(&(*value).into()).await.unwrap();
            let target_object_identity_morphism
                = mapped_target.get_identity_morphism(&expected_mapped_target_objects[index].into()).await.unwrap();
            assert!(arrow_mappings.contains_key(source_object_identity_morphism));

            let mapped = arrow_mappings.get(source_object_identity_morphism).unwrap();
            assert_eq!(mapped, target_object_identity_morphism);
        }

    }

}