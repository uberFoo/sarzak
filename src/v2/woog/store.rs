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
//! * [`Expression`]
//! * [`GenerationUnit`]
//! * [`GraceType`]
//! * [`XLet`]
//! * [`Local`]
//! * [`ObjectMethod`]
//! * [`WoogOption`]
//! * [`Ownership`]
//! * [`Parameter`]
//! * [`Reference`]
//! * [`Statement`]
//! * [`SymbolTable`]
//! * [`Value`]
//! * [`Variable`]
//! * [`Visibility`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog-object-store-definition"}}}
use std::collections::HashMap;
use std::{fs, io, path::Path, time::SystemTime};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v2::woog::types::{
    Access, Block, Call, Expression, GenerationUnit, GraceType, Local, ObjectMethod, Ownership,
    Parameter, Reference, Statement, SymbolTable, Value, Variable, Visibility, WoogOption, XLet,
    BORROWED, KRATE, LITERAL, MUTABLE, OWNED, PRIVATE, PUBLIC, TIME_STAMP,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    access: HashMap<Uuid, (Access, SystemTime)>,
    block: HashMap<Uuid, (Block, SystemTime)>,
    call: HashMap<Uuid, (Call, SystemTime)>,
    expression: HashMap<Uuid, (Expression, SystemTime)>,
    generation_unit: HashMap<Uuid, (GenerationUnit, SystemTime)>,
    grace_type: HashMap<Uuid, (GraceType, SystemTime)>,
    x_let: HashMap<Uuid, (XLet, SystemTime)>,
    local: HashMap<Uuid, (Local, SystemTime)>,
    object_method: HashMap<Uuid, (ObjectMethod, SystemTime)>,
    woog_option: HashMap<Uuid, (WoogOption, SystemTime)>,
    ownership: HashMap<Uuid, (Ownership, SystemTime)>,
    parameter: HashMap<Uuid, (Parameter, SystemTime)>,
    reference: HashMap<Uuid, (Reference, SystemTime)>,
    statement: HashMap<Uuid, (Statement, SystemTime)>,
    symbol_table: HashMap<Uuid, (SymbolTable, SystemTime)>,
    value: HashMap<Uuid, (Value, SystemTime)>,
    variable: HashMap<Uuid, (Variable, SystemTime)>,
    visibility: HashMap<Uuid, (Visibility, SystemTime)>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            access: HashMap::new(),
            block: HashMap::new(),
            call: HashMap::new(),
            expression: HashMap::new(),
            generation_unit: HashMap::new(),
            grace_type: HashMap::new(),
            x_let: HashMap::new(),
            local: HashMap::new(),
            object_method: HashMap::new(),
            woog_option: HashMap::new(),
            ownership: HashMap::new(),
            parameter: HashMap::new(),
            reference: HashMap::new(),
            statement: HashMap::new(),
            symbol_table: HashMap::new(),
            value: HashMap::new(),
            variable: HashMap::new(),
            visibility: HashMap::new(),
        };

        // Initialize Singleton Subtypes
        store.inter_expression(Expression::Literal(LITERAL));
        store.inter_grace_type(GraceType::TimeStamp(TIME_STAMP));
        store.inter_ownership(Ownership::Borrowed(BORROWED));
        store.inter_ownership(Ownership::Mutable(MUTABLE));
        store.inter_ownership(Ownership::Owned(OWNED));
        store.inter_visibility(Visibility::Krate(KRATE));
        store.inter_visibility(Visibility::Private(PRIVATE));
        store.inter_visibility(Visibility::Public(PUBLIC));

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog-object-store-methods"}}}
    /// Inter [`Access`] into the store.
    ///
    pub fn inter_access(&mut self, access: Access) {
        self.access.insert(access.id, (access, SystemTime::now()));
    }

    /// Exhume [`Access`] from the store.
    ///
    pub fn exhume_access(&self, id: &Uuid) -> Option<&Access> {
        self.access.get(id).map(|access| &access.0)
    }

    /// Exhume [`Access`] from the store — mutably.
    ///
    pub fn exhume_access_mut(&mut self, id: &Uuid) -> Option<&mut Access> {
        self.access.get_mut(id).map(|access| &mut access.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Access>`.
    ///
    pub fn iter_access(&self) -> impl Iterator<Item = &Access> {
        self.access.values().map(|access| &access.0)
    }

    /// Get the timestamp for Access.
    ///
    pub fn access_timestamp(&self, access: &Access) -> SystemTime {
        self.access
            .get(&access.id)
            .map(|access| access.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Block`] into the store.
    ///
    pub fn inter_block(&mut self, block: Block) {
        self.block.insert(block.id, (block, SystemTime::now()));
    }

    /// Exhume [`Block`] from the store.
    ///
    pub fn exhume_block(&self, id: &Uuid) -> Option<&Block> {
        self.block.get(id).map(|block| &block.0)
    }

    /// Exhume [`Block`] from the store — mutably.
    ///
    pub fn exhume_block_mut(&mut self, id: &Uuid) -> Option<&mut Block> {
        self.block.get_mut(id).map(|block| &mut block.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Block>`.
    ///
    pub fn iter_block(&self) -> impl Iterator<Item = &Block> {
        self.block.values().map(|block| &block.0)
    }

    /// Get the timestamp for Block.
    ///
    pub fn block_timestamp(&self, block: &Block) -> SystemTime {
        self.block
            .get(&block.id)
            .map(|block| block.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Call`] into the store.
    ///
    pub fn inter_call(&mut self, call: Call) {
        self.call.insert(call.id, (call, SystemTime::now()));
    }

    /// Exhume [`Call`] from the store.
    ///
    pub fn exhume_call(&self, id: &Uuid) -> Option<&Call> {
        self.call.get(id).map(|call| &call.0)
    }

    /// Exhume [`Call`] from the store — mutably.
    ///
    pub fn exhume_call_mut(&mut self, id: &Uuid) -> Option<&mut Call> {
        self.call.get_mut(id).map(|call| &mut call.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Call>`.
    ///
    pub fn iter_call(&self) -> impl Iterator<Item = &Call> {
        self.call.values().map(|call| &call.0)
    }

    /// Get the timestamp for Call.
    ///
    pub fn call_timestamp(&self, call: &Call) -> SystemTime {
        self.call
            .get(&call.id)
            .map(|call| call.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Expression`] into the store.
    ///
    pub fn inter_expression(&mut self, expression: Expression) {
        self.expression
            .insert(expression.id(), (expression, SystemTime::now()));
    }

    /// Exhume [`Expression`] from the store.
    ///
    pub fn exhume_expression(&self, id: &Uuid) -> Option<&Expression> {
        self.expression.get(id).map(|expression| &expression.0)
    }

    /// Exhume [`Expression`] from the store — mutably.
    ///
    pub fn exhume_expression_mut(&mut self, id: &Uuid) -> Option<&mut Expression> {
        self.expression
            .get_mut(id)
            .map(|expression| &mut expression.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Expression>`.
    ///
    pub fn iter_expression(&self) -> impl Iterator<Item = &Expression> {
        self.expression.values().map(|expression| &expression.0)
    }

    /// Get the timestamp for Expression.
    ///
    pub fn expression_timestamp(&self, expression: &Expression) -> SystemTime {
        self.expression
            .get(&expression.id())
            .map(|expression| expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`GenerationUnit`] into the store.
    ///
    pub fn inter_generation_unit(&mut self, generation_unit: GenerationUnit) {
        self.generation_unit
            .insert(generation_unit.id, (generation_unit, SystemTime::now()));
    }

    /// Exhume [`GenerationUnit`] from the store.
    ///
    pub fn exhume_generation_unit(&self, id: &Uuid) -> Option<&GenerationUnit> {
        self.generation_unit
            .get(id)
            .map(|generation_unit| &generation_unit.0)
    }

    /// Exhume [`GenerationUnit`] from the store — mutably.
    ///
    pub fn exhume_generation_unit_mut(&mut self, id: &Uuid) -> Option<&mut GenerationUnit> {
        self.generation_unit
            .get_mut(id)
            .map(|generation_unit| &mut generation_unit.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, GenerationUnit>`.
    ///
    pub fn iter_generation_unit(&self) -> impl Iterator<Item = &GenerationUnit> {
        self.generation_unit
            .values()
            .map(|generation_unit| &generation_unit.0)
    }

    /// Get the timestamp for GenerationUnit.
    ///
    pub fn generation_unit_timestamp(&self, generation_unit: &GenerationUnit) -> SystemTime {
        self.generation_unit
            .get(&generation_unit.id)
            .map(|generation_unit| generation_unit.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`GraceType`] into the store.
    ///
    pub fn inter_grace_type(&mut self, grace_type: GraceType) {
        self.grace_type
            .insert(grace_type.id(), (grace_type, SystemTime::now()));
    }

    /// Exhume [`GraceType`] from the store.
    ///
    pub fn exhume_grace_type(&self, id: &Uuid) -> Option<&GraceType> {
        self.grace_type.get(id).map(|grace_type| &grace_type.0)
    }

    /// Exhume [`GraceType`] from the store — mutably.
    ///
    pub fn exhume_grace_type_mut(&mut self, id: &Uuid) -> Option<&mut GraceType> {
        self.grace_type
            .get_mut(id)
            .map(|grace_type| &mut grace_type.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, GraceType>`.
    ///
    pub fn iter_grace_type(&self) -> impl Iterator<Item = &GraceType> {
        self.grace_type.values().map(|grace_type| &grace_type.0)
    }

    /// Get the timestamp for GraceType.
    ///
    pub fn grace_type_timestamp(&self, grace_type: &GraceType) -> SystemTime {
        self.grace_type
            .get(&grace_type.id())
            .map(|grace_type| grace_type.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`XLet`] into the store.
    ///
    pub fn inter_x_let(&mut self, x_let: XLet) {
        self.x_let.insert(x_let.id, (x_let, SystemTime::now()));
    }

    /// Exhume [`XLet`] from the store.
    ///
    pub fn exhume_x_let(&self, id: &Uuid) -> Option<&XLet> {
        self.x_let.get(id).map(|x_let| &x_let.0)
    }

    /// Exhume [`XLet`] from the store — mutably.
    ///
    pub fn exhume_x_let_mut(&mut self, id: &Uuid) -> Option<&mut XLet> {
        self.x_let.get_mut(id).map(|x_let| &mut x_let.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XLet>`.
    ///
    pub fn iter_x_let(&self) -> impl Iterator<Item = &XLet> {
        self.x_let.values().map(|x_let| &x_let.0)
    }

    /// Get the timestamp for XLet.
    ///
    pub fn x_let_timestamp(&self, x_let: &XLet) -> SystemTime {
        self.x_let
            .get(&x_let.id)
            .map(|x_let| x_let.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Local`] into the store.
    ///
    pub fn inter_local(&mut self, local: Local) {
        self.local.insert(local.id, (local, SystemTime::now()));
    }

    /// Exhume [`Local`] from the store.
    ///
    pub fn exhume_local(&self, id: &Uuid) -> Option<&Local> {
        self.local.get(id).map(|local| &local.0)
    }

    /// Exhume [`Local`] from the store — mutably.
    ///
    pub fn exhume_local_mut(&mut self, id: &Uuid) -> Option<&mut Local> {
        self.local.get_mut(id).map(|local| &mut local.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Local>`.
    ///
    pub fn iter_local(&self) -> impl Iterator<Item = &Local> {
        self.local.values().map(|local| &local.0)
    }

    /// Get the timestamp for Local.
    ///
    pub fn local_timestamp(&self, local: &Local) -> SystemTime {
        self.local
            .get(&local.id)
            .map(|local| local.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`ObjectMethod`] into the store.
    ///
    pub fn inter_object_method(&mut self, object_method: ObjectMethod) {
        self.object_method
            .insert(object_method.id, (object_method, SystemTime::now()));
    }

    /// Exhume [`ObjectMethod`] from the store.
    ///
    pub fn exhume_object_method(&self, id: &Uuid) -> Option<&ObjectMethod> {
        self.object_method
            .get(id)
            .map(|object_method| &object_method.0)
    }

    /// Exhume [`ObjectMethod`] from the store — mutably.
    ///
    pub fn exhume_object_method_mut(&mut self, id: &Uuid) -> Option<&mut ObjectMethod> {
        self.object_method
            .get_mut(id)
            .map(|object_method| &mut object_method.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ObjectMethod>`.
    ///
    pub fn iter_object_method(&self) -> impl Iterator<Item = &ObjectMethod> {
        self.object_method
            .values()
            .map(|object_method| &object_method.0)
    }

    /// Get the timestamp for ObjectMethod.
    ///
    pub fn object_method_timestamp(&self, object_method: &ObjectMethod) -> SystemTime {
        self.object_method
            .get(&object_method.id)
            .map(|object_method| object_method.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`WoogOption`] into the store.
    ///
    pub fn inter_woog_option(&mut self, woog_option: WoogOption) {
        self.woog_option
            .insert(woog_option.id, (woog_option, SystemTime::now()));
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
            .get(&woog_option.id)
            .map(|woog_option| woog_option.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Ownership`] into the store.
    ///
    pub fn inter_ownership(&mut self, ownership: Ownership) {
        self.ownership
            .insert(ownership.id(), (ownership, SystemTime::now()));
    }

    /// Exhume [`Ownership`] from the store.
    ///
    pub fn exhume_ownership(&self, id: &Uuid) -> Option<&Ownership> {
        self.ownership.get(id).map(|ownership| &ownership.0)
    }

    /// Exhume [`Ownership`] from the store — mutably.
    ///
    pub fn exhume_ownership_mut(&mut self, id: &Uuid) -> Option<&mut Ownership> {
        self.ownership.get_mut(id).map(|ownership| &mut ownership.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Ownership>`.
    ///
    pub fn iter_ownership(&self) -> impl Iterator<Item = &Ownership> {
        self.ownership.values().map(|ownership| &ownership.0)
    }

    /// Get the timestamp for Ownership.
    ///
    pub fn ownership_timestamp(&self, ownership: &Ownership) -> SystemTime {
        self.ownership
            .get(&ownership.id())
            .map(|ownership| ownership.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Parameter`] into the store.
    ///
    pub fn inter_parameter(&mut self, parameter: Parameter) {
        self.parameter
            .insert(parameter.id, (parameter, SystemTime::now()));
    }

    /// Exhume [`Parameter`] from the store.
    ///
    pub fn exhume_parameter(&self, id: &Uuid) -> Option<&Parameter> {
        self.parameter.get(id).map(|parameter| &parameter.0)
    }

    /// Exhume [`Parameter`] from the store — mutably.
    ///
    pub fn exhume_parameter_mut(&mut self, id: &Uuid) -> Option<&mut Parameter> {
        self.parameter.get_mut(id).map(|parameter| &mut parameter.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Parameter>`.
    ///
    pub fn iter_parameter(&self) -> impl Iterator<Item = &Parameter> {
        self.parameter.values().map(|parameter| &parameter.0)
    }

    /// Get the timestamp for Parameter.
    ///
    pub fn parameter_timestamp(&self, parameter: &Parameter) -> SystemTime {
        self.parameter
            .get(&parameter.id)
            .map(|parameter| parameter.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Reference`] into the store.
    ///
    pub fn inter_reference(&mut self, reference: Reference) {
        self.reference
            .insert(reference.id, (reference, SystemTime::now()));
    }

    /// Exhume [`Reference`] from the store.
    ///
    pub fn exhume_reference(&self, id: &Uuid) -> Option<&Reference> {
        self.reference.get(id).map(|reference| &reference.0)
    }

    /// Exhume [`Reference`] from the store — mutably.
    ///
    pub fn exhume_reference_mut(&mut self, id: &Uuid) -> Option<&mut Reference> {
        self.reference.get_mut(id).map(|reference| &mut reference.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Reference>`.
    ///
    pub fn iter_reference(&self) -> impl Iterator<Item = &Reference> {
        self.reference.values().map(|reference| &reference.0)
    }

    /// Get the timestamp for Reference.
    ///
    pub fn reference_timestamp(&self, reference: &Reference) -> SystemTime {
        self.reference
            .get(&reference.id)
            .map(|reference| reference.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Statement`] into the store.
    ///
    pub fn inter_statement(&mut self, statement: Statement) {
        self.statement
            .insert(statement.id, (statement, SystemTime::now()));
    }

    /// Exhume [`Statement`] from the store.
    ///
    pub fn exhume_statement(&self, id: &Uuid) -> Option<&Statement> {
        self.statement.get(id).map(|statement| &statement.0)
    }

    /// Exhume [`Statement`] from the store — mutably.
    ///
    pub fn exhume_statement_mut(&mut self, id: &Uuid) -> Option<&mut Statement> {
        self.statement.get_mut(id).map(|statement| &mut statement.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Statement>`.
    ///
    pub fn iter_statement(&self) -> impl Iterator<Item = &Statement> {
        self.statement.values().map(|statement| &statement.0)
    }

    /// Get the timestamp for Statement.
    ///
    pub fn statement_timestamp(&self, statement: &Statement) -> SystemTime {
        self.statement
            .get(&statement.id)
            .map(|statement| statement.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`SymbolTable`] into the store.
    ///
    pub fn inter_symbol_table(&mut self, symbol_table: SymbolTable) {
        self.symbol_table
            .insert(symbol_table.id, (symbol_table, SystemTime::now()));
    }

    /// Exhume [`SymbolTable`] from the store.
    ///
    pub fn exhume_symbol_table(&self, id: &Uuid) -> Option<&SymbolTable> {
        self.symbol_table
            .get(id)
            .map(|symbol_table| &symbol_table.0)
    }

    /// Exhume [`SymbolTable`] from the store — mutably.
    ///
    pub fn exhume_symbol_table_mut(&mut self, id: &Uuid) -> Option<&mut SymbolTable> {
        self.symbol_table
            .get_mut(id)
            .map(|symbol_table| &mut symbol_table.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SymbolTable>`.
    ///
    pub fn iter_symbol_table(&self) -> impl Iterator<Item = &SymbolTable> {
        self.symbol_table
            .values()
            .map(|symbol_table| &symbol_table.0)
    }

    /// Get the timestamp for SymbolTable.
    ///
    pub fn symbol_table_timestamp(&self, symbol_table: &SymbolTable) -> SystemTime {
        self.symbol_table
            .get(&symbol_table.id)
            .map(|symbol_table| symbol_table.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Value`] into the store.
    ///
    pub fn inter_value(&mut self, value: Value) {
        self.value.insert(value.id, (value, SystemTime::now()));
    }

    /// Exhume [`Value`] from the store.
    ///
    pub fn exhume_value(&self, id: &Uuid) -> Option<&Value> {
        self.value.get(id).map(|value| &value.0)
    }

    /// Exhume [`Value`] from the store — mutably.
    ///
    pub fn exhume_value_mut(&mut self, id: &Uuid) -> Option<&mut Value> {
        self.value.get_mut(id).map(|value| &mut value.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Value>`.
    ///
    pub fn iter_value(&self) -> impl Iterator<Item = &Value> {
        self.value.values().map(|value| &value.0)
    }

    /// Get the timestamp for Value.
    ///
    pub fn value_timestamp(&self, value: &Value) -> SystemTime {
        self.value
            .get(&value.id)
            .map(|value| value.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Variable`] into the store.
    ///
    pub fn inter_variable(&mut self, variable: Variable) {
        self.variable
            .insert(variable.id, (variable, SystemTime::now()));
    }

    /// Exhume [`Variable`] from the store.
    ///
    pub fn exhume_variable(&self, id: &Uuid) -> Option<&Variable> {
        self.variable.get(id).map(|variable| &variable.0)
    }

    /// Exhume [`Variable`] from the store — mutably.
    ///
    pub fn exhume_variable_mut(&mut self, id: &Uuid) -> Option<&mut Variable> {
        self.variable.get_mut(id).map(|variable| &mut variable.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Variable>`.
    ///
    pub fn iter_variable(&self) -> impl Iterator<Item = &Variable> {
        self.variable.values().map(|variable| &variable.0)
    }

    /// Get the timestamp for Variable.
    ///
    pub fn variable_timestamp(&self, variable: &Variable) -> SystemTime {
        self.variable
            .get(&variable.id)
            .map(|variable| variable.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Visibility`] into the store.
    ///
    pub fn inter_visibility(&mut self, visibility: Visibility) {
        self.visibility
            .insert(visibility.id(), (visibility, SystemTime::now()));
    }

    /// Exhume [`Visibility`] from the store.
    ///
    pub fn exhume_visibility(&self, id: &Uuid) -> Option<&Visibility> {
        self.visibility.get(id).map(|visibility| &visibility.0)
    }

    /// Exhume [`Visibility`] from the store — mutably.
    ///
    pub fn exhume_visibility_mut(&mut self, id: &Uuid) -> Option<&mut Visibility> {
        self.visibility
            .get_mut(id)
            .map(|visibility| &mut visibility.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Visibility>`.
    ///
    pub fn iter_visibility(&self) -> impl Iterator<Item = &Visibility> {
        self.visibility.values().map(|visibility| &visibility.0)
    }

    /// Get the timestamp for Visibility.
    ///
    pub fn visibility_timestamp(&self, visibility: &Visibility) -> SystemTime {
        self.visibility
            .get(&visibility.id())
            .map(|visibility| visibility.1)
            .unwrap_or(SystemTime::now())
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::woog-object-store-persistence"}}}
    /// Persist the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automaagic git integration as an option.
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        let path = path.join("woog.json");
        fs::create_dir_all(&path)?;

        // Persist Access.
        {
            let path = path.join("access");
            fs::create_dir_all(&path)?;
            for access_tuple in self.access.values() {
                let path = path.join(format!("{}.json", access_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Access, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != access_tuple.0 {
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
        }

        // Persist Block.
        {
            let path = path.join("block");
            fs::create_dir_all(&path)?;
            for block_tuple in self.block.values() {
                let path = path.join(format!("{}.json", block_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Block, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != block_tuple.0 {
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
        }

        // Persist Call.
        {
            let path = path.join("call");
            fs::create_dir_all(&path)?;
            for call_tuple in self.call.values() {
                let path = path.join(format!("{}.json", call_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Call, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != call_tuple.0 {
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
        }

        // Persist Expression.
        {
            let path = path.join("expression");
            fs::create_dir_all(&path)?;
            for expression_tuple in self.expression.values() {
                let path = path.join(format!("{}.json", expression_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Expression, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != expression_tuple.0 {
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
        }

        // Persist Generation Unit.
        {
            let path = path.join("generation_unit");
            fs::create_dir_all(&path)?;
            for generation_unit_tuple in self.generation_unit.values() {
                let path = path.join(format!("{}.json", generation_unit_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (GenerationUnit, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != generation_unit_tuple.0 {
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
        }

        // Persist Grace Type.
        {
            let path = path.join("grace_type");
            fs::create_dir_all(&path)?;
            for grace_type_tuple in self.grace_type.values() {
                let path = path.join(format!("{}.json", grace_type_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (GraceType, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != grace_type_tuple.0 {
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
        }

        // Persist Let.
        {
            let path = path.join("x_let");
            fs::create_dir_all(&path)?;
            for x_let_tuple in self.x_let.values() {
                let path = path.join(format!("{}.json", x_let_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (XLet, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != x_let_tuple.0 {
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
        }

        // Persist Local.
        {
            let path = path.join("local");
            fs::create_dir_all(&path)?;
            for local_tuple in self.local.values() {
                let path = path.join(format!("{}.json", local_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Local, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != local_tuple.0 {
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
        }

        // Persist Object Method.
        {
            let path = path.join("object_method");
            fs::create_dir_all(&path)?;
            for object_method_tuple in self.object_method.values() {
                let path = path.join(format!("{}.json", object_method_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (ObjectMethod, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != object_method_tuple.0 {
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
        }

        // Persist Option.
        {
            let path = path.join("woog_option");
            fs::create_dir_all(&path)?;
            for woog_option_tuple in self.woog_option.values() {
                let path = path.join(format!("{}.json", woog_option_tuple.0.id));
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
        }

        // Persist Ownership.
        {
            let path = path.join("ownership");
            fs::create_dir_all(&path)?;
            for ownership_tuple in self.ownership.values() {
                let path = path.join(format!("{}.json", ownership_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Ownership, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != ownership_tuple.0 {
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
        }

        // Persist Parameter.
        {
            let path = path.join("parameter");
            fs::create_dir_all(&path)?;
            for parameter_tuple in self.parameter.values() {
                let path = path.join(format!("{}.json", parameter_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Parameter, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != parameter_tuple.0 {
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
        }

        // Persist Reference.
        {
            let path = path.join("reference");
            fs::create_dir_all(&path)?;
            for reference_tuple in self.reference.values() {
                let path = path.join(format!("{}.json", reference_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Reference, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != reference_tuple.0 {
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
        }

        // Persist Statement.
        {
            let path = path.join("statement");
            fs::create_dir_all(&path)?;
            for statement_tuple in self.statement.values() {
                let path = path.join(format!("{}.json", statement_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Statement, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != statement_tuple.0 {
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
        }

        // Persist Symbol Table.
        {
            let path = path.join("symbol_table");
            fs::create_dir_all(&path)?;
            for symbol_table_tuple in self.symbol_table.values() {
                let path = path.join(format!("{}.json", symbol_table_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (SymbolTable, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != symbol_table_tuple.0 {
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
        }

        // Persist Value.
        {
            let path = path.join("value");
            fs::create_dir_all(&path)?;
            for value_tuple in self.value.values() {
                let path = path.join(format!("{}.json", value_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Value, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != value_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &value_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &value_tuple)?;
                }
            }
        }

        // Persist Variable.
        {
            let path = path.join("variable");
            fs::create_dir_all(&path)?;
            for variable_tuple in self.variable.values() {
                let path = path.join(format!("{}.json", variable_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Variable, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != variable_tuple.0 {
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
        }

        // Persist Visibility.
        {
            let path = path.join("visibility");
            fs::create_dir_all(&path)?;
            for visibility_tuple in self.visibility.values() {
                let path = path.join(format!("{}.json", visibility_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Visibility, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != visibility_tuple.0 {
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

        // Load Access.
        {
            let path = path.join("access");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let access: (Access, SystemTime) = serde_json::from_reader(reader)?;
                store.access.insert(access.0.id, access);
            }
        }

        // Load Block.
        {
            let path = path.join("block");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let block: (Block, SystemTime) = serde_json::from_reader(reader)?;
                store.block.insert(block.0.id, block);
            }
        }

        // Load Call.
        {
            let path = path.join("call");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let call: (Call, SystemTime) = serde_json::from_reader(reader)?;
                store.call.insert(call.0.id, call);
            }
        }

        // Load Expression.
        {
            let path = path.join("expression");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let expression: (Expression, SystemTime) = serde_json::from_reader(reader)?;
                store.expression.insert(expression.0.id(), expression);
            }
        }

        // Load Generation Unit.
        {
            let path = path.join("generation_unit");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let generation_unit: (GenerationUnit, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .generation_unit
                    .insert(generation_unit.0.id, generation_unit);
            }
        }

        // Load Grace Type.
        {
            let path = path.join("grace_type");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let grace_type: (GraceType, SystemTime) = serde_json::from_reader(reader)?;
                store.grace_type.insert(grace_type.0.id(), grace_type);
            }
        }

        // Load Let.
        {
            let path = path.join("x_let");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let x_let: (XLet, SystemTime) = serde_json::from_reader(reader)?;
                store.x_let.insert(x_let.0.id, x_let);
            }
        }

        // Load Local.
        {
            let path = path.join("local");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let local: (Local, SystemTime) = serde_json::from_reader(reader)?;
                store.local.insert(local.0.id, local);
            }
        }

        // Load Object Method.
        {
            let path = path.join("object_method");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let object_method: (ObjectMethod, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .object_method
                    .insert(object_method.0.id, object_method);
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
                store.woog_option.insert(woog_option.0.id, woog_option);
            }
        }

        // Load Ownership.
        {
            let path = path.join("ownership");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let ownership: (Ownership, SystemTime) = serde_json::from_reader(reader)?;
                store.ownership.insert(ownership.0.id(), ownership);
            }
        }

        // Load Parameter.
        {
            let path = path.join("parameter");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let parameter: (Parameter, SystemTime) = serde_json::from_reader(reader)?;
                store.parameter.insert(parameter.0.id, parameter);
            }
        }

        // Load Reference.
        {
            let path = path.join("reference");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let reference: (Reference, SystemTime) = serde_json::from_reader(reader)?;
                store.reference.insert(reference.0.id, reference);
            }
        }

        // Load Statement.
        {
            let path = path.join("statement");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let statement: (Statement, SystemTime) = serde_json::from_reader(reader)?;
                store.statement.insert(statement.0.id, statement);
            }
        }

        // Load Symbol Table.
        {
            let path = path.join("symbol_table");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let symbol_table: (SymbolTable, SystemTime) = serde_json::from_reader(reader)?;
                store.symbol_table.insert(symbol_table.0.id, symbol_table);
            }
        }

        // Load Value.
        {
            let path = path.join("value");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let value: (Value, SystemTime) = serde_json::from_reader(reader)?;
                store.value.insert(value.0.id, value);
            }
        }

        // Load Variable.
        {
            let path = path.join("variable");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let variable: (Variable, SystemTime) = serde_json::from_reader(reader)?;
                store.variable.insert(variable.0.id, variable);
            }
        }

        // Load Visibility.
        {
            let path = path.join("visibility");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let visibility: (Visibility, SystemTime) = serde_json::from_reader(reader)?;
                store.visibility.insert(visibility.0.id(), visibility);
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
