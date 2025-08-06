use crate::core::generic_morphism::GenericMorphism;
use crate::core::identifier::Identifier;
use crate::core::traits::morphism_trait::MorphismTrait;
use crate::core::ncategory::{NCategory, NCategoryError};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
pub struct GenericNCategory<
    'a,
    Id: Identifier<Id = Id> + 'a,
    Category: NCategory<'a, Identifier = Id> + Debug + Eq + Hash + Clone,
> {
    id: Id,
    objects: HashMap<Id, &'a Category>,
    object_mapping: HashMap<Id, HashMap<Id, HashSet<Id>>>,
    cells: HashMap<Id, GenericMorphism<'a, Self>>,
}

impl<
    'a,
    Id: Identifier<Id = Id>,
    Category: NCategory<'a, Identifier = Id> + Debug + Eq + Hash + Clone,
> GenericNCategory<'a, Id, Category>
{
    pub fn new() -> Self {
        GenericNCategory {
            id: Id::generate(),
            objects: HashMap::new(),
            object_mapping: HashMap::new(),
            cells: HashMap::new(),
        }
    }
}

impl<
    'a,
    Id: Identifier<Id = Id> + 'a,
    Category: NCategory<'a, Identifier = Id> + 'a + Debug + Eq + Hash + Clone,
> NCategory<'a> for GenericNCategory<'a, Id, Category>
{
    type Identifier = Id;
    type Object = Category;
    type Morphism = GenericMorphism<'a, Self>;

    fn category_id(&self) -> &Self::Identifier {
        todo!()
    }

    fn add_object(&mut self, object: &'a Self::Object) -> Result<(), NCategoryError> {
        self.objects.insert(object.category_id().clone(), object);
        let identity_cell = GenericMorphism::new(
            object.category_id().clone(),
            object,
            object,
            "identity".to_string(),
        );
        self.add_morphism(identity_cell)?;
        Ok(())
    }

    fn add_morphism(&mut self, cell: Self::Morphism) -> Result<Self::Identifier, NCategoryError> {
        if self.cells.contains_key(cell.id()) {
            return Err(NCategoryError::MorphismAlreadyExists);
        }
        let cell = self.cells.entry(cell.id().clone()).or_insert(cell);
        self.object_mapping
            .entry(cell.source_object().category_id().clone())
            .or_default()
            .entry(cell.target_object().category_id().clone())
            .or_default()
            .insert(cell.cell_id().clone());
        let cell_id = cell.id().clone();
        Ok(cell_id)
    }

    fn get_identity_morphism(
        &self,
        object_id: &Self::Identifier,
    ) -> Result<&Self::Morphism, NCategoryError> {
        // it's basically the cell with the same id as the object
        self.get_moprhism(object_id)
    }

    fn get_all_object_ids(&self) -> Result<HashSet<&Self::Identifier>, NCategoryError> {
        todo!()
    }

    fn get_all_morphisms(&self) -> Result<HashSet<&Self::Morphism>, NCategoryError> {
        // Todo needs optimization
        // Ok(self.cells.values().collect())

        let result: HashSet<&Self::Morphism> = HashSet::new();
        // for (_id, cell) in &self.cells {
        //     result.insert(cell);
        // }
        Ok(result)
    }

    fn get_object_morphisms(
        &self,
        object_id: &Self::Identifier,
    ) -> Result<Vec<&Self::Morphism>, NCategoryError> {
        if let Some(cells) = self.object_mapping.get(object_id) {
            let mut result: Vec<&Self::Morphism> = Vec::new();
            for cell_set in cells.values() {
                for cell_id in cell_set {
                    if let Some(cell) = self.cells.get(cell_id) {
                        if cell.source_object().category_id() == object_id {
                            result.push(cell);
                        }
                    }
                }
            }
            Ok(result)
        } else {
            Err(NCategoryError::ObjectNotFound)
        }
    }

    fn get_moprhism(&self, cell_id: &Self::Identifier) -> Result<&Self::Morphism, NCategoryError> {
        if let Some(cell) = self.cells.get(cell_id) {
            Ok(cell)
        } else {
            Err(NCategoryError::MorphismNotFound)
        }
    }
}
