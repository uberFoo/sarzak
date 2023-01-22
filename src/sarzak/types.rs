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

// {"magic":"","kind":"CriticalBlockBegin"}
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"imports"}}}
use crate::sarzak::store::ObjectStore;
use crate::sarzak::UUID_NS;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"imports"}}}

/// The Boolean Type
///
/// This type holds `true` and `false` values. This type is just a placeholder. It's implementation
/// is determined downstream by the code generator.
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"isa-test_default"}}}
impl Isa {
    pub fn test_default(store: &mut ObjectStore) -> Self {
        // {"magic":"","kind":"IgnoreBlockBegin"}
        let salty_gold = "warm_airplane".to_owned();
        let object_hpb = Object::new(store, salty_gold);
        let test = Self::Subtype(Subtype::new(store, &object_hpb).id);
        // {"magic":"","kind":"IgnoreBlockEnd"}

        store.inter_isa(test.clone());

        test
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"isa-test_default"}}}

// {"magic":"","kind":"CriticalBlockBegin"}
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
    /// # use sarzak::sarzak::Associative;
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::AssociativeSide;
    /// # use sarzak::sarzak::OtherSide;
    /// # use sarzak::sarzak::OneSide;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let therapeutic_cemetery = "precious_house".to_owned();
    /// let object_qbd = Object::new(&mut store, therapeutic_cemetery);
    /// let one_side_sve = OneSide::new(&mut store, &object_qbd);
    /// let icky_silk = "arrogant_value".to_owned();
    /// let object_pqt = Object::new(&mut store, icky_silk);
    /// let other_side_fxk = OtherSide::new(&mut store, &object_pqt);
    /// let paltry_flag = "devilish_join".to_owned();
    /// let object_ylf = Object::new(&mut store, paltry_flag);
    /// let associative_side_zdb = AssociativeSide::new(&mut store, &object_ylf);
    ///
    /// let associative = Associative::new(&mut store, &one_side_sve, &other_side_fxk, &associative_side_zdb, 42);
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

/// An Event that Does Something
///
/// An acknowledged event is an event that a [`State`] knows how to handle.
///
/// _Generated code_
// {"magic":"","kind":"CriticalBlockBegin"}
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
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::AcknowledgedEvent;
    /// # use sarzak::sarzak::State;
    /// # use sarzak::sarzak::Event;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let tangy_pencil = "lush_pizzas".to_owned();
    /// let object_iqa = Object::new(&mut store, tangy_pencil);
    /// let squeamish_bridge = "natural_muscle".to_owned();
    /// let state_qht = State::new(&mut store, &object_iqa, squeamish_bridge);
    /// let tense_prose = "peaceful_smash".to_owned();
    /// let object_lea = Object::new(&mut store, tense_prose);
    /// let grateful_trees = "rambunctious_fruit".to_owned();
    /// let event_pnp = Event::new(&mut store, &object_lea, grateful_trees);
    ///
    /// let acknowledged_event = AcknowledgedEvent::new(&mut store, &state_qht, &event_pnp);
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

/// The *subtype* in a *supertype-subtype* relationship.
///
/// _Generated code_
// {"magic":"","kind":"CriticalBlockBegin"}
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
    /// let educated_milk = "shrill_story".to_owned();
    /// let object_chu = Object::new(&mut store, educated_milk);
    ///
    /// let subtype = Subtype::new(&mut store, &object_chu);
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

/// A `Relationship` indicates that a set of objects are connected to each other in some manner
///. Typically it is a _real world_ relationship. In the
/// case of this model it is strictly an abstraction.
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"relationship-test_default"}}}
impl Relationship {
    pub fn test_default(store: &mut ObjectStore) -> Self {
        let test = Self::Isa(Isa::test_default(store).get_id());

        store.inter_relationship(test.clone());

        test
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"relationship-test_default"}}}

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
/// _Generated code_
// {"magic":"","kind":"CriticalBlockBegin"}
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
    /// # use sarzak::sarzak::Binary;
    /// # use sarzak::sarzak::Referent;
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Referrer;
    /// # use sarzak::sarzak::Cardinality;
    /// # use sarzak::sarzak::Conditionality;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let conditionality_iil = Conditionality::test_default(&mut store);
    /// let phobic_statement = "colorful_respect".to_owned();
    /// let object_kau = Object::new(&mut store, phobic_statement);
    /// let cardinality_xlh = Cardinality::test_default(&mut store);
    /// let referent_byn = Referent::new(&mut store, &conditionality_iil, &object_kau, &cardinality_xlh);
    /// let conditionality_vqa = Conditionality::test_default(&mut store);
    /// let vacuous_bite = "testy_slope".to_owned();
    /// let object_hvh = Object::new(&mut store, vacuous_bite);
    /// let cardinality_ltb = Cardinality::test_default(&mut store);
    /// let dead_bat = "married_army".to_owned();
    /// let referrer_xgl = Referrer::new(&mut store, &conditionality_vqa, &object_hvh, &cardinality_ltb, dead_bat);
    ///
    /// let binary = Binary::new(&mut store, &referent_byn, &referrer_xgl, 42);
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

/// An `Attribute` represents a single value. Each value must have a
/// [`Type`], which constrains the values of data that may be assigned to
/// an `Attribute`.
///
/// _Generated code_
// {"magic":"","kind":"CriticalBlockBegin"}
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
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Type;
    /// # use sarzak::sarzak::Attribute;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let perfect_sky = "lying_gold".to_owned();
    /// let object = Object::new(&mut store, perfect_sky);
    /// let type_crf = Type::test_default(&mut store);
    /// let deserted_uncle = "colorful_bulb".to_owned();
    ///
    /// let attribute = Attribute::new(&mut store, Some(&object), &type_crf, deserted_uncle);
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

/// An [Object] state, more precisely, a set of states, is where all the action happens.
///
/// _Generated code_
// {"magic":"","kind":"CriticalBlockBegin"}
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
    /// let melodic_apples = "colossal_goldfish".to_owned();
    /// let object_luv = Object::new(&mut store, melodic_apples);
    /// let cheerful_grandfather = "obsequious_motion".to_owned();
    ///
    /// let state = State::new(&mut store, &object_luv, cheerful_grandfather);
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

/// A constant value that indicates a cardinality of _one_.
///
/// ❗️{"singleton_object": true}
///
/// _Generated code_
//
pub const ONE: Uuid = uuid!["bf6924bb-089d-5c1f-bc1f-123ba1fd1ea3"];

/// An `Object` is a collection of related data. By creating `Object`s, and
/// connecting them with `Relationships` we build a powerful abstraction.
///
/// `Object`s contain [Attribute]s that represent the data that the
/// `Object`encapsulates. All `Object`s have an attribute called `id`, which
/// is a unique idenifier for each class of `Object`. The `id` attribute is a
/// version 5 UUID.
///
/// _Generated code_
// {"magic":"","kind":"CriticalBlockBegin"}
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
    /// let lame_lunchroom = "tricky_veil".to_owned();
    ///
    /// let object = Object::new(&mut store, lame_lunchroom);
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

/// This is the side being referred to in a binary relationship. It is the “to” side.
///
/// _Generated code_
// {"magic":"","kind":"CriticalBlockBegin"}
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
    /// # use sarzak::sarzak::Conditionality;
    /// # use sarzak::sarzak::Cardinality;
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Referent;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let conditionality_wej = Conditionality::test_default(&mut store);
    /// let thinkable_guide = "breakable_trees".to_owned();
    /// let object_qkj = Object::new(&mut store, thinkable_guide);
    /// let cardinality_wwf = Cardinality::test_default(&mut store);
    ///
    /// let referent = Referent::new(&mut store, &conditionality_wej, &object_qkj, &cardinality_wwf);
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

/// The String Type
///
/// This type holds unicode characters. This type is just a placeholder. It's implementation
/// is determined downstream by the code generator.
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
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Cardinality;
    /// # use sarzak::sarzak::Conditionality;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let conditionality_lxm = Conditionality::test_default(&mut store);
    /// let cloudy_caption = "hanging_volleyball".to_owned();
    /// let object_vav = Object::new(&mut store, cloudy_caption);
    /// let cardinality_pnf = Cardinality::test_default(&mut store);
    /// let ablaze_daughter = "average_whip".to_owned();
    ///
    /// let referrer = Referrer::new(&mut store, &conditionality_lxm, &object_vav, &cardinality_pnf, ablaze_daughter);
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
/// _Generated code_
//
pub const UUID: Uuid = uuid!["dc1639ca-7e20-5a39-92e5-9a478471b8e5"];

/// A constant value that indicates a cardinality of _many_.
///
/// ❗️{"singleton_object": true}
///
/// _Generated code_
//
pub const MANY: Uuid = uuid!["0614a507-4422-5994-a59d-68dc57d2c328"];

/// This object represents the *supertype* in a *supertype-subtype*
/// relationship.
///
/// _Generated code_
// {"magic":"","kind":"CriticalBlockBegin"}
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
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Supertype;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let willing_crown = "dangerous_lift".to_owned();
    /// let object_fwb = Object::new(&mut store, willing_crown);
    ///
    /// let supertype = Supertype::new(&mut store, &object_fwb);
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

/// A constant value that indicates a conditionality of _unconditional_.
///
/// ❗️{"singleton_object": true}
///
/// _Generated code_
//
pub const UNCONDITIONAL: Uuid = uuid!["0148e8ea-cf04-50f3-920c-b1aed9903e3a"];

// {"magic":"","kind":"CriticalBlockBegin"}
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
    /// # use sarzak::sarzak::OneSide;
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let troubled_fifth = "faded_basketball".to_owned();
    /// let object_wjw = Object::new(&mut store, troubled_fifth);
    ///
    /// let one_side = OneSide::new(&mut store, &object_wjw);
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

/// The type of a value
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"ty-test_default"}}}
impl Type {
    pub fn test_default(store: &mut ObjectStore) -> Self {
        let test = Self::Boolean(BOOLEAN);

        store.inter_ty(test.clone());

        test
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"ty-test_default"}}}

/// A constant value that indicates a conditionality of _conditional_.
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"cardinality-test_default"}}}
impl Cardinality {
    pub fn test_default(store: &mut ObjectStore) -> Self {
        let test = Self::One(ONE);

        store.inter_cardinality(test.clone());

        test
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"cardinality-test_default"}}}

/// An event is sent to an object, and processed by the current state. Assuming it accepts the
/// event. Otherwise it’s dropped on the floor.
///
/// _Generated code_
// {"magic":"","kind":"CriticalBlockBegin"}
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
    /// # use sarzak::sarzak::Event;
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let warlike_harmony = "aggressive_ladybug".to_owned();
    /// let object_gps = Object::new(&mut store, warlike_harmony);
    /// let nutritious_lawyer = "threatening_push".to_owned();
    ///
    /// let event = Event::new(&mut store, &object_gps, nutritious_lawyer);
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
    /// # use sarzak::sarzak::OtherSide;
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let political_year = "giddy_pets".to_owned();
    /// let object_qzp = Object::new(&mut store, political_year);
    ///
    /// let other_side = OtherSide::new(&mut store, &object_qzp);
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

/// The Floating Point Type
///
/// This type holds numbers from ℝ. This type is just a placeholder. It's implementation is
/// determined downstream by the code generator.
///
/// ❗️{"singleton_object": true}
///
/// _Generated code_
//
pub const FLOAT: Uuid = uuid!["8ca8decc-f87b-587a-a390-593d20203b6f"];

// {"magic":"","kind":"CriticalBlockBegin"}
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
    /// let electric_paint = "heavenly_receipt".to_owned();
    /// let object_fzr = Object::new(&mut store, electric_paint);
    ///
    /// let associative_side = AssociativeSide::new(&mut store, &object_fzr);
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

/// The Integer Type
///
/// This is an interger that can hold positive and negative values. This type is just a placeholder
///. It's implementation is determined downstream by the code generator.
///
/// ❗️{"singleton_object": true}
///
/// _Generated code_
//
pub const INTEGER: Uuid = uuid!["70ec7fbd-44a2-5800-8558-349e3b22cf17"];
