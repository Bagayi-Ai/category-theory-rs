use crate::core::category::*;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

struct GenericCategory<
    CategoryObjectId: CategoryObjectIdTrait,
    CategoryObjectValue,
    MorphismId: MorphismIdTrait,
> {
    objects: HashMap<CategoryObjectId, CategoryObject<CategoryObjectId, CategoryObjectValue>>,
    object_to_mapping: HashMap<CategoryObjectId, HashMap<CategoryObjectId, Vec<MorphismId>>>,
    object_from_mapping: HashMap<CategoryObjectId, HashMap<CategoryObjectId, Vec<MorphismId>>>,
    morphisms: HashMap<MorphismId, Morphism<MorphismId, CategoryObjectId>>,
}

impl<
    CategoryObjectId: CategoryObjectIdTrait,
    CategoryObjectValue: Eq + Hash + Clone,
    MorphismId: MorphismIdTrait,
> GenericCategory<CategoryObjectId, CategoryObjectValue, MorphismId>
{
    pub fn new() -> Self {
        GenericCategory {
            objects: HashMap::new(),
            object_to_mapping: HashMap::new(),
            object_from_mapping: HashMap::new(),
            morphisms: HashMap::new(),
        }
    }
}

impl<
    CategoryObjectId: CategoryObjectIdTrait,
    CategoryObjectValue: Eq + Hash + Clone,
    MorphismId: MorphismIdTrait<IdType = CategoryObjectId>,
> Category for GenericCategory<CategoryObjectId, CategoryObjectValue, MorphismId>
{
    type CategoryObjectId = CategoryObjectId;
    type CategoryObjectValue = CategoryObjectValue;
    type MorphismId = MorphismId;

    fn get_object(
        &self,
        id: &Self::CategoryObjectId,
    ) -> Option<&CategoryObject<Self::CategoryObjectId, Self::CategoryObjectValue>> {
        self.objects.get(id)
    }

    fn add_object_with_id(
        &mut self,
        id: Self::CategoryObjectId,
        object: Self::CategoryObjectValue,
    ) {
        let category_object = CategoryObject::new(id.clone(), object);
        self.objects.insert(id.clone(), category_object);

        // Add identity morphism for the new object
        let identity_morphism =
            Morphism::new(MorphismId::new(id.clone()), id.clone(), id.clone(), true);
        self.morphisms
            .insert(identity_morphism.id.clone(), identity_morphism.clone());

        // Initialize the object mapping for the new object
        self.object_to_mapping
            .entry(id.clone())
            .or_default()
            .insert(id.clone(), vec![identity_morphism.id.clone()]);
        self.object_from_mapping
            .entry(id.clone())
            .or_default()
            .insert(id, vec![identity_morphism.id]);
    }

    fn add_object(&mut self, object: Self::CategoryObjectValue) {
        let id = Self::CategoryObjectId::new(); // Generate a unique ID for the object
        self.add_object_with_id(id.clone(), object);
    }

    fn get_object_morphisms(
        &self,
        id: &Self::CategoryObjectId,
    ) -> HashSet<&Morphism<Self::MorphismId, Self::CategoryObjectId>> {
        let mut morphisms = HashSet::new();
        if let Some(mappings) = self.object_to_mapping.get(id) {
            for (_, morphism_ids) in mappings {
                for morphism_id in morphism_ids {
                    if let Some(morphism) = self.morphisms.get(morphism_id) {
                        morphisms.insert(morphism);
                    }
                }
            }
        }
        morphisms
    }

    fn get_object_targets(&self, id: &Self::CategoryObjectId) -> HashSet<&Self::CategoryObjectId> {
        let mut targets = HashSet::new();
        if let Some(mappings) = self.object_to_mapping.get(id) {
            for (target_id, morphism_ids) in mappings {
                if !morphism_ids.is_empty() {
                    targets.insert(target_id);
                }
            }
        }
        targets
    }

    fn get_object_sources(&self, id: &Self::CategoryObjectId) -> HashSet<&Self::CategoryObjectId> {
        let mut sources = HashSet::new();
        if let Some(mappings) = self.object_from_mapping.get(id) {
            for (source_id, morphism_ids) in mappings {
                if !morphism_ids.is_empty() {
                    sources.insert(source_id);
                }
            }
        }
        sources
    }

    fn get_morphism(
        &self,
        id: &Self::MorphismId,
    ) -> Option<&Morphism<Self::MorphismId, Self::CategoryObjectId>> {
        todo!()
    }

    fn add_morphism(
        &mut self,
        id: Self::MorphismId,
        source_id: Self::CategoryObjectId,
        target_id: Self::CategoryObjectId,
    ) -> Result<(), CategoryError<Self::CategoryObjectId, Self::MorphismId>> {
        if !self.objects.contains_key(&source_id) {
            return Err(CategoryError::ObjectNotFound(source_id));
        }
        if !self.objects.contains_key(&target_id) {
            return Err(CategoryError::ObjectNotFound(target_id));
        }

        let morphism = Morphism::new(id.clone(), source_id.clone(), target_id.clone(), false);
        self.morphisms.insert(id.clone(), morphism.clone());

        // Add the morphism to the object mapping
        self.object_to_mapping
            .entry(source_id.clone())
            .or_default()
            .entry(target_id.clone())
            .or_default()
            .push(morphism.id.clone());

        self.object_from_mapping
            .entry(target_id)
            .or_default()
            .entry(source_id)
            .or_default()
            .push(morphism.id);

        Ok(())
    }

    fn morphisms_commute(
        &self,
        left: &Self::MorphismId,
        right: &Self::MorphismId,
    ) -> bool
    {
        if let (Some(left_morphism), Some(right_morphism)) =
            (self.morphisms.get(left), self.morphisms.get(right))
        {
            // Check if the source of the left morphism is the same as the target of the right morphism
            if left_morphism.target_id == right_morphism.source_id
                && left_morphism.source_id == right_morphism.target_id{

                if left_morphism.is_identity || right_morphism.is_identity {
                    return true; // Identity morphisms commute with everything
                }

                // if object is a category object then check if the objects and morphisms commute
                // for the object.

                if let Some(left_object) = self.objects.get(&left_morphism.source_id) {
                    if let Some(right_object) = self.objects.get(&right_morphism.target_id) {

                    }
                }
            }
        }
        false

    }
}


mod tests {
    use super::*;
    use crate::core::uuid_id::{UuidCategoryObjectId, UuidMorphismId};


    fn create_dummy_category_for_tests() -> (GenericCategory<UuidCategoryObjectId, String,UuidMorphismId>, UuidCategoryObjectId, UuidCategoryObjectId) {
        let mut category = GenericCategory::<UuidCategoryObjectId, String, UuidMorphismId>::new();

        // Create two objects
        let obj1_id = UuidCategoryObjectId::new();
        let obj2_id = UuidCategoryObjectId::new();

        category.add_object_with_id(obj1_id.clone(), "Object 1".to_string());
        category.add_object_with_id(obj2_id.clone(), "Object 2".to_string());

        (category, obj1_id, obj2_id)

    }


    #[test]
    fn test_base_scenarios() {

        let (mut category, obj1_id, obj2_id) =
            create_dummy_category_for_tests();

        let obj1 = category.get_object(&obj1_id).unwrap();
        assert_eq!(obj1_id.clone(), obj1.id.clone());
        let mophism = category.get_object_morphisms(&obj1_id);
        assert_eq!(mophism.len(), 1);
        let identity_morphism = mophism.iter().find(|m| m.is_identity).unwrap();
        assert_eq!(identity_morphism.source_id, obj1_id.clone());
        assert_eq!(identity_morphism.target_id, obj1_id.clone());

        // add a morphism from obj1 to obj2
        let morphism_id = UuidMorphismId::new();
        category
            .add_morphism(morphism_id.clone(), obj1_id.clone(), obj2_id.clone())
            .unwrap();

        let morphisms = category.get_object_morphisms(&obj1_id);

        assert_eq!(morphisms.len(), 2); // identity + new morphism
        let morphism = morphisms.iter().find(
            |m| m.id == morphism_id).unwrap();
        assert_eq!(morphism.source_id, obj1_id);
        assert_eq!(morphism.target_id, obj2_id);

        let targets = category.get_object_targets(&obj1_id);
        // should contain only two it self and obj2
        assert_eq!(targets.len(), 2);
        assert!(targets.contains(&obj1_id));
        assert!(targets.contains(&obj2_id));

        let sources = category.get_object_sources(&obj2_id);
        // should contain only two it self and obj1
        assert_eq!(sources.len(), 2);
        assert!(sources.contains(&obj1_id));
        assert!(sources.contains(&obj2_id));

        // check object1 does not have objects mapping to it only the identity morphism
        let object1_sources = category.get_object_sources(&obj1_id);
        assert_eq!(object1_sources.len(), 1, "Object1 should not have any sources");
        assert!(object1_sources.contains(&obj1_id), "Object1 should only have itself as source");
    }

    #[test]
    fn test_composition() {
        let (mut category, obj1_id, obj2_id) =
            create_dummy_category_for_tests();
        let obj3_id = UuidCategoryObjectId::new();

        category.add_object_with_id(obj3_id.clone(), "value3".to_string());

        // add morphisms
        let morphism1_id = UuidMorphismId::new();
        category
            .add_morphism(morphism1_id.clone(), obj1_id.clone(), obj2_id.clone())
            .unwrap();

        let morphism2_id = UuidMorphismId::new();
        category
            .add_morphism(morphism2_id.clone(), obj2_id.clone(), obj3_id.clone())
            .unwrap();

        assert!(!morphism1_id.is_composite(), "Morphism1 should not be composite");
        assert!(!morphism2_id.is_composite(), "Morphism2 should not be composite");
        // check composition
        let composition_result = morphism1_id.compose(&morphism2_id);
        assert!(composition_result.is_composite(), "Composition should succeed");


    }

    #[test]
    fn test_commuting_triangle_diagram(){
        let (mut category, obj1_id, obj2_id) =
            create_dummy_category_for_tests();

        let obj3_id = UuidCategoryObjectId::new();
        category.add_object_with_id(obj3_id.clone(), "value3".to_string());

        // add morphisms object1 -> object2 and object2 -> object3
        // and object1 -> object3 and it should be a commuting triangle diagram
        let morphism1_id = UuidMorphismId::new();
        category
            .add_morphism(morphism1_id.clone(), obj1_id.clone(), obj2_id.clone())
            .unwrap();

        let morphism2_id = UuidMorphismId::new();
        category
            .add_morphism(morphism2_id.clone(), obj2_id.clone(), obj3_id.clone())
            .unwrap();

        let morphism3_id = UuidMorphismId::new();
        category
            .add_morphism(morphism3_id.clone(), obj1_id.clone(), obj3_id.clone())
            .unwrap();

        // check that morphisms commute
        assert!(category.morphisms_commute(&morphism1_id.compose(&morphism2_id), &morphism3_id));

    }
}
