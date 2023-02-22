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
use std::{fs, io, path::Path};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v2::woog::types::{
    Mutability, ObjectMethod, Parameter, Visibility, BORROWED, KRATE, MUTABLE, PRIVATE, PUBLIC,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    mutability: HashMap<Uuid, Mutability>,
    object_method: HashMap<Uuid, ObjectMethod>,
    parameter: HashMap<Uuid, Parameter>,
    visibility: HashMap<Uuid, Visibility>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            mutability: HashMap::new(),
            object_method: HashMap::new(),
            parameter: HashMap::new(),
            visibility: HashMap::new(),
        };

        // Initialize Singleton Subtypes
        store.inter_mutability(Mutability::Borrowed(BORROWED));
        store.inter_mutability(Mutability::Mutable(MUTABLE));
        store.inter_visibility(Visibility::Krate(KRATE));
        store.inter_visibility(Visibility::Private(PRIVATE));
        store.inter_visibility(Visibility::Public(PUBLIC));

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog-object-store-methods"}}}
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
    pub fn iter_mutability(&self) -> impl Iterator<Item = &Mutability> {
        self.mutability.values()
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
    pub fn iter_object_method(&self) -> impl Iterator<Item = &ObjectMethod> {
        self.object_method.values()
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
    pub fn iter_parameter(&self) -> impl Iterator<Item = &Parameter> {
        self.parameter.values()
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
    pub fn iter_visibility(&self) -> impl Iterator<Item = &Visibility> {
        self.visibility.values()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog-object-store-persistence"}}}
    /// Persist the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automaagic git integration as an option.
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref();
        let path = path.join("woog.json");
        fs::create_dir_all(&path)?;

        // Persist mutability.
        {
            let path = path.join("mutability.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.mutability.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist object_method.
        {
            let path = path.join("object_method.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.object_method.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist parameter.
        {
            let path = path.join("parameter.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.parameter.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist visibility.
        {
            let path = path.join("visibility.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.visibility.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        Ok(())
    }

    /// Load the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automaagic git integration as an option.
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let path = path.join("woog.json");

        let mut store = Self::new();

        // Load mutability.
        {
            let path = path.join("mutability.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let mutability: Vec<Mutability> = serde_json::from_reader(reader)?;
            store.mutability = mutability.into_iter().map(|道| (道.id(), 道)).collect();
        }
        // Load object_method.
        {
            let path = path.join("object_method.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let object_method: Vec<ObjectMethod> = serde_json::from_reader(reader)?;
            store.object_method = object_method.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load parameter.
        {
            let path = path.join("parameter.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let parameter: Vec<Parameter> = serde_json::from_reader(reader)?;
            store.parameter = parameter.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load visibility.
        {
            let path = path.join("visibility.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let visibility: Vec<Visibility> = serde_json::from_reader(reader)?;
            store.visibility = visibility.into_iter().map(|道| (道.id(), 道)).collect();
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
