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
//!    * [`External`]
//!    * [`FLOAT`]
//!    * [`INTEGER`]
//!    * [`Isa`]
//!    * [`MANY`]
//!    * [`Object`]
//!    * [`ONE`]
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
//!  sarzak gen
//! ```
// {"magic":"","kind":"IgnoreBlockEnd"}
// {"magic":"","version":"0.5.0"}
// {"magic":"","version":"1.0.0"}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"imports", "is_uber": true}}}
use crate::v1::sarzak::store::ObjectStore;
use crate::v1::sarzak::UUID_NS;
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
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use sarzak::v1::sarzak::Object;
    /// # use sarzak::v1::sarzak::AcknowledgedEvent;
    /// # use sarzak::v1::sarzak::State;
    /// # use sarzak::v1::sarzak::Event;
    /// # let mut store = sarzak::v1::sarzak::ObjectStore::new();
    ///
    /// let boiling_join = "dashing_winter".to_owned();
    /// let tawdry_lunchroom = "learned_clock".to_owned();
    /// let vivacious_nut = "many_cherries".to_owned();
    /// let object_ovf = Object::new(&mut store, boiling_join, tawdry_lunchroom, vivacious_nut);
    /// let remarkable_use = "labored_pleasure".to_owned();
    /// let state_scf = State::new(&mut store, &object_ovf, remarkable_use);
    /// let zany_crowd = "lamentable_bone".to_owned();
    /// let uncovered_insurance = "handsome_rate".to_owned();
    /// let knowing_women = "ablaze_mind".to_owned();
    /// let object_xdf = Object::new(&mut store, zany_crowd, uncovered_insurance, knowing_women);
    /// let protective_guitar = "luxuriant_stretch".to_owned();
    /// let event_cah = Event::new(&mut store, &object_xdf, protective_guitar);
    ///
    /// let acknowledged_event = AcknowledgedEvent::new(&mut store, &state_scf, &event_cah);
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
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use sarzak::v1::sarzak::Conditionality;
    /// # use sarzak::v1::sarzak::Cardinality;
    /// # use sarzak::v1::sarzak::AssociativeReferrer;
    /// # use sarzak::v1::sarzak::AssociativeReferent;
    /// # use sarzak::v1::sarzak::Associative;
    /// # use sarzak::v1::sarzak::Object;
    /// # let mut store = sarzak::v1::sarzak::ObjectStore::new();
    ///
    /// let conditionality_xmj = Conditionality::test_default(&mut store);
    /// let charming_disease = "uneven_bubble".to_owned();
    /// let gainful_flower = "graceful_mist".to_owned();
    /// let observant_breakfast = "necessary_coach".to_owned();
    /// let object_azc = Object::new(&mut store, charming_disease, gainful_flower, observant_breakfast);
    /// let cardinality_mqd = Cardinality::test_default(&mut store);
    /// let silky_coat = "guiltless_property".to_owned();
    /// let associative_referent_hbg = AssociativeReferent::new(&mut store, &conditionality_xmj, &object_azc, &cardinality_mqd, silky_coat);
    /// let conditionality_bif = Conditionality::test_default(&mut store);
    /// let frightening_sofa = "halting_ship".to_owned();
    /// let uncovered_army = "therapeutic_rat".to_owned();
    /// let false_haircut = "unarmed_pot".to_owned();
    /// let object_xrv = Object::new(&mut store, frightening_sofa, uncovered_army, false_haircut);
    /// let cardinality_aiu = Cardinality::test_default(&mut store);
    /// let next_thunder = "knowing_pet".to_owned();
    /// let associative_referent_hrb = AssociativeReferent::new(&mut store, &conditionality_bif, &object_xrv, &cardinality_aiu, next_thunder);
    /// let determined_event = "careless_drawer".to_owned();
    /// let fluttering_talk = "boorish_needle".to_owned();
    /// let labored_knee = "bright_hose".to_owned();
    /// let object_see = Object::new(&mut store, determined_event, fluttering_talk, labored_knee);
    /// let cardinality_wrk = Cardinality::test_default(&mut store);
    /// let pushy_lace = "addicted_rings".to_owned();
    /// let uttermost_seat = "flowery_stitch".to_owned();
    /// let associative_referrer_qgt = AssociativeReferrer::new(&mut store, &object_see, &cardinality_wrk, pushy_lace, uttermost_seat);
    ///
    /// let associative = Associative::new(&mut store, &associative_referent_hbg, &associative_referent_hrb, &associative_referrer_qgt, 42);
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
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"associative_referent-struct-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"associative_referent-new_impl"}}}
impl AssociativeReferent {
    /// Inter a new AssociativeReferent and return it's `id`
    ///
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use sarzak::v1::sarzak::Object;
    /// # use sarzak::v1::sarzak::Cardinality;
    /// # use sarzak::v1::sarzak::Conditionality;
    /// # use sarzak::v1::sarzak::AssociativeReferent;
    /// # let mut store = sarzak::v1::sarzak::ObjectStore::new();
    ///
    /// let conditionality_ehn = Conditionality::test_default(&mut store);
    /// let easy_offer = "amused_hearing".to_owned();
    /// let little_flowers = "fascinated_cemetery".to_owned();
    /// let ahead_cake = "ludicrous_coil".to_owned();
    /// let object_wpa = Object::new(&mut store, easy_offer, little_flowers, ahead_cake);
    /// let cardinality_wlm = Cardinality::test_default(&mut store);
    /// let thick_expert = "lame_paint".to_owned();
    ///
    /// let associative_referent = AssociativeReferent::new(&mut store, &conditionality_ehn, &object_wpa, &cardinality_wlm, thick_expert);
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
            description: orig.description.to_owned(),
            cardinality: Cardinality::get_cardinality_from_nut(&orig.cardinality),
            conditionality: Conditionality::get_conditionality_from_nut(&orig.conditionality),
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
    /// pub one_referential_attribute: `std::string::String`,
    ///
    pub one_referential_attribute: std::string::String,
    /// pub other_referential_attribute: `std::string::String`,
    ///
    pub other_referential_attribute: std::string::String,
    /// pub cardinality: `Cardinality`,
    ///
    pub cardinality: Uuid,
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
    /// # use sarzak::v1::sarzak::Object;
    /// # use sarzak::v1::sarzak::Cardinality;
    /// # use sarzak::v1::sarzak::AssociativeReferrer;
    /// # let mut store = sarzak::v1::sarzak::ObjectStore::new();
    ///
    /// let versed_cloud = "efficacious_visitor".to_owned();
    /// let acrid_circle = "different_chance".to_owned();
    /// let exuberant_game = "willing_desire".to_owned();
    /// let object_wux = Object::new(&mut store, versed_cloud, acrid_circle, exuberant_game);
    /// let cardinality_tyt = Cardinality::test_default(&mut store);
    /// let high_pitched_song = "good_tent".to_owned();
    /// let insidious_insurance = "callous_science".to_owned();
    ///
    /// let associative_referrer = AssociativeReferrer::new(&mut store, &object_wux, &cardinality_tyt, high_pitched_song, insidious_insurance);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(
        store: &mut ObjectStore,
        obj_id: &Object,
        cardinality: &Cardinality,
        other_referential_attribute: std::string::String,
        one_referential_attribute: std::string::String,
    ) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!(
                "{:?}::{:?}::{}::{}::",
                obj_id, cardinality, other_referential_attribute, one_referential_attribute,
            )
            .as_bytes(),
        );
        let new = Self {
            id,
            obj_id: obj_id.id,
            cardinality: cardinality.get_id(),
            other_referential_attribute,
            one_referential_attribute,
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
            cardinality: Cardinality::get_cardinality_from_nut(&orig.cardinality),
            obj_id: orig.obj_id,
            one_referential_attribute: orig.one_referential_attribute.inner().clone(),
            other_referential_attribute: orig.other_referential_attribute.inner().clone(),
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
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use sarzak::v1::sarzak::Object;
    /// # use sarzak::v1::sarzak::Attribute;
    /// # use sarzak::v1::sarzak::Type;
    /// # let mut store = sarzak::v1::sarzak::ObjectStore::new();
    ///
    /// let quick_protest = "wicked_rain".to_owned();
    /// let slippery_seat = "rigid_action".to_owned();
    /// let agreeable_development = "groovy_plantation".to_owned();
    /// let object = Object::new(&mut store, quick_protest, slippery_seat, agreeable_development);
    /// let type_ekk = Type::test_default(&mut store);
    /// let joyous_sticks = "mindless_goldfish".to_owned();
    ///
    /// let attribute = Attribute::new(&mut store, Some(&object), &type_ekk, joyous_sticks);
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
                if loop {
                    if let Some((_, attr_id)) = attr_iter.next() {
                        if attr_id == &orig.id {
                            break Some(true);
                        }
                    } else {
                        break None;
                    }
                }.is_some() {
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
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use sarzak::v1::sarzak::Referrer;
    /// # use sarzak::v1::sarzak::Binary;
    /// # use sarzak::v1::sarzak::Object;
    /// # use sarzak::v1::sarzak::Referent;
    /// # use sarzak::v1::sarzak::Conditionality;
    /// # use sarzak::v1::sarzak::Cardinality;
    /// # let mut store = sarzak::v1::sarzak::ObjectStore::new();
    ///
    /// let conditionality_vnj = Conditionality::test_default(&mut store);
    /// let spectacular_brother = "accidental_girl".to_owned();
    /// let neighborly_nerve = "wakeful_friend".to_owned();
    /// let shut_rice = "wholesale_apples".to_owned();
    /// let object_xlf = Object::new(&mut store, spectacular_brother, neighborly_nerve, shut_rice);
    /// let cardinality_ate = Cardinality::test_default(&mut store);
    /// let miscreant_cakes = "quixotic_slave".to_owned();
    /// let referent_tlg = Referent::new(&mut store, &conditionality_vnj, &object_xlf, &cardinality_ate, miscreant_cakes);
    /// let conditionality_wye = Conditionality::test_default(&mut store);
    /// let living_spark = "enormous_ball".to_owned();
    /// let boorish_cars = "elegant_cloth".to_owned();
    /// let far_flung_horses = "torpid_steam".to_owned();
    /// let object_pxk = Object::new(&mut store, living_spark, boorish_cars, far_flung_horses);
    /// let cardinality_nev = Cardinality::test_default(&mut store);
    /// let thirsty_powder = "sweet_cows".to_owned();
    /// let abhorrent_tooth = "material_actor".to_owned();
    /// let referrer_jlq = Referrer::new(&mut store, &conditionality_wye, &object_pxk, &cardinality_nev, thirsty_powder, abhorrent_tooth);
    ///
    /// let binary = Binary::new(&mut store, &referent_tlg, &referrer_jlq, 42);
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
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use sarzak::v1::sarzak::Object;
    /// # use sarzak::v1::sarzak::Event;
    /// # let mut store = sarzak::v1::sarzak::ObjectStore::new();
    ///
    /// let fresh_ocean = "normal_anger".to_owned();
    /// let aboard_ship = "acoustic_quarter".to_owned();
    /// let right_angle = "animated_money".to_owned();
    /// let object_bjs = Object::new(&mut store, fresh_ocean, aboard_ship, right_angle);
    /// let horrible_top = "deranged_treatment".to_owned();
    ///
    /// let event = Event::new(&mut store, &object_bjs, horrible_top);
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

/// External Type
///
/// This may literally be anything. It's used during code generation to generate variables names
/// and type names for things that are outside of a modeled domain. For example, a timer would
/// be an external type. The specifics of how it is used is up to the model compiler.
///
/// In grace, the `name` attribute is used during code generation to create variable names by
/// converting it to `snake_case`. When used as a type, it is converted to `UpperCamelCase`
///.
///
/// We use `path` as the path is a `use` statement.
///
/// I'm updating this while trying to use it, so this description is going to be rather incoherent
/// until things settle down.
///
/// The way I'm using this, and hopefully the way that will always accommodate, is as a singleton
/// within a particular function scope. Maybe it's a system-wide singleton? I dunno. But it's
/// a singleton.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"external-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct External {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub name: `std::string::String`,
    ///
    pub name: std::string::String,
    /// pub path: `std::string::String`,
    ///
    pub path: std::string::String,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"external-struct-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"external-new_impl"}}}
impl External {
    /// Inter a new External and return it's `id`
    ///
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use sarzak::v1::sarzak::External;
    /// # let mut store = sarzak::v1::sarzak::ObjectStore::new();
    ///
    /// let dull_respect = "male_structure".to_owned();
    /// let cloistered_rub = "huge_measure".to_owned();
    ///
    /// let external = External::new(&mut store, dull_respect, cloistered_rub);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(
        store: &mut ObjectStore,
        name: std::string::String,
        path: std::string::String,
    ) -> Self {
        let id = Uuid::new_v5(&UUID_NS, format!("{}::{}::", name, path,).as_bytes());
        let new = Self { id, name, path };

        store.inter_external(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"external-new_impl"}}}
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
    /// # use sarzak::v1::sarzak::Supertype;
    /// # use sarzak::v1::sarzak::Isa;
    /// # use sarzak::v1::sarzak::Object;
    /// # let mut store = sarzak::v1::sarzak::ObjectStore::new();
    ///
    /// let big_books = "simplistic_stranger".to_owned();
    /// let milky_square = "orange_bait".to_owned();
    /// let glib_tree = "wanting_push".to_owned();
    /// let object_wzz = Object::new(&mut store, big_books, milky_square, glib_tree);
    /// let supertype_qtb = Supertype::new(&mut store, &object_wzz);
    ///
    /// let isa = Isa::new(&mut store, &supertype_qtb, 42);
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
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
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
    /// # use sarzak::v1::sarzak::Object;
    /// # let mut store = sarzak::v1::sarzak::ObjectStore::new();
    ///
    /// let somber_woman = "draconian_water".to_owned();
    /// let grumpy_transport = "even_plastic".to_owned();
    /// let astonishing_bit = "youthful_cake".to_owned();
    ///
    /// let object = Object::new(&mut store, somber_woman, grumpy_transport, astonishing_bit);
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
    /// # use sarzak::v1::sarzak::Object;
    /// # use sarzak::v1::sarzak::Referent;
    /// # use sarzak::v1::sarzak::Conditionality;
    /// # use sarzak::v1::sarzak::Cardinality;
    /// # let mut store = sarzak::v1::sarzak::ObjectStore::new();
    ///
    /// let conditionality_lwa = Conditionality::test_default(&mut store);
    /// let nonchalant_time = "glib_paint".to_owned();
    /// let damaging_shake = "strange_light".to_owned();
    /// let raspy_lamp = "scary_magic".to_owned();
    /// let object_txc = Object::new(&mut store, nonchalant_time, damaging_shake, raspy_lamp);
    /// let cardinality_egm = Cardinality::test_default(&mut store);
    /// let womanly_shoe = "slimy_frogs".to_owned();
    ///
    /// let referent = Referent::new(&mut store, &conditionality_lwa, &object_txc, &cardinality_egm, womanly_shoe);
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
            //             format!("{:?}::{:?}::{:?}::", conditionality, obj_id, cardinality,).as_bytes(), //⚡️
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

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"referent-extrude_impl", "is_uber": true}}}
impl Extrude<nut::sarzak::Referent, Context<'_>> for Referent {
    fn extrude(orig: nut::sarzak::Referent, _context: &mut Context<'_>) -> Self {
        Self {
            id: orig.id,
            description: orig.description.to_owned(),
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
    /// # use sarzak::v1::sarzak::Referrer;
    /// # use sarzak::v1::sarzak::Object;
    /// # use sarzak::v1::sarzak::Conditionality;
    /// # use sarzak::v1::sarzak::Cardinality;
    /// # let mut store = sarzak::v1::sarzak::ObjectStore::new();
    ///
    /// let conditionality_ccp = Conditionality::test_default(&mut store);
    /// let rude_earthquake = "utter_ducks".to_owned();
    /// let puzzled_umbrella = "shallow_face".to_owned();
    /// let ambiguous_cows = "cloudy_business".to_owned();
    /// let object_vbx = Object::new(&mut store, rude_earthquake, puzzled_umbrella, ambiguous_cows);
    /// let cardinality_dvk = Cardinality::test_default(&mut store);
    /// let yellow_donkey = "angry_party".to_owned();
    /// let short_boat = "hospitable_fuel".to_owned();
    ///
    /// let referrer = Referrer::new(&mut store, &conditionality_ccp, &object_vbx, &cardinality_dvk, yellow_donkey, short_boat);
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
                //                 "{:?}::{:?}::{:?}::{}::", //⚡️
                //                 conditionality, obj_id, cardinality, referential_attribute, //⚡️
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
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"referrer-extrude_impl", "is_uber": true}}}
impl Extrude<nut::sarzak::Referrer, Context<'_>> for Referrer {
    fn extrude(orig: nut::sarzak::Referrer, _context: &mut Context<'_>) -> Self {
        Self {
            id: orig.id,
            description: orig.description.to_owned(),
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
        // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
        let spotless_degree = "accurate_pie".to_owned();
        let true_insurance = "pumped_kittens".to_owned();
        let spiritual_cabbage = "adaptable_pizzas".to_owned();
        let object_ccq = Object::new(store, spotless_degree, true_insurance, spiritual_cabbage);
        let supertype_why = Supertype::new(store, &object_ccq);
        let test = Self::Isa(Isa::new(store, &supertype_why, 42).id);
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
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use sarzak::v1::sarzak::Object;
    /// # use sarzak::v1::sarzak::State;
    /// # let mut store = sarzak::v1::sarzak::ObjectStore::new();
    ///
    /// let nauseating_current = "equal_start".to_owned();
    /// let illegal_decision = "homely_milk".to_owned();
    /// let mountainous_impulse = "cooperative_thumb".to_owned();
    /// let object_hca = Object::new(&mut store, nauseating_current, illegal_decision, mountainous_impulse);
    /// let gaudy_fall = "romantic_icicle".to_owned();
    ///
    /// let state = State::new(&mut store, &object_hca, gaudy_fall);
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
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use sarzak::v1::sarzak::Object;
    /// # use sarzak::v1::sarzak::Supertype;
    /// # use sarzak::v1::sarzak::Subtype;
    /// # use sarzak::v1::sarzak::Isa;
    /// # let mut store = sarzak::v1::sarzak::ObjectStore::new();
    ///
    /// let cruel_thunder = "supreme_measure".to_owned();
    /// let unsightly_pan = "unbecoming_middle".to_owned();
    /// let utopian_voyage = "combative_society".to_owned();
    /// let object_omm = Object::new(&mut store, cruel_thunder, unsightly_pan, utopian_voyage);
    /// let supertype_gxl = Supertype::new(&mut store, &object_omm);
    /// let isa_rdy = Isa::new(&mut store, &supertype_gxl, 42);
    /// let apathetic_lizards = "funny_canvas".to_owned();
    /// let tense_use = "embarrassed_apparatus".to_owned();
    /// let lewd_nerve = "troubled_record".to_owned();
    /// let object_lne = Object::new(&mut store, apathetic_lizards, tense_use, lewd_nerve);
    ///
    /// let subtype = Subtype::new(&mut store, &isa_rdy, &object_lne);
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
                if loop {
                    if let Some(sub_id) = sub_iter.next() {
                        if sub_id == &orig.id {
                            break Some(true);
                        }
                    } else {
                        break None;
                    }
                }.is_some() {
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
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use sarzak::v1::sarzak::Supertype;
    /// # use sarzak::v1::sarzak::Object;
    /// # let mut store = sarzak::v1::sarzak::ObjectStore::new();
    ///
    /// let condemned_purpose = "mammoth_wire".to_owned();
    /// let immense_poison = "alluring_slope".to_owned();
    /// let adaptable_shock = "comfortable_stranger".to_owned();
    /// let object_hnd = Object::new(&mut store, condemned_purpose, immense_poison, adaptable_shock);
    ///
    /// let supertype = Supertype::new(&mut store, &object_hnd);
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
    /// `Object(Object)`,
    ///
    Object(Uuid),
    /// `String(String)`,
    ///
    String(Uuid),
    /// `Uuid(Uuid)`,
    ///
    Uuid(Uuid),
    /// `External(External)`,
    ///
    External(Uuid),
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
            Self::Object(z) => z,
            Self::String(z) => z,
            Self::Uuid(z) => z,
            Self::External(z) => z,
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
