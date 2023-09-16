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
use std::sync::Arc;
use std::sync::RwLock;
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
    time::SystemTime,
};

use rustc_hash::FxHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v2::drawing::types::{
    Anchor, AssociativeUi, BinaryUi, Edge, IsaUi, ObjectEdge, ObjectUi, Point, RelationshipUi,
    SubtypeAnchors, BOTTOM, LEFT, RIGHT, TOP,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    anchor: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Anchor>>, SystemTime)>>>,
    associative_ui: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<AssociativeUi>>, SystemTime)>>>,
    binary_ui: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<BinaryUi>>, SystemTime)>>>,
    edge: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Edge>>, SystemTime)>>>,
    isa_ui: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<IsaUi>>, SystemTime)>>>,
    object_edge: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<ObjectEdge>>, SystemTime)>>>,
    object_ui: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<ObjectUi>>, SystemTime)>>>,
    point: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Point>>, SystemTime)>>>,
    relationship_ui: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<RelationshipUi>>, SystemTime)>>>,
    subtype_anchors: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<SubtypeAnchors>>, SystemTime)>>>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            anchor: Arc::new(RwLock::new(HashMap::default())),
            associative_ui: Arc::new(RwLock::new(HashMap::default())),
            binary_ui: Arc::new(RwLock::new(HashMap::default())),
            edge: Arc::new(RwLock::new(HashMap::default())),
            isa_ui: Arc::new(RwLock::new(HashMap::default())),
            object_edge: Arc::new(RwLock::new(HashMap::default())),
            object_ui: Arc::new(RwLock::new(HashMap::default())),
            point: Arc::new(RwLock::new(HashMap::default())),
            relationship_ui: Arc::new(RwLock::new(HashMap::default())),
            subtype_anchors: Arc::new(RwLock::new(HashMap::default())),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥
        store.inter_edge(Arc::new(RwLock::new(Edge::Bottom(BOTTOM))));
        store.inter_edge(Arc::new(RwLock::new(Edge::Left(LEFT))));
        store.inter_edge(Arc::new(RwLock::new(Edge::Right(RIGHT))));
        store.inter_edge(Arc::new(RwLock::new(Edge::Top(TOP))));

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::drawing-object-store-methods"}}}
    /// Inter (insert) [`Anchor`] into the store.
    ///
    pub fn inter_anchor(&mut self, anchor: Arc<RwLock<Anchor>>) {
        let read = anchor.read().unwrap();
        self.anchor
            .write()
            .unwrap()
            .insert(read.id, (anchor.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Anchor`] from the store.
    ///
    pub fn exhume_anchor(&self, id: &Uuid) -> Option<Arc<RwLock<Anchor>>> {
        self.anchor
            .read()
            .unwrap()
            .get(id)
            .map(|anchor| anchor.0.clone())
    }

    /// Exorcise (remove) [`Anchor`] from the store.
    ///
    pub fn exorcise_anchor(&mut self, id: &Uuid) -> Option<Arc<RwLock<Anchor>>> {
        self.anchor
            .write()
            .unwrap()
            .remove(id)
            .map(|anchor| anchor.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Anchor>`.
    ///
    pub fn iter_anchor(&self) -> impl Iterator<Item = Arc<RwLock<Anchor>>> + '_ {
        let values: Vec<Arc<RwLock<Anchor>>> = self
            .anchor
            .read()
            .unwrap()
            .values()
            .map(|anchor| anchor.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Anchor.
    ///
    pub fn anchor_timestamp(&self, anchor: &Anchor) -> SystemTime {
        self.anchor
            .read()
            .unwrap()
            .get(&anchor.id)
            .map(|anchor| anchor.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`AssociativeUi`] into the store.
    ///
    pub fn inter_associative_ui(&mut self, associative_ui: Arc<RwLock<AssociativeUi>>) {
        let read = associative_ui.read().unwrap();
        self.associative_ui
            .write()
            .unwrap()
            .insert(read.id, (associative_ui.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`AssociativeUi`] from the store.
    ///
    pub fn exhume_associative_ui(&self, id: &Uuid) -> Option<Arc<RwLock<AssociativeUi>>> {
        self.associative_ui
            .read()
            .unwrap()
            .get(id)
            .map(|associative_ui| associative_ui.0.clone())
    }

    /// Exorcise (remove) [`AssociativeUi`] from the store.
    ///
    pub fn exorcise_associative_ui(&mut self, id: &Uuid) -> Option<Arc<RwLock<AssociativeUi>>> {
        self.associative_ui
            .write()
            .unwrap()
            .remove(id)
            .map(|associative_ui| associative_ui.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, AssociativeUi>`.
    ///
    pub fn iter_associative_ui(&self) -> impl Iterator<Item = Arc<RwLock<AssociativeUi>>> + '_ {
        let values: Vec<Arc<RwLock<AssociativeUi>>> = self
            .associative_ui
            .read()
            .unwrap()
            .values()
            .map(|associative_ui| associative_ui.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for AssociativeUi.
    ///
    pub fn associative_ui_timestamp(&self, associative_ui: &AssociativeUi) -> SystemTime {
        self.associative_ui
            .read()
            .unwrap()
            .get(&associative_ui.id)
            .map(|associative_ui| associative_ui.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`BinaryUi`] into the store.
    ///
    pub fn inter_binary_ui(&mut self, binary_ui: Arc<RwLock<BinaryUi>>) {
        let read = binary_ui.read().unwrap();
        self.binary_ui
            .write()
            .unwrap()
            .insert(read.id, (binary_ui.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`BinaryUi`] from the store.
    ///
    pub fn exhume_binary_ui(&self, id: &Uuid) -> Option<Arc<RwLock<BinaryUi>>> {
        self.binary_ui
            .read()
            .unwrap()
            .get(id)
            .map(|binary_ui| binary_ui.0.clone())
    }

    /// Exorcise (remove) [`BinaryUi`] from the store.
    ///
    pub fn exorcise_binary_ui(&mut self, id: &Uuid) -> Option<Arc<RwLock<BinaryUi>>> {
        self.binary_ui
            .write()
            .unwrap()
            .remove(id)
            .map(|binary_ui| binary_ui.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BinaryUi>`.
    ///
    pub fn iter_binary_ui(&self) -> impl Iterator<Item = Arc<RwLock<BinaryUi>>> + '_ {
        let values: Vec<Arc<RwLock<BinaryUi>>> = self
            .binary_ui
            .read()
            .unwrap()
            .values()
            .map(|binary_ui| binary_ui.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for BinaryUi.
    ///
    pub fn binary_ui_timestamp(&self, binary_ui: &BinaryUi) -> SystemTime {
        self.binary_ui
            .read()
            .unwrap()
            .get(&binary_ui.id)
            .map(|binary_ui| binary_ui.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Edge`] into the store.
    ///
    pub fn inter_edge(&mut self, edge: Arc<RwLock<Edge>>) {
        let read = edge.read().unwrap();
        self.edge
            .write()
            .unwrap()
            .insert(read.id(), (edge.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Edge`] from the store.
    ///
    pub fn exhume_edge(&self, id: &Uuid) -> Option<Arc<RwLock<Edge>>> {
        self.edge.read().unwrap().get(id).map(|edge| edge.0.clone())
    }

    /// Exorcise (remove) [`Edge`] from the store.
    ///
    pub fn exorcise_edge(&mut self, id: &Uuid) -> Option<Arc<RwLock<Edge>>> {
        self.edge
            .write()
            .unwrap()
            .remove(id)
            .map(|edge| edge.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Edge>`.
    ///
    pub fn iter_edge(&self) -> impl Iterator<Item = Arc<RwLock<Edge>>> + '_ {
        let values: Vec<Arc<RwLock<Edge>>> = self
            .edge
            .read()
            .unwrap()
            .values()
            .map(|edge| edge.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Edge.
    ///
    pub fn edge_timestamp(&self, edge: &Edge) -> SystemTime {
        self.edge
            .read()
            .unwrap()
            .get(&edge.id())
            .map(|edge| edge.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`IsaUi`] into the store.
    ///
    pub fn inter_isa_ui(&mut self, isa_ui: Arc<RwLock<IsaUi>>) {
        let read = isa_ui.read().unwrap();
        self.isa_ui
            .write()
            .unwrap()
            .insert(read.id, (isa_ui.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`IsaUi`] from the store.
    ///
    pub fn exhume_isa_ui(&self, id: &Uuid) -> Option<Arc<RwLock<IsaUi>>> {
        self.isa_ui
            .read()
            .unwrap()
            .get(id)
            .map(|isa_ui| isa_ui.0.clone())
    }

    /// Exorcise (remove) [`IsaUi`] from the store.
    ///
    pub fn exorcise_isa_ui(&mut self, id: &Uuid) -> Option<Arc<RwLock<IsaUi>>> {
        self.isa_ui
            .write()
            .unwrap()
            .remove(id)
            .map(|isa_ui| isa_ui.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, IsaUi>`.
    ///
    pub fn iter_isa_ui(&self) -> impl Iterator<Item = Arc<RwLock<IsaUi>>> + '_ {
        let values: Vec<Arc<RwLock<IsaUi>>> = self
            .isa_ui
            .read()
            .unwrap()
            .values()
            .map(|isa_ui| isa_ui.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for IsaUi.
    ///
    pub fn isa_ui_timestamp(&self, isa_ui: &IsaUi) -> SystemTime {
        self.isa_ui
            .read()
            .unwrap()
            .get(&isa_ui.id)
            .map(|isa_ui| isa_ui.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ObjectEdge`] into the store.
    ///
    pub fn inter_object_edge(&mut self, object_edge: Arc<RwLock<ObjectEdge>>) {
        let read = object_edge.read().unwrap();
        self.object_edge
            .write()
            .unwrap()
            .insert(read.id, (object_edge.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ObjectEdge`] from the store.
    ///
    pub fn exhume_object_edge(&self, id: &Uuid) -> Option<Arc<RwLock<ObjectEdge>>> {
        self.object_edge
            .read()
            .unwrap()
            .get(id)
            .map(|object_edge| object_edge.0.clone())
    }

    /// Exorcise (remove) [`ObjectEdge`] from the store.
    ///
    pub fn exorcise_object_edge(&mut self, id: &Uuid) -> Option<Arc<RwLock<ObjectEdge>>> {
        self.object_edge
            .write()
            .unwrap()
            .remove(id)
            .map(|object_edge| object_edge.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ObjectEdge>`.
    ///
    pub fn iter_object_edge(&self) -> impl Iterator<Item = Arc<RwLock<ObjectEdge>>> + '_ {
        let values: Vec<Arc<RwLock<ObjectEdge>>> = self
            .object_edge
            .read()
            .unwrap()
            .values()
            .map(|object_edge| object_edge.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for ObjectEdge.
    ///
    pub fn object_edge_timestamp(&self, object_edge: &ObjectEdge) -> SystemTime {
        self.object_edge
            .read()
            .unwrap()
            .get(&object_edge.id)
            .map(|object_edge| object_edge.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ObjectUi`] into the store.
    ///
    pub fn inter_object_ui(&mut self, object_ui: Arc<RwLock<ObjectUi>>) {
        let read = object_ui.read().unwrap();
        self.object_ui
            .write()
            .unwrap()
            .insert(read.id, (object_ui.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ObjectUi`] from the store.
    ///
    pub fn exhume_object_ui(&self, id: &Uuid) -> Option<Arc<RwLock<ObjectUi>>> {
        self.object_ui
            .read()
            .unwrap()
            .get(id)
            .map(|object_ui| object_ui.0.clone())
    }

    /// Exorcise (remove) [`ObjectUi`] from the store.
    ///
    pub fn exorcise_object_ui(&mut self, id: &Uuid) -> Option<Arc<RwLock<ObjectUi>>> {
        self.object_ui
            .write()
            .unwrap()
            .remove(id)
            .map(|object_ui| object_ui.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ObjectUi>`.
    ///
    pub fn iter_object_ui(&self) -> impl Iterator<Item = Arc<RwLock<ObjectUi>>> + '_ {
        let values: Vec<Arc<RwLock<ObjectUi>>> = self
            .object_ui
            .read()
            .unwrap()
            .values()
            .map(|object_ui| object_ui.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for ObjectUi.
    ///
    pub fn object_ui_timestamp(&self, object_ui: &ObjectUi) -> SystemTime {
        self.object_ui
            .read()
            .unwrap()
            .get(&object_ui.id)
            .map(|object_ui| object_ui.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Point`] into the store.
    ///
    pub fn inter_point(&mut self, point: Arc<RwLock<Point>>) {
        let read = point.read().unwrap();
        self.point
            .write()
            .unwrap()
            .insert(read.id, (point.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Point`] from the store.
    ///
    pub fn exhume_point(&self, id: &Uuid) -> Option<Arc<RwLock<Point>>> {
        self.point
            .read()
            .unwrap()
            .get(id)
            .map(|point| point.0.clone())
    }

    /// Exorcise (remove) [`Point`] from the store.
    ///
    pub fn exorcise_point(&mut self, id: &Uuid) -> Option<Arc<RwLock<Point>>> {
        self.point
            .write()
            .unwrap()
            .remove(id)
            .map(|point| point.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Point>`.
    ///
    pub fn iter_point(&self) -> impl Iterator<Item = Arc<RwLock<Point>>> + '_ {
        let values: Vec<Arc<RwLock<Point>>> = self
            .point
            .read()
            .unwrap()
            .values()
            .map(|point| point.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Point.
    ///
    pub fn point_timestamp(&self, point: &Point) -> SystemTime {
        self.point
            .read()
            .unwrap()
            .get(&point.id)
            .map(|point| point.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`RelationshipUi`] into the store.
    ///
    pub fn inter_relationship_ui(&mut self, relationship_ui: Arc<RwLock<RelationshipUi>>) {
        let read = relationship_ui.read().unwrap();
        self.relationship_ui
            .write()
            .unwrap()
            .insert(read.id(), (relationship_ui.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`RelationshipUi`] from the store.
    ///
    pub fn exhume_relationship_ui(&self, id: &Uuid) -> Option<Arc<RwLock<RelationshipUi>>> {
        self.relationship_ui
            .read()
            .unwrap()
            .get(id)
            .map(|relationship_ui| relationship_ui.0.clone())
    }

    /// Exorcise (remove) [`RelationshipUi`] from the store.
    ///
    pub fn exorcise_relationship_ui(&mut self, id: &Uuid) -> Option<Arc<RwLock<RelationshipUi>>> {
        self.relationship_ui
            .write()
            .unwrap()
            .remove(id)
            .map(|relationship_ui| relationship_ui.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RelationshipUi>`.
    ///
    pub fn iter_relationship_ui(&self) -> impl Iterator<Item = Arc<RwLock<RelationshipUi>>> + '_ {
        let values: Vec<Arc<RwLock<RelationshipUi>>> = self
            .relationship_ui
            .read()
            .unwrap()
            .values()
            .map(|relationship_ui| relationship_ui.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for RelationshipUi.
    ///
    pub fn relationship_ui_timestamp(&self, relationship_ui: &RelationshipUi) -> SystemTime {
        self.relationship_ui
            .read()
            .unwrap()
            .get(&relationship_ui.id())
            .map(|relationship_ui| relationship_ui.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`SubtypeAnchors`] into the store.
    ///
    pub fn inter_subtype_anchors(&mut self, subtype_anchors: Arc<RwLock<SubtypeAnchors>>) {
        let read = subtype_anchors.read().unwrap();
        self.subtype_anchors
            .write()
            .unwrap()
            .insert(read.id, (subtype_anchors.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`SubtypeAnchors`] from the store.
    ///
    pub fn exhume_subtype_anchors(&self, id: &Uuid) -> Option<Arc<RwLock<SubtypeAnchors>>> {
        self.subtype_anchors
            .read()
            .unwrap()
            .get(id)
            .map(|subtype_anchors| subtype_anchors.0.clone())
    }

    /// Exorcise (remove) [`SubtypeAnchors`] from the store.
    ///
    pub fn exorcise_subtype_anchors(&mut self, id: &Uuid) -> Option<Arc<RwLock<SubtypeAnchors>>> {
        self.subtype_anchors
            .write()
            .unwrap()
            .remove(id)
            .map(|subtype_anchors| subtype_anchors.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SubtypeAnchors>`.
    ///
    pub fn iter_subtype_anchors(&self) -> impl Iterator<Item = Arc<RwLock<SubtypeAnchors>>> + '_ {
        let values: Vec<Arc<RwLock<SubtypeAnchors>>> = self
            .subtype_anchors
            .read()
            .unwrap()
            .values()
            .map(|subtype_anchors| subtype_anchors.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for SubtypeAnchors.
    ///
    pub fn subtype_anchors_timestamp(&self, subtype_anchors: &SubtypeAnchors) -> SystemTime {
        self.subtype_anchors
            .read()
            .unwrap()
            .get(&subtype_anchors.id)
            .map(|subtype_anchors| subtype_anchors.1)
            .unwrap_or(SystemTime::now())
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::drawing-object-store-persistence"}}}
    /// Persist the store.
    ///
    /// The store is persisted as a a bincode file.
    pub fn persist_bincode<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        let mut bin_file = fs::File::create(path)?;
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        bin_file.write_all(&encoded)?;
        Ok(())
    }

    /// Persist the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automagic git integration as an option.
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        fs::create_dir_all(path)?;

        let path = path.join("drawing.json");
        fs::create_dir_all(&path)?;

        // Persist Anchor.
        {
            let path = path.join("anchor");
            fs::create_dir_all(&path)?;
            for anchor_tuple in self.anchor.read().unwrap().values() {
                let path = path.join(format!("{}.json", anchor_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Anchor>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != anchor_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &anchor_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &anchor_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.anchor.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist AssociativeUI.
        {
            let path = path.join("associative_ui");
            fs::create_dir_all(&path)?;
            for associative_ui_tuple in self.associative_ui.read().unwrap().values() {
                let path = path.join(format!(
                    "{}.json",
                    associative_ui_tuple.0.read().unwrap().id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<AssociativeUi>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != associative_ui_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &associative_ui_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &associative_ui_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.associative_ui.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist BinaryUI.
        {
            let path = path.join("binary_ui");
            fs::create_dir_all(&path)?;
            for binary_ui_tuple in self.binary_ui.read().unwrap().values() {
                let path = path.join(format!("{}.json", binary_ui_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<BinaryUi>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != binary_ui_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &binary_ui_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &binary_ui_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.binary_ui.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Edge.
        {
            let path = path.join("edge");
            fs::create_dir_all(&path)?;
            for edge_tuple in self.edge.read().unwrap().values() {
                let path = path.join(format!("{}.json", edge_tuple.0.read().unwrap().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Edge>>, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != edge_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &edge_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &edge_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.edge.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist IsaUI.
        {
            let path = path.join("isa_ui");
            fs::create_dir_all(&path)?;
            for isa_ui_tuple in self.isa_ui.read().unwrap().values() {
                let path = path.join(format!("{}.json", isa_ui_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<IsaUi>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != isa_ui_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &isa_ui_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &isa_ui_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.isa_ui.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Object Edge.
        {
            let path = path.join("object_edge");
            fs::create_dir_all(&path)?;
            for object_edge_tuple in self.object_edge.read().unwrap().values() {
                let path = path.join(format!("{}.json", object_edge_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<ObjectEdge>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != object_edge_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &object_edge_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &object_edge_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.object_edge.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist ObjectUI.
        {
            let path = path.join("object_ui");
            fs::create_dir_all(&path)?;
            for object_ui_tuple in self.object_ui.read().unwrap().values() {
                let path = path.join(format!("{}.json", object_ui_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<ObjectUi>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != object_ui_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &object_ui_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &object_ui_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.object_ui.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Point.
        {
            let path = path.join("point");
            fs::create_dir_all(&path)?;
            for point_tuple in self.point.read().unwrap().values() {
                let path = path.join(format!("{}.json", point_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Point>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != point_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &point_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &point_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.point.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist RelationshipUI.
        {
            let path = path.join("relationship_ui");
            fs::create_dir_all(&path)?;
            for relationship_ui_tuple in self.relationship_ui.read().unwrap().values() {
                let path = path.join(format!(
                    "{}.json",
                    relationship_ui_tuple.0.read().unwrap().id()
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<RelationshipUi>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != relationship_ui_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &relationship_ui_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &relationship_ui_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.relationship_ui.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Subtype Anchors.
        {
            let path = path.join("subtype_anchors");
            fs::create_dir_all(&path)?;
            for subtype_anchors_tuple in self.subtype_anchors.read().unwrap().values() {
                let path = path.join(format!(
                    "{}.json",
                    subtype_anchors_tuple.0.read().unwrap().id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<SubtypeAnchors>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != subtype_anchors_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &subtype_anchors_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &subtype_anchors_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.subtype_anchors.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Load the store.
    ///
    pub fn from_bincode(code: &[u8]) -> io::Result<Self> {
        Ok(bincode::deserialize(code).unwrap())
    }

    /// The store is as a bincode file.
    pub fn load_bincode<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let bin_file = fs::File::open(path)?;
        Ok(bincode::deserialize_from(bin_file).unwrap())
    }

    /// Load the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automagic git integration as an option.
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let path = path.join("drawing.json");

        let store = Self::new();

        // Load Anchor.
        {
            let path = path.join("anchor");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let anchor: (Arc<RwLock<Anchor>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .anchor
                    .write()
                    .unwrap()
                    .insert(anchor.0.read().unwrap().id, anchor.clone());
            }
        }

        // Load AssociativeUI.
        {
            let path = path.join("associative_ui");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let associative_ui: (Arc<RwLock<AssociativeUi>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .associative_ui
                    .write()
                    .unwrap()
                    .insert(associative_ui.0.read().unwrap().id, associative_ui.clone());
            }
        }

        // Load BinaryUI.
        {
            let path = path.join("binary_ui");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let binary_ui: (Arc<RwLock<BinaryUi>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .binary_ui
                    .write()
                    .unwrap()
                    .insert(binary_ui.0.read().unwrap().id, binary_ui.clone());
            }
        }

        // Load Edge.
        {
            let path = path.join("edge");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let edge: (Arc<RwLock<Edge>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .edge
                    .write()
                    .unwrap()
                    .insert(edge.0.read().unwrap().id(), edge.clone());
            }
        }

        // Load IsaUI.
        {
            let path = path.join("isa_ui");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let isa_ui: (Arc<RwLock<IsaUi>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .isa_ui
                    .write()
                    .unwrap()
                    .insert(isa_ui.0.read().unwrap().id, isa_ui.clone());
            }
        }

        // Load Object Edge.
        {
            let path = path.join("object_edge");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let object_edge: (Arc<RwLock<ObjectEdge>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .object_edge
                    .write()
                    .unwrap()
                    .insert(object_edge.0.read().unwrap().id, object_edge.clone());
            }
        }

        // Load ObjectUI.
        {
            let path = path.join("object_ui");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let object_ui: (Arc<RwLock<ObjectUi>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .object_ui
                    .write()
                    .unwrap()
                    .insert(object_ui.0.read().unwrap().id, object_ui.clone());
            }
        }

        // Load Point.
        {
            let path = path.join("point");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let point: (Arc<RwLock<Point>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .point
                    .write()
                    .unwrap()
                    .insert(point.0.read().unwrap().id, point.clone());
            }
        }

        // Load RelationshipUI.
        {
            let path = path.join("relationship_ui");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let relationship_ui: (Arc<RwLock<RelationshipUi>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.relationship_ui.write().unwrap().insert(
                    relationship_ui.0.read().unwrap().id(),
                    relationship_ui.clone(),
                );
            }
        }

        // Load Subtype Anchors.
        {
            let path = path.join("subtype_anchors");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let subtype_anchors: (Arc<RwLock<SubtypeAnchors>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.subtype_anchors.write().unwrap().insert(
                    subtype_anchors.0.read().unwrap().id,
                    subtype_anchors.clone(),
                );
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
