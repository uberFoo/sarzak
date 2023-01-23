//! Types for instances of the "Drawing" domain
//! # Domain Description
//!
//! Domain for drawing boxen and lines.
//!
//!
//! # Contents
//!
//! The following types are defined herein:
//!    * [`Anchor`]
//!    * [`AssociativeUi`]
//!    * [`BinaryUi`]
//!    * [`BOTTOM`]
//!    * [`Edge`]
//!    * [`IsaUi`]
//!    * [`LEFT`]
//!    * [`ObjectEdge`]
//!    * [`ObjectUi`]
//!    * [`Point`]
//!    * [`RelationshipUi`]
//!    * [`RIGHT`]
//!    * [`SubtypeAnchors`]
//!    * [`TOP`]
//!
//! # Generated Code -- edit _with care_.
//!
//! Don't mess with anything between `{"magic":"","kind":"CriticalBlockBegin"}`
//! and `{"magic":"","kind":"CriticalBlockEnd"}`. Otherwise, you should be free
//! to go wild. Happy hacking!
//!
//! Use the following invocation to reproduce:
// {"magic":"","kind":"IgnoreBlockBegin"}
//! ```shell
//!  sarzak gen sarzak -i true -m true -d true -e true
//! ```
// {"magic":"","kind":"IgnoreBlockEnd"}
// {"magic":"","version":"0.5.0"}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"imports", "is_uber": true}}}
use crate::drawing::store::ObjectStore;
use crate::drawing::UUID_NS;
use nut::codegen::{DrawingObjectStore, Extrude};
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"imports"}}}

// Imported Objects
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"imported-objects"}}}
use crate::sarzak::types::Associative;
use crate::sarzak::types::Binary;
use crate::sarzak::types::Isa;
use crate::sarzak::types::Object;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"imported-objects"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"context-extrude_impl", "is_uber": true}}}
pub(crate) struct Context<'a> {
    from: &'a DrawingObjectStore,
    to: &'a mut ObjectStore,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"context-extrude_impl"}}}

/// An anchor, or anchor point, is the location where an arrow from a relationship attached
/// to an object.
///
/// Rather than storing the `x` and `y` coordinates of where the anchor attaches, we are related
/// to an [Edge], which is related to a box, which is related to the [Object] to which we are
/// attached. This of course completes the circuit from the [Relationship] for which we are
/// drawing the lines in the first place.
///
/// Anchor also contains a direction, so that we know the orientation to draw the arrows. Finally
///, there is an offset, which is a point that describes the offset from the anchor for the
/// relationship phrase.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"anchor-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Anchor {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub edge: `Edge`,
    ///
    pub edge: Uuid,
    /// pub location: `Point`,
    ///
    pub location: Uuid,
    /// pub offset: `Point`,
    ///
    pub offset: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"anchor-struct-definition"}}}

impl Anchor {
    // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"anchor-new_impl"}}}
    /// Inter a new Anchor and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::drawing::Anchor;
    /// # use sarzak::drawing::Edge;
    /// # use sarzak::drawing::Point;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    ///
    /// let point_kar = Point::new(&mut store, 42, 42);
    /// let point_ewy = Point::new(&mut store, 42, 42);
    /// let edge_onp = Edge::test_default(&mut store);
    ///
    /// let anchor = Anchor::new(&mut store, &point_kar, &point_ewy, &edge_onp);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, location: &Point, offset: &Point, edge: &Edge) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{:?}::{:?}::{:?}::", location, offset, edge,).as_bytes(),
        );
        let new = Self {
            id,
            location: location.id,
            offset: offset.id,
            edge: edge.get_id(),
        };

        store.inter_anchor(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"anchor-new_impl"}}}
}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"anchor-extrude_impl"}}}
impl Extrude<nut::drawing::Anchor, Context<'_>> for Anchor {
    fn extrude(orig: nut::drawing::Anchor, context: &mut Context<'_>) -> Self {
        let Context { from, ref mut to } = context;

        Self::default()
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"anchor-extrude_impl"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"associative_ui-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AssociativeUi {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// Imported from the sarzak domain.
    /// [`nut::sarzak::Associative`]
    ///
    pub associative_id: Uuid,
    /// pub from: `Anchor`,
    ///
    pub from: Uuid,
    /// pub middle: `Point`,
    ///
    pub middle: Uuid,
    /// pub one: `Anchor`,
    ///
    pub one: Uuid,
    /// pub other: `Anchor`,
    ///
    pub other: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"associative_ui-struct-definition"}}}

impl AssociativeUi {
    // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"associative_ui-new_impl"}}}
    /// Inter a new AssociativeUi and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Associative;
    /// # use sarzak::drawing::Point;
    /// # use sarzak::drawing::Edge;
    /// # use sarzak::drawing::AssociativeUi;
    /// # use sarzak::drawing::Anchor;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    ///
    /// let associative_tav = Associative::default();
    ///
    /// let point_kiq = Point::new(&mut store, 42, 42);
    /// let point_hij = Point::new(&mut store, 42, 42);
    /// let edge_soj = Edge::test_default(&mut store);
    /// let anchor_vxi = Anchor::new(&mut store, &point_kiq, &point_hij, &edge_soj);
    /// let point_ljm = Point::new(&mut store, 42, 42);
    /// let point_ixc = Point::new(&mut store, 42, 42);
    /// let edge_vnq = Edge::test_default(&mut store);
    /// let anchor_rfe = Anchor::new(&mut store, &point_ljm, &point_ixc, &edge_vnq);
    /// let point_bhn = Point::new(&mut store, 42, 42);
    /// let point_jig = Point::new(&mut store, 42, 42);
    /// let edge_gzx = Edge::test_default(&mut store);
    /// let anchor_lmi = Anchor::new(&mut store, &point_bhn, &point_jig, &edge_gzx);
    /// let point_vhf = Point::new(&mut store, 42, 42);
    ///
    /// let associative_ui = AssociativeUi::new(&mut store, &associative_tav, &anchor_vxi, &anchor_rfe, &anchor_lmi, &point_vhf);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(
        store: &mut ObjectStore,
        associative_id: &Associative,
        from: &Anchor,
        one: &Anchor,
        other: &Anchor,
        middle: &Point,
    ) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!(
                "{:?}::{:?}::{:?}::{:?}::{:?}::",
                associative_id, from, one, other, middle,
            )
            .as_bytes(),
        );
        let new = Self {
            id,
            associative_id: associative_id.id,
            from: from.id,
            one: one.id,
            other: other.id,
            middle: middle.id,
        };

        store.inter_associative_ui(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"associative_ui-new_impl"}}}
}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"associative_ui-extrude_impl", "is_uber":true}}}
impl Extrude<nut::drawing::AssociativeUI, Context<'_>> for AssociativeUi {
    fn extrude(orig: nut::drawing::AssociativeUI, context: &mut Context<'_>) -> Self {
        let Context { from, ref mut to } = context;

        Self::default()
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"associative_ui-extrude_impl"}}}

/// This represents additional information necessary to render a `Binary` relationship in the
/// user interface.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"binary_ui-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BinaryUi {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// Imported from the sarzak domain.
    /// [`nut::sarzak::Binary`]
    ///
    pub binary_id: Uuid,
    /// pub from: `Anchor`,
    ///
    pub from: Uuid,
    /// pub to: `Anchor`,
    ///
    pub to: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"binary_ui-struct-definition"}}}

impl BinaryUi {
    // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"binary_ui-new_impl"}}}
    /// Inter a new BinaryUi and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::drawing::Point;
    /// # use sarzak::sarzak::Binary;
    /// # use sarzak::drawing::Edge;
    /// # use sarzak::drawing::Anchor;
    /// # use sarzak::drawing::BinaryUi;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    ///
    /// let point_ayi = Point::new(&mut store, 42, 42);
    /// let point_qai = Point::new(&mut store, 42, 42);
    /// let edge_wdp = Edge::test_default(&mut store);
    /// let anchor_xbd = Anchor::new(&mut store, &point_ayi, &point_qai, &edge_wdp);
    /// let point_llp = Point::new(&mut store, 42, 42);
    /// let point_prh = Point::new(&mut store, 42, 42);
    /// let edge_wtg = Edge::test_default(&mut store);
    /// let anchor_ftw = Anchor::new(&mut store, &point_llp, &point_prh, &edge_wtg);
    /// let binary_xlk = Binary::default();
    ///
    ///
    /// let binary_ui = BinaryUi::new(&mut store, &anchor_xbd, &anchor_ftw, &binary_xlk);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, from: &Anchor, to: &Anchor, binary_id: &Binary) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{:?}::{:?}::{:?}::", from, to, binary_id,).as_bytes(),
        );
        let new = Self {
            id,
            from: from.id,
            to: to.id,
            binary_id: binary_id.id,
        };

        store.inter_binary_ui(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"binary_ui-new_impl"}}}
}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"binary_ui-extrude_impl", "is_uber":true}}}
impl Extrude<nut::drawing::BinaryUI, Context<'_>> for BinaryUi {
    fn extrude(orig: nut::drawing::BinaryUI, context: &mut Context<'_>) -> Self {
        let Context { from, ref mut to } = context;

        Self::default()
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"binary_ui-extrude_impl"}}}

/// The bottom of a rendered box
///
/// ❗️{"singleton_object": true}
///
//
pub const BOTTOM: Uuid = uuid!["cd977757-dbcb-5e5d-a0dc-d0e6624db6a0"];

/// An attachment point for an [Anchor]
///
/// It’s used with [Anchor] to orient the arrows on the ends of the lines according to the
/// side of the box to which they are attached. Some arrows are on top, some bottom, etc.
///
/// This is not rendered as a visible item. The [ObjectUI] manages that by itself. This instead
/// renders an invisible line. The line is used for several things. For one, when hovered over
/// the cursor changes to the appropriate one for resizing.
///
/// Also, this is used to register where relationship may anchor.
///
/// It’s this last regard that is somewhat concerning. Indicating that an anchor is attached
/// to an edge get’s us the connection we need between an [Object] and a [Relationship]. But
/// it’s under-specified. It doesn’t indicate where along the edge the arrow is connected
///.
///
/// I’m considering put a relationship back between [Anchor] and [Point].
///
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Edge {
    /// `Top(Top)`,
    ///
    Top(Uuid),
    /// `Left(Left)`,
    ///
    Left(Uuid),
    /// `Right(Right)`,
    ///
    Right(Uuid),
    /// `Bottom(Bottom)`,
    ///
    Bottom(Uuid),
}

impl Edge {
    pub fn get_id(&self) -> Uuid {
        match *self {
            Self::Top(z) => z,
            Self::Left(z) => z,
            Self::Right(z) => z,
            Self::Bottom(z) => z,
        }
    }
}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"edge-test_default"}}}
impl Edge {
    pub fn test_default(store: &mut ObjectStore) -> Self {
        let test = Self::Top(TOP);

        store.inter_edge(test.clone());

        test
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"edge-test_default"}}}

/// This represents additional data necessary to render an `Isa` relationship in the user interface
///.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"isa_ui-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct IsaUi {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub from: `Anchor`,
    ///
    pub from: Uuid,
    /// Imported from the sarzak domain.
    /// [`nut::sarzak::Isa`]
    ///
    pub isa: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"isa_ui-struct-definition"}}}

impl IsaUi {
    // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"isa_ui-new_impl"}}}
    /// Inter a new IsaUi and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Isa;
    /// # use sarzak::drawing::Anchor;
    /// # use sarzak::drawing::Point;
    /// # use sarzak::drawing::Edge;
    /// # use sarzak::drawing::IsaUi;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    /// # let mut sarzak_store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let isa_kqa = Isa::test_default(&mut sarzak_store);
    ///
    /// let point_ynu = Point::new(&mut store, 42, 42);
    /// let point_aml = Point::new(&mut store, 42, 42);
    /// let edge_sws = Edge::test_default(&mut store);
    /// let anchor_qzx = Anchor::new(&mut store, &point_ynu, &point_aml, &edge_sws);
    ///
    /// let isa_ui = IsaUi::new(&mut store, &isa_kqa, &anchor_qzx);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, isa: &Isa, from: &Anchor) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::{:?}::", isa, from,).as_bytes());
        let new = Self {
            id,
            isa: isa.get_id(),
            from: from.id,
        };

        store.inter_isa_ui(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"isa_ui-new_impl"}}}
}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"isa_ui-extrude_impl", "is_uber":true}}}
impl Extrude<nut::drawing::IsaUI, Context<'_>> for IsaUi {
    fn extrude(orig: nut::drawing::IsaUI, context: &mut Context<'_>) -> Self {
        let Context { from, ref mut to } = context;

        Self::default()
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"isa_ui-extrude_impl"}}}

/// The left side of a rendered box
///
/// ❗️{"singleton_object": true}
///
//
pub const LEFT: Uuid = uuid!["8125ae7e-9edb-5e9c-be33-643f1277e0e0"];

/// The Edge of an Object Depiction
///
/// There are four edges to a rendered object.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"object_edge-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ObjectEdge {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub edge: `Edge`,
    ///
    pub edge: Uuid,
    /// pub oui_id: `ObjectUI`,
    ///
    pub oui_id: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"object_edge-struct-definition"}}}

impl ObjectEdge {
    // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"object_edge-new_impl"}}}
    /// Inter a new ObjectEdge and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::drawing::Edge;
    /// # use sarzak::drawing::Point;
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::drawing::ObjectEdge;
    /// # use sarzak::drawing::ObjectUi;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    ///
    /// let edge_dul = Edge::test_default(&mut store);
    /// let point_ept = Point::new(&mut store, 42, 42);
    /// let evasive_learning = "fluffy_thrill".to_owned();
    /// let object_vhn = Object::default();
    ///
    /// let object_ui_lja = ObjectUi::new(&mut store, &point_ept, &object_vhn, 42, 42);
    ///
    /// let object_edge = ObjectEdge::new(&mut store, &edge_dul, &object_ui_lja);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, edge: &Edge, oui_id: &ObjectUi) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::{:?}::", edge, oui_id,).as_bytes());
        let new = Self {
            id,
            edge: edge.get_id(),
            oui_id: oui_id.id,
        };

        store.inter_object_edge(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"object_edge-new_impl"}}}
}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"object_edge-extrude_impl"}}}
impl Extrude<nut::drawing::ObjectEdge, Context<'_>> for ObjectEdge {
    fn extrude(orig: nut::drawing::ObjectEdge, context: &mut Context<'_>) -> Self {
        let Context { from, ref mut to } = context;

        Self::default()
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"object_edge-extrude_impl"}}}

/// Render a rectangle
///
/// This represents additional information that is necessary to draw an Object in the user interface
///.
///
/// Note that although we are logically related to [Edge] via `R14` we actually render our own
/// edges. We use the svg rect primitive to do this.
///
/// I’m throwing this in for the fuck of it. I don’t know if it’ll be useful or not.
///
/// ```js
/// var rect = document.createElementNS('http://www.w3.org/2000/svg', 'rect');
/// rect.setAttribute('class', 'objectRect');
/// rect.setAttribute('id', obj.id);
/// rect.setAttribute('x', obj.x);
/// rect.setAttribute('y', obj.y);
/// rect.setAttribute('width', obj.width);
///  rect.setAttribute('height', obj.height);
/// ```
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"object_ui-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ObjectUi {
    /// pub height: `i64`,
    ///
    pub height: i64,
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub width: `i64`,
    ///
    pub width: i64,
    /// Imported from the sarzak domain.
    /// [`nut::sarzak::Object`]
    ///
    pub object_id: Uuid,
    /// pub origin: `Point`,
    ///
    pub origin: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"object_ui-struct-definition"}}}

impl ObjectUi {
    // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"object_ui-new_impl"}}}
    /// Inter a new ObjectUi and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::drawing::Point;
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::drawing::ObjectUi;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    ///
    /// let point_qkb = Point::new(&mut store, 42, 42);
    /// let ablaze_silk = "hollow_leg".to_owned();
    /// let object_psj = Object::default();
    ///
    ///
    /// let object_ui = ObjectUi::new(&mut store, &point_qkb, &object_psj, 42, 42);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(
        store: &mut ObjectStore,
        origin: &Point,
        object_id: &Object,
        width: i64,
        height: i64,
    ) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{:?}::{:?}::{}::{}::", origin, object_id, width, height,).as_bytes(),
        );
        let new = Self {
            id,
            origin: origin.id,
            object_id: object_id.id,
            width,
            height,
        };

        store.inter_object_ui(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"object_ui-new_impl"}}}
}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"object_ui-extrude_impl", "is_uber":true}}}
impl Extrude<nut::drawing::ObjectUI, Context<'_>> for ObjectUi {
    fn extrude(orig: nut::drawing::ObjectUI, context: &mut Context<'_>) -> Self {
        let Context { from, ref mut to } = context;

        Self::default()
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"object_ui-extrude_impl"}}}

/// A point is a two-tuple that represents a location on the drawing canvas.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"point-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Point {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub x: `i64`,
    ///
    pub x: i64,
    /// pub y: `i64`,
    ///
    pub y: i64,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"point-struct-definition"}}}

impl Point {
    // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"point-new_impl"}}}
    /// Inter a new Point and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::drawing::Point;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    ///
    ///
    /// let point = Point::new(&mut store, 42, 42);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, y: i64, x: i64) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{}::{}::", y, x,).as_bytes());
        let new = Self { id, y, x };

        store.inter_point(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"point-new_impl"}}}
}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"point-extrude_impl"}}}
impl Extrude<nut::drawing::Point, Context<'_>> for Point {
    fn extrude(orig: nut::drawing::Point, context: &mut Context<'_>) -> Self {
        let Context { from, ref mut to } = context;

        Self::default()
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"point-extrude_impl"}}}

/// Additional information necessary to render relationships in the user interface.
///
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum RelationshipUi {
    /// `BinaryUi(BinaryUi)`,
    ///
    BinaryUi(Uuid),
    /// `IsaUi(IsaUi)`,
    ///
    IsaUi(Uuid),
    /// `AssociativeUi(AssociativeUi)`,
    ///
    AssociativeUi(Uuid),
}

impl RelationshipUi {
    pub fn get_id(&self) -> Uuid {
        match *self {
            Self::BinaryUi(z) => z,
            Self::IsaUi(z) => z,
            Self::AssociativeUi(z) => z,
        }
    }
}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"relationship_ui-test_default"}}}
impl RelationshipUi {
    pub fn test_default(store: &mut ObjectStore) -> Self {
        // {"magic":"","kind":"IgnoreBlockBegin"}
        let point_owj = Point::new(store, 42, 42);
        let point_tss = Point::new(store, 42, 42);
        let edge_nqp = Edge::test_default(store);
        let anchor_zzx = Anchor::new(store, &point_owj, &point_tss, &edge_nqp);
        let point_fqj = Point::new(store, 42, 42);
        let point_pjr = Point::new(store, 42, 42);
        let edge_uuz = Edge::test_default(store);
        let anchor_cvc = Anchor::new(store, &point_fqj, &point_pjr, &edge_uuz);
        let binary_tch = Binary::default();

        let test = Self::BinaryUi(BinaryUi::new(store, &anchor_zzx, &anchor_cvc, &binary_tch).id);
        // {"magic":"","kind":"IgnoreBlockEnd"}

        store.inter_relationship_ui(test.clone());

        test
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"relationship_ui-test_default"}}}

/// The right side of a rendered box
///
/// ❗️{"singleton_object": true}
///
//
pub const RIGHT: Uuid = uuid!["1d99f96f-d110-5adf-a108-9fb0b707dae3"];

/// Subtype Anchors
///
/// Just as it sounds, these are [`Anchor`]s used by [`Subtype`]s in an [`Isa`] relationship
///.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"subtype_anchors-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SubtypeAnchors {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub anchor_id: `Anchor`,
    ///
    pub anchor_id: Uuid,
    /// pub isaui_id: `IsaUI`,
    ///
    pub isaui_id: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"subtype_anchors-struct-definition"}}}

impl SubtypeAnchors {
    // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"subtype_anchors-new_impl"}}}
    /// Inter a new SubtypeAnchors and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::drawing::IsaUi;
    /// # use sarzak::drawing::SubtypeAnchors;
    /// # use sarzak::drawing::Point;
    /// # use sarzak::sarzak::Isa;
    /// # use sarzak::drawing::Anchor;
    /// # use sarzak::drawing::Edge;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    /// # let mut sarzak_store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let point_pio = Point::new(&mut store, 42, 42);
    /// let point_eki = Point::new(&mut store, 42, 42);
    /// let edge_jdx = Edge::test_default(&mut store);
    /// let anchor_dir = Anchor::new(&mut store, &point_pio, &point_eki, &edge_jdx);
    /// let isa_pbi = Isa::test_default(&mut sarzak_store);
    ///
    /// let point_nar = Point::new(&mut store, 42, 42);
    /// let point_cgd = Point::new(&mut store, 42, 42);
    /// let edge_iym = Edge::test_default(&mut store);
    /// let anchor_hwg = Anchor::new(&mut store, &point_nar, &point_cgd, &edge_iym);
    /// let isa_ui_xqv = IsaUi::new(&mut store, &isa_pbi, &anchor_hwg);
    ///
    /// let subtype_anchors = SubtypeAnchors::new(&mut store, &anchor_dir, &isa_ui_xqv);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, anchor_id: &Anchor, isaui_id: &IsaUi) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{:?}::{:?}::", anchor_id, isaui_id,).as_bytes(),
        );
        let new = Self {
            id,
            anchor_id: anchor_id.id,
            isaui_id: isaui_id.id,
        };

        store.inter_subtype_anchors(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"subtype_anchors-new_impl"}}}
}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"subtype_anchors-extrude_impl", "is_uber": true}}}
// impl Extrude<nut::drawing::SubtypeAnchors, Context<'_>> for SubtypeAnchors {
//     fn extrude(orig: nut::drawing::SubtypeAnchors, context: &mut Context<'_>) -> Self {
//         let Context { from, ref mut to } = context;

//         Self::default()
//     }
// }
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"subtype_anchors-extrude_impl"}}}

/// The top edge of the rendered box
///
/// ❗️{"singleton_object": true}
///
//
pub const TOP: Uuid = uuid!["66416a6d-1227-53b5-bb5a-bfd45e0ea72e"];
