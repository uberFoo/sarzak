//! v2::woog Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`Access`]
//! * [`Block`]
//! * [`Call`]
//! * [`Constant`]
//! * [`Enumeration`]
//! * [`EnumerationField`]
//! * [`Expression`]
//! * [`Field`]
//! * [`Function`]
//! * [`GenerationUnit`]
//! * [`GraceType`]
//! * [`Item`]
//! * [`XLet`]
//! * [`Local`]
//! * [`ObjectMethod`]
//! * [`WoogOption`]
//! * [`Ownership`]
//! * [`Parameter`]
//! * [`Reference`]
//! * [`Statement`]
//! * [`Structure`]
//! * [`StructureField`]
//! * [`SymbolTable`]
//! * [`TimeStamp`]
//! * [`XValue`]
//! * [`Variable`]
//! * [`Visibility`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog-object-store-definition"}}}
use std::sync::Arc;
use std::sync::RwLock;
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
    time::SystemTime,
};

use heck::ToUpperCamelCase;
use rustc_hash::FxHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v2::woog::types::{
    Access, Block, Call, Constant, Enumeration, EnumerationField, Expression, Field, Function,
    GenerationUnit, GraceType, Item, Local, ObjectMethod, Ownership, Parameter, Reference,
    Statement, Structure, StructureField, SymbolTable, TimeStamp, Variable, Visibility, WoogOption,
    XLet, XValue, BORROWED, IMPLEMENTATION, KRATE, LITERAL, MUTABLE, OWNED, PRIVATE, PUBLIC, USIZE,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    access: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Access>>, SystemTime)>>>,
    block: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Block>>, SystemTime)>>>,
    call: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Call>>, SystemTime)>>>,
    constant: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Constant>>, SystemTime)>>>,
    enumeration: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Enumeration>>, SystemTime)>>>,
    enumeration_field: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<EnumerationField>>, SystemTime)>>>,
    expression: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Expression>>, SystemTime)>>>,
    field: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Field>>, SystemTime)>>>,
    field_id_by_name: Arc<RwLock<HashMap<String, (Uuid, SystemTime)>>>,
    function: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Function>>, SystemTime)>>>,
    function_id_by_name: Arc<RwLock<HashMap<String, (Uuid, SystemTime)>>>,
    generation_unit: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<GenerationUnit>>, SystemTime)>>>,
    grace_type: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<GraceType>>, SystemTime)>>>,
    item: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Item>>, SystemTime)>>>,
    x_let: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<XLet>>, SystemTime)>>>,
    local: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Local>>, SystemTime)>>>,
    object_method: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<ObjectMethod>>, SystemTime)>>>,
    woog_option: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<WoogOption>>, SystemTime)>>>,
    ownership: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Ownership>>, SystemTime)>>>,
    parameter: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Parameter>>, SystemTime)>>>,
    reference: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Reference>>, SystemTime)>>>,
    statement: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Statement>>, SystemTime)>>>,
    structure: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Structure>>, SystemTime)>>>,
    structure_field: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<StructureField>>, SystemTime)>>>,
    symbol_table: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<SymbolTable>>, SystemTime)>>>,
    time_stamp: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<TimeStamp>>, SystemTime)>>>,
    x_value: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<XValue>>, SystemTime)>>>,
    variable: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Variable>>, SystemTime)>>>,
    visibility: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Visibility>>, SystemTime)>>>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            access: Arc::new(RwLock::new(HashMap::default())),
            block: Arc::new(RwLock::new(HashMap::default())),
            call: Arc::new(RwLock::new(HashMap::default())),
            constant: Arc::new(RwLock::new(HashMap::default())),
            enumeration: Arc::new(RwLock::new(HashMap::default())),
            enumeration_field: Arc::new(RwLock::new(HashMap::default())),
            expression: Arc::new(RwLock::new(HashMap::default())),
            field: Arc::new(RwLock::new(HashMap::default())),
            field_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            function: Arc::new(RwLock::new(HashMap::default())),
            function_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            generation_unit: Arc::new(RwLock::new(HashMap::default())),
            grace_type: Arc::new(RwLock::new(HashMap::default())),
            item: Arc::new(RwLock::new(HashMap::default())),
            x_let: Arc::new(RwLock::new(HashMap::default())),
            local: Arc::new(RwLock::new(HashMap::default())),
            object_method: Arc::new(RwLock::new(HashMap::default())),
            woog_option: Arc::new(RwLock::new(HashMap::default())),
            ownership: Arc::new(RwLock::new(HashMap::default())),
            parameter: Arc::new(RwLock::new(HashMap::default())),
            reference: Arc::new(RwLock::new(HashMap::default())),
            statement: Arc::new(RwLock::new(HashMap::default())),
            structure: Arc::new(RwLock::new(HashMap::default())),
            structure_field: Arc::new(RwLock::new(HashMap::default())),
            symbol_table: Arc::new(RwLock::new(HashMap::default())),
            time_stamp: Arc::new(RwLock::new(HashMap::default())),
            x_value: Arc::new(RwLock::new(HashMap::default())),
            variable: Arc::new(RwLock::new(HashMap::default())),
            visibility: Arc::new(RwLock::new(HashMap::default())),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥
        store.inter_expression(Arc::new(RwLock::new(Expression::Literal(LITERAL))));
        store.inter_grace_type(Arc::new(RwLock::new(GraceType::Usize(USIZE))));
        store.inter_item(Arc::new(RwLock::new(Item::Implementation(IMPLEMENTATION))));
        store.inter_ownership(Arc::new(RwLock::new(Ownership::Borrowed(BORROWED))));
        store.inter_ownership(Arc::new(RwLock::new(Ownership::Mutable(MUTABLE))));
        store.inter_ownership(Arc::new(RwLock::new(Ownership::Owned(OWNED))));
        store.inter_visibility(Arc::new(RwLock::new(Visibility::Krate(KRATE))));
        store.inter_visibility(Arc::new(RwLock::new(Visibility::Private(PRIVATE))));
        store.inter_visibility(Arc::new(RwLock::new(Visibility::Public(PUBLIC))));

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog-object-store-methods"}}}
    /// Inter (insert) [`Access`] into the store.
    ///
    pub fn inter_access(&mut self, access: Arc<RwLock<Access>>) {
        let read = access.read().unwrap();
        self.access
            .write()
            .unwrap()
            .insert(read.id, (access.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Access`] from the store.
    ///
    pub fn exhume_access(&self, id: &Uuid) -> Option<Arc<RwLock<Access>>> {
        self.access
            .read()
            .unwrap()
            .get(id)
            .map(|access| access.0.clone())
    }

    /// Exorcise (remove) [`Access`] from the store.
    ///
    pub fn exorcise_access(&mut self, id: &Uuid) -> Option<Arc<RwLock<Access>>> {
        self.access
            .write()
            .unwrap()
            .remove(id)
            .map(|access| access.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Access>`.
    ///
    pub fn iter_access(&self) -> impl Iterator<Item = Arc<RwLock<Access>>> + '_ {
        let values: Vec<Arc<RwLock<Access>>> = self
            .access
            .read()
            .unwrap()
            .values()
            .map(|access| access.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Access.
    ///
    pub fn access_timestamp(&self, access: &Access) -> SystemTime {
        self.access
            .read()
            .unwrap()
            .get(&access.id)
            .map(|access| access.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Block`] into the store.
    ///
    pub fn inter_block(&mut self, block: Arc<RwLock<Block>>) {
        let read = block.read().unwrap();
        self.block
            .write()
            .unwrap()
            .insert(read.id, (block.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Block`] from the store.
    ///
    pub fn exhume_block(&self, id: &Uuid) -> Option<Arc<RwLock<Block>>> {
        self.block
            .read()
            .unwrap()
            .get(id)
            .map(|block| block.0.clone())
    }

    /// Exorcise (remove) [`Block`] from the store.
    ///
    pub fn exorcise_block(&mut self, id: &Uuid) -> Option<Arc<RwLock<Block>>> {
        self.block
            .write()
            .unwrap()
            .remove(id)
            .map(|block| block.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Block>`.
    ///
    pub fn iter_block(&self) -> impl Iterator<Item = Arc<RwLock<Block>>> + '_ {
        let values: Vec<Arc<RwLock<Block>>> = self
            .block
            .read()
            .unwrap()
            .values()
            .map(|block| block.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Block.
    ///
    pub fn block_timestamp(&self, block: &Block) -> SystemTime {
        self.block
            .read()
            .unwrap()
            .get(&block.id)
            .map(|block| block.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Call`] into the store.
    ///
    pub fn inter_call(&mut self, call: Arc<RwLock<Call>>) {
        let read = call.read().unwrap();
        self.call
            .write()
            .unwrap()
            .insert(read.id, (call.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Call`] from the store.
    ///
    pub fn exhume_call(&self, id: &Uuid) -> Option<Arc<RwLock<Call>>> {
        self.call.read().unwrap().get(id).map(|call| call.0.clone())
    }

    /// Exorcise (remove) [`Call`] from the store.
    ///
    pub fn exorcise_call(&mut self, id: &Uuid) -> Option<Arc<RwLock<Call>>> {
        self.call
            .write()
            .unwrap()
            .remove(id)
            .map(|call| call.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Call>`.
    ///
    pub fn iter_call(&self) -> impl Iterator<Item = Arc<RwLock<Call>>> + '_ {
        let values: Vec<Arc<RwLock<Call>>> = self
            .call
            .read()
            .unwrap()
            .values()
            .map(|call| call.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Call.
    ///
    pub fn call_timestamp(&self, call: &Call) -> SystemTime {
        self.call
            .read()
            .unwrap()
            .get(&call.id)
            .map(|call| call.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Constant`] into the store.
    ///
    pub fn inter_constant(&mut self, constant: Arc<RwLock<Constant>>) {
        let read = constant.read().unwrap();
        self.constant
            .write()
            .unwrap()
            .insert(read.id, (constant.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Constant`] from the store.
    ///
    pub fn exhume_constant(&self, id: &Uuid) -> Option<Arc<RwLock<Constant>>> {
        self.constant
            .read()
            .unwrap()
            .get(id)
            .map(|constant| constant.0.clone())
    }

    /// Exorcise (remove) [`Constant`] from the store.
    ///
    pub fn exorcise_constant(&mut self, id: &Uuid) -> Option<Arc<RwLock<Constant>>> {
        self.constant
            .write()
            .unwrap()
            .remove(id)
            .map(|constant| constant.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Constant>`.
    ///
    pub fn iter_constant(&self) -> impl Iterator<Item = Arc<RwLock<Constant>>> + '_ {
        let values: Vec<Arc<RwLock<Constant>>> = self
            .constant
            .read()
            .unwrap()
            .values()
            .map(|constant| constant.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Constant.
    ///
    pub fn constant_timestamp(&self, constant: &Constant) -> SystemTime {
        self.constant
            .read()
            .unwrap()
            .get(&constant.id)
            .map(|constant| constant.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Enumeration`] into the store.
    ///
    pub fn inter_enumeration(&mut self, enumeration: Arc<RwLock<Enumeration>>) {
        let read = enumeration.read().unwrap();
        self.enumeration
            .write()
            .unwrap()
            .insert(read.id, (enumeration.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Enumeration`] from the store.
    ///
    pub fn exhume_enumeration(&self, id: &Uuid) -> Option<Arc<RwLock<Enumeration>>> {
        self.enumeration
            .read()
            .unwrap()
            .get(id)
            .map(|enumeration| enumeration.0.clone())
    }

    /// Exorcise (remove) [`Enumeration`] from the store.
    ///
    pub fn exorcise_enumeration(&mut self, id: &Uuid) -> Option<Arc<RwLock<Enumeration>>> {
        self.enumeration
            .write()
            .unwrap()
            .remove(id)
            .map(|enumeration| enumeration.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Enumeration>`.
    ///
    pub fn iter_enumeration(&self) -> impl Iterator<Item = Arc<RwLock<Enumeration>>> + '_ {
        let values: Vec<Arc<RwLock<Enumeration>>> = self
            .enumeration
            .read()
            .unwrap()
            .values()
            .map(|enumeration| enumeration.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Enumeration.
    ///
    pub fn enumeration_timestamp(&self, enumeration: &Enumeration) -> SystemTime {
        self.enumeration
            .read()
            .unwrap()
            .get(&enumeration.id)
            .map(|enumeration| enumeration.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`EnumerationField`] into the store.
    ///
    pub fn inter_enumeration_field(&mut self, enumeration_field: Arc<RwLock<EnumerationField>>) {
        let read = enumeration_field.read().unwrap();
        self.enumeration_field
            .write()
            .unwrap()
            .insert(read.id, (enumeration_field.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`EnumerationField`] from the store.
    ///
    pub fn exhume_enumeration_field(&self, id: &Uuid) -> Option<Arc<RwLock<EnumerationField>>> {
        self.enumeration_field
            .read()
            .unwrap()
            .get(id)
            .map(|enumeration_field| enumeration_field.0.clone())
    }

    /// Exorcise (remove) [`EnumerationField`] from the store.
    ///
    pub fn exorcise_enumeration_field(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<EnumerationField>>> {
        self.enumeration_field
            .write()
            .unwrap()
            .remove(id)
            .map(|enumeration_field| enumeration_field.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, EnumerationField>`.
    ///
    pub fn iter_enumeration_field(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<EnumerationField>>> + '_ {
        let values: Vec<Arc<RwLock<EnumerationField>>> = self
            .enumeration_field
            .read()
            .unwrap()
            .values()
            .map(|enumeration_field| enumeration_field.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for EnumerationField.
    ///
    pub fn enumeration_field_timestamp(&self, enumeration_field: &EnumerationField) -> SystemTime {
        self.enumeration_field
            .read()
            .unwrap()
            .get(&enumeration_field.id)
            .map(|enumeration_field| enumeration_field.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Expression`] into the store.
    ///
    pub fn inter_expression(&mut self, expression: Arc<RwLock<Expression>>) {
        let read = expression.read().unwrap();
        self.expression
            .write()
            .unwrap()
            .insert(read.id(), (expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Expression`] from the store.
    ///
    pub fn exhume_expression(&self, id: &Uuid) -> Option<Arc<RwLock<Expression>>> {
        self.expression
            .read()
            .unwrap()
            .get(id)
            .map(|expression| expression.0.clone())
    }

    /// Exorcise (remove) [`Expression`] from the store.
    ///
    pub fn exorcise_expression(&mut self, id: &Uuid) -> Option<Arc<RwLock<Expression>>> {
        self.expression
            .write()
            .unwrap()
            .remove(id)
            .map(|expression| expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Expression>`.
    ///
    pub fn iter_expression(&self) -> impl Iterator<Item = Arc<RwLock<Expression>>> + '_ {
        let values: Vec<Arc<RwLock<Expression>>> = self
            .expression
            .read()
            .unwrap()
            .values()
            .map(|expression| expression.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Expression.
    ///
    pub fn expression_timestamp(&self, expression: &Expression) -> SystemTime {
        self.expression
            .read()
            .unwrap()
            .get(&expression.id())
            .map(|expression| expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Field`] into the store.
    ///
    pub fn inter_field(&mut self, field: Arc<RwLock<Field>>) {
        let read = field.read().unwrap();
        let value = (field.clone(), SystemTime::now());
        self.field_id_by_name
            .write()
            .unwrap()
            .insert(read.name.to_upper_camel_case(), (read.id, value.1));
        self.field.write().unwrap().insert(read.id, value);
    }

    /// Exhume (get) [`Field`] from the store.
    ///
    pub fn exhume_field(&self, id: &Uuid) -> Option<Arc<RwLock<Field>>> {
        self.field
            .read()
            .unwrap()
            .get(id)
            .map(|field| field.0.clone())
    }

    /// Exorcise (remove) [`Field`] from the store.
    ///
    pub fn exorcise_field(&mut self, id: &Uuid) -> Option<Arc<RwLock<Field>>> {
        self.field
            .write()
            .unwrap()
            .remove(id)
            .map(|field| field.0.clone())
    }

    /// Exhume [`Field`] id from the store by name.
    ///
    pub fn exhume_field_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.field_id_by_name
            .read()
            .unwrap()
            .get(name)
            .map(|field| field.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Field>`.
    ///
    pub fn iter_field(&self) -> impl Iterator<Item = Arc<RwLock<Field>>> + '_ {
        let values: Vec<Arc<RwLock<Field>>> = self
            .field
            .read()
            .unwrap()
            .values()
            .map(|field| field.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Field.
    ///
    pub fn field_timestamp(&self, field: &Field) -> SystemTime {
        self.field
            .read()
            .unwrap()
            .get(&field.id)
            .map(|field| field.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Function`] into the store.
    ///
    pub fn inter_function(&mut self, function: Arc<RwLock<Function>>) {
        let read = function.read().unwrap();
        let value = (function.clone(), SystemTime::now());
        self.function_id_by_name
            .write()
            .unwrap()
            .insert(read.name.to_upper_camel_case(), (read.id, value.1));
        self.function.write().unwrap().insert(read.id, value);
    }

    /// Exhume (get) [`Function`] from the store.
    ///
    pub fn exhume_function(&self, id: &Uuid) -> Option<Arc<RwLock<Function>>> {
        self.function
            .read()
            .unwrap()
            .get(id)
            .map(|function| function.0.clone())
    }

    /// Exorcise (remove) [`Function`] from the store.
    ///
    pub fn exorcise_function(&mut self, id: &Uuid) -> Option<Arc<RwLock<Function>>> {
        self.function
            .write()
            .unwrap()
            .remove(id)
            .map(|function| function.0.clone())
    }

    /// Exhume [`Function`] id from the store by name.
    ///
    pub fn exhume_function_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.function_id_by_name
            .read()
            .unwrap()
            .get(name)
            .map(|function| function.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Function>`.
    ///
    pub fn iter_function(&self) -> impl Iterator<Item = Arc<RwLock<Function>>> + '_ {
        let values: Vec<Arc<RwLock<Function>>> = self
            .function
            .read()
            .unwrap()
            .values()
            .map(|function| function.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Function.
    ///
    pub fn function_timestamp(&self, function: &Function) -> SystemTime {
        self.function
            .read()
            .unwrap()
            .get(&function.id)
            .map(|function| function.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`GenerationUnit`] into the store.
    ///
    pub fn inter_generation_unit(&mut self, generation_unit: Arc<RwLock<GenerationUnit>>) {
        let read = generation_unit.read().unwrap();
        self.generation_unit
            .write()
            .unwrap()
            .insert(read.id, (generation_unit.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`GenerationUnit`] from the store.
    ///
    pub fn exhume_generation_unit(&self, id: &Uuid) -> Option<Arc<RwLock<GenerationUnit>>> {
        self.generation_unit
            .read()
            .unwrap()
            .get(id)
            .map(|generation_unit| generation_unit.0.clone())
    }

    /// Exorcise (remove) [`GenerationUnit`] from the store.
    ///
    pub fn exorcise_generation_unit(&mut self, id: &Uuid) -> Option<Arc<RwLock<GenerationUnit>>> {
        self.generation_unit
            .write()
            .unwrap()
            .remove(id)
            .map(|generation_unit| generation_unit.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, GenerationUnit>`.
    ///
    pub fn iter_generation_unit(&self) -> impl Iterator<Item = Arc<RwLock<GenerationUnit>>> + '_ {
        let values: Vec<Arc<RwLock<GenerationUnit>>> = self
            .generation_unit
            .read()
            .unwrap()
            .values()
            .map(|generation_unit| generation_unit.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for GenerationUnit.
    ///
    pub fn generation_unit_timestamp(&self, generation_unit: &GenerationUnit) -> SystemTime {
        self.generation_unit
            .read()
            .unwrap()
            .get(&generation_unit.id)
            .map(|generation_unit| generation_unit.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`GraceType`] into the store.
    ///
    pub fn inter_grace_type(&mut self, grace_type: Arc<RwLock<GraceType>>) {
        let read = grace_type.read().unwrap();
        self.grace_type
            .write()
            .unwrap()
            .insert(read.id(), (grace_type.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`GraceType`] from the store.
    ///
    pub fn exhume_grace_type(&self, id: &Uuid) -> Option<Arc<RwLock<GraceType>>> {
        self.grace_type
            .read()
            .unwrap()
            .get(id)
            .map(|grace_type| grace_type.0.clone())
    }

    /// Exorcise (remove) [`GraceType`] from the store.
    ///
    pub fn exorcise_grace_type(&mut self, id: &Uuid) -> Option<Arc<RwLock<GraceType>>> {
        self.grace_type
            .write()
            .unwrap()
            .remove(id)
            .map(|grace_type| grace_type.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, GraceType>`.
    ///
    pub fn iter_grace_type(&self) -> impl Iterator<Item = Arc<RwLock<GraceType>>> + '_ {
        let values: Vec<Arc<RwLock<GraceType>>> = self
            .grace_type
            .read()
            .unwrap()
            .values()
            .map(|grace_type| grace_type.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for GraceType.
    ///
    pub fn grace_type_timestamp(&self, grace_type: &GraceType) -> SystemTime {
        self.grace_type
            .read()
            .unwrap()
            .get(&grace_type.id())
            .map(|grace_type| grace_type.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Item`] into the store.
    ///
    pub fn inter_item(&mut self, item: Arc<RwLock<Item>>) {
        let read = item.read().unwrap();
        self.item
            .write()
            .unwrap()
            .insert(read.id(), (item.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Item`] from the store.
    ///
    pub fn exhume_item(&self, id: &Uuid) -> Option<Arc<RwLock<Item>>> {
        self.item.read().unwrap().get(id).map(|item| item.0.clone())
    }

    /// Exorcise (remove) [`Item`] from the store.
    ///
    pub fn exorcise_item(&mut self, id: &Uuid) -> Option<Arc<RwLock<Item>>> {
        self.item
            .write()
            .unwrap()
            .remove(id)
            .map(|item| item.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Item>`.
    ///
    pub fn iter_item(&self) -> impl Iterator<Item = Arc<RwLock<Item>>> + '_ {
        let values: Vec<Arc<RwLock<Item>>> = self
            .item
            .read()
            .unwrap()
            .values()
            .map(|item| item.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Item.
    ///
    pub fn item_timestamp(&self, item: &Item) -> SystemTime {
        self.item
            .read()
            .unwrap()
            .get(&item.id())
            .map(|item| item.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`XLet`] into the store.
    ///
    pub fn inter_x_let(&mut self, x_let: Arc<RwLock<XLet>>) {
        let read = x_let.read().unwrap();
        self.x_let
            .write()
            .unwrap()
            .insert(read.id, (x_let.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`XLet`] from the store.
    ///
    pub fn exhume_x_let(&self, id: &Uuid) -> Option<Arc<RwLock<XLet>>> {
        self.x_let
            .read()
            .unwrap()
            .get(id)
            .map(|x_let| x_let.0.clone())
    }

    /// Exorcise (remove) [`XLet`] from the store.
    ///
    pub fn exorcise_x_let(&mut self, id: &Uuid) -> Option<Arc<RwLock<XLet>>> {
        self.x_let
            .write()
            .unwrap()
            .remove(id)
            .map(|x_let| x_let.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XLet>`.
    ///
    pub fn iter_x_let(&self) -> impl Iterator<Item = Arc<RwLock<XLet>>> + '_ {
        let values: Vec<Arc<RwLock<XLet>>> = self
            .x_let
            .read()
            .unwrap()
            .values()
            .map(|x_let| x_let.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for XLet.
    ///
    pub fn x_let_timestamp(&self, x_let: &XLet) -> SystemTime {
        self.x_let
            .read()
            .unwrap()
            .get(&x_let.id)
            .map(|x_let| x_let.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Local`] into the store.
    ///
    pub fn inter_local(&mut self, local: Arc<RwLock<Local>>) {
        let read = local.read().unwrap();
        self.local
            .write()
            .unwrap()
            .insert(read.id, (local.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Local`] from the store.
    ///
    pub fn exhume_local(&self, id: &Uuid) -> Option<Arc<RwLock<Local>>> {
        self.local
            .read()
            .unwrap()
            .get(id)
            .map(|local| local.0.clone())
    }

    /// Exorcise (remove) [`Local`] from the store.
    ///
    pub fn exorcise_local(&mut self, id: &Uuid) -> Option<Arc<RwLock<Local>>> {
        self.local
            .write()
            .unwrap()
            .remove(id)
            .map(|local| local.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Local>`.
    ///
    pub fn iter_local(&self) -> impl Iterator<Item = Arc<RwLock<Local>>> + '_ {
        let values: Vec<Arc<RwLock<Local>>> = self
            .local
            .read()
            .unwrap()
            .values()
            .map(|local| local.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Local.
    ///
    pub fn local_timestamp(&self, local: &Local) -> SystemTime {
        self.local
            .read()
            .unwrap()
            .get(&local.id)
            .map(|local| local.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ObjectMethod`] into the store.
    ///
    pub fn inter_object_method(&mut self, object_method: Arc<RwLock<ObjectMethod>>) {
        let read = object_method.read().unwrap();
        self.object_method
            .write()
            .unwrap()
            .insert(read.id, (object_method.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ObjectMethod`] from the store.
    ///
    pub fn exhume_object_method(&self, id: &Uuid) -> Option<Arc<RwLock<ObjectMethod>>> {
        self.object_method
            .read()
            .unwrap()
            .get(id)
            .map(|object_method| object_method.0.clone())
    }

    /// Exorcise (remove) [`ObjectMethod`] from the store.
    ///
    pub fn exorcise_object_method(&mut self, id: &Uuid) -> Option<Arc<RwLock<ObjectMethod>>> {
        self.object_method
            .write()
            .unwrap()
            .remove(id)
            .map(|object_method| object_method.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ObjectMethod>`.
    ///
    pub fn iter_object_method(&self) -> impl Iterator<Item = Arc<RwLock<ObjectMethod>>> + '_ {
        let values: Vec<Arc<RwLock<ObjectMethod>>> = self
            .object_method
            .read()
            .unwrap()
            .values()
            .map(|object_method| object_method.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for ObjectMethod.
    ///
    pub fn object_method_timestamp(&self, object_method: &ObjectMethod) -> SystemTime {
        self.object_method
            .read()
            .unwrap()
            .get(&object_method.id)
            .map(|object_method| object_method.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`WoogOption`] into the store.
    ///
    pub fn inter_woog_option(&mut self, woog_option: Arc<RwLock<WoogOption>>) {
        let read = woog_option.read().unwrap();
        self.woog_option
            .write()
            .unwrap()
            .insert(read.id, (woog_option.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`WoogOption`] from the store.
    ///
    pub fn exhume_woog_option(&self, id: &Uuid) -> Option<Arc<RwLock<WoogOption>>> {
        self.woog_option
            .read()
            .unwrap()
            .get(id)
            .map(|woog_option| woog_option.0.clone())
    }

    /// Exorcise (remove) [`WoogOption`] from the store.
    ///
    pub fn exorcise_woog_option(&mut self, id: &Uuid) -> Option<Arc<RwLock<WoogOption>>> {
        self.woog_option
            .write()
            .unwrap()
            .remove(id)
            .map(|woog_option| woog_option.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, WoogOption>`.
    ///
    pub fn iter_woog_option(&self) -> impl Iterator<Item = Arc<RwLock<WoogOption>>> + '_ {
        let values: Vec<Arc<RwLock<WoogOption>>> = self
            .woog_option
            .read()
            .unwrap()
            .values()
            .map(|woog_option| woog_option.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for WoogOption.
    ///
    pub fn woog_option_timestamp(&self, woog_option: &WoogOption) -> SystemTime {
        self.woog_option
            .read()
            .unwrap()
            .get(&woog_option.id)
            .map(|woog_option| woog_option.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Ownership`] into the store.
    ///
    pub fn inter_ownership(&mut self, ownership: Arc<RwLock<Ownership>>) {
        let read = ownership.read().unwrap();
        self.ownership
            .write()
            .unwrap()
            .insert(read.id(), (ownership.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Ownership`] from the store.
    ///
    pub fn exhume_ownership(&self, id: &Uuid) -> Option<Arc<RwLock<Ownership>>> {
        self.ownership
            .read()
            .unwrap()
            .get(id)
            .map(|ownership| ownership.0.clone())
    }

    /// Exorcise (remove) [`Ownership`] from the store.
    ///
    pub fn exorcise_ownership(&mut self, id: &Uuid) -> Option<Arc<RwLock<Ownership>>> {
        self.ownership
            .write()
            .unwrap()
            .remove(id)
            .map(|ownership| ownership.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Ownership>`.
    ///
    pub fn iter_ownership(&self) -> impl Iterator<Item = Arc<RwLock<Ownership>>> + '_ {
        let values: Vec<Arc<RwLock<Ownership>>> = self
            .ownership
            .read()
            .unwrap()
            .values()
            .map(|ownership| ownership.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Ownership.
    ///
    pub fn ownership_timestamp(&self, ownership: &Ownership) -> SystemTime {
        self.ownership
            .read()
            .unwrap()
            .get(&ownership.id())
            .map(|ownership| ownership.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Parameter`] into the store.
    ///
    pub fn inter_parameter(&mut self, parameter: Arc<RwLock<Parameter>>) {
        let read = parameter.read().unwrap();
        self.parameter
            .write()
            .unwrap()
            .insert(read.id, (parameter.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Parameter`] from the store.
    ///
    pub fn exhume_parameter(&self, id: &Uuid) -> Option<Arc<RwLock<Parameter>>> {
        self.parameter
            .read()
            .unwrap()
            .get(id)
            .map(|parameter| parameter.0.clone())
    }

    /// Exorcise (remove) [`Parameter`] from the store.
    ///
    pub fn exorcise_parameter(&mut self, id: &Uuid) -> Option<Arc<RwLock<Parameter>>> {
        self.parameter
            .write()
            .unwrap()
            .remove(id)
            .map(|parameter| parameter.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Parameter>`.
    ///
    pub fn iter_parameter(&self) -> impl Iterator<Item = Arc<RwLock<Parameter>>> + '_ {
        let values: Vec<Arc<RwLock<Parameter>>> = self
            .parameter
            .read()
            .unwrap()
            .values()
            .map(|parameter| parameter.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Parameter.
    ///
    pub fn parameter_timestamp(&self, parameter: &Parameter) -> SystemTime {
        self.parameter
            .read()
            .unwrap()
            .get(&parameter.id)
            .map(|parameter| parameter.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Reference`] into the store.
    ///
    pub fn inter_reference(&mut self, reference: Arc<RwLock<Reference>>) {
        let read = reference.read().unwrap();
        self.reference
            .write()
            .unwrap()
            .insert(read.id, (reference.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Reference`] from the store.
    ///
    pub fn exhume_reference(&self, id: &Uuid) -> Option<Arc<RwLock<Reference>>> {
        self.reference
            .read()
            .unwrap()
            .get(id)
            .map(|reference| reference.0.clone())
    }

    /// Exorcise (remove) [`Reference`] from the store.
    ///
    pub fn exorcise_reference(&mut self, id: &Uuid) -> Option<Arc<RwLock<Reference>>> {
        self.reference
            .write()
            .unwrap()
            .remove(id)
            .map(|reference| reference.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Reference>`.
    ///
    pub fn iter_reference(&self) -> impl Iterator<Item = Arc<RwLock<Reference>>> + '_ {
        let values: Vec<Arc<RwLock<Reference>>> = self
            .reference
            .read()
            .unwrap()
            .values()
            .map(|reference| reference.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Reference.
    ///
    pub fn reference_timestamp(&self, reference: &Reference) -> SystemTime {
        self.reference
            .read()
            .unwrap()
            .get(&reference.id)
            .map(|reference| reference.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Statement`] into the store.
    ///
    pub fn inter_statement(&mut self, statement: Arc<RwLock<Statement>>) {
        let read = statement.read().unwrap();
        self.statement
            .write()
            .unwrap()
            .insert(read.id, (statement.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Statement`] from the store.
    ///
    pub fn exhume_statement(&self, id: &Uuid) -> Option<Arc<RwLock<Statement>>> {
        self.statement
            .read()
            .unwrap()
            .get(id)
            .map(|statement| statement.0.clone())
    }

    /// Exorcise (remove) [`Statement`] from the store.
    ///
    pub fn exorcise_statement(&mut self, id: &Uuid) -> Option<Arc<RwLock<Statement>>> {
        self.statement
            .write()
            .unwrap()
            .remove(id)
            .map(|statement| statement.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Statement>`.
    ///
    pub fn iter_statement(&self) -> impl Iterator<Item = Arc<RwLock<Statement>>> + '_ {
        let values: Vec<Arc<RwLock<Statement>>> = self
            .statement
            .read()
            .unwrap()
            .values()
            .map(|statement| statement.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Statement.
    ///
    pub fn statement_timestamp(&self, statement: &Statement) -> SystemTime {
        self.statement
            .read()
            .unwrap()
            .get(&statement.id)
            .map(|statement| statement.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Structure`] into the store.
    ///
    pub fn inter_structure(&mut self, structure: Arc<RwLock<Structure>>) {
        let read = structure.read().unwrap();
        self.structure
            .write()
            .unwrap()
            .insert(read.id, (structure.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Structure`] from the store.
    ///
    pub fn exhume_structure(&self, id: &Uuid) -> Option<Arc<RwLock<Structure>>> {
        self.structure
            .read()
            .unwrap()
            .get(id)
            .map(|structure| structure.0.clone())
    }

    /// Exorcise (remove) [`Structure`] from the store.
    ///
    pub fn exorcise_structure(&mut self, id: &Uuid) -> Option<Arc<RwLock<Structure>>> {
        self.structure
            .write()
            .unwrap()
            .remove(id)
            .map(|structure| structure.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Structure>`.
    ///
    pub fn iter_structure(&self) -> impl Iterator<Item = Arc<RwLock<Structure>>> + '_ {
        let values: Vec<Arc<RwLock<Structure>>> = self
            .structure
            .read()
            .unwrap()
            .values()
            .map(|structure| structure.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Structure.
    ///
    pub fn structure_timestamp(&self, structure: &Structure) -> SystemTime {
        self.structure
            .read()
            .unwrap()
            .get(&structure.id)
            .map(|structure| structure.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`StructureField`] into the store.
    ///
    pub fn inter_structure_field(&mut self, structure_field: Arc<RwLock<StructureField>>) {
        let read = structure_field.read().unwrap();
        self.structure_field
            .write()
            .unwrap()
            .insert(read.id, (structure_field.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`StructureField`] from the store.
    ///
    pub fn exhume_structure_field(&self, id: &Uuid) -> Option<Arc<RwLock<StructureField>>> {
        self.structure_field
            .read()
            .unwrap()
            .get(id)
            .map(|structure_field| structure_field.0.clone())
    }

    /// Exorcise (remove) [`StructureField`] from the store.
    ///
    pub fn exorcise_structure_field(&mut self, id: &Uuid) -> Option<Arc<RwLock<StructureField>>> {
        self.structure_field
            .write()
            .unwrap()
            .remove(id)
            .map(|structure_field| structure_field.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructureField>`.
    ///
    pub fn iter_structure_field(&self) -> impl Iterator<Item = Arc<RwLock<StructureField>>> + '_ {
        let values: Vec<Arc<RwLock<StructureField>>> = self
            .structure_field
            .read()
            .unwrap()
            .values()
            .map(|structure_field| structure_field.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for StructureField.
    ///
    pub fn structure_field_timestamp(&self, structure_field: &StructureField) -> SystemTime {
        self.structure_field
            .read()
            .unwrap()
            .get(&structure_field.id)
            .map(|structure_field| structure_field.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`SymbolTable`] into the store.
    ///
    pub fn inter_symbol_table(&mut self, symbol_table: Arc<RwLock<SymbolTable>>) {
        let read = symbol_table.read().unwrap();
        self.symbol_table
            .write()
            .unwrap()
            .insert(read.id, (symbol_table.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`SymbolTable`] from the store.
    ///
    pub fn exhume_symbol_table(&self, id: &Uuid) -> Option<Arc<RwLock<SymbolTable>>> {
        self.symbol_table
            .read()
            .unwrap()
            .get(id)
            .map(|symbol_table| symbol_table.0.clone())
    }

    /// Exorcise (remove) [`SymbolTable`] from the store.
    ///
    pub fn exorcise_symbol_table(&mut self, id: &Uuid) -> Option<Arc<RwLock<SymbolTable>>> {
        self.symbol_table
            .write()
            .unwrap()
            .remove(id)
            .map(|symbol_table| symbol_table.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SymbolTable>`.
    ///
    pub fn iter_symbol_table(&self) -> impl Iterator<Item = Arc<RwLock<SymbolTable>>> + '_ {
        let values: Vec<Arc<RwLock<SymbolTable>>> = self
            .symbol_table
            .read()
            .unwrap()
            .values()
            .map(|symbol_table| symbol_table.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for SymbolTable.
    ///
    pub fn symbol_table_timestamp(&self, symbol_table: &SymbolTable) -> SystemTime {
        self.symbol_table
            .read()
            .unwrap()
            .get(&symbol_table.id)
            .map(|symbol_table| symbol_table.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`TimeStamp`] into the store.
    ///
    pub fn inter_time_stamp(&mut self, time_stamp: Arc<RwLock<TimeStamp>>) {
        let read = time_stamp.read().unwrap();
        self.time_stamp
            .write()
            .unwrap()
            .insert(read.id, (time_stamp.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`TimeStamp`] from the store.
    ///
    pub fn exhume_time_stamp(&self, id: &Uuid) -> Option<Arc<RwLock<TimeStamp>>> {
        self.time_stamp
            .read()
            .unwrap()
            .get(id)
            .map(|time_stamp| time_stamp.0.clone())
    }

    /// Exorcise (remove) [`TimeStamp`] from the store.
    ///
    pub fn exorcise_time_stamp(&mut self, id: &Uuid) -> Option<Arc<RwLock<TimeStamp>>> {
        self.time_stamp
            .write()
            .unwrap()
            .remove(id)
            .map(|time_stamp| time_stamp.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, TimeStamp>`.
    ///
    pub fn iter_time_stamp(&self) -> impl Iterator<Item = Arc<RwLock<TimeStamp>>> + '_ {
        let values: Vec<Arc<RwLock<TimeStamp>>> = self
            .time_stamp
            .read()
            .unwrap()
            .values()
            .map(|time_stamp| time_stamp.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for TimeStamp.
    ///
    pub fn time_stamp_timestamp(&self, time_stamp: &TimeStamp) -> SystemTime {
        self.time_stamp
            .read()
            .unwrap()
            .get(&time_stamp.id)
            .map(|time_stamp| time_stamp.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`XValue`] into the store.
    ///
    pub fn inter_x_value(&mut self, x_value: Arc<RwLock<XValue>>) {
        let read = x_value.read().unwrap();
        self.x_value
            .write()
            .unwrap()
            .insert(read.id, (x_value.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`XValue`] from the store.
    ///
    pub fn exhume_x_value(&self, id: &Uuid) -> Option<Arc<RwLock<XValue>>> {
        self.x_value
            .read()
            .unwrap()
            .get(id)
            .map(|x_value| x_value.0.clone())
    }

    /// Exorcise (remove) [`XValue`] from the store.
    ///
    pub fn exorcise_x_value(&mut self, id: &Uuid) -> Option<Arc<RwLock<XValue>>> {
        self.x_value
            .write()
            .unwrap()
            .remove(id)
            .map(|x_value| x_value.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XValue>`.
    ///
    pub fn iter_x_value(&self) -> impl Iterator<Item = Arc<RwLock<XValue>>> + '_ {
        let values: Vec<Arc<RwLock<XValue>>> = self
            .x_value
            .read()
            .unwrap()
            .values()
            .map(|x_value| x_value.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for XValue.
    ///
    pub fn x_value_timestamp(&self, x_value: &XValue) -> SystemTime {
        self.x_value
            .read()
            .unwrap()
            .get(&x_value.id)
            .map(|x_value| x_value.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Variable`] into the store.
    ///
    pub fn inter_variable(&mut self, variable: Arc<RwLock<Variable>>) {
        let read = variable.read().unwrap();
        self.variable
            .write()
            .unwrap()
            .insert(read.id, (variable.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Variable`] from the store.
    ///
    pub fn exhume_variable(&self, id: &Uuid) -> Option<Arc<RwLock<Variable>>> {
        self.variable
            .read()
            .unwrap()
            .get(id)
            .map(|variable| variable.0.clone())
    }

    /// Exorcise (remove) [`Variable`] from the store.
    ///
    pub fn exorcise_variable(&mut self, id: &Uuid) -> Option<Arc<RwLock<Variable>>> {
        self.variable
            .write()
            .unwrap()
            .remove(id)
            .map(|variable| variable.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Variable>`.
    ///
    pub fn iter_variable(&self) -> impl Iterator<Item = Arc<RwLock<Variable>>> + '_ {
        let values: Vec<Arc<RwLock<Variable>>> = self
            .variable
            .read()
            .unwrap()
            .values()
            .map(|variable| variable.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Variable.
    ///
    pub fn variable_timestamp(&self, variable: &Variable) -> SystemTime {
        self.variable
            .read()
            .unwrap()
            .get(&variable.id)
            .map(|variable| variable.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Visibility`] into the store.
    ///
    pub fn inter_visibility(&mut self, visibility: Arc<RwLock<Visibility>>) {
        let read = visibility.read().unwrap();
        self.visibility
            .write()
            .unwrap()
            .insert(read.id(), (visibility.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Visibility`] from the store.
    ///
    pub fn exhume_visibility(&self, id: &Uuid) -> Option<Arc<RwLock<Visibility>>> {
        self.visibility
            .read()
            .unwrap()
            .get(id)
            .map(|visibility| visibility.0.clone())
    }

    /// Exorcise (remove) [`Visibility`] from the store.
    ///
    pub fn exorcise_visibility(&mut self, id: &Uuid) -> Option<Arc<RwLock<Visibility>>> {
        self.visibility
            .write()
            .unwrap()
            .remove(id)
            .map(|visibility| visibility.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Visibility>`.
    ///
    pub fn iter_visibility(&self) -> impl Iterator<Item = Arc<RwLock<Visibility>>> + '_ {
        let values: Vec<Arc<RwLock<Visibility>>> = self
            .visibility
            .read()
            .unwrap()
            .values()
            .map(|visibility| visibility.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Visibility.
    ///
    pub fn visibility_timestamp(&self, visibility: &Visibility) -> SystemTime {
        self.visibility
            .read()
            .unwrap()
            .get(&visibility.id())
            .map(|visibility| visibility.1)
            .unwrap_or(SystemTime::now())
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog-object-store-persistence"}}}
    /// Persist the store.
    ///
    /// The store is persisted as a a bincode file.
    pub fn persist_bincode<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        let mut bin_file = fs::File::create(path)?;
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        bin_file.write_all(&encoded)?;
        Ok(())
    }

    /// Persist the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automagic git integration as an option.
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        fs::create_dir_all(path)?;

        let path = path.join("woog.json");
        fs::create_dir_all(&path)?;

        // Persist Access.
        {
            let path = path.join("access");
            fs::create_dir_all(&path)?;
            for access_tuple in self.access.read().unwrap().values() {
                let path = path.join(format!("{}.json", access_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Access>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != access_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &access_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &access_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.access.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Block.
        {
            let path = path.join("block");
            fs::create_dir_all(&path)?;
            for block_tuple in self.block.read().unwrap().values() {
                let path = path.join(format!("{}.json", block_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Block>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != block_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &block_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &block_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.block.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Call.
        {
            let path = path.join("call");
            fs::create_dir_all(&path)?;
            for call_tuple in self.call.read().unwrap().values() {
                let path = path.join(format!("{}.json", call_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Call>>, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != call_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &call_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &call_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.call.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Constant.
        {
            let path = path.join("constant");
            fs::create_dir_all(&path)?;
            for constant_tuple in self.constant.read().unwrap().values() {
                let path = path.join(format!("{}.json", constant_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Constant>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != constant_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &constant_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &constant_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.constant.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Enumeration.
        {
            let path = path.join("enumeration");
            fs::create_dir_all(&path)?;
            for enumeration_tuple in self.enumeration.read().unwrap().values() {
                let path = path.join(format!("{}.json", enumeration_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Enumeration>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != enumeration_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &enumeration_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &enumeration_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.enumeration.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Enumeration Field.
        {
            let path = path.join("enumeration_field");
            fs::create_dir_all(&path)?;
            for enumeration_field_tuple in self.enumeration_field.read().unwrap().values() {
                let path = path.join(format!(
                    "{}.json",
                    enumeration_field_tuple.0.read().unwrap().id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<EnumerationField>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != enumeration_field_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &enumeration_field_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &enumeration_field_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.enumeration_field.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Expression.
        {
            let path = path.join("expression");
            fs::create_dir_all(&path)?;
            for expression_tuple in self.expression.read().unwrap().values() {
                let path = path.join(format!("{}.json", expression_tuple.0.read().unwrap().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Expression>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != expression_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &expression_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &expression_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.expression.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Field.
        {
            let path = path.join("field");
            fs::create_dir_all(&path)?;
            for field_tuple in self.field.read().unwrap().values() {
                let path = path.join(format!("{}.json", field_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Field>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != field_tuple.0.read().unwrap().to_owned()
                    {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.field.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Function.
        {
            let path = path.join("function");
            fs::create_dir_all(&path)?;
            for function_tuple in self.function.read().unwrap().values() {
                let path = path.join(format!("{}.json", function_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Function>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != function_tuple.0.read().unwrap().to_owned()
                    {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.function.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Generation Unit.
        {
            let path = path.join("generation_unit");
            fs::create_dir_all(&path)?;
            for generation_unit_tuple in self.generation_unit.read().unwrap().values() {
                let path = path.join(format!(
                    "{}.json",
                    generation_unit_tuple.0.read().unwrap().id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<GenerationUnit>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != generation_unit_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &generation_unit_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &generation_unit_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.generation_unit.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Grace Type.
        {
            let path = path.join("grace_type");
            fs::create_dir_all(&path)?;
            for grace_type_tuple in self.grace_type.read().unwrap().values() {
                let path = path.join(format!("{}.json", grace_type_tuple.0.read().unwrap().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<GraceType>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != grace_type_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &grace_type_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &grace_type_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.grace_type.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Item.
        {
            let path = path.join("item");
            fs::create_dir_all(&path)?;
            for item_tuple in self.item.read().unwrap().values() {
                let path = path.join(format!("{}.json", item_tuple.0.read().unwrap().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Item>>, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != item_tuple.0.read().unwrap().to_owned()
                    {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.item.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Let.
        {
            let path = path.join("x_let");
            fs::create_dir_all(&path)?;
            for x_let_tuple in self.x_let.read().unwrap().values() {
                let path = path.join(format!("{}.json", x_let_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<XLet>>, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != x_let_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &x_let_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &x_let_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.x_let.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Local.
        {
            let path = path.join("local");
            fs::create_dir_all(&path)?;
            for local_tuple in self.local.read().unwrap().values() {
                let path = path.join(format!("{}.json", local_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Local>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != local_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &local_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &local_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.local.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Object Method.
        {
            let path = path.join("object_method");
            fs::create_dir_all(&path)?;
            for object_method_tuple in self.object_method.read().unwrap().values() {
                let path = path.join(format!("{}.json", object_method_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<ObjectMethod>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != object_method_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &object_method_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &object_method_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.object_method.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Option.
        {
            let path = path.join("woog_option");
            fs::create_dir_all(&path)?;
            for woog_option_tuple in self.woog_option.read().unwrap().values() {
                let path = path.join(format!("{}.json", woog_option_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<WoogOption>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != woog_option_tuple.0.read().unwrap().to_owned()
                    {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.woog_option.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Ownership.
        {
            let path = path.join("ownership");
            fs::create_dir_all(&path)?;
            for ownership_tuple in self.ownership.read().unwrap().values() {
                let path = path.join(format!("{}.json", ownership_tuple.0.read().unwrap().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Ownership>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != ownership_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &ownership_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &ownership_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.ownership.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Parameter.
        {
            let path = path.join("parameter");
            fs::create_dir_all(&path)?;
            for parameter_tuple in self.parameter.read().unwrap().values() {
                let path = path.join(format!("{}.json", parameter_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Parameter>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != parameter_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &parameter_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &parameter_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.parameter.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Reference.
        {
            let path = path.join("reference");
            fs::create_dir_all(&path)?;
            for reference_tuple in self.reference.read().unwrap().values() {
                let path = path.join(format!("{}.json", reference_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Reference>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != reference_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &reference_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &reference_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.reference.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Statement.
        {
            let path = path.join("statement");
            fs::create_dir_all(&path)?;
            for statement_tuple in self.statement.read().unwrap().values() {
                let path = path.join(format!("{}.json", statement_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Statement>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != statement_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &statement_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &statement_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.statement.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Structure.
        {
            let path = path.join("structure");
            fs::create_dir_all(&path)?;
            for structure_tuple in self.structure.read().unwrap().values() {
                let path = path.join(format!("{}.json", structure_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Structure>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != structure_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &structure_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &structure_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.structure.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Structure Field.
        {
            let path = path.join("structure_field");
            fs::create_dir_all(&path)?;
            for structure_field_tuple in self.structure_field.read().unwrap().values() {
                let path = path.join(format!(
                    "{}.json",
                    structure_field_tuple.0.read().unwrap().id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<StructureField>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != structure_field_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &structure_field_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &structure_field_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.structure_field.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Symbol Table.
        {
            let path = path.join("symbol_table");
            fs::create_dir_all(&path)?;
            for symbol_table_tuple in self.symbol_table.read().unwrap().values() {
                let path = path.join(format!("{}.json", symbol_table_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<SymbolTable>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != symbol_table_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &symbol_table_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &symbol_table_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.symbol_table.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Time Stamp.
        {
            let path = path.join("time_stamp");
            fs::create_dir_all(&path)?;
            for time_stamp_tuple in self.time_stamp.read().unwrap().values() {
                let path = path.join(format!("{}.json", time_stamp_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<TimeStamp>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != time_stamp_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &time_stamp_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &time_stamp_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.time_stamp.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Value.
        {
            let path = path.join("x_value");
            fs::create_dir_all(&path)?;
            for x_value_tuple in self.x_value.read().unwrap().values() {
                let path = path.join(format!("{}.json", x_value_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<XValue>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != x_value_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &x_value_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &x_value_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.x_value.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Variable.
        {
            let path = path.join("variable");
            fs::create_dir_all(&path)?;
            for variable_tuple in self.variable.read().unwrap().values() {
                let path = path.join(format!("{}.json", variable_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Variable>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != variable_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &variable_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &variable_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.variable.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Visibility.
        {
            let path = path.join("visibility");
            fs::create_dir_all(&path)?;
            for visibility_tuple in self.visibility.read().unwrap().values() {
                let path = path.join(format!("{}.json", visibility_tuple.0.read().unwrap().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Visibility>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != visibility_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &visibility_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &visibility_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.visibility.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Load the store.
    ///
    pub fn from_bincode(code: &[u8]) -> io::Result<Self> {
        Ok(bincode::deserialize(code).unwrap())
    }

    /// The store is as a bincode file.
    pub fn load_bincode<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let bin_file = fs::File::open(path)?;
        Ok(bincode::deserialize_from(bin_file).unwrap())
    }

    /// Load the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automagic git integration as an option.
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let path = path.join("woog.json");

        let store = Self::new();

        // Load Access.
        {
            let path = path.join("access");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let access: (Arc<RwLock<Access>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .access
                    .write()
                    .unwrap()
                    .insert(access.0.read().unwrap().id, access.clone());
            }
        }

        // Load Block.
        {
            let path = path.join("block");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let block: (Arc<RwLock<Block>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .block
                    .write()
                    .unwrap()
                    .insert(block.0.read().unwrap().id, block.clone());
            }
        }

        // Load Call.
        {
            let path = path.join("call");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let call: (Arc<RwLock<Call>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .call
                    .write()
                    .unwrap()
                    .insert(call.0.read().unwrap().id, call.clone());
            }
        }

        // Load Constant.
        {
            let path = path.join("constant");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let constant: (Arc<RwLock<Constant>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .constant
                    .write()
                    .unwrap()
                    .insert(constant.0.read().unwrap().id, constant.clone());
            }
        }

        // Load Enumeration.
        {
            let path = path.join("enumeration");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let enumeration: (Arc<RwLock<Enumeration>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .enumeration
                    .write()
                    .unwrap()
                    .insert(enumeration.0.read().unwrap().id, enumeration.clone());
            }
        }

        // Load Enumeration Field.
        {
            let path = path.join("enumeration_field");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let enumeration_field: (Arc<RwLock<EnumerationField>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.enumeration_field.write().unwrap().insert(
                    enumeration_field.0.read().unwrap().id,
                    enumeration_field.clone(),
                );
            }
        }

        // Load Expression.
        {
            let path = path.join("expression");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let expression: (Arc<RwLock<Expression>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .expression
                    .write()
                    .unwrap()
                    .insert(expression.0.read().unwrap().id(), expression.clone());
            }
        }

        // Load Field.
        {
            let path = path.join("field");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let field: (Arc<RwLock<Field>>, SystemTime) = serde_json::from_reader(reader)?;
                store.field_id_by_name.write().unwrap().insert(
                    field.0.read().unwrap().name.to_upper_camel_case(),
                    (field.0.read().unwrap().id, field.1),
                );
                store
                    .field
                    .write()
                    .unwrap()
                    .insert(field.0.read().unwrap().id, field.clone());
            }
        }

        // Load Function.
        {
            let path = path.join("function");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let function: (Arc<RwLock<Function>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.function_id_by_name.write().unwrap().insert(
                    function.0.read().unwrap().name.to_upper_camel_case(),
                    (function.0.read().unwrap().id, function.1),
                );
                store
                    .function
                    .write()
                    .unwrap()
                    .insert(function.0.read().unwrap().id, function.clone());
            }
        }

        // Load Generation Unit.
        {
            let path = path.join("generation_unit");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let generation_unit: (Arc<RwLock<GenerationUnit>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.generation_unit.write().unwrap().insert(
                    generation_unit.0.read().unwrap().id,
                    generation_unit.clone(),
                );
            }
        }

        // Load Grace Type.
        {
            let path = path.join("grace_type");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let grace_type: (Arc<RwLock<GraceType>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .grace_type
                    .write()
                    .unwrap()
                    .insert(grace_type.0.read().unwrap().id(), grace_type.clone());
            }
        }

        // Load Item.
        {
            let path = path.join("item");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let item: (Arc<RwLock<Item>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .item
                    .write()
                    .unwrap()
                    .insert(item.0.read().unwrap().id(), item.clone());
            }
        }

        // Load Let.
        {
            let path = path.join("x_let");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let x_let: (Arc<RwLock<XLet>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .x_let
                    .write()
                    .unwrap()
                    .insert(x_let.0.read().unwrap().id, x_let.clone());
            }
        }

        // Load Local.
        {
            let path = path.join("local");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let local: (Arc<RwLock<Local>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .local
                    .write()
                    .unwrap()
                    .insert(local.0.read().unwrap().id, local.clone());
            }
        }

        // Load Object Method.
        {
            let path = path.join("object_method");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let object_method: (Arc<RwLock<ObjectMethod>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .object_method
                    .write()
                    .unwrap()
                    .insert(object_method.0.read().unwrap().id, object_method.clone());
            }
        }

        // Load Option.
        {
            let path = path.join("woog_option");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let woog_option: (Arc<RwLock<WoogOption>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .woog_option
                    .write()
                    .unwrap()
                    .insert(woog_option.0.read().unwrap().id, woog_option.clone());
            }
        }

        // Load Ownership.
        {
            let path = path.join("ownership");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let ownership: (Arc<RwLock<Ownership>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .ownership
                    .write()
                    .unwrap()
                    .insert(ownership.0.read().unwrap().id(), ownership.clone());
            }
        }

        // Load Parameter.
        {
            let path = path.join("parameter");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let parameter: (Arc<RwLock<Parameter>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .parameter
                    .write()
                    .unwrap()
                    .insert(parameter.0.read().unwrap().id, parameter.clone());
            }
        }

        // Load Reference.
        {
            let path = path.join("reference");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let reference: (Arc<RwLock<Reference>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .reference
                    .write()
                    .unwrap()
                    .insert(reference.0.read().unwrap().id, reference.clone());
            }
        }

        // Load Statement.
        {
            let path = path.join("statement");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let statement: (Arc<RwLock<Statement>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .statement
                    .write()
                    .unwrap()
                    .insert(statement.0.read().unwrap().id, statement.clone());
            }
        }

        // Load Structure.
        {
            let path = path.join("structure");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let structure: (Arc<RwLock<Structure>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .structure
                    .write()
                    .unwrap()
                    .insert(structure.0.read().unwrap().id, structure.clone());
            }
        }

        // Load Structure Field.
        {
            let path = path.join("structure_field");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let structure_field: (Arc<RwLock<StructureField>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.structure_field.write().unwrap().insert(
                    structure_field.0.read().unwrap().id,
                    structure_field.clone(),
                );
            }
        }

        // Load Symbol Table.
        {
            let path = path.join("symbol_table");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let symbol_table: (Arc<RwLock<SymbolTable>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .symbol_table
                    .write()
                    .unwrap()
                    .insert(symbol_table.0.read().unwrap().id, symbol_table.clone());
            }
        }

        // Load Time Stamp.
        {
            let path = path.join("time_stamp");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let time_stamp: (Arc<RwLock<TimeStamp>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .time_stamp
                    .write()
                    .unwrap()
                    .insert(time_stamp.0.read().unwrap().id, time_stamp.clone());
            }
        }

        // Load Value.
        {
            let path = path.join("x_value");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let x_value: (Arc<RwLock<XValue>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .x_value
                    .write()
                    .unwrap()
                    .insert(x_value.0.read().unwrap().id, x_value.clone());
            }
        }

        // Load Variable.
        {
            let path = path.join("variable");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let variable: (Arc<RwLock<Variable>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .variable
                    .write()
                    .unwrap()
                    .insert(variable.0.read().unwrap().id, variable.clone());
            }
        }

        // Load Visibility.
        {
            let path = path.join("visibility");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let visibility: (Arc<RwLock<Visibility>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .visibility
                    .write()
                    .unwrap()
                    .insert(visibility.0.read().unwrap().id(), visibility.clone());
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
