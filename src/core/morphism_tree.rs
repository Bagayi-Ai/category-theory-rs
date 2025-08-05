use std::fmt::Debug;

use crate::core::identifier::Identifier;
use crate::core::ncategory::NCategory;

pub trait MorphismMappingTreeTrait<'a>: Debug {
    type SourceCategory: NCategory<'a>;
    type TargetCategory: NCategory<'a>;
    type Id: Identifier;

    fn id(&self) -> &Self::Id;
    fn source_cell(&self) -> &<Self::SourceCategory as NCategory<'a>>::Arrow;
    fn target_cell(&self) -> &<Self::TargetCategory as NCategory<'a>>::Arrow;
}

#[derive(Debug)]
pub struct MorphismMappingTree<
    'a,
    Id: Identifier,
    SourceCategory: NCategory<'a>,
    TargetCategory: NCategory<'a>,
> {
    id: &'a Id,
    source_cell: &'a SourceCategory::Arrow,
    target_cell: &'a TargetCategory::Arrow,
    children: Vec<
        Box<
            dyn MorphismMappingTreeTrait<
                    'a,
                    SourceCategory = <SourceCategory as NCategory<'a>>::Object,
                    TargetCategory = <TargetCategory as NCategory<'a>>::Object,
                    Id = Id,
                >,
        >,
    >,
}

impl<'a, Id: Identifier, SourceCategory: NCategory<'a>, TargetCategory: NCategory<'a>>
    MorphismMappingTree<'a, Id, SourceCategory, TargetCategory>
{
    pub fn new(
        id: &'a Id,
        source_cell: &'a SourceCategory::Arrow,
        target_cell: &'a TargetCategory::Arrow,
    ) -> Self {
        MorphismMappingTree {
            id,
            source_cell,
            target_cell,
            children: Vec::new(),
        }
    }

    pub fn new_with_children(
        id: &'a Id,
        source_cell: &'a SourceCategory::Arrow,
        target_cell: &'a TargetCategory::Arrow,
        children: Vec<
            Box<
                dyn MorphismMappingTreeTrait<
                        'a,
                        SourceCategory = <SourceCategory as NCategory<'a>>::Object,
                        TargetCategory = <TargetCategory as NCategory<'a>>::Object,
                        Id = Id,
                    >,
            >,
        >,
    ) -> Self {
        MorphismMappingTree {
            id,
            source_cell,
            target_cell,
            children,
        }
    }

    pub fn add_child(
        &mut self,
        child: MorphismMappingTree<'a, Id, SourceCategory, TargetCategory>,
    ) {
        todo!()
    }

    pub fn id(&self) -> &Id {
        self.id
    }

    pub fn source_cell(&self) -> &<SourceCategory as NCategory<'a>>::Arrow {
        self.source_cell()
    }

    pub fn target_cell_id(&self) -> &<TargetCategory as NCategory<'a>>::Arrow {
        self.target_cell
    }
}

impl<'a, Id: Identifier, SourceCategory: NCategory<'a>, TargetCategory: NCategory<'a>>
    MorphismMappingTreeTrait<'a> for MorphismMappingTree<'a, Id, SourceCategory, TargetCategory>
{
    type SourceCategory = SourceCategory;
    type TargetCategory = TargetCategory;
    type Id = Id;

    fn id(&self) -> &Self::Id {
        self.id
    }

    fn source_cell(&self) -> &<Self::SourceCategory as NCategory<'a>>::Arrow {
        self.source_cell
    }

    fn target_cell(&self) -> &<Self::TargetCategory as NCategory<'a>>::Arrow {
        self.target_cell
    }
}
