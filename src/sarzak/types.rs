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
// {"magic":"","version":"1.0.0"}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"imports"}}}
use crate::sarzak::store::ObjectStore;
use crate::sarzak::UUID_NS;
use nut::codegen::{Extrude, SarzakObjectStore};
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"imports"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"context-extrude_impl", "is_uber": true}}}
pub(crate) struct Context<'a> {
    pub(crate) from: &'a SarzakObjectStore,
    pub(crate) to: &'a mut ObjectStore,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"context-extrude_impl"}}}

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
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Event;
    /// # use sarzak::sarzak::State;
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::AcknowledgedEvent;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let nervous_sack = "literate_back".to_owned();
    /// let helpful_shelf = "ignorant_property".to_owned();
    /// let overt_marble = "tranquil_rhythm".to_owned();
    /// let object_dlp = Object::new(&mut store, nervous_sack, helpful_shelf, overt_marble);
    /// let living_sleet = "bad_canvas".to_owned();
    /// let state_dca = State::new(&mut store, &object_dlp, living_sleet);
    /// let instinctive_good_bye = "macho_price".to_owned();
    /// let old_fashioned_war = "zonked_chin".to_owned();
    /// let ten_sneeze = "elderly_finger".to_owned();
    /// let object_etk = Object::new(&mut store, instinctive_good_bye, old_fashioned_war, ten_sneeze);
    /// let better_cake = "equable_balance".to_owned();
    /// let event_ota = Event::new(&mut store, &object_etk, better_cake);
    ///
    /// let acknowledged_event = AcknowledgedEvent::new(&mut store, &state_dca, &event_ota);
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"acknowledged_event-extrude_impl", "is_uber": true}}}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"acknowledged_event-extrude_impl"}}}

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
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::AssociativeReferent;
    /// # use sarzak::sarzak::Associative;
    /// # use sarzak::sarzak::AssociativeReferrer;
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let massive_sticks = "present_reading".to_owned();
    /// let chilly_education = "madly_nose".to_owned();
    /// let complete_flight = "oceanic_cheese".to_owned();
    /// let object_cea = Object::new(&mut store, massive_sticks, chilly_education, complete_flight);
    /// let associative_referent_szg = AssociativeReferent::new(&mut store, &object_cea);
    /// let closed_invention = "high_pitched_sky".to_owned();
    /// let ethereal_potato = "gusty_kitten".to_owned();
    /// let wasteful_garden = "tangible_knife".to_owned();
    /// let object_tzz = Object::new(&mut store, closed_invention, ethereal_potato, wasteful_garden);
    /// let associative_referent_sra = AssociativeReferent::new(&mut store, &object_tzz);
    /// let harsh_tendency = "useful_plot".to_owned();
    /// let brawny_passenger = "juicy_cake".to_owned();
    /// let unused_scent = "wonderful_space".to_owned();
    /// let object_qch = Object::new(&mut store, harsh_tendency, brawny_passenger, unused_scent);
    /// let associative_referrer_wkq = AssociativeReferrer::new(&mut store, &object_qch);
    ///
    /// let associative = Associative::new(&mut store, &associative_referent_szg, &associative_referent_sra, &associative_referrer_wkq, 42);
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"associative-extrude_impl", "is_uber": true}}}
impl Extrude<nut::sarzak::Associative, Context<'_>> for Associative {
    fn extrude(orig: nut::sarzak::Associative, context: &mut Context<'_>) -> Self {
        let from = context
            .from
            .exhume_associative_referrer(&orig.from)
            .unwrap();
        let from = AssociativeReferrer::extrude(from.clone(), context);
        context.to.inter_associative_referrer(from.clone());

        let one = context.from.exhume_associative_referent(&orig.one).unwrap();
        let one = AssociativeReferent::extrude(one.clone(), context);
        context.to.inter_associative_referent(one.clone());

        let other = context
            .from
            .exhume_associative_referent(&orig.other)
            .unwrap();
        let other = AssociativeReferent::extrude(other.clone(), context);
        context.to.inter_associative_referent(other.clone());

        Self {
            id: orig.id,
            number: orig.number as i64,
            from: from.id,
            one: one.id,
            other: other.id,
        }
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"associative-extrude_impl"}}}

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
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::AssociativeReferent;
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let conscious_change = "scattered_root".to_owned();
    /// let selfish_boot = "pale_grain".to_owned();
    /// let jazzy_meal = "groovy_fairies".to_owned();
    /// let object_hbc = Object::new(&mut store, conscious_change, selfish_boot, jazzy_meal);
    ///
    /// let associative_referent = AssociativeReferent::new(&mut store, &object_hbc);
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"associative_referent-extrude_impl", "is_uber": true}}}
impl Extrude<nut::sarzak::AssociativeReferent, Context<'_>> for AssociativeReferent {
    fn extrude(orig: nut::sarzak::AssociativeReferent, _context: &mut Context<'_>) -> Self {
        Self {
            id: orig.id,
            obj_id: orig.obj_id,
        }
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"associative_referent-extrude_impl"}}}

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
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::AssociativeReferrer;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let chubby_protest = "normal_person".to_owned();
    /// let previous_fact = "late_minister".to_owned();
    /// let delightful_balloon = "truculent_mark".to_owned();
    /// let object_vpy = Object::new(&mut store, chubby_protest, previous_fact, delightful_balloon);
    ///
    /// let associative_referrer = AssociativeReferrer::new(&mut store, &object_vpy);
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"associative_referrer-extrude_impl", "is_uber": true}}}
impl Extrude<nut::sarzak::AssociativeReferrer, Context<'_>> for AssociativeReferrer {
    fn extrude(orig: nut::sarzak::AssociativeReferrer, _context: &mut Context<'_>) -> Self {
        Self {
            id: orig.id,
            obj_id: orig.obj_id,
        }
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"associative_referrer-extrude_impl"}}}

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
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Type;
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Attribute;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let blue_eyed_fact = "fumbling_iron".to_owned();
    /// let round_can = "deeply_fork".to_owned();
    /// let needless_table = "dirty_birth".to_owned();
    /// let object = Object::new(&mut store, blue_eyed_fact, round_can, needless_table);
    /// let type_avi = Type::test_default(&mut store);
    /// let selfish_bun = "truculent_spiders".to_owned();
    ///
    /// let attribute = Attribute::new(&mut store, Some(&object), &type_avi, selfish_bun);
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"attribute-extrude_impl", "is_uber": true}}}
impl Extrude<nut::sarzak::Attribute, Context<'_>> for Attribute {
    fn extrude(orig: nut::sarzak::Attribute, context: &mut Context<'_>) -> Self {
        // Ugh, this is a slog.
        // In nut, the attributes are stored in a dict (name, uuid) hanging off
        // of object. We need to search all of the objects, and all of their
        // attributes to find the one with this id.
        let mut obj_iter = context.from.iter_object();
        let obj_id = loop {
            if let Some((_, obj)) = obj_iter.next() {
                let mut attr_iter = obj.attributes.iter();
                if let Some(_) = loop {
                    if let Some((_, attr_id)) = attr_iter.next() {
                        if attr_id == &orig.id {
                            break Some(true);
                        }
                    } else {
                        break None;
                    }
                } {
                    break obj.id;
                }
            }
        };

        Self {
            id: orig.id,
            name: orig.name.inner().clone(),
            obj_id: Some(obj_id),
            ty: Type::get_type_from_nut(&orig.attr_t),
        }
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"attribute-extrude_impl"}}}

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
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Cardinality;
    /// # use sarzak::sarzak::Referrer;
    /// # use sarzak::sarzak::Conditionality;
    /// # use sarzak::sarzak::Referent;
    /// # use sarzak::sarzak::Binary;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let conditionality_khc = Conditionality::test_default(&mut store);
    /// let robust_shape = "wicked_sock".to_owned();
    /// let testy_eye = "wide_eyed_payment".to_owned();
    /// let simple_berry = "brainy_floor".to_owned();
    /// let object_pqo = Object::new(&mut store, robust_shape, testy_eye, simple_berry);
    /// let cardinality_ror = Cardinality::test_default(&mut store);
    /// let referent_fzr = Referent::new(&mut store, &conditionality_khc, &object_pqo, &cardinality_ror);
    /// let conditionality_eqk = Conditionality::test_default(&mut store);
    /// let brainy_suggestion = "damp_actor".to_owned();
    /// let wrathful_star = "aggressive_downtown".to_owned();
    /// let tasteless_smile = "loutish_milk".to_owned();
    /// let object_ojl = Object::new(&mut store, brainy_suggestion, wrathful_star, tasteless_smile);
    /// let cardinality_yfg = Cardinality::test_default(&mut store);
    /// let profuse_level = "penitent_yak".to_owned();
    /// let referrer_rmn = Referrer::new(&mut store, &conditionality_eqk, &object_ojl, &cardinality_yfg, profuse_level);
    ///
    /// let binary = Binary::new(&mut store, &referent_fzr, &referrer_rmn, 42);
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"binary-extrude_impl", "is_uber": true}}}
impl Extrude<nut::sarzak::Binary, Context<'_>> for Binary {
    fn extrude(orig: nut::sarzak::Binary, context: &mut Context<'_>) -> Self {
        let referrer = context.from.exhume_referrer(&orig.from).unwrap();
        let referrer = Referrer::extrude(referrer.clone(), context);
        let referrer_id = referrer.id;
        context.to.inter_referrer(referrer);

        let referent = context.from.exhume_referent(&orig.to).unwrap();
        let referent = Referent::extrude(referent.clone(), context);
        let referent_id = referent.id;
        context.to.inter_referent(referent);

        Self {
            id: orig.id,
            number: orig.number as i64,
            from: referrer_id,
            to: referent_id,
        }
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"binary-extrude_impl"}}}

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

impl Cardinality {
    fn get_cardinality_from_nut(card: &nut::sarzak::Cardinality) -> Uuid {
        match card {
            nut::sarzak::Cardinality::One => ONE,
            nut::sarzak::Cardinality::Many => MANY,
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

impl Conditionality {
    fn get_conditionality_from_nut(cond: &nut::sarzak::Conditionality) -> Uuid {
        match cond {
            nut::sarzak::Conditionality::Conditional => CONDITIONAL,
            nut::sarzak::Conditionality::Unconditional => UNCONDITIONAL,
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"event-new_impl"}}}
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
    /// let ablaze_society = "spicy_snake".to_owned();
    /// let female_seashore = "cheap_cord".to_owned();
    /// let spooky_test = "macho_parent".to_owned();
    /// let object_ccq = Object::new(&mut store, ablaze_society, female_seashore, spooky_test);
    /// let axiomatic_match = "cheerful_club".to_owned();
    ///
    /// let event = Event::new(&mut store, &object_ccq, axiomatic_match);
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"event-extrude_impl", "is_uber": true}}}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"event-extrude_impl"}}}

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
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Supertype;
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Isa;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let ceaseless_thumb = "private_loss".to_owned();
    /// let questionable_view = "bumpy_milk".to_owned();
    /// let squalid_zebra = "internal_shame".to_owned();
    /// let object_ujl = Object::new(&mut store, ceaseless_thumb, questionable_view, squalid_zebra);
    /// let supertype_ifj = Supertype::new(&mut store, &object_ujl);
    ///
    /// let isa = Isa::new(&mut store, &supertype_ifj, 42);
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"isa-extrude_impl", "is_uber": true}}}
impl Extrude<nut::sarzak::Isa, Context<'_>> for Isa {
    fn extrude(orig: nut::sarzak::Isa, context: &mut Context<'_>) -> Self {
        let supertype = context.from.exhume_supertype(&orig.supertype).unwrap();
        let supertype = Supertype::extrude(supertype.clone(), context);
        let id = supertype.id;
        context.to.inter_supertype(supertype);

        Self {
            id: orig.id,
            number: orig.number as i64,
            supertype: id,
        }
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"isa-extrude_impl"}}}

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
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let neighborly_shirt = "irate_pies".to_owned();
    /// let small_horses = "disgusting_bite".to_owned();
    /// let normal_knife = "efficient_rate".to_owned();
    ///
    /// let object = Object::new(&mut store, neighborly_shirt, small_horses, normal_knife);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(
        store: &mut ObjectStore,
        description: std::string::String,
        //         key_letters: std::string::String, //⚡️
        name: std::string::String,
        key_letters: std::string::String,
    ) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            //             format!("{}::{}::{}::", description, key_letters, name,).as_bytes(), //⚡️
            format!("{}::{}::{}::", description, name, key_letters,).as_bytes(),
        );
        let new = Self {
            id,
            description,
            //             key_letters, //⚡️
            name,
            key_letters,
        };

        store.inter_object(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"object-new_impl"}}}
}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"object-extrude_impl", "is_uber": true}}}
impl Extrude<nut::sarzak::Object, Context<'_>> for Object {
    fn extrude(orig: nut::sarzak::Object, _context: &mut Context<'_>) -> Self {
        Self {
            id: orig.id,
            description: orig.description,
            key_letters: orig.key_letter,
            name: orig.name.inner().clone(),
        }
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"object-extrude_impl"}}}

/// A constant value that indicates a cardinality of _one_.
///
/// ❗️{"singleton_object": true}
///
//
pub const ONE: Uuid = uuid!["bf6924bb-089d-5c1f-bc1f-123ba1fd1ea3"];

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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"referent-new_impl"}}}
impl Referent {
    /// Inter a new Referent and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Cardinality;
    /// # use sarzak::sarzak::Conditionality;
    /// # use sarzak::sarzak::Referent;
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let conditionality_mlj = Conditionality::test_default(&mut store);
    /// let oval_weight = "loving_polish".to_owned();
    /// let fumbling_observation = "imported_pie".to_owned();
    /// let slippery_start = "selfish_zebra".to_owned();
    /// let object_yuk = Object::new(&mut store, oval_weight, fumbling_observation, slippery_start);
    /// let cardinality_gpv = Cardinality::test_default(&mut store);
    ///
    /// let referent = Referent::new(&mut store, &conditionality_mlj, &object_yuk, &cardinality_gpv);
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"referent-extrude_impl", "is_uber": true}}}
impl Extrude<nut::sarzak::Referent, Context<'_>> for Referent {
    fn extrude(orig: nut::sarzak::Referent, _context: &mut Context<'_>) -> Self {
        Self {
            id: orig.id,
            cardinality: Cardinality::get_cardinality_from_nut(&orig.cardinality),
            conditionality: Conditionality::get_conditionality_from_nut(&orig.conditionality),
            obj_id: orig.obj_id,
        }
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"referent-extrude_impl"}}}

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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"referrer-new_impl"}}}
impl Referrer {
    /// Inter a new Referrer and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Cardinality;
    /// # use sarzak::sarzak::Referrer;
    /// # use sarzak::sarzak::Conditionality;
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let conditionality_nob = Conditionality::test_default(&mut store);
    /// let temporary_offer = "jazzy_beam".to_owned();
    /// let truculent_basketball = "violet_cakes".to_owned();
    /// let earthy_blood = "troubled_woman".to_owned();
    /// let object_eso = Object::new(&mut store, temporary_offer, truculent_basketball, earthy_blood);
    /// let cardinality_wxj = Cardinality::test_default(&mut store);
    /// let frightened_beetle = "legal_corn".to_owned();
    ///
    /// let referrer = Referrer::new(&mut store, &conditionality_nob, &object_eso, &cardinality_wxj, frightened_beetle);
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
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"referrer-extrude_impl", "is_uber": true}}}
impl Extrude<nut::sarzak::Referrer, Context<'_>> for Referrer {
    fn extrude(orig: nut::sarzak::Referrer, _context: &mut Context<'_>) -> Self {
        Self {
            id: orig.id,
            referential_attribute: orig.referential_attribute.inner().clone(),
            cardinality: Cardinality::get_cardinality_from_nut(&orig.cardinality),
            conditionality: Conditionality::get_conditionality_from_nut(&orig.conditionality),
            obj_id: orig.obj_id,
        }
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"referrer-extrude_impl"}}}

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
        // {"magic":"","kind":"IgnoreBlockBegin"}
        let eminent_shelf = "full_horn".to_owned();
        let adorable_heat = "pointless_cup".to_owned();
        let sticky_pot = "illegal_nut".to_owned();
        let object_ccy = Object::new(store, eminent_shelf, adorable_heat, sticky_pot);
        let supertype_uzr = Supertype::new(store, &object_ccy);
        let test = Self::Isa(Isa::new(store, &supertype_uzr, 42).id);
        // {"magic":"","kind":"IgnoreBlockEnd"}

        store.inter_relationship(test.clone());

        test
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"relationship-test_default"}}}

impl Extrude<nut::sarzak::Relationship, Context<'_>> for Relationship {
    fn extrude(orig: nut::sarzak::Relationship, context: &mut Context<'_>) -> Self {
        match orig {
            nut::sarzak::Relationship::Binary(b_id) => {
                let b = context.from.exhume_binary(&b_id).unwrap();
                let binary = Binary::extrude(b.clone(), context);
                let id = binary.id;
                context.to.inter_binary(binary);

                Self::Binary(id)
            }
            nut::sarzak::Relationship::Isa(i_id) => {
                let i = context.from.exhume_isa(&i_id).unwrap();
                let isa = Isa::extrude(i.clone(), context);
                let id = isa.id;
                context.to.inter_isa(isa);

                Self::Isa(id)
            }
            nut::sarzak::Relationship::Associative(a_id) => {
                let a = context.from.exhume_associative(&a_id).unwrap();
                let associative = Associative::extrude(a.clone(), context);
                let id = associative.id;
                context.to.inter_associative(associative);

                Self::Associative(id)
            }
        }
    }
}

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
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::State;
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let chemical_badge = "spectacular_knowledge".to_owned();
    /// let knotty_mice = "ancient_degree".to_owned();
    /// let dynamic_dolls = "elastic_square".to_owned();
    /// let object_jqp = Object::new(&mut store, chemical_badge, knotty_mice, dynamic_dolls);
    /// let rampant_operation = "husky_truck".to_owned();
    ///
    /// let state = State::new(&mut store, &object_jqp, rampant_operation);
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"state-extrude_impl", "is_uber": true}}}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"state-extrude_impl"}}}

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
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Isa;
    /// # use sarzak::sarzak::Subtype;
    /// # use sarzak::sarzak::Supertype;
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let victorious_talk = "accessible_thought".to_owned();
    /// let jumpy_kittens = "deadpan_tramp".to_owned();
    /// let imperfect_drum = "wistful_dust".to_owned();
    /// let object_ogz = Object::new(&mut store, victorious_talk, jumpy_kittens, imperfect_drum);
    /// let supertype_efv = Supertype::new(&mut store, &object_ogz);
    /// let isa_lyr = Isa::new(&mut store, &supertype_efv, 42);
    /// let immense_marble = "maddening_string".to_owned();
    /// let quarrelsome_hall = "wrong_coat".to_owned();
    /// let forgetful_toad = "tranquil_boats".to_owned();
    /// let object_jan = Object::new(&mut store, immense_marble, quarrelsome_hall, forgetful_toad);
    ///
    /// let subtype = Subtype::new(&mut store, &isa_lyr, &object_jan);
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"subtype-extrude_impl", "is_uber": true}}}
impl Extrude<nut::sarzak::Subtype, Context<'_>> for Subtype {
    fn extrude(orig: nut::sarzak::Subtype, context: &mut Context<'_>) -> Self {
        // In nut the subtypes are stored in a Vec hanging off of Isa.
        // We search all the Isa's, and all their subtypes, looking for this
        // id. The code is sort of dense, and using a loop as the rhs in an if
        // let is an interesting choice. On one hand I like it, and on another
        // I wonder if it couldn't be clearer? I think it's better than a
        // nested for loop with sentinels.
        let mut isa_iter = context.from.iter_isa();
        let isa_id = loop {
            if let Some((_, isa)) = isa_iter.next() {
                let mut sub_iter = isa.subtypes.iter();
                if let Some(_) = loop {
                    if let Some(sub_id) = sub_iter.next() {
                        if sub_id == &orig.id {
                            break Some(true);
                        }
                    } else {
                        break None;
                    }
                } {
                    break isa.id;
                }
            }
        };

        Self {
            id: orig.id,
            isa: isa_id,
            obj_id: orig.obj_id,
        }
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"subtype-extrude_impl"}}}

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
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Supertype;
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let scattered_cast = "famous_question".to_owned();
    /// let vigorous_force = "domineering_seed".to_owned();
    /// let incompetent_rose = "last_measure".to_owned();
    /// let object_ejc = Object::new(&mut store, scattered_cast, vigorous_force, incompetent_rose);
    ///
    /// let supertype = Supertype::new(&mut store, &object_ejc);
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"supertype-extrude_impl", "is_uber": true}}}
impl Extrude<nut::sarzak::Supertype, Context<'_>> for Supertype {
    fn extrude(orig: nut::sarzak::Supertype, _context: &mut Context<'_>) -> Self {
        Self {
            id: orig.id,
            obj_id: orig.obj_id,
        }
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"supertype-extrude_impl"}}}

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
            Self::String(z) => z,
            Self::Uuid(z) => z,
            Self::Float(z) => z,
            Self::Integer(z) => z,
        }
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"Type-enum-get-id-impl"}}}

impl Type {
    fn get_type_from_nut(ty: &nut::sarzak::Type) -> Uuid {
        match ty {
            nut::sarzak::Type::Boolean => BOOLEAN,
            nut::sarzak::Type::Float => FLOAT,
            nut::sarzak::Type::Integer => INTEGER,
            nut::sarzak::Type::String => STRING,
            nut::sarzak::Type::Uuid => UUID,
            nut::sarzak::Type::ForeignKey(_) => panic!("ForeignKey is invalid."),
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
