//! v2::woog_single Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog_single-object-store-file"}}}
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog_single-object-store-definition"}}}
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
};

use heck::ToUpperCamelCase;
use rustc_hash::FxHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v2::woog_single::types::{
    Access, Block, Call, Constant, Enumeration, EnumerationField, Expression, Field, Function,
    GenerationUnit, GraceType, Item, Local, ObjectMethod, Ownership, Parameter, Reference,
    Statement, Structure, StructureField, SymbolTable, TimeStamp, Variable, Visibility, WoogOption,
    XLet, XValue, BORROWED, IMPLEMENTATION, KRATE, LITERAL, MUTABLE, OWNED, PRIVATE, PUBLIC, USIZE,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    access: HashMap<Uuid, Access>,
    block: HashMap<Uuid, Block>,
    call: HashMap<Uuid, Call>,
    constant: HashMap<Uuid, Constant>,
    enumeration: HashMap<Uuid, Enumeration>,
    enumeration_id_by_name: HashMap<String, Uuid>,
    enumeration_field: HashMap<Uuid, EnumerationField>,
    expression: HashMap<Uuid, Expression>,
    field: HashMap<Uuid, Field>,
    field_id_by_name: HashMap<String, Uuid>,
    function: HashMap<Uuid, Function>,
    function_id_by_name: HashMap<String, Uuid>,
    generation_unit: HashMap<Uuid, GenerationUnit>,
    grace_type: HashMap<Uuid, GraceType>,
    item: HashMap<Uuid, Item>,
    x_let: HashMap<Uuid, XLet>,
    local: HashMap<Uuid, Local>,
    object_method: HashMap<Uuid, ObjectMethod>,
    woog_option: HashMap<Uuid, WoogOption>,
    ownership: HashMap<Uuid, Ownership>,
    parameter: HashMap<Uuid, Parameter>,
    reference: HashMap<Uuid, Reference>,
    statement: HashMap<Uuid, Statement>,
    structure: HashMap<Uuid, Structure>,
    structure_field: HashMap<Uuid, StructureField>,
    symbol_table: HashMap<Uuid, SymbolTable>,
    time_stamp: HashMap<Uuid, TimeStamp>,
    x_value: HashMap<Uuid, XValue>,
    variable: HashMap<Uuid, Variable>,
    visibility: HashMap<Uuid, Visibility>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            access: HashMap::default(),
            block: HashMap::default(),
            call: HashMap::default(),
            constant: HashMap::default(),
            enumeration: HashMap::default(),
            enumeration_id_by_name: HashMap::default(),
            enumeration_field: HashMap::default(),
            expression: HashMap::default(),
            field: HashMap::default(),
            field_id_by_name: HashMap::default(),
            function: HashMap::default(),
            function_id_by_name: HashMap::default(),
            generation_unit: HashMap::default(),
            grace_type: HashMap::default(),
            item: HashMap::default(),
            x_let: HashMap::default(),
            local: HashMap::default(),
            object_method: HashMap::default(),
            woog_option: HashMap::default(),
            ownership: HashMap::default(),
            parameter: HashMap::default(),
            reference: HashMap::default(),
            statement: HashMap::default(),
            structure: HashMap::default(),
            structure_field: HashMap::default(),
            symbol_table: HashMap::default(),
            time_stamp: HashMap::default(),
            x_value: HashMap::default(),
            variable: HashMap::default(),
            visibility: HashMap::default(),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥
        store.inter_expression(Expression::Literal(LITERAL));
        store.inter_grace_type(GraceType::Usize(USIZE));
        store.inter_item(Item::Implementation(IMPLEMENTATION));
        store.inter_ownership(Ownership::Borrowed(BORROWED));
        store.inter_ownership(Ownership::Mutable(MUTABLE));
        store.inter_ownership(Ownership::Owned(OWNED));
        store.inter_visibility(Visibility::Krate(KRATE));
        store.inter_visibility(Visibility::Private(PRIVATE));
        store.inter_visibility(Visibility::Public(PUBLIC));

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog_single-object-store-methods"}}}
    /// Inter (insert) [`Access`] into the store.
    ///
    pub fn inter_access(&mut self, access: Access) {
        self.access.insert(access.id, access);
    }

    /// Exhume (get) [`Access`] from the store.
    ///
    pub fn exhume_access(&self, id: &Uuid) -> Option<&Access> {
        self.access.get(id)
    }

    /// Exorcise (remove) [`Access`] from the store.
    ///
    pub fn exorcise_access(&mut self, id: &Uuid) -> Option<Access> {
        self.access.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Access>`.
    ///
    pub fn iter_access(&self) -> impl Iterator<Item = &Access> {
        self.access.values()
    }

    /// Inter (insert) [`Block`] into the store.
    ///
    pub fn inter_block(&mut self, block: Block) {
        self.block.insert(block.id, block);
    }

    /// Exhume (get) [`Block`] from the store.
    ///
    pub fn exhume_block(&self, id: &Uuid) -> Option<&Block> {
        self.block.get(id)
    }

    /// Exorcise (remove) [`Block`] from the store.
    ///
    pub fn exorcise_block(&mut self, id: &Uuid) -> Option<Block> {
        self.block.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Block>`.
    ///
    pub fn iter_block(&self) -> impl Iterator<Item = &Block> {
        self.block.values()
    }

    /// Inter (insert) [`Call`] into the store.
    ///
    pub fn inter_call(&mut self, call: Call) {
        self.call.insert(call.id, call);
    }

    /// Exhume (get) [`Call`] from the store.
    ///
    pub fn exhume_call(&self, id: &Uuid) -> Option<&Call> {
        self.call.get(id)
    }

    /// Exorcise (remove) [`Call`] from the store.
    ///
    pub fn exorcise_call(&mut self, id: &Uuid) -> Option<Call> {
        self.call.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Call>`.
    ///
    pub fn iter_call(&self) -> impl Iterator<Item = &Call> {
        self.call.values()
    }

    /// Inter (insert) [`Constant`] into the store.
    ///
    pub fn inter_constant(&mut self, constant: Constant) {
        self.constant.insert(constant.id, constant);
    }

    /// Exhume (get) [`Constant`] from the store.
    ///
    pub fn exhume_constant(&self, id: &Uuid) -> Option<&Constant> {
        self.constant.get(id)
    }

    /// Exorcise (remove) [`Constant`] from the store.
    ///
    pub fn exorcise_constant(&mut self, id: &Uuid) -> Option<Constant> {
        self.constant.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Constant>`.
    ///
    pub fn iter_constant(&self) -> impl Iterator<Item = &Constant> {
        self.constant.values()
    }

    /// Inter (insert) [`Enumeration`] into the store.
    ///
    pub fn inter_enumeration(&mut self, enumeration: Enumeration) {
        self.enumeration_id_by_name
            .insert(enumeration.name.to_upper_camel_case(), enumeration.id);
        self.enumeration.insert(enumeration.id, enumeration);
    }

    /// Exhume (get) [`Enumeration`] from the store.
    ///
    pub fn exhume_enumeration(&self, id: &Uuid) -> Option<&Enumeration> {
        self.enumeration.get(id)
    }

    /// Exorcise (remove) [`Enumeration`] from the store.
    ///
    pub fn exorcise_enumeration(&mut self, id: &Uuid) -> Option<Enumeration> {
        self.enumeration.remove(id)
    }

    /// Exhume [`Enumeration`] id from the store by name.
    ///
    pub fn exhume_enumeration_id_by_name(&self, name: &str) -> Option<&Uuid> {
        self.enumeration_id_by_name.get(name)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Enumeration>`.
    ///
    pub fn iter_enumeration(&self) -> impl Iterator<Item = &Enumeration> {
        self.enumeration.values()
    }

    /// Inter (insert) [`EnumerationField`] into the store.
    ///
    pub fn inter_enumeration_field(&mut self, enumeration_field: EnumerationField) {
        self.enumeration_field
            .insert(enumeration_field.id, enumeration_field);
    }

    /// Exhume (get) [`EnumerationField`] from the store.
    ///
    pub fn exhume_enumeration_field(&self, id: &Uuid) -> Option<&EnumerationField> {
        self.enumeration_field.get(id)
    }

    /// Exorcise (remove) [`EnumerationField`] from the store.
    ///
    pub fn exorcise_enumeration_field(&mut self, id: &Uuid) -> Option<EnumerationField> {
        self.enumeration_field.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, EnumerationField>`.
    ///
    pub fn iter_enumeration_field(&self) -> impl Iterator<Item = &EnumerationField> {
        self.enumeration_field.values()
    }

    /// Inter (insert) [`Expression`] into the store.
    ///
    pub fn inter_expression(&mut self, expression: Expression) {
        self.expression.insert(expression.id(), expression);
    }

    /// Exhume (get) [`Expression`] from the store.
    ///
    pub fn exhume_expression(&self, id: &Uuid) -> Option<&Expression> {
        self.expression.get(id)
    }

    /// Exorcise (remove) [`Expression`] from the store.
    ///
    pub fn exorcise_expression(&mut self, id: &Uuid) -> Option<Expression> {
        self.expression.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Expression>`.
    ///
    pub fn iter_expression(&self) -> impl Iterator<Item = &Expression> {
        self.expression.values()
    }

    /// Inter (insert) [`Field`] into the store.
    ///
    pub fn inter_field(&mut self, field: Field) {
        self.field_id_by_name
            .insert(field.name.to_upper_camel_case(), field.id);
        self.field.insert(field.id, field);
    }

    /// Exhume (get) [`Field`] from the store.
    ///
    pub fn exhume_field(&self, id: &Uuid) -> Option<&Field> {
        self.field.get(id)
    }

    /// Exorcise (remove) [`Field`] from the store.
    ///
    pub fn exorcise_field(&mut self, id: &Uuid) -> Option<Field> {
        self.field.remove(id)
    }

    /// Exhume [`Field`] id from the store by name.
    ///
    pub fn exhume_field_id_by_name(&self, name: &str) -> Option<&Uuid> {
        self.field_id_by_name.get(name)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Field>`.
    ///
    pub fn iter_field(&self) -> impl Iterator<Item = &Field> {
        self.field.values()
    }

    /// Inter (insert) [`Function`] into the store.
    ///
    pub fn inter_function(&mut self, function: Function) {
        self.function_id_by_name
            .insert(function.name.to_upper_camel_case(), function.id);
        self.function.insert(function.id, function);
    }

    /// Exhume (get) [`Function`] from the store.
    ///
    pub fn exhume_function(&self, id: &Uuid) -> Option<&Function> {
        self.function.get(id)
    }

    /// Exorcise (remove) [`Function`] from the store.
    ///
    pub fn exorcise_function(&mut self, id: &Uuid) -> Option<Function> {
        self.function.remove(id)
    }

    /// Exhume [`Function`] id from the store by name.
    ///
    pub fn exhume_function_id_by_name(&self, name: &str) -> Option<&Uuid> {
        self.function_id_by_name.get(name)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Function>`.
    ///
    pub fn iter_function(&self) -> impl Iterator<Item = &Function> {
        self.function.values()
    }

    /// Inter (insert) [`GenerationUnit`] into the store.
    ///
    pub fn inter_generation_unit(&mut self, generation_unit: GenerationUnit) {
        self.generation_unit
            .insert(generation_unit.id, generation_unit);
    }

    /// Exhume (get) [`GenerationUnit`] from the store.
    ///
    pub fn exhume_generation_unit(&self, id: &Uuid) -> Option<&GenerationUnit> {
        self.generation_unit.get(id)
    }

    /// Exorcise (remove) [`GenerationUnit`] from the store.
    ///
    pub fn exorcise_generation_unit(&mut self, id: &Uuid) -> Option<GenerationUnit> {
        self.generation_unit.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, GenerationUnit>`.
    ///
    pub fn iter_generation_unit(&self) -> impl Iterator<Item = &GenerationUnit> {
        self.generation_unit.values()
    }

    /// Inter (insert) [`GraceType`] into the store.
    ///
    pub fn inter_grace_type(&mut self, grace_type: GraceType) {
        self.grace_type.insert(grace_type.id(), grace_type);
    }

    /// Exhume (get) [`GraceType`] from the store.
    ///
    pub fn exhume_grace_type(&self, id: &Uuid) -> Option<&GraceType> {
        self.grace_type.get(id)
    }

    /// Exorcise (remove) [`GraceType`] from the store.
    ///
    pub fn exorcise_grace_type(&mut self, id: &Uuid) -> Option<GraceType> {
        self.grace_type.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, GraceType>`.
    ///
    pub fn iter_grace_type(&self) -> impl Iterator<Item = &GraceType> {
        self.grace_type.values()
    }

    /// Inter (insert) [`Item`] into the store.
    ///
    pub fn inter_item(&mut self, item: Item) {
        self.item.insert(item.id(), item);
    }

    /// Exhume (get) [`Item`] from the store.
    ///
    pub fn exhume_item(&self, id: &Uuid) -> Option<&Item> {
        self.item.get(id)
    }

    /// Exorcise (remove) [`Item`] from the store.
    ///
    pub fn exorcise_item(&mut self, id: &Uuid) -> Option<Item> {
        self.item.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Item>`.
    ///
    pub fn iter_item(&self) -> impl Iterator<Item = &Item> {
        self.item.values()
    }

    /// Inter (insert) [`XLet`] into the store.
    ///
    pub fn inter_x_let(&mut self, x_let: XLet) {
        self.x_let.insert(x_let.id, x_let);
    }

    /// Exhume (get) [`XLet`] from the store.
    ///
    pub fn exhume_x_let(&self, id: &Uuid) -> Option<&XLet> {
        self.x_let.get(id)
    }

    /// Exorcise (remove) [`XLet`] from the store.
    ///
    pub fn exorcise_x_let(&mut self, id: &Uuid) -> Option<XLet> {
        self.x_let.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XLet>`.
    ///
    pub fn iter_x_let(&self) -> impl Iterator<Item = &XLet> {
        self.x_let.values()
    }

    /// Inter (insert) [`Local`] into the store.
    ///
    pub fn inter_local(&mut self, local: Local) {
        self.local.insert(local.id, local);
    }

    /// Exhume (get) [`Local`] from the store.
    ///
    pub fn exhume_local(&self, id: &Uuid) -> Option<&Local> {
        self.local.get(id)
    }

    /// Exorcise (remove) [`Local`] from the store.
    ///
    pub fn exorcise_local(&mut self, id: &Uuid) -> Option<Local> {
        self.local.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Local>`.
    ///
    pub fn iter_local(&self) -> impl Iterator<Item = &Local> {
        self.local.values()
    }

    /// Inter (insert) [`ObjectMethod`] into the store.
    ///
    pub fn inter_object_method(&mut self, object_method: ObjectMethod) {
        self.object_method.insert(object_method.id, object_method);
    }

    /// Exhume (get) [`ObjectMethod`] from the store.
    ///
    pub fn exhume_object_method(&self, id: &Uuid) -> Option<&ObjectMethod> {
        self.object_method.get(id)
    }

    /// Exorcise (remove) [`ObjectMethod`] from the store.
    ///
    pub fn exorcise_object_method(&mut self, id: &Uuid) -> Option<ObjectMethod> {
        self.object_method.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ObjectMethod>`.
    ///
    pub fn iter_object_method(&self) -> impl Iterator<Item = &ObjectMethod> {
        self.object_method.values()
    }

    /// Inter (insert) [`WoogOption`] into the store.
    ///
    pub fn inter_woog_option(&mut self, woog_option: WoogOption) {
        self.woog_option.insert(woog_option.id, woog_option);
    }

    /// Exhume (get) [`WoogOption`] from the store.
    ///
    pub fn exhume_woog_option(&self, id: &Uuid) -> Option<&WoogOption> {
        self.woog_option.get(id)
    }

    /// Exorcise (remove) [`WoogOption`] from the store.
    ///
    pub fn exorcise_woog_option(&mut self, id: &Uuid) -> Option<WoogOption> {
        self.woog_option.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, WoogOption>`.
    ///
    pub fn iter_woog_option(&self) -> impl Iterator<Item = &WoogOption> {
        self.woog_option.values()
    }

    /// Inter (insert) [`Ownership`] into the store.
    ///
    pub fn inter_ownership(&mut self, ownership: Ownership) {
        self.ownership.insert(ownership.id(), ownership);
    }

    /// Exhume (get) [`Ownership`] from the store.
    ///
    pub fn exhume_ownership(&self, id: &Uuid) -> Option<&Ownership> {
        self.ownership.get(id)
    }

    /// Exorcise (remove) [`Ownership`] from the store.
    ///
    pub fn exorcise_ownership(&mut self, id: &Uuid) -> Option<Ownership> {
        self.ownership.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Ownership>`.
    ///
    pub fn iter_ownership(&self) -> impl Iterator<Item = &Ownership> {
        self.ownership.values()
    }

    /// Inter (insert) [`Parameter`] into the store.
    ///
    pub fn inter_parameter(&mut self, parameter: Parameter) {
        self.parameter.insert(parameter.id, parameter);
    }

    /// Exhume (get) [`Parameter`] from the store.
    ///
    pub fn exhume_parameter(&self, id: &Uuid) -> Option<&Parameter> {
        self.parameter.get(id)
    }

    /// Exorcise (remove) [`Parameter`] from the store.
    ///
    pub fn exorcise_parameter(&mut self, id: &Uuid) -> Option<Parameter> {
        self.parameter.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Parameter>`.
    ///
    pub fn iter_parameter(&self) -> impl Iterator<Item = &Parameter> {
        self.parameter.values()
    }

    /// Inter (insert) [`Reference`] into the store.
    ///
    pub fn inter_reference(&mut self, reference: Reference) {
        self.reference.insert(reference.id, reference);
    }

    /// Exhume (get) [`Reference`] from the store.
    ///
    pub fn exhume_reference(&self, id: &Uuid) -> Option<&Reference> {
        self.reference.get(id)
    }

    /// Exorcise (remove) [`Reference`] from the store.
    ///
    pub fn exorcise_reference(&mut self, id: &Uuid) -> Option<Reference> {
        self.reference.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Reference>`.
    ///
    pub fn iter_reference(&self) -> impl Iterator<Item = &Reference> {
        self.reference.values()
    }

    /// Inter (insert) [`Statement`] into the store.
    ///
    pub fn inter_statement(&mut self, statement: Statement) {
        self.statement.insert(statement.id, statement);
    }

    /// Exhume (get) [`Statement`] from the store.
    ///
    pub fn exhume_statement(&self, id: &Uuid) -> Option<&Statement> {
        self.statement.get(id)
    }

    /// Exorcise (remove) [`Statement`] from the store.
    ///
    pub fn exorcise_statement(&mut self, id: &Uuid) -> Option<Statement> {
        self.statement.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Statement>`.
    ///
    pub fn iter_statement(&self) -> impl Iterator<Item = &Statement> {
        self.statement.values()
    }

    /// Inter (insert) [`Structure`] into the store.
    ///
    pub fn inter_structure(&mut self, structure: Structure) {
        self.structure.insert(structure.id, structure);
    }

    /// Exhume (get) [`Structure`] from the store.
    ///
    pub fn exhume_structure(&self, id: &Uuid) -> Option<&Structure> {
        self.structure.get(id)
    }

    /// Exorcise (remove) [`Structure`] from the store.
    ///
    pub fn exorcise_structure(&mut self, id: &Uuid) -> Option<Structure> {
        self.structure.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Structure>`.
    ///
    pub fn iter_structure(&self) -> impl Iterator<Item = &Structure> {
        self.structure.values()
    }

    /// Inter (insert) [`StructureField`] into the store.
    ///
    pub fn inter_structure_field(&mut self, structure_field: StructureField) {
        self.structure_field
            .insert(structure_field.id, structure_field);
    }

    /// Exhume (get) [`StructureField`] from the store.
    ///
    pub fn exhume_structure_field(&self, id: &Uuid) -> Option<&StructureField> {
        self.structure_field.get(id)
    }

    /// Exorcise (remove) [`StructureField`] from the store.
    ///
    pub fn exorcise_structure_field(&mut self, id: &Uuid) -> Option<StructureField> {
        self.structure_field.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructureField>`.
    ///
    pub fn iter_structure_field(&self) -> impl Iterator<Item = &StructureField> {
        self.structure_field.values()
    }

    /// Inter (insert) [`SymbolTable`] into the store.
    ///
    pub fn inter_symbol_table(&mut self, symbol_table: SymbolTable) {
        self.symbol_table.insert(symbol_table.id, symbol_table);
    }

    /// Exhume (get) [`SymbolTable`] from the store.
    ///
    pub fn exhume_symbol_table(&self, id: &Uuid) -> Option<&SymbolTable> {
        self.symbol_table.get(id)
    }

    /// Exorcise (remove) [`SymbolTable`] from the store.
    ///
    pub fn exorcise_symbol_table(&mut self, id: &Uuid) -> Option<SymbolTable> {
        self.symbol_table.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SymbolTable>`.
    ///
    pub fn iter_symbol_table(&self) -> impl Iterator<Item = &SymbolTable> {
        self.symbol_table.values()
    }

    /// Inter (insert) [`TimeStamp`] into the store.
    ///
    pub fn inter_time_stamp(&mut self, time_stamp: TimeStamp) {
        self.time_stamp.insert(time_stamp.id, time_stamp);
    }

    /// Exhume (get) [`TimeStamp`] from the store.
    ///
    pub fn exhume_time_stamp(&self, id: &Uuid) -> Option<&TimeStamp> {
        self.time_stamp.get(id)
    }

    /// Exorcise (remove) [`TimeStamp`] from the store.
    ///
    pub fn exorcise_time_stamp(&mut self, id: &Uuid) -> Option<TimeStamp> {
        self.time_stamp.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, TimeStamp>`.
    ///
    pub fn iter_time_stamp(&self) -> impl Iterator<Item = &TimeStamp> {
        self.time_stamp.values()
    }

    /// Inter (insert) [`XValue`] into the store.
    ///
    pub fn inter_x_value(&mut self, x_value: XValue) {
        self.x_value.insert(x_value.id, x_value);
    }

    /// Exhume (get) [`XValue`] from the store.
    ///
    pub fn exhume_x_value(&self, id: &Uuid) -> Option<&XValue> {
        self.x_value.get(id)
    }

    /// Exorcise (remove) [`XValue`] from the store.
    ///
    pub fn exorcise_x_value(&mut self, id: &Uuid) -> Option<XValue> {
        self.x_value.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XValue>`.
    ///
    pub fn iter_x_value(&self) -> impl Iterator<Item = &XValue> {
        self.x_value.values()
    }

    /// Inter (insert) [`Variable`] into the store.
    ///
    pub fn inter_variable(&mut self, variable: Variable) {
        self.variable.insert(variable.id, variable);
    }

    /// Exhume (get) [`Variable`] from the store.
    ///
    pub fn exhume_variable(&self, id: &Uuid) -> Option<&Variable> {
        self.variable.get(id)
    }

    /// Exorcise (remove) [`Variable`] from the store.
    ///
    pub fn exorcise_variable(&mut self, id: &Uuid) -> Option<Variable> {
        self.variable.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Variable>`.
    ///
    pub fn iter_variable(&self) -> impl Iterator<Item = &Variable> {
        self.variable.values()
    }

    /// Inter (insert) [`Visibility`] into the store.
    ///
    pub fn inter_visibility(&mut self, visibility: Visibility) {
        self.visibility.insert(visibility.id(), visibility);
    }

    /// Exhume (get) [`Visibility`] from the store.
    ///
    pub fn exhume_visibility(&self, id: &Uuid) -> Option<&Visibility> {
        self.visibility.get(id)
    }

    /// Exorcise (remove) [`Visibility`] from the store.
    ///
    pub fn exorcise_visibility(&mut self, id: &Uuid) -> Option<Visibility> {
        self.visibility.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Visibility>`.
    ///
    pub fn iter_visibility(&self) -> impl Iterator<Item = &Visibility> {
        self.visibility.values()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog_single-object-store-persistence"}}}
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
            for access in self.access.values() {
                let path = path.join(format!("{}.json", access.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &access)?;
            }
        }

        // Persist Block.
        {
            let path = path.join("block");
            fs::create_dir_all(&path)?;
            for block in self.block.values() {
                let path = path.join(format!("{}.json", block.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &block)?;
            }
        }

        // Persist Call.
        {
            let path = path.join("call");
            fs::create_dir_all(&path)?;
            for call in self.call.values() {
                let path = path.join(format!("{}.json", call.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &call)?;
            }
        }

        // Persist Constant.
        {
            let path = path.join("constant");
            fs::create_dir_all(&path)?;
            for constant in self.constant.values() {
                let path = path.join(format!("{}.json", constant.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &constant)?;
            }
        }

        // Persist Enumeration.
        {
            let path = path.join("enumeration");
            fs::create_dir_all(&path)?;
            for enumeration in self.enumeration.values() {
                let path = path.join(format!("{}.json", enumeration.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &enumeration)?;
            }
        }

        // Persist Enumeration Field.
        {
            let path = path.join("enumeration_field");
            fs::create_dir_all(&path)?;
            for enumeration_field in self.enumeration_field.values() {
                let path = path.join(format!("{}.json", enumeration_field.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &enumeration_field)?;
            }
        }

        // Persist Expression.
        {
            let path = path.join("expression");
            fs::create_dir_all(&path)?;
            for expression in self.expression.values() {
                let path = path.join(format!("{}.json", expression.id()));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &expression)?;
            }
        }

        // Persist Field.
        {
            let path = path.join("field");
            fs::create_dir_all(&path)?;
            for field in self.field.values() {
                let path = path.join(format!("{}.json", field.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &field)?;
            }
        }

        // Persist Function.
        {
            let path = path.join("function");
            fs::create_dir_all(&path)?;
            for function in self.function.values() {
                let path = path.join(format!("{}.json", function.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &function)?;
            }
        }

        // Persist Generation Unit.
        {
            let path = path.join("generation_unit");
            fs::create_dir_all(&path)?;
            for generation_unit in self.generation_unit.values() {
                let path = path.join(format!("{}.json", generation_unit.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &generation_unit)?;
            }
        }

        // Persist Grace Type.
        {
            let path = path.join("grace_type");
            fs::create_dir_all(&path)?;
            for grace_type in self.grace_type.values() {
                let path = path.join(format!("{}.json", grace_type.id()));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &grace_type)?;
            }
        }

        // Persist Item.
        {
            let path = path.join("item");
            fs::create_dir_all(&path)?;
            for item in self.item.values() {
                let path = path.join(format!("{}.json", item.id()));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &item)?;
            }
        }

        // Persist Let.
        {
            let path = path.join("x_let");
            fs::create_dir_all(&path)?;
            for x_let in self.x_let.values() {
                let path = path.join(format!("{}.json", x_let.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_let)?;
            }
        }

        // Persist Local.
        {
            let path = path.join("local");
            fs::create_dir_all(&path)?;
            for local in self.local.values() {
                let path = path.join(format!("{}.json", local.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &local)?;
            }
        }

        // Persist Object Method.
        {
            let path = path.join("object_method");
            fs::create_dir_all(&path)?;
            for object_method in self.object_method.values() {
                let path = path.join(format!("{}.json", object_method.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &object_method)?;
            }
        }

        // Persist Option.
        {
            let path = path.join("woog_option");
            fs::create_dir_all(&path)?;
            for woog_option in self.woog_option.values() {
                let path = path.join(format!("{}.json", woog_option.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &woog_option)?;
            }
        }

        // Persist Ownership.
        {
            let path = path.join("ownership");
            fs::create_dir_all(&path)?;
            for ownership in self.ownership.values() {
                let path = path.join(format!("{}.json", ownership.id()));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &ownership)?;
            }
        }

        // Persist Parameter.
        {
            let path = path.join("parameter");
            fs::create_dir_all(&path)?;
            for parameter in self.parameter.values() {
                let path = path.join(format!("{}.json", parameter.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &parameter)?;
            }
        }

        // Persist Reference.
        {
            let path = path.join("reference");
            fs::create_dir_all(&path)?;
            for reference in self.reference.values() {
                let path = path.join(format!("{}.json", reference.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &reference)?;
            }
        }

        // Persist Statement.
        {
            let path = path.join("statement");
            fs::create_dir_all(&path)?;
            for statement in self.statement.values() {
                let path = path.join(format!("{}.json", statement.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &statement)?;
            }
        }

        // Persist Structure.
        {
            let path = path.join("structure");
            fs::create_dir_all(&path)?;
            for structure in self.structure.values() {
                let path = path.join(format!("{}.json", structure.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &structure)?;
            }
        }

        // Persist Structure Field.
        {
            let path = path.join("structure_field");
            fs::create_dir_all(&path)?;
            for structure_field in self.structure_field.values() {
                let path = path.join(format!("{}.json", structure_field.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &structure_field)?;
            }
        }

        // Persist Symbol Table.
        {
            let path = path.join("symbol_table");
            fs::create_dir_all(&path)?;
            for symbol_table in self.symbol_table.values() {
                let path = path.join(format!("{}.json", symbol_table.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &symbol_table)?;
            }
        }

        // Persist Time Stamp.
        {
            let path = path.join("time_stamp");
            fs::create_dir_all(&path)?;
            for time_stamp in self.time_stamp.values() {
                let path = path.join(format!("{}.json", time_stamp.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &time_stamp)?;
            }
        }

        // Persist Value.
        {
            let path = path.join("x_value");
            fs::create_dir_all(&path)?;
            for x_value in self.x_value.values() {
                let path = path.join(format!("{}.json", x_value.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_value)?;
            }
        }

        // Persist Variable.
        {
            let path = path.join("variable");
            fs::create_dir_all(&path)?;
            for variable in self.variable.values() {
                let path = path.join(format!("{}.json", variable.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &variable)?;
            }
        }

        // Persist Visibility.
        {
            let path = path.join("visibility");
            fs::create_dir_all(&path)?;
            for visibility in self.visibility.values() {
                let path = path.join(format!("{}.json", visibility.id()));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &visibility)?;
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

        let mut store = Self::new();

        // Load Access.
        {
            let path = path.join("access");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let access: Access = serde_json::from_reader(reader)?;
                store.access.insert(access.id, access);
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
                let block: Block = serde_json::from_reader(reader)?;
                store.block.insert(block.id, block);
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
                let call: Call = serde_json::from_reader(reader)?;
                store.call.insert(call.id, call);
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
                let constant: Constant = serde_json::from_reader(reader)?;
                store.constant.insert(constant.id, constant);
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
                let enumeration: Enumeration = serde_json::from_reader(reader)?;
                store
                    .enumeration_id_by_name
                    .insert(enumeration.name.to_upper_camel_case(), enumeration.id);
                store.enumeration.insert(enumeration.id, enumeration);
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
                let enumeration_field: EnumerationField = serde_json::from_reader(reader)?;
                store
                    .enumeration_field
                    .insert(enumeration_field.id, enumeration_field);
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
                let expression: Expression = serde_json::from_reader(reader)?;
                store.expression.insert(expression.id(), expression);
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
                let field: Field = serde_json::from_reader(reader)?;
                store
                    .field_id_by_name
                    .insert(field.name.to_upper_camel_case(), field.id);
                store.field.insert(field.id, field);
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
                let function: Function = serde_json::from_reader(reader)?;
                store
                    .function_id_by_name
                    .insert(function.name.to_upper_camel_case(), function.id);
                store.function.insert(function.id, function);
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
                let generation_unit: GenerationUnit = serde_json::from_reader(reader)?;
                store
                    .generation_unit
                    .insert(generation_unit.id, generation_unit);
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
                let grace_type: GraceType = serde_json::from_reader(reader)?;
                store.grace_type.insert(grace_type.id(), grace_type);
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
                let item: Item = serde_json::from_reader(reader)?;
                store.item.insert(item.id(), item);
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
                let x_let: XLet = serde_json::from_reader(reader)?;
                store.x_let.insert(x_let.id, x_let);
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
                let local: Local = serde_json::from_reader(reader)?;
                store.local.insert(local.id, local);
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
                let object_method: ObjectMethod = serde_json::from_reader(reader)?;
                store.object_method.insert(object_method.id, object_method);
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
                let woog_option: WoogOption = serde_json::from_reader(reader)?;
                store.woog_option.insert(woog_option.id, woog_option);
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
                let ownership: Ownership = serde_json::from_reader(reader)?;
                store.ownership.insert(ownership.id(), ownership);
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
                let parameter: Parameter = serde_json::from_reader(reader)?;
                store.parameter.insert(parameter.id, parameter);
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
                let reference: Reference = serde_json::from_reader(reader)?;
                store.reference.insert(reference.id, reference);
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
                let statement: Statement = serde_json::from_reader(reader)?;
                store.statement.insert(statement.id, statement);
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
                let structure: Structure = serde_json::from_reader(reader)?;
                store.structure.insert(structure.id, structure);
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
                let structure_field: StructureField = serde_json::from_reader(reader)?;
                store
                    .structure_field
                    .insert(structure_field.id, structure_field);
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
                let symbol_table: SymbolTable = serde_json::from_reader(reader)?;
                store.symbol_table.insert(symbol_table.id, symbol_table);
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
                let time_stamp: TimeStamp = serde_json::from_reader(reader)?;
                store.time_stamp.insert(time_stamp.id, time_stamp);
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
                let x_value: XValue = serde_json::from_reader(reader)?;
                store.x_value.insert(x_value.id, x_value);
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
                let variable: Variable = serde_json::from_reader(reader)?;
                store.variable.insert(variable.id, variable);
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
                let visibility: Visibility = serde_json::from_reader(reader)?;
                store.visibility.insert(visibility.id(), visibility);
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
