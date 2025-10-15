use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use serde::Serialize;
use serde_json::{json, Value};

use crate::core::errors::Errors;
use crate::core::traits::arrow_trait::ArrowTrait;
use crate::core::traits::category_trait::CategoryTrait;

/// A simple strongly-typed representation (mainly for documentation) of a Cytoscape element.
/// We ultimately emit `serde_json::Value` so consumers can extend freely without changing this crate.
#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum CytoscapeElement {
    Node { data: HashMap<String, Value> },
    Edge { data: HashMap<String, Value> },
}

const EMPTY_ID_SENTINEL: &str = "__empty_string__";

fn normalized_object_id<O: CategoryTrait>(obj: &Arc<O>) -> String {
    let raw = obj.category_id().to_string();
    if raw.is_empty() { EMPTY_ID_SENTINEL.to_string() } else { raw }
}

impl CytoscapeElement {
    fn node(id: &str, label: &str, parent: Option<&str>) -> Self {
        let mut data = HashMap::new();
        data.insert("id".into(), json!(id));
        data.insert("label".into(), json!(label));
        if let Some(p) = parent { data.insert("parent".into(), json!(p)); }
        CytoscapeElement::Node { data }
    }
    fn edge(id: &str, source: &str, target: &str, label: &str, functor: bool, identity: bool) -> Self {
        let mut data = HashMap::new();
        data.insert("id".into(), json!(id));
        data.insert("source".into(), json!(source));
        data.insert("target".into(), json!(target));
        if !label.is_empty() { data.insert("label".into(), json!(label)); }
        if functor { data.insert("functorEdge".into(), json!(true)); }
        if identity { data.insert("identity".into(), json!(true)); }
        CytoscapeElement::Edge { data }
    }
}

/// Convert a (possibly flat) category into Cytoscape elements (nodes + edges).
/// This treats every object as a top-level node (no compound nesting) and enumerates all morphisms.
/// For categories whose `Object` is itself a `CategoryTrait`, consider using `nested_category_to_cytoscape_elements` instead.
pub async fn category_to_cytoscape_elements<C>(category: &C) -> Result<Vec<Value>, Errors>
where
    C: CategoryTrait + Sync,
    C::Object: CategoryTrait + Sync,
    C::Morphism: ArrowTrait<C::Object, C::Object> + Sync,
{
    let mut elements: Vec<Value> = Vec::new();
    let objects = category.get_all_objects().await?; // HashSet<&Arc<C::Object>>

    // Nodes
    for obj in &objects {
        let id_str = normalized_object_id(obj);
        elements.push(serde_json::to_value(CytoscapeElement::node(&id_str, &id_str, None)).unwrap());
    }

    // Edges (collect via each object's outgoing morphisms to ensure we get morphisms even if get_all_morphisms is unimplemented)
    let mut seen_edges: HashSet<String> = HashSet::new();
    for obj in &objects {
        for morphism in category.get_object_morphisms(&***obj).await? { // Vec<&Arc<C::Morphism>>
            let id = morphism.arrow_id().to_string();
            if !seen_edges.insert(id.clone()) { continue; }
            let source = normalized_object_id(morphism.source_object());
            let target = normalized_object_id(morphism.target_object());
            let is_identity = morphism.is_identity();
            // Attempt to see if this morphism stems from a functor mapping (heuristic: functor present OR non-identity with mapping)
            let has_functor = morphism.functor().is_some();
            let label = if is_identity { "id" } else { morphism.arrow_id() };
            elements.push(serde_json::to_value(CytoscapeElement::edge(&id, &source, &target, label, has_functor, is_identity)).unwrap());
        }
    }

    Ok(elements)
}

/// Recursively convert a nested category into Cytoscape compound nodes + edges.
/// Relaxed version: we no longer require `Object = <Object as CategoryTrait>::Object` (idempotent object type).
/// Instead, we generically recurse as long as each encountered object’s `Object` associated type also
/// implements `CategoryTrait`. Because Rust monomorphizes each recursion level separately, we do not need
/// a single homogeneous vector storing all levels. We only collect:
///  * Nodes: every object encountered (added once by id)
///  * Edges: ONLY those belonging to the top-level category (same behavior as before)
/// If you want inner-level morphisms added, set `include_inner_morphisms = true` (future enhancement).
pub async fn nested_category_to_cytoscape_elements<C>(
    category: &C,
    max_depth: usize,
) -> Result<Vec<Value>, Errors>
where
    C: CategoryTrait + Sync,
    C::Object: CategoryTrait + Sync,
    C::Morphism: ArrowTrait<C::Object, C::Object> + Sync,
{
    use futures::future::BoxFuture;
    let mut elements: Vec<Value> = Vec::new();
    let mut seen_nodes: HashSet<String> = HashSet::new();

    // Boxed recursive helper to avoid infinitely sized future.
    fn expand<O>(
        obj: Arc<O>,
        depth: usize,
        max_depth: usize,
        parent: Option<String>,
        mut seen: HashSet<String>,
        mut elements: Vec<Value>,
    ) -> BoxFuture<'static, Result<(HashSet<String>, Vec<Value>), Errors>>
    where
        O: CategoryTrait + Sync + 'static,
        O::Object: CategoryTrait + Sync + 'static,
        O::Morphism: ArrowTrait<O::Object, O::Object> + Sync + 'static,
    {
        Box::pin(async move {
            let id_str = normalized_object_id(&obj);
            if seen.insert(id_str.clone()) {
                elements.push(serde_json::to_value(CytoscapeElement::node(&id_str, &id_str, parent.as_deref())).unwrap());
            }
            if depth < max_depth {
                if let Ok(children) = obj.get_all_objects().await {
                    for child in children {
                        let child_arc = child.clone();
                        let (s, e) = expand(child_arc, depth + 1, max_depth, Some(id_str.clone()), seen, elements).await?;
                        seen = s; elements = e;
                    }
                }
            }
            Ok((seen, elements))
        })
    }

    for o in category.get_all_objects().await? {
        let arc = o.clone();
        let (s, e) = expand(arc, 1, max_depth, None, seen_nodes, elements).await?;
        seen_nodes = s;
        elements = e;
    }

    // Top-level edges
    let mut seen_edges: HashSet<String> = HashSet::new();
    for o in category.get_all_objects().await? {
        for m in category.get_object_morphisms(&**o).await? {
            let id = m.arrow_id().to_string();
            if !seen_edges.insert(id.clone()) { continue; }
            let src = normalized_object_id(m.source_object());
            let tgt = normalized_object_id(m.target_object());
            let is_identity = m.is_identity();
            let has_functor = m.functor().is_some();
            let label = if is_identity { "id" } else { m.arrow_id() };
            elements.push(serde_json::to_value(CytoscapeElement::edge(&id, &src, &tgt, label, has_functor, is_identity)).unwrap());
        }
    }

    Ok(elements)
}

/// Convenience helper: returns a pretty JSON string for direct embedding in HTML/JS.
pub async fn category_to_cytoscape_json<C>(category: &C, nested: bool, max_depth: usize) -> Result<String, Errors>
where
    C: CategoryTrait + Sync,
    C::Object: CategoryTrait + Sync,
    C::Morphism: ArrowTrait<C::Object, C::Object> + Sync,
{
    let elems = if nested { nested_category_to_cytoscape_elements(category, max_depth).await? } else { category_to_cytoscape_elements(category).await? };
    Ok(serde_json::to_string_pretty(&elems).unwrap())
}

/// Save the category as Cytoscape JSON to the specified file path.
/// Creates parent directories if they do not exist.
pub async fn save_category_to_cytoscape_json_file<C, P>(
    category: &C,
    nested: bool,
    max_depth: usize,
    path: P,
) -> Result<(), Errors>
where
    C: CategoryTrait + Sync,
    C::Object: CategoryTrait + Sync,
    C::Morphism: ArrowTrait<C::Object, C::Object> + Sync,
    P: AsRef<std::path::Path>,
{
    let json_str = category_to_cytoscape_json(category, nested, max_depth).await?;
    let path_ref = path.as_ref();
    if let Some(parent) = path_ref.parent() {
        if !parent.exists() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| Errors::InvalidOperation(format!("Failed to create directories: {}", e)))?;
        }
    }
    tokio::fs::write(path_ref, json_str)
        .await
        .map_err(|e| Errors::InvalidOperation(format!("Failed to write file: {}", e)))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::dynamic_category::DynamicCategory;
    use crate::core::traits::category_trait::{CategoryFromObjects, CategoryTrait};
    use crate::core::arrow::Morphism;
    use std::collections::HashMap;

    // Build a small dynamic category with two objects (each itself a category with 3 inner objects)
    // and two non-identity morphisms between them. Then verify element counts.
    #[tokio::test]
    async fn test_flat_export_dynamic_category() {
        let mut cat = DynamicCategory::new();
        let obj_letters = Arc::new(DynamicCategory::from_objects(vec!["a", "b", "c"]).await.unwrap());
        let obj_numbers = Arc::new(DynamicCategory::from_objects(vec![1, 2, 3]).await.unwrap());
        cat.add_object(obj_letters.clone()).await.unwrap();
        cat.add_object(obj_numbers.clone()).await.unwrap();

        // Add two different functor-based morphisms between letters -> numbers
        let mapping1 = HashMap::from([
            (obj_letters.get_identity_morphism(&<&str as Into<DynamicCategory>>::into("a")).await.unwrap().clone(), obj_numbers.get_identity_morphism(&DynamicCategory::from(1)).await.unwrap().clone()),
            (obj_letters.get_identity_morphism(&<&str as Into<DynamicCategory>>::into("b")).await.unwrap().clone(), obj_numbers.get_identity_morphism(&DynamicCategory::from(2)).await.unwrap().clone()),
            (obj_letters.get_identity_morphism(&<&str as Into<DynamicCategory>>::into("c")).await.unwrap().clone(), obj_numbers.get_identity_morphism(&DynamicCategory::from(3)).await.unwrap().clone()),
        ]);
        let mapping2 = HashMap::from([
            (obj_letters.get_identity_morphism(&DynamicCategory::from("a".to_string())).await.unwrap().clone(), obj_numbers.get_identity_morphism(&DynamicCategory::from(3)).await.unwrap().clone()),
            (obj_letters.get_identity_morphism(&DynamicCategory::from("b".to_string())).await.unwrap().clone(), obj_numbers.get_identity_morphism(&DynamicCategory::from(2)).await.unwrap().clone()),
            (obj_letters.get_identity_morphism(&DynamicCategory::from("c".to_string())).await.unwrap().clone(), obj_numbers.get_identity_morphism(&DynamicCategory::from(1)).await.unwrap().clone()),
        ]);

        let morphism1 = Arc::new(Morphism::new_with_mappings(obj_letters.clone(), obj_numbers.clone(), mapping1));
        let morphism2 = Arc::new(Morphism::new_with_mappings(obj_letters.clone(), obj_numbers.clone(), mapping2));
        cat.add_morphism(morphism1).await.unwrap();
        cat.add_morphism(morphism2).await.unwrap();

        let elems = category_to_cytoscape_elements(&cat).await.unwrap();
        // Expect 2 object nodes + (at least) 2 identity morphisms + 2 non-identity morphisms => edges >= 4.
        // But since identity morphisms are represented as edges (one per object) and we have two objects, plus 2 added morphisms = 4 edges total.
        let node_count = elems.iter().filter(|v| v.get("data").and_then(|d| d.get("source")).is_none()).count();
        let edge_count = elems.iter().filter(|v| v.get("data").and_then(|d| d.get("source")).is_some()).count();
        assert_eq!(node_count, 2, "Expected exactly two top-level object nodes");
        assert_eq!(edge_count, 4, "Expected two identity + two added morphism edges");
    }

    #[tokio::test]
    async fn test_nested_export_dynamic_category() {
        let mut cat = DynamicCategory::new();
        let obj_letters = Arc::new(DynamicCategory::from_objects(vec!["a", "b", "c"]).await.unwrap());
        cat.add_object(obj_letters.clone()).await.unwrap();

        let elems = nested_category_to_cytoscape_elements(&cat, 3).await.unwrap();
        // We expect 1 parent (letters) + 3 children + 1 identity edge = 5 elements (nodes) + edges (1 identity) = 5? Actually element vector mixes both.
        let node_ids: HashSet<String> = elems.iter().filter_map(|v| v.get("data").and_then(|d| d.get("id")).and_then(|id| id.as_str()).map(|s| s.to_string())).collect();
        assert!(node_ids.contains(&obj_letters.category_id().to_string()));
        assert!(node_ids.len() >= 4, "Should contain parent + at least 3 inner objects");
    }

    #[tokio::test]
    async fn test_empty_id_normalization() {
        // Create an object with an empty string id and ensure it normalizes.
        use crate::core::dynamic_category::DynamicCategory;
        use crate::core::object_id::ObjectId;
        let mut top = DynamicCategory::new();
        let empty_obj = Arc::new(DynamicCategory::new_with_id(ObjectId::Str("".into())));
        top.add_object(empty_obj.clone()).await.unwrap();
        let json = category_to_cytoscape_json(&top, false, 1).await.unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        let arr = v.as_array().unwrap();
        assert!(arr.iter().any(|el| el.get("data").and_then(|d| d.get("id")).and_then(|id| id.as_str()) == Some(EMPTY_ID_SENTINEL)));
        assert!(arr.iter().any(|el| el.get("data").and_then(|d| d.get("label")).and_then(|id| id.as_str()) == Some(EMPTY_ID_SENTINEL)));
    }
}

#[cfg(test)]
mod file_tests {
    use super::*;
    use crate::core::dynamic_category::DynamicCategory;
    use crate::core::traits::category_trait::CategoryFromObjects;
    use std::fs;

    #[tokio::test]
    async fn test_save_category_to_file() {
        let cat = DynamicCategory::from_objects(vec!["x", "y"]).await.unwrap();
        let tmp_dir = std::env::temp_dir();
        let file_path = tmp_dir.join(format!("cytoscape_export_{}.json", uuid::Uuid::new_v4()));
        save_category_to_cytoscape_json_file(&cat, false, 2, &file_path)
            .await
            .expect("save should succeed");
        let content = fs::read_to_string(&file_path).expect("file should exist");
        let parsed: serde_json::Value = serde_json::from_str(&content).expect("valid json");
        assert!(parsed.is_array(), "Export must be a JSON array");
        // Clean up (best effort)
        let _ = fs::remove_file(&file_path);
    }
}
