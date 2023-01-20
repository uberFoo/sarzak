//! {"magic":"","version":"0.0.1"}
//! ObjectStore for the instances of the "Drawing" domain
//!
//! An end user should have little need to use this directly.
//!
//! This store contains the following instances:
//!    * [`Anchor`]
//!    * [`BinaryUi`]
//!    * [`Point`]
//!    * [`ObjectEdge`]
//!    * [`Edge`]
//!    * [`RelationshipUi`]
//!    * [`ObjectUi`]
//!    * [`IsaUi`]
//!    * [`AssociativeUi`]
//!    * [`SubtypeAnchors`]
//!
//! Generated Code -- edit _carefully_.
//! Don't mess with anything between {"magic":"","kind":"CriticalBlockBegin"}
//! and {"magic":"","kind":"CriticalBlockEnd"}. Otherwise, you should be free
//! to go wild. Happy hacking!
//! Use the following invocation to reproduce:
//! ```shell
//!  sarzak gen
//! ```
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::drawing::types::{Anchor, BinaryUi, Point, ObjectEdge, Edge, RelationshipUi, ObjectUi, IsaUi, AssociativeUi, SubtypeAnchors, };

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    anchor: HashMap<Uuid, Anchor>,
    binary_ui: HashMap<Uuid, BinaryUi>,
    point: HashMap<Uuid, Point>,
    object_edge: HashMap<Uuid, ObjectEdge>,
    edge: HashMap<Uuid, Edge>,
    relationship_ui: HashMap<Uuid, RelationshipUi>,
    object_ui: HashMap<Uuid, ObjectUi>,
    isa_ui: HashMap<Uuid, IsaUi>,
    associative_ui: HashMap<Uuid, AssociativeUi>,
    subtype_anchors: HashMap<Uuid, SubtypeAnchors>,
}

impl ObjectStore {
    pub fn new() -> Self {
        Self {
            anchor: HashMap::new(),
            binary_ui: HashMap::new(),
            point: HashMap::new(),
            object_edge: HashMap::new(),
            edge: HashMap::new(),
            relationship_ui: HashMap::new(),
            object_ui: HashMap::new(),
            isa_ui: HashMap::new(),
            associative_ui: HashMap::new(),
            subtype_anchors: HashMap::new(),
        }
    }

    /// Inter [`Anchor`] into the [`ObjectStore`]
    ///
    pub fn inter_anchor(&mut self, anchor: Anchor) {
        self.anchor.insert(anchor.id, anchor);
    }

    /// Exhume [`Anchor`] from the [`ObjectStore`]
    ///
    pub fn exhume_anchor(&self, id: &Uuid) -> Option<&Anchor> {
        self.anchor.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, Anchor)>` in the [`ObjectStore`]
    ///
    pub fn iter_anchor(&self) -> impl Iterator<Item = (&Uuid, &Anchor)> {
        self.anchor.iter()
    }

    /// Inter [`BinaryUi`] into the [`ObjectStore`]
    ///
    pub fn inter_binary_ui(&mut self, binary_ui: BinaryUi) {
        self.binary_ui.insert(binary_ui.id, binary_ui);
    }

    /// Exhume [`BinaryUI`] from the [`ObjectStore`]
    ///
    pub fn exhume_binary_ui(&self, id: &Uuid) -> Option<&BinaryUi> {
        self.binary_ui.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, BinaryUi)>` in the [`ObjectStore`]
    ///
    pub fn iter_binary_ui(&self) -> impl Iterator<Item = (&Uuid, &BinaryUi)> {
        self.binary_ui.iter()
    }

    /// Inter [`Point`] into the [`ObjectStore`]
    ///
    pub fn inter_point(&mut self, point: Point) {
        self.point.insert(point.id, point);
    }

    /// Exhume [`Point`] from the [`ObjectStore`]
    ///
    pub fn exhume_point(&self, id: &Uuid) -> Option<&Point> {
        self.point.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, Point)>` in the [`ObjectStore`]
    ///
    pub fn iter_point(&self) -> impl Iterator<Item = (&Uuid, &Point)> {
        self.point.iter()
    }

    /// Inter [`ObjectEdge`] into the [`ObjectStore`]
    ///
    pub fn inter_object_edge(&mut self, object_edge: ObjectEdge) {
        self.object_edge.insert(object_edge.id, object_edge);
    }

    /// Exhume [`Object Edge`] from the [`ObjectStore`]
    ///
    pub fn exhume_object_edge(&self, id: &Uuid) -> Option<&ObjectEdge> {
        self.object_edge.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, ObjectEdge)>` in the [`ObjectStore`]
    ///
    pub fn iter_object_edge(&self) -> impl Iterator<Item = (&Uuid, &ObjectEdge)> {
        self.object_edge.iter()
    }

    /// Inter [`Edge`] into the [`ObjectStore`]
    ///
    pub fn inter_edge(&mut self, edge: Edge) {
        self.edge.insert(edge.get_id(), edge);
    }

    /// Exhume [`Edge`] from the [`ObjectStore`]
    ///
    pub fn exhume_edge(&self, id: &Uuid) -> Option<&Edge> {
        self.edge.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, Edge)>` in the [`ObjectStore`]
    ///
    pub fn iter_edge(&self) -> impl Iterator<Item = (&Uuid, &Edge)> {
        self.edge.iter()
    }

    /// Inter [`RelationshipUi`] into the [`ObjectStore`]
    ///
    pub fn inter_relationship_ui(&mut self, relationship_ui: RelationshipUi) {
        self.relationship_ui.insert(relationship_ui.get_id(), relationship_ui);
    }

    /// Exhume [`RelationshipUI`] from the [`ObjectStore`]
    ///
    pub fn exhume_relationship_ui(&self, id: &Uuid) -> Option<&RelationshipUi> {
        self.relationship_ui.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, RelationshipUi)>` in the [`ObjectStore`]
    ///
    pub fn iter_relationship_ui(&self) -> impl Iterator<Item = (&Uuid, &RelationshipUi)> {
        self.relationship_ui.iter()
    }

    /// Inter [`ObjectUi`] into the [`ObjectStore`]
    ///
    pub fn inter_object_ui(&mut self, object_ui: ObjectUi) {
        self.object_ui.insert(object_ui.id, object_ui);
    }

    /// Exhume [`ObjectUI`] from the [`ObjectStore`]
    ///
    pub fn exhume_object_ui(&self, id: &Uuid) -> Option<&ObjectUi> {
        self.object_ui.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, ObjectUi)>` in the [`ObjectStore`]
    ///
    pub fn iter_object_ui(&self) -> impl Iterator<Item = (&Uuid, &ObjectUi)> {
        self.object_ui.iter()
    }

    /// Inter [`IsaUi`] into the [`ObjectStore`]
    ///
    pub fn inter_isa_ui(&mut self, isa_ui: IsaUi) {
        self.isa_ui.insert(isa_ui.id, isa_ui);
    }

    /// Exhume [`IsaUI`] from the [`ObjectStore`]
    ///
    pub fn exhume_isa_ui(&self, id: &Uuid) -> Option<&IsaUi> {
        self.isa_ui.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, IsaUi)>` in the [`ObjectStore`]
    ///
    pub fn iter_isa_ui(&self) -> impl Iterator<Item = (&Uuid, &IsaUi)> {
        self.isa_ui.iter()
    }

    /// Inter [`AssociativeUi`] into the [`ObjectStore`]
    ///
    pub fn inter_associative_ui(&mut self, associative_ui: AssociativeUi) {
        self.associative_ui.insert(associative_ui.id, associative_ui);
    }

    /// Exhume [`AssociativeUI`] from the [`ObjectStore`]
    ///
    pub fn exhume_associative_ui(&self, id: &Uuid) -> Option<&AssociativeUi> {
        self.associative_ui.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, AssociativeUi)>` in the [`ObjectStore`]
    ///
    pub fn iter_associative_ui(&self) -> impl Iterator<Item = (&Uuid, &AssociativeUi)> {
        self.associative_ui.iter()
    }

    /// Inter [`SubtypeAnchors`] into the [`ObjectStore`]
    ///
    pub fn inter_subtype_anchors(&mut self, subtype_anchors: SubtypeAnchors) {
        self.subtype_anchors.insert(subtype_anchors.id, subtype_anchors);
    }

    /// Exhume [`Subtype Anchors`] from the [`ObjectStore`]
    ///
    pub fn exhume_subtype_anchors(&self, id: &Uuid) -> Option<&SubtypeAnchors> {
        self.subtype_anchors.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, SubtypeAnchors)>` in the [`ObjectStore`]
    ///
    pub fn iter_subtype_anchors(&self) -> impl Iterator<Item = (&Uuid, &SubtypeAnchors)> {
        self.subtype_anchors.iter()
    }

}
