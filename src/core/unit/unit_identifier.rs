use crate::core::identifier::Identifier;

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct UnitIdentifier{}

impl Identifier for UnitIdentifier {
    type Id = ();

    fn id(&self) -> &Self::Id {
        &()
    }

    fn generate() -> Self {
        UnitIdentifier {}
    }
}