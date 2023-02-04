//! ObjectStore for the instances of the "Woog" domain
//!
//! An end user should have little need to use this directly.
//!
//! This store contains the following instances:
//!    * [`ObjectMethod`]
//!    * [`Parameter`]
//!    * [`Visibility`]
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
//!  sarzak gen -d woog sarzak -e false
//! ```
// {"magic":"","kind":"IgnoreBlockEnd"}
// {"magic":"","version":"1.0.0"}
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::woog::types::{ObjectMethod, Parameter, Visibility};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    object_method: HashMap<Uuid, ObjectMethod>,
    parameter: HashMap<Uuid, Parameter>,
    visibility: HashMap<Uuid, Visibility>,
}

impl ObjectStore {
    pub fn new() -> Self {
        Self {
            object_method: HashMap::new(),
            parameter: HashMap::new(),
            visibility: HashMap::new(),
        }
    }

    /// Inter [`ObjectMethod`] into the [`ObjectStore`]
    ///
    pub fn inter_object_method(&mut self, object_method: ObjectMethod) {
        self.object_method.insert(object_method.id, object_method);
    }

    /// Exhume [`Object Method`] from the [`ObjectStore`]
    ///
    pub fn exhume_object_method(&self, id: &Uuid) -> Option<&ObjectMethod> {
        self.object_method.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, ObjectMethod)>` in the [`ObjectStore`]
    ///
    pub fn iter_object_method(&self) -> impl Iterator<Item = (&Uuid, &ObjectMethod)> {
        self.object_method.iter()
    }

    /// Inter [`Parameter`] into the [`ObjectStore`]
    ///
    pub fn inter_parameter(&mut self, parameter: Parameter) {
        self.parameter.insert(parameter.id, parameter);
    }

    /// Exhume [`Parameter`] from the [`ObjectStore`]
    ///
    pub fn exhume_parameter(&self, id: &Uuid) -> Option<&Parameter> {
        self.parameter.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, Parameter)>` in the [`ObjectStore`]
    ///
    pub fn iter_parameter(&self) -> impl Iterator<Item = (&Uuid, &Parameter)> {
        self.parameter.iter()
    }

    /// Inter [`Visibility`] into the [`ObjectStore`]
    ///
    pub fn inter_visibility(&mut self, visibility: Visibility) {
        self.visibility.insert(visibility.get_id(), visibility);
    }

    /// Exhume [`Visibility`] from the [`ObjectStore`]
    ///
    pub fn exhume_visibility(&self, id: &Uuid) -> Option<&Visibility> {
        self.visibility.get(id)
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, Visibility)>` in the [`ObjectStore`]
    ///
    pub fn iter_visibility(&self) -> impl Iterator<Item = (&Uuid, &Visibility)> {
        self.visibility.iter()
    }
}
