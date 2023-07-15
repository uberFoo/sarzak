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
};

use rustc_hash::FxHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v2::merlin::types::{
    Anchor, Bisection, Edge, Glyph, Line, LineSegment, LineSegmentPoint, Point, RelationshipName,
    RelationshipPhrase, XBox, BOTTOM, LEFT, RIGHT, TOP,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    anchor: HashMap<Uuid, Anchor>,
    bisection: HashMap<Uuid, Bisection>,
    x_box: HashMap<Uuid, XBox>,
    edge: HashMap<Uuid, Edge>,
    glyph: HashMap<Uuid, Glyph>,
    line: HashMap<Uuid, Line>,
    line_segment: HashMap<Uuid, LineSegment>,
    line_segment_point: HashMap<Uuid, LineSegmentPoint>,
    point: HashMap<Uuid, Point>,
    relationship_name: HashMap<Uuid, RelationshipName>,
    relationship_phrase: HashMap<Uuid, RelationshipPhrase>,
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
    /// Inter (insert) [`Anchor`] into the store.
    ///
    pub fn inter_anchor(&mut self, anchor: Anchor) {
        self.anchor.insert(anchor.id, anchor);
    }

    /// Exhume (get) [`Anchor`] from the store.
    ///
    pub fn exhume_anchor(&self, id: &Uuid) -> Option<&Anchor> {
        self.anchor.get(id)
    }

    /// Exorcise (remove) [`Anchor`] from the store.
    ///
    pub fn exorcise_anchor(&mut self, id: &Uuid) -> Option<Anchor> {
        self.anchor.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Anchor>`.
    ///
    pub fn iter_anchor(&self) -> impl Iterator<Item = &Anchor> {
        self.anchor.values()
    }

    /// Inter (insert) [`Bisection`] into the store.
    ///
    pub fn inter_bisection(&mut self, bisection: Bisection) {
        self.bisection.insert(bisection.id, bisection);
    }

    /// Exhume (get) [`Bisection`] from the store.
    ///
    pub fn exhume_bisection(&self, id: &Uuid) -> Option<&Bisection> {
        self.bisection.get(id)
    }

    /// Exorcise (remove) [`Bisection`] from the store.
    ///
    pub fn exorcise_bisection(&mut self, id: &Uuid) -> Option<Bisection> {
        self.bisection.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Bisection>`.
    ///
    pub fn iter_bisection(&self) -> impl Iterator<Item = &Bisection> {
        self.bisection.values()
    }

    /// Inter (insert) [`XBox`] into the store.
    ///
    pub fn inter_x_box(&mut self, x_box: XBox) {
        self.x_box.insert(x_box.id, x_box);
    }

    /// Exhume (get) [`XBox`] from the store.
    ///
    pub fn exhume_x_box(&self, id: &Uuid) -> Option<&XBox> {
        self.x_box.get(id)
    }

    /// Exorcise (remove) [`XBox`] from the store.
    ///
    pub fn exorcise_x_box(&mut self, id: &Uuid) -> Option<XBox> {
        self.x_box.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XBox>`.
    ///
    pub fn iter_x_box(&self) -> impl Iterator<Item = &XBox> {
        self.x_box.values()
    }

    /// Inter (insert) [`Edge`] into the store.
    ///
    pub fn inter_edge(&mut self, edge: Edge) {
        self.edge.insert(edge.id(), edge);
    }

    /// Exhume (get) [`Edge`] from the store.
    ///
    pub fn exhume_edge(&self, id: &Uuid) -> Option<&Edge> {
        self.edge.get(id)
    }

    /// Exorcise (remove) [`Edge`] from the store.
    ///
    pub fn exorcise_edge(&mut self, id: &Uuid) -> Option<Edge> {
        self.edge.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Edge>`.
    ///
    pub fn iter_edge(&self) -> impl Iterator<Item = &Edge> {
        self.edge.values()
    }

    /// Inter (insert) [`Glyph`] into the store.
    ///
    pub fn inter_glyph(&mut self, glyph: Glyph) {
        self.glyph.insert(glyph.id, glyph);
    }

    /// Exhume (get) [`Glyph`] from the store.
    ///
    pub fn exhume_glyph(&self, id: &Uuid) -> Option<&Glyph> {
        self.glyph.get(id)
    }

    /// Exorcise (remove) [`Glyph`] from the store.
    ///
    pub fn exorcise_glyph(&mut self, id: &Uuid) -> Option<Glyph> {
        self.glyph.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Glyph>`.
    ///
    pub fn iter_glyph(&self) -> impl Iterator<Item = &Glyph> {
        self.glyph.values()
    }

    /// Inter (insert) [`Line`] into the store.
    ///
    pub fn inter_line(&mut self, line: Line) {
        self.line.insert(line.id, line);
    }

    /// Exhume (get) [`Line`] from the store.
    ///
    pub fn exhume_line(&self, id: &Uuid) -> Option<&Line> {
        self.line.get(id)
    }

    /// Exorcise (remove) [`Line`] from the store.
    ///
    pub fn exorcise_line(&mut self, id: &Uuid) -> Option<Line> {
        self.line.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Line>`.
    ///
    pub fn iter_line(&self) -> impl Iterator<Item = &Line> {
        self.line.values()
    }

    /// Inter (insert) [`LineSegment`] into the store.
    ///
    pub fn inter_line_segment(&mut self, line_segment: LineSegment) {
        self.line_segment.insert(line_segment.id, line_segment);
    }

    /// Exhume (get) [`LineSegment`] from the store.
    ///
    pub fn exhume_line_segment(&self, id: &Uuid) -> Option<&LineSegment> {
        self.line_segment.get(id)
    }

    /// Exorcise (remove) [`LineSegment`] from the store.
    ///
    pub fn exorcise_line_segment(&mut self, id: &Uuid) -> Option<LineSegment> {
        self.line_segment.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LineSegment>`.
    ///
    pub fn iter_line_segment(&self) -> impl Iterator<Item = &LineSegment> {
        self.line_segment.values()
    }

    /// Inter (insert) [`LineSegmentPoint`] into the store.
    ///
    pub fn inter_line_segment_point(&mut self, line_segment_point: LineSegmentPoint) {
        self.line_segment_point
            .insert(line_segment_point.id, line_segment_point);
    }

    /// Exhume (get) [`LineSegmentPoint`] from the store.
    ///
    pub fn exhume_line_segment_point(&self, id: &Uuid) -> Option<&LineSegmentPoint> {
        self.line_segment_point.get(id)
    }

    /// Exorcise (remove) [`LineSegmentPoint`] from the store.
    ///
    pub fn exorcise_line_segment_point(&mut self, id: &Uuid) -> Option<LineSegmentPoint> {
        self.line_segment_point.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LineSegmentPoint>`.
    ///
    pub fn iter_line_segment_point(&self) -> impl Iterator<Item = &LineSegmentPoint> {
        self.line_segment_point.values()
    }

    /// Inter (insert) [`Point`] into the store.
    ///
    pub fn inter_point(&mut self, point: Point) {
        self.point.insert(point.id, point);
    }

    /// Exhume (get) [`Point`] from the store.
    ///
    pub fn exhume_point(&self, id: &Uuid) -> Option<&Point> {
        self.point.get(id)
    }

    /// Exorcise (remove) [`Point`] from the store.
    ///
    pub fn exorcise_point(&mut self, id: &Uuid) -> Option<Point> {
        self.point.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Point>`.
    ///
    pub fn iter_point(&self) -> impl Iterator<Item = &Point> {
        self.point.values()
    }

    /// Inter (insert) [`RelationshipName`] into the store.
    ///
    pub fn inter_relationship_name(&mut self, relationship_name: RelationshipName) {
        self.relationship_name
            .insert(relationship_name.id, relationship_name);
    }

    /// Exhume (get) [`RelationshipName`] from the store.
    ///
    pub fn exhume_relationship_name(&self, id: &Uuid) -> Option<&RelationshipName> {
        self.relationship_name.get(id)
    }

    /// Exorcise (remove) [`RelationshipName`] from the store.
    ///
    pub fn exorcise_relationship_name(&mut self, id: &Uuid) -> Option<RelationshipName> {
        self.relationship_name.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RelationshipName>`.
    ///
    pub fn iter_relationship_name(&self) -> impl Iterator<Item = &RelationshipName> {
        self.relationship_name.values()
    }

    /// Inter (insert) [`RelationshipPhrase`] into the store.
    ///
    pub fn inter_relationship_phrase(&mut self, relationship_phrase: RelationshipPhrase) {
        self.relationship_phrase
            .insert(relationship_phrase.id, relationship_phrase);
    }

    /// Exhume (get) [`RelationshipPhrase`] from the store.
    ///
    pub fn exhume_relationship_phrase(&self, id: &Uuid) -> Option<&RelationshipPhrase> {
        self.relationship_phrase.get(id)
    }

    /// Exorcise (remove) [`RelationshipPhrase`] from the store.
    ///
    pub fn exorcise_relationship_phrase(&mut self, id: &Uuid) -> Option<RelationshipPhrase> {
        self.relationship_phrase.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RelationshipPhrase>`.
    ///
    pub fn iter_relationship_phrase(&self) -> impl Iterator<Item = &RelationshipPhrase> {
        self.relationship_phrase.values()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::merlin-object-store-persistence"}}}
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

        let path = path.join("merlin.json");
        fs::create_dir_all(&path)?;

        // Persist Anchor.
        {
            let path = path.join("anchor");
            fs::create_dir_all(&path)?;
            for anchor in self.anchor.values() {
                let path = path.join(format!("{}.json", anchor.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &anchor)?;
            }
        }

        // Persist Bisection.
        {
            let path = path.join("bisection");
            fs::create_dir_all(&path)?;
            for bisection in self.bisection.values() {
                let path = path.join(format!("{}.json", bisection.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &bisection)?;
            }
        }

        // Persist Box.
        {
            let path = path.join("x_box");
            fs::create_dir_all(&path)?;
            for x_box in self.x_box.values() {
                let path = path.join(format!("{}.json", x_box.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_box)?;
            }
        }

        // Persist Edge.
        {
            let path = path.join("edge");
            fs::create_dir_all(&path)?;
            for edge in self.edge.values() {
                let path = path.join(format!("{}.json", edge.id()));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &edge)?;
            }
        }

        // Persist Glyph.
        {
            let path = path.join("glyph");
            fs::create_dir_all(&path)?;
            for glyph in self.glyph.values() {
                let path = path.join(format!("{}.json", glyph.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &glyph)?;
            }
        }

        // Persist Line.
        {
            let path = path.join("line");
            fs::create_dir_all(&path)?;
            for line in self.line.values() {
                let path = path.join(format!("{}.json", line.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &line)?;
            }
        }

        // Persist Line Segment.
        {
            let path = path.join("line_segment");
            fs::create_dir_all(&path)?;
            for line_segment in self.line_segment.values() {
                let path = path.join(format!("{}.json", line_segment.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &line_segment)?;
            }
        }

        // Persist Line Segment Point.
        {
            let path = path.join("line_segment_point");
            fs::create_dir_all(&path)?;
            for line_segment_point in self.line_segment_point.values() {
                let path = path.join(format!("{}.json", line_segment_point.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &line_segment_point)?;
            }
        }

        // Persist Point.
        {
            let path = path.join("point");
            fs::create_dir_all(&path)?;
            for point in self.point.values() {
                let path = path.join(format!("{}.json", point.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &point)?;
            }
        }

        // Persist Relationship Name.
        {
            let path = path.join("relationship_name");
            fs::create_dir_all(&path)?;
            for relationship_name in self.relationship_name.values() {
                let path = path.join(format!("{}.json", relationship_name.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &relationship_name)?;
            }
        }

        // Persist Relationship Phrase.
        {
            let path = path.join("relationship_phrase");
            fs::create_dir_all(&path)?;
            for relationship_phrase in self.relationship_phrase.values() {
                let path = path.join(format!("{}.json", relationship_phrase.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &relationship_phrase)?;
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
        let path = path.join("merlin.json");

        let mut store = Self::new();

        // Load Anchor.
        {
            let path = path.join("anchor");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let anchor: Anchor = serde_json::from_reader(reader)?;
                store.anchor.insert(anchor.id, anchor);
            }
        }

        // Load Bisection.
        {
            let path = path.join("bisection");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let bisection: Bisection = serde_json::from_reader(reader)?;
                store.bisection.insert(bisection.id, bisection);
            }
        }

        // Load Box.
        {
            let path = path.join("x_box");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let x_box: XBox = serde_json::from_reader(reader)?;
                store.x_box.insert(x_box.id, x_box);
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
                let edge: Edge = serde_json::from_reader(reader)?;
                store.edge.insert(edge.id(), edge);
            }
        }

        // Load Glyph.
        {
            let path = path.join("glyph");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let glyph: Glyph = serde_json::from_reader(reader)?;
                store.glyph.insert(glyph.id, glyph);
            }
        }

        // Load Line.
        {
            let path = path.join("line");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let line: Line = serde_json::from_reader(reader)?;
                store.line.insert(line.id, line);
            }
        }

        // Load Line Segment.
        {
            let path = path.join("line_segment");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let line_segment: LineSegment = serde_json::from_reader(reader)?;
                store.line_segment.insert(line_segment.id, line_segment);
            }
        }

        // Load Line Segment Point.
        {
            let path = path.join("line_segment_point");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let line_segment_point: LineSegmentPoint = serde_json::from_reader(reader)?;
                store
                    .line_segment_point
                    .insert(line_segment_point.id, line_segment_point);
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
                let point: Point = serde_json::from_reader(reader)?;
                store.point.insert(point.id, point);
            }
        }

        // Load Relationship Name.
        {
            let path = path.join("relationship_name");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let relationship_name: RelationshipName = serde_json::from_reader(reader)?;
                store
                    .relationship_name
                    .insert(relationship_name.id, relationship_name);
            }
        }

        // Load Relationship Phrase.
        {
            let path = path.join("relationship_phrase");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let relationship_phrase: RelationshipPhrase = serde_json::from_reader(reader)?;
                store
                    .relationship_phrase
                    .insert(relationship_phrase.id, relationship_phrase);
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
