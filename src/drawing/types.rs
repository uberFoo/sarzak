//! {"magic":"","version":"0.0.1"}
//! Types for instances of the "Drawing" domain
//!
//! The following types are defined herein:
//!    * [`Anchor`]
//!    * [`BinaryUi`]
//!    * [`Point`]
//!    * [`ObjectEdge`]
//!    * [`TOP`]
//!    * [`LEFT`]
//!    * [`Edge`]
//!    * [`RelationshipUi`]
//!    * [`ObjectUi`]
//!    * [`IsaUi`]
//!    * [`AssociativeUi`]
//!    * [`SubtypeAnchors`]
//!    * [`RIGHT`]
//!    * [`BOTTOM`]
//!
//! Generated Code -- edit _carefully_.
//! Don't mess with anything between {"magic":"","kind":"CriticalBlockBegin"}
//! and {"magic":"","kind":"CriticalBlockEnd"}. Otherwise, you should be free
//! to go wild. Happy hacking!
//! Use the following invocation to reproduce:
//! ```shell
//!  sarzak gen
//! ```
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};

// Re-exports
// {"magic":"","kind":"CriticalBlockBegin"}
use crate::drawing::store::ObjectStore;
use crate::drawing::UUID_NS;
// {"magic":"","kind":"CriticalBlockEnd"}

// Imported Objects
// {"magic":"","kind":"CriticalBlockBegin"}
use crate::sarzak::types::Associative;
use crate::sarzak::types::Binary;
use crate::sarzak::types::Isa;
use crate::sarzak::types::Object;
// {"magic":"","kind":"CriticalBlockEnd"}

/// An anchor, or anchor point, is the location where an arrow from a relationship attached
/// to an object.
///
///
///
///
/// 
/// Rather than storing the `x` and `y` coordinates of where the anchor attaches, we are related
/// to an [Edge], which is related to a box, which is related to the [Object] to which we are
/// attached. This of course completes the circuit from the [Relationship] for which we are
/// drawing the lines in the first place.
///
///
///
///
/// 
/// Anchor also contains a direction, so that we know the orientation to draw the arrows. Finally
///, there is an offset, which is a point that describes the offset from the anchor for the
/// relationship phrase.
///
/// _Generated code_
// {"magic":"","kind":"CriticalBlockBegin"}
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
// {"magic":"","kind":"CriticalBlockEnd"}

impl Anchor {
    /// Inter a new Anchor and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::drawing::Anchor;
    /// # use sarzak::drawing::Point;
    /// # use sarzak::drawing::Edge;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    ///
    /// let point_gvs = Point::new(&mut store, 42, 42);
    /// let point_onx = Point::new(&mut store, 42, 42);
    /// let edge_zcm = Edge::test_default(&mut store);
    ///
    /// let anchor = Anchor::new(&mut store, &point_gvs, &point_onx, &edge_zcm);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    // {"magic":"","kind":"CriticalBlockBegin"}
    #[rustfmt::skip]
    pub fn new(store: &mut ObjectStore, location: &Point, offset: &Point, edge: &Edge, ) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::{:?}::{:?}::", location, offset, edge, ).as_bytes());
        let new = Self {
            id,
            location: location.id,
            offset: offset.id,
            edge: edge.get_id(),
        };




        
        store.inter_anchor(new.clone());




        
        new
    }
    // {"magic":"","kind":"CriticalBlockEnd"}
}
/// This represents additional information necessary to render a `Binary` relationship in the
/// user interface.
///
/// _Generated code_
// {"magic":"","kind":"CriticalBlockBegin"}
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
// {"magic":"","kind":"CriticalBlockEnd"}

impl BinaryUi {
    /// Inter a new BinaryUi and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::drawing::Edge;
    /// # use sarzak::drawing::BinaryUi;
    /// # use sarzak::drawing::Point;
    /// # use sarzak::sarzak::Binary;
    /// # use sarzak::drawing::Anchor;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    ///
    /// let point_ihj = Point::new(&mut store, 42, 42);
    /// let point_vcx = Point::new(&mut store, 42, 42);
    /// let edge_glp = Edge::test_default(&mut store);
    /// let anchor_fio = Anchor::new(&mut store, &point_ihj, &point_vcx, &edge_glp);
    /// let point_hoe = Point::new(&mut store, 42, 42);
    /// let point_zrq = Point::new(&mut store, 42, 42);
    /// let edge_nve = Edge::test_default(&mut store);
    /// let anchor_exm = Anchor::new(&mut store, &point_hoe, &point_zrq, &edge_nve);
    /// let binary_yip = Binary::default();
    ///
    ///
    /// let binary_ui = BinaryUi::new(&mut store, &anchor_fio, &anchor_exm, &binary_yip);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    // {"magic":"","kind":"CriticalBlockBegin"}
    #[rustfmt::skip]
    pub fn new(store: &mut ObjectStore, from: &Anchor, to: &Anchor, binary_id: &Binary, ) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::{:?}::{:?}::", from, to, binary_id, ).as_bytes());
        let new = Self {
            id,
            from: from.id,
            to: to.id,
            binary_id: binary_id.id,
        };




        
        store.inter_binary_ui(new.clone());




        
        new
    }
    // {"magic":"","kind":"CriticalBlockEnd"}
}
/// A point is a two-tuple that represents a location on the drawing canvas.
///
/// _Generated code_
// {"magic":"","kind":"CriticalBlockBegin"}
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
// {"magic":"","kind":"CriticalBlockEnd"}

impl Point {
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
    // {"magic":"","kind":"CriticalBlockBegin"}
    #[rustfmt::skip]
    pub fn new(store: &mut ObjectStore, y: i64, x: i64, ) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{}::{}::", y, x, ).as_bytes());
        let new = Self {
            id,
            y,
            x,
        };




        
        store.inter_point(new.clone());




        
        new
    }
    // {"magic":"","kind":"CriticalBlockEnd"}
}
/// The Edge of an Object Depiction
///
///
///
///
/// 
/// There are four edges to a rendered object.
///
/// _Generated code_
// {"magic":"","kind":"CriticalBlockBegin"}
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
// {"magic":"","kind":"CriticalBlockEnd"}

impl ObjectEdge {
    /// Inter a new ObjectEdge and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::drawing::ObjectEdge;
    /// # use sarzak::drawing::ObjectUi;
    /// # use sarzak::drawing::Edge;
    /// # use sarzak::drawing::Point;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    ///
    /// let edge_spz = Edge::test_default(&mut store);
    /// let point_rra = Point::new(&mut store, 42, 42);
    /// let petite_hose = "wakeful_science".to_owned();
    /// let object_brf = Object::default();
    ///
    /// let object_ui_gfp = ObjectUi::new(&mut store, &point_rra, &object_brf, 42, 42);
    ///
    /// let object_edge = ObjectEdge::new(&mut store, &edge_spz, &object_ui_gfp);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    // {"magic":"","kind":"CriticalBlockBegin"}
    #[rustfmt::skip]
    pub fn new(store: &mut ObjectStore, edge: &Edge, oui_id: &ObjectUi, ) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::{:?}::", edge, oui_id, ).as_bytes());
        let new = Self {
            id,
            edge: edge.get_id(),
            oui_id: oui_id.id,
        };




        
        store.inter_object_edge(new.clone());




        
        new
    }
    // {"magic":"","kind":"CriticalBlockEnd"}
}
/// The top edge of the rendered box
///
///
///
///
/// 
/// ❗️{"singleton_object": true}
///
/// _Generated code_
//
pub const TOP: Uuid = uuid!["66416a6d-1227-53b5-bb5a-bfd45e0ea72e"];

/// The left side of a rendered box
///
///
///
///
/// 
/// ❗️{"singleton_object": true}
///
/// _Generated code_
//
pub const LEFT: Uuid = uuid!["8125ae7e-9edb-5e9c-be33-643f1277e0e0"];

/// An attachment point for an [Anchor]
///
///
///
///
/// 
/// It’s used with [Anchor] to orient the arrows on the ends of the lines according to the
/// side of the box to which they are attached. Some arrows are on top, some bottom, etc.
///
///
///
///
/// 
/// This is not rendered as a visible item. The [ObjectUI] manages that by itself. This instead
/// renders an invisible line. The line is used for several things. For one, when hovered over
/// the cursor changes to the appropriate one for resizing.
///
///
///
///
/// 
/// Also, this is used to register where relationship may anchor.
///
///
///
///
/// 
/// It’s this last regard that is somewhat concerning. Indicating that an anchor is attached
/// to an edge get’s us the connection we need between an [Object] and a [Relationship]. But
/// it’s under-specified. It doesn’t indicate where along the edge the arrow is connected
///.
///
///
///
///
/// 
/// I’m considering put a relationship back between [Anchor] and [Point].
///
/// _Generated code_
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

impl Edge {
    // {"magic":"","kind":"CriticalBlockBegin"}
    pub fn test_default(store: &mut ObjectStore) -> Self {
        let test = Self::Top(TOP);

        
        store.inter_edge(test.clone());

        
        test
    }
    // {"magic":"","kind":"CriticalBlockEnd"}
}

/// Additional information necessary to render relationships in the user interface.
///
/// _Generated code_
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

impl RelationshipUi {
    // {"magic":"","kind":"CriticalBlockBegin"}
    pub fn test_default(store: &mut ObjectStore) -> Self {
        // This is a totally valid, if wasteful, and odd thing to do. Sorry. 🐶
        // ⚡️         let point_auv = Point::new(store, 42, 42);
        // ⚡️         let point_psb = Point::new(store, 42, 42);
        // ⚡️         let edge_gjl = Edge::test_default(store);
        // ⚡️         let anchor_iao = Anchor::new(store, &point_auv, &point_psb, &edge_gjl);
        // ⚡️         let point_jtc = Point::new(store, 42, 42);
        // ⚡️         let point_dwm = Point::new(store, 42, 42);
        // ⚡️         let edge_qkr = Edge::test_default(store);
        // ⚡️         let anchor_tux = Anchor::new(store, &point_jtc, &point_dwm, &edge_qkr);
        // ⚡️         let binary_byu = Binary::default();

        // ⚡️         let test = Self::BinaryUi(BinaryUi::new(store, &anchor_iao, &anchor_tux, &binary_byu).id);

        // ⚡️         let point_gah = Point::new(store, 42, 42);
        // ⚡️         let point_zlr = Point::new(store, 42, 42);
        // ⚡️         let edge_jrb = Edge::test_default(store);
        // ⚡️         let anchor_nqc = Anchor::new(store, &point_gah, &point_zlr, &edge_jrb);
        // ⚡️         let point_sbd = Point::new(store, 42, 42);
        // ⚡️         let point_jkc = Point::new(store, 42, 42);
        // ⚡️         let edge_yyt = Edge::test_default(store);
        // ⚡️         let anchor_gug = Anchor::new(store, &point_sbd, &point_jkc, &edge_yyt);
        // ⚡️         let binary_duc = Binary::default();

        // ⚡️         let test = Self::BinaryUi(BinaryUi::new(store, &anchor_nqc, &anchor_gug, &binary_duc).id);

        // ⚡️         let point_deg = Point::new(store, 42, 42);
        // ⚡️         let point_xyu = Point::new(store, 42, 42);
        // ⚡️         let edge_mhg = Edge::test_default(store);
        // ⚡️         let anchor_ton = Anchor::new(store, &point_deg, &point_xyu, &edge_mhg);
        // ⚡️         let point_psq = Point::new(store, 42, 42);
        // ⚡️         let point_gjc = Point::new(store, 42, 42);
        // ⚡️         let edge_lrv = Edge::test_default(store);
        // ⚡️         let anchor_xse = Anchor::new(store, &point_psq, &point_gjc, &edge_lrv);
        // ⚡️         let binary_dnz = Binary::default();

        // ⚡️         let test = Self::BinaryUi(BinaryUi::new(store, &anchor_ton, &anchor_xse, &binary_dnz).id);

        // ⚡️         let point_mnz = Point::new(store, 42, 42);
        // ⚡️         let point_ofb = Point::new(store, 42, 42);
        // ⚡️         let edge_xnz = Edge::test_default(store);
        // ⚡️         let anchor_xir = Anchor::new(store, &point_mnz, &point_ofb, &edge_xnz);
        // ⚡️         let point_nqn = Point::new(store, 42, 42);
        // ⚡️         let point_gqf = Point::new(store, 42, 42);
        // ⚡️         let edge_goz = Edge::test_default(store);
        // ⚡️         let anchor_nvz = Anchor::new(store, &point_nqn, &point_gqf, &edge_goz);
        // ⚡️         let binary_txe = Binary::default();
// ⚡️         let point_omf = Point::new(store, 42, 42);
// ⚡️         let point_tcq = Point::new(store, 42, 42);
// ⚡️         let edge_hfn = Edge::test_default(store);
// ⚡️         let anchor_fsm = Anchor::new(store, &point_omf, &point_tcq, &edge_hfn);
// ⚡️         let point_fua = Point::new(store, 42, 42);
// ⚡️         let point_gox = Point::new(store, 42, 42);
// ⚡️         let edge_set = Edge::test_default(store);
// ⚡️         let anchor_jrx = Anchor::new(store, &point_fua, &point_gox, &edge_set);
// ⚡️         let binary_ptl = Binary::default();

        // ⚡️         let test = Self::BinaryUi(BinaryUi::new(store, &anchor_xir, &anchor_nvz, &binary_txe).id);
// ⚡️         let test = Self::BinaryUi(BinaryUi::new(store, &anchor_fsm, &anchor_jrx, &binary_ptl).id);

        let point_qtd = Point::new(store, 42, 42);
        let point_fuv = Point::new(store, 42, 42);
        let edge_hhm = Edge::test_default(store);
        let anchor_zwt = Anchor::new(store, &point_qtd, &point_fuv, &edge_hhm);
        let point_enr = Point::new(store, 42, 42);
        let point_ifm = Point::new(store, 42, 42);
        let edge_etr = Edge::test_default(store);
        let anchor_uzf = Anchor::new(store, &point_enr, &point_ifm, &edge_etr);
        let binary_rov = Binary::default();
        
        let test = Self::BinaryUi(BinaryUi::new(store, &anchor_zwt, &anchor_uzf, &binary_rov).id);
        
        store.inter_relationship_ui(test.clone());

        
        test
    }
    // {"magic":"","kind":"CriticalBlockEnd"}
}

/// Render a rectangle
///
///
///
///
/// 
/// This represents additional information that is necessary to draw an Object in the user interface
///.
///
///
///
///
/// 
/// Note that although we are logically related to [Edge] via `R14` we actually render our own
/// edges. We use the svg rect primitive to do this.
///
///
///
///
/// 
/// I’m throwing this in for the fuck of it. I don’t know if it’ll be useful or not.
///
///
///
///
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
/// _Generated code_
// {"magic":"","kind":"CriticalBlockBegin"}
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
// {"magic":"","kind":"CriticalBlockEnd"}

impl ObjectUi {
    /// Inter a new ObjectUi and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::drawing::ObjectUi;
    /// # use sarzak::drawing::Point;
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    ///
    /// let point_jig = Point::new(&mut store, 42, 42);
    /// let electric_body = "acoustic_suggestion".to_owned();
    /// let object_fue = Object::default();
    ///
    ///
    /// let object_ui = ObjectUi::new(&mut store, &point_jig, &object_fue, 42, 42);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    // {"magic":"","kind":"CriticalBlockBegin"}
    #[rustfmt::skip]
    pub fn new(store: &mut ObjectStore, origin: &Point, object_id: &Object, width: i64, height: i64, ) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::{:?}::{}::{}::", origin, object_id, width, height, ).as_bytes());
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
    // {"magic":"","kind":"CriticalBlockEnd"}
}
/// This represents additional data necessary to render an `Isa` relationship in the user interface
///.
///
/// _Generated code_
// {"magic":"","kind":"CriticalBlockBegin"}
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
// {"magic":"","kind":"CriticalBlockEnd"}

impl IsaUi {
    /// Inter a new IsaUi and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::drawing::Anchor;
    /// # use sarzak::sarzak::Isa;
    /// # use sarzak::drawing::Edge;
    /// # use sarzak::drawing::Point;
    /// # use sarzak::drawing::IsaUi;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    /// # let mut sarzak_store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let isa_tqx = Isa::test_default(&mut sarzak_store);
    ///
    /// let point_ksc = Point::new(&mut store, 42, 42);
    /// let point_xjh = Point::new(&mut store, 42, 42);
    /// let edge_hnv = Edge::test_default(&mut store);
    /// let anchor_coc = Anchor::new(&mut store, &point_ksc, &point_xjh, &edge_hnv);
    ///
    /// let isa_ui = IsaUi::new(&mut store, &isa_tqx, &anchor_coc);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    // {"magic":"","kind":"CriticalBlockBegin"}
    #[rustfmt::skip]
    pub fn new(store: &mut ObjectStore, isa: &Isa, from: &Anchor, ) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::{:?}::", isa, from, ).as_bytes());
        let new = Self {
            id,
            isa: isa.get_id(),
            from: from.id,
        };




        
        store.inter_isa_ui(new.clone());




        
        new
    }
    // {"magic":"","kind":"CriticalBlockEnd"}
}
// {"magic":"","kind":"CriticalBlockBegin"}
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
// {"magic":"","kind":"CriticalBlockEnd"}

impl AssociativeUi {
    /// Inter a new AssociativeUi and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Associative;
    /// # use sarzak::drawing::Anchor;
    /// # use sarzak::drawing::Edge;
    /// # use sarzak::drawing::AssociativeUi;
    /// # use sarzak::drawing::Point;
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    ///
    /// let associative_bwp = Associative::default();
    ///
    /// let point_kzk = Point::new(&mut store, 42, 42);
    /// let point_kbb = Point::new(&mut store, 42, 42);
    /// let edge_umy = Edge::test_default(&mut store);
    /// let anchor_oum = Anchor::new(&mut store, &point_kzk, &point_kbb, &edge_umy);
    /// let point_kid = Point::new(&mut store, 42, 42);
    /// let point_jaa = Point::new(&mut store, 42, 42);
    /// let edge_ugb = Edge::test_default(&mut store);
    /// let anchor_wtu = Anchor::new(&mut store, &point_kid, &point_jaa, &edge_ugb);
    /// let point_wvl = Point::new(&mut store, 42, 42);
    /// let point_wzf = Point::new(&mut store, 42, 42);
    /// let edge_uke = Edge::test_default(&mut store);
    /// let anchor_qym = Anchor::new(&mut store, &point_wvl, &point_wzf, &edge_uke);
    /// let point_lhn = Point::new(&mut store, 42, 42);
    ///
    /// let associative_ui = AssociativeUi::new(&mut store, &associative_bwp, &anchor_oum, &anchor_wtu, &anchor_qym, &point_lhn);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    // {"magic":"","kind":"CriticalBlockBegin"}
    #[rustfmt::skip]
    pub fn new(store: &mut ObjectStore, associative_id: &Associative, from: &Anchor, one: &Anchor, other: &Anchor, middle: &Point, ) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::{:?}::{:?}::{:?}::{:?}::", associative_id, from, one, other, middle, ).as_bytes());
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
    // {"magic":"","kind":"CriticalBlockEnd"}
}
/// Subtype Anchors
///
///
///
///
/// 
/// Just as it sounds, these are [`Anchor`]s used by [`Subtype`]s in an [`Isa`] relationship
///.
///
/// _Generated code_
// {"magic":"","kind":"CriticalBlockBegin"}
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
// {"magic":"","kind":"CriticalBlockEnd"}

impl SubtypeAnchors {
    /// Inter a new SubtypeAnchors and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::drawing::IsaUi;
    /// # use sarzak::drawing::Anchor;
    /// # use sarzak::drawing::Point;
    /// # use sarzak::drawing::SubtypeAnchors;
    /// # use sarzak::sarzak::Isa;
    /// # use sarzak::drawing::Edge;
    ///
    /// # let mut store = sarzak::drawing::ObjectStore::new();
    /// # let mut sarzak_store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let point_lzn = Point::new(&mut store, 42, 42);
    /// let point_fxr = Point::new(&mut store, 42, 42);
    /// let edge_xvg = Edge::test_default(&mut store);
    /// let anchor_rek = Anchor::new(&mut store, &point_lzn, &point_fxr, &edge_xvg);
    /// let isa_wbm = Isa::test_default(&mut sarzak_store);
    ///
    /// let point_ibm = Point::new(&mut store, 42, 42);
    /// let point_ren = Point::new(&mut store, 42, 42);
    /// let edge_scq = Edge::test_default(&mut store);
    /// let anchor_hva = Anchor::new(&mut store, &point_ibm, &point_ren, &edge_scq);
    /// let isa_ui_sqd = IsaUi::new(&mut store, &isa_wbm, &anchor_hva);
    ///
    /// let subtype_anchors = SubtypeAnchors::new(&mut store, &anchor_rek, &isa_ui_sqd);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    // {"magic":"","kind":"CriticalBlockBegin"}
    #[rustfmt::skip]
    pub fn new(store: &mut ObjectStore, anchor_id: &Anchor, isaui_id: &IsaUi, ) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::{:?}::", anchor_id, isaui_id, ).as_bytes());
        let new = Self {
            id,
            anchor_id: anchor_id.id,
            isaui_id: isaui_id.id,
        };




        
        store.inter_subtype_anchors(new.clone());




        
        new
    }
    // {"magic":"","kind":"CriticalBlockEnd"}
}
/// The right side of a rendered box
///
///
///
///
/// 
/// ❗️{"singleton_object": true}
///
/// _Generated code_
//
pub const RIGHT: Uuid = uuid!["1d99f96f-d110-5adf-a108-9fb0b707dae3"];

/// The bottom of a rendered box
///
///
///
///
/// 
/// ❗️{"singleton_object": true}
///
/// _Generated code_
//
pub const BOTTOM: Uuid = uuid!["cd977757-dbcb-5e5d-a0dc-d0e6624db6a0"];

