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
use std::cell::RefCell;
use std::rc::Rc;
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
    access: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Access>>, SystemTime)>>>,
    block: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Block>>, SystemTime)>>>,
    call: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Call>>, SystemTime)>>>,
    constant: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Constant>>, SystemTime)>>>,
    enumeration: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Enumeration>>, SystemTime)>>>,
    enumeration_field: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<EnumerationField>>, SystemTime)>>>,
    expression: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Expression>>, SystemTime)>>>,
    field: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Field>>, SystemTime)>>>,
    field_id_by_name: Rc<RefCell<HashMap<String, (Uuid, SystemTime)>>>,
    function: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Function>>, SystemTime)>>>,
    function_id_by_name: Rc<RefCell<HashMap<String, (Uuid, SystemTime)>>>,
    generation_unit: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<GenerationUnit>>, SystemTime)>>>,
    grace_type: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<GraceType>>, SystemTime)>>>,
    item: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Item>>, SystemTime)>>>,
    x_let: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<XLet>>, SystemTime)>>>,
    local: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Local>>, SystemTime)>>>,
    object_method: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<ObjectMethod>>, SystemTime)>>>,
    woog_option: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<WoogOption>>, SystemTime)>>>,
    ownership: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Ownership>>, SystemTime)>>>,
    parameter: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Parameter>>, SystemTime)>>>,
    reference: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Reference>>, SystemTime)>>>,
    statement: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Statement>>, SystemTime)>>>,
    structure: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Structure>>, SystemTime)>>>,
    structure_field: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<StructureField>>, SystemTime)>>>,
    symbol_table: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<SymbolTable>>, SystemTime)>>>,
    time_stamp: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<TimeStamp>>, SystemTime)>>>,
    x_value: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<XValue>>, SystemTime)>>>,
    variable: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Variable>>, SystemTime)>>>,
    visibility: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Visibility>>, SystemTime)>>>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            access: Rc::new(RefCell::new(HashMap::default())),
            block: Rc::new(RefCell::new(HashMap::default())),
            call: Rc::new(RefCell::new(HashMap::default())),
            constant: Rc::new(RefCell::new(HashMap::default())),
            enumeration: Rc::new(RefCell::new(HashMap::default())),
            enumeration_field: Rc::new(RefCell::new(HashMap::default())),
            expression: Rc::new(RefCell::new(HashMap::default())),
            field: Rc::new(RefCell::new(HashMap::default())),
            field_id_by_name: Rc::new(RefCell::new(HashMap::default())),
            function: Rc::new(RefCell::new(HashMap::default())),
            function_id_by_name: Rc::new(RefCell::new(HashMap::default())),
            generation_unit: Rc::new(RefCell::new(HashMap::default())),
            grace_type: Rc::new(RefCell::new(HashMap::default())),
            item: Rc::new(RefCell::new(HashMap::default())),
            x_let: Rc::new(RefCell::new(HashMap::default())),
            local: Rc::new(RefCell::new(HashMap::default())),
            object_method: Rc::new(RefCell::new(HashMap::default())),
            woog_option: Rc::new(RefCell::new(HashMap::default())),
            ownership: Rc::new(RefCell::new(HashMap::default())),
            parameter: Rc::new(RefCell::new(HashMap::default())),
            reference: Rc::new(RefCell::new(HashMap::default())),
            statement: Rc::new(RefCell::new(HashMap::default())),
            structure: Rc::new(RefCell::new(HashMap::default())),
            structure_field: Rc::new(RefCell::new(HashMap::default())),
            symbol_table: Rc::new(RefCell::new(HashMap::default())),
            time_stamp: Rc::new(RefCell::new(HashMap::default())),
            x_value: Rc::new(RefCell::new(HashMap::default())),
            variable: Rc::new(RefCell::new(HashMap::default())),
            visibility: Rc::new(RefCell::new(HashMap::default())),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥
        store.inter_expression(Rc::new(RefCell::new(Expression::Literal(LITERAL))));
        store.inter_grace_type(Rc::new(RefCell::new(GraceType::Usize(USIZE))));
        store.inter_item(Rc::new(RefCell::new(Item::Implementation(IMPLEMENTATION))));
        store.inter_ownership(Rc::new(RefCell::new(Ownership::Borrowed(BORROWED))));
        store.inter_ownership(Rc::new(RefCell::new(Ownership::Mutable(MUTABLE))));
        store.inter_ownership(Rc::new(RefCell::new(Ownership::Owned(OWNED))));
        store.inter_visibility(Rc::new(RefCell::new(Visibility::Krate(KRATE))));
        store.inter_visibility(Rc::new(RefCell::new(Visibility::Private(PRIVATE))));
        store.inter_visibility(Rc::new(RefCell::new(Visibility::Public(PUBLIC))));

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog-object-store-methods"}}}
    /// Inter (insert) [`Access`] into the store.
    ///
    pub fn inter_access(&mut self, access: Rc<RefCell<Access>>) {
        let read = access.borrow();
        self.access
            .borrow_mut()
            .insert(read.id, (access.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Access`] from the store.
    ///
    pub fn exhume_access(&self, id: &Uuid) -> Option<Rc<RefCell<Access>>> {
        self.access.borrow().get(id).map(|access| access.0.clone())
    }

    /// Exorcise (remove) [`Access`] from the store.
    ///
    pub fn exorcise_access(&mut self, id: &Uuid) -> Option<Rc<RefCell<Access>>> {
        self.access
            .borrow_mut()
            .remove(id)
            .map(|access| access.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Access>`.
    ///
    pub fn iter_access(&self) -> impl Iterator<Item = Rc<RefCell<Access>>> + '_ {
        let values: Vec<Rc<RefCell<Access>>> = self
            .access
            .borrow()
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
            .borrow()
            .get(&access.id)
            .map(|access| access.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Block`] into the store.
    ///
    pub fn inter_block(&mut self, block: Rc<RefCell<Block>>) {
        let read = block.borrow();
        self.block
            .borrow_mut()
            .insert(read.id, (block.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Block`] from the store.
    ///
    pub fn exhume_block(&self, id: &Uuid) -> Option<Rc<RefCell<Block>>> {
        self.block.borrow().get(id).map(|block| block.0.clone())
    }

    /// Exorcise (remove) [`Block`] from the store.
    ///
    pub fn exorcise_block(&mut self, id: &Uuid) -> Option<Rc<RefCell<Block>>> {
        self.block
            .borrow_mut()
            .remove(id)
            .map(|block| block.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Block>`.
    ///
    pub fn iter_block(&self) -> impl Iterator<Item = Rc<RefCell<Block>>> + '_ {
        let values: Vec<Rc<RefCell<Block>>> = self
            .block
            .borrow()
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
            .borrow()
            .get(&block.id)
            .map(|block| block.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Call`] into the store.
    ///
    pub fn inter_call(&mut self, call: Rc<RefCell<Call>>) {
        let read = call.borrow();
        self.call
            .borrow_mut()
            .insert(read.id, (call.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Call`] from the store.
    ///
    pub fn exhume_call(&self, id: &Uuid) -> Option<Rc<RefCell<Call>>> {
        self.call.borrow().get(id).map(|call| call.0.clone())
    }

    /// Exorcise (remove) [`Call`] from the store.
    ///
    pub fn exorcise_call(&mut self, id: &Uuid) -> Option<Rc<RefCell<Call>>> {
        self.call.borrow_mut().remove(id).map(|call| call.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Call>`.
    ///
    pub fn iter_call(&self) -> impl Iterator<Item = Rc<RefCell<Call>>> + '_ {
        let values: Vec<Rc<RefCell<Call>>> = self
            .call
            .borrow()
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
            .borrow()
            .get(&call.id)
            .map(|call| call.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Constant`] into the store.
    ///
    pub fn inter_constant(&mut self, constant: Rc<RefCell<Constant>>) {
        let read = constant.borrow();
        self.constant
            .borrow_mut()
            .insert(read.id, (constant.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Constant`] from the store.
    ///
    pub fn exhume_constant(&self, id: &Uuid) -> Option<Rc<RefCell<Constant>>> {
        self.constant
            .borrow()
            .get(id)
            .map(|constant| constant.0.clone())
    }

    /// Exorcise (remove) [`Constant`] from the store.
    ///
    pub fn exorcise_constant(&mut self, id: &Uuid) -> Option<Rc<RefCell<Constant>>> {
        self.constant
            .borrow_mut()
            .remove(id)
            .map(|constant| constant.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Constant>`.
    ///
    pub fn iter_constant(&self) -> impl Iterator<Item = Rc<RefCell<Constant>>> + '_ {
        let values: Vec<Rc<RefCell<Constant>>> = self
            .constant
            .borrow()
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
            .borrow()
            .get(&constant.id)
            .map(|constant| constant.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Enumeration`] into the store.
    ///
    pub fn inter_enumeration(&mut self, enumeration: Rc<RefCell<Enumeration>>) {
        let read = enumeration.borrow();
        self.enumeration
            .borrow_mut()
            .insert(read.id, (enumeration.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Enumeration`] from the store.
    ///
    pub fn exhume_enumeration(&self, id: &Uuid) -> Option<Rc<RefCell<Enumeration>>> {
        self.enumeration
            .borrow()
            .get(id)
            .map(|enumeration| enumeration.0.clone())
    }

    /// Exorcise (remove) [`Enumeration`] from the store.
    ///
    pub fn exorcise_enumeration(&mut self, id: &Uuid) -> Option<Rc<RefCell<Enumeration>>> {
        self.enumeration
            .borrow_mut()
            .remove(id)
            .map(|enumeration| enumeration.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Enumeration>`.
    ///
    pub fn iter_enumeration(&self) -> impl Iterator<Item = Rc<RefCell<Enumeration>>> + '_ {
        let values: Vec<Rc<RefCell<Enumeration>>> = self
            .enumeration
            .borrow()
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
            .borrow()
            .get(&enumeration.id)
            .map(|enumeration| enumeration.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`EnumerationField`] into the store.
    ///
    pub fn inter_enumeration_field(&mut self, enumeration_field: Rc<RefCell<EnumerationField>>) {
        let read = enumeration_field.borrow();
        self.enumeration_field
            .borrow_mut()
            .insert(read.id, (enumeration_field.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`EnumerationField`] from the store.
    ///
    pub fn exhume_enumeration_field(&self, id: &Uuid) -> Option<Rc<RefCell<EnumerationField>>> {
        self.enumeration_field
            .borrow()
            .get(id)
            .map(|enumeration_field| enumeration_field.0.clone())
    }

    /// Exorcise (remove) [`EnumerationField`] from the store.
    ///
    pub fn exorcise_enumeration_field(
        &mut self,
        id: &Uuid,
    ) -> Option<Rc<RefCell<EnumerationField>>> {
        self.enumeration_field
            .borrow_mut()
            .remove(id)
            .map(|enumeration_field| enumeration_field.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, EnumerationField>`.
    ///
    pub fn iter_enumeration_field(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<EnumerationField>>> + '_ {
        let values: Vec<Rc<RefCell<EnumerationField>>> = self
            .enumeration_field
            .borrow()
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
            .borrow()
            .get(&enumeration_field.id)
            .map(|enumeration_field| enumeration_field.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Expression`] into the store.
    ///
    pub fn inter_expression(&mut self, expression: Rc<RefCell<Expression>>) {
        let read = expression.borrow();
        self.expression
            .borrow_mut()
            .insert(read.id(), (expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Expression`] from the store.
    ///
    pub fn exhume_expression(&self, id: &Uuid) -> Option<Rc<RefCell<Expression>>> {
        self.expression
            .borrow()
            .get(id)
            .map(|expression| expression.0.clone())
    }

    /// Exorcise (remove) [`Expression`] from the store.
    ///
    pub fn exorcise_expression(&mut self, id: &Uuid) -> Option<Rc<RefCell<Expression>>> {
        self.expression
            .borrow_mut()
            .remove(id)
            .map(|expression| expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Expression>`.
    ///
    pub fn iter_expression(&self) -> impl Iterator<Item = Rc<RefCell<Expression>>> + '_ {
        let values: Vec<Rc<RefCell<Expression>>> = self
            .expression
            .borrow()
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
            .borrow()
            .get(&expression.id())
            .map(|expression| expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Field`] into the store.
    ///
    pub fn inter_field(&mut self, field: Rc<RefCell<Field>>) {
        let read = field.borrow();
        let value = (field.clone(), SystemTime::now());
        self.field_id_by_name
            .borrow_mut()
            .insert(read.name.to_upper_camel_case(), (read.id, value.1));
        self.field.borrow_mut().insert(read.id, value);
    }

    /// Exhume (get) [`Field`] from the store.
    ///
    pub fn exhume_field(&self, id: &Uuid) -> Option<Rc<RefCell<Field>>> {
        self.field.borrow().get(id).map(|field| field.0.clone())
    }

    /// Exorcise (remove) [`Field`] from the store.
    ///
    pub fn exorcise_field(&mut self, id: &Uuid) -> Option<Rc<RefCell<Field>>> {
        self.field
            .borrow_mut()
            .remove(id)
            .map(|field| field.0.clone())
    }

    /// Exhume [`Field`] id from the store by name.
    ///
    pub fn exhume_field_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.field_id_by_name
            .borrow()
            .get(name)
            .map(|field| field.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Field>`.
    ///
    pub fn iter_field(&self) -> impl Iterator<Item = Rc<RefCell<Field>>> + '_ {
        let values: Vec<Rc<RefCell<Field>>> = self
            .field
            .borrow()
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
            .borrow()
            .get(&field.id)
            .map(|field| field.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Function`] into the store.
    ///
    pub fn inter_function(&mut self, function: Rc<RefCell<Function>>) {
        let read = function.borrow();
        let value = (function.clone(), SystemTime::now());
        self.function_id_by_name
            .borrow_mut()
            .insert(read.name.to_upper_camel_case(), (read.id, value.1));
        self.function.borrow_mut().insert(read.id, value);
    }

    /// Exhume (get) [`Function`] from the store.
    ///
    pub fn exhume_function(&self, id: &Uuid) -> Option<Rc<RefCell<Function>>> {
        self.function
            .borrow()
            .get(id)
            .map(|function| function.0.clone())
    }

    /// Exorcise (remove) [`Function`] from the store.
    ///
    pub fn exorcise_function(&mut self, id: &Uuid) -> Option<Rc<RefCell<Function>>> {
        self.function
            .borrow_mut()
            .remove(id)
            .map(|function| function.0.clone())
    }

    /// Exhume [`Function`] id from the store by name.
    ///
    pub fn exhume_function_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.function_id_by_name
            .borrow()
            .get(name)
            .map(|function| function.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Function>`.
    ///
    pub fn iter_function(&self) -> impl Iterator<Item = Rc<RefCell<Function>>> + '_ {
        let values: Vec<Rc<RefCell<Function>>> = self
            .function
            .borrow()
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
            .borrow()
            .get(&function.id)
            .map(|function| function.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`GenerationUnit`] into the store.
    ///
    pub fn inter_generation_unit(&mut self, generation_unit: Rc<RefCell<GenerationUnit>>) {
        let read = generation_unit.borrow();
        self.generation_unit
            .borrow_mut()
            .insert(read.id, (generation_unit.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`GenerationUnit`] from the store.
    ///
    pub fn exhume_generation_unit(&self, id: &Uuid) -> Option<Rc<RefCell<GenerationUnit>>> {
        self.generation_unit
            .borrow()
            .get(id)
            .map(|generation_unit| generation_unit.0.clone())
    }

    /// Exorcise (remove) [`GenerationUnit`] from the store.
    ///
    pub fn exorcise_generation_unit(&mut self, id: &Uuid) -> Option<Rc<RefCell<GenerationUnit>>> {
        self.generation_unit
            .borrow_mut()
            .remove(id)
            .map(|generation_unit| generation_unit.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, GenerationUnit>`.
    ///
    pub fn iter_generation_unit(&self) -> impl Iterator<Item = Rc<RefCell<GenerationUnit>>> + '_ {
        let values: Vec<Rc<RefCell<GenerationUnit>>> = self
            .generation_unit
            .borrow()
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
            .borrow()
            .get(&generation_unit.id)
            .map(|generation_unit| generation_unit.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`GraceType`] into the store.
    ///
    pub fn inter_grace_type(&mut self, grace_type: Rc<RefCell<GraceType>>) {
        let read = grace_type.borrow();
        self.grace_type
            .borrow_mut()
            .insert(read.id(), (grace_type.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`GraceType`] from the store.
    ///
    pub fn exhume_grace_type(&self, id: &Uuid) -> Option<Rc<RefCell<GraceType>>> {
        self.grace_type
            .borrow()
            .get(id)
            .map(|grace_type| grace_type.0.clone())
    }

    /// Exorcise (remove) [`GraceType`] from the store.
    ///
    pub fn exorcise_grace_type(&mut self, id: &Uuid) -> Option<Rc<RefCell<GraceType>>> {
        self.grace_type
            .borrow_mut()
            .remove(id)
            .map(|grace_type| grace_type.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, GraceType>`.
    ///
    pub fn iter_grace_type(&self) -> impl Iterator<Item = Rc<RefCell<GraceType>>> + '_ {
        let values: Vec<Rc<RefCell<GraceType>>> = self
            .grace_type
            .borrow()
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
            .borrow()
            .get(&grace_type.id())
            .map(|grace_type| grace_type.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Item`] into the store.
    ///
    pub fn inter_item(&mut self, item: Rc<RefCell<Item>>) {
        let read = item.borrow();
        self.item
            .borrow_mut()
            .insert(read.id(), (item.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Item`] from the store.
    ///
    pub fn exhume_item(&self, id: &Uuid) -> Option<Rc<RefCell<Item>>> {
        self.item.borrow().get(id).map(|item| item.0.clone())
    }

    /// Exorcise (remove) [`Item`] from the store.
    ///
    pub fn exorcise_item(&mut self, id: &Uuid) -> Option<Rc<RefCell<Item>>> {
        self.item.borrow_mut().remove(id).map(|item| item.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Item>`.
    ///
    pub fn iter_item(&self) -> impl Iterator<Item = Rc<RefCell<Item>>> + '_ {
        let values: Vec<Rc<RefCell<Item>>> = self
            .item
            .borrow()
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
            .borrow()
            .get(&item.id())
            .map(|item| item.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`XLet`] into the store.
    ///
    pub fn inter_x_let(&mut self, x_let: Rc<RefCell<XLet>>) {
        let read = x_let.borrow();
        self.x_let
            .borrow_mut()
            .insert(read.id, (x_let.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`XLet`] from the store.
    ///
    pub fn exhume_x_let(&self, id: &Uuid) -> Option<Rc<RefCell<XLet>>> {
        self.x_let.borrow().get(id).map(|x_let| x_let.0.clone())
    }

    /// Exorcise (remove) [`XLet`] from the store.
    ///
    pub fn exorcise_x_let(&mut self, id: &Uuid) -> Option<Rc<RefCell<XLet>>> {
        self.x_let
            .borrow_mut()
            .remove(id)
            .map(|x_let| x_let.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XLet>`.
    ///
    pub fn iter_x_let(&self) -> impl Iterator<Item = Rc<RefCell<XLet>>> + '_ {
        let values: Vec<Rc<RefCell<XLet>>> = self
            .x_let
            .borrow()
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
            .borrow()
            .get(&x_let.id)
            .map(|x_let| x_let.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Local`] into the store.
    ///
    pub fn inter_local(&mut self, local: Rc<RefCell<Local>>) {
        let read = local.borrow();
        self.local
            .borrow_mut()
            .insert(read.id, (local.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Local`] from the store.
    ///
    pub fn exhume_local(&self, id: &Uuid) -> Option<Rc<RefCell<Local>>> {
        self.local.borrow().get(id).map(|local| local.0.clone())
    }

    /// Exorcise (remove) [`Local`] from the store.
    ///
    pub fn exorcise_local(&mut self, id: &Uuid) -> Option<Rc<RefCell<Local>>> {
        self.local
            .borrow_mut()
            .remove(id)
            .map(|local| local.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Local>`.
    ///
    pub fn iter_local(&self) -> impl Iterator<Item = Rc<RefCell<Local>>> + '_ {
        let values: Vec<Rc<RefCell<Local>>> = self
            .local
            .borrow()
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
            .borrow()
            .get(&local.id)
            .map(|local| local.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ObjectMethod`] into the store.
    ///
    pub fn inter_object_method(&mut self, object_method: Rc<RefCell<ObjectMethod>>) {
        let read = object_method.borrow();
        self.object_method
            .borrow_mut()
            .insert(read.id, (object_method.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ObjectMethod`] from the store.
    ///
    pub fn exhume_object_method(&self, id: &Uuid) -> Option<Rc<RefCell<ObjectMethod>>> {
        self.object_method
            .borrow()
            .get(id)
            .map(|object_method| object_method.0.clone())
    }

    /// Exorcise (remove) [`ObjectMethod`] from the store.
    ///
    pub fn exorcise_object_method(&mut self, id: &Uuid) -> Option<Rc<RefCell<ObjectMethod>>> {
        self.object_method
            .borrow_mut()
            .remove(id)
            .map(|object_method| object_method.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ObjectMethod>`.
    ///
    pub fn iter_object_method(&self) -> impl Iterator<Item = Rc<RefCell<ObjectMethod>>> + '_ {
        let values: Vec<Rc<RefCell<ObjectMethod>>> = self
            .object_method
            .borrow()
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
            .borrow()
            .get(&object_method.id)
            .map(|object_method| object_method.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`WoogOption`] into the store.
    ///
    pub fn inter_woog_option(&mut self, woog_option: Rc<RefCell<WoogOption>>) {
        let read = woog_option.borrow();
        self.woog_option
            .borrow_mut()
            .insert(read.id, (woog_option.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`WoogOption`] from the store.
    ///
    pub fn exhume_woog_option(&self, id: &Uuid) -> Option<Rc<RefCell<WoogOption>>> {
        self.woog_option
            .borrow()
            .get(id)
            .map(|woog_option| woog_option.0.clone())
    }

    /// Exorcise (remove) [`WoogOption`] from the store.
    ///
    pub fn exorcise_woog_option(&mut self, id: &Uuid) -> Option<Rc<RefCell<WoogOption>>> {
        self.woog_option
            .borrow_mut()
            .remove(id)
            .map(|woog_option| woog_option.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, WoogOption>`.
    ///
    pub fn iter_woog_option(&self) -> impl Iterator<Item = Rc<RefCell<WoogOption>>> + '_ {
        let values: Vec<Rc<RefCell<WoogOption>>> = self
            .woog_option
            .borrow()
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
            .borrow()
            .get(&woog_option.id)
            .map(|woog_option| woog_option.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Ownership`] into the store.
    ///
    pub fn inter_ownership(&mut self, ownership: Rc<RefCell<Ownership>>) {
        let read = ownership.borrow();
        self.ownership
            .borrow_mut()
            .insert(read.id(), (ownership.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Ownership`] from the store.
    ///
    pub fn exhume_ownership(&self, id: &Uuid) -> Option<Rc<RefCell<Ownership>>> {
        self.ownership
            .borrow()
            .get(id)
            .map(|ownership| ownership.0.clone())
    }

    /// Exorcise (remove) [`Ownership`] from the store.
    ///
    pub fn exorcise_ownership(&mut self, id: &Uuid) -> Option<Rc<RefCell<Ownership>>> {
        self.ownership
            .borrow_mut()
            .remove(id)
            .map(|ownership| ownership.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Ownership>`.
    ///
    pub fn iter_ownership(&self) -> impl Iterator<Item = Rc<RefCell<Ownership>>> + '_ {
        let values: Vec<Rc<RefCell<Ownership>>> = self
            .ownership
            .borrow()
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
            .borrow()
            .get(&ownership.id())
            .map(|ownership| ownership.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Parameter`] into the store.
    ///
    pub fn inter_parameter(&mut self, parameter: Rc<RefCell<Parameter>>) {
        let read = parameter.borrow();
        self.parameter
            .borrow_mut()
            .insert(read.id, (parameter.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Parameter`] from the store.
    ///
    pub fn exhume_parameter(&self, id: &Uuid) -> Option<Rc<RefCell<Parameter>>> {
        self.parameter
            .borrow()
            .get(id)
            .map(|parameter| parameter.0.clone())
    }

    /// Exorcise (remove) [`Parameter`] from the store.
    ///
    pub fn exorcise_parameter(&mut self, id: &Uuid) -> Option<Rc<RefCell<Parameter>>> {
        self.parameter
            .borrow_mut()
            .remove(id)
            .map(|parameter| parameter.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Parameter>`.
    ///
    pub fn iter_parameter(&self) -> impl Iterator<Item = Rc<RefCell<Parameter>>> + '_ {
        let values: Vec<Rc<RefCell<Parameter>>> = self
            .parameter
            .borrow()
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
            .borrow()
            .get(&parameter.id)
            .map(|parameter| parameter.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Reference`] into the store.
    ///
    pub fn inter_reference(&mut self, reference: Rc<RefCell<Reference>>) {
        let read = reference.borrow();
        self.reference
            .borrow_mut()
            .insert(read.id, (reference.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Reference`] from the store.
    ///
    pub fn exhume_reference(&self, id: &Uuid) -> Option<Rc<RefCell<Reference>>> {
        self.reference
            .borrow()
            .get(id)
            .map(|reference| reference.0.clone())
    }

    /// Exorcise (remove) [`Reference`] from the store.
    ///
    pub fn exorcise_reference(&mut self, id: &Uuid) -> Option<Rc<RefCell<Reference>>> {
        self.reference
            .borrow_mut()
            .remove(id)
            .map(|reference| reference.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Reference>`.
    ///
    pub fn iter_reference(&self) -> impl Iterator<Item = Rc<RefCell<Reference>>> + '_ {
        let values: Vec<Rc<RefCell<Reference>>> = self
            .reference
            .borrow()
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
            .borrow()
            .get(&reference.id)
            .map(|reference| reference.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Statement`] into the store.
    ///
    pub fn inter_statement(&mut self, statement: Rc<RefCell<Statement>>) {
        let read = statement.borrow();
        self.statement
            .borrow_mut()
            .insert(read.id, (statement.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Statement`] from the store.
    ///
    pub fn exhume_statement(&self, id: &Uuid) -> Option<Rc<RefCell<Statement>>> {
        self.statement
            .borrow()
            .get(id)
            .map(|statement| statement.0.clone())
    }

    /// Exorcise (remove) [`Statement`] from the store.
    ///
    pub fn exorcise_statement(&mut self, id: &Uuid) -> Option<Rc<RefCell<Statement>>> {
        self.statement
            .borrow_mut()
            .remove(id)
            .map(|statement| statement.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Statement>`.
    ///
    pub fn iter_statement(&self) -> impl Iterator<Item = Rc<RefCell<Statement>>> + '_ {
        let values: Vec<Rc<RefCell<Statement>>> = self
            .statement
            .borrow()
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
            .borrow()
            .get(&statement.id)
            .map(|statement| statement.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Structure`] into the store.
    ///
    pub fn inter_structure(&mut self, structure: Rc<RefCell<Structure>>) {
        let read = structure.borrow();
        self.structure
            .borrow_mut()
            .insert(read.id, (structure.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Structure`] from the store.
    ///
    pub fn exhume_structure(&self, id: &Uuid) -> Option<Rc<RefCell<Structure>>> {
        self.structure
            .borrow()
            .get(id)
            .map(|structure| structure.0.clone())
    }

    /// Exorcise (remove) [`Structure`] from the store.
    ///
    pub fn exorcise_structure(&mut self, id: &Uuid) -> Option<Rc<RefCell<Structure>>> {
        self.structure
            .borrow_mut()
            .remove(id)
            .map(|structure| structure.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Structure>`.
    ///
    pub fn iter_structure(&self) -> impl Iterator<Item = Rc<RefCell<Structure>>> + '_ {
        let values: Vec<Rc<RefCell<Structure>>> = self
            .structure
            .borrow()
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
            .borrow()
            .get(&structure.id)
            .map(|structure| structure.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`StructureField`] into the store.
    ///
    pub fn inter_structure_field(&mut self, structure_field: Rc<RefCell<StructureField>>) {
        let read = structure_field.borrow();
        self.structure_field
            .borrow_mut()
            .insert(read.id, (structure_field.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`StructureField`] from the store.
    ///
    pub fn exhume_structure_field(&self, id: &Uuid) -> Option<Rc<RefCell<StructureField>>> {
        self.structure_field
            .borrow()
            .get(id)
            .map(|structure_field| structure_field.0.clone())
    }

    /// Exorcise (remove) [`StructureField`] from the store.
    ///
    pub fn exorcise_structure_field(&mut self, id: &Uuid) -> Option<Rc<RefCell<StructureField>>> {
        self.structure_field
            .borrow_mut()
            .remove(id)
            .map(|structure_field| structure_field.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructureField>`.
    ///
    pub fn iter_structure_field(&self) -> impl Iterator<Item = Rc<RefCell<StructureField>>> + '_ {
        let values: Vec<Rc<RefCell<StructureField>>> = self
            .structure_field
            .borrow()
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
            .borrow()
            .get(&structure_field.id)
            .map(|structure_field| structure_field.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`SymbolTable`] into the store.
    ///
    pub fn inter_symbol_table(&mut self, symbol_table: Rc<RefCell<SymbolTable>>) {
        let read = symbol_table.borrow();
        self.symbol_table
            .borrow_mut()
            .insert(read.id, (symbol_table.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`SymbolTable`] from the store.
    ///
    pub fn exhume_symbol_table(&self, id: &Uuid) -> Option<Rc<RefCell<SymbolTable>>> {
        self.symbol_table
            .borrow()
            .get(id)
            .map(|symbol_table| symbol_table.0.clone())
    }

    /// Exorcise (remove) [`SymbolTable`] from the store.
    ///
    pub fn exorcise_symbol_table(&mut self, id: &Uuid) -> Option<Rc<RefCell<SymbolTable>>> {
        self.symbol_table
            .borrow_mut()
            .remove(id)
            .map(|symbol_table| symbol_table.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SymbolTable>`.
    ///
    pub fn iter_symbol_table(&self) -> impl Iterator<Item = Rc<RefCell<SymbolTable>>> + '_ {
        let values: Vec<Rc<RefCell<SymbolTable>>> = self
            .symbol_table
            .borrow()
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
            .borrow()
            .get(&symbol_table.id)
            .map(|symbol_table| symbol_table.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`TimeStamp`] into the store.
    ///
    pub fn inter_time_stamp(&mut self, time_stamp: Rc<RefCell<TimeStamp>>) {
        let read = time_stamp.borrow();
        self.time_stamp
            .borrow_mut()
            .insert(read.id, (time_stamp.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`TimeStamp`] from the store.
    ///
    pub fn exhume_time_stamp(&self, id: &Uuid) -> Option<Rc<RefCell<TimeStamp>>> {
        self.time_stamp
            .borrow()
            .get(id)
            .map(|time_stamp| time_stamp.0.clone())
    }

    /// Exorcise (remove) [`TimeStamp`] from the store.
    ///
    pub fn exorcise_time_stamp(&mut self, id: &Uuid) -> Option<Rc<RefCell<TimeStamp>>> {
        self.time_stamp
            .borrow_mut()
            .remove(id)
            .map(|time_stamp| time_stamp.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, TimeStamp>`.
    ///
    pub fn iter_time_stamp(&self) -> impl Iterator<Item = Rc<RefCell<TimeStamp>>> + '_ {
        let values: Vec<Rc<RefCell<TimeStamp>>> = self
            .time_stamp
            .borrow()
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
            .borrow()
            .get(&time_stamp.id)
            .map(|time_stamp| time_stamp.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`XValue`] into the store.
    ///
    pub fn inter_x_value(&mut self, x_value: Rc<RefCell<XValue>>) {
        let read = x_value.borrow();
        self.x_value
            .borrow_mut()
            .insert(read.id, (x_value.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`XValue`] from the store.
    ///
    pub fn exhume_x_value(&self, id: &Uuid) -> Option<Rc<RefCell<XValue>>> {
        self.x_value
            .borrow()
            .get(id)
            .map(|x_value| x_value.0.clone())
    }

    /// Exorcise (remove) [`XValue`] from the store.
    ///
    pub fn exorcise_x_value(&mut self, id: &Uuid) -> Option<Rc<RefCell<XValue>>> {
        self.x_value
            .borrow_mut()
            .remove(id)
            .map(|x_value| x_value.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XValue>`.
    ///
    pub fn iter_x_value(&self) -> impl Iterator<Item = Rc<RefCell<XValue>>> + '_ {
        let values: Vec<Rc<RefCell<XValue>>> = self
            .x_value
            .borrow()
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
            .borrow()
            .get(&x_value.id)
            .map(|x_value| x_value.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Variable`] into the store.
    ///
    pub fn inter_variable(&mut self, variable: Rc<RefCell<Variable>>) {
        let read = variable.borrow();
        self.variable
            .borrow_mut()
            .insert(read.id, (variable.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Variable`] from the store.
    ///
    pub fn exhume_variable(&self, id: &Uuid) -> Option<Rc<RefCell<Variable>>> {
        self.variable
            .borrow()
            .get(id)
            .map(|variable| variable.0.clone())
    }

    /// Exorcise (remove) [`Variable`] from the store.
    ///
    pub fn exorcise_variable(&mut self, id: &Uuid) -> Option<Rc<RefCell<Variable>>> {
        self.variable
            .borrow_mut()
            .remove(id)
            .map(|variable| variable.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Variable>`.
    ///
    pub fn iter_variable(&self) -> impl Iterator<Item = Rc<RefCell<Variable>>> + '_ {
        let values: Vec<Rc<RefCell<Variable>>> = self
            .variable
            .borrow()
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
            .borrow()
            .get(&variable.id)
            .map(|variable| variable.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Visibility`] into the store.
    ///
    pub fn inter_visibility(&mut self, visibility: Rc<RefCell<Visibility>>) {
        let read = visibility.borrow();
        self.visibility
            .borrow_mut()
            .insert(read.id(), (visibility.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Visibility`] from the store.
    ///
    pub fn exhume_visibility(&self, id: &Uuid) -> Option<Rc<RefCell<Visibility>>> {
        self.visibility
            .borrow()
            .get(id)
            .map(|visibility| visibility.0.clone())
    }

    /// Exorcise (remove) [`Visibility`] from the store.
    ///
    pub fn exorcise_visibility(&mut self, id: &Uuid) -> Option<Rc<RefCell<Visibility>>> {
        self.visibility
            .borrow_mut()
            .remove(id)
            .map(|visibility| visibility.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Visibility>`.
    ///
    pub fn iter_visibility(&self) -> impl Iterator<Item = Rc<RefCell<Visibility>>> + '_ {
        let values: Vec<Rc<RefCell<Visibility>>> = self
            .visibility
            .borrow()
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
            .borrow()
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
            for access_tuple in self.access.borrow().values() {
                let path = path.join(format!("{}.json", access_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Access>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != access_tuple.0.borrow().to_owned() {
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
                    if !self.access.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Block.
        {
            let path = path.join("block");
            fs::create_dir_all(&path)?;
            for block_tuple in self.block.borrow().values() {
                let path = path.join(format!("{}.json", block_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Block>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != block_tuple.0.borrow().to_owned() {
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
                    if !self.block.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Call.
        {
            let path = path.join("call");
            fs::create_dir_all(&path)?;
            for call_tuple in self.call.borrow().values() {
                let path = path.join(format!("{}.json", call_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Call>>, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != call_tuple.0.borrow().to_owned() {
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
                    if !self.call.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Constant.
        {
            let path = path.join("constant");
            fs::create_dir_all(&path)?;
            for constant_tuple in self.constant.borrow().values() {
                let path = path.join(format!("{}.json", constant_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Constant>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != constant_tuple.0.borrow().to_owned() {
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
                    if !self.constant.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Enumeration.
        {
            let path = path.join("enumeration");
            fs::create_dir_all(&path)?;
            for enumeration_tuple in self.enumeration.borrow().values() {
                let path = path.join(format!("{}.json", enumeration_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Enumeration>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != enumeration_tuple.0.borrow().to_owned() {
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
                    if !self.enumeration.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Enumeration Field.
        {
            let path = path.join("enumeration_field");
            fs::create_dir_all(&path)?;
            for enumeration_field_tuple in self.enumeration_field.borrow().values() {
                let path = path.join(format!("{}.json", enumeration_field_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<EnumerationField>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned()
                        != enumeration_field_tuple.0.borrow().to_owned()
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
                    if !self.enumeration_field.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Expression.
        {
            let path = path.join("expression");
            fs::create_dir_all(&path)?;
            for expression_tuple in self.expression.borrow().values() {
                let path = path.join(format!("{}.json", expression_tuple.0.borrow().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Expression>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != expression_tuple.0.borrow().to_owned() {
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
                    if !self.expression.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Field.
        {
            let path = path.join("field");
            fs::create_dir_all(&path)?;
            for field_tuple in self.field.borrow().values() {
                let path = path.join(format!("{}.json", field_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Field>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != field_tuple.0.borrow().to_owned() {
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
                    if !self.field.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Function.
        {
            let path = path.join("function");
            fs::create_dir_all(&path)?;
            for function_tuple in self.function.borrow().values() {
                let path = path.join(format!("{}.json", function_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Function>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != function_tuple.0.borrow().to_owned() {
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
                    if !self.function.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Generation Unit.
        {
            let path = path.join("generation_unit");
            fs::create_dir_all(&path)?;
            for generation_unit_tuple in self.generation_unit.borrow().values() {
                let path = path.join(format!("{}.json", generation_unit_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<GenerationUnit>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != generation_unit_tuple.0.borrow().to_owned()
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
                    if !self.generation_unit.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Grace Type.
        {
            let path = path.join("grace_type");
            fs::create_dir_all(&path)?;
            for grace_type_tuple in self.grace_type.borrow().values() {
                let path = path.join(format!("{}.json", grace_type_tuple.0.borrow().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<GraceType>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != grace_type_tuple.0.borrow().to_owned() {
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
                    if !self.grace_type.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Item.
        {
            let path = path.join("item");
            fs::create_dir_all(&path)?;
            for item_tuple in self.item.borrow().values() {
                let path = path.join(format!("{}.json", item_tuple.0.borrow().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Item>>, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != item_tuple.0.borrow().to_owned() {
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
                    if !self.item.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Let.
        {
            let path = path.join("x_let");
            fs::create_dir_all(&path)?;
            for x_let_tuple in self.x_let.borrow().values() {
                let path = path.join(format!("{}.json", x_let_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<XLet>>, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != x_let_tuple.0.borrow().to_owned() {
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
                    if !self.x_let.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Local.
        {
            let path = path.join("local");
            fs::create_dir_all(&path)?;
            for local_tuple in self.local.borrow().values() {
                let path = path.join(format!("{}.json", local_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Local>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != local_tuple.0.borrow().to_owned() {
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
                    if !self.local.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Object Method.
        {
            let path = path.join("object_method");
            fs::create_dir_all(&path)?;
            for object_method_tuple in self.object_method.borrow().values() {
                let path = path.join(format!("{}.json", object_method_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<ObjectMethod>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != object_method_tuple.0.borrow().to_owned() {
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
                    if !self.object_method.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Option.
        {
            let path = path.join("woog_option");
            fs::create_dir_all(&path)?;
            for woog_option_tuple in self.woog_option.borrow().values() {
                let path = path.join(format!("{}.json", woog_option_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<WoogOption>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != woog_option_tuple.0.borrow().to_owned() {
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
                    if !self.woog_option.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Ownership.
        {
            let path = path.join("ownership");
            fs::create_dir_all(&path)?;
            for ownership_tuple in self.ownership.borrow().values() {
                let path = path.join(format!("{}.json", ownership_tuple.0.borrow().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Ownership>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != ownership_tuple.0.borrow().to_owned() {
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
                    if !self.ownership.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Parameter.
        {
            let path = path.join("parameter");
            fs::create_dir_all(&path)?;
            for parameter_tuple in self.parameter.borrow().values() {
                let path = path.join(format!("{}.json", parameter_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Parameter>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != parameter_tuple.0.borrow().to_owned() {
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
                    if !self.parameter.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Reference.
        {
            let path = path.join("reference");
            fs::create_dir_all(&path)?;
            for reference_tuple in self.reference.borrow().values() {
                let path = path.join(format!("{}.json", reference_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Reference>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != reference_tuple.0.borrow().to_owned() {
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
                    if !self.reference.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Statement.
        {
            let path = path.join("statement");
            fs::create_dir_all(&path)?;
            for statement_tuple in self.statement.borrow().values() {
                let path = path.join(format!("{}.json", statement_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Statement>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != statement_tuple.0.borrow().to_owned() {
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
                    if !self.statement.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Structure.
        {
            let path = path.join("structure");
            fs::create_dir_all(&path)?;
            for structure_tuple in self.structure.borrow().values() {
                let path = path.join(format!("{}.json", structure_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Structure>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != structure_tuple.0.borrow().to_owned() {
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
                    if !self.structure.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Structure Field.
        {
            let path = path.join("structure_field");
            fs::create_dir_all(&path)?;
            for structure_field_tuple in self.structure_field.borrow().values() {
                let path = path.join(format!("{}.json", structure_field_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<StructureField>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != structure_field_tuple.0.borrow().to_owned()
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
                    if !self.structure_field.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Symbol Table.
        {
            let path = path.join("symbol_table");
            fs::create_dir_all(&path)?;
            for symbol_table_tuple in self.symbol_table.borrow().values() {
                let path = path.join(format!("{}.json", symbol_table_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<SymbolTable>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != symbol_table_tuple.0.borrow().to_owned() {
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
                    if !self.symbol_table.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Time Stamp.
        {
            let path = path.join("time_stamp");
            fs::create_dir_all(&path)?;
            for time_stamp_tuple in self.time_stamp.borrow().values() {
                let path = path.join(format!("{}.json", time_stamp_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<TimeStamp>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != time_stamp_tuple.0.borrow().to_owned() {
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
                    if !self.time_stamp.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Value.
        {
            let path = path.join("x_value");
            fs::create_dir_all(&path)?;
            for x_value_tuple in self.x_value.borrow().values() {
                let path = path.join(format!("{}.json", x_value_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<XValue>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != x_value_tuple.0.borrow().to_owned() {
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
                    if !self.x_value.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Variable.
        {
            let path = path.join("variable");
            fs::create_dir_all(&path)?;
            for variable_tuple in self.variable.borrow().values() {
                let path = path.join(format!("{}.json", variable_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Variable>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != variable_tuple.0.borrow().to_owned() {
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
                    if !self.variable.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Visibility.
        {
            let path = path.join("visibility");
            fs::create_dir_all(&path)?;
            for visibility_tuple in self.visibility.borrow().values() {
                let path = path.join(format!("{}.json", visibility_tuple.0.borrow().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Visibility>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != visibility_tuple.0.borrow().to_owned() {
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
                    if !self.visibility.borrow().contains_key(&id) {
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
                let access: (Rc<RefCell<Access>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .access
                    .borrow_mut()
                    .insert(access.0.borrow().id, access.clone());
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
                let block: (Rc<RefCell<Block>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .block
                    .borrow_mut()
                    .insert(block.0.borrow().id, block.clone());
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
                let call: (Rc<RefCell<Call>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .call
                    .borrow_mut()
                    .insert(call.0.borrow().id, call.clone());
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
                let constant: (Rc<RefCell<Constant>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .constant
                    .borrow_mut()
                    .insert(constant.0.borrow().id, constant.clone());
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
                let enumeration: (Rc<RefCell<Enumeration>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .enumeration
                    .borrow_mut()
                    .insert(enumeration.0.borrow().id, enumeration.clone());
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
                let enumeration_field: (Rc<RefCell<EnumerationField>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .enumeration_field
                    .borrow_mut()
                    .insert(enumeration_field.0.borrow().id, enumeration_field.clone());
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
                let expression: (Rc<RefCell<Expression>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .expression
                    .borrow_mut()
                    .insert(expression.0.borrow().id(), expression.clone());
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
                let field: (Rc<RefCell<Field>>, SystemTime) = serde_json::from_reader(reader)?;
                store.field_id_by_name.borrow_mut().insert(
                    field.0.borrow().name.to_upper_camel_case(),
                    (field.0.borrow().id, field.1),
                );
                store
                    .field
                    .borrow_mut()
                    .insert(field.0.borrow().id, field.clone());
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
                let function: (Rc<RefCell<Function>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.function_id_by_name.borrow_mut().insert(
                    function.0.borrow().name.to_upper_camel_case(),
                    (function.0.borrow().id, function.1),
                );
                store
                    .function
                    .borrow_mut()
                    .insert(function.0.borrow().id, function.clone());
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
                let generation_unit: (Rc<RefCell<GenerationUnit>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .generation_unit
                    .borrow_mut()
                    .insert(generation_unit.0.borrow().id, generation_unit.clone());
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
                let grace_type: (Rc<RefCell<GraceType>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .grace_type
                    .borrow_mut()
                    .insert(grace_type.0.borrow().id(), grace_type.clone());
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
                let item: (Rc<RefCell<Item>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .item
                    .borrow_mut()
                    .insert(item.0.borrow().id(), item.clone());
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
                let x_let: (Rc<RefCell<XLet>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .x_let
                    .borrow_mut()
                    .insert(x_let.0.borrow().id, x_let.clone());
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
                let local: (Rc<RefCell<Local>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .local
                    .borrow_mut()
                    .insert(local.0.borrow().id, local.clone());
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
                let object_method: (Rc<RefCell<ObjectMethod>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .object_method
                    .borrow_mut()
                    .insert(object_method.0.borrow().id, object_method.clone());
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
                let woog_option: (Rc<RefCell<WoogOption>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .woog_option
                    .borrow_mut()
                    .insert(woog_option.0.borrow().id, woog_option.clone());
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
                let ownership: (Rc<RefCell<Ownership>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .ownership
                    .borrow_mut()
                    .insert(ownership.0.borrow().id(), ownership.clone());
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
                let parameter: (Rc<RefCell<Parameter>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .parameter
                    .borrow_mut()
                    .insert(parameter.0.borrow().id, parameter.clone());
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
                let reference: (Rc<RefCell<Reference>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .reference
                    .borrow_mut()
                    .insert(reference.0.borrow().id, reference.clone());
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
                let statement: (Rc<RefCell<Statement>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .statement
                    .borrow_mut()
                    .insert(statement.0.borrow().id, statement.clone());
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
                let structure: (Rc<RefCell<Structure>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .structure
                    .borrow_mut()
                    .insert(structure.0.borrow().id, structure.clone());
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
                let structure_field: (Rc<RefCell<StructureField>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .structure_field
                    .borrow_mut()
                    .insert(structure_field.0.borrow().id, structure_field.clone());
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
                let symbol_table: (Rc<RefCell<SymbolTable>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .symbol_table
                    .borrow_mut()
                    .insert(symbol_table.0.borrow().id, symbol_table.clone());
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
                let time_stamp: (Rc<RefCell<TimeStamp>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .time_stamp
                    .borrow_mut()
                    .insert(time_stamp.0.borrow().id, time_stamp.clone());
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
                let x_value: (Rc<RefCell<XValue>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .x_value
                    .borrow_mut()
                    .insert(x_value.0.borrow().id, x_value.clone());
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
                let variable: (Rc<RefCell<Variable>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .variable
                    .borrow_mut()
                    .insert(variable.0.borrow().id, variable.clone());
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
                let visibility: (Rc<RefCell<Visibility>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .visibility
                    .borrow_mut()
                    .insert(visibility.0.borrow().id(), visibility.clone());
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
