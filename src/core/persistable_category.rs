use crate::DB;
use crate::core::arrow::Morphism;
use crate::core::dynamic_category::DynamicCategory;
use crate::core::errors::Errors;
use crate::core::identifier::Identifier;
use crate::core::object_id::ObjectId;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::{CategoryCloneWithNewId, CategoryTrait};
use crate::core::traits::functor_trait::FunctorTrait;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::string::ToString;
use std::sync::Arc;
use surrealdb::RecordId;
use surrealdb::sql::Thing;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PersistableCategory<InnerCategory>
where
    InnerCategory: CategoryTrait + Hash + Eq,
{
    category: InnerCategory,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PersistableCategoryObject {
    object_id: ObjectId,
}

impl PersistableCategoryObject {
    pub const TABLE_NAME: &'static str = "object";

    fn object_thing<Category: CategoryTrait>(category: &Category) -> Thing {
        Thing::from((
            Self::TABLE_NAME.to_string(),
            category.category_id().to_string(),
        ))
    }

    fn resource(&self) -> (String, String) {
        (Self::TABLE_NAME.to_string(), self.object_id.to_string())
    }

    pub async fn persist(&self) -> Result<(), Errors> {
        let record: Option<Record> = DB
            .create(self.resource())
            .content(PersistableCategoryObject {
                object_id: self.object_id.clone(),
            })
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;
        dbg!(record);
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PersistableArrow {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<RecordId>,
    #[serde(rename = "in")]
    source: Thing,
    #[serde(rename = "out")]
    target: Thing,
    #[serde(skip_serializing_if = "Option::is_none")]
    functor: Option<Thing>,
    category: Thing,
    is_identity: bool,
}

impl PersistableArrow {
    const MORPHISM_TABLE_NAME: &'static str = "morphism";

    const FUNCTOR_TABLE_NAME: &'static str = "functor";

    const ARROW_MAPPING_TABLE_NAME: &'static str = "functor_mapping";

    fn morphism_resource<Category: CategoryTrait, Morphism: ArrowTrait<Category, Category>>(
        morphism: &Morphism,
    ) -> (String, String) {
        (
            Self::MORPHISM_TABLE_NAME.to_string(),
            morphism.arrow_id().clone(),
        )
    }

    fn functor_resource<Category: CategoryTrait, Functor: FunctorTrait<Category, Category>>(
        functor: &Functor,
    ) -> (String, String) {
        (
            Self::FUNCTOR_TABLE_NAME.to_string(),
            functor.arrow_id().clone(),
        )
    }

    fn functor_thing<Category: CategoryTrait, Functor: FunctorTrait<Category, Category>>(
        functor: &Functor,
    ) -> Thing {
        Thing::from(Self::functor_resource(functor))
    }

    fn functor_mapping_thing<Category: CategoryTrait, Functor: FunctorTrait<Category, Category>>(
        functor: &Functor,
    ) -> Thing {
        Thing::from((
            Self::ARROW_MAPPING_TABLE_NAME.to_string(),
            functor.arrow_id().clone(),
        ))
    }

    pub fn arrow_thing<Category: CategoryTrait>(morphism: &Category::Morphism) -> Thing {
        Thing::from(PersistableArrow::morphism_resource(morphism))
    }

    async fn create_functor<Category: CategoryTrait, Functor: FunctorTrait<Category, Category>>(
        functor: &Functor,
    ) -> Result<Thing, Errors> {
        let functor_query = r#"
        RELATE $source_category -> $functor_id -> $target_category
        RETURN *
        "#;

        println!(
            "Source category id: {:?}",
            functor.source_object().category_id()
        );
        println!(
            "Target category id: {:?}",
            functor.target_object().category_id()
        );
        let response = DB
            .query(functor_query)
            .bind(("functor_id", Self::functor_thing(functor)))
            .bind((
                "source_category",
                PersistableCategoryObject::object_thing(&**functor.source_object()),
            ))
            .bind((
                "target_category",
                PersistableCategoryObject::object_thing(&**functor.target_object()),
            ))
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;
        println!("{:?}", response);
        dbg!(&response);

        let query = r#"
        RELATE $source_morphism -> functor_mapping -> $target_morphisms
        SET functor = $functor,
            created_at = time::now()
        RETURN *
        "#;

        for (source_morphism, target_morphism) in functor.morphisms_mappings() {
            let response = DB
                .query(query)
                .bind((
                    "source_morphism",
                    Self::arrow_thing::<Category>(&**source_morphism),
                ))
                .bind((
                    "target_morphisms",
                    Self::arrow_thing::<Category>(&**target_morphism),
                ))
                .bind(("functor", Self::functor_thing(functor)))
                .await
                .map_err(|e| Errors::DatabaseError(e.to_string()))?;
            dbg!(&response);
        }
        Ok(Thing::from(Self::functor_resource(functor)))
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PersistableCategoryFunctor {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<RecordId>,
    #[serde(rename = "in")]
    source: Thing,
    #[serde(rename = "out")]
    target: Thing,
    is_identity: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct PersistableCategoryFunctorMapping {
    functor_id: Thing,
    source_morphism: Thing,
    target_morphism: Thing,
}

#[derive(Debug, Deserialize)]
struct Record {
    id: RecordId,
}

impl<InnerCategory> PersistableCategory<InnerCategory>
where
    InnerCategory: CategoryTrait + Hash + Eq + Clone,
{
    pub async fn new() -> Result<Self, Errors> {
        let category = PersistableCategory {
            category: InnerCategory::new().await?,
        };
        let object = PersistableCategoryObject {
            object_id: category.category.category_id().clone(),
        };
        object.persist().await?;
        Ok(category)
    }

    pub async fn new_with_id(id: ObjectId) -> Result<Self, Errors> {
        let category = PersistableCategory {
            category: InnerCategory::new_with_id(id.into()).await?,
        };
        let object = PersistableCategoryObject {
            object_id: category.category.category_id().clone(),
        };
        object.persist().await?;
        Ok(category)
    }

    pub fn inner_category(&self) -> &InnerCategory {
        &self.category
    }

    pub fn thing(&self) -> Thing {
        PersistableCategoryObject::object_thing(&self.category)
    }

    async fn create_record<Category: CategoryTrait>(
        &self,
        object: &Category,
    ) -> Result<(), Errors> {
        // now persist the object
        let sql = r#"
        LET $object = (UPSERT type::thing($table_name, $id));
        RETURN $object;
        "#;

        let response = DB
            .query(sql)
            .bind(("table_name", PersistableCategoryObject::TABLE_NAME))
            .bind(("id", object.category_id().to_string()))
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;
        dbg!(response);
        Ok(())
    }

    async fn create_morphism(&self, morphism: &InnerCategory::Morphism) -> Result<(), Errors> {
        let sql = r#"
            LET $m = (RELATE $src->$rel_id->$dst
                SET is_identity = $is_identity,
                    functor = $functor,
                    created_at = time::now()
                RETURN id);
            RELATE $m -> morphism_in -> $category;
            RETURN $m;
        "#;

        let response = DB
            .query(sql)
            .bind(("rel_id", Self::arrow_thing(&*morphism)))
            .bind((
                "src",
                PersistableCategoryObject::object_thing(&**morphism.source_object()),
            ))
            .bind((
                "dst",
                PersistableCategoryObject::object_thing(&**morphism.target_object()),
            ))
            .bind(("is_identity", morphism.is_identity()))
            .bind(("category", self.thing()))
            .bind((
                "functor",
                morphism
                    .functor()
                    .map(|f| PersistableArrow::functor_thing(f)),
            ))
            .await
            .map_err(|e| Errors::DatabaseError(e.to_string()))?;
        dbg!(response);
        Ok(())
    }

    pub fn arrow_thing(morphism: &InnerCategory::Morphism) -> Thing {
        Thing::from(PersistableArrow::morphism_resource(morphism))
    }

    fn functor_thing<Functor: FunctorTrait<InnerCategory, InnerCategory>>(
        functor: &Functor,
    ) -> Thing {
        Thing::from(PersistableArrow::functor_resource(functor))
    }
}

#[async_trait]
impl<InnerCategory> CategoryTrait for PersistableCategory<InnerCategory>
where
    InnerCategory: CategoryTrait + Hash + Eq + Clone,
{
    type Object = InnerCategory::Object;
    type Morphism = InnerCategory::Morphism;

    async fn new_with_id(object_id: ObjectId) -> Result<Self, Errors>
    where
        Self: Sized,
    {
        Ok(PersistableCategory::new_with_id(object_id).await.unwrap())
    }

    fn category_id(&self) -> &ObjectId {
        &self.category.category_id()
    }

    async fn add_object(
        &mut self,
        object: Arc<Self::Object>,
    ) -> Result<Arc<Self::Morphism>, Errors> {
        self.create_record(&*object).await?;
        let identity_morphism = self.category.add_object(object.clone()).await?;
        self.create_morphism(&*identity_morphism).await?;
        Ok(identity_morphism)
    }

    async fn add_morphism(&mut self, morphism: Arc<Self::Morphism>) -> Result<(), Errors> {
        // create morphism in the inner category before persisting
        self.category.add_morphism(morphism.clone()).await?;

        // if functor is not none and does not exist in the database create it
        if let Some(functor) = morphism.functor() {
            let functor_record: Option<serde_json::Value> = DB
                .select(PersistableArrow::functor_resource(functor))
                .await?;
            dbg!(&functor_record);
            if functor_record.is_none() {
                // if its none create it.
                PersistableArrow::create_functor(functor).await?;
            }
        }

        let record = self.create_morphism(&*morphism).await?;
        dbg!(record);
        Ok(())
    }

    async fn get_object(&self, object: &Self::Object) -> Result<&Arc<Self::Object>, Errors> {
        self.category.get_object(object).await
    }

    async fn get_all_objects(&self) -> Result<HashSet<&Arc<Self::Object>>, Errors> {
        self.category.get_all_objects().await
    }

    async fn get_morphism(&self, morphism_id: &String) -> Result<&Arc<Self::Morphism>, Errors> {
        self.category.get_morphism(morphism_id).await
    }

    async fn get_all_morphisms(&self) -> Result<HashSet<&Arc<Self::Morphism>>, Errors> {
        self.category.get_all_morphisms().await
    }

    async fn get_hom_set_x(
        &self,
        source_object: &Self::Object,
    ) -> Result<HashSet<&Arc<Self::Morphism>>, Errors> {
        self.category.get_hom_set_x(source_object).await
    }

    async fn get_object_morphisms(
        &self,
        object: &Self::Object,
    ) -> Result<Vec<&Arc<Self::Morphism>>, Errors> {
        self.category.get_object_morphisms(object).await
    }
}

#[async_trait]
impl<InnerCategory> CategoryCloneWithNewId for PersistableCategory<InnerCategory>
where
    InnerCategory: CategoryTrait + Hash + Eq + Clone + Send + Sync,
{
    async fn clone_with_new_id(&self) -> Result<Self, Errors>
    where
        Self: Sized,
    {
        // for persistable category we need to create a new record in the database
        // so we clone the inner category with a new id and then create a new persistable
        // category with that inner category
        let mut category = InnerCategory::new().await?;
        // populate all the objects and morphisms from self to category
        for object in self.get_all_objects().await? {
            category.add_object(object.clone()).await?;
        }

        for morphism in self.get_all_morphisms().await? {
            category.add_morphism(morphism.clone()).await?;
        }

        let persistable_category = PersistableCategory { category };
        Ok(persistable_category)
    }
}

impl<InnerCategory> From<String> for PersistableCategory<InnerCategory>
where
    InnerCategory: CategoryTrait + Hash + Eq + From<String>,
{
    fn from(s: String) -> Self {
        PersistableCategory { category: s.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::dynamic_category::DynamicCategory;
    use crate::core::functor::Functor;
    use crate::core::traits::category_trait::CategoryFromObjects;
    use std::collections::HashMap;
    use tokio::sync::OnceCell;

    static TEST_DB_INIT: OnceCell<()> = OnceCell::const_new();

    pub async fn init_db_once() {
        TEST_DB_INIT
            .get_or_init(|| async {
                // Your existing initialization
                crate::init_db(None).await.expect("DB init failed");
            })
            .await;
    }

    #[tokio::test]
    async fn test_persistable_category() {
        init_db_once().await;
        let mut category: PersistableCategory<DynamicCategory> =
            PersistableCategory::new().await.unwrap();

        let identity_morphism1 = category
            .add_object(Arc::new("TestObject".into()))
            .await
            .unwrap();
        let identity_morphism2 = category
            .add_object(Arc::new("TestObject2".into()))
            .await
            .unwrap();

        // add morphism between the two objects
        let morphism = Arc::new(Morphism::new(
            String::generate(),
            identity_morphism1.source_object().clone(),
            identity_morphism2.source_object().clone(),
            None,
        ));
        category.add_morphism(morphism).await.unwrap();

        // try get the first object
        let obj = category.get_object(&"TestObject".into()).await;
        assert!(obj.is_ok());
        let obj = obj.unwrap();
        assert_eq!(obj.category_id(), &ObjectId::Str("TestObject".to_string()));

        let mut category2: PersistableCategory<DynamicCategory> =
            PersistableCategory::new().await.unwrap();
        // create same object as in category 2 to make sure they are independent
        category2
            .add_object(Arc::new("TestObject".into()))
            .await
            .unwrap();

        // now create a higher category that contains the two previous categories as objects
        let mut higher_category: PersistableCategory<DynamicCategory> =
            PersistableCategory::new().await.unwrap();
        higher_category
            .add_object(Arc::new(category.inner_category().clone()))
            .await
            .unwrap();
        higher_category
            .add_object(Arc::new(category2.inner_category().clone()))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_persistable_morphisms() {
        init_db_once().await;
        let mut category_abc: PersistableCategory<DynamicCategory> =
            PersistableCategory::new_with_id("abc".into())
                .await
                .unwrap();

        // PersistableCategory::from_objectdo()
        // let catb: PersistableCategory<DynamicCategory> = PersistableCategory::from_objects::<DynamicCategory>(vec!["1".into(), "2".into(), "3".into()]).await.unwrap();

        // objects a, b and c.
        let object_a = Arc::new(DynamicCategory::new_with_id("a".into()));
        category_abc.add_object(object_a.clone()).await.unwrap();
        let object_b = Arc::new(DynamicCategory::new_with_id("b".into()));
        category_abc.add_object(object_b.clone()).await.unwrap();
        let object_c = Arc::new(DynamicCategory::new_with_id("c".into()));
        category_abc.add_object(object_c.clone()).await.unwrap();
        let category_abc = Arc::new(category_abc.inner_category().clone());

        // objects A, B and C.
        let mut category_ABC: PersistableCategory<DynamicCategory> =
            PersistableCategory::new_with_id("ABC".into())
                .await
                .unwrap();
        let object_A = Arc::new(DynamicCategory::new_with_id("A".into()));
        category_ABC.add_object(object_A.clone()).await.unwrap();
        let object_B = Arc::new(DynamicCategory::new_with_id("B".into()));
        category_ABC.add_object(object_B.clone()).await.unwrap();
        let object_C = Arc::new(DynamicCategory::new_with_id("C".into()));
        category_ABC.add_object(object_C.clone()).await.unwrap();
        let category_ABC = Arc::new(category_ABC.inner_category().clone());

        // functor from objectabc to objectABC
        let mut mapping = HashMap::new();
        mapping.insert(
            category_abc
                .get_identity_morphism(&"a".into())
                .await
                .unwrap()
                .clone(),
            category_ABC
                .get_identity_morphism(&"A".into())
                .await
                .unwrap()
                .clone(),
        );
        mapping.insert(
            category_abc
                .get_identity_morphism(&"b".into())
                .await
                .unwrap()
                .clone(),
            category_ABC
                .get_identity_morphism(&"B".into())
                .await
                .unwrap()
                .clone(),
        );
        mapping.insert(
            category_abc
                .get_identity_morphism(&"c".into())
                .await
                .unwrap()
                .clone(),
            category_ABC
                .get_identity_morphism(&"C".into())
                .await
                .unwrap()
                .clone(),
        );

        let functor = Arc::new(Functor::new(
            String::generate(),
            category_abc.clone(),
            category_ABC.clone(),
            mapping,
        ));

        // morphism from objectabc to objectABC
        let morphism = Arc::new(Morphism::new(
            String::generate(),
            category_abc.clone(),
            category_ABC.clone(),
            Some(functor.clone()),
        ));

        let mut category: PersistableCategory<DynamicCategory> =
            PersistableCategory::new_with_id("ABCabc".into())
                .await
                .unwrap();

        category.add_object(category_abc.clone()).await.unwrap();
        category.add_object(category_ABC.clone()).await.unwrap();
        category.add_morphism(morphism.clone()).await.unwrap();
    }
}
