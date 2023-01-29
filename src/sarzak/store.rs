//! ObjectStore for the instances of the "Sarzak" domain
//!
//! An end user should have little need to use this directly.
//!
//! This store contains the following instances:
//!    * [`Isa`]
//!    * [`Associative`]
//!    * [`AcknowledgedEvent`]
//!    * [`Subtype`]
//!    * [`Conditionality`]
//!    * [`Relationship`]
//!    * [`Binary`]
//!    * [`Attribute`]
//!    * [`State`]
//!    * [`Object`]
//!    * [`Referent`]
//!    * [`Referrer`]
//!    * [`Supertype`]
//!    * [`Type`]
//!    * [`Cardinality`]
//!    * [`Event`]
//!    * [`AssociativeReferent`]
//!    * [`AssociativeReferrer`]
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
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::sarzak::types::{
    AcknowledgedEvent, Associative, AssociativeReferent, AssociativeReferrer, Attribute, Binary,
    Cardinality, Conditionality, Event, Isa, Object, Referent, Referrer, Relationship, State,
    Subtype, Supertype, Type,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    isa: HashMap<Uuid, Isa>,
    associative: HashMap<Uuid, Associative>,
    acknowledged_event: HashMap<Uuid, AcknowledgedEvent>,
    subtype: HashMap<Uuid, Subtype>,
    conditionality: HashMap<Uuid, Conditionality>,
    relationship: HashMap<Uuid, Relationship>,
    binary: HashMap<Uuid, Binary>,
    attribute: HashMap<Uuid, Attribute>,
    state: HashMap<Uuid, State>,
    object: HashMap<Uuid, Object>,
    referent: HashMap<Uuid, Referent>,
    referrer: HashMap<Uuid, Referrer>,
    supertype: HashMap<Uuid, Supertype>,
    ty: HashMap<Uuid, Type>,
    cardinality: HashMap<Uuid, Cardinality>,
    event: HashMap<Uuid, Event>,
    associative_referent: HashMap<Uuid, AssociativeReferent>,
    associative_referrer: HashMap<Uuid, AssociativeReferrer>,
}

impl ObjectStore {
    pub fn new() -> Self {
        Self {
            isa: HashMap::new(),
            associative: HashMap::new(),
            acknowledged_event: HashMap::new(),
            subtype: HashMap::new(),
            conditionality: HashMap::new(),
            relationship: HashMap::new(),
            binary: HashMap::new(),
            attribute: HashMap::new(),
            state: HashMap::new(),
            object: HashMap::new(),
            referent: HashMap::new(),
            referrer: HashMap::new(),
            supertype: HashMap::new(),
            ty: HashMap::new(),
            cardinality: HashMap::new(),
            event: HashMap::new(),
            associative_referent: HashMap::new(),
            associative_referrer: HashMap::new(),
        }
    }

    /// Inter [`Isa`] into the [`ObjectStore`]
    ///
    pub fn inter_isa(&mut self, isa: Isa) {
        self.isa.insert(isa.id, isa);
    }

    /// Exhume [`Isa`] from the [`ObjectStore`]
    ///
    pub fn exhume_isa(&self, id: &Uuid) -> Option<&Isa> {
        self.isa.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, Isa)>` in the [`ObjectStore`]
    ///
    pub fn iter_isa(&self) -> impl Iterator<Item = (&Uuid, &Isa)> {
        self.isa.iter()
    }

    /// Inter [`Associative`] into the [`ObjectStore`]
    ///
    pub fn inter_associative(&mut self, associative: Associative) {
        self.associative.insert(associative.id, associative);
    }

    /// Exhume [`Associative`] from the [`ObjectStore`]
    ///
    pub fn exhume_associative(&self, id: &Uuid) -> Option<&Associative> {
        self.associative.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, Associative)>` in the [`ObjectStore`]
    ///
    pub fn iter_associative(&self) -> impl Iterator<Item = (&Uuid, &Associative)> {
        self.associative.iter()
    }

    /// Inter [`AcknowledgedEvent`] into the [`ObjectStore`]
    ///
    pub fn inter_acknowledged_event(&mut self, acknowledged_event: AcknowledgedEvent) {
        self.acknowledged_event
            .insert(acknowledged_event.id, acknowledged_event);
    }

    /// Exhume [`Acknowledged Event`] from the [`ObjectStore`]
    ///
    pub fn exhume_acknowledged_event(&self, id: &Uuid) -> Option<&AcknowledgedEvent> {
        self.acknowledged_event.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, AcknowledgedEvent)>` in the [`ObjectStore`]
    ///
    pub fn iter_acknowledged_event(&self) -> impl Iterator<Item = (&Uuid, &AcknowledgedEvent)> {
        self.acknowledged_event.iter()
    }

    /// Inter [`Subtype`] into the [`ObjectStore`]
    ///
    pub fn inter_subtype(&mut self, subtype: Subtype) {
        self.subtype.insert(subtype.id, subtype);
    }

    /// Exhume [`Subtype`] from the [`ObjectStore`]
    ///
    pub fn exhume_subtype(&self, id: &Uuid) -> Option<&Subtype> {
        self.subtype.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, Subtype)>` in the [`ObjectStore`]
    ///
    pub fn iter_subtype(&self) -> impl Iterator<Item = (&Uuid, &Subtype)> {
        self.subtype.iter()
    }

    /// Inter [`Conditionality`] into the [`ObjectStore`]
    ///
    pub fn inter_conditionality(&mut self, conditionality: Conditionality) {
        self.conditionality
            .insert(conditionality.get_id(), conditionality);
    }

    /// Exhume [`Conditionality`] from the [`ObjectStore`]
    ///
    pub fn exhume_conditionality(&self, id: &Uuid) -> Option<&Conditionality> {
        self.conditionality.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, Conditionality)>` in the [`ObjectStore`]
    ///
    pub fn iter_conditionality(&self) -> impl Iterator<Item = (&Uuid, &Conditionality)> {
        self.conditionality.iter()
    }

    /// Inter [`Relationship`] into the [`ObjectStore`]
    ///
    pub fn inter_relationship(&mut self, relationship: Relationship) {
        self.relationship
            .insert(relationship.get_id(), relationship);
    }

    /// Exhume [`Relationship`] from the [`ObjectStore`]
    ///
    pub fn exhume_relationship(&self, id: &Uuid) -> Option<&Relationship> {
        self.relationship.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, Relationship)>` in the [`ObjectStore`]
    ///
    pub fn iter_relationship(&self) -> impl Iterator<Item = (&Uuid, &Relationship)> {
        self.relationship.iter()
    }

    /// Inter [`Binary`] into the [`ObjectStore`]
    ///
    pub fn inter_binary(&mut self, binary: Binary) {
        self.binary.insert(binary.id, binary);
    }

    /// Exhume [`Binary`] from the [`ObjectStore`]
    ///
    pub fn exhume_binary(&self, id: &Uuid) -> Option<&Binary> {
        self.binary.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, Binary)>` in the [`ObjectStore`]
    ///
    pub fn iter_binary(&self) -> impl Iterator<Item = (&Uuid, &Binary)> {
        self.binary.iter()
    }

    /// Inter [`Attribute`] into the [`ObjectStore`]
    ///
    pub fn inter_attribute(&mut self, attribute: Attribute) {
        self.attribute.insert(attribute.id, attribute);
    }

    /// Exhume [`Attribute`] from the [`ObjectStore`]
    ///
    pub fn exhume_attribute(&self, id: &Uuid) -> Option<&Attribute> {
        self.attribute.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, Attribute)>` in the [`ObjectStore`]
    ///
    pub fn iter_attribute(&self) -> impl Iterator<Item = (&Uuid, &Attribute)> {
        self.attribute.iter()
    }

    /// Inter [`State`] into the [`ObjectStore`]
    ///
    pub fn inter_state(&mut self, state: State) {
        self.state.insert(state.id, state);
    }

    /// Exhume [`State`] from the [`ObjectStore`]
    ///
    pub fn exhume_state(&self, id: &Uuid) -> Option<&State> {
        self.state.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, State)>` in the [`ObjectStore`]
    ///
    pub fn iter_state(&self) -> impl Iterator<Item = (&Uuid, &State)> {
        self.state.iter()
    }

    /// Inter [`Object`] into the [`ObjectStore`]
    ///
    pub fn inter_object(&mut self, object: Object) {
        self.object.insert(object.id, object);
    }

    /// Exhume [`Object`] from the [`ObjectStore`]
    ///
    pub fn exhume_object(&self, id: &Uuid) -> Option<&Object> {
        self.object.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, Object)>` in the [`ObjectStore`]
    ///
    pub fn iter_object(&self) -> impl Iterator<Item = (&Uuid, &Object)> {
        self.object.iter()
    }

    /// Inter [`Referent`] into the [`ObjectStore`]
    ///
    pub fn inter_referent(&mut self, referent: Referent) {
        self.referent.insert(referent.id, referent);
    }

    /// Exhume [`Referent`] from the [`ObjectStore`]
    ///
    pub fn exhume_referent(&self, id: &Uuid) -> Option<&Referent> {
        self.referent.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, Referent)>` in the [`ObjectStore`]
    ///
    pub fn iter_referent(&self) -> impl Iterator<Item = (&Uuid, &Referent)> {
        self.referent.iter()
    }

    /// Inter [`Referrer`] into the [`ObjectStore`]
    ///
    pub fn inter_referrer(&mut self, referrer: Referrer) {
        self.referrer.insert(referrer.id, referrer);
    }

    /// Exhume [`Referrer`] from the [`ObjectStore`]
    ///
    pub fn exhume_referrer(&self, id: &Uuid) -> Option<&Referrer> {
        self.referrer.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, Referrer)>` in the [`ObjectStore`]
    ///
    pub fn iter_referrer(&self) -> impl Iterator<Item = (&Uuid, &Referrer)> {
        self.referrer.iter()
    }

    /// Inter [`Supertype`] into the [`ObjectStore`]
    ///
    pub fn inter_supertype(&mut self, supertype: Supertype) {
        self.supertype.insert(supertype.id, supertype);
    }

    /// Exhume [`Supertype`] from the [`ObjectStore`]
    ///
    pub fn exhume_supertype(&self, id: &Uuid) -> Option<&Supertype> {
        self.supertype.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, Supertype)>` in the [`ObjectStore`]
    ///
    pub fn iter_supertype(&self) -> impl Iterator<Item = (&Uuid, &Supertype)> {
        self.supertype.iter()
    }

    /// Inter [`Type`] into the [`ObjectStore`]
    ///
    pub fn inter_ty(&mut self, ty: Type) {
        self.ty.insert(ty.get_id(), ty);
    }

    /// Exhume [`Type`] from the [`ObjectStore`]
    ///
    pub fn exhume_ty(&self, id: &Uuid) -> Option<&Type> {
        self.ty.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, Type)>` in the [`ObjectStore`]
    ///
    pub fn iter_ty(&self) -> impl Iterator<Item = (&Uuid, &Type)> {
        self.ty.iter()
    }

    /// Inter [`Cardinality`] into the [`ObjectStore`]
    ///
    pub fn inter_cardinality(&mut self, cardinality: Cardinality) {
        self.cardinality.insert(cardinality.get_id(), cardinality);
    }

    /// Exhume [`Cardinality`] from the [`ObjectStore`]
    ///
    pub fn exhume_cardinality(&self, id: &Uuid) -> Option<&Cardinality> {
        self.cardinality.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, Cardinality)>` in the [`ObjectStore`]
    ///
    pub fn iter_cardinality(&self) -> impl Iterator<Item = (&Uuid, &Cardinality)> {
        self.cardinality.iter()
    }

    /// Inter [`Event`] into the [`ObjectStore`]
    ///
    pub fn inter_event(&mut self, event: Event) {
        self.event.insert(event.id, event);
    }

    /// Exhume [`Event`] from the [`ObjectStore`]
    ///
    pub fn exhume_event(&self, id: &Uuid) -> Option<&Event> {
        self.event.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, Event)>` in the [`ObjectStore`]
    ///
    pub fn iter_event(&self) -> impl Iterator<Item = (&Uuid, &Event)> {
        self.event.iter()
    }

    /// Inter [`AssociativeReferent`] into the [`ObjectStore`]
    ///
    pub fn inter_associative_referent(&mut self, associative_referent: AssociativeReferent) {
        self.associative_referent
            .insert(associative_referent.id, associative_referent);
    }

    /// Exhume [`Associative Referent`] from the [`ObjectStore`]
    ///
    pub fn exhume_associative_referent(&self, id: &Uuid) -> Option<&AssociativeReferent> {
        self.associative_referent.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, AssociativeReferent)>` in the [`ObjectStore`]
    ///
    pub fn iter_associative_referent(&self) -> impl Iterator<Item = (&Uuid, &AssociativeReferent)> {
        self.associative_referent.iter()
    }

    /// Inter [`AssociativeReferrer`] into the [`ObjectStore`]
    ///
    pub fn inter_associative_referrer(&mut self, associative_referrer: AssociativeReferrer) {
        self.associative_referrer
            .insert(associative_referrer.id, associative_referrer);
    }

    /// Exhume [`Associative Referrer`] from the [`ObjectStore`]
    ///
    pub fn exhume_associative_referrer(&self, id: &Uuid) -> Option<&AssociativeReferrer> {
        self.associative_referrer.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, AssociativeReferrer)>` in the [`ObjectStore`]
    ///
    pub fn iter_associative_referrer(&self) -> impl Iterator<Item = (&Uuid, &AssociativeReferrer)> {
        self.associative_referrer.iter()
    }
}
