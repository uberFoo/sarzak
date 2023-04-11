//! v2::lu_dog Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`Field`]
//! * [`Function`]
//! * [`Implementation`]
//! * [`Item`]
//! * [`ModelType`]
//! * [`WoogOption`]
//! * [`Some`]
//! * [`ValueType`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog-object-store-definition"}}}
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
    time::SystemTime,
};

use fnv::FnvHashMap as HashMap;
use heck::ToUpperCamelCase;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v2::lu_dog::types::{
    Field, Function, Implementation, Item, ModelType, Some, ValueType, WoogOption, NONE,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    field: HashMap<Uuid, (Field, SystemTime)>,
    field_by_name: HashMap<String, (Field, SystemTime)>,
    function: HashMap<Uuid, (Function, SystemTime)>,
    function_by_name: HashMap<String, (Function, SystemTime)>,
    implementation: HashMap<Uuid, (Implementation, SystemTime)>,
    item: HashMap<Uuid, (Item, SystemTime)>,
    model_type: HashMap<Uuid, (ModelType, SystemTime)>,
    woog_option: HashMap<Uuid, (WoogOption, SystemTime)>,
    some: HashMap<Uuid, (Some, SystemTime)>,
    value_type: HashMap<Uuid, (ValueType, SystemTime)>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            field: HashMap::default(),
            field_by_name: HashMap::default(),
            function: HashMap::default(),
            function_by_name: HashMap::default(),
            implementation: HashMap::default(),
            item: HashMap::default(),
            model_type: HashMap::default(),
            woog_option: HashMap::default(),
            some: HashMap::default(),
            value_type: HashMap::default(),
        };

        // Initialize Singleton Subtypes
        store.inter_woog_option(WoogOption::None(NONE));
        store.inter_value_type(ValueType::WoogOption(WoogOption::None(NONE).id()));

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog-object-store-methods"}}}
    /// Inter [`Field`] into the store.
    ///
    pub fn inter_field(&mut self, field: Field) {
        let value = (field, SystemTime::now());
        self.field.insert(value.0.id, value.clone());
        self.field_by_name
            .insert(value.0.name.to_upper_camel_case(), value);
    }

    /// Exhume [`Field`] from the store.
    ///
    pub fn exhume_field(&self, id: &Uuid) -> Option<&Field> {
        self.field.get(id).map(|field| &field.0)
    }

    /// Exhume [`Field`] from the store — mutably.
    ///
    pub fn exhume_field_mut(&mut self, id: &Uuid) -> Option<&mut Field> {
        self.field.get_mut(id).map(|field| &mut field.0)
    }

    /// Exhume [`Field`] from the store by name.
    ///
    pub fn exhume_field_by_name(&self, name: &str) -> Option<&Field> {
        self.field_by_name.get(name).map(|field| &field.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Field>`.
    ///
    pub fn iter_field(&self) -> impl Iterator<Item = &Field> {
        self.field.values().map(|field| &field.0)
    }

    /// Get the timestamp for Field.
    ///
    pub fn field_timestamp(&self, field: &Field) -> SystemTime {
        self.field
            .get(&field.id)
            .map(|field| field.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Function`] into the store.
    ///
    pub fn inter_function(&mut self, function: Function) {
        let value = (function, SystemTime::now());
        self.function.insert(value.0.id, value.clone());
        self.function_by_name
            .insert(value.0.name.to_upper_camel_case(), value);
    }

    /// Exhume [`Function`] from the store.
    ///
    pub fn exhume_function(&self, id: &Uuid) -> Option<&Function> {
        self.function.get(id).map(|function| &function.0)
    }

    /// Exhume [`Function`] from the store — mutably.
    ///
    pub fn exhume_function_mut(&mut self, id: &Uuid) -> Option<&mut Function> {
        self.function.get_mut(id).map(|function| &mut function.0)
    }

    /// Exhume [`Function`] from the store by name.
    ///
    pub fn exhume_function_by_name(&self, name: &str) -> Option<&Function> {
        self.function_by_name.get(name).map(|function| &function.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Function>`.
    ///
    pub fn iter_function(&self) -> impl Iterator<Item = &Function> {
        self.function.values().map(|function| &function.0)
    }

    /// Get the timestamp for Function.
    ///
    pub fn function_timestamp(&self, function: &Function) -> SystemTime {
        self.function
            .get(&function.id)
            .map(|function| function.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Implementation`] into the store.
    ///
    pub fn inter_implementation(&mut self, implementation: Implementation) {
        self.implementation
            .insert(implementation.id, (implementation, SystemTime::now()));
    }

    /// Exhume [`Implementation`] from the store.
    ///
    pub fn exhume_implementation(&self, id: &Uuid) -> Option<&Implementation> {
        self.implementation
            .get(id)
            .map(|implementation| &implementation.0)
    }

    /// Exhume [`Implementation`] from the store — mutably.
    ///
    pub fn exhume_implementation_mut(&mut self, id: &Uuid) -> Option<&mut Implementation> {
        self.implementation
            .get_mut(id)
            .map(|implementation| &mut implementation.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Implementation>`.
    ///
    pub fn iter_implementation(&self) -> impl Iterator<Item = &Implementation> {
        self.implementation
            .values()
            .map(|implementation| &implementation.0)
    }

    /// Get the timestamp for Implementation.
    ///
    pub fn implementation_timestamp(&self, implementation: &Implementation) -> SystemTime {
        self.implementation
            .get(&implementation.id)
            .map(|implementation| implementation.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Item`] into the store.
    ///
    pub fn inter_item(&mut self, item: Item) {
        self.item.insert(item.id(), (item, SystemTime::now()));
    }

    /// Exhume [`Item`] from the store.
    ///
    pub fn exhume_item(&self, id: &Uuid) -> Option<&Item> {
        self.item.get(id).map(|item| &item.0)
    }

    /// Exhume [`Item`] from the store — mutably.
    ///
    pub fn exhume_item_mut(&mut self, id: &Uuid) -> Option<&mut Item> {
        self.item.get_mut(id).map(|item| &mut item.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Item>`.
    ///
    pub fn iter_item(&self) -> impl Iterator<Item = &Item> {
        self.item.values().map(|item| &item.0)
    }

    /// Get the timestamp for Item.
    ///
    pub fn item_timestamp(&self, item: &Item) -> SystemTime {
        self.item
            .get(&item.id())
            .map(|item| item.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`ModelType`] into the store.
    ///
    pub fn inter_model_type(&mut self, model_type: ModelType) {
        self.model_type
            .insert(model_type.id, (model_type, SystemTime::now()));
    }

    /// Exhume [`ModelType`] from the store.
    ///
    pub fn exhume_model_type(&self, id: &Uuid) -> Option<&ModelType> {
        self.model_type.get(id).map(|model_type| &model_type.0)
    }

    /// Exhume [`ModelType`] from the store — mutably.
    ///
    pub fn exhume_model_type_mut(&mut self, id: &Uuid) -> Option<&mut ModelType> {
        self.model_type
            .get_mut(id)
            .map(|model_type| &mut model_type.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ModelType>`.
    ///
    pub fn iter_model_type(&self) -> impl Iterator<Item = &ModelType> {
        self.model_type.values().map(|model_type| &model_type.0)
    }

    /// Get the timestamp for ModelType.
    ///
    pub fn model_type_timestamp(&self, model_type: &ModelType) -> SystemTime {
        self.model_type
            .get(&model_type.id)
            .map(|model_type| model_type.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`WoogOption`] into the store.
    ///
    pub fn inter_woog_option(&mut self, woog_option: WoogOption) {
        self.woog_option
            .insert(woog_option.id(), (woog_option, SystemTime::now()));
    }

    /// Exhume [`WoogOption`] from the store.
    ///
    pub fn exhume_woog_option(&self, id: &Uuid) -> Option<&WoogOption> {
        self.woog_option.get(id).map(|woog_option| &woog_option.0)
    }

    /// Exhume [`WoogOption`] from the store — mutably.
    ///
    pub fn exhume_woog_option_mut(&mut self, id: &Uuid) -> Option<&mut WoogOption> {
        self.woog_option
            .get_mut(id)
            .map(|woog_option| &mut woog_option.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, WoogOption>`.
    ///
    pub fn iter_woog_option(&self) -> impl Iterator<Item = &WoogOption> {
        self.woog_option.values().map(|woog_option| &woog_option.0)
    }

    /// Get the timestamp for WoogOption.
    ///
    pub fn woog_option_timestamp(&self, woog_option: &WoogOption) -> SystemTime {
        self.woog_option
            .get(&woog_option.id())
            .map(|woog_option| woog_option.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Some`] into the store.
    ///
    pub fn inter_some(&mut self, some: Some) {
        self.some.insert(some.id, (some, SystemTime::now()));
    }

    /// Exhume [`Some`] from the store.
    ///
    pub fn exhume_some(&self, id: &Uuid) -> Option<&Some> {
        self.some.get(id).map(|some| &some.0)
    }

    /// Exhume [`Some`] from the store — mutably.
    ///
    pub fn exhume_some_mut(&mut self, id: &Uuid) -> Option<&mut Some> {
        self.some.get_mut(id).map(|some| &mut some.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Some>`.
    ///
    pub fn iter_some(&self) -> impl Iterator<Item = &Some> {
        self.some.values().map(|some| &some.0)
    }

    /// Get the timestamp for Some.
    ///
    pub fn some_timestamp(&self, some: &Some) -> SystemTime {
        self.some
            .get(&some.id)
            .map(|some| some.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`ValueType`] into the store.
    ///
    pub fn inter_value_type(&mut self, value_type: ValueType) {
        self.value_type
            .insert(value_type.id(), (value_type, SystemTime::now()));
    }

    /// Exhume [`ValueType`] from the store.
    ///
    pub fn exhume_value_type(&self, id: &Uuid) -> Option<&ValueType> {
        self.value_type.get(id).map(|value_type| &value_type.0)
    }

    /// Exhume [`ValueType`] from the store — mutably.
    ///
    pub fn exhume_value_type_mut(&mut self, id: &Uuid) -> Option<&mut ValueType> {
        self.value_type
            .get_mut(id)
            .map(|value_type| &mut value_type.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ValueType>`.
    ///
    pub fn iter_value_type(&self) -> impl Iterator<Item = &ValueType> {
        self.value_type.values().map(|value_type| &value_type.0)
    }

    /// Get the timestamp for ValueType.
    ///
    pub fn value_type_timestamp(&self, value_type: &ValueType) -> SystemTime {
        self.value_type
            .get(&value_type.id())
            .map(|value_type| value_type.1)
            .unwrap_or(SystemTime::now())
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog-object-store-persistence"}}}
    /// Persist the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automaagic git integration as an option.
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        fs::create_dir_all(&path)?;

        let bin_path = path.clone().join("lu_dog.bin");
        let mut bin_file = fs::File::create(bin_path)?;
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        bin_file.write_all(&encoded)?;

        let path = path.join("lu_dog.json");
        fs::create_dir_all(&path)?;

        // Persist Field.
        {
            let path = path.join("field");
            fs::create_dir_all(&path)?;
            for field_tuple in self.field.values() {
                let path = path.join(format!("{}.json", field_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Field, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != field_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &field_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &field_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.field.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Function.
        {
            let path = path.join("function");
            fs::create_dir_all(&path)?;
            for function_tuple in self.function.values() {
                let path = path.join(format!("{}.json", function_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Function, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != function_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &function_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &function_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.function.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Implementation.
        {
            let path = path.join("implementation");
            fs::create_dir_all(&path)?;
            for implementation_tuple in self.implementation.values() {
                let path = path.join(format!("{}.json", implementation_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Implementation, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != implementation_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &implementation_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &implementation_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.implementation.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Item.
        {
            let path = path.join("item");
            fs::create_dir_all(&path)?;
            for item_tuple in self.item.values() {
                let path = path.join(format!("{}.json", item_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Item, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != item_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &item_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &item_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.item.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Model Type.
        {
            let path = path.join("model_type");
            fs::create_dir_all(&path)?;
            for model_type_tuple in self.model_type.values() {
                let path = path.join(format!("{}.json", model_type_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (ModelType, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != model_type_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &model_type_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &model_type_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.model_type.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Option.
        {
            let path = path.join("woog_option");
            fs::create_dir_all(&path)?;
            for woog_option_tuple in self.woog_option.values() {
                let path = path.join(format!("{}.json", woog_option_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (WoogOption, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != woog_option_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &woog_option_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &woog_option_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.woog_option.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Some.
        {
            let path = path.join("some");
            fs::create_dir_all(&path)?;
            for some_tuple in self.some.values() {
                let path = path.join(format!("{}.json", some_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Some, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != some_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &some_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &some_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.some.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Value Type.
        {
            let path = path.join("value_type");
            fs::create_dir_all(&path)?;
            for value_type_tuple in self.value_type.values() {
                let path = path.join(format!("{}.json", value_type_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (ValueType, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != value_type_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &value_type_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &value_type_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.value_type.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
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
        let path = path.join("lu_dog.json");

        let mut store = Self::new();

        // Load Field.
        {
            let path = path.join("field");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let field: (Field, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .field_by_name
                    .insert(field.0.name.to_upper_camel_case(), field.clone());
                store.field.insert(field.0.id, field);
            }
        }

        // Load Function.
        {
            let path = path.join("function");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let function: (Function, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .function_by_name
                    .insert(function.0.name.to_upper_camel_case(), function.clone());
                store.function.insert(function.0.id, function);
            }
        }

        // Load Implementation.
        {
            let path = path.join("implementation");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let implementation: (Implementation, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .implementation
                    .insert(implementation.0.id, implementation);
            }
        }

        // Load Item.
        {
            let path = path.join("item");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let item: (Item, SystemTime) = serde_json::from_reader(reader)?;
                store.item.insert(item.0.id(), item);
            }
        }

        // Load Model Type.
        {
            let path = path.join("model_type");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let model_type: (ModelType, SystemTime) = serde_json::from_reader(reader)?;
                store.model_type.insert(model_type.0.id, model_type);
            }
        }

        // Load Option.
        {
            let path = path.join("woog_option");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let woog_option: (WoogOption, SystemTime) = serde_json::from_reader(reader)?;
                store.woog_option.insert(woog_option.0.id(), woog_option);
            }
        }

        // Load Some.
        {
            let path = path.join("some");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let some: (Some, SystemTime) = serde_json::from_reader(reader)?;
                store.some.insert(some.0.id, some);
            }
        }

        // Load Value Type.
        {
            let path = path.join("value_type");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let value_type: (ValueType, SystemTime) = serde_json::from_reader(reader)?;
                store.value_type.insert(value_type.0.id(), value_type);
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
