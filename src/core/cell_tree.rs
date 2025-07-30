use crate::core::identifier::Identifier;

pub struct CellTree<'a, Id: Identifier> {
    cell_id: &'a Id,
    source_cell_id: &'a Id,
    target_cell_id: &'a Id,
    children: Vec<CellTree<'a, Id>>,
}

impl <'a, Id: Identifier> CellTree<'a, Id> {
    pub fn new(cell_id: &'a Id, source_cell_id: &'a Id, target_cell_id: &'a Id) -> Self {
        CellTree {
            cell_id,
            source_cell_id,
            target_cell_id,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: CellTree<'_, Id>) {
        todo!()
    }

    pub fn cell_id(&self) -> &Id {
        self.cell_id
    }

    pub fn source_cell_id(&self) -> &Id {
        self.source_cell_id
    }

    pub fn target_cell_id(&self) -> &Id {
        self.target_cell_id
    }
}

