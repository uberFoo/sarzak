//! v2::woog Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`Mutability`]
//! * [`ObjectMethod`]
//! * [`Parameter`]
//! * [`Visibility`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog-object-store-definition"}}}
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v2::woog::types::{Mutability, ObjectMethod, Parameter, Visibility};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    mutability: HashMap<Uuid, Mutability>,
    object_method: HashMap<Uuid, ObjectMethod>,
    parameter: HashMap<Uuid, Parameter>,
    visibility: HashMap<Uuid, Visibility>,
}

impl ObjectStore {
    pub fn new() -> Self {
        Self {
            mutability: HashMap::new(),
            object_method: HashMap::new(),
            parameter: HashMap::new(),
            visibility: HashMap::new(),
        }
    }

    /// Inter [`Mutability`] into the store.
    ///
    pub fn inter_mutability(&mut self, mutability: Mutability) {
        self.mutability.insert(mutability.id(), mutability);
    }

    /// Exhume [`Mutability`] from the store.
    ///
    pub fn exhume_mutability(&self, id: &Uuid) -> Option<&Mutability> {
        self.mutability.get(id)
    }
    /// Exhume [`Mutability`] from the store — mutably.
    ///
    pub fn exhume_mutability_mut(&mut self, id: &Uuid) -> Option<&mut Mutability> {
        self.mutability.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Mutability>`.
    ///
    pub fn iter_mutability(&self) -> impl Iterator<Item = (&Uuid, &Mutability)> {
        self.mutability.iter()
    }
    /// Inter [`ObjectMethod`] into the store.
    ///
    pub fn inter_object_method(&mut self, object_method: ObjectMethod) {
        self.object_method.insert(object_method.id, object_method);
    }

    /// Exhume [`ObjectMethod`] from the store.
    ///
    pub fn exhume_object_method(&self, id: &Uuid) -> Option<&ObjectMethod> {
        self.object_method.get(id)
    }
    /// Exhume [`ObjectMethod`] from the store — mutably.
    ///
    pub fn exhume_object_method_mut(&mut self, id: &Uuid) -> Option<&mut ObjectMethod> {
        self.object_method.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, ObjectMethod>`.
    ///
    pub fn iter_object_method(&self) -> impl Iterator<Item = (&Uuid, &ObjectMethod)> {
        self.object_method.iter()
    }
    /// Inter [`Parameter`] into the store.
    ///
    pub fn inter_parameter(&mut self, parameter: Parameter) {
        self.parameter.insert(parameter.id, parameter);
    }

    /// Exhume [`Parameter`] from the store.
    ///
    pub fn exhume_parameter(&self, id: &Uuid) -> Option<&Parameter> {
        self.parameter.get(id)
    }
    /// Exhume [`Parameter`] from the store — mutably.
    ///
    pub fn exhume_parameter_mut(&mut self, id: &Uuid) -> Option<&mut Parameter> {
        self.parameter.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Parameter>`.
    ///
    pub fn iter_parameter(&self) -> impl Iterator<Item = (&Uuid, &Parameter)> {
        self.parameter.iter()
    }
    /// Inter [`Visibility`] into the store.
    ///
    pub fn inter_visibility(&mut self, visibility: Visibility) {
        self.visibility.insert(visibility.id(), visibility);
    }

    /// Exhume [`Visibility`] from the store.
    ///
    pub fn exhume_visibility(&self, id: &Uuid) -> Option<&Visibility> {
        self.visibility.get(id)
    }
    /// Exhume [`Visibility`] from the store — mutably.
    ///
    pub fn exhume_visibility_mut(&mut self, id: &Uuid) -> Option<&mut Visibility> {
        self.visibility.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Visibility>`.
    ///
    pub fn iter_visibility(&self) -> impl Iterator<Item = (&Uuid, &Visibility)> {
        self.visibility.iter()
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
