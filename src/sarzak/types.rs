//! Types for instances of the "Sarzak" domain
//! # Domain Description
//!
//! The _Metamodel_
//!
//! This is the model of the model. From here all is generated...
//!
//!
//! # Contents
//!
//! The following types are defined herein:
//!    * [`AcknowledgedEvent`]
//!    * [`Associative`]
//!    * [`AssociativeSide`]
//!    * [`Attribute`]
//!    * [`Binary`]
//!    * [`BOOLEAN`]
//!    * [`Cardinality`]
//!    * [`CONDITIONAL`]
//!    * [`Conditionality`]
//!    * [`Event`]
//!    * [`FLOAT`]
//!    * [`INTEGER`]
//!    * [`Isa`]
//!    * [`MANY`]
//!    * [`Object`]
//!    * [`ONE`]
//!    * [`OneSide`]
//!    * [`OtherSide`]
//!    * [`Referent`]
//!    * [`Referrer`]
//!    * [`Relationship`]
//!    * [`State`]
//!    * [`STRING`]
//!    * [`Subtype`]
//!    * [`Supertype`]
//!    * [`Type`]
//!    * [`UUID`]
//!    * [`UNCONDITIONAL`]
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
use crate::sarzak::store::ObjectStore;
use crate::sarzak::UUID_NS;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"imports"}}}

/// An Event that Does Something
///
/// An acknowledged event is an event that a [`State`] knows how to handle.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"acknowledged_event-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AcknowledgedEvent {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub event_id: `Event`,
    ///
    pub event_id: Uuid,
    /// pub state_id: `State`,
    ///
    pub state_id: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"acknowledged_event-struct-definition"}}}

impl AcknowledgedEvent {
    // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"acknowledged_event-new_impl"}}}
    /// Inter a new AcknowledgedEvent and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::AcknowledgedEvent;
    /// # use sarzak::sarzak::Event;
    /// # use sarzak::sarzak::State;
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let thoughtless_representative = "jagged_lip".to_owned();
    /// let object_mvq = Object::new(&mut store, thoughtless_representative);
    /// let brown_rat = "defiant_zebra".to_owned();
    /// let state_mfg = State::new(&mut store, &object_mvq, brown_rat);
    /// let medical_texture = "fluffy_door".to_owned();
    /// let object_ytp = Object::new(&mut store, medical_texture);
    /// let alcoholic_bulb = "verdant_visitor".to_owned();
    /// let event_bdj = Event::new(&mut store, &object_ytp, alcoholic_bulb);
    ///
    /// let acknowledged_event = AcknowledgedEvent::new(&mut store, &state_mfg, &event_bdj);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, state_id: &State, event_id: &Event) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{:?}::{:?}::", state_id, event_id,).as_bytes(),
        );
        let new = Self {
            id,
            state_id: state_id.id,
            event_id: event_id.id,
        };

        store.inter_acknowledged_event(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"acknowledged_event-new_impl"}}}
}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"associative-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Associative {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub number: `i64`,
    ///
    pub number: i64,
    /// pub from: `Associative Side`,
    ///
    pub from: Uuid,
    /// pub one: `One Side`,
    ///
    pub one: Uuid,
    /// pub other: `Other Side`,
    ///
    pub other: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"associative-struct-definition"}}}

impl Associative {
    // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"associative-new_impl"}}}
    /// Inter a new Associative and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::OneSide;
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::OtherSide;
    /// # use sarzak::sarzak::AssociativeSide;
    /// # use sarzak::sarzak::Associative;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let wandering_bed = "damp_trail".to_owned();
    /// let object_umz = Object::new(&mut store, wandering_bed);
    /// let one_side_fxk = OneSide::new(&mut store, &object_umz);
    /// let placid_profit = "large_fold".to_owned();
    /// let object_bwf = Object::new(&mut store, placid_profit);
    /// let other_side_tle = OtherSide::new(&mut store, &object_bwf);
    /// let snotty_roof = "jumbled_purpose".to_owned();
    /// let object_fjm = Object::new(&mut store, snotty_roof);
    /// let associative_side_nmw = AssociativeSide::new(&mut store, &object_fjm);
    ///
    /// let associative = Associative::new(&mut store, &one_side_fxk, &other_side_tle, &associative_side_nmw, 42);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(
        store: &mut ObjectStore,
        one: &OneSide,
        other: &OtherSide,
        from: &AssociativeSide,
        number: i64,
    ) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{:?}::{:?}::{:?}::{}::", one, other, from, number,).as_bytes(),
        );
        let new = Self {
            id,
            one: one.id,
            other: other.id,
            from: from.id,
            number,
        };

        store.inter_associative(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"associative-new_impl"}}}
}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"associative_side-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AssociativeSide {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub obj_id: `Object`,
    ///
    pub obj_id: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"associative_side-struct-definition"}}}

impl AssociativeSide {
    // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"associative_side-new_impl"}}}
    /// Inter a new AssociativeSide and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::AssociativeSide;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let thinkable_cannon = "obese_achieve".to_owned();
    /// let object_yeu = Object::new(&mut store, thinkable_cannon);
    ///
    /// let associative_side = AssociativeSide::new(&mut store, &object_yeu);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, obj_id: &Object) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::", obj_id,).as_bytes());
        let new = Self {
            id,
            obj_id: obj_id.id,
        };

        store.inter_associative_side(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"associative_side-new_impl"}}}
}

/// An `Attribute` represents a single value. Each value must have a
/// [`Type`], which constrains the values of data that may be assigned to
/// an `Attribute`.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"attribute-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Attribute {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub name: `std::string::String`,
    ///
    pub name: std::string::String,
    /// pub obj_id: `Option<Object>`,
    ///
    pub obj_id: Option<Uuid>,
    /// pub r#type: `Type`,
    ///
    pub ty: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"attribute-struct-definition"}}}

impl Attribute {
    // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"attribute-new_impl"}}}
    /// Inter a new Attribute and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Attribute;
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Type;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let vagabond_boundary = "spooky_play".to_owned();
    /// let object = Object::new(&mut store, vagabond_boundary);
    /// let type_eoq = Type::test_default(&mut store);
    /// let macho_friends = "frightening_tail".to_owned();
    ///
    /// let attribute = Attribute::new(&mut store, Some(&object), &type_eoq, macho_friends);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(
        store: &mut ObjectStore,
        obj_id: Option<&Object>,
        ty: &Type,
        name: std::string::String,
    ) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{:?}::{:?}::{}::", obj_id, ty, name,).as_bytes(),
        );
        let new = Self {
            id,
            obj_id: obj_id.map(|o| o.id),
            ty: ty.get_id(),
            name,
        };

        store.inter_attribute(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"attribute-new_impl"}}}
}

/// A `Binary` relationship, as it’s name implies, is a relationship between
/// two objects. It consists of two parts, the `Dependent` end of the
/// relationship and the `Independent` end.
///
/// The former is so named because it has the job of formalizing the
/// relationship. It stores a pointer to the independent object as an attribute.
///
/// The latter is aware of the relationship, but it does not store any
/// information about the relationship. That said, there are means of
/// traversing the relationship from the `Independent` object.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"binary-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Binary {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub number: `i64`,
    ///
    pub number: i64,
    /// pub from: `Referrer`,
    ///
    pub from: Uuid,
    /// pub to: `Referent`,
    ///
    pub to: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"binary-struct-definition"}}}

impl Binary {
    // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"binary-new_impl"}}}
    /// Inter a new Binary and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Referrer;
    /// # use sarzak::sarzak::Conditionality;
    /// # use sarzak::sarzak::Referent;
    /// # use sarzak::sarzak::Binary;
    /// # use sarzak::sarzak::Cardinality;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let conditionality_wtm = Conditionality::test_default(&mut store);
    /// let damaged_ladybug = "numberless_quilt".to_owned();
    /// let object_mgf = Object::new(&mut store, damaged_ladybug);
    /// let cardinality_ycz = Cardinality::test_default(&mut store);
    /// let referent_ztd = Referent::new(&mut store, &conditionality_wtm, &object_mgf, &cardinality_ycz);
    /// let conditionality_gxo = Conditionality::test_default(&mut store);
    /// let faint_cracker = "delicious_zoo".to_owned();
    /// let object_wie = Object::new(&mut store, faint_cracker);
    /// let cardinality_atg = Cardinality::test_default(&mut store);
    /// let aberrant_bushes = "spiteful_bee".to_owned();
    /// let referrer_ngd = Referrer::new(&mut store, &conditionality_gxo, &object_wie, &cardinality_atg, aberrant_bushes);
    ///
    /// let binary = Binary::new(&mut store, &referent_ztd, &referrer_ngd, 42);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, to: &Referent, from: &Referrer, number: i64) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{:?}::{:?}::{}::", to, from, number,).as_bytes(),
        );
        let new = Self {
            id,
            to: to.id,
            from: from.id,
            number,
        };

        store.inter_binary(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"binary-new_impl"}}}
}

/// The Boolean Type
///
/// This type holds `true` and `false` values. This type is just a placeholder. It's implementation
/// is determined downstream by the code generator.
///
/// ❗️{"singleton_object": true}
///
//
pub const BOOLEAN: Uuid = uuid!["4554e9f9-0506-5fde-836c-07cb3cbb0399"];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Cardinality {
    /// `One(One)`,
    ///
    One(Uuid),
    /// `Many(Many)`,
    ///
    Many(Uuid),
}

impl Cardinality {
    pub fn get_id(&self) -> Uuid {
        match *self {
            Self::One(z) => z,
            Self::Many(z) => z,
        }
    }
}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"cardinality-test_default"}}}
impl Cardinality {
    pub fn test_default(store: &mut ObjectStore) -> Self {
        let test = Self::One(ONE);

        store.inter_cardinality(test.clone());

        test
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"cardinality-test_default"}}}

/// A constant value that indicates a conditionality of _conditional_.
///
/// ❗️{"singleton_object": true}
///
//
pub const CONDITIONAL: Uuid = uuid!["fc6aa4ae-4ab5-5b43-a7c1-52bbd3e69f34"];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Conditionality {
    /// `Unconditional(Unconditional)`,
    ///
    Unconditional(Uuid),
    /// `Conditional(Conditional)`,
    ///
    Conditional(Uuid),
}

impl Conditionality {
    pub fn get_id(&self) -> Uuid {
        match *self {
            Self::Unconditional(z) => z,
            Self::Conditional(z) => z,
        }
    }
}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"conditionality-test_default"}}}
impl Conditionality {
    pub fn test_default(store: &mut ObjectStore) -> Self {
        let test = Self::Unconditional(UNCONDITIONAL);

        store.inter_conditionality(test.clone());

        test
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"conditionality-test_default"}}}

/// An event is sent to an object, and processed by the current state. Assuming it accepts the
/// event. Otherwise it’s dropped on the floor.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"event-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Event {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub name: `std::string::String`,
    ///
    pub name: std::string::String,
    /// pub obj_id: `Object`,
    ///
    pub obj_id: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"event-struct-definition"}}}

impl Event {
    // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"event-new_impl"}}}
    /// Inter a new Event and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Event;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let exclusive_pie = "chunky_pleasure".to_owned();
    /// let object_whr = Object::new(&mut store, exclusive_pie);
    /// let aberrant_ship = "keen_drop".to_owned();
    ///
    /// let event = Event::new(&mut store, &object_whr, aberrant_ship);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, obj_id: &Object, name: std::string::String) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::{}::", obj_id, name,).as_bytes());
        let new = Self {
            id,
            obj_id: obj_id.id,
            name,
        };

        store.inter_event(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"event-new_impl"}}}
}

/// The Floating Point Type
///
/// This type holds numbers from ℝ. This type is just a placeholder. It's implementation is
/// determined downstream by the code generator.
///
/// ❗️{"singleton_object": true}
///
//
pub const FLOAT: Uuid = uuid!["8ca8decc-f87b-587a-a390-593d20203b6f"];

/// The Integer Type
///
/// This is an interger that can hold positive and negative values. This type is just a placeholder
///. It's implementation is determined downstream by the code generator.
///
/// ❗️{"singleton_object": true}
///
//
pub const INTEGER: Uuid = uuid!["70ec7fbd-44a2-5800-8558-349e3b22cf17"];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Isa {
    /// `Subtype(Subtype)`,
    ///
    Subtype(Uuid),
    /// `Supertype(Supertype)`,
    ///
    Supertype(Uuid),
}

impl Isa {
    pub fn get_id(&self) -> Uuid {
        match *self {
            Self::Subtype(z) => z,
            Self::Supertype(z) => z,
        }
    }
}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"isa-test_default"}}}
impl Isa {
    pub fn test_default(store: &mut ObjectStore) -> Self {
        // {"magic":"","kind":"IgnoreBlockBegin"}
        let uppity_slave = "calm_drop".to_owned();
        let object_rfh = Object::new(store, uppity_slave);
        let test = Self::Subtype(Subtype::new(store, &object_rfh).id);
        // {"magic":"","kind":"IgnoreBlockEnd"}

        store.inter_isa(test.clone());

        test
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"isa-test_default"}}}

/// A constant value that indicates a cardinality of _many_.
///
/// ❗️{"singleton_object": true}
///
//
pub const MANY: Uuid = uuid!["0614a507-4422-5994-a59d-68dc57d2c328"];

/// An `Object` is a collection of related data. By creating `Object`s, and
/// connecting them with `Relationships` we build a powerful abstraction.
///
/// `Object`s contain [Attribute]s that represent the data that the
/// `Object`encapsulates. All `Object`s have an attribute called `id`, which
/// is a unique idenifier for each class of `Object`. The `id` attribute is a
/// version 5 UUID.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"object-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Object {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub name: `std::string::String`,
    ///
    pub name: std::string::String,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"object-struct-definition"}}}

impl Object {
    // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"object-new_impl"}}}
    /// Inter a new Object and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let super_support = "sick_effect".to_owned();
    ///
    /// let object = Object::new(&mut store, super_support);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, name: std::string::String) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{}::", name,).as_bytes());
        let new = Self { id, name };

        store.inter_object(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"object-new_impl"}}}
}

/// A constant value that indicates a cardinality of _one_.
///
/// ❗️{"singleton_object": true}
///
//
pub const ONE: Uuid = uuid!["bf6924bb-089d-5c1f-bc1f-123ba1fd1ea3"];

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"one_side-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OneSide {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub obj_id: `Object`,
    ///
    pub obj_id: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"one_side-struct-definition"}}}

impl OneSide {
    // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"one_side-new_impl"}}}
    /// Inter a new OneSide and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::OneSide;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let third_baseball = "yielding_legs".to_owned();
    /// let object_zcc = Object::new(&mut store, third_baseball);
    ///
    /// let one_side = OneSide::new(&mut store, &object_zcc);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, obj_id: &Object) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::", obj_id,).as_bytes());
        let new = Self {
            id,
            obj_id: obj_id.id,
        };

        store.inter_one_side(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"one_side-new_impl"}}}
}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"other_side-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OtherSide {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub obj_id: `Object`,
    ///
    pub obj_id: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"other_side-struct-definition"}}}

impl OtherSide {
    // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"other_side-new_impl"}}}
    /// Inter a new OtherSide and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::OtherSide;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let delicious_operation = "willing_kite".to_owned();
    /// let object_drt = Object::new(&mut store, delicious_operation);
    ///
    /// let other_side = OtherSide::new(&mut store, &object_drt);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, obj_id: &Object) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::", obj_id,).as_bytes());
        let new = Self {
            id,
            obj_id: obj_id.id,
        };

        store.inter_other_side(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"other_side-new_impl"}}}
}

/// This is the side being referred to in a binary relationship. It is the “to” side.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"referent-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Referent {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub cardinality: `Cardinality`,
    ///
    pub cardinality: Uuid,
    /// pub conditionality: `Conditionality`,
    ///
    pub conditionality: Uuid,
    /// pub obj_id: `Object`,
    ///
    pub obj_id: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"referent-struct-definition"}}}

impl Referent {
    // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"referent-new_impl"}}}
    /// Inter a new Referent and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Conditionality;
    /// # use sarzak::sarzak::Referent;
    /// # use sarzak::sarzak::Cardinality;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let conditionality_rcy = Conditionality::test_default(&mut store);
    /// let alluring_plantation = "abnormal_game".to_owned();
    /// let object_irt = Object::new(&mut store, alluring_plantation);
    /// let cardinality_psm = Cardinality::test_default(&mut store);
    ///
    /// let referent = Referent::new(&mut store, &conditionality_rcy, &object_irt, &cardinality_psm);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(
        store: &mut ObjectStore,
        conditionality: &Conditionality,
        obj_id: &Object,
        cardinality: &Cardinality,
    ) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{:?}::{:?}::{:?}::", conditionality, obj_id, cardinality,).as_bytes(),
        );
        let new = Self {
            id,
            conditionality: conditionality.get_id(),
            obj_id: obj_id.id,
            cardinality: cardinality.get_id(),
        };

        store.inter_referent(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"referent-new_impl"}}}
}

/// This is the side of a binary relationship that is doing the pointing, thus it contains the
/// referential attribute. It is connected to the “from” side of a binary relationship.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"referrer-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Referrer {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub referential_attribute: `std::string::String`,
    ///
    pub referential_attribute: std::string::String,
    /// pub cardinality: `Cardinality`,
    ///
    pub cardinality: Uuid,
    /// pub conditionality: `Conditionality`,
    ///
    pub conditionality: Uuid,
    /// pub obj_id: `Object`,
    ///
    pub obj_id: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"referrer-struct-definition"}}}

impl Referrer {
    // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"referrer-new_impl"}}}
    /// Inter a new Referrer and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Referrer;
    /// # use sarzak::sarzak::Cardinality;
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Conditionality;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let conditionality_adh = Conditionality::test_default(&mut store);
    /// let mellow_stitch = "abounding_health".to_owned();
    /// let object_okw = Object::new(&mut store, mellow_stitch);
    /// let cardinality_zcu = Cardinality::test_default(&mut store);
    /// let squalid_dogs = "incandescent_dirt".to_owned();
    ///
    /// let referrer = Referrer::new(&mut store, &conditionality_adh, &object_okw, &cardinality_zcu, squalid_dogs);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(
        store: &mut ObjectStore,
        conditionality: &Conditionality,
        obj_id: &Object,
        cardinality: &Cardinality,
        referential_attribute: std::string::String,
    ) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!(
                "{:?}::{:?}::{:?}::{}::",
                conditionality, obj_id, cardinality, referential_attribute,
            )
            .as_bytes(),
        );
        let new = Self {
            id,
            conditionality: conditionality.get_id(),
            obj_id: obj_id.id,
            cardinality: cardinality.get_id(),
            referential_attribute,
        };

        store.inter_referrer(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"referrer-new_impl"}}}
}

/// A `Relationship` indicates that a set of objects are connected to each other in some manner
///. Typically it is a _real world_ relationship. In the
/// case of this model it is strictly an abstraction.
///
/// There are three types of `Relationship`: [`Isa`], [`Binary`], and [`Associative`]. Thus
/// `Relationship` is itself the *supertype* in an [`Isa`] relationship. It is a partitioning
/// *supertype-subtype* relationship, rather one of inheritance. As such, it’s  perfectly
/// suited to a rust `enum`! 😃
///
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Relationship {
    /// `Isa(Isa)`,
    ///
    Isa(Uuid),
    /// `Associative(Associative)`,
    ///
    Associative(Uuid),
    /// `Binary(Binary)`,
    ///
    Binary(Uuid),
}

impl Relationship {
    pub fn get_id(&self) -> Uuid {
        match *self {
            Self::Isa(z) => z,
            Self::Associative(z) => z,
            Self::Binary(z) => z,
        }
    }
}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"relationship-test_default"}}}
impl Relationship {
    pub fn test_default(store: &mut ObjectStore) -> Self {
        let test = Self::Isa(Isa::test_default(store).get_id());

        store.inter_relationship(test.clone());

        test
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"relationship-test_default"}}}

/// An [Object] state, more precisely, a set of states, is where all the action happens.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"state-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct State {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub name: `std::string::String`,
    ///
    pub name: std::string::String,
    /// pub obj_id: `Object`,
    ///
    pub obj_id: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"state-struct-definition"}}}

impl State {
    // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"state-new_impl"}}}
    /// Inter a new State and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::State;
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let sore_man = "curvy_temper".to_owned();
    /// let object_cul = Object::new(&mut store, sore_man);
    /// let brave_sister = "romantic_stretch".to_owned();
    ///
    /// let state = State::new(&mut store, &object_cul, brave_sister);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, obj_id: &Object, name: std::string::String) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::{}::", obj_id, name,).as_bytes());
        let new = Self {
            id,
            obj_id: obj_id.id,
            name,
        };

        store.inter_state(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"state-new_impl"}}}
}

/// The String Type
///
/// This type holds unicode characters. This type is just a placeholder. It's implementation
/// is determined downstream by the code generator.
///
/// ❗️{"singleton_object": true}
///
//
pub const STRING: Uuid = uuid!["d2f03ddf-cb09-546e-9a7a-c9d4e871efb0"];

/// The *subtype* in a *supertype-subtype* relationship.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"subtype-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Subtype {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub obj_id: `Object`,
    ///
    pub obj_id: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"subtype-struct-definition"}}}

impl Subtype {
    // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"subtype-new_impl"}}}
    /// Inter a new Subtype and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Subtype;
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let black_and_white_jeans = "cool_paint".to_owned();
    /// let object_tgn = Object::new(&mut store, black_and_white_jeans);
    ///
    /// let subtype = Subtype::new(&mut store, &object_tgn);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, obj_id: &Object) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::", obj_id,).as_bytes());
        let new = Self {
            id,
            obj_id: obj_id.id,
        };

        store.inter_subtype(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"subtype-new_impl"}}}
}

/// This object represents the *supertype* in a *supertype-subtype*
/// relationship.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"supertype-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Supertype {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub obj_id: `Object`,
    ///
    pub obj_id: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"supertype-struct-definition"}}}

impl Supertype {
    // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"supertype-new_impl"}}}
    /// Inter a new Supertype and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Supertype;
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let five_bomb = "deranged_whip".to_owned();
    /// let object_ytj = Object::new(&mut store, five_bomb);
    ///
    /// let supertype = Supertype::new(&mut store, &object_ytj);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, obj_id: &Object) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::", obj_id,).as_bytes());
        let new = Self {
            id,
            obj_id: obj_id.id,
        };

        store.inter_supertype(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"supertype-new_impl"}}}
}

/// The type of a value
///
/// There are several values available: [Integer], [Boolean], [Float], [String], and [UUID]
///.
///
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Type {
    /// `Boolean(Boolean)`,
    ///
    Boolean(Uuid),
    /// `String(String)`,
    ///
    String(Uuid),
    /// `Uuid(Uuid)`,
    ///
    Uuid(Uuid),
    /// `Float(Float)`,
    ///
    Float(Uuid),
    /// `Integer(Integer)`,
    ///
    Integer(Uuid),
}

impl Type {
    pub fn get_id(&self) -> Uuid {
        match *self {
            Self::Boolean(z) => z,
            Self::String(z) => z,
            Self::Uuid(z) => z,
            Self::Float(z) => z,
            Self::Integer(z) => z,
        }
    }
}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"ty-test_default"}}}
impl Type {
    pub fn test_default(store: &mut ObjectStore) -> Self {
        let test = Self::Boolean(BOOLEAN);

        store.inter_ty(test.clone());

        test
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"ty-test_default"}}}

/// The UUID Type
///
/// I feel like there are too many implementation details here.
///
/// This UUID is expected to be version 5. Generally we produce input
/// to the hash function from other UUIDs, coupled with additional
/// information from the creator to ensure a unique UUID.
///
/// The `ns` attribute is the namespace used to generate generate UUIDs
/// given a particular instance of `UUID`.
///
/// ❗️{"singleton_object": true, "translation_name": "SarzakUuid"}
///
//
pub const UUID: Uuid = uuid!["dc1639ca-7e20-5a39-92e5-9a478471b8e5"];

/// A constant value that indicates a conditionality of _unconditional_.
///
/// ❗️{"singleton_object": true}
///
//
pub const UNCONDITIONAL: Uuid = uuid!["0148e8ea-cf04-50f3-920c-b1aed9903e3a"];
