use std::collections::HashMap;
use crate::core::identifier::Identifier;
use crate::core::ncategory::{NCategory, UnitCategory};

pub trait NCell
where
    Self::Category: NCategory,
    <Self::Category as NCategory>::BaseCategory: NCategory<Identifier = <Self::Category as NCategory>::Identifier>,
{
    type Category: NCategory;

    type BaseCell: NCell<Category = <Self::Category as NCategory>::BaseCategory>;

    fn id(&self) -> &<Self::Category as NCategory>::Identifier;

    fn source_category_id(&self) -> &<Self::Category as NCategory>::Identifier;

    fn source_object_id(&self) -> &<Self::Category as NCategory>::Identifier;

    fn target_category_id(&self) -> &<Self::Category as NCategory>::Identifier;

    fn target_object_id(&self) -> &<Self::Category as NCategory>::Identifier;

    fn category_id(&self) -> &<Self::Category as NCategory>::Identifier;

    fn base_cell_id(&self) -> &<<Self::BaseCell as NCell>::Category as NCategory>::Identifier;

    fn base_cell(&self) -> Self::BaseCell;
}

impl <T: Identifier> NCell for UnitCategory<T> {
    type Category = UnitCategory<T>;

    type BaseCell = UnitCategory<T>;

    fn id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn source_category_id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn source_object_id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn target_category_id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn target_object_id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn category_id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn base_cell_id(&self) -> &<<Self::BaseCell as NCell>::Category as NCategory>::Identifier {
        todo!()
    }

    fn base_cell(&self) -> Self::BaseCell {
        todo!()
    }
}