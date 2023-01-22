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
//!  sarzak gen
//! ```
// {"magic":"","kind":"IgnoreBlockEnd"}
// {"magic":"","version":"0.5.0"}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"imports"}}}
use crate::drawing::store::ObjectStore;
use crate::drawing::UUID_NS;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"imports"}}}

// Imported Objects
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"imported-objects"}}}
use crate::sarzak::types::Associative;
use crate::sarzak::types::Binary;
use crate::sarzak::types::Isa;
use crate::sarzak::types::Object;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"imported-objects"}}}

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
    /// # use sarzak::drawing::Point;
    /// # use sarzak::drawing::Edge;
    /// # use sarzak::drawing::Anchor;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    ///
    /// let point_voy = Point::new(&mut store, 42, 42);
    /// let point_nvi = Point::new(&mut store, 42, 42);
    /// let edge_crt = Edge::test_default(&mut store);
    ///
    /// let anchor = Anchor::new(&mut store, &point_voy, &point_nvi, &edge_crt);
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
    /// # use sarzak::drawing::Edge;
    /// # use sarzak::drawing::Point;
    /// # use sarzak::drawing::Anchor;
    /// # use sarzak::drawing::AssociativeUi;
    /// # use sarzak::sarzak::Associative;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    ///
    /// let associative_lwd = Associative::default();
    ///
    /// let point_dxm = Point::new(&mut store, 42, 42);
    /// let point_ifp = Point::new(&mut store, 42, 42);
    /// let edge_syz = Edge::test_default(&mut store);
    /// let anchor_wqo = Anchor::new(&mut store, &point_dxm, &point_ifp, &edge_syz);
    /// let point_gqp = Point::new(&mut store, 42, 42);
    /// let point_abq = Point::new(&mut store, 42, 42);
    /// let edge_cmn = Edge::test_default(&mut store);
    /// let anchor_haj = Anchor::new(&mut store, &point_gqp, &point_abq, &edge_cmn);
    /// let point_nda = Point::new(&mut store, 42, 42);
    /// let point_xmf = Point::new(&mut store, 42, 42);
    /// let edge_nps = Edge::test_default(&mut store);
    /// let anchor_wyq = Anchor::new(&mut store, &point_nda, &point_xmf, &edge_nps);
    /// let point_zlb = Point::new(&mut store, 42, 42);
    ///
    /// let associative_ui = AssociativeUi::new(&mut store, &associative_lwd, &anchor_wqo, &anchor_haj, &anchor_wyq, &point_zlb);
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
    /// # use sarzak::drawing::BinaryUi;
    /// # use sarzak::drawing::Edge;
    /// # use sarzak::drawing::Anchor;
    /// # use sarzak::sarzak::Binary;
    /// # use sarzak::drawing::Point;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    ///
    /// let point_rzx = Point::new(&mut store, 42, 42);
    /// let point_rea = Point::new(&mut store, 42, 42);
    /// let edge_qbe = Edge::test_default(&mut store);
    /// let anchor_zla = Anchor::new(&mut store, &point_rzx, &point_rea, &edge_qbe);
    /// let point_mch = Point::new(&mut store, 42, 42);
    /// let point_hdt = Point::new(&mut store, 42, 42);
    /// let edge_brw = Edge::test_default(&mut store);
    /// let anchor_xqi = Anchor::new(&mut store, &point_mch, &point_hdt, &edge_brw);
    /// let binary_hcy = Binary::default();
    ///
    ///
    /// let binary_ui = BinaryUi::new(&mut store, &anchor_zla, &anchor_xqi, &binary_hcy);
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
    /// # use sarzak::drawing::Edge;
    /// # use sarzak::drawing::IsaUi;
    /// # use sarzak::drawing::Anchor;
    /// # use sarzak::drawing::Point;
    /// # use sarzak::sarzak::Isa;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    /// # let mut sarzak_store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let isa_tww = Isa::test_default(&mut sarzak_store);
    ///
    /// let point_hah = Point::new(&mut store, 42, 42);
    /// let point_ois = Point::new(&mut store, 42, 42);
    /// let edge_nwl = Edge::test_default(&mut store);
    /// let anchor_kul = Anchor::new(&mut store, &point_hah, &point_ois, &edge_nwl);
    ///
    /// let isa_ui = IsaUi::new(&mut store, &isa_tww, &anchor_kul);
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
    /// # use sarzak::drawing::ObjectEdge;
    /// # use sarzak::drawing::ObjectUi;
    /// # use sarzak::drawing::Point;
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::drawing::Edge;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    ///
    /// let edge_elf = Edge::test_default(&mut store);
    /// let point_kia = Point::new(&mut store, 42, 42);
    /// let inexpensive_dock = "jumpy_profit".to_owned();
    /// let object_kof = Object::default();
    ///
    /// let object_ui_wlh = ObjectUi::new(&mut store, &point_kia, &object_kof, 42, 42);
    ///
    /// let object_edge = ObjectEdge::new(&mut store, &edge_elf, &object_ui_wlh);
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
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::drawing::ObjectUi;
    /// # use sarzak::drawing::Point;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    ///
    /// let point_vbc = Point::new(&mut store, 42, 42);
    /// let attractive_sugar = "quarrelsome_hen".to_owned();
    /// let object_cba = Object::default();
    ///
    ///
    /// let object_ui = ObjectUi::new(&mut store, &point_vbc, &object_cba, 42, 42);
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
        let point_mmx = Point::new(store, 42, 42);
        let point_cuv = Point::new(store, 42, 42);
        let edge_xgl = Edge::test_default(store);
        let anchor_uyy = Anchor::new(store, &point_mmx, &point_cuv, &edge_xgl);
        let point_owm = Point::new(store, 42, 42);
        let point_lkl = Point::new(store, 42, 42);
        let edge_anw = Edge::test_default(store);
        let anchor_kvk = Anchor::new(store, &point_owm, &point_lkl, &edge_anw);
        let binary_uwi = Binary::default();

        let test = Self::BinaryUi(BinaryUi::new(store, &anchor_uyy, &anchor_kvk, &binary_uwi).id);
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
    /// # use sarzak::sarzak::Isa;
    /// # use sarzak::drawing::Point;
    /// # use sarzak::drawing::Anchor;
    /// # use sarzak::drawing::Edge;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    /// # let mut sarzak_store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let point_lpa = Point::new(&mut store, 42, 42);
    /// let point_aed = Point::new(&mut store, 42, 42);
    /// let edge_lth = Edge::test_default(&mut store);
    /// let anchor_wmm = Anchor::new(&mut store, &point_lpa, &point_aed, &edge_lth);
    /// let isa_vuw = Isa::test_default(&mut sarzak_store);
    ///
    /// let point_nvh = Point::new(&mut store, 42, 42);
    /// let point_iaw = Point::new(&mut store, 42, 42);
    /// let edge_jat = Edge::test_default(&mut store);
    /// let anchor_htu = Anchor::new(&mut store, &point_nvh, &point_iaw, &edge_jat);
    /// let isa_ui_xpk = IsaUi::new(&mut store, &isa_vuw, &anchor_htu);
    ///
    /// let subtype_anchors = SubtypeAnchors::new(&mut store, &anchor_wmm, &isa_ui_xpk);
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

/// The top edge of the rendered box
///
/// ❗️{"singleton_object": true}
///
//
pub const TOP: Uuid = uuid!["66416a6d-1227-53b5-bb5a-bfd45e0ea72e"];
