use crate::core::generic_id::GenericObjectIdTrait;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UuidCategoryObjectId {
    id: Uuid,
}

impl Display for UuidCategoryObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl UuidCategoryObjectId {
    pub fn new() -> Self {
        UuidCategoryObjectId { id: Uuid::new_v4() }
    }

    pub fn from_uuid(id: Uuid) -> Self {
        UuidCategoryObjectId { id }
    }

}

impl From<Uuid> for UuidCategoryObjectId {
    fn from(uuid: Uuid) -> Self {
        UuidCategoryObjectId::from_uuid(uuid)
    }
}

impl Default for UuidCategoryObjectId {
    fn default() -> Self {
        Self::new()
    }
}

impl GenericObjectIdTrait for UuidCategoryObjectId {
    fn new() -> Self {
        UuidCategoryObjectId::new()
    }
}


impl GenericObjectIdTrait for String {
    fn new() -> Self {
        Uuid::new_v4().to_string()
    }
}