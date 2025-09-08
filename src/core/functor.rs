use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::morphism::Morphism;
use crate::core::object_id::ObjectId;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::{CategorySubObjectAlias, CategoryTrait};
use crate::core::traits::functor_trait::FunctorTrait;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct Functor<SourceCategory: CategoryTrait, TargetCategory: CategoryTrait> {
    id: ObjectId,
    source_category: Rc<SourceCategory>,
    target_category: Rc<TargetCategory>,
    mappings: HashMap<String, String>,
    source_morphisms: HashMap<String, Weak<Morphism<CategorySubObjectAlias<SourceCategory>>>>,
    target_morphisms: HashMap<String, Weak<Morphism<CategorySubObjectAlias<TargetCategory>>>>,
}

impl<SourceCategory: CategoryTrait, TargetCategory: CategoryTrait> Debug
    for Functor<SourceCategory, TargetCategory>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Functor {{ id: {}, source_category: {:?}, target_category: {:?}, mappings: {:?} }}",
            self.id,
            self.source_category.category_id(),
            self.target_category.category_id(),
            self.mappings
        )
    }
}

impl<SourceCategory: CategoryTrait, TargetCategory: CategoryTrait> PartialEq
    for Functor<SourceCategory, TargetCategory>
{
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl<SourceCategory: CategoryTrait, TargetCategory: CategoryTrait> Eq
    for Functor<SourceCategory, TargetCategory>
{
}

impl<SourceCategory: CategoryTrait, TargetCategory: CategoryTrait> Hash
    for Functor<SourceCategory, TargetCategory>
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<SourceCategory: CategoryTrait, TargetCategory: CategoryTrait>
    Functor<SourceCategory, TargetCategory>
{
    pub fn new(
        id: String,
        source_category: Rc<SourceCategory>,
        target_category: Rc<TargetCategory>,
        morphism_mapping: HashMap<
            Rc<Morphism<CategorySubObjectAlias<SourceCategory>>>,
            Rc<Morphism<CategorySubObjectAlias<TargetCategory>>>,
        >,
    ) -> Self {
        let mut mappings = HashMap::new();
        let mut source_morphisms = HashMap::new();
        let mut target_morphisms = HashMap::new();

        for (source_morphism, target_morphism) in morphism_mapping.iter() {
            mappings.insert(source_morphism.id().clone(), target_morphism.id().clone());
            source_morphisms.insert(source_morphism.id().clone(), Rc::downgrade(source_morphism));
            target_morphisms.insert(target_morphism.id().clone(), Rc::downgrade(target_morphism));
        }

        Functor {
            id: ObjectId::Str(id),
            source_category,
            target_category,
            mappings,
            source_morphisms,
            target_morphisms,
        }
    }

    pub fn id(&self) -> &String {
        let s = match &self.id {
            ObjectId::Str(v) => v,
            _ => unreachable!(),
        };
        s
    }
}

impl<SourceCategory: CategoryTrait, TargetCategory: CategoryTrait>
    ArrowTrait<SourceCategory, TargetCategory> for Functor<SourceCategory, TargetCategory>
where
    Morphism<SourceCategory>: ArrowTrait<SourceCategory, TargetCategory>,
{
    fn source_object(&self) -> &Rc<SourceCategory> {
        &self.source_category
    }

    fn target_object(&self) -> &Rc<TargetCategory> {
        &self.target_category
    }

    fn is_identity(&self) -> bool {
        todo!()
    }

    fn compose(
        &self,
        other: &impl ArrowTrait<SourceCategory, TargetCategory>,
    ) -> Result<Rc<Morphism<SourceCategory>>, Errors> {
        todo!()
    }

    fn arrows(&self) -> Vec<&Morphism<SourceCategory>> {
        todo!()
    }
}

impl<SourceCategory: CategoryTrait, TargetCategory: CategoryTrait>
    FunctorTrait<SourceCategory, TargetCategory> for Functor<SourceCategory, TargetCategory>
where
    Morphism<<TargetCategory as CategoryTrait>::Object>: ArrowTrait<
            <SourceCategory as CategoryTrait>::Object,
            <SourceCategory as CategoryTrait>::Object,
        >,
    Morphism<SourceCategory>: ArrowTrait<SourceCategory, TargetCategory>,
{
    fn new(
        source_category: Rc<SourceCategory>,
        target_category: Rc<TargetCategory>,
        mappings: HashMap<
            Rc<Morphism<CategorySubObjectAlias<SourceCategory>>>,
            Rc<Morphism<CategorySubObjectAlias<TargetCategory>>>,
        >,
    ) -> Result<Self, Errors>
    where
        Self: Sized,
    {
        todo!()
    }

    fn source_category(&self) -> &Rc<SourceCategory> {
        todo!()
    }

    fn target_category(&self) -> &Rc<TargetCategory> {
        todo!()
    }

    fn arrow_mappings(
        &self,
    ) -> &HashMap<
        Rc<Morphism<CategorySubObjectAlias<SourceCategory>>>,
        Rc<Morphism<CategorySubObjectAlias<TargetCategory>>>,
    > {
        // self.mappings.iter().map(|item| {
        //     let source_morphism = self
        //         .source_morphisms
        //         .get(item.0)
        //         .and_then(|weak| weak.upgrade())
        //         .or_else(self.source_category)
        //     let target_morphism = self
        //         .target_morphisms
        //         .get(item.1)
        //         .and_then(|weak| weak.upgrade())
        //         .expect("Target morphism not found");
        //     (source_morphism, target_morphism)
        // }).collect()
        todo!()
    }
}

impl<
    SourceCategory: CategoryTrait + Clone,
    TargetCategory: CategoryTrait + Clone + Eq + Hash + Debug,
> CategoryTrait for Functor<SourceCategory, TargetCategory>
{
    type Object = TargetCategory;

    fn new() -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn new_with_id(id: &ObjectId) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn category_id(&self) -> &ObjectId {
        &self.id
    }

    fn add_object(&mut self, object: Rc<Self::Object>) -> Result<(), Errors> {
        todo!()
    }

    fn add_morphism(
        &mut self,
        morphism: Rc<Morphism<Self::Object>>,
    ) -> Result<&Rc<Morphism<Self::Object>>, Errors> {
        todo!()
    }

    fn get_object(&self, object: &Self::Object) -> Result<&Rc<Self::Object>, Errors> {
        todo!()
    }

    fn get_all_objects(&self) -> Result<HashSet<&Rc<Self::Object>>, Errors> {
        todo!()
    }

    fn get_all_morphisms(&self) -> Result<HashSet<&Rc<Morphism<Self::Object>>>, Errors> {
        todo!()
    }

    fn get_hom_set_x(
        &self,
        source_object: &Self::Object,
    ) -> Result<HashSet<&Rc<Morphism<Self::Object>>>, Errors> {
        todo!()
    }

    fn get_object_morphisms(
        &self,
        object: &Self::Object,
    ) -> Result<Vec<&Rc<Morphism<Self::Object>>>, Errors> {
        todo!()
    }
}
