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
//!    * [`AssociativeReferent`]
//!    * [`AssociativeReferrer`]
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
//!    * [`REFERENCE`]
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
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
//! ```shell
//!  sarzak gen -d sarzak
//! ```
// {"magic":"","kind":"IgnoreBlockEnd"}
// {"magic":"","version":"1.0.0"}
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"acknowledged_event-new_impl"}}}
impl AcknowledgedEvent {
    /// Inter a new AcknowledgedEvent and return it's `id`
    ///
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Event;
    /// # use sarzak::sarzak::State;
    /// # use sarzak::sarzak::AcknowledgedEvent;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let wistful_class = "rigid_ocean".to_owned();
    /// let well_off_basin = "five_feeling".to_owned();
    /// let living_dirt = "gruesome_honey".to_owned();
    /// let object_nqv = Object::new(&mut store, wistful_class, well_off_basin, living_dirt);
    /// let minor_power = "glib_scale".to_owned();
    /// let state_fux = State::new(&mut store, &object_nqv, minor_power);
    /// let disastrous_pan = "royal_grip".to_owned();
    /// let right_hot = "quick_donkey".to_owned();
    /// let defective_door = "efficacious_burn".to_owned();
    /// let object_pyf = Object::new(&mut store, disastrous_pan, right_hot, defective_door);
    /// let cut_rub = "abounding_apples".to_owned();
    /// let event_sax = Event::new(&mut store, &object_pyf, cut_rub);
    ///
    /// let acknowledged_event = AcknowledgedEvent::new(&mut store, &state_fux, &event_sax);
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
    /// pub from: `Associative Referrer`,
    ///
    pub from: Uuid,
    /// pub one: `Associative Referent`,
    ///
    pub one: Uuid,
    /// pub other: `Associative Referent`,
    ///
    pub other: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"associative-struct-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"associative-new_impl"}}}
impl Associative {
    /// Inter a new Associative and return it's `id`
    ///
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::AssociativeReferrer;
    /// # use sarzak::sarzak::AssociativeReferent;
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Associative;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let oceanic_stem = "pumped_use".to_owned();
    /// let narrow_growth = "roomy_sea".to_owned();
    /// let rustic_respect = "painstaking_selection".to_owned();
    /// let object_xin = Object::new(&mut store, oceanic_stem, narrow_growth, rustic_respect);
    /// let associative_referent_olc = AssociativeReferent::new(&mut store, &object_xin);
    /// let quixotic_vest = "dark_knowledge".to_owned();
    /// let alive_cub = "youthful_angle".to_owned();
    /// let gullible_boys = "flippant_scarf".to_owned();
    /// let object_vqg = Object::new(&mut store, quixotic_vest, alive_cub, gullible_boys);
    /// let associative_referent_fib = AssociativeReferent::new(&mut store, &object_vqg);
    /// let insidious_dirt = "impossible_regret".to_owned();
    /// let ruddy_face = "rotten_band".to_owned();
    /// let infamous_park = "chivalrous_knowledge".to_owned();
    /// let object_uix = Object::new(&mut store, insidious_dirt, ruddy_face, infamous_park);
    /// let associative_referrer_tbw = AssociativeReferrer::new(&mut store, &object_uix);
    ///
    /// let associative = Associative::new(&mut store, &associative_referent_olc, &associative_referent_fib, &associative_referrer_tbw, 42);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(
        store: &mut ObjectStore,
        one: &AssociativeReferent,
        other: &AssociativeReferent,
        from: &AssociativeReferrer,
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

/// The other objects in an Associative Relationship
///
/// This represents one of the two objects that are related in an [`Associative`] relationhip
///.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"associative_referent-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AssociativeReferent {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub obj_id: `Object`,
    ///
    pub obj_id: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"associative_referent-struct-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"associative_referent-new_impl"}}}
impl AssociativeReferent {
    /// Inter a new AssociativeReferent and return it's `id`
    ///
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::AssociativeReferent;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let silent_fifth = "early_jail".to_owned();
    /// let incompetent_things = "malicious_basket".to_owned();
    /// let tiny_bridge = "jagged_spring".to_owned();
    /// let object_rpo = Object::new(&mut store, silent_fifth, incompetent_things, tiny_bridge);
    ///
    /// let associative_referent = AssociativeReferent::new(&mut store, &object_rpo);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, obj_id: &Object) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::", obj_id,).as_bytes());
        let new = Self {
            id,
            obj_id: obj_id.id,
        };

        store.inter_associative_referent(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"associative_referent-new_impl"}}}
}

/// Associative Object
///
/// This is used in an [`Associative`] relationship to point to the Associative object itself
///. It's the box with the line pointing at another line.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"associative_referrer-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AssociativeReferrer {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub obj_id: `Object`,
    ///
    pub obj_id: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"associative_referrer-struct-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"associative_referrer-new_impl"}}}
impl AssociativeReferrer {
    /// Inter a new AssociativeReferrer and return it's `id`
    ///
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::AssociativeReferrer;
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let typical_border = "animated_back".to_owned();
    /// let mean_eye = "understood_relation".to_owned();
    /// let tense_girl = "obsequious_station".to_owned();
    /// let object_epf = Object::new(&mut store, typical_border, mean_eye, tense_girl);
    ///
    /// let associative_referrer = AssociativeReferrer::new(&mut store, &object_epf);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, obj_id: &Object) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::", obj_id,).as_bytes());
        let new = Self {
            id,
            obj_id: obj_id.id,
        };

        store.inter_associative_referrer(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"associative_referrer-new_impl"}}}
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"attribute-new_impl"}}}
impl Attribute {
    /// Inter a new Attribute and return it's `id`
    ///
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Type;
    /// # use sarzak::sarzak::Attribute;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let far_error = "far_flung_sand".to_owned();
    /// let nutty_rice = "ugly_temper".to_owned();
    /// let scattered_shoe = "tasteful_creature".to_owned();
    /// let object = Object::new(&mut store, far_error, nutty_rice, scattered_shoe);
    /// let type_lek = Type::test_default(&mut store);
    /// let succinct_property = "unique_poison".to_owned();
    ///
    /// let attribute = Attribute::new(&mut store, Some(&object), &type_lek, succinct_property);
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"binary-new_impl"}}}
impl Binary {
    /// Inter a new Binary and return it's `id`
    ///
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Referrer;
    /// # use sarzak::sarzak::Referent;
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Binary;
    /// # use sarzak::sarzak::Cardinality;
    /// # use sarzak::sarzak::Conditionality;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let conditionality_vgn = Conditionality::test_default(&mut store);
    /// let chunky_sleet = "thoughtless_agreement".to_owned();
    /// let chivalrous_wing = "caring_clouds".to_owned();
    /// let spiffy_taste = "lethal_muscle".to_owned();
    /// let object_vuf = Object::new(&mut store, chunky_sleet, chivalrous_wing, spiffy_taste);
    /// let cardinality_per = Cardinality::test_default(&mut store);
    /// let certain_eye = "overrated_sheet".to_owned();
    /// let referent_hck = Referent::new(&mut store, &conditionality_vgn, &object_vuf, &cardinality_per, certain_eye);
    /// let conditionality_jci = Conditionality::test_default(&mut store);
    /// let immense_walk = "well_off_transport".to_owned();
    /// let brave_beast = "nine_silk".to_owned();
    /// let enthusiastic_button = "elastic_cloth".to_owned();
    /// let object_xjs = Object::new(&mut store, immense_walk, brave_beast, enthusiastic_button);
    /// let cardinality_gfv = Cardinality::test_default(&mut store);
    /// let curved_print = "overwrought_bead".to_owned();
    /// let rambunctious_rice = "shaggy_holiday".to_owned();
    /// let referrer_zdn = Referrer::new(&mut store, &conditionality_jci, &object_xjs, &cardinality_gfv, curved_print, rambunctious_rice);
    ///
    /// let binary = Binary::new(&mut store, &referent_hck, &referrer_zdn, 42);
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"cardinality-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Cardinality {
    /// `One(One)`,
    ///
    One(Uuid),
    /// `Many(Many)`,
    ///
    Many(Uuid),
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"{}-enum-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"Cardinality-enum-get-id-impl"}}}
impl Cardinality {
    pub fn get_id(&self) -> Uuid {
        match *self {
            Self::One(z) => z,
            Self::Many(z) => z,
        }
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"Cardinality-enum-get-id-impl"}}}

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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"conditionality-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Conditionality {
    /// `Unconditional(Unconditional)`,
    ///
    Unconditional(Uuid),
    /// `Conditional(Conditional)`,
    ///
    Conditional(Uuid),
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"{}-enum-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"Conditionality-enum-get-id-impl"}}}
impl Conditionality {
    pub fn get_id(&self) -> Uuid {
        match *self {
            Self::Unconditional(z) => z,
            Self::Conditional(z) => z,
        }
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"Conditionality-enum-get-id-impl"}}}

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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"event-new_impl"}}}
impl Event {
    /// Inter a new Event and return it's `id`
    ///
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Event;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let glorious_crow = "yellow_beast".to_owned();
    /// let disgusted_crayon = "unsuitable_vessel".to_owned();
    /// let tremendous_writing = "coherent_twig".to_owned();
    /// let object_nih = Object::new(&mut store, glorious_crow, disgusted_crayon, tremendous_writing);
    /// let like_spark = "blushing_honey".to_owned();
    ///
    /// let event = Event::new(&mut store, &object_nih, like_spark);
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"isa-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Isa {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub number: `i64`,
    ///
    pub number: i64,
    /// pub supertype: `Supertype`,
    ///
    pub supertype: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"isa-struct-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"isa-new_impl"}}}
impl Isa {
    /// Inter a new Isa and return it's `id`
    ///
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Supertype;
    /// # use sarzak::sarzak::Isa;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let learned_twist = "adamant_frogs".to_owned();
    /// let fertile_silver = "astonishing_thing".to_owned();
    /// let defeated_locket = "different_minister".to_owned();
    /// let object_iei = Object::new(&mut store, learned_twist, fertile_silver, defeated_locket);
    /// let supertype_spq = Supertype::new(&mut store, &object_iei);
    ///
    /// let isa = Isa::new(&mut store, &supertype_spq, 42);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, supertype: &Supertype, number: i64) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{:?}::{}::", supertype, number,).as_bytes(),
        );
        let new = Self {
            id,
            supertype: supertype.id,
            number,
        };

        store.inter_isa(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"isa-new_impl"}}}
}

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
    /// pub description: `std::string::String`,
    ///
    pub description: std::string::String,
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub key letters: `std::string::String`,
    ///
    pub key_letters: std::string::String,
    /// pub name: `std::string::String`,
    ///
    pub name: std::string::String,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"object-struct-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"object-new_impl"}}}
impl Object {
    /// Inter a new Object and return it's `id`
    ///
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let disgusting_hour = "many_payment".to_owned();
    /// let plastic_sidewalk = "evasive_airport".to_owned();
    /// let ill_informed_teeth = "ritzy_apples".to_owned();
    ///
    /// let object = Object::new(&mut store, disgusting_hour, plastic_sidewalk, ill_informed_teeth);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(
        store: &mut ObjectStore,
        description: std::string::String,
        name: std::string::String,
        key_letters: std::string::String,
    ) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{}::{}::{}::", description, name, key_letters,).as_bytes(),
        );
        let new = Self {
            id,
            description,
            name,
            key_letters,
        };

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

/// A Reference Type
///
/// This is the type used in relationship formalization. I'm actually not completely buying
/// into this. I need a reference type, and this does meet that criterion. It's a reference
/// to [`Object`] though. The things formalizing relationships look like references to the things
/// on the other side of thet arrow.
///
/// Maybe if I take this to mean just a reference type, and don't tie it directly to [`Object
///`]? I'm going to go that route I think.
///
/// ❗️{"singleton_object": true}
///
//
pub const REFERENCE: Uuid = uuid!["47cb41d9-8f9c-5f3d-8892-85c1dea4b102"];

/// This is the side being referred to in a binary relationship. It is the “to” side.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"referent-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Referent {
    /// pub description: `std::string::String`,
    ///
    pub description: std::string::String,
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"referent-new_impl"}}}
impl Referent {
    /// Inter a new Referent and return it's `id`
    ///
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Conditionality;
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Referent;
    /// # use sarzak::sarzak::Cardinality;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let conditionality_fuu = Conditionality::test_default(&mut store);
    /// let billowy_doll = "measly_week".to_owned();
    /// let overt_question = "trite_man".to_owned();
    /// let fragile_week = "ancient_goat".to_owned();
    /// let object_muu = Object::new(&mut store, billowy_doll, overt_question, fragile_week);
    /// let cardinality_tmu = Cardinality::test_default(&mut store);
    /// let magenta_flower = "kindhearted_copper".to_owned();
    ///
    /// let referent = Referent::new(&mut store, &conditionality_fuu, &object_muu, &cardinality_tmu, magenta_flower);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(
        store: &mut ObjectStore,
        conditionality: &Conditionality,
        obj_id: &Object,
        cardinality: &Cardinality,
        description: std::string::String,
    ) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!(
                "{:?}::{:?}::{:?}::{}::",
                conditionality, obj_id, cardinality, description,
            )
            .as_bytes(),
        );
        let new = Self {
            id,
            conditionality: conditionality.get_id(),
            obj_id: obj_id.id,
            cardinality: cardinality.get_id(),
            description,
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
    /// pub description: `std::string::String`,
    ///
    pub description: std::string::String,
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"referrer-new_impl"}}}
impl Referrer {
    /// Inter a new Referrer and return it's `id`
    ///
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Conditionality;
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Referrer;
    /// # use sarzak::sarzak::Cardinality;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let conditionality_pkj = Conditionality::test_default(&mut store);
    /// let dry_quartz = "nappy_light".to_owned();
    /// let unused_person = "disillusioned_lock".to_owned();
    /// let homeless_cover = "humdrum_harmony".to_owned();
    /// let object_dat = Object::new(&mut store, dry_quartz, unused_person, homeless_cover);
    /// let cardinality_tri = Cardinality::test_default(&mut store);
    /// let sassy_discussion = "endurable_smash".to_owned();
    /// let orange_hammer = "seemly_wash".to_owned();
    ///
    /// let referrer = Referrer::new(&mut store, &conditionality_pkj, &object_dat, &cardinality_tri, sassy_discussion, orange_hammer);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(
        store: &mut ObjectStore,
        conditionality: &Conditionality,
        obj_id: &Object,
        cardinality: &Cardinality,
        referential_attribute: std::string::String,
        description: std::string::String,
    ) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!(
                "{:?}::{:?}::{:?}::{}::{}::",
                conditionality, obj_id, cardinality, referential_attribute, description,
            )
            .as_bytes(),
        );
        let new = Self {
            id,
            conditionality: conditionality.get_id(),
            obj_id: obj_id.id,
            cardinality: cardinality.get_id(),
            referential_attribute,
            description,
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
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"relationship-enum-definition"}}}
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
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"{}-enum-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"Relationship-enum-get-id-impl"}}}
impl Relationship {
    pub fn get_id(&self) -> Uuid {
        match *self {
            Self::Isa(z) => z,
            Self::Associative(z) => z,
            Self::Binary(z) => z,
        }
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"Relationship-enum-get-id-impl"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"relationship-test_default"}}}
impl Relationship {
    pub fn test_default(store: &mut ObjectStore) -> Self {
        // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
        let omniscient_stew = "subsequent_girls".to_owned();
        let aspiring_governor = "yielding_orange".to_owned();
        let smelly_wax = "overwrought_position".to_owned();
        let object_wmc = Object::new(store, omniscient_stew, aspiring_governor, smelly_wax);
        let supertype_uvh = Supertype::new(store, &object_wmc);
        let test = Self::Isa(Isa::new(store, &supertype_uvh, 42).id);
        // {"magic":"","kind":"IgnoreBlockEnd"}

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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"state-new_impl"}}}
impl State {
    /// Inter a new State and return it's `id`
    ///
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::State;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let jobless_hobbies = "puny_limit".to_owned();
    /// let unkempt_humor = "remarkable_point".to_owned();
    /// let heavenly_grape = "curly_tin".to_owned();
    /// let object_dyo = Object::new(&mut store, jobless_hobbies, unkempt_humor, heavenly_grape);
    /// let callous_stage = "lacking_blade".to_owned();
    ///
    /// let state = State::new(&mut store, &object_dyo, callous_stage);
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
    /// pub isa: `Isa`,
    ///
    pub isa: Uuid,
    /// pub obj_id: `Object`,
    ///
    pub obj_id: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"subtype-struct-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"subtype-new_impl"}}}
impl Subtype {
    /// Inter a new Subtype and return it's `id`
    ///
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Isa;
    /// # use sarzak::sarzak::Subtype;
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Supertype;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let silly_adjustment = "fallacious_notebook".to_owned();
    /// let idiotic_kitty = "green_family".to_owned();
    /// let jittery_yoke = "cuddly_anger".to_owned();
    /// let object_nxk = Object::new(&mut store, silly_adjustment, idiotic_kitty, jittery_yoke);
    /// let supertype_vnk = Supertype::new(&mut store, &object_nxk);
    /// let isa_mwf = Isa::new(&mut store, &supertype_vnk, 42);
    /// let conscious_rest = "icky_kitty".to_owned();
    /// let defeated_crate = "observant_change".to_owned();
    /// let previous_needle = "faint_sidewalk".to_owned();
    /// let object_zca = Object::new(&mut store, conscious_rest, defeated_crate, previous_needle);
    ///
    /// let subtype = Subtype::new(&mut store, &isa_mwf, &object_zca);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(store: &mut ObjectStore, isa: &Isa, obj_id: &Object) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}::{:?}::", isa, obj_id,).as_bytes());
        let new = Self {
            id,
            isa: isa.id,
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"supertype-new_impl"}}}
impl Supertype {
    /// Inter a new Supertype and return it's `id`
    ///
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Supertype;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let soggy_library = "pretty_fan".to_owned();
    /// let pink_crime = "wacky_event".to_owned();
    /// let mysterious_flight = "annoying_home".to_owned();
    /// let object_taa = Object::new(&mut store, soggy_library, pink_crime, mysterious_flight);
    ///
    /// let supertype = Supertype::new(&mut store, &object_taa);
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
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"ty-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Type {
    /// `Boolean(Boolean)`,
    ///
    Boolean(Uuid),
    /// `Reference(Reference)`,
    ///
    Reference(Uuid),
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
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"{}-enum-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"Type-enum-get-id-impl"}}}
impl Type {
    pub fn get_id(&self) -> Uuid {
        match *self {
            Self::Boolean(z) => z,
            Self::Reference(z) => z,
            Self::String(z) => z,
            Self::Uuid(z) => z,
            Self::Float(z) => z,
            Self::Integer(z) => z,
        }
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"Type-enum-get-id-impl"}}}

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
