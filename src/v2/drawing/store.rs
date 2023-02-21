//! v2::drawing Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::drawing-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`Anchor`]
//! * [`AssociativeUi`]
//! * [`BinaryUi`]
//! * [`Edge`]
//! * [`IsaUi`]
//! * [`ObjectEdge`]
//! * [`ObjectUi`]
//! * [`Point`]
//! * [`RelationshipUi`]
//! * [`SubtypeAnchors`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::drawing-object-store-definition"}}}
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v2::drawing::types::{
    Anchor, AssociativeUi, BinaryUi, Edge, IsaUi, ObjectEdge, ObjectUi, Point, RelationshipUi,
    SubtypeAnchors,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    anchor: HashMap<Uuid, Anchor>,
    associative_ui: HashMap<Uuid, AssociativeUi>,
    binary_ui: HashMap<Uuid, BinaryUi>,
    edge: HashMap<Uuid, Edge>,
    isa_ui: HashMap<Uuid, IsaUi>,
    object_edge: HashMap<Uuid, ObjectEdge>,
    object_ui: HashMap<Uuid, ObjectUi>,
    point: HashMap<Uuid, Point>,
    relationship_ui: HashMap<Uuid, RelationshipUi>,
    subtype_anchors: HashMap<Uuid, SubtypeAnchors>,
}

impl ObjectStore {
    pub fn new() -> Self {
        Self {
            anchor: HashMap::new(),
            associative_ui: HashMap::new(),
            binary_ui: HashMap::new(),
            edge: HashMap::new(),
            isa_ui: HashMap::new(),
            object_edge: HashMap::new(),
            object_ui: HashMap::new(),
            point: HashMap::new(),
            relationship_ui: HashMap::new(),
            subtype_anchors: HashMap::new(),
        }
    }

    /// Inter [`Anchor`] into the store.
    ///
    pub fn inter_anchor(&mut self, anchor: Anchor) {
        self.anchor.insert(anchor.id, anchor);
    }

    /// Exhume [`Anchor`] from the store.
    ///
    pub fn exhume_anchor(&self, id: &Uuid) -> Option<&Anchor> {
        self.anchor.get(id)
    }
    /// Exhume [`Anchor`] from the store — mutably.
    ///
    pub fn exhume_anchor_mut(&mut self, id: &Uuid) -> Option<&mut Anchor> {
        self.anchor.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Anchor>`.
    ///
    pub fn iter_anchor(&self) -> impl Iterator<Item = (&Uuid, &Anchor)> {
        self.anchor.iter()
    }
    /// Inter [`AssociativeUi`] into the store.
    ///
    pub fn inter_associative_ui(&mut self, associative_ui: AssociativeUi) {
        self.associative_ui
            .insert(associative_ui.id, associative_ui);
    }

    /// Exhume [`AssociativeUi`] from the store.
    ///
    pub fn exhume_associative_ui(&self, id: &Uuid) -> Option<&AssociativeUi> {
        self.associative_ui.get(id)
    }
    /// Exhume [`AssociativeUi`] from the store — mutably.
    ///
    pub fn exhume_associative_ui_mut(&mut self, id: &Uuid) -> Option<&mut AssociativeUi> {
        self.associative_ui.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, AssociativeUi>`.
    ///
    pub fn iter_associative_ui(&self) -> impl Iterator<Item = (&Uuid, &AssociativeUi)> {
        self.associative_ui.iter()
    }
    /// Inter [`BinaryUi`] into the store.
    ///
    pub fn inter_binary_ui(&mut self, binary_ui: BinaryUi) {
        self.binary_ui.insert(binary_ui.id, binary_ui);
    }

    /// Exhume [`BinaryUi`] from the store.
    ///
    pub fn exhume_binary_ui(&self, id: &Uuid) -> Option<&BinaryUi> {
        self.binary_ui.get(id)
    }
    /// Exhume [`BinaryUi`] from the store — mutably.
    ///
    pub fn exhume_binary_ui_mut(&mut self, id: &Uuid) -> Option<&mut BinaryUi> {
        self.binary_ui.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, BinaryUi>`.
    ///
    pub fn iter_binary_ui(&self) -> impl Iterator<Item = (&Uuid, &BinaryUi)> {
        self.binary_ui.iter()
    }
    /// Inter [`Edge`] into the store.
    ///
    pub fn inter_edge(&mut self, edge: Edge) {
        self.edge.insert(edge.id(), edge);
    }

    /// Exhume [`Edge`] from the store.
    ///
    pub fn exhume_edge(&self, id: &Uuid) -> Option<&Edge> {
        self.edge.get(id)
    }
    /// Exhume [`Edge`] from the store — mutably.
    ///
    pub fn exhume_edge_mut(&mut self, id: &Uuid) -> Option<&mut Edge> {
        self.edge.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Edge>`.
    ///
    pub fn iter_edge(&self) -> impl Iterator<Item = (&Uuid, &Edge)> {
        self.edge.iter()
    }
    /// Inter [`IsaUi`] into the store.
    ///
    pub fn inter_isa_ui(&mut self, isa_ui: IsaUi) {
        self.isa_ui.insert(isa_ui.id, isa_ui);
    }

    /// Exhume [`IsaUi`] from the store.
    ///
    pub fn exhume_isa_ui(&self, id: &Uuid) -> Option<&IsaUi> {
        self.isa_ui.get(id)
    }
    /// Exhume [`IsaUi`] from the store — mutably.
    ///
    pub fn exhume_isa_ui_mut(&mut self, id: &Uuid) -> Option<&mut IsaUi> {
        self.isa_ui.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, IsaUi>`.
    ///
    pub fn iter_isa_ui(&self) -> impl Iterator<Item = (&Uuid, &IsaUi)> {
        self.isa_ui.iter()
    }
    /// Inter [`ObjectEdge`] into the store.
    ///
    pub fn inter_object_edge(&mut self, object_edge: ObjectEdge) {
        self.object_edge.insert(object_edge.id, object_edge);
    }

    /// Exhume [`ObjectEdge`] from the store.
    ///
    pub fn exhume_object_edge(&self, id: &Uuid) -> Option<&ObjectEdge> {
        self.object_edge.get(id)
    }
    /// Exhume [`ObjectEdge`] from the store — mutably.
    ///
    pub fn exhume_object_edge_mut(&mut self, id: &Uuid) -> Option<&mut ObjectEdge> {
        self.object_edge.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, ObjectEdge>`.
    ///
    pub fn iter_object_edge(&self) -> impl Iterator<Item = (&Uuid, &ObjectEdge)> {
        self.object_edge.iter()
    }
    /// Inter [`ObjectUi`] into the store.
    ///
    pub fn inter_object_ui(&mut self, object_ui: ObjectUi) {
        self.object_ui.insert(object_ui.id, object_ui);
    }

    /// Exhume [`ObjectUi`] from the store.
    ///
    pub fn exhume_object_ui(&self, id: &Uuid) -> Option<&ObjectUi> {
        self.object_ui.get(id)
    }
    /// Exhume [`ObjectUi`] from the store — mutably.
    ///
    pub fn exhume_object_ui_mut(&mut self, id: &Uuid) -> Option<&mut ObjectUi> {
        self.object_ui.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, ObjectUi>`.
    ///
    pub fn iter_object_ui(&self) -> impl Iterator<Item = (&Uuid, &ObjectUi)> {
        self.object_ui.iter()
    }
    /// Inter [`Point`] into the store.
    ///
    pub fn inter_point(&mut self, point: Point) {
        self.point.insert(point.id, point);
    }

    /// Exhume [`Point`] from the store.
    ///
    pub fn exhume_point(&self, id: &Uuid) -> Option<&Point> {
        self.point.get(id)
    }
    /// Exhume [`Point`] from the store — mutably.
    ///
    pub fn exhume_point_mut(&mut self, id: &Uuid) -> Option<&mut Point> {
        self.point.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Point>`.
    ///
    pub fn iter_point(&self) -> impl Iterator<Item = (&Uuid, &Point)> {
        self.point.iter()
    }
    /// Inter [`RelationshipUi`] into the store.
    ///
    pub fn inter_relationship_ui(&mut self, relationship_ui: RelationshipUi) {
        self.relationship_ui
            .insert(relationship_ui.id(), relationship_ui);
    }

    /// Exhume [`RelationshipUi`] from the store.
    ///
    pub fn exhume_relationship_ui(&self, id: &Uuid) -> Option<&RelationshipUi> {
        self.relationship_ui.get(id)
    }
    /// Exhume [`RelationshipUi`] from the store — mutably.
    ///
    pub fn exhume_relationship_ui_mut(&mut self, id: &Uuid) -> Option<&mut RelationshipUi> {
        self.relationship_ui.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, RelationshipUi>`.
    ///
    pub fn iter_relationship_ui(&self) -> impl Iterator<Item = (&Uuid, &RelationshipUi)> {
        self.relationship_ui.iter()
    }
    /// Inter [`SubtypeAnchors`] into the store.
    ///
    pub fn inter_subtype_anchors(&mut self, subtype_anchors: SubtypeAnchors) {
        self.subtype_anchors
            .insert(subtype_anchors.id, subtype_anchors);
    }

    /// Exhume [`SubtypeAnchors`] from the store.
    ///
    pub fn exhume_subtype_anchors(&self, id: &Uuid) -> Option<&SubtypeAnchors> {
        self.subtype_anchors.get(id)
    }
    /// Exhume [`SubtypeAnchors`] from the store — mutably.
    ///
    pub fn exhume_subtype_anchors_mut(&mut self, id: &Uuid) -> Option<&mut SubtypeAnchors> {
        self.subtype_anchors.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, SubtypeAnchors>`.
    ///
    pub fn iter_subtype_anchors(&self) -> impl Iterator<Item = (&Uuid, &SubtypeAnchors)> {
        self.subtype_anchors.iter()
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
