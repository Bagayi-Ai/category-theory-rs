use crate::core::category::{CategoryObjectIdTrait, MorphismIdTrait};
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

impl CategoryObjectIdTrait for UuidCategoryObjectId {
    type IdType = Uuid;

    fn new() -> Self {
        UuidCategoryObjectId::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UuidMorphismId {
    id: UuidCategoryObjectId,
    left_id: Option<Box<UuidMorphismId>>,
    right_id: Option<Box<UuidMorphismId>>,
}

impl UuidMorphismId {
    pub fn new() -> Self {
        let id = UuidCategoryObjectId::new();
        UuidMorphismId {
            id,
            left_id: None,
            right_id: None,
        }
    }

    pub fn from_uuid(id: Uuid) -> Self {
        UuidMorphismId {
            id: UuidCategoryObjectId::from_uuid(id),
            left_id: None,
            right_id: None,
        }
    }

    pub fn is_simple(&self) -> bool {
        self.validate_invariant();
        self.left_id.is_none() && self.right_id.is_none()
    }

    /// Validate that the morphism maintains the invariant:
    /// Either both left_id and right_id exist, or neither exists
    fn validate_invariant(&self) {
        match (&self.left_id, &self.right_id) {
            (None, None) => {}       // Valid: simple morphism
            (Some(_), Some(_)) => {} // Valid: composite morphism
            (Some(_), None) => panic!(
                "Invalid UuidMorphismId state: left_id exists but right_id is None for morphism {}",
                self.id
            ),
            (None, Some(_)) => panic!(
                "Invalid UuidMorphismId state: right_id exists but left_id is None for morphism {}",
                self.id
            ),
        }
    }
}

impl MorphismIdTrait for UuidMorphismId {
    type IdType = UuidCategoryObjectId;

    fn new(id: Self::IdType) -> Self {
        UuidMorphismId {
            id,
            left_id: None,
            right_id: None,
        }
    }

    fn id(&self) -> &Self::IdType {
        &self.id
    }

    fn compose(&self, other: &Self) -> Self {
        // Validate both inputs before composing
        self.validate_invariant();
        other.validate_invariant();

        UuidMorphismId {
            id: UuidCategoryObjectId::from_uuid(Uuid::new_v4()), // Generate a new UUID for the composed morphism
            left_id: Some(Box::new(self.clone())),
            right_id: Some(Box::new(other.clone())),
        }
    }

    fn decompose(&self) -> Option<(Self, Self)> {
        self.validate_invariant();

        match (&self.left_id, &self.right_id) {
            (Some(left), Some(right)) => {
                // Validate the components before returning them
                left.validate_invariant();
                right.validate_invariant();
                Some(((**left).clone(), (**right).clone()))
            }
            (None, None) => None, // Simple morphism cannot be decomposed
            // The validate_invariant() call above ensures we never reach the invalid cases
            _ => unreachable!("Invalid state should have been caught by validate_invariant"),
        }
    }
}

impl Default for UuidMorphismId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for UuidMorphismId {
    fn from(uuid: Uuid) -> Self {
        UuidMorphismId::from_uuid(uuid)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_simple_morphism() {
        let morphism = UuidMorphismId::new();
        assert!(morphism.is_simple());
        assert!(!morphism.is_composite());
        assert!(morphism.decompose().is_none());
    }

    #[test]
    fn test_composite_morphism() {
        let m1 = UuidMorphismId::new();
        let m2 = UuidMorphismId::new();
        let composed = m1.compose(&m2);

        assert!(!composed.is_simple());
        assert!(composed.is_composite());

        let decomposed = composed.decompose();
        assert!(decomposed.is_some());

        if let Some((left, right)) = decomposed {
            assert_eq!(left.id(), m1.id());
            assert_eq!(right.id(), m2.id());
        }
    }

    #[test]
    #[should_panic(expected = "Invalid UuidMorphismId state: left_id exists but right_id is None")]
    fn test_invalid_state_left_only() {
        let mut morphism = UuidMorphismId::new();
        // Manually create an invalid state (this would never happen in normal usage)
        morphism.left_id = Some(Box::new(UuidMorphismId::new()));
        // This should panic when validate_invariant is called
        morphism.is_simple();
    }

    #[test]
    #[should_panic(expected = "Invalid UuidMorphismId state: right_id exists but left_id is None")]
    fn test_invalid_state_right_only() {
        let mut morphism = UuidMorphismId::new();
        // Manually create an invalid state (this would never happen in normal usage)
        morphism.right_id = Some(Box::new(UuidMorphismId::new()));
        // This should panic when validate_invariant is called
        morphism.is_composite();
    }

    #[test]
    fn test_nested_composition() {
        let m1 = UuidMorphismId::new();
        let m2 = UuidMorphismId::new();
        let m3 = UuidMorphismId::new();

        let composed_12 = m1.compose(&m2);
        let composed_123 = composed_12.compose(&m3);

        assert!(composed_123.is_composite());

        if let Some((left, right)) = composed_123.decompose() {
            assert!(left.is_composite()); // Should be the composition of m1 and m2
            assert!(right.is_simple()); // Should be m3
        }
    }
}
