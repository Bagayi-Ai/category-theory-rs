use std::collections::HashMap;
use crate::core::identifier::Identifier;
use crate::core::ncategory::{NCategory, UnitCategory};

struct CellTree
{
    cell_id: String,
    source_object_id: String,
    target_object_id: String,
    children: HashMap<String, CellTree>,
}

pub trait NCell
where
    Self::Category: NCategory,
    <Self::Category as NCategory>::BaseCategory: NCategory<Identifier = <Self::Category as NCategory>::Identifier>,
{
    type Category: NCategory;

    type BaseCell: NCell<Category = <Self::Category as NCategory>::BaseCategory>;

    fn id(&self) -> &<Self::Category as NCategory>::Identifier;

    fn source_category(&self) -> &Self::Category;

    fn source_category_id(&self) -> &<Self::Category as NCategory>::Identifier;

    fn source_object(&self) -> &<Self::Category as NCategory>::Object;

    fn source_object_id(&self) -> &<Self::Category as NCategory>::Identifier;

    fn target_category(&self) -> &Self::Category;

    fn target_category_id(&self) -> &<Self::Category as NCategory>::Identifier;

    fn target_object(&self) -> &<Self::Category as NCategory>::Object;

    fn target_object_id(&self) -> &<Self::Category as NCategory>::Identifier;

    fn category_id(&self) -> &<Self::Category as NCategory>::Identifier;

    fn base_cell_id(&self) -> &<<Self::BaseCell as NCell>::Category as NCategory>::Identifier;

    fn base_cell(&self) -> Self::BaseCell;

    fn cell_tree(&self) -> CellTree {
        /*
            Cell tree is a recursive structure that represents the hierarchy of cells and mapping
            of objects.

            We first drill down all the way down the BaseCell until we reach the () cell then start
            reconstructing the cell tree by mapping the BaseCell to the current cell.
        */

        // let mut cell_tree = CellTree {
        //     cell_id: self.id().to_string(),
        //     source_object_id: self.source_object_id().to_string(),
        //     target_object_id: self.target_object_id().to_string(),
        //     children: HashMap::new(),
        // };
        //
        // // now we need to map cells for the base source object to the base target object
        // let base_cell = self.base_cell();
       todo!()

    }
}

impl <T: Identifier> NCell for UnitCategory<T> {
    type Category = UnitCategory<T>;

    type BaseCell = UnitCategory<T>;

    fn id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn source_category(&self) -> &Self::Category {
        todo!()
    }

    fn source_category_id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn source_object(&self) -> &<Self::Category as NCategory>::Object {
        todo!()
    }

    fn source_object_id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn target_category(&self) -> &Self::Category {
        todo!()
    }

    fn target_category_id(&self) -> &<Self::Category as NCategory>::Identifier {
        todo!()
    }

    fn target_object(&self) -> &<Self::Category as NCategory>::Object {
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