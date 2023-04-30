//! v2::merlin Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::merlin-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`Anchor`]
//! * [`Bisection`]
//! * [`XBox`]
//! * [`Edge`]
//! * [`Glyph`]
//! * [`Line`]
//! * [`LineSegment`]
//! * [`LineSegmentPoint`]
//! * [`Point`]
//! * [`RelationshipName`]
//! * [`RelationshipPhrase`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::merlin-object-store-definition"}}}
use std::sync::{Arc, RwLock};
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
    time::SystemTime,
};

use fnv::FnvHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v2::merlin::types::{
    Anchor, Bisection, Edge, Glyph, Line, LineSegment, LineSegmentPoint, Point, RelationshipName,
    RelationshipPhrase, XBox, BOTTOM, LEFT, RIGHT, TOP,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    anchor: HashMap<Uuid, (Arc<RwLock<Anchor>>, SystemTime)>,
    bisection: HashMap<Uuid, (Arc<RwLock<Bisection>>, SystemTime)>,
    x_box: HashMap<Uuid, (Arc<RwLock<XBox>>, SystemTime)>,
    edge: HashMap<Uuid, (Arc<RwLock<Edge>>, SystemTime)>,
    glyph: HashMap<Uuid, (Arc<RwLock<Glyph>>, SystemTime)>,
    line: HashMap<Uuid, (Arc<RwLock<Line>>, SystemTime)>,
    line_segment: HashMap<Uuid, (Arc<RwLock<LineSegment>>, SystemTime)>,
    line_segment_point: HashMap<Uuid, (Arc<RwLock<LineSegmentPoint>>, SystemTime)>,
    point: HashMap<Uuid, (Arc<RwLock<Point>>, SystemTime)>,
    relationship_name: HashMap<Uuid, (Arc<RwLock<RelationshipName>>, SystemTime)>,
    relationship_phrase: HashMap<Uuid, (Arc<RwLock<RelationshipPhrase>>, SystemTime)>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            anchor: HashMap::default(),
            bisection: HashMap::default(),
            x_box: HashMap::default(),
            edge: HashMap::default(),
            glyph: HashMap::default(),
            line: HashMap::default(),
            line_segment: HashMap::default(),
            line_segment_point: HashMap::default(),
            point: HashMap::default(),
            relationship_name: HashMap::default(),
            relationship_phrase: HashMap::default(),
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

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::merlin-object-store-methods"}}}
    /// Inter [`Anchor`] into the store.
    ///
    pub fn inter_anchor(&mut self, anchor: Arc<RwLock<Anchor>>) {
        let read = anchor.read().unwrap();
        self.anchor
            .insert(read.id, (anchor.clone(), SystemTime::now()));
    }

    /// Exhume [`Anchor`] from the store.
    ///
    pub fn exhume_anchor(&self, id: &Uuid) -> Option<Arc<RwLock<Anchor>>> {
        self.anchor.get(id).map(|anchor| anchor.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Anchor>`.
    ///
    pub fn iter_anchor(&self) -> impl Iterator<Item = Arc<RwLock<Anchor>>> + '_ {
        self.anchor.values().map(|anchor| anchor.0.clone())
    }

    /// Get the timestamp for Anchor.
    ///
    pub fn anchor_timestamp(&self, anchor: &Anchor) -> SystemTime {
        self.anchor
            .get(&anchor.id)
            .map(|anchor| anchor.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Bisection`] into the store.
    ///
    pub fn inter_bisection(&mut self, bisection: Arc<RwLock<Bisection>>) {
        let read = bisection.read().unwrap();
        self.bisection
            .insert(read.id, (bisection.clone(), SystemTime::now()));
    }

    /// Exhume [`Bisection`] from the store.
    ///
    pub fn exhume_bisection(&self, id: &Uuid) -> Option<Arc<RwLock<Bisection>>> {
        self.bisection.get(id).map(|bisection| bisection.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Bisection>`.
    ///
    pub fn iter_bisection(&self) -> impl Iterator<Item = Arc<RwLock<Bisection>>> + '_ {
        self.bisection.values().map(|bisection| bisection.0.clone())
    }

    /// Get the timestamp for Bisection.
    ///
    pub fn bisection_timestamp(&self, bisection: &Bisection) -> SystemTime {
        self.bisection
            .get(&bisection.id)
            .map(|bisection| bisection.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`XBox`] into the store.
    ///
    pub fn inter_x_box(&mut self, x_box: Arc<RwLock<XBox>>) {
        let read = x_box.read().unwrap();
        self.x_box
            .insert(read.id, (x_box.clone(), SystemTime::now()));
    }

    /// Exhume [`XBox`] from the store.
    ///
    pub fn exhume_x_box(&self, id: &Uuid) -> Option<Arc<RwLock<XBox>>> {
        self.x_box.get(id).map(|x_box| x_box.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XBox>`.
    ///
    pub fn iter_x_box(&self) -> impl Iterator<Item = Arc<RwLock<XBox>>> + '_ {
        self.x_box.values().map(|x_box| x_box.0.clone())
    }

    /// Get the timestamp for XBox.
    ///
    pub fn x_box_timestamp(&self, x_box: &XBox) -> SystemTime {
        self.x_box
            .get(&x_box.id)
            .map(|x_box| x_box.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Edge`] into the store.
    ///
    pub fn inter_edge(&mut self, edge: Arc<RwLock<Edge>>) {
        let read = edge.read().unwrap();
        self.edge
            .insert(read.id(), (edge.clone(), SystemTime::now()));
    }

    /// Exhume [`Edge`] from the store.
    ///
    pub fn exhume_edge(&self, id: &Uuid) -> Option<Arc<RwLock<Edge>>> {
        self.edge.get(id).map(|edge| edge.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Edge>`.
    ///
    pub fn iter_edge(&self) -> impl Iterator<Item = Arc<RwLock<Edge>>> + '_ {
        self.edge.values().map(|edge| edge.0.clone())
    }

    /// Get the timestamp for Edge.
    ///
    pub fn edge_timestamp(&self, edge: &Edge) -> SystemTime {
        self.edge
            .get(&edge.id())
            .map(|edge| edge.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Glyph`] into the store.
    ///
    pub fn inter_glyph(&mut self, glyph: Arc<RwLock<Glyph>>) {
        let read = glyph.read().unwrap();
        self.glyph
            .insert(read.id, (glyph.clone(), SystemTime::now()));
    }

    /// Exhume [`Glyph`] from the store.
    ///
    pub fn exhume_glyph(&self, id: &Uuid) -> Option<Arc<RwLock<Glyph>>> {
        self.glyph.get(id).map(|glyph| glyph.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Glyph>`.
    ///
    pub fn iter_glyph(&self) -> impl Iterator<Item = Arc<RwLock<Glyph>>> + '_ {
        self.glyph.values().map(|glyph| glyph.0.clone())
    }

    /// Get the timestamp for Glyph.
    ///
    pub fn glyph_timestamp(&self, glyph: &Glyph) -> SystemTime {
        self.glyph
            .get(&glyph.id)
            .map(|glyph| glyph.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Line`] into the store.
    ///
    pub fn inter_line(&mut self, line: Arc<RwLock<Line>>) {
        let read = line.read().unwrap();
        self.line.insert(read.id, (line.clone(), SystemTime::now()));
    }

    /// Exhume [`Line`] from the store.
    ///
    pub fn exhume_line(&self, id: &Uuid) -> Option<Arc<RwLock<Line>>> {
        self.line.get(id).map(|line| line.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Line>`.
    ///
    pub fn iter_line(&self) -> impl Iterator<Item = Arc<RwLock<Line>>> + '_ {
        self.line.values().map(|line| line.0.clone())
    }

    /// Get the timestamp for Line.
    ///
    pub fn line_timestamp(&self, line: &Line) -> SystemTime {
        self.line
            .get(&line.id)
            .map(|line| line.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`LineSegment`] into the store.
    ///
    pub fn inter_line_segment(&mut self, line_segment: Arc<RwLock<LineSegment>>) {
        let read = line_segment.read().unwrap();
        self.line_segment
            .insert(read.id, (line_segment.clone(), SystemTime::now()));
    }

    /// Exhume [`LineSegment`] from the store.
    ///
    pub fn exhume_line_segment(&self, id: &Uuid) -> Option<Arc<RwLock<LineSegment>>> {
        self.line_segment
            .get(id)
            .map(|line_segment| line_segment.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LineSegment>`.
    ///
    pub fn iter_line_segment(&self) -> impl Iterator<Item = Arc<RwLock<LineSegment>>> + '_ {
        self.line_segment
            .values()
            .map(|line_segment| line_segment.0.clone())
    }

    /// Get the timestamp for LineSegment.
    ///
    pub fn line_segment_timestamp(&self, line_segment: &LineSegment) -> SystemTime {
        self.line_segment
            .get(&line_segment.id)
            .map(|line_segment| line_segment.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`LineSegmentPoint`] into the store.
    ///
    pub fn inter_line_segment_point(&mut self, line_segment_point: Arc<RwLock<LineSegmentPoint>>) {
        let read = line_segment_point.read().unwrap();
        self.line_segment_point
            .insert(read.id, (line_segment_point.clone(), SystemTime::now()));
    }

    /// Exhume [`LineSegmentPoint`] from the store.
    ///
    pub fn exhume_line_segment_point(&self, id: &Uuid) -> Option<Arc<RwLock<LineSegmentPoint>>> {
        self.line_segment_point
            .get(id)
            .map(|line_segment_point| line_segment_point.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LineSegmentPoint>`.
    ///
    pub fn iter_line_segment_point(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<LineSegmentPoint>>> + '_ {
        self.line_segment_point
            .values()
            .map(|line_segment_point| line_segment_point.0.clone())
    }

    /// Get the timestamp for LineSegmentPoint.
    ///
    pub fn line_segment_point_timestamp(
        &self,
        line_segment_point: &LineSegmentPoint,
    ) -> SystemTime {
        self.line_segment_point
            .get(&line_segment_point.id)
            .map(|line_segment_point| line_segment_point.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Point`] into the store.
    ///
    pub fn inter_point(&mut self, point: Arc<RwLock<Point>>) {
        let read = point.read().unwrap();
        self.point
            .insert(read.id, (point.clone(), SystemTime::now()));
    }

    /// Exhume [`Point`] from the store.
    ///
    pub fn exhume_point(&self, id: &Uuid) -> Option<Arc<RwLock<Point>>> {
        self.point.get(id).map(|point| point.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Point>`.
    ///
    pub fn iter_point(&self) -> impl Iterator<Item = Arc<RwLock<Point>>> + '_ {
        self.point.values().map(|point| point.0.clone())
    }

    /// Get the timestamp for Point.
    ///
    pub fn point_timestamp(&self, point: &Point) -> SystemTime {
        self.point
            .get(&point.id)
            .map(|point| point.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`RelationshipName`] into the store.
    ///
    pub fn inter_relationship_name(&mut self, relationship_name: Arc<RwLock<RelationshipName>>) {
        let read = relationship_name.read().unwrap();
        self.relationship_name
            .insert(read.id, (relationship_name.clone(), SystemTime::now()));
    }

    /// Exhume [`RelationshipName`] from the store.
    ///
    pub fn exhume_relationship_name(&self, id: &Uuid) -> Option<Arc<RwLock<RelationshipName>>> {
        self.relationship_name
            .get(id)
            .map(|relationship_name| relationship_name.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RelationshipName>`.
    ///
    pub fn iter_relationship_name(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<RelationshipName>>> + '_ {
        self.relationship_name
            .values()
            .map(|relationship_name| relationship_name.0.clone())
    }

    /// Get the timestamp for RelationshipName.
    ///
    pub fn relationship_name_timestamp(&self, relationship_name: &RelationshipName) -> SystemTime {
        self.relationship_name
            .get(&relationship_name.id)
            .map(|relationship_name| relationship_name.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`RelationshipPhrase`] into the store.
    ///
    pub fn inter_relationship_phrase(
        &mut self,
        relationship_phrase: Arc<RwLock<RelationshipPhrase>>,
    ) {
        let read = relationship_phrase.read().unwrap();
        self.relationship_phrase
            .insert(read.id, (relationship_phrase.clone(), SystemTime::now()));
    }

    /// Exhume [`RelationshipPhrase`] from the store.
    ///
    pub fn exhume_relationship_phrase(&self, id: &Uuid) -> Option<Arc<RwLock<RelationshipPhrase>>> {
        self.relationship_phrase
            .get(id)
            .map(|relationship_phrase| relationship_phrase.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RelationshipPhrase>`.
    ///
    pub fn iter_relationship_phrase(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<RelationshipPhrase>>> + '_ {
        self.relationship_phrase
            .values()
            .map(|relationship_phrase| relationship_phrase.0.clone())
    }

    /// Get the timestamp for RelationshipPhrase.
    ///
    pub fn relationship_phrase_timestamp(
        &self,
        relationship_phrase: &RelationshipPhrase,
    ) -> SystemTime {
        self.relationship_phrase
            .get(&relationship_phrase.id)
            .map(|relationship_phrase| relationship_phrase.1)
            .unwrap_or(SystemTime::now())
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::merlin-object-store-persistence"}}}
    /// Persist the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automagic git integration as an option.
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        fs::create_dir_all(&path)?;

        let bin_path = path.clone().join("merlin.bin");
        let mut bin_file = fs::File::create(bin_path)?;
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        bin_file.write_all(&encoded)?;

        let path = path.join("merlin.json");
        fs::create_dir_all(&path)?;

        // Persist Anchor.
        {
            let path = path.join("anchor");
            fs::create_dir_all(&path)?;
            for anchor_tuple in self.anchor.values() {
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
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.anchor.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Bisection.
        {
            let path = path.join("bisection");
            fs::create_dir_all(&path)?;
            for bisection_tuple in self.bisection.values() {
                let path = path.join(format!("{}.json", bisection_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Bisection>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != bisection_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &bisection_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &bisection_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.bisection.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Box.
        {
            let path = path.join("x_box");
            fs::create_dir_all(&path)?;
            for x_box_tuple in self.x_box.values() {
                let path = path.join(format!("{}.json", x_box_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<XBox>>, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != x_box_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &x_box_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &x_box_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.x_box.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Edge.
        {
            let path = path.join("edge");
            fs::create_dir_all(&path)?;
            for edge_tuple in self.edge.values() {
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
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.edge.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Glyph.
        {
            let path = path.join("glyph");
            fs::create_dir_all(&path)?;
            for glyph_tuple in self.glyph.values() {
                let path = path.join(format!("{}.json", glyph_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Glyph>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != glyph_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &glyph_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &glyph_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.glyph.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Line.
        {
            let path = path.join("line");
            fs::create_dir_all(&path)?;
            for line_tuple in self.line.values() {
                let path = path.join(format!("{}.json", line_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Line>>, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != line_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &line_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &line_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.line.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Line Segment.
        {
            let path = path.join("line_segment");
            fs::create_dir_all(&path)?;
            for line_segment_tuple in self.line_segment.values() {
                let path = path.join(format!("{}.json", line_segment_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<LineSegment>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != line_segment_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &line_segment_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &line_segment_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.line_segment.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Line Segment Point.
        {
            let path = path.join("line_segment_point");
            fs::create_dir_all(&path)?;
            for line_segment_point_tuple in self.line_segment_point.values() {
                let path = path.join(format!(
                    "{}.json",
                    line_segment_point_tuple.0.read().unwrap().id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<LineSegmentPoint>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != line_segment_point_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &line_segment_point_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &line_segment_point_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.line_segment_point.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Point.
        {
            let path = path.join("point");
            fs::create_dir_all(&path)?;
            for point_tuple in self.point.values() {
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
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.point.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Relationship Name.
        {
            let path = path.join("relationship_name");
            fs::create_dir_all(&path)?;
            for relationship_name_tuple in self.relationship_name.values() {
                let path = path.join(format!(
                    "{}.json",
                    relationship_name_tuple.0.read().unwrap().id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<RelationshipName>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != relationship_name_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &relationship_name_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &relationship_name_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.relationship_name.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Relationship Phrase.
        {
            let path = path.join("relationship_phrase");
            fs::create_dir_all(&path)?;
            for relationship_phrase_tuple in self.relationship_phrase.values() {
                let path = path.join(format!(
                    "{}.json",
                    relationship_phrase_tuple.0.read().unwrap().id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<RelationshipPhrase>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != relationship_phrase_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &relationship_phrase_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &relationship_phrase_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.relationship_phrase.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Load the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automagic git integration as an option.
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let path = path.join("merlin.json");

        let mut store = Self::new();

        // Load Anchor.
        {
            let path = path.join("anchor");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let anchor: (Arc<RwLock<Anchor>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .anchor
                    .insert(anchor.0.read().unwrap().id, anchor.clone());
            }
        }

        // Load Bisection.
        {
            let path = path.join("bisection");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let bisection: (Arc<RwLock<Bisection>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .bisection
                    .insert(bisection.0.read().unwrap().id, bisection.clone());
            }
        }

        // Load Box.
        {
            let path = path.join("x_box");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let x_box: (Arc<RwLock<XBox>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .x_box
                    .insert(x_box.0.read().unwrap().id, x_box.clone());
            }
        }

        // Load Edge.
        {
            let path = path.join("edge");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let edge: (Arc<RwLock<Edge>>, SystemTime) = serde_json::from_reader(reader)?;
                store.edge.insert(edge.0.read().unwrap().id(), edge.clone());
            }
        }

        // Load Glyph.
        {
            let path = path.join("glyph");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let glyph: (Arc<RwLock<Glyph>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .glyph
                    .insert(glyph.0.read().unwrap().id, glyph.clone());
            }
        }

        // Load Line.
        {
            let path = path.join("line");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let line: (Arc<RwLock<Line>>, SystemTime) = serde_json::from_reader(reader)?;
                store.line.insert(line.0.read().unwrap().id, line.clone());
            }
        }

        // Load Line Segment.
        {
            let path = path.join("line_segment");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let line_segment: (Arc<RwLock<LineSegment>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .line_segment
                    .insert(line_segment.0.read().unwrap().id, line_segment.clone());
            }
        }

        // Load Line Segment Point.
        {
            let path = path.join("line_segment_point");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let line_segment_point: (Arc<RwLock<LineSegmentPoint>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.line_segment_point.insert(
                    line_segment_point.0.read().unwrap().id,
                    line_segment_point.clone(),
                );
            }
        }

        // Load Point.
        {
            let path = path.join("point");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let point: (Arc<RwLock<Point>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .point
                    .insert(point.0.read().unwrap().id, point.clone());
            }
        }

        // Load Relationship Name.
        {
            let path = path.join("relationship_name");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let relationship_name: (Arc<RwLock<RelationshipName>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.relationship_name.insert(
                    relationship_name.0.read().unwrap().id,
                    relationship_name.clone(),
                );
            }
        }

        // Load Relationship Phrase.
        {
            let path = path.join("relationship_phrase");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let relationship_phrase: (Arc<RwLock<RelationshipPhrase>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.relationship_phrase.insert(
                    relationship_phrase.0.read().unwrap().id,
                    relationship_phrase.clone(),
                );
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
