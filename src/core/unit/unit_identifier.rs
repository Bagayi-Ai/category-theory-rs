use crate::core::identifier::Identifier;
use std::fmt::Display;

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct UnitIdentifier {}

impl Display for UnitIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UnitIdentifier")
    }
}

impl Identifier for UnitIdentifier {
    type Id = ();

    fn id(&self) -> &Self::Id {
        &()
    }

    fn generate() -> Self {
        UnitIdentifier {}
    }
}
