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
// {"magic":"","kind":{"IgnoreBlockBegin":{}}
//! ```shell
//!  sarzak gen
//! ```
// {"magic":"","kind":"IgnoreBlockEnd"}
// {"magic":"","version":"0.5.0"}
// {"magic":"","version":"1.0.0"}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"imports", "is_uber": true}}}
use crate::drawing::store::ObjectStore;
use crate::drawing::UUID_NS;
use crate::sarzak::store::ObjectStore as SarzakObjectStore;
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
    pub(crate) from: &'a DrawingObjectStore,
    pub(crate) to: &'a mut ObjectStore,
    pub(crate) sarzak: &'a SarzakObjectStore,
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"anchor-new_impl"}}}
impl Anchor {
    /// Inter a new Anchor and return it's `id`
    ///
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}
    /// # Example
    ///
    ///```
    /// # use sarzak::drawing::Point;
    /// # use sarzak::drawing::Edge;
    /// # use sarzak::drawing::Anchor;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    ///
    /// let point_fad = Point::new(&mut store, 42, 42);
    /// let point_khv = Point::new(&mut store, 42, 42);
    /// let edge_vmb = Edge::test_default(&mut store);
    ///
    /// let anchor = Anchor::new(&mut store, &point_fad, &point_khv, &edge_vmb);
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"anchor-extrude_impl", "is_uber": true}}}
impl Extrude<nut::drawing::Anchor, Context<'_>> for Anchor {
    fn extrude(orig: nut::drawing::Anchor, context: &mut Context<'_>) -> Self {
        // This is kosher because we keep the id when we extrude Point.
        for i in [orig.location, orig.offset] {
            let point = context.from.exhume_point(&i).unwrap();
            let point = Point::extrude(point.clone(), context);
            context.to.inter_point(point);
        }

        let edge = context.from.exhume_edge(&orig.edge).unwrap();
        let edge = Edge::get_edge_from_nut(&edge);

        Self {
            id: orig.id,
            edge,
            location: orig.location,
            offset: orig.offset,
        }
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
    /// pub from: `Point`,
    ///
    pub from: Uuid,
    /// pub middle: `Anchor`,
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"associative_ui-new_impl"}}}
impl AssociativeUi {
    /// Inter a new AssociativeUi and return it's `id`
    ///
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}
    /// # Example
    ///
    ///```
    /// # use sarzak::drawing::AssociativeUi;
    /// # use sarzak::drawing::Point;
    /// # use sarzak::sarzak::Associative;
    /// # use sarzak::drawing::Edge;
    /// # use sarzak::drawing::Anchor;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    ///
    /// let associative_vuf = Associative::default();
    ///
    /// let point_kjx = Point::new(&mut store, 42, 42);
    /// let point_sas = Point::new(&mut store, 42, 42);
    /// let edge_xez = Edge::test_default(&mut store);
    /// let anchor_wzx = Anchor::new(&mut store, &point_kjx, &point_sas, &edge_xez);
    /// let point_dbi = Point::new(&mut store, 42, 42);
    /// let point_wop = Point::new(&mut store, 42, 42);
    /// let edge_kzz = Edge::test_default(&mut store);
    /// let anchor_ist = Anchor::new(&mut store, &point_dbi, &point_wop, &edge_kzz);
    /// let point_gup = Point::new(&mut store, 42, 42);
    /// let point_kla = Point::new(&mut store, 42, 42);
    /// let edge_xix = Edge::test_default(&mut store);
    /// let anchor_yxl = Anchor::new(&mut store, &point_gup, &point_kla, &edge_xix);
    /// let point_deb = Point::new(&mut store, 42, 42);
    ///
    /// let associative_ui = AssociativeUi::new(&mut store, &associative_vuf, &anchor_wzx, &anchor_ist, &anchor_yxl, &point_deb);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(
        store: &mut ObjectStore,
        associative_id: &Associative,
        middle: &Anchor,
        one: &Anchor,
        other: &Anchor,
        from: &Point,
    ) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!(
                "{:?}::{:?}::{:?}::{:?}::{:?}::",
                associative_id, middle, one, other, from,
            )
            .as_bytes(),
        );
        let new = Self {
            id,
            associative_id: associative_id.id,
            middle: middle.id,
            one: one.id,
            other: other.id,
            from: from.id,
        };

        store.inter_associative_ui(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"associative_ui-new_impl"}}}
}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"associative_ui-extrude_impl", "is_uber":true}}}
impl Extrude<nut::drawing::AssociativeUI, Context<'_>> for AssociativeUi {
    fn extrude(orig: nut::drawing::AssociativeUI, context: &mut Context<'_>) -> Self {
        // This is 😎 because we keep the same id when we extrude Anchor.
        for i in [orig.one, orig.other, orig.middle] {
            let anchor = context.from.exhume_anchor(&i).unwrap();
            let anchor = Anchor::extrude(anchor.clone(), context);
            context.to.inter_anchor(anchor);
        }

        let point = context.from.exhume_point(&orig.from).unwrap();
        let point = Point::extrude(point.clone(), context);
        context.to.inter_point(point);

        Self {
            id: orig.id,
            associative_id: orig.associative_id,
            from: orig.from,
            middle: orig.middle,
            one: orig.one,
            other: orig.other,
        }
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"binary_ui-new_impl"}}}
impl BinaryUi {
    /// Inter a new BinaryUi and return it's `id`
    ///
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}
    /// # Example
    ///
    ///```
    /// # use sarzak::drawing::Point;
    /// # use sarzak::sarzak::Binary;
    /// # use sarzak::drawing::Anchor;
    /// # use sarzak::drawing::Edge;
    /// # use sarzak::drawing::BinaryUi;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    ///
    /// let point_kvm = Point::new(&mut store, 42, 42);
    /// let point_wmj = Point::new(&mut store, 42, 42);
    /// let edge_hgc = Edge::test_default(&mut store);
    /// let anchor_mrk = Anchor::new(&mut store, &point_kvm, &point_wmj, &edge_hgc);
    /// let point_joh = Point::new(&mut store, 42, 42);
    /// let point_nqm = Point::new(&mut store, 42, 42);
    /// let edge_fbf = Edge::test_default(&mut store);
    /// let anchor_xuq = Anchor::new(&mut store, &point_joh, &point_nqm, &edge_fbf);
    /// let binary_saz = Binary::default();
    ///
    ///
    /// let binary_ui = BinaryUi::new(&mut store, &anchor_mrk, &anchor_xuq, &binary_saz);
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
        // Extrude from and to. Leave binary_id alone, as it's the object in the
        // sarzak domain. I suppose we can do a sanity check...
        assert!(context.sarzak.exhume_binary(&orig.binary).is_some());

        // This is 😎 because we keep the same id when we extrude Anchor.
        for i in [orig.from, orig.to] {
            let anchor = context.from.exhume_anchor(&i).unwrap();
            let anchor = Anchor::extrude(anchor.clone(), context);
            context.to.inter_anchor(anchor);
        }

        Self {
            id: orig.id,
            binary_id: orig.binary,
            from: orig.from,
            to: orig.to,
        }
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
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"edge-enum-definition"}}}
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
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"{}-enum-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"Edge-enum-get-id-impl"}}}
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
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"Edge-enum-get-id-impl"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"edge-test_default"}}}
impl Edge {
    pub fn test_default(store: &mut ObjectStore) -> Self {
        let test = Self::Top(TOP);

        store.inter_edge(test.clone());

        test
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"edge-test_default"}}}

impl Edge {
    fn get_edge_from_nut(edge: &nut::drawing::Edge) -> Uuid {
        match edge {
            nut::drawing::Edge::Top(_) => TOP,
            nut::drawing::Edge::Left(_) => LEFT,
            nut::drawing::Edge::Right(_) => RIGHT,
            nut::drawing::Edge::Bottom(_) => BOTTOM,
        }
    }
}

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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"isa_ui-new_impl"}}}
impl IsaUi {
    /// Inter a new IsaUi and return it's `id`
    ///
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Isa;
    /// # use sarzak::drawing::Edge;
    /// # use sarzak::drawing::IsaUi;
    /// # use sarzak::drawing::Anchor;
    /// # use sarzak::drawing::Point;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    ///
    /// let isa_hlw = Isa::default();
    ///
    /// let point_khe = Point::new(&mut store, 42, 42);
    /// let point_jzl = Point::new(&mut store, 42, 42);
    /// let edge_jcb = Edge::test_default(&mut store);
    /// let anchor_zht = Anchor::new(&mut store, &point_khe, &point_jzl, &edge_jcb);
    ///
    /// let isa_ui = IsaUi::new(&mut store, &isa_hlw, &anchor_zht);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, isa: &Isa, from: &Anchor) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::{:?}::", isa, from,).as_bytes());
        let new = Self {
            id,
            isa: isa.id,
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
        // Verify the imported object.
        assert!(context.sarzak.exhume_isa(&orig.isa).is_some());

        let anchor = context.from.exhume_anchor(&orig.from).unwrap();
        let anchor = Anchor::extrude(anchor.clone(), context);
        let id = anchor.id;
        context.to.inter_anchor(anchor);

        let isa_ui = Self {
            id: orig.id,
            from: id,
            isa: orig.isa,
        };

        // In nut the to anchors are stored in a Vec. We break those out to
        // SubtypeAnchors here.
        for to in orig.to.iter() {
            let anchor = context.from.exhume_anchor(&to).unwrap();
            let anchor = Anchor::extrude(anchor.clone(), context);

            SubtypeAnchors::new(context.to, &anchor, &isa_ui);
            context.to.inter_anchor(anchor);
        }

        isa_ui
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"object_edge-new_impl"}}}
impl ObjectEdge {
    /// Inter a new ObjectEdge and return it's `id`
    ///
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}
    /// # Example
    ///
    ///```
    /// # use sarzak::drawing::ObjectUi;
    /// # use sarzak::drawing::Edge;
    /// # use sarzak::drawing::Point;
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::drawing::ObjectEdge;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    ///
    /// let edge_wyj = Edge::test_default(&mut store);
    /// let point_net = Point::new(&mut store, 42, 42);
    /// let uptight_hill = "lumpy_shame".to_owned();
    /// let object_gai = Object::default();
    ///
    /// let object_ui_fbw = ObjectUi::new(&mut store, &point_net, &object_gai, 42, 42);
    ///
    /// let object_edge = ObjectEdge::new(&mut store, &edge_wyj, &object_ui_fbw);
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"object_edge-extrude_impl", "is_uber": true}}}
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"object_ui-new_impl"}}}
impl ObjectUi {
    /// Inter a new ObjectUi and return it's `id`
    ///
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::drawing::ObjectUi;
    /// # use sarzak::drawing::Point;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    ///
    /// let point_ixw = Point::new(&mut store, 42, 42);
    /// let distinct_creator = "testy_paper".to_owned();
    /// let object_ljb = Object::default();
    ///
    ///
    /// let object_ui = ObjectUi::new(&mut store, &point_ixw, &object_ljb, 42, 42);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(
        store: &mut ObjectStore,
        origin: &Point,
        object_id: &Object,
        //         width: i64, //⚡️
        height: i64,
        width: i64,
    ) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            //             format!("{:?}::{:?}::{}::{}::", origin, object_id, width, height,).as_bytes(), //⚡️
            format!("{:?}::{:?}::{}::{}::", origin, object_id, height, width,).as_bytes(),
        );
        let new = Self {
            id,
            origin: origin.id,
            object_id: object_id.id,
            //             width, //⚡️
            height,
            width,
        };

        store.inter_object_ui(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"object_ui-new_impl"}}}
}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"object_ui-extrude_impl", "is_uber":true}}}
impl Extrude<nut::drawing::ObjectUI, Context<'_>> for ObjectUi {
    fn extrude(orig: nut::drawing::ObjectUI, context: &mut Context<'_>) -> Self {
        let point = context.from.exhume_point(&orig.origin).unwrap();
        let point = Point::extrude(point.clone(), context);
        context.to.inter_point(point);

        let oui = Self {
            height: orig.height,
            id: orig.id,
            width: orig.width,
            object_id: orig.object,
            origin: orig.origin,
        };

        // Create the four edges. I think it's a waste of time, because I think that
        // the abstraction is incorrect.
        ObjectEdge::new(context.to, &Edge::Top(TOP), &oui);
        ObjectEdge::new(context.to, &Edge::Left(LEFT), &oui);
        ObjectEdge::new(context.to, &Edge::Right(RIGHT), &oui);
        ObjectEdge::new(context.to, &Edge::Bottom(BOTTOM), &oui);

        oui
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"point-new_impl"}}}
impl Point {
    /// Inter a new Point and return it's `id`
    ///
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}
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
    //     pub fn new(store: &mut ObjectStore, y: i64, x: i64) -> Self { //⚡️
    //         let id = Uuid::new_v5(&UUID_NS, format!("{}::{}::", y, x,).as_bytes()); //⚡️
    //         let new = Self { id, y, x }; //⚡️
    pub fn new(store: &mut ObjectStore, x: i64, y: i64) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{}::{}::", x, y,).as_bytes());
        let new = Self { id, x, y };

        store.inter_point(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"point-new_impl"}}}
}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"point-extrude_impl", "is_uber": true}}}
impl Extrude<nut::drawing::Point, Context<'_>> for Point {
    fn extrude(orig: nut::drawing::Point, _context: &mut Context<'_>) -> Self {
        Self {
            id: orig.id,
            x: orig.x,
            y: orig.x,
        }
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"point-extrude_impl"}}}

/// Additional information necessary to render relationships in the user interface.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"relationship_ui-enum-definition"}}}
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
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"{}-enum-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"RelationshipUi-enum-get-id-impl"}}}
impl RelationshipUi {
    pub fn get_id(&self) -> Uuid {
        match *self {
            Self::BinaryUi(z) => z,
            Self::IsaUi(z) => z,
            Self::AssociativeUi(z) => z,
        }
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"RelationshipUi-enum-get-id-impl"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"relationship_ui-test_default"}}}
impl RelationshipUi {
    pub fn test_default(store: &mut ObjectStore) -> Self {
        // {"magic":"","kind":{"IgnoreBlockBegin":{}}
        let point_lxr = Point::new(store, 42, 42);
        let point_ars = Point::new(store, 42, 42);
        let edge_vql = Edge::test_default(store);
        let anchor_rra = Anchor::new(store, &point_lxr, &point_ars, &edge_vql);
        let point_irx = Point::new(store, 42, 42);
        let point_slq = Point::new(store, 42, 42);
        let edge_voe = Edge::test_default(store);
        let anchor_wsb = Anchor::new(store, &point_irx, &point_slq, &edge_voe);
        let binary_ocd = Binary::default();

        let test = Self::BinaryUi(BinaryUi::new(store, &anchor_rra, &anchor_wsb, &binary_ocd).id);
        // {"magic":"","kind":"IgnoreBlockEnd"}

        store.inter_relationship_ui(test.clone());

        test
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"relationship_ui-test_default"}}}

impl Extrude<nut::drawing::RelationshipUI, Context<'_>> for RelationshipUi {
    fn extrude(input: nut::drawing::RelationshipUI, context: &mut Context<'_>) -> Self {
        match input {
            nut::drawing::RelationshipUI::BinaryUI(b_id) => {
                let b = context.from.exhume_binary_ui(&b_id).unwrap();
                let binary_ui = BinaryUi::extrude(b.clone(), context);
                let id = binary_ui.id;
                context.to.inter_binary_ui(binary_ui);

                Self::BinaryUi(id)
            }
            nut::drawing::RelationshipUI::IsaUI(i_id) => {
                let i = context.from.exhume_isa_ui(&i_id).unwrap();
                let isa_ui = IsaUi::extrude(i.clone(), context);
                let id = isa_ui.id;
                context.to.inter_isa_ui(isa_ui);

                Self::IsaUi(id)
            }
            nut::drawing::RelationshipUI::AssociativeUI(a_id) => {
                let a = context.from.exhume_associative_ui(&a_id).unwrap();
                let associative_ui = AssociativeUi::extrude(a.clone(), context);
                let id = associative_ui.id;
                context.to.inter_associative_ui(associative_ui);

                Self::AssociativeUi(id)
            }
        }
    }
}

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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"subtype_anchors-new_impl"}}}
impl SubtypeAnchors {
    /// Inter a new SubtypeAnchors and return it's `id`
    ///
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}
    /// # Example
    ///
    ///```
    /// # use sarzak::drawing::Anchor;
    /// # use sarzak::drawing::IsaUi;
    /// # use sarzak::sarzak::Isa;
    /// # use sarzak::drawing::SubtypeAnchors;
    /// # use sarzak::drawing::Point;
    /// # use sarzak::drawing::Edge;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    ///
    /// let point_fzf = Point::new(&mut store, 42, 42);
    /// let point_eav = Point::new(&mut store, 42, 42);
    /// let edge_ajv = Edge::test_default(&mut store);
    /// let anchor_aqu = Anchor::new(&mut store, &point_fzf, &point_eav, &edge_ajv);
    /// let isa_mdx = Isa::default();
    ///
    /// let point_lnk = Point::new(&mut store, 42, 42);
    /// let point_qzr = Point::new(&mut store, 42, 42);
    /// let edge_wik = Edge::test_default(&mut store);
    /// let anchor_aey = Anchor::new(&mut store, &point_lnk, &point_qzr, &edge_wik);
    /// let isa_ui_oyt = IsaUi::new(&mut store, &isa_mdx, &anchor_aey);
    ///
    /// let subtype_anchors = SubtypeAnchors::new(&mut store, &anchor_aqu, &isa_ui_oyt);
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
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"subtype_anchors-extrude_impl"}}}

/// The top edge of the rendered box
///
/// ❗️{"singleton_object": true}
///
//
pub const TOP: Uuid = uuid!["66416a6d-1227-53b5-bb5a-bfd45e0ea72e"];
