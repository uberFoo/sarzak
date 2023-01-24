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

impl AcknowledgedEvent {
    // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"acknowledged_event-new_impl"}}}
    /// Inter a new AcknowledgedEvent and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::AcknowledgedEvent;
    /// # use sarzak::sarzak::State;
    /// # use sarzak::sarzak::Event;
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let terrific_yam = "curvy_dime".to_owned();
    /// let zippy_owner = "aquatic_arch".to_owned();
    /// let delirious_reward = "spiffy_team".to_owned();
    /// let object_xpb = Object::new(&mut store, terrific_yam, zippy_owner, delirious_reward);
    /// let unused_hope = "acoustic_town".to_owned();
    /// let state_zau = State::new(&mut store, &object_xpb, unused_hope);
    /// let gentle_coast = "wacky_believe".to_owned();
    /// let successful_glass = "glistening_crate".to_owned();
    /// let rambunctious_distance = "lazy_cause".to_owned();
    /// let object_bqk = Object::new(&mut store, gentle_coast, successful_glass, rambunctious_distance);
    /// let unbecoming_frogs = "rainy_owl".to_owned();
    /// let event_ulg = Event::new(&mut store, &object_bqk, unbecoming_frogs);
    ///
    /// let acknowledged_event = AcknowledgedEvent::new(&mut store, &state_zau, &event_ulg);
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
    /// # use sarzak::sarzak::OtherSide;
    /// # use sarzak::sarzak::OneSide;
    /// # use sarzak::sarzak::AssociativeSide;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let animated_ray = "violent_fight".to_owned();
    /// let amusing_sneeze = "aspiring_process".to_owned();
    /// let pale_credit = "thirsty_engine".to_owned();
    /// let object_pji = Object::new(&mut store, animated_ray, amusing_sneeze, pale_credit);
    /// let one_side_rim = OneSide::new(&mut store, &object_pji);
    /// let excited_voice = "youthful_bat".to_owned();
    /// let outrageous_sweater = "old_cushion".to_owned();
    /// let lewd_oven = "standing_hand".to_owned();
    /// let object_dnm = Object::new(&mut store, excited_voice, outrageous_sweater, lewd_oven);
    /// let other_side_isa = OtherSide::new(&mut store, &object_dnm);
    /// let craven_gold = "exuberant_butter".to_owned();
    /// let abiding_cherry = "hanging_swim".to_owned();
    /// let red_mother = "unbiased_process".to_owned();
    /// let object_rcb = Object::new(&mut store, craven_gold, abiding_cherry, red_mother);
    /// let associative_side_ggy = AssociativeSide::new(&mut store, &object_rcb);
    ///
    /// let associative = Associative::new(&mut store, &one_side_rim, &other_side_isa, &associative_side_ggy, 42);
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"associative-extrude_impl"}}}
impl Extrude<nut::sarzak::Associative, Context<'_>> for Associative {
    fn extrude(orig: nut::sarzak::Associative, context: &mut Context<'_>) -> Self {
        let Context { from, ref mut to } = context;

        Self::default()
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"associative-extrude_impl"}}}

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
    /// # use sarzak::sarzak::AssociativeSide;
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let aboard_brick = "untidy_country".to_owned();
    /// let poor_father = "present_doll".to_owned();
    /// let fallacious_cattle = "abnormal_egg".to_owned();
    /// let object_jiw = Object::new(&mut store, aboard_brick, poor_father, fallacious_cattle);
    ///
    /// let associative_side = AssociativeSide::new(&mut store, &object_jiw);
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"associative_side-extrude_impl", "is_uber":true}}}
impl Extrude<nut::sarzak::AssociativeReferrer, Context<'_>> for AssociativeSide {
    fn extrude(orig: nut::sarzak::AssociativeReferrer, context: &mut Context<'_>) -> Self {
        let Context { from, ref mut to } = context;

        Self::default()
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"associative_side-extrude_impl"}}}

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
    /// # use sarzak::sarzak::Type;
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Attribute;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let flimsy_blade = "amuck_clocks".to_owned();
    /// let embarrassed_carriage = "coordinated_shelf".to_owned();
    /// let purple_corn = "jumbled_sisters".to_owned();
    /// let object = Object::new(&mut store, flimsy_blade, embarrassed_carriage, purple_corn);
    /// let type_gdt = Type::test_default(&mut store);
    /// let wary_cake = "actually_lamp".to_owned();
    ///
    /// let attribute = Attribute::new(&mut store, Some(&object), &type_gdt, wary_cake);
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

impl Binary {
    // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"binary-new_impl"}}}
    /// Inter a new Binary and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Cardinality;
    /// # use sarzak::sarzak::Referrer;
    /// # use sarzak::sarzak::Referent;
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Conditionality;
    /// # use sarzak::sarzak::Binary;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let conditionality_fsp = Conditionality::test_default(&mut store);
    /// let gigantic_observation = "tacit_scissors".to_owned();
    /// let cuddly_slip = "oval_lace".to_owned();
    /// let ubiquitous_hose = "scrawny_group".to_owned();
    /// let object_pnw = Object::new(&mut store, gigantic_observation, cuddly_slip, ubiquitous_hose);
    /// let cardinality_hwo = Cardinality::test_default(&mut store);
    /// let referent_kwm = Referent::new(&mut store, &conditionality_fsp, &object_pnw, &cardinality_hwo);
    /// let conditionality_uuw = Conditionality::test_default(&mut store);
    /// let chilly_attraction = "lackadaisical_key".to_owned();
    /// let last_apparel = "deep_distance".to_owned();
    /// let sable_brush = "cool_tendency".to_owned();
    /// let object_vel = Object::new(&mut store, chilly_attraction, last_apparel, sable_brush);
    /// let cardinality_uzb = Cardinality::test_default(&mut store);
    /// let rebel_metal = "cloudy_tent".to_owned();
    /// let referrer_usg = Referrer::new(&mut store, &conditionality_uuw, &object_vel, &cardinality_uzb, rebel_metal);
    ///
    /// let binary = Binary::new(&mut store, &referent_kwm, &referrer_usg, 42);
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
    /// let disturbed_whistle = "nervous_fifth".to_owned();
    /// let optimal_spy = "pricey_quiver".to_owned();
    /// let questionable_arch = "overwrought_cast".to_owned();
    /// let object_umu = Object::new(&mut store, disturbed_whistle, optimal_spy, questionable_arch);
    /// let questionable_language = "real_chess".to_owned();
    ///
    /// let event = Event::new(&mut store, &object_umu, questionable_language);
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

impl Isa {
    // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"isa-new_impl"}}}
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
    /// let abstracted_badge = "nifty_geese".to_owned();
    /// let rampant_horses = "obeisant_education".to_owned();
    /// let steep_soda = "general_hook".to_owned();
    /// let object_zyt = Object::new(&mut store, abstracted_badge, rampant_horses, steep_soda);
    /// let supertype_bgj = Supertype::new(&mut store, &object_zyt);
    ///
    /// let isa = Isa::new(&mut store, &supertype_bgj, 42);
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
    /// let gullible_vein = "supreme_dock".to_owned();
    /// let spiritual_toad = "plastic_snake".to_owned();
    /// let plain_form = "young_ball".to_owned();
    ///
    /// let object = Object::new(&mut store, gullible_vein, spiritual_toad, plain_form);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(
        store: &mut ObjectStore,
        description: std::string::String,
        key_letters: std::string::String,
        name: std::string::String,
    ) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{}::{}::{}::", description, key_letters, name,).as_bytes(),
        );
        let new = Self {
            id,
            description,
            key_letters,
            name,
        };

        store.inter_object(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"object-new_impl"}}}
}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"object-extrude_impl", "is_uber": true}}}
impl Extrude<nut::sarzak::Object, Context<'_>> for Object {
    fn extrude(orig: nut::sarzak::Object, context: &mut Context<'_>) -> Self {
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
    /// let lean_party = "wretched_coil".to_owned();
    /// let evasive_songs = "tender_jeans".to_owned();
    /// let tangy_earth = "hard_railway".to_owned();
    /// let object_lzj = Object::new(&mut store, lean_party, evasive_songs, tangy_earth);
    ///
    /// let one_side = OneSide::new(&mut store, &object_lzj);
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"one_side-extrude_impl", "is_uber": true}}}
impl Extrude<nut::sarzak::AssociativeReferent, Context<'_>> for OneSide {
    fn extrude(orig: nut::sarzak::AssociativeReferent, context: &mut Context<'_>) -> Self {
        let Context { from, ref mut to } = context;

        Self::default()
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"one_side-extrude_impl"}}}

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
    /// let violet_beginner = "changeable_jump".to_owned();
    /// let damaging_hand = "foolish_chance".to_owned();
    /// let loud_pest = "spiky_tramp".to_owned();
    /// let object_pxk = Object::new(&mut store, violet_beginner, damaging_hand, loud_pest);
    ///
    /// let other_side = OtherSide::new(&mut store, &object_pxk);
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"other_side-extrude_impl", "is_uber": true}}}
impl Extrude<nut::sarzak::AssociativeReferrer, Context<'_>> for OtherSide {
    fn extrude(orig: nut::sarzak::AssociativeReferrer, context: &mut Context<'_>) -> Self {
        let Context { from, ref mut to } = context;

        Self::default()
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"other_side-extrude_impl"}}}

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
    /// # use sarzak::sarzak::Referent;
    /// # use sarzak::sarzak::Conditionality;
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Cardinality;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let conditionality_xhx = Conditionality::test_default(&mut store);
    /// let raspy_wrench = "filthy_lettuce".to_owned();
    /// let spectacular_lock = "strong_distribution".to_owned();
    /// let thinkable_battle = "meaty_care".to_owned();
    /// let object_qjg = Object::new(&mut store, raspy_wrench, spectacular_lock, thinkable_battle);
    /// let cardinality_qxx = Cardinality::test_default(&mut store);
    ///
    /// let referent = Referent::new(&mut store, &conditionality_xhx, &object_qjg, &cardinality_qxx);
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

impl Referrer {
    // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"referrer-new_impl"}}}
    /// Inter a new Referrer and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Referrer;
    /// # use sarzak::sarzak::Conditionality;
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Cardinality;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let conditionality_nqk = Conditionality::test_default(&mut store);
    /// let eatable_vein = "cruel_bed".to_owned();
    /// let superficial_ball = "married_jellyfish".to_owned();
    /// let subdued_baby = "mixed_trail".to_owned();
    /// let object_pep = Object::new(&mut store, eatable_vein, superficial_ball, subdued_baby);
    /// let cardinality_idn = Cardinality::test_default(&mut store);
    /// let overrated_birds = "mushy_cheese".to_owned();
    ///
    /// let referrer = Referrer::new(&mut store, &conditionality_nqk, &object_pep, &cardinality_idn, overrated_birds);
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
        let burly_harbor = "square_curve".to_owned();
        let puzzled_jar = "ill_informed_soda".to_owned();
        let didactic_question = "premium_bedroom".to_owned();
        let object_trr = Object::new(store, burly_harbor, puzzled_jar, didactic_question);
        let supertype_chv = Supertype::new(store, &object_trr);
        let test = Self::Isa(Isa::new(store, &supertype_chv, 42).id);
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
                let Associative = Associative::extrude(a.clone(), context);
                let id = Associative.id;
                context.to.inter_associative(Associative);
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
    /// let brave_manager = "homely_class".to_owned();
    /// let weary_linen = "untidy_downtown".to_owned();
    /// let exuberant_letters = "fascinated_pets".to_owned();
    /// let object_xkj = Object::new(&mut store, brave_manager, weary_linen, exuberant_letters);
    /// let concerned_daughter = "obscene_digestion".to_owned();
    ///
    /// let state = State::new(&mut store, &object_xkj, concerned_daughter);
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

impl Subtype {
    // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"subtype-new_impl"}}}
    /// Inter a new Subtype and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin"}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Supertype;
    /// # use sarzak::sarzak::Subtype;
    /// # use sarzak::sarzak::Isa;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let plausible_note = "freezing_quilt".to_owned();
    /// let awful_wax = "free_iron".to_owned();
    /// let eight_stream = "smelly_boot".to_owned();
    /// let object_aof = Object::new(&mut store, plausible_note, awful_wax, eight_stream);
    /// let supertype_pah = Supertype::new(&mut store, &object_aof);
    /// let isa_wit = Isa::new(&mut store, &supertype_pah, 42);
    /// let pastoral_chain = "questionable_party".to_owned();
    /// let gaping_plastic = "tightfisted_foot".to_owned();
    /// let lying_driving = "demonic_idea".to_owned();
    /// let object_htx = Object::new(&mut store, pastoral_chain, gaping_plastic, lying_driving);
    ///
    /// let subtype = Subtype::new(&mut store, &isa_wit, &object_htx);
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
    /// let cultured_key = "teeny_tiny_bit".to_owned();
    /// let dapper_brothers = "macabre_pig".to_owned();
    /// let juvenile_gun = "nifty_appliance".to_owned();
    /// let object_lvs = Object::new(&mut store, cultured_key, dapper_brothers, juvenile_gun);
    ///
    /// let supertype = Supertype::new(&mut store, &object_lvs);
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
