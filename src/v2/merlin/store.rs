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
    anchor: HashMap<Uuid, (Anchor, SystemTime)>,
    bisection: HashMap<Uuid, (Bisection, SystemTime)>,
    x_box: HashMap<Uuid, (XBox, SystemTime)>,
    edge: HashMap<Uuid, (Edge, SystemTime)>,
    glyph: HashMap<Uuid, (Glyph, SystemTime)>,
    line: HashMap<Uuid, (Line, SystemTime)>,
    line_segment: HashMap<Uuid, (LineSegment, SystemTime)>,
    line_segment_point: HashMap<Uuid, (LineSegmentPoint, SystemTime)>,
    point: HashMap<Uuid, (Point, SystemTime)>,
    relationship_name: HashMap<Uuid, (RelationshipName, SystemTime)>,
    relationship_phrase: HashMap<Uuid, (RelationshipPhrase, SystemTime)>,
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
        store.inter_edge(Edge::Bottom(BOTTOM));
        store.inter_edge(Edge::Left(LEFT));
        store.inter_edge(Edge::Right(RIGHT));
        store.inter_edge(Edge::Top(TOP));

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::merlin-object-store-methods"}}}
    /// Inter [`Anchor`] into the store.
    ///
    pub fn inter_anchor(&mut self, anchor: Anchor) {
        self.anchor.insert(anchor.id, (anchor, SystemTime::now()));
    }

    /// Exhume [`Anchor`] from the store.
    ///
    pub fn exhume_anchor(&self, id: &Uuid) -> Option<&Anchor> {
        self.anchor.get(id).map(|anchor| &anchor.0)
    }

    /// Exhume [`Anchor`] from the store — mutably.
    ///
    pub fn exhume_anchor_mut(&mut self, id: &Uuid) -> Option<&mut Anchor> {
        self.anchor.get_mut(id).map(|anchor| &mut anchor.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Anchor>`.
    ///
    pub fn iter_anchor(&self) -> impl Iterator<Item = &Anchor> {
        self.anchor.values().map(|anchor| &anchor.0)
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
    pub fn inter_bisection(&mut self, bisection: Bisection) {
        self.bisection
            .insert(bisection.id, (bisection, SystemTime::now()));
    }

    /// Exhume [`Bisection`] from the store.
    ///
    pub fn exhume_bisection(&self, id: &Uuid) -> Option<&Bisection> {
        self.bisection.get(id).map(|bisection| &bisection.0)
    }

    /// Exhume [`Bisection`] from the store — mutably.
    ///
    pub fn exhume_bisection_mut(&mut self, id: &Uuid) -> Option<&mut Bisection> {
        self.bisection.get_mut(id).map(|bisection| &mut bisection.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Bisection>`.
    ///
    pub fn iter_bisection(&self) -> impl Iterator<Item = &Bisection> {
        self.bisection.values().map(|bisection| &bisection.0)
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
    pub fn inter_x_box(&mut self, x_box: XBox) {
        self.x_box.insert(x_box.id, (x_box, SystemTime::now()));
    }

    /// Exhume [`XBox`] from the store.
    ///
    pub fn exhume_x_box(&self, id: &Uuid) -> Option<&XBox> {
        self.x_box.get(id).map(|x_box| &x_box.0)
    }

    /// Exhume [`XBox`] from the store — mutably.
    ///
    pub fn exhume_x_box_mut(&mut self, id: &Uuid) -> Option<&mut XBox> {
        self.x_box.get_mut(id).map(|x_box| &mut x_box.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XBox>`.
    ///
    pub fn iter_x_box(&self) -> impl Iterator<Item = &XBox> {
        self.x_box.values().map(|x_box| &x_box.0)
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
    pub fn inter_edge(&mut self, edge: Edge) {
        self.edge.insert(edge.id(), (edge, SystemTime::now()));
    }

    /// Exhume [`Edge`] from the store.
    ///
    pub fn exhume_edge(&self, id: &Uuid) -> Option<&Edge> {
        self.edge.get(id).map(|edge| &edge.0)
    }

    /// Exhume [`Edge`] from the store — mutably.
    ///
    pub fn exhume_edge_mut(&mut self, id: &Uuid) -> Option<&mut Edge> {
        self.edge.get_mut(id).map(|edge| &mut edge.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Edge>`.
    ///
    pub fn iter_edge(&self) -> impl Iterator<Item = &Edge> {
        self.edge.values().map(|edge| &edge.0)
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
    pub fn inter_glyph(&mut self, glyph: Glyph) {
        self.glyph.insert(glyph.id, (glyph, SystemTime::now()));
    }

    /// Exhume [`Glyph`] from the store.
    ///
    pub fn exhume_glyph(&self, id: &Uuid) -> Option<&Glyph> {
        self.glyph.get(id).map(|glyph| &glyph.0)
    }

    /// Exhume [`Glyph`] from the store — mutably.
    ///
    pub fn exhume_glyph_mut(&mut self, id: &Uuid) -> Option<&mut Glyph> {
        self.glyph.get_mut(id).map(|glyph| &mut glyph.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Glyph>`.
    ///
    pub fn iter_glyph(&self) -> impl Iterator<Item = &Glyph> {
        self.glyph.values().map(|glyph| &glyph.0)
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
    pub fn inter_line(&mut self, line: Line) {
        self.line.insert(line.id, (line, SystemTime::now()));
    }

    /// Exhume [`Line`] from the store.
    ///
    pub fn exhume_line(&self, id: &Uuid) -> Option<&Line> {
        self.line.get(id).map(|line| &line.0)
    }

    /// Exhume [`Line`] from the store — mutably.
    ///
    pub fn exhume_line_mut(&mut self, id: &Uuid) -> Option<&mut Line> {
        self.line.get_mut(id).map(|line| &mut line.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Line>`.
    ///
    pub fn iter_line(&self) -> impl Iterator<Item = &Line> {
        self.line.values().map(|line| &line.0)
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
    pub fn inter_line_segment(&mut self, line_segment: LineSegment) {
        self.line_segment
            .insert(line_segment.id, (line_segment, SystemTime::now()));
    }

    /// Exhume [`LineSegment`] from the store.
    ///
    pub fn exhume_line_segment(&self, id: &Uuid) -> Option<&LineSegment> {
        self.line_segment
            .get(id)
            .map(|line_segment| &line_segment.0)
    }

    /// Exhume [`LineSegment`] from the store — mutably.
    ///
    pub fn exhume_line_segment_mut(&mut self, id: &Uuid) -> Option<&mut LineSegment> {
        self.line_segment
            .get_mut(id)
            .map(|line_segment| &mut line_segment.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LineSegment>`.
    ///
    pub fn iter_line_segment(&self) -> impl Iterator<Item = &LineSegment> {
        self.line_segment
            .values()
            .map(|line_segment| &line_segment.0)
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
    pub fn inter_line_segment_point(&mut self, line_segment_point: LineSegmentPoint) {
        self.line_segment_point.insert(
            line_segment_point.id,
            (line_segment_point, SystemTime::now()),
        );
    }

    /// Exhume [`LineSegmentPoint`] from the store.
    ///
    pub fn exhume_line_segment_point(&self, id: &Uuid) -> Option<&LineSegmentPoint> {
        self.line_segment_point
            .get(id)
            .map(|line_segment_point| &line_segment_point.0)
    }

    /// Exhume [`LineSegmentPoint`] from the store — mutably.
    ///
    pub fn exhume_line_segment_point_mut(&mut self, id: &Uuid) -> Option<&mut LineSegmentPoint> {
        self.line_segment_point
            .get_mut(id)
            .map(|line_segment_point| &mut line_segment_point.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LineSegmentPoint>`.
    ///
    pub fn iter_line_segment_point(&self) -> impl Iterator<Item = &LineSegmentPoint> {
        self.line_segment_point
            .values()
            .map(|line_segment_point| &line_segment_point.0)
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
    pub fn inter_point(&mut self, point: Point) {
        self.point.insert(point.id, (point, SystemTime::now()));
    }

    /// Exhume [`Point`] from the store.
    ///
    pub fn exhume_point(&self, id: &Uuid) -> Option<&Point> {
        self.point.get(id).map(|point| &point.0)
    }

    /// Exhume [`Point`] from the store — mutably.
    ///
    pub fn exhume_point_mut(&mut self, id: &Uuid) -> Option<&mut Point> {
        self.point.get_mut(id).map(|point| &mut point.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Point>`.
    ///
    pub fn iter_point(&self) -> impl Iterator<Item = &Point> {
        self.point.values().map(|point| &point.0)
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
    pub fn inter_relationship_name(&mut self, relationship_name: RelationshipName) {
        self.relationship_name
            .insert(relationship_name.id, (relationship_name, SystemTime::now()));
    }

    /// Exhume [`RelationshipName`] from the store.
    ///
    pub fn exhume_relationship_name(&self, id: &Uuid) -> Option<&RelationshipName> {
        self.relationship_name
            .get(id)
            .map(|relationship_name| &relationship_name.0)
    }

    /// Exhume [`RelationshipName`] from the store — mutably.
    ///
    pub fn exhume_relationship_name_mut(&mut self, id: &Uuid) -> Option<&mut RelationshipName> {
        self.relationship_name
            .get_mut(id)
            .map(|relationship_name| &mut relationship_name.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RelationshipName>`.
    ///
    pub fn iter_relationship_name(&self) -> impl Iterator<Item = &RelationshipName> {
        self.relationship_name
            .values()
            .map(|relationship_name| &relationship_name.0)
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
    pub fn inter_relationship_phrase(&mut self, relationship_phrase: RelationshipPhrase) {
        self.relationship_phrase.insert(
            relationship_phrase.id,
            (relationship_phrase, SystemTime::now()),
        );
    }

    /// Exhume [`RelationshipPhrase`] from the store.
    ///
    pub fn exhume_relationship_phrase(&self, id: &Uuid) -> Option<&RelationshipPhrase> {
        self.relationship_phrase
            .get(id)
            .map(|relationship_phrase| &relationship_phrase.0)
    }

    /// Exhume [`RelationshipPhrase`] from the store — mutably.
    ///
    pub fn exhume_relationship_phrase_mut(&mut self, id: &Uuid) -> Option<&mut RelationshipPhrase> {
        self.relationship_phrase
            .get_mut(id)
            .map(|relationship_phrase| &mut relationship_phrase.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RelationshipPhrase>`.
    ///
    pub fn iter_relationship_phrase(&self) -> impl Iterator<Item = &RelationshipPhrase> {
        self.relationship_phrase
            .values()
            .map(|relationship_phrase| &relationship_phrase.0)
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
                let path = path.join(format!("{}.json", anchor_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Anchor, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != anchor_tuple.0 {
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
                let path = path.join(format!("{}.json", bisection_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Bisection, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != bisection_tuple.0 {
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
                let path = path.join(format!("{}.json", x_box_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (XBox, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != x_box_tuple.0 {
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
                let path = path.join(format!("{}.json", edge_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Edge, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != edge_tuple.0 {
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
                let path = path.join(format!("{}.json", glyph_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Glyph, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != glyph_tuple.0 {
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
                let path = path.join(format!("{}.json", line_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Line, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != line_tuple.0 {
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
                let path = path.join(format!("{}.json", line_segment_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (LineSegment, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != line_segment_tuple.0 {
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
                let path = path.join(format!("{}.json", line_segment_point_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (LineSegmentPoint, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != line_segment_point_tuple.0 {
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
                let path = path.join(format!("{}.json", point_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Point, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != point_tuple.0 {
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
                let path = path.join(format!("{}.json", relationship_name_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (RelationshipName, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != relationship_name_tuple.0 {
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
                let path = path.join(format!("{}.json", relationship_phrase_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (RelationshipPhrase, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0 != relationship_phrase_tuple.0 {
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
                let anchor: (Anchor, SystemTime) = serde_json::from_reader(reader)?;
                store.anchor.insert(anchor.0.id, anchor);
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
                let bisection: (Bisection, SystemTime) = serde_json::from_reader(reader)?;
                store.bisection.insert(bisection.0.id, bisection);
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
                let x_box: (XBox, SystemTime) = serde_json::from_reader(reader)?;
                store.x_box.insert(x_box.0.id, x_box);
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
                let edge: (Edge, SystemTime) = serde_json::from_reader(reader)?;
                store.edge.insert(edge.0.id(), edge);
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
                let glyph: (Glyph, SystemTime) = serde_json::from_reader(reader)?;
                store.glyph.insert(glyph.0.id, glyph);
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
                let line: (Line, SystemTime) = serde_json::from_reader(reader)?;
                store.line.insert(line.0.id, line);
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
                let line_segment: (LineSegment, SystemTime) = serde_json::from_reader(reader)?;
                store.line_segment.insert(line_segment.0.id, line_segment);
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
                let line_segment_point: (LineSegmentPoint, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .line_segment_point
                    .insert(line_segment_point.0.id, line_segment_point);
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
                let point: (Point, SystemTime) = serde_json::from_reader(reader)?;
                store.point.insert(point.0.id, point);
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
                let relationship_name: (RelationshipName, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .relationship_name
                    .insert(relationship_name.0.id, relationship_name);
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
                let relationship_phrase: (RelationshipPhrase, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .relationship_phrase
                    .insert(relationship_phrase.0.id, relationship_phrase);
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
