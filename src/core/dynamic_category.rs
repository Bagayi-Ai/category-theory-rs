use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::morphism::Morphism;
use crate::core::traits::category_trait::CategoryTrait;
use crate::core::traits::morphism_trait::MorphismTrait;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug)]
pub struct DynamicCategory {
    id: String,
    objects:
        HashMap<String, Rc<dyn CategoryTrait<Object = Self, Morphism = Morphism<String, Self>>>>,
    object_mapping: HashMap<String, HashSet<Rc<Morphism<String, Self>>>>,
    morphisms: HashMap<String, Rc<Morphism<String, Self>>>,
}

impl PartialEq for DynamicCategory {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for DynamicCategory {}

impl Hash for DynamicCategory {
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!()
    }
}

impl CategoryTrait for DynamicCategory {
    type Object = DynamicCategory;
    type Morphism = Morphism<String, Self::Object>;

    fn new() -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn add_object(&mut self, object: Rc<Self::Object>) -> Result<(), Errors> {
        if self.objects.contains_key(&object.id) {
            return Err(Errors::ObjectAlreadyExists);
        }
        self.objects.insert(object.id.clone(), object.clone());
        let identity_cell = Morphism::new_identity_morphism(object.clone());
        self.object_mapping
            .entry(object.id.clone())
            .or_insert(HashSet::new())
            .insert(identity_cell.clone());
        self.add_morphism(identity_cell)?;
        Ok(())
    }

    fn add_morphism(
        &mut self,
        morphism: Rc<Self::Morphism>,
    ) -> Result<&Rc<Self::Morphism>, Errors> {
        todo!()
    }

    fn get_object(&self, object: &Self::Object) -> Result<&Rc<Self::Object>, Errors> {
        todo!()
    }

    fn get_all_objects(&self) -> Result<HashSet<&Rc<Self::Object>>, Errors> {
        todo!()
    }

    fn get_all_morphisms(&self) -> Result<HashSet<&Rc<Self::Morphism>>, Errors> {
        todo!()
    }

    fn get_hom_set_x(
        &self,
        source_object: &Self::Object,
    ) -> Result<HashSet<&Rc<Self::Morphism>>, Errors> {
        todo!()
    }

    fn get_object_morphisms(
        &self,
        object_id: &Self::Object,
    ) -> Result<Vec<&Rc<Self::Morphism>>, Errors> {
        todo!()
    }
}
