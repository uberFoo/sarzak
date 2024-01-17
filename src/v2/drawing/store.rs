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
    anchor: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Anchor>>>>>,
    associative_ui: Arc<RwLock<HashMap<Uuid, Arc<RwLock<AssociativeUi>>>>>,
    binary_ui: Arc<RwLock<HashMap<Uuid, Arc<RwLock<BinaryUi>>>>>,
    edge: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Edge>>>>>,
    isa_ui: Arc<RwLock<HashMap<Uuid, Arc<RwLock<IsaUi>>>>>,
    object_edge: Arc<RwLock<HashMap<Uuid, Arc<RwLock<ObjectEdge>>>>>,
    object_ui: Arc<RwLock<HashMap<Uuid, Arc<RwLock<ObjectUi>>>>>,
    point: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Point>>>>>,
    relationship_ui: Arc<RwLock<HashMap<Uuid, Arc<RwLock<RelationshipUi>>>>>,
    subtype_anchors: Arc<RwLock<HashMap<Uuid, Arc<RwLock<SubtypeAnchors>>>>>,
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
        self.anchor.write().unwrap().insert(read.id, anchor.clone());
    }

    /// Exhume (get) [`Anchor`] from the store.
    ///
    pub fn exhume_anchor(&self, id: &Uuid) -> Option<Arc<RwLock<Anchor>>> {
        self.anchor
            .read()
            .unwrap()
            .get(id)
            .map(|anchor| anchor.clone())
    }

    /// Exorcise (remove) [`Anchor`] from the store.
    ///
    pub fn exorcise_anchor(&mut self, id: &Uuid) -> Option<Arc<RwLock<Anchor>>> {
        self.anchor
            .write()
            .unwrap()
            .remove(id)
            .map(|anchor| anchor.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Anchor>`.
    ///
    pub fn iter_anchor(&self) -> impl Iterator<Item = Arc<RwLock<Anchor>>> + '_ {
        let values: Vec<Arc<RwLock<Anchor>>> = self
            .anchor
            .read()
            .unwrap()
            .values()
            .map(|anchor| anchor.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`AssociativeUi`] into the store.
    ///
    pub fn inter_associative_ui(&mut self, associative_ui: Arc<RwLock<AssociativeUi>>) {
        let read = associative_ui.read().unwrap();
        self.associative_ui
            .write()
            .unwrap()
            .insert(read.id, associative_ui.clone());
    }

    /// Exhume (get) [`AssociativeUi`] from the store.
    ///
    pub fn exhume_associative_ui(&self, id: &Uuid) -> Option<Arc<RwLock<AssociativeUi>>> {
        self.associative_ui
            .read()
            .unwrap()
            .get(id)
            .map(|associative_ui| associative_ui.clone())
    }

    /// Exorcise (remove) [`AssociativeUi`] from the store.
    ///
    pub fn exorcise_associative_ui(&mut self, id: &Uuid) -> Option<Arc<RwLock<AssociativeUi>>> {
        self.associative_ui
            .write()
            .unwrap()
            .remove(id)
            .map(|associative_ui| associative_ui.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, AssociativeUi>`.
    ///
    pub fn iter_associative_ui(&self) -> impl Iterator<Item = Arc<RwLock<AssociativeUi>>> + '_ {
        let values: Vec<Arc<RwLock<AssociativeUi>>> = self
            .associative_ui
            .read()
            .unwrap()
            .values()
            .map(|associative_ui| associative_ui.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`BinaryUi`] into the store.
    ///
    pub fn inter_binary_ui(&mut self, binary_ui: Arc<RwLock<BinaryUi>>) {
        let read = binary_ui.read().unwrap();
        self.binary_ui
            .write()
            .unwrap()
            .insert(read.id, binary_ui.clone());
    }

    /// Exhume (get) [`BinaryUi`] from the store.
    ///
    pub fn exhume_binary_ui(&self, id: &Uuid) -> Option<Arc<RwLock<BinaryUi>>> {
        self.binary_ui
            .read()
            .unwrap()
            .get(id)
            .map(|binary_ui| binary_ui.clone())
    }

    /// Exorcise (remove) [`BinaryUi`] from the store.
    ///
    pub fn exorcise_binary_ui(&mut self, id: &Uuid) -> Option<Arc<RwLock<BinaryUi>>> {
        self.binary_ui
            .write()
            .unwrap()
            .remove(id)
            .map(|binary_ui| binary_ui.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BinaryUi>`.
    ///
    pub fn iter_binary_ui(&self) -> impl Iterator<Item = Arc<RwLock<BinaryUi>>> + '_ {
        let values: Vec<Arc<RwLock<BinaryUi>>> = self
            .binary_ui
            .read()
            .unwrap()
            .values()
            .map(|binary_ui| binary_ui.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Edge`] into the store.
    ///
    pub fn inter_edge(&mut self, edge: Arc<RwLock<Edge>>) {
        let read = edge.read().unwrap();
        self.edge.write().unwrap().insert(read.id(), edge.clone());
    }

    /// Exhume (get) [`Edge`] from the store.
    ///
    pub fn exhume_edge(&self, id: &Uuid) -> Option<Arc<RwLock<Edge>>> {
        self.edge.read().unwrap().get(id).map(|edge| edge.clone())
    }

    /// Exorcise (remove) [`Edge`] from the store.
    ///
    pub fn exorcise_edge(&mut self, id: &Uuid) -> Option<Arc<RwLock<Edge>>> {
        self.edge
            .write()
            .unwrap()
            .remove(id)
            .map(|edge| edge.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Edge>`.
    ///
    pub fn iter_edge(&self) -> impl Iterator<Item = Arc<RwLock<Edge>>> + '_ {
        let values: Vec<Arc<RwLock<Edge>>> = self
            .edge
            .read()
            .unwrap()
            .values()
            .map(|edge| edge.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`IsaUi`] into the store.
    ///
    pub fn inter_isa_ui(&mut self, isa_ui: Arc<RwLock<IsaUi>>) {
        let read = isa_ui.read().unwrap();
        self.isa_ui.write().unwrap().insert(read.id, isa_ui.clone());
    }

    /// Exhume (get) [`IsaUi`] from the store.
    ///
    pub fn exhume_isa_ui(&self, id: &Uuid) -> Option<Arc<RwLock<IsaUi>>> {
        self.isa_ui
            .read()
            .unwrap()
            .get(id)
            .map(|isa_ui| isa_ui.clone())
    }

    /// Exorcise (remove) [`IsaUi`] from the store.
    ///
    pub fn exorcise_isa_ui(&mut self, id: &Uuid) -> Option<Arc<RwLock<IsaUi>>> {
        self.isa_ui
            .write()
            .unwrap()
            .remove(id)
            .map(|isa_ui| isa_ui.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, IsaUi>`.
    ///
    pub fn iter_isa_ui(&self) -> impl Iterator<Item = Arc<RwLock<IsaUi>>> + '_ {
        let values: Vec<Arc<RwLock<IsaUi>>> = self
            .isa_ui
            .read()
            .unwrap()
            .values()
            .map(|isa_ui| isa_ui.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`ObjectEdge`] into the store.
    ///
    pub fn inter_object_edge(&mut self, object_edge: Arc<RwLock<ObjectEdge>>) {
        let read = object_edge.read().unwrap();
        self.object_edge
            .write()
            .unwrap()
            .insert(read.id, object_edge.clone());
    }

    /// Exhume (get) [`ObjectEdge`] from the store.
    ///
    pub fn exhume_object_edge(&self, id: &Uuid) -> Option<Arc<RwLock<ObjectEdge>>> {
        self.object_edge
            .read()
            .unwrap()
            .get(id)
            .map(|object_edge| object_edge.clone())
    }

    /// Exorcise (remove) [`ObjectEdge`] from the store.
    ///
    pub fn exorcise_object_edge(&mut self, id: &Uuid) -> Option<Arc<RwLock<ObjectEdge>>> {
        self.object_edge
            .write()
            .unwrap()
            .remove(id)
            .map(|object_edge| object_edge.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ObjectEdge>`.
    ///
    pub fn iter_object_edge(&self) -> impl Iterator<Item = Arc<RwLock<ObjectEdge>>> + '_ {
        let values: Vec<Arc<RwLock<ObjectEdge>>> = self
            .object_edge
            .read()
            .unwrap()
            .values()
            .map(|object_edge| object_edge.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`ObjectUi`] into the store.
    ///
    pub fn inter_object_ui(&mut self, object_ui: Arc<RwLock<ObjectUi>>) {
        let read = object_ui.read().unwrap();
        self.object_ui
            .write()
            .unwrap()
            .insert(read.id, object_ui.clone());
    }

    /// Exhume (get) [`ObjectUi`] from the store.
    ///
    pub fn exhume_object_ui(&self, id: &Uuid) -> Option<Arc<RwLock<ObjectUi>>> {
        self.object_ui
            .read()
            .unwrap()
            .get(id)
            .map(|object_ui| object_ui.clone())
    }

    /// Exorcise (remove) [`ObjectUi`] from the store.
    ///
    pub fn exorcise_object_ui(&mut self, id: &Uuid) -> Option<Arc<RwLock<ObjectUi>>> {
        self.object_ui
            .write()
            .unwrap()
            .remove(id)
            .map(|object_ui| object_ui.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ObjectUi>`.
    ///
    pub fn iter_object_ui(&self) -> impl Iterator<Item = Arc<RwLock<ObjectUi>>> + '_ {
        let values: Vec<Arc<RwLock<ObjectUi>>> = self
            .object_ui
            .read()
            .unwrap()
            .values()
            .map(|object_ui| object_ui.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Point`] into the store.
    ///
    pub fn inter_point(&mut self, point: Arc<RwLock<Point>>) {
        let read = point.read().unwrap();
        self.point.write().unwrap().insert(read.id, point.clone());
    }

    /// Exhume (get) [`Point`] from the store.
    ///
    pub fn exhume_point(&self, id: &Uuid) -> Option<Arc<RwLock<Point>>> {
        self.point
            .read()
            .unwrap()
            .get(id)
            .map(|point| point.clone())
    }

    /// Exorcise (remove) [`Point`] from the store.
    ///
    pub fn exorcise_point(&mut self, id: &Uuid) -> Option<Arc<RwLock<Point>>> {
        self.point
            .write()
            .unwrap()
            .remove(id)
            .map(|point| point.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Point>`.
    ///
    pub fn iter_point(&self) -> impl Iterator<Item = Arc<RwLock<Point>>> + '_ {
        let values: Vec<Arc<RwLock<Point>>> = self
            .point
            .read()
            .unwrap()
            .values()
            .map(|point| point.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`RelationshipUi`] into the store.
    ///
    pub fn inter_relationship_ui(&mut self, relationship_ui: Arc<RwLock<RelationshipUi>>) {
        let read = relationship_ui.read().unwrap();
        self.relationship_ui
            .write()
            .unwrap()
            .insert(read.id(), relationship_ui.clone());
    }

    /// Exhume (get) [`RelationshipUi`] from the store.
    ///
    pub fn exhume_relationship_ui(&self, id: &Uuid) -> Option<Arc<RwLock<RelationshipUi>>> {
        self.relationship_ui
            .read()
            .unwrap()
            .get(id)
            .map(|relationship_ui| relationship_ui.clone())
    }

    /// Exorcise (remove) [`RelationshipUi`] from the store.
    ///
    pub fn exorcise_relationship_ui(&mut self, id: &Uuid) -> Option<Arc<RwLock<RelationshipUi>>> {
        self.relationship_ui
            .write()
            .unwrap()
            .remove(id)
            .map(|relationship_ui| relationship_ui.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RelationshipUi>`.
    ///
    pub fn iter_relationship_ui(&self) -> impl Iterator<Item = Arc<RwLock<RelationshipUi>>> + '_ {
        let values: Vec<Arc<RwLock<RelationshipUi>>> = self
            .relationship_ui
            .read()
            .unwrap()
            .values()
            .map(|relationship_ui| relationship_ui.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`SubtypeAnchors`] into the store.
    ///
    pub fn inter_subtype_anchors(&mut self, subtype_anchors: Arc<RwLock<SubtypeAnchors>>) {
        let read = subtype_anchors.read().unwrap();
        self.subtype_anchors
            .write()
            .unwrap()
            .insert(read.id, subtype_anchors.clone());
    }

    /// Exhume (get) [`SubtypeAnchors`] from the store.
    ///
    pub fn exhume_subtype_anchors(&self, id: &Uuid) -> Option<Arc<RwLock<SubtypeAnchors>>> {
        self.subtype_anchors
            .read()
            .unwrap()
            .get(id)
            .map(|subtype_anchors| subtype_anchors.clone())
    }

    /// Exorcise (remove) [`SubtypeAnchors`] from the store.
    ///
    pub fn exorcise_subtype_anchors(&mut self, id: &Uuid) -> Option<Arc<RwLock<SubtypeAnchors>>> {
        self.subtype_anchors
            .write()
            .unwrap()
            .remove(id)
            .map(|subtype_anchors| subtype_anchors.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SubtypeAnchors>`.
    ///
    pub fn iter_subtype_anchors(&self) -> impl Iterator<Item = Arc<RwLock<SubtypeAnchors>>> + '_ {
        let values: Vec<Arc<RwLock<SubtypeAnchors>>> = self
            .subtype_anchors
            .read()
            .unwrap()
            .values()
            .map(|subtype_anchors| subtype_anchors.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
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
            for anchor in self.anchor.read().unwrap().values() {
                let path = path.join(format!("{}.json", anchor.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &anchor)?;
            }
        }

        // Persist AssociativeUI.
        {
            let path = path.join("associative_ui");
            fs::create_dir_all(&path)?;
            for associative_ui in self.associative_ui.read().unwrap().values() {
                let path = path.join(format!("{}.json", associative_ui.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &associative_ui)?;
            }
        }

        // Persist BinaryUI.
        {
            let path = path.join("binary_ui");
            fs::create_dir_all(&path)?;
            for binary_ui in self.binary_ui.read().unwrap().values() {
                let path = path.join(format!("{}.json", binary_ui.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &binary_ui)?;
            }
        }

        // Persist Edge.
        {
            let path = path.join("edge");
            fs::create_dir_all(&path)?;
            for edge in self.edge.read().unwrap().values() {
                let path = path.join(format!("{}.json", edge.read().unwrap().id()));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &edge)?;
            }
        }

        // Persist IsaUI.
        {
            let path = path.join("isa_ui");
            fs::create_dir_all(&path)?;
            for isa_ui in self.isa_ui.read().unwrap().values() {
                let path = path.join(format!("{}.json", isa_ui.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &isa_ui)?;
            }
        }

        // Persist Object Edge.
        {
            let path = path.join("object_edge");
            fs::create_dir_all(&path)?;
            for object_edge in self.object_edge.read().unwrap().values() {
                let path = path.join(format!("{}.json", object_edge.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &object_edge)?;
            }
        }

        // Persist ObjectUI.
        {
            let path = path.join("object_ui");
            fs::create_dir_all(&path)?;
            for object_ui in self.object_ui.read().unwrap().values() {
                let path = path.join(format!("{}.json", object_ui.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &object_ui)?;
            }
        }

        // Persist Point.
        {
            let path = path.join("point");
            fs::create_dir_all(&path)?;
            for point in self.point.read().unwrap().values() {
                let path = path.join(format!("{}.json", point.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &point)?;
            }
        }

        // Persist RelationshipUI.
        {
            let path = path.join("relationship_ui");
            fs::create_dir_all(&path)?;
            for relationship_ui in self.relationship_ui.read().unwrap().values() {
                let path = path.join(format!("{}.json", relationship_ui.read().unwrap().id()));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &relationship_ui)?;
            }
        }

        // Persist Subtype Anchors.
        {
            let path = path.join("subtype_anchors");
            fs::create_dir_all(&path)?;
            for subtype_anchors in self.subtype_anchors.read().unwrap().values() {
                let path = path.join(format!("{}.json", subtype_anchors.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &subtype_anchors)?;
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
                let anchor: Arc<RwLock<Anchor>> = serde_json::from_reader(reader)?;
                store
                    .anchor
                    .write()
                    .unwrap()
                    .insert(anchor.read().unwrap().id, anchor.clone());
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
                let associative_ui: Arc<RwLock<AssociativeUi>> = serde_json::from_reader(reader)?;
                store
                    .associative_ui
                    .write()
                    .unwrap()
                    .insert(associative_ui.read().unwrap().id, associative_ui.clone());
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
                let binary_ui: Arc<RwLock<BinaryUi>> = serde_json::from_reader(reader)?;
                store
                    .binary_ui
                    .write()
                    .unwrap()
                    .insert(binary_ui.read().unwrap().id, binary_ui.clone());
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
                let edge: Arc<RwLock<Edge>> = serde_json::from_reader(reader)?;
                store
                    .edge
                    .write()
                    .unwrap()
                    .insert(edge.read().unwrap().id(), edge.clone());
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
                let isa_ui: Arc<RwLock<IsaUi>> = serde_json::from_reader(reader)?;
                store
                    .isa_ui
                    .write()
                    .unwrap()
                    .insert(isa_ui.read().unwrap().id, isa_ui.clone());
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
                let object_edge: Arc<RwLock<ObjectEdge>> = serde_json::from_reader(reader)?;
                store
                    .object_edge
                    .write()
                    .unwrap()
                    .insert(object_edge.read().unwrap().id, object_edge.clone());
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
                let object_ui: Arc<RwLock<ObjectUi>> = serde_json::from_reader(reader)?;
                store
                    .object_ui
                    .write()
                    .unwrap()
                    .insert(object_ui.read().unwrap().id, object_ui.clone());
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
                let point: Arc<RwLock<Point>> = serde_json::from_reader(reader)?;
                store
                    .point
                    .write()
                    .unwrap()
                    .insert(point.read().unwrap().id, point.clone());
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
                let relationship_ui: Arc<RwLock<RelationshipUi>> = serde_json::from_reader(reader)?;
                store.relationship_ui.write().unwrap().insert(
                    relationship_ui.read().unwrap().id(),
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
                let subtype_anchors: Arc<RwLock<SubtypeAnchors>> = serde_json::from_reader(reader)?;
                store
                    .subtype_anchors
                    .write()
                    .unwrap()
                    .insert(subtype_anchors.read().unwrap().id, subtype_anchors.clone());
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
