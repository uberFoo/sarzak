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
//!  sarzak gen sarzak -i true -m true -d true -e true
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
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::State;
    /// # use sarzak::sarzak::AcknowledgedEvent;
    /// # use sarzak::sarzak::Event;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let hissing_peace = "knotty_book".to_owned();
    /// let obsolete_sea = "animated_doll".to_owned();
    /// let object_vvu = Object::new(&mut store, hissing_peace, obsolete_sea);
    /// let standing_cherry = "labored_eggs".to_owned();
    /// let state_uzm = State::new(&mut store, &object_vvu, standing_cherry);
    /// let fluffy_condition = "old_fashioned_statement".to_owned();
    /// let kaput_houses = "reflective_bead".to_owned();
    /// let object_atg = Object::new(&mut store, fluffy_condition, kaput_houses);
    /// let two_apples = "wild_kiss".to_owned();
    /// let event_qqw = Event::new(&mut store, &object_atg, two_apples);
    ///
    /// let acknowledged_event = AcknowledgedEvent::new(&mut store, &state_uzm, &event_qqw);
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
    /// # use sarzak::sarzak::OneSide;
    /// # use sarzak::sarzak::Associative;
    /// # use sarzak::sarzak::OtherSide;
    /// # use sarzak::sarzak::AssociativeSide;
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let acid_coal = "various_roll".to_owned();
    /// let mighty_behavior = "spiky_rod".to_owned();
    /// let object_bdk = Object::new(&mut store, acid_coal, mighty_behavior);
    /// let one_side_dpl = OneSide::new(&mut store, &object_bdk);
    /// let delicate_fuel = "tough_drawer".to_owned();
    /// let feeble_size = "zonked_lip".to_owned();
    /// let object_zbp = Object::new(&mut store, delicate_fuel, feeble_size);
    /// let other_side_fcj = OtherSide::new(&mut store, &object_zbp);
    /// let ritzy_bushes = "fearful_history".to_owned();
    /// let hesitant_knowledge = "futuristic_earthquake".to_owned();
    /// let object_twb = Object::new(&mut store, ritzy_bushes, hesitant_knowledge);
    /// let associative_side_fbq = AssociativeSide::new(&mut store, &object_twb);
    ///
    /// let associative = Associative::new(&mut store, &one_side_dpl, &other_side_fcj, &associative_side_fbq, 42);
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
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::AssociativeSide;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let towering_shelf = "flaky_motion".to_owned();
    /// let aware_hobbies = "far_flung_floor".to_owned();
    /// let object_ain = Object::new(&mut store, towering_shelf, aware_hobbies);
    ///
    /// let associative_side = AssociativeSide::new(&mut store, &object_ain);
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
    /// # use sarzak::sarzak::Attribute;
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Type;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let careful_rhythm = "exuberant_truck".to_owned();
    /// let towering_babies = "stereotyped_shape".to_owned();
    /// let object = Object::new(&mut store, careful_rhythm, towering_babies);
    /// let type_mqc = Type::test_default(&mut store);
    /// let third_clocks = "tough_feeling".to_owned();
    ///
    /// let attribute = Attribute::new(&mut store, Some(&object), &type_mqc, third_clocks);
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"attribute-extrude_impl"}}}
impl Extrude<nut::sarzak::Attribute, Context<'_>> for Attribute {
    fn extrude(orig: nut::sarzak::Attribute, context: &mut Context<'_>) -> Self {
        let Context { from, ref mut to } = context;

        Self::default()
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
    /// # use sarzak::sarzak::Referrer;
    /// # use sarzak::sarzak::Referent;
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Binary;
    /// # use sarzak::sarzak::Conditionality;
    /// # use sarzak::sarzak::Cardinality;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let conditionality_jlf = Conditionality::test_default(&mut store);
    /// let scrawny_skin = "caring_jelly".to_owned();
    /// let intelligent_brass = "unnatural_caption".to_owned();
    /// let object_kdu = Object::new(&mut store, scrawny_skin, intelligent_brass);
    /// let cardinality_zqk = Cardinality::test_default(&mut store);
    /// let referent_dgg = Referent::new(&mut store, &conditionality_jlf, &object_kdu, &cardinality_zqk);
    /// let conditionality_nll = Conditionality::test_default(&mut store);
    /// let worried_opinion = "internal_quilt".to_owned();
    /// let healthy_mark = "slippery_creature".to_owned();
    /// let object_myq = Object::new(&mut store, worried_opinion, healthy_mark);
    /// let cardinality_voe = Cardinality::test_default(&mut store);
    /// let unique_joke = "tightfisted_reason".to_owned();
    /// let referrer_phm = Referrer::new(&mut store, &conditionality_nll, &object_myq, &cardinality_voe, unique_joke);
    ///
    /// let binary = Binary::new(&mut store, &referent_dgg, &referrer_phm, 42);
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"binary-extrude_impl"}}}
impl Extrude<nut::sarzak::Binary, Context<'_>> for Binary {
    fn extrude(orig: nut::sarzak::Binary, context: &mut Context<'_>) -> Self {
        let Context { from, ref mut to } = context;

        Self::default()
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
    /// # use sarzak::sarzak::Event;
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let zonked_idea = "violent_iron".to_owned();
    /// let hospitable_ticket = "different_color".to_owned();
    /// let object_hxl = Object::new(&mut store, zonked_idea, hospitable_ticket);
    /// let cluttered_quiver = "murky_sky".to_owned();
    ///
    /// let event = Event::new(&mut store, &object_hxl, cluttered_quiver);
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
        let lying_balance = "gamy_debt".to_owned();
        let illustrious_partner = "grumpy_pan".to_owned();
        let object_fwd = Object::new(store, lying_balance, illustrious_partner);
        let test = Self::Subtype(Subtype::new(store, &object_fwd).id);
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
    /// pub description: `std::string::String`,
    ///
    pub description: std::string::String,
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
    /// let distinct_lunch = "unequal_thought".to_owned();
    /// let political_cats = "truculent_wood".to_owned();
    ///
    /// let object = Object::new(&mut store, distinct_lunch, political_cats);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(
        store: &mut ObjectStore,
        description: std::string::String,
        name: std::string::String,
    ) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{}::{}::", description, name,).as_bytes());
        let new = Self {
            id,
            description,
            name,
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
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::OneSide;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let cautious_trees = "ubiquitous_chair".to_owned();
    /// let ubiquitous_fuel = "bouncy_brake".to_owned();
    /// let object_xca = Object::new(&mut store, cautious_trees, ubiquitous_fuel);
    ///
    /// let one_side = OneSide::new(&mut store, &object_xca);
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
    /// # use sarzak::sarzak::OtherSide;
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let alert_match = "colorful_name".to_owned();
    /// let flawless_letters = "reflective_hill".to_owned();
    /// let object_cet = Object::new(&mut store, alert_match, flawless_letters);
    ///
    /// let other_side = OtherSide::new(&mut store, &object_cet);
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
    /// # use sarzak::sarzak::Conditionality;
    /// # use sarzak::sarzak::Referent;
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::Cardinality;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let conditionality_oyq = Conditionality::test_default(&mut store);
    /// let sassy_start = "upbeat_hole".to_owned();
    /// let mixed_beds = "faint_nail".to_owned();
    /// let object_wmo = Object::new(&mut store, sassy_start, mixed_beds);
    /// let cardinality_fvg = Cardinality::test_default(&mut store);
    ///
    /// let referent = Referent::new(&mut store, &conditionality_oyq, &object_wmo, &cardinality_fvg);
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"referent-extrude_impl"}}}
impl Extrude<nut::sarzak::Referent, Context<'_>> for Referent {
    fn extrude(orig: nut::sarzak::Referent, context: &mut Context<'_>) -> Self {
        let Context { from, ref mut to } = context;

        Self::default()
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
    /// # use sarzak::sarzak::Cardinality;
    /// # use sarzak::sarzak::Referrer;
    /// # use sarzak::sarzak::Conditionality;
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let conditionality_bja = Conditionality::test_default(&mut store);
    /// let careful_stage = "squalid_transport".to_owned();
    /// let common_property = "reflective_pies".to_owned();
    /// let object_qok = Object::new(&mut store, careful_stage, common_property);
    /// let cardinality_cst = Cardinality::test_default(&mut store);
    /// let absurd_system = "satisfying_children".to_owned();
    ///
    /// let referrer = Referrer::new(&mut store, &conditionality_bja, &object_qok, &cardinality_cst, absurd_system);
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
    fn extrude(orig: nut::sarzak::Referrer, context: &mut Context<'_>) -> Self {
        let Context {
            ref from,
            ref mut to,
        } = context;

        Self::default()
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
    /// # use sarzak::sarzak::Object;
    /// # use sarzak::sarzak::State;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let painful_grip = "solid_girls".to_owned();
    /// let savory_care = "omniscient_worm".to_owned();
    /// let object_dby = Object::new(&mut store, painful_grip, savory_care);
    /// let strong_middle = "quixotic_letter".to_owned();
    ///
    /// let state = State::new(&mut store, &object_dby, strong_middle);
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
    /// # use sarzak::sarzak::Subtype;
    /// # let mut store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let drab_bells = "mammoth_arm".to_owned();
    /// let zealous_adjustment = "wary_bike".to_owned();
    /// let object_tmt = Object::new(&mut store, drab_bells, zealous_adjustment);
    ///
    /// let subtype = Subtype::new(&mut store, &object_tmt);
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"subtype-extrude_impl"}}}
impl Extrude<nut::sarzak::Subtype, Context<'_>> for Subtype {
    fn extrude(orig: nut::sarzak::Subtype, context: &mut Context<'_>) -> Self {
        let Context { from, ref mut to } = context;

        Self::default()
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
    /// let damaging_slip = "whispering_crack".to_owned();
    /// let successful_religion = "knowing_rest".to_owned();
    /// let object_nku = Object::new(&mut store, damaging_slip, successful_religion);
    ///
    /// let supertype = Supertype::new(&mut store, &object_nku);
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"supertype-extrude_impl"}}}
impl Extrude<nut::sarzak::Supertype, Context<'_>> for Supertype {
    fn extrude(orig: nut::sarzak::Supertype, context: &mut Context<'_>) -> Self {
        let Context { from, ref mut to } = context;

        Self::default()
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"supertype-extrude_impl"}}}

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
