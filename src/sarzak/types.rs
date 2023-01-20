//! {"magic":"","version":"0.0.1"}
//! Types for instances of the "Sarzak" domain
//!
//! The following types are defined herein:
//!    * [`BOOLEAN`]
//!    * [`Isa`]
//!    * [`Associative`]
//!    * [`AcknowledgedEvent`]
//!    * [`Subtype`]
//!    * [`Conditionality`]
//!    * [`Relationship`]
//!    * [`Binary`]
//!    * [`Attribute`]
//!    * [`State`]
//!    * [`ONE`]
//!    * [`Object`]
//!    * [`Referent`]
//!    * [`STRING`]
//!    * [`Referrer`]
//!    * [`UUID`]
//!    * [`MANY`]
//!    * [`Supertype`]
//!    * [`UNCONDITIONAL`]
//!    * [`OneSide`]
//!    * [`Type`]
//!    * [`CONDITIONAL`]
//!    * [`Cardinality`]
//!    * [`Event`]
//!    * [`OtherSide`]
//!    * [`FLOAT`]
//!    * [`AssociativeSide`]
//!    * [`INTEGER`]
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
use crate::sarzak::store::ObjectStore;
use crate::sarzak::UUID_NS;
// {"magic":"","kind":"CriticalBlockEnd"}

// Imported Objects
// {"magic":"","kind":"CriticalBlockBegin"}
// {"magic":"","kind":"CriticalBlockEnd"}

/// The Boolean Type
///
///
///
///
/// 
/// This type holds `true` and `false` values. This type is just a placeholder. It's implementation
/// is determined downstream by the code generator.
///
///
///
///
/// 
/// ❗️{"singleton_object": true}
///
/// _Generated code_
//
pub const BOOLEAN: Uuid = uuid!["4554e9f9-0506-5fde-836c-07cb3cbb0399"];

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

impl Isa {
    // {"magic":"","kind":"CriticalBlockBegin"}
    pub fn test_default(store: &mut ObjectStore) -> Self {
        // This is a totally valid, if wasteful, and odd thing to do. Sorry. 🐶
        // ⚡️         let mighty_end = "melted_foot".to_owned();
        // ⚡️         let object_bwe = Object::new(store, mighty_end);
        // ⚡️         let test = Self::Subtype(Subtype::new(store, &object_bwe).id);

        // ⚡️         let dapper_song = "verdant_map".to_owned();
        // ⚡️         let object_pwg = Object::new(store, dapper_song);
        // ⚡️         let test = Self::Subtype(Subtype::new(store, &object_pwg).id);

        // ⚡️         let blue_eyed_rod = "thundering_sticks".to_owned();
        // ⚡️         let object_nhu = Object::new(store, blue_eyed_rod);
        // ⚡️         let test = Self::Subtype(Subtype::new(store, &object_nhu).id);
// ⚡️         let shut_question = "ceaseless_measure".to_owned();
// ⚡️         let object_dew = Object::new(store, shut_question);
// ⚡️         let test = Self::Subtype(Subtype::new(store, &object_dew).id);

        let numberless_quill = "vivacious_action".to_owned();
        let object_yay = Object::new(store, numberless_quill);
        let test = Self::Subtype(Subtype::new(store, &object_yay).id);
        
        store.inter_isa(test.clone());

        
        test
    }
    // {"magic":"","kind":"CriticalBlockEnd"}
}

// {"magic":"","kind":"CriticalBlockBegin"}
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
// {"magic":"","kind":"CriticalBlockEnd"}

impl Associative {
    /// Inter a new Associative and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::AssociativeSide;
    /// # use sarzak::sarzak::Associative;
    /// # use sarzak::sarzak::OtherSide;
    /// # use sarzak::sarzak::OneSide;
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let mellow_skirt = "phobic_level".to_owned();
    /// let object_pzu = Object::new(&mut store, mellow_skirt);
    /// let one_side_wbw = OneSide::new(&mut store, &object_pzu);
    /// let ludicrous_quince = "dynamic_ticket".to_owned();
    /// let object_zry = Object::new(&mut store, ludicrous_quince);
    /// let other_side_jls = OtherSide::new(&mut store, &object_zry);
    /// let witty_run = "grotesque_toothbrush".to_owned();
    /// let object_mxk = Object::new(&mut store, witty_run);
    /// let associative_side_mzv = AssociativeSide::new(&mut store, &object_mxk);
    ///
    /// let associative = Associative::new(&mut store, &one_side_wbw, &other_side_jls, &associative_side_mzv, 42);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    // {"magic":"","kind":"CriticalBlockBegin"}
    #[rustfmt::skip]
    pub fn new(store: &mut ObjectStore, one: &OneSide, other: &OtherSide, from: &AssociativeSide, number: i64, ) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::{:?}::{:?}::{}::", one, other, from, number, ).as_bytes());
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
    // {"magic":"","kind":"CriticalBlockEnd"}
}
/// An Event that Does Something
///
///
///
///
/// 
/// An acknowledged event is an event that a [`State`] knows how to handle.
///
/// _Generated code_
// {"magic":"","kind":"CriticalBlockBegin"}
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
// {"magic":"","kind":"CriticalBlockEnd"}

impl AcknowledgedEvent {
    /// Inter a new AcknowledgedEvent and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::AcknowledgedEvent;
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Event;
    /// # use sarzak::sarzak::State;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let bite_sized_word = "hapless_hot".to_owned();
    /// let object_jgp = Object::new(&mut store, bite_sized_word);
    /// let amuck_smile = "untidy_surprise".to_owned();
    /// let state_emu = State::new(&mut store, &object_jgp, amuck_smile);
    /// let rainy_wire = "axiomatic_pot".to_owned();
    /// let object_hfp = Object::new(&mut store, rainy_wire);
    /// let noisy_print = "skinny_basketball".to_owned();
    /// let event_xyk = Event::new(&mut store, &object_hfp, noisy_print);
    ///
    /// let acknowledged_event = AcknowledgedEvent::new(&mut store, &state_emu, &event_xyk);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    // {"magic":"","kind":"CriticalBlockBegin"}
    #[rustfmt::skip]
    pub fn new(store: &mut ObjectStore, state_id: &State, event_id: &Event, ) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::{:?}::", state_id, event_id, ).as_bytes());
        let new = Self {
            id,
            state_id: state_id.id,
            event_id: event_id.id,
        };




        
        store.inter_acknowledged_event(new.clone());




        
        new
    }
    // {"magic":"","kind":"CriticalBlockEnd"}
}
/// The *subtype* in a *supertype-subtype* relationship.
///
/// _Generated code_
// {"magic":"","kind":"CriticalBlockBegin"}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Subtype {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub obj_id: `Object`,
    ///
    pub obj_id: Uuid,
}
// {"magic":"","kind":"CriticalBlockEnd"}

impl Subtype {
    /// Inter a new Subtype and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Subtype;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let false_books = "average_robin".to_owned();
    /// let object_shm = Object::new(&mut store, false_books);
    ///
    /// let subtype = Subtype::new(&mut store, &object_shm);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    // {"magic":"","kind":"CriticalBlockBegin"}
    #[rustfmt::skip]
    pub fn new(store: &mut ObjectStore, obj_id: &Object, ) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::", obj_id, ).as_bytes());
        let new = Self {
            id,
            obj_id: obj_id.id,
        };




        
        store.inter_subtype(new.clone());




        
        new
    }
    // {"magic":"","kind":"CriticalBlockEnd"}
}
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

impl Conditionality {
    // {"magic":"","kind":"CriticalBlockBegin"}
    pub fn test_default(store: &mut ObjectStore) -> Self {
        let test = Self::Unconditional(UNCONDITIONAL);

        
        store.inter_conditionality(test.clone());

        
        test
    }
    // {"magic":"","kind":"CriticalBlockEnd"}
}

/// A `Relationship` indicates that a set of objects are connected to each other in some manner
///. Typically it is a _real world_ relationship. In the
///. Typically it is a _real world_ relationship. In the
///. Typically it is a _real world_ relationship. In the
///. Typically it is a _real world_ relationship. In the
///. Typically it is a _real world_ relationship. In the 
/// case of this model it is strictly an abstraction.
///
///
///
///
/// 
/// There are three types of `Relationship`: [`Isa`], [`Binary`], and [`Associative`]. Thus
/// `Relationship` is itself the *supertype* in an [`Isa`] relationship. It is a partitioning
/// *supertype-subtype* relationship, rather one of inheritance. As such, it’s  perfectly
/// suited to a rust `enum`! 😃
///
/// _Generated code_
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

impl Relationship {
    // {"magic":"","kind":"CriticalBlockBegin"}
    pub fn test_default(store: &mut ObjectStore) -> Self {
        let test = Self::Isa(Isa::test_default(store).get_id());

        
        store.inter_relationship(test.clone());

        
        test
    }
    // {"magic":"","kind":"CriticalBlockEnd"}
}

/// A `Binary` relationship, as it’s name implies, is a relationship between
/// two objects. It consists of two parts, the `Dependent` end of the
/// two objects. It consists of two parts, the `Dependent` end of the
/// two objects. It consists of two parts, the `Dependent` end of the
/// two objects. It consists of two parts, the `Dependent` end of the
/// two objects. It consists of two parts, the `Dependent` end of the 
/// relationship and the `Independent` end.
///
///
///
///
/// 
/// The former is so named because it has the job of formalizing the
/// relationship. It stores a pointer to the independent object as an attribute.
///
/// The latter is aware of the relationship, but it does not store any
///
/// The latter is aware of the relationship, but it does not store any
///
/// The latter is aware of the relationship, but it does not store any
///
/// The latter is aware of the relationship, but it does not store any
/// 
/// The latter is aware of the relationship, but it does not store any 
/// information about the relationship. That said, there are means of
/// traversing the relationship from the `Independent` object.
/// traversing the relationship from the `Independent` object.
/// traversing the relationship from the `Independent` object.
/// traversing the relationship from the `Independent` object.
/// traversing the relationship from the `Independent` object. 
///
/// _Generated code_
// {"magic":"","kind":"CriticalBlockBegin"}
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
// {"magic":"","kind":"CriticalBlockEnd"}

impl Binary {
    /// Inter a new Binary and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Binary;
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Referent;
    /// # use sarzak::sarzak::Cardinality;
    /// # use sarzak::sarzak::Referrer;
    /// # use sarzak::sarzak::Conditionality;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let conditionality_obk = Conditionality::test_default(&mut store);
    /// let abrupt_growth = "aloof_dad".to_owned();
    /// let object_whg = Object::new(&mut store, abrupt_growth);
    /// let cardinality_gst = Cardinality::test_default(&mut store);
    /// let referent_zxw = Referent::new(&mut store, &conditionality_obk, &object_whg, &cardinality_gst);
    /// let conditionality_tte = Conditionality::test_default(&mut store);
    /// let fresh_class = "parched_force".to_owned();
    /// let object_hvi = Object::new(&mut store, fresh_class);
    /// let cardinality_wtv = Cardinality::test_default(&mut store);
    /// let cheap_gate = "nonstop_back".to_owned();
    /// let referrer_vhx = Referrer::new(&mut store, &conditionality_tte, &object_hvi, &cardinality_wtv, cheap_gate);
    ///
    /// let binary = Binary::new(&mut store, &referent_zxw, &referrer_vhx, 42);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    // {"magic":"","kind":"CriticalBlockBegin"}
    #[rustfmt::skip]
    pub fn new(store: &mut ObjectStore, to: &Referent, from: &Referrer, number: i64, ) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::{:?}::{}::", to, from, number, ).as_bytes());
        let new = Self {
            id,
            to: to.id,
            from: from.id,
            number,
        };




        
        store.inter_binary(new.clone());




        
        new
    }
    // {"magic":"","kind":"CriticalBlockEnd"}
}
/// An `Attribute` represents a single value. Each value must have a
/// An `Attribute` represents a single value. Each value must have a
/// An `Attribute` represents a single value. Each value must have a
/// An `Attribute` represents a single value. Each value must have a
/// An `Attribute` represents a single value. Each value must have a 
/// [`Type`], which constrains the values of data that may be assigned to
/// an `Attribute`.
///
/// _Generated code_
// {"magic":"","kind":"CriticalBlockBegin"}
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
// {"magic":"","kind":"CriticalBlockEnd"}

impl Attribute {
    /// Inter a new Attribute and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Type;
    /// # use sarzak::sarzak::Attribute;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let first_seed = "lewd_wind".to_owned();
    /// let object = Object::new(&mut store, first_seed);
    /// let type_dsq = Type::test_default(&mut store);
    /// let literate_sheet = "warm_business".to_owned();
    ///
    /// let attribute = Attribute::new(&mut store, Some(&object), &type_dsq, literate_sheet);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    // {"magic":"","kind":"CriticalBlockBegin"}
    #[rustfmt::skip]
    pub fn new(store: &mut ObjectStore, obj_id: Option<&Object>, ty: &Type, name: std::string::String, ) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::{:?}::{}::", obj_id, ty, name, ).as_bytes());
        let new = Self {
            id,
            obj_id: obj_id.map(|o| o.id),
            ty: ty.get_id(),
            name,
        };




        
        store.inter_attribute(new.clone());




        
        new
    }
    // {"magic":"","kind":"CriticalBlockEnd"}
}
/// An [Object] state, more precisely, a set of states, is where all the action happens.
///
/// _Generated code_
// {"magic":"","kind":"CriticalBlockBegin"}
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
// {"magic":"","kind":"CriticalBlockEnd"}

impl State {
    /// Inter a new State and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::State;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let healthy_loss = "gaudy_zebra".to_owned();
    /// let object_hqs = Object::new(&mut store, healthy_loss);
    /// let special_doll = "garrulous_jail".to_owned();
    ///
    /// let state = State::new(&mut store, &object_hqs, special_doll);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    // {"magic":"","kind":"CriticalBlockBegin"}
    #[rustfmt::skip]
    pub fn new(store: &mut ObjectStore, obj_id: &Object, name: std::string::String, ) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::{}::", obj_id, name, ).as_bytes());
        let new = Self {
            id,
            obj_id: obj_id.id,
            name,
        };




        
        store.inter_state(new.clone());




        
        new
    }
    // {"magic":"","kind":"CriticalBlockEnd"}
}
/// A constant value that indicates a cardinality of _one_.
///
///
///
///
/// 
/// ❗️{"singleton_object": true}
///
/// _Generated code_
//
pub const ONE: Uuid = uuid!["bf6924bb-089d-5c1f-bc1f-123ba1fd1ea3"];

/// An `Object` is a collection of related data. By creating `Object`s, and
/// An `Object` is a collection of related data. By creating `Object`s, and
/// An `Object` is a collection of related data. By creating `Object`s, and
/// An `Object` is a collection of related data. By creating `Object`s, and
/// An `Object` is a collection of related data. By creating `Object`s, and 
/// connecting them with `Relationships` we build a powerful abstraction.
///
/// `Object`s contain [Attribute]s that represent the data that the
/// `Object`encapsulates. All `Object`s have an attribute called `id`, which
///
/// `Object`s contain [Attribute]s that represent the data that the
/// `Object`encapsulates. All `Object`s have an attribute called `id`, which
///
/// `Object`s contain [Attribute]s that represent the data that the
/// `Object`encapsulates. All `Object`s have an attribute called `id`, which
///
/// `Object`s contain [Attribute]s that represent the data that the
/// `Object`encapsulates. All `Object`s have an attribute called `id`, which
/// 
/// `Object`s contain [Attribute]s that represent the data that the 
/// `Object`encapsulates. All `Object`s have an attribute called `id`, which 
/// is a unique idenifier for each class of `Object`. The `id` attribute is a
/// version 5 UUID.
///
/// _Generated code_
// {"magic":"","kind":"CriticalBlockBegin"}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Object {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub name: `std::string::String`,
    ///
    pub name: std::string::String,
}
// {"magic":"","kind":"CriticalBlockEnd"}

impl Object {
    /// Inter a new Object and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let true_channel = "sulky_connection".to_owned();
    ///
    /// let object = Object::new(&mut store, true_channel);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    // {"magic":"","kind":"CriticalBlockBegin"}
    #[rustfmt::skip]
    pub fn new(store: &mut ObjectStore, name: std::string::String, ) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{}::", name, ).as_bytes());
        let new = Self {
            id,
            name,
        };




        
        store.inter_object(new.clone());




        
        new
    }
    // {"magic":"","kind":"CriticalBlockEnd"}
}
/// This is the side being referred to in a binary relationship. It is the “to” side.
///
/// _Generated code_
// {"magic":"","kind":"CriticalBlockBegin"}
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
// {"magic":"","kind":"CriticalBlockEnd"}

impl Referent {
    /// Inter a new Referent and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Cardinality;
    /// # use sarzak::sarzak::Referent;
    /// # use sarzak::sarzak::Conditionality;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let conditionality_mls = Conditionality::test_default(&mut store);
    /// let vast_mask = "delicate_cloud".to_owned();
    /// let object_bbs = Object::new(&mut store, vast_mask);
    /// let cardinality_als = Cardinality::test_default(&mut store);
    ///
    /// let referent = Referent::new(&mut store, &conditionality_mls, &object_bbs, &cardinality_als);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    // {"magic":"","kind":"CriticalBlockBegin"}
    #[rustfmt::skip]
    pub fn new(store: &mut ObjectStore, conditionality: &Conditionality, obj_id: &Object, cardinality: &Cardinality, ) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::{:?}::{:?}::", conditionality, obj_id, cardinality, ).as_bytes());
        let new = Self {
            id,
            conditionality: conditionality.get_id(),
            obj_id: obj_id.id,
            cardinality: cardinality.get_id(),
        };




        
        store.inter_referent(new.clone());




        
        new
    }
    // {"magic":"","kind":"CriticalBlockEnd"}
}
/// The String Type
///
///
///
///
/// 
/// This type holds unicode characters. This type is just a placeholder. It's implementation
/// is determined downstream by the code generator.
///
///
///
///
/// 
/// ❗️{"singleton_object": true}
///
/// _Generated code_
//
pub const STRING: Uuid = uuid!["d2f03ddf-cb09-546e-9a7a-c9d4e871efb0"];

/// This is the side of a binary relationship that is doing the pointing, thus it contains the
/// referential attribute. It is connected to the “from” side of a binary relationship.
///
/// _Generated code_
// {"magic":"","kind":"CriticalBlockBegin"}
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
// {"magic":"","kind":"CriticalBlockEnd"}

impl Referrer {
    /// Inter a new Referrer and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Conditionality;
    /// # use sarzak::sarzak::Referrer;
    /// # use sarzak::sarzak::Cardinality;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let conditionality_vjc = Conditionality::test_default(&mut store);
    /// let noxious_wind = "gabby_ship".to_owned();
    /// let object_nzc = Object::new(&mut store, noxious_wind);
    /// let cardinality_fus = Cardinality::test_default(&mut store);
    /// let omniscient_change = "mammoth_fight".to_owned();
    ///
    /// let referrer = Referrer::new(&mut store, &conditionality_vjc, &object_nzc, &cardinality_fus, omniscient_change);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    // {"magic":"","kind":"CriticalBlockBegin"}
    #[rustfmt::skip]
    pub fn new(store: &mut ObjectStore, conditionality: &Conditionality, obj_id: &Object, cardinality: &Cardinality, referential_attribute: std::string::String, ) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::{:?}::{:?}::{}::", conditionality, obj_id, cardinality, referential_attribute, ).as_bytes());
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
    // {"magic":"","kind":"CriticalBlockEnd"}
}
/// The UUID Type
///
/// I feel like there are too many implementation details here.
///
///
/// I feel like there are too many implementation details here.
///
///
/// I feel like there are too many implementation details here.
///
///
/// I feel like there are too many implementation details here.
///
/// 
/// I feel like there are too many implementation details here. 
/// 
/// This UUID is expected to be version 5. Generally we produce input
/// to the hash function from other UUIDs, coupled with additional
/// to the hash function from other UUIDs, coupled with additional
/// to the hash function from other UUIDs, coupled with additional
/// to the hash function from other UUIDs, coupled with additional
/// to the hash function from other UUIDs, coupled with additional 
/// information from the creator to ensure a unique UUID.
///
///
///
///
/// 
/// The `ns` attribute is the namespace used to generate generate UUIDs
/// given a particular instance of `UUID`.
///
///
///
///
/// 
/// ❗️{"singleton_object": true, "translation_name": "SarzakUuid"}
///
/// _Generated code_
//
pub const UUID: Uuid = uuid!["dc1639ca-7e20-5a39-92e5-9a478471b8e5"];

/// A constant value that indicates a cardinality of _many_.
///
///
///
///
/// 
/// ❗️{"singleton_object": true}
///
/// _Generated code_
//
pub const MANY: Uuid = uuid!["0614a507-4422-5994-a59d-68dc57d2c328"];

/// This object represents the *supertype* in a *supertype-subtype*
/// This object represents the *supertype* in a *supertype-subtype*
/// This object represents the *supertype* in a *supertype-subtype*
/// This object represents the *supertype* in a *supertype-subtype*
/// This object represents the *supertype* in a *supertype-subtype* 
/// relationship.
///
/// _Generated code_
// {"magic":"","kind":"CriticalBlockBegin"}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Supertype {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub obj_id: `Object`,
    ///
    pub obj_id: Uuid,
}
// {"magic":"","kind":"CriticalBlockEnd"}

impl Supertype {
    /// Inter a new Supertype and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Supertype;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let naive_hose = "jazzy_rock".to_owned();
    /// let object_lhn = Object::new(&mut store, naive_hose);
    ///
    /// let supertype = Supertype::new(&mut store, &object_lhn);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    // {"magic":"","kind":"CriticalBlockBegin"}
    #[rustfmt::skip]
    pub fn new(store: &mut ObjectStore, obj_id: &Object, ) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::", obj_id, ).as_bytes());
        let new = Self {
            id,
            obj_id: obj_id.id,
        };




        
        store.inter_supertype(new.clone());




        
        new
    }
    // {"magic":"","kind":"CriticalBlockEnd"}
}
/// A constant value that indicates a conditionality of _unconditional_.
///
///
///
///
/// 
/// ❗️{"singleton_object": true}
///
/// _Generated code_
//
pub const UNCONDITIONAL: Uuid = uuid!["0148e8ea-cf04-50f3-920c-b1aed9903e3a"];

// {"magic":"","kind":"CriticalBlockBegin"}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OneSide {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub obj_id: `Object`,
    ///
    pub obj_id: Uuid,
}
// {"magic":"","kind":"CriticalBlockEnd"}

impl OneSide {
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
    /// let abrasive_rose = "literate_foot".to_owned();
    /// let object_dkv = Object::new(&mut store, abrasive_rose);
    ///
    /// let one_side = OneSide::new(&mut store, &object_dkv);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    // {"magic":"","kind":"CriticalBlockBegin"}
    #[rustfmt::skip]
    pub fn new(store: &mut ObjectStore, obj_id: &Object, ) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::", obj_id, ).as_bytes());
        let new = Self {
            id,
            obj_id: obj_id.id,
        };




        
        store.inter_one_side(new.clone());




        
        new
    }
    // {"magic":"","kind":"CriticalBlockEnd"}
}
/// The type of a value
///
///
///
///
/// 
/// There are several values available: [Integer], [Boolean], [Float], [String], and [UUID]
///.
///
/// _Generated code_
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

impl Type {
    // {"magic":"","kind":"CriticalBlockBegin"}
    pub fn test_default(store: &mut ObjectStore) -> Self {
        let test = Self::Boolean(BOOLEAN);

        
        store.inter_ty(test.clone());

        
        test
    }
    // {"magic":"","kind":"CriticalBlockEnd"}
}

/// A constant value that indicates a conditionality of _conditional_.
///
///
///
///
/// 
/// ❗️{"singleton_object": true}
///
/// _Generated code_
//
pub const CONDITIONAL: Uuid = uuid!["fc6aa4ae-4ab5-5b43-a7c1-52bbd3e69f34"];

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

impl Cardinality {
    // {"magic":"","kind":"CriticalBlockBegin"}
    pub fn test_default(store: &mut ObjectStore) -> Self {
        let test = Self::One(ONE);

        
        store.inter_cardinality(test.clone());

        
        test
    }
    // {"magic":"","kind":"CriticalBlockEnd"}
}

/// An event is sent to an object, and processed by the current state. Assuming it accepts the
/// event. Otherwise it’s dropped on the floor.
///
/// _Generated code_
// {"magic":"","kind":"CriticalBlockBegin"}
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
// {"magic":"","kind":"CriticalBlockEnd"}

impl Event {
    /// Inter a new Event and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Event;
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let abusive_appliance = "damaged_expert".to_owned();
    /// let object_sra = Object::new(&mut store, abusive_appliance);
    /// let aspiring_sack = "nice_plastic".to_owned();
    ///
    /// let event = Event::new(&mut store, &object_sra, aspiring_sack);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    // {"magic":"","kind":"CriticalBlockBegin"}
    #[rustfmt::skip]
    pub fn new(store: &mut ObjectStore, obj_id: &Object, name: std::string::String, ) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::{}::", obj_id, name, ).as_bytes());
        let new = Self {
            id,
            obj_id: obj_id.id,
            name,
        };




        
        store.inter_event(new.clone());




        
        new
    }
    // {"magic":"","kind":"CriticalBlockEnd"}
}
// {"magic":"","kind":"CriticalBlockBegin"}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OtherSide {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub obj_id: `Object`,
    ///
    pub obj_id: Uuid,
}
// {"magic":"","kind":"CriticalBlockEnd"}

impl OtherSide {
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
    /// let future_advertisement = "merciful_squirrel".to_owned();
    /// let object_vpv = Object::new(&mut store, future_advertisement);
    ///
    /// let other_side = OtherSide::new(&mut store, &object_vpv);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    // {"magic":"","kind":"CriticalBlockBegin"}
    #[rustfmt::skip]
    pub fn new(store: &mut ObjectStore, obj_id: &Object, ) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::", obj_id, ).as_bytes());
        let new = Self {
            id,
            obj_id: obj_id.id,
        };




        
        store.inter_other_side(new.clone());




        
        new
    }
    // {"magic":"","kind":"CriticalBlockEnd"}
}
/// The Floating Point Type
///
///
///
///
/// 
/// This type holds numbers from ℝ. This type is just a placeholder. It's implementation is
/// determined downstream by the code generator.
///
///
///
///
/// 
/// ❗️{"singleton_object": true}
///
/// _Generated code_
//
pub const FLOAT: Uuid = uuid!["8ca8decc-f87b-587a-a390-593d20203b6f"];

// {"magic":"","kind":"CriticalBlockBegin"}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AssociativeSide {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub obj_id: `Object`,
    ///
    pub obj_id: Uuid,
}
// {"magic":"","kind":"CriticalBlockEnd"}

impl AssociativeSide {
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
    /// let best_veil = "pretty_caption".to_owned();
    /// let object_hqe = Object::new(&mut store, best_veil);
    ///
    /// let associative_side = AssociativeSide::new(&mut store, &object_hqe);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    // {"magic":"","kind":"CriticalBlockBegin"}
    #[rustfmt::skip]
    pub fn new(store: &mut ObjectStore, obj_id: &Object, ) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::", obj_id, ).as_bytes());
        let new = Self {
            id,
            obj_id: obj_id.id,
        };




        
        store.inter_associative_side(new.clone());




        
        new
    }
    // {"magic":"","kind":"CriticalBlockEnd"}
}
/// The Integer Type
///
///
///
///
/// 
/// This is an interger that can hold positive and negative values. This type is just a placeholder
///. It's implementation is determined downstream by the code generator.
///
///
///
///
/// 
/// ❗️{"singleton_object": true}
///
/// _Generated code_
//
pub const INTEGER: Uuid = uuid!["70ec7fbd-44a2-5800-8558-349e3b22cf17"];

