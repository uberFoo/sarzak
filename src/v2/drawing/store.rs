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
use std::cell::RefCell;
use std::rc::Rc;
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
    anchor: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Anchor>>, SystemTime)>>>,
    associative_ui: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<AssociativeUi>>, SystemTime)>>>,
    binary_ui: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<BinaryUi>>, SystemTime)>>>,
    edge: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Edge>>, SystemTime)>>>,
    isa_ui: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<IsaUi>>, SystemTime)>>>,
    object_edge: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<ObjectEdge>>, SystemTime)>>>,
    object_ui: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<ObjectUi>>, SystemTime)>>>,
    point: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Point>>, SystemTime)>>>,
    relationship_ui: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<RelationshipUi>>, SystemTime)>>>,
    subtype_anchors: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<SubtypeAnchors>>, SystemTime)>>>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            anchor: Rc::new(RefCell::new(HashMap::default())),
            associative_ui: Rc::new(RefCell::new(HashMap::default())),
            binary_ui: Rc::new(RefCell::new(HashMap::default())),
            edge: Rc::new(RefCell::new(HashMap::default())),
            isa_ui: Rc::new(RefCell::new(HashMap::default())),
            object_edge: Rc::new(RefCell::new(HashMap::default())),
            object_ui: Rc::new(RefCell::new(HashMap::default())),
            point: Rc::new(RefCell::new(HashMap::default())),
            relationship_ui: Rc::new(RefCell::new(HashMap::default())),
            subtype_anchors: Rc::new(RefCell::new(HashMap::default())),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥
        store.inter_edge(Rc::new(RefCell::new(Edge::Bottom(BOTTOM))));
        store.inter_edge(Rc::new(RefCell::new(Edge::Left(LEFT))));
        store.inter_edge(Rc::new(RefCell::new(Edge::Right(RIGHT))));
        store.inter_edge(Rc::new(RefCell::new(Edge::Top(TOP))));

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::drawing-object-store-methods"}}}
    /// Inter (insert) [`Anchor`] into the store.
    ///
    pub fn inter_anchor(&mut self, anchor: Rc<RefCell<Anchor>>) {
        let read = anchor.borrow();
        self.anchor
            .borrow_mut()
            .insert(read.id, (anchor.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Anchor`] from the store.
    ///
    pub fn exhume_anchor(&self, id: &Uuid) -> Option<Rc<RefCell<Anchor>>> {
        self.anchor.borrow().get(id).map(|anchor| anchor.0.clone())
    }

    /// Exorcise (remove) [`Anchor`] from the store.
    ///
    pub fn exorcise_anchor(&mut self, id: &Uuid) -> Option<Rc<RefCell<Anchor>>> {
        self.anchor
            .borrow_mut()
            .remove(id)
            .map(|anchor| anchor.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Anchor>`.
    ///
    pub fn iter_anchor(&self) -> impl Iterator<Item = Rc<RefCell<Anchor>>> + '_ {
        let values: Vec<Rc<RefCell<Anchor>>> = self
            .anchor
            .borrow()
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
            .borrow()
            .get(&anchor.id)
            .map(|anchor| anchor.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`AssociativeUi`] into the store.
    ///
    pub fn inter_associative_ui(&mut self, associative_ui: Rc<RefCell<AssociativeUi>>) {
        let read = associative_ui.borrow();
        self.associative_ui
            .borrow_mut()
            .insert(read.id, (associative_ui.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`AssociativeUi`] from the store.
    ///
    pub fn exhume_associative_ui(&self, id: &Uuid) -> Option<Rc<RefCell<AssociativeUi>>> {
        self.associative_ui
            .borrow()
            .get(id)
            .map(|associative_ui| associative_ui.0.clone())
    }

    /// Exorcise (remove) [`AssociativeUi`] from the store.
    ///
    pub fn exorcise_associative_ui(&mut self, id: &Uuid) -> Option<Rc<RefCell<AssociativeUi>>> {
        self.associative_ui
            .borrow_mut()
            .remove(id)
            .map(|associative_ui| associative_ui.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, AssociativeUi>`.
    ///
    pub fn iter_associative_ui(&self) -> impl Iterator<Item = Rc<RefCell<AssociativeUi>>> + '_ {
        let values: Vec<Rc<RefCell<AssociativeUi>>> = self
            .associative_ui
            .borrow()
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
            .borrow()
            .get(&associative_ui.id)
            .map(|associative_ui| associative_ui.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`BinaryUi`] into the store.
    ///
    pub fn inter_binary_ui(&mut self, binary_ui: Rc<RefCell<BinaryUi>>) {
        let read = binary_ui.borrow();
        self.binary_ui
            .borrow_mut()
            .insert(read.id, (binary_ui.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`BinaryUi`] from the store.
    ///
    pub fn exhume_binary_ui(&self, id: &Uuid) -> Option<Rc<RefCell<BinaryUi>>> {
        self.binary_ui
            .borrow()
            .get(id)
            .map(|binary_ui| binary_ui.0.clone())
    }

    /// Exorcise (remove) [`BinaryUi`] from the store.
    ///
    pub fn exorcise_binary_ui(&mut self, id: &Uuid) -> Option<Rc<RefCell<BinaryUi>>> {
        self.binary_ui
            .borrow_mut()
            .remove(id)
            .map(|binary_ui| binary_ui.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BinaryUi>`.
    ///
    pub fn iter_binary_ui(&self) -> impl Iterator<Item = Rc<RefCell<BinaryUi>>> + '_ {
        let values: Vec<Rc<RefCell<BinaryUi>>> = self
            .binary_ui
            .borrow()
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
            .borrow()
            .get(&binary_ui.id)
            .map(|binary_ui| binary_ui.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Edge`] into the store.
    ///
    pub fn inter_edge(&mut self, edge: Rc<RefCell<Edge>>) {
        let read = edge.borrow();
        self.edge
            .borrow_mut()
            .insert(read.id(), (edge.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Edge`] from the store.
    ///
    pub fn exhume_edge(&self, id: &Uuid) -> Option<Rc<RefCell<Edge>>> {
        self.edge.borrow().get(id).map(|edge| edge.0.clone())
    }

    /// Exorcise (remove) [`Edge`] from the store.
    ///
    pub fn exorcise_edge(&mut self, id: &Uuid) -> Option<Rc<RefCell<Edge>>> {
        self.edge.borrow_mut().remove(id).map(|edge| edge.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Edge>`.
    ///
    pub fn iter_edge(&self) -> impl Iterator<Item = Rc<RefCell<Edge>>> + '_ {
        let values: Vec<Rc<RefCell<Edge>>> = self
            .edge
            .borrow()
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
            .borrow()
            .get(&edge.id())
            .map(|edge| edge.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`IsaUi`] into the store.
    ///
    pub fn inter_isa_ui(&mut self, isa_ui: Rc<RefCell<IsaUi>>) {
        let read = isa_ui.borrow();
        self.isa_ui
            .borrow_mut()
            .insert(read.id, (isa_ui.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`IsaUi`] from the store.
    ///
    pub fn exhume_isa_ui(&self, id: &Uuid) -> Option<Rc<RefCell<IsaUi>>> {
        self.isa_ui.borrow().get(id).map(|isa_ui| isa_ui.0.clone())
    }

    /// Exorcise (remove) [`IsaUi`] from the store.
    ///
    pub fn exorcise_isa_ui(&mut self, id: &Uuid) -> Option<Rc<RefCell<IsaUi>>> {
        self.isa_ui
            .borrow_mut()
            .remove(id)
            .map(|isa_ui| isa_ui.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, IsaUi>`.
    ///
    pub fn iter_isa_ui(&self) -> impl Iterator<Item = Rc<RefCell<IsaUi>>> + '_ {
        let values: Vec<Rc<RefCell<IsaUi>>> = self
            .isa_ui
            .borrow()
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
            .borrow()
            .get(&isa_ui.id)
            .map(|isa_ui| isa_ui.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ObjectEdge`] into the store.
    ///
    pub fn inter_object_edge(&mut self, object_edge: Rc<RefCell<ObjectEdge>>) {
        let read = object_edge.borrow();
        self.object_edge
            .borrow_mut()
            .insert(read.id, (object_edge.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ObjectEdge`] from the store.
    ///
    pub fn exhume_object_edge(&self, id: &Uuid) -> Option<Rc<RefCell<ObjectEdge>>> {
        self.object_edge
            .borrow()
            .get(id)
            .map(|object_edge| object_edge.0.clone())
    }

    /// Exorcise (remove) [`ObjectEdge`] from the store.
    ///
    pub fn exorcise_object_edge(&mut self, id: &Uuid) -> Option<Rc<RefCell<ObjectEdge>>> {
        self.object_edge
            .borrow_mut()
            .remove(id)
            .map(|object_edge| object_edge.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ObjectEdge>`.
    ///
    pub fn iter_object_edge(&self) -> impl Iterator<Item = Rc<RefCell<ObjectEdge>>> + '_ {
        let values: Vec<Rc<RefCell<ObjectEdge>>> = self
            .object_edge
            .borrow()
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
            .borrow()
            .get(&object_edge.id)
            .map(|object_edge| object_edge.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ObjectUi`] into the store.
    ///
    pub fn inter_object_ui(&mut self, object_ui: Rc<RefCell<ObjectUi>>) {
        let read = object_ui.borrow();
        self.object_ui
            .borrow_mut()
            .insert(read.id, (object_ui.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ObjectUi`] from the store.
    ///
    pub fn exhume_object_ui(&self, id: &Uuid) -> Option<Rc<RefCell<ObjectUi>>> {
        self.object_ui
            .borrow()
            .get(id)
            .map(|object_ui| object_ui.0.clone())
    }

    /// Exorcise (remove) [`ObjectUi`] from the store.
    ///
    pub fn exorcise_object_ui(&mut self, id: &Uuid) -> Option<Rc<RefCell<ObjectUi>>> {
        self.object_ui
            .borrow_mut()
            .remove(id)
            .map(|object_ui| object_ui.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ObjectUi>`.
    ///
    pub fn iter_object_ui(&self) -> impl Iterator<Item = Rc<RefCell<ObjectUi>>> + '_ {
        let values: Vec<Rc<RefCell<ObjectUi>>> = self
            .object_ui
            .borrow()
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
            .borrow()
            .get(&object_ui.id)
            .map(|object_ui| object_ui.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Point`] into the store.
    ///
    pub fn inter_point(&mut self, point: Rc<RefCell<Point>>) {
        let read = point.borrow();
        self.point
            .borrow_mut()
            .insert(read.id, (point.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Point`] from the store.
    ///
    pub fn exhume_point(&self, id: &Uuid) -> Option<Rc<RefCell<Point>>> {
        self.point.borrow().get(id).map(|point| point.0.clone())
    }

    /// Exorcise (remove) [`Point`] from the store.
    ///
    pub fn exorcise_point(&mut self, id: &Uuid) -> Option<Rc<RefCell<Point>>> {
        self.point
            .borrow_mut()
            .remove(id)
            .map(|point| point.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Point>`.
    ///
    pub fn iter_point(&self) -> impl Iterator<Item = Rc<RefCell<Point>>> + '_ {
        let values: Vec<Rc<RefCell<Point>>> = self
            .point
            .borrow()
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
            .borrow()
            .get(&point.id)
            .map(|point| point.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`RelationshipUi`] into the store.
    ///
    pub fn inter_relationship_ui(&mut self, relationship_ui: Rc<RefCell<RelationshipUi>>) {
        let read = relationship_ui.borrow();
        self.relationship_ui
            .borrow_mut()
            .insert(read.id(), (relationship_ui.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`RelationshipUi`] from the store.
    ///
    pub fn exhume_relationship_ui(&self, id: &Uuid) -> Option<Rc<RefCell<RelationshipUi>>> {
        self.relationship_ui
            .borrow()
            .get(id)
            .map(|relationship_ui| relationship_ui.0.clone())
    }

    /// Exorcise (remove) [`RelationshipUi`] from the store.
    ///
    pub fn exorcise_relationship_ui(&mut self, id: &Uuid) -> Option<Rc<RefCell<RelationshipUi>>> {
        self.relationship_ui
            .borrow_mut()
            .remove(id)
            .map(|relationship_ui| relationship_ui.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RelationshipUi>`.
    ///
    pub fn iter_relationship_ui(&self) -> impl Iterator<Item = Rc<RefCell<RelationshipUi>>> + '_ {
        let values: Vec<Rc<RefCell<RelationshipUi>>> = self
            .relationship_ui
            .borrow()
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
            .borrow()
            .get(&relationship_ui.id())
            .map(|relationship_ui| relationship_ui.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`SubtypeAnchors`] into the store.
    ///
    pub fn inter_subtype_anchors(&mut self, subtype_anchors: Rc<RefCell<SubtypeAnchors>>) {
        let read = subtype_anchors.borrow();
        self.subtype_anchors
            .borrow_mut()
            .insert(read.id, (subtype_anchors.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`SubtypeAnchors`] from the store.
    ///
    pub fn exhume_subtype_anchors(&self, id: &Uuid) -> Option<Rc<RefCell<SubtypeAnchors>>> {
        self.subtype_anchors
            .borrow()
            .get(id)
            .map(|subtype_anchors| subtype_anchors.0.clone())
    }

    /// Exorcise (remove) [`SubtypeAnchors`] from the store.
    ///
    pub fn exorcise_subtype_anchors(&mut self, id: &Uuid) -> Option<Rc<RefCell<SubtypeAnchors>>> {
        self.subtype_anchors
            .borrow_mut()
            .remove(id)
            .map(|subtype_anchors| subtype_anchors.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SubtypeAnchors>`.
    ///
    pub fn iter_subtype_anchors(&self) -> impl Iterator<Item = Rc<RefCell<SubtypeAnchors>>> + '_ {
        let values: Vec<Rc<RefCell<SubtypeAnchors>>> = self
            .subtype_anchors
            .borrow()
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
            .borrow()
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
            for anchor_tuple in self.anchor.borrow().values() {
                let path = path.join(format!("{}.json", anchor_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Anchor>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != anchor_tuple.0.borrow().to_owned() {
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
                    if !self.anchor.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist AssociativeUI.
        {
            let path = path.join("associative_ui");
            fs::create_dir_all(&path)?;
            for associative_ui_tuple in self.associative_ui.borrow().values() {
                let path = path.join(format!("{}.json", associative_ui_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<AssociativeUi>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != associative_ui_tuple.0.borrow().to_owned() {
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
                    if !self.associative_ui.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist BinaryUI.
        {
            let path = path.join("binary_ui");
            fs::create_dir_all(&path)?;
            for binary_ui_tuple in self.binary_ui.borrow().values() {
                let path = path.join(format!("{}.json", binary_ui_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<BinaryUi>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != binary_ui_tuple.0.borrow().to_owned() {
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
                    if !self.binary_ui.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Edge.
        {
            let path = path.join("edge");
            fs::create_dir_all(&path)?;
            for edge_tuple in self.edge.borrow().values() {
                let path = path.join(format!("{}.json", edge_tuple.0.borrow().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Edge>>, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != edge_tuple.0.borrow().to_owned() {
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
                    if !self.edge.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist IsaUI.
        {
            let path = path.join("isa_ui");
            fs::create_dir_all(&path)?;
            for isa_ui_tuple in self.isa_ui.borrow().values() {
                let path = path.join(format!("{}.json", isa_ui_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<IsaUi>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != isa_ui_tuple.0.borrow().to_owned() {
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
                    if !self.isa_ui.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Object Edge.
        {
            let path = path.join("object_edge");
            fs::create_dir_all(&path)?;
            for object_edge_tuple in self.object_edge.borrow().values() {
                let path = path.join(format!("{}.json", object_edge_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<ObjectEdge>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != object_edge_tuple.0.borrow().to_owned() {
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
                    if !self.object_edge.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist ObjectUI.
        {
            let path = path.join("object_ui");
            fs::create_dir_all(&path)?;
            for object_ui_tuple in self.object_ui.borrow().values() {
                let path = path.join(format!("{}.json", object_ui_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<ObjectUi>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != object_ui_tuple.0.borrow().to_owned() {
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
                    if !self.object_ui.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Point.
        {
            let path = path.join("point");
            fs::create_dir_all(&path)?;
            for point_tuple in self.point.borrow().values() {
                let path = path.join(format!("{}.json", point_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Point>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != point_tuple.0.borrow().to_owned() {
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
                    if !self.point.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist RelationshipUI.
        {
            let path = path.join("relationship_ui");
            fs::create_dir_all(&path)?;
            for relationship_ui_tuple in self.relationship_ui.borrow().values() {
                let path = path.join(format!("{}.json", relationship_ui_tuple.0.borrow().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<RelationshipUi>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != relationship_ui_tuple.0.borrow().to_owned()
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
                    if !self.relationship_ui.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Subtype Anchors.
        {
            let path = path.join("subtype_anchors");
            fs::create_dir_all(&path)?;
            for subtype_anchors_tuple in self.subtype_anchors.borrow().values() {
                let path = path.join(format!("{}.json", subtype_anchors_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<SubtypeAnchors>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != subtype_anchors_tuple.0.borrow().to_owned()
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
                    if !self.subtype_anchors.borrow().contains_key(&id) {
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
                let anchor: (Rc<RefCell<Anchor>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .anchor
                    .borrow_mut()
                    .insert(anchor.0.borrow().id, anchor.clone());
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
                let associative_ui: (Rc<RefCell<AssociativeUi>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .associative_ui
                    .borrow_mut()
                    .insert(associative_ui.0.borrow().id, associative_ui.clone());
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
                let binary_ui: (Rc<RefCell<BinaryUi>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .binary_ui
                    .borrow_mut()
                    .insert(binary_ui.0.borrow().id, binary_ui.clone());
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
                let edge: (Rc<RefCell<Edge>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .edge
                    .borrow_mut()
                    .insert(edge.0.borrow().id(), edge.clone());
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
                let isa_ui: (Rc<RefCell<IsaUi>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .isa_ui
                    .borrow_mut()
                    .insert(isa_ui.0.borrow().id, isa_ui.clone());
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
                let object_edge: (Rc<RefCell<ObjectEdge>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .object_edge
                    .borrow_mut()
                    .insert(object_edge.0.borrow().id, object_edge.clone());
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
                let object_ui: (Rc<RefCell<ObjectUi>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .object_ui
                    .borrow_mut()
                    .insert(object_ui.0.borrow().id, object_ui.clone());
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
                let point: (Rc<RefCell<Point>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .point
                    .borrow_mut()
                    .insert(point.0.borrow().id, point.clone());
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
                let relationship_ui: (Rc<RefCell<RelationshipUi>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .relationship_ui
                    .borrow_mut()
                    .insert(relationship_ui.0.borrow().id(), relationship_ui.clone());
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
                let subtype_anchors: (Rc<RefCell<SubtypeAnchors>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .subtype_anchors
                    .borrow_mut()
                    .insert(subtype_anchors.0.borrow().id, subtype_anchors.clone());
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
