use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::morphism::Morphism;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Category<Id: Identifier<Id = Id>, Object: CategoryTrait + Hash + Eq> {
    id: Id,
    objects: HashMap<Rc<Object>, HashSet<Rc<Morphism<Id, Object>>>>,
    morphism: HashMap<Id, Rc<Morphism<Id, Object>>>,
}

impl<'a, Id: Identifier<Id = Id>, Object: CategoryTrait + Hash + Eq> Default for Category<Id, Object> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Id: Identifier<Id = Id>, Object: CategoryTrait + Hash + Eq> Category<Id, Object> {
    pub fn new() -> Self {
        Self::new_with_id(Id::generate())
    }

    pub fn new_with_id(id: Id) -> Self {
        Category {
            id,
            objects: HashMap::new(),
            morphism: HashMap::new(),
        }
    }
}

impl<Id: Identifier<Id = Id>, Object> CategoryTrait for Category<Id, Object>
where
    Id: Identifier<Id = Id>,
    Object: CategoryTrait + Hash + Eq,
{
    type Object = Object;
    type Morphism = Morphism<Id, Object>;

    fn new() -> Self {
        Category::new()
    }

    fn add_object(&mut self, object: Rc<Self::Object>) -> Result<(), Errors> {
        if self.objects.contains_key(&object) {
            return Err(Errors::ObjectAlreadyExists);
        }
        let identity_cell = Morphism::new_identity_morphism(object.clone());
        self.objects
            .entry(object)
            .or_default()
            .insert(identity_cell.clone());
        self.add_morphism(identity_cell)?;
        Ok(())
    }

    fn add_morphism(
        &mut self,
        morphism: Rc<Self::Morphism>,
    ) -> Result<&Rc<Self::Morphism>, Errors> {
        if self.morphism.contains_key(morphism.id()) {
            return Err(Errors::MorphismAlreadyExists);
        }
        // validate target object is part of the category
        if !self.objects.contains_key(morphism.target_object()) {
            return Err(Errors::ObjectNotFound);
        }

        // if its not identity morphism add it to the objects as part of the hom-set
        if !morphism.is_identity() {
            self.objects
                .get_mut(morphism.source_object())
                .ok_or(Errors::ObjectNotFound)?
                .insert(morphism.clone());
        }

        let cell = self
            .morphism
            .entry(morphism.id().clone())
            .or_insert(morphism);
        Ok(cell)
    }

    fn get_object(&self, object: &Self::Object) -> Result<&Rc<Self::Object>, Errors> {
        self.objects
            .get_key_value(object)
            .map(|(k, _)| k)
            .ok_or(Errors::ObjectNotFound)
    }

    fn get_all_objects(&self) -> Result<HashSet<&Rc<Self::Object>>, Errors> {
        Ok(self.objects.keys().collect())
    }

    fn get_all_morphisms(&self) -> Result<HashSet<&Rc<Self::Morphism>>, Errors> {
        // Todo needs optimization
        // Ok(self.cells.values().collect())

        let result: HashSet<&Rc<Self::Morphism>> = HashSet::new();
        // for (_id, cell) in &self.cells {
        //     result.insert(cell);
        // }
        Ok(result)
    }

    fn get_hom_set_x(
        &self,
        source_object: &Self::Object,
    ) -> Result<HashSet<&Rc<Self::Morphism>>, Errors> {
        let result = self
            .objects
            .get(source_object)
            .ok_or(Errors::ObjectNotFound)?
            .iter()
            .collect::<HashSet<_>>();
        Ok(result)
    }

    fn get_object_morphisms(
        &self,
        object: &Self::Object,
    ) -> Result<Vec<&Rc<Self::Morphism>>, Errors> {
        // if let Some(cells) = self.object_mapping.get(object.category_id()) {
        //     let mut result: Vec<&Rc<Self::Morphism>> = Vec::new();
        //     for cell_set in cells.values() {
        //         for cell_id in cell_set {
        //             if let Some(cell) = self.morphism.get(cell_id) {
        //                 if cell.source_object().category_id() == object.category_id() {
        //                     result.push(cell);
        //                 }
        //             }
        //         }
        //     }
        //     Ok(result)
        // } else {
        //     Err(Errors::ObjectNotFound)
        // }
        let result = self.objects.get(object).ok_or(Errors::ObjectNotFound)?;
        Ok(result.iter().collect())
    }
}

impl<Id: Identifier<Id = Id>, Object: CategoryTrait + Hash + Eq> Hash for Category<Id, Object> {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        todo!()
    }
}
