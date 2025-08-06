use std::fmt::Debug;

use crate::core::identifier::Identifier;
use crate::core::traits::category_trait::CategoryTrait;

pub trait MorphismMappingTreeTrait<'a>: Debug {
    type SourceCategory: CategoryTrait<'a>;
    type TargetCategory: CategoryTrait<'a>;
    type Id: Identifier;

    fn id(&self) -> &Self::Id;
    fn source_cell(&self) -> &<Self::SourceCategory as CategoryTrait<'a>>::Morphism;
    fn target_cell(&self) -> &<Self::TargetCategory as CategoryTrait<'a>>::Morphism;
}

#[derive(Debug)]
pub struct MorphismMappingTree<
    'a,
    Id: Identifier,
    SourceCategory: CategoryTrait<'a>,
    TargetCategory: CategoryTrait<'a>,
> {
    id: &'a Id,
    source_cell: &'a SourceCategory::Morphism,
    target_cell: &'a TargetCategory::Morphism,
    children: Vec<
        Box<
            dyn MorphismMappingTreeTrait<
                    'a,
                    SourceCategory = <SourceCategory as CategoryTrait<'a>>::Object,
                    TargetCategory = <TargetCategory as CategoryTrait<'a>>::Object,
                    Id = Id,
                >,
        >,
    >,
}

impl<'a, Id: Identifier, SourceCategory: CategoryTrait<'a>, TargetCategory: CategoryTrait<'a>>
    MorphismMappingTree<'a, Id, SourceCategory, TargetCategory>
{
    pub fn new(
        id: &'a Id,
        source_cell: &'a SourceCategory::Morphism,
        target_cell: &'a TargetCategory::Morphism,
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
        source_cell: &'a SourceCategory::Morphism,
        target_cell: &'a TargetCategory::Morphism,
        children: Vec<
            Box<
                dyn MorphismMappingTreeTrait<
                        'a,
                        SourceCategory = <SourceCategory as CategoryTrait<'a>>::Object,
                        TargetCategory = <TargetCategory as CategoryTrait<'a>>::Object,
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

    pub fn source_cell(&self) -> &<SourceCategory as CategoryTrait<'a>>::Morphism {
        self.source_cell()
    }

    pub fn target_cell_id(&self) -> &<TargetCategory as CategoryTrait<'a>>::Morphism {
        self.target_cell
    }
}

impl<'a, Id: Identifier, SourceCategory: CategoryTrait<'a>, TargetCategory: CategoryTrait<'a>>
    MorphismMappingTreeTrait<'a> for MorphismMappingTree<'a, Id, SourceCategory, TargetCategory>
{
    type SourceCategory = SourceCategory;
    type TargetCategory = TargetCategory;
    type Id = Id;

    fn id(&self) -> &Self::Id {
        self.id
    }

    fn source_cell(&self) -> &<Self::SourceCategory as CategoryTrait<'a>>::Morphism {
        self.source_cell
    }

    fn target_cell(&self) -> &<Self::TargetCategory as CategoryTrait<'a>>::Morphism {
        self.target_cell
    }
}
