//! v2::lu_dog Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`Argument`]
//! * [`Block`]
//! * [`BooleanLiteral`]
//! * [`Call`]
//! * [`DwarfSourceFile`]
//! * [`Error`]
//! * [`ErrorExpression`]
//! * [`Expression`]
//! * [`ExpressionStatement`]
//! * [`Field`]
//! * [`FieldAccess`]
//! * [`FieldExpression`]
//! * [`FloatLiteral`]
//! * [`Function`]
//! * [`Implementation`]
//! * [`Import`]
//! * [`IntegerLiteral`]
//! * [`Item`]
//! * [`LetStatement`]
//! * [`List`]
//! * [`Literal`]
//! * [`LocalVariable`]
//! * [`MethodCall`]
//! * [`ZObjectStore`]
//! * [`WoogOption`]
//! * [`Parameter`]
//! * [`Print`]
//! * [`Reference`]
//! * [`ResultStatement`]
//! * [`ZSome`]
//! * [`Statement`]
//! * [`StaticMethodCall`]
//! * [`StringLiteral`]
//! * [`WoogStruct`]
//! * [`StructExpression`]
//! * [`Value`]
//! * [`ValueType`]
//! * [`Variable`]
//! * [`VariableExpression`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog-object-store-definition"}}}
use std::sync::{Arc, RwLock};
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
    Argument, Block, BooleanLiteral, Call, DwarfSourceFile, Error, ErrorExpression, Expression,
    ExpressionStatement, Field, FieldAccess, FieldExpression, FloatLiteral, Function,
    Implementation, Import, IntegerLiteral, Item, LetStatement, List, Literal, LocalVariable,
    MethodCall, Parameter, Print, Reference, ResultStatement, Statement, StaticMethodCall,
    StringLiteral, StructExpression, Value, ValueType, Variable, VariableExpression, WoogOption,
    WoogStruct, ZObjectStore, ZSome, EMPTY, FALSE_LITERAL, TRUE_LITERAL, UNKNOWN, UNKNOWN_VARIABLE,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    argument: HashMap<Uuid, (Arc<RwLock<Argument>>, SystemTime)>,
    block: HashMap<Uuid, (Arc<RwLock<Block>>, SystemTime)>,
    boolean_literal: HashMap<Uuid, (Arc<RwLock<BooleanLiteral>>, SystemTime)>,
    call: HashMap<Uuid, (Arc<RwLock<Call>>, SystemTime)>,
    dwarf_source_file: HashMap<Uuid, (Arc<RwLock<DwarfSourceFile>>, SystemTime)>,
    error: HashMap<Uuid, (Arc<RwLock<Error>>, SystemTime)>,
    error_expression: HashMap<Uuid, (Arc<RwLock<ErrorExpression>>, SystemTime)>,
    expression: HashMap<Uuid, (Arc<RwLock<Expression>>, SystemTime)>,
    expression_statement: HashMap<Uuid, (Arc<RwLock<ExpressionStatement>>, SystemTime)>,
    field: HashMap<Uuid, (Arc<RwLock<Field>>, SystemTime)>,
    field_access: HashMap<Uuid, (Arc<RwLock<FieldAccess>>, SystemTime)>,
    field_expression: HashMap<Uuid, (Arc<RwLock<FieldExpression>>, SystemTime)>,
    float_literal: HashMap<Uuid, (Arc<RwLock<FloatLiteral>>, SystemTime)>,
    function: HashMap<Uuid, (Arc<RwLock<Function>>, SystemTime)>,
    implementation: HashMap<Uuid, (Arc<RwLock<Implementation>>, SystemTime)>,
    import: HashMap<Uuid, (Arc<RwLock<Import>>, SystemTime)>,
    integer_literal: HashMap<Uuid, (Arc<RwLock<IntegerLiteral>>, SystemTime)>,
    item: HashMap<Uuid, (Arc<RwLock<Item>>, SystemTime)>,
    let_statement: HashMap<Uuid, (Arc<RwLock<LetStatement>>, SystemTime)>,
    list: HashMap<Uuid, (Arc<RwLock<List>>, SystemTime)>,
    literal: HashMap<Uuid, (Arc<RwLock<Literal>>, SystemTime)>,
    local_variable: HashMap<Uuid, (Arc<RwLock<LocalVariable>>, SystemTime)>,
    method_call: HashMap<Uuid, (Arc<RwLock<MethodCall>>, SystemTime)>,
    z_object_store: HashMap<Uuid, (Arc<RwLock<ZObjectStore>>, SystemTime)>,
    woog_option: HashMap<Uuid, (Arc<RwLock<WoogOption>>, SystemTime)>,
    parameter: HashMap<Uuid, (Arc<RwLock<Parameter>>, SystemTime)>,
    print: HashMap<Uuid, (Arc<RwLock<Print>>, SystemTime)>,
    reference: HashMap<Uuid, (Arc<RwLock<Reference>>, SystemTime)>,
    result_statement: HashMap<Uuid, (Arc<RwLock<ResultStatement>>, SystemTime)>,
    z_some: HashMap<Uuid, (Arc<RwLock<ZSome>>, SystemTime)>,
    statement: HashMap<Uuid, (Arc<RwLock<Statement>>, SystemTime)>,
    static_method_call: HashMap<Uuid, (Arc<RwLock<StaticMethodCall>>, SystemTime)>,
    string_literal: HashMap<Uuid, (Arc<RwLock<StringLiteral>>, SystemTime)>,
    woog_struct: HashMap<Uuid, (Arc<RwLock<WoogStruct>>, SystemTime)>,
    woog_struct_id_by_name: HashMap<String, (Uuid, SystemTime)>,
    struct_expression: HashMap<Uuid, (Arc<RwLock<StructExpression>>, SystemTime)>,
    value: HashMap<Uuid, (Arc<RwLock<Value>>, SystemTime)>,
    value_type: HashMap<Uuid, (Arc<RwLock<ValueType>>, SystemTime)>,
    variable: HashMap<Uuid, (Arc<RwLock<Variable>>, SystemTime)>,
    variable_expression: HashMap<Uuid, (Arc<RwLock<VariableExpression>>, SystemTime)>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            argument: HashMap::default(),
            block: HashMap::default(),
            boolean_literal: HashMap::default(),
            call: HashMap::default(),
            dwarf_source_file: HashMap::default(),
            error: HashMap::default(),
            error_expression: HashMap::default(),
            expression: HashMap::default(),
            expression_statement: HashMap::default(),
            field: HashMap::default(),
            field_access: HashMap::default(),
            field_expression: HashMap::default(),
            float_literal: HashMap::default(),
            function: HashMap::default(),
            implementation: HashMap::default(),
            import: HashMap::default(),
            integer_literal: HashMap::default(),
            item: HashMap::default(),
            let_statement: HashMap::default(),
            list: HashMap::default(),
            literal: HashMap::default(),
            local_variable: HashMap::default(),
            method_call: HashMap::default(),
            z_object_store: HashMap::default(),
            woog_option: HashMap::default(),
            parameter: HashMap::default(),
            print: HashMap::default(),
            reference: HashMap::default(),
            result_statement: HashMap::default(),
            z_some: HashMap::default(),
            statement: HashMap::default(),
            static_method_call: HashMap::default(),
            string_literal: HashMap::default(),
            woog_struct: HashMap::default(),
            woog_struct_id_by_name: HashMap::default(),
            struct_expression: HashMap::default(),
            value: HashMap::default(),
            value_type: HashMap::default(),
            variable: HashMap::default(),
            variable_expression: HashMap::default(),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥
        store.inter_boolean_literal(Arc::new(RwLock::new(BooleanLiteral::FalseLiteral(
            FALSE_LITERAL,
        ))));
        store.inter_boolean_literal(Arc::new(RwLock::new(BooleanLiteral::TrueLiteral(
            TRUE_LITERAL,
        ))));
        store.inter_error(Arc::new(RwLock::new(Error::UnknownVariable(
            UNKNOWN_VARIABLE,
        ))));
        store.inter_expression(Arc::new(RwLock::new(Expression::Literal(
            Literal::BooleanLiteral(BooleanLiteral::FalseLiteral(FALSE_LITERAL).id()).id(),
        ))));
        store.inter_expression(Arc::new(RwLock::new(Expression::Literal(
            Literal::BooleanLiteral(BooleanLiteral::TrueLiteral(TRUE_LITERAL).id()).id(),
        ))));
        store.inter_literal(Arc::new(RwLock::new(Literal::BooleanLiteral(
            BooleanLiteral::FalseLiteral(FALSE_LITERAL).id(),
        ))));
        store.inter_literal(Arc::new(RwLock::new(Literal::BooleanLiteral(
            BooleanLiteral::TrueLiteral(TRUE_LITERAL).id(),
        ))));
        store.inter_value_type(Arc::new(RwLock::new(ValueType::Empty(EMPTY))));
        store.inter_value_type(Arc::new(RwLock::new(ValueType::Error(
            Error::UnknownVariable(UNKNOWN_VARIABLE).id(),
        ))));
        store.inter_value_type(Arc::new(RwLock::new(ValueType::Unknown(UNKNOWN))));

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog-object-store-methods"}}}
    /// Inter [`Argument`] into the store.
    ///
    pub fn inter_argument(&mut self, argument: Arc<RwLock<Argument>>) {
        let read = argument.read().unwrap();
        self.argument
            .insert(read.id, (argument.clone(), SystemTime::now()));
    }

    /// Exhume [`Argument`] from the store.
    ///
    pub fn exhume_argument(&self, id: &Uuid) -> Option<Arc<RwLock<Argument>>> {
        self.argument.get(id).map(|argument| argument.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Argument>`.
    ///
    pub fn iter_argument(&self) -> impl Iterator<Item = Arc<RwLock<Argument>>> + '_ {
        self.argument.values().map(|argument| argument.0.clone())
    }

    /// Get the timestamp for Argument.
    ///
    pub fn argument_timestamp(&self, argument: &Argument) -> SystemTime {
        self.argument
            .get(&argument.id)
            .map(|argument| argument.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Block`] into the store.
    ///
    pub fn inter_block(&mut self, block: Arc<RwLock<Block>>) {
        let read = block.read().unwrap();
        self.block
            .insert(read.id, (block.clone(), SystemTime::now()));
    }

    /// Exhume [`Block`] from the store.
    ///
    pub fn exhume_block(&self, id: &Uuid) -> Option<Arc<RwLock<Block>>> {
        self.block.get(id).map(|block| block.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Block>`.
    ///
    pub fn iter_block(&self) -> impl Iterator<Item = Arc<RwLock<Block>>> + '_ {
        self.block.values().map(|block| block.0.clone())
    }

    /// Get the timestamp for Block.
    ///
    pub fn block_timestamp(&self, block: &Block) -> SystemTime {
        self.block
            .get(&block.id)
            .map(|block| block.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`BooleanLiteral`] into the store.
    ///
    pub fn inter_boolean_literal(&mut self, boolean_literal: Arc<RwLock<BooleanLiteral>>) {
        let read = boolean_literal.read().unwrap();
        self.boolean_literal
            .insert(read.id(), (boolean_literal.clone(), SystemTime::now()));
    }

    /// Exhume [`BooleanLiteral`] from the store.
    ///
    pub fn exhume_boolean_literal(&self, id: &Uuid) -> Option<Arc<RwLock<BooleanLiteral>>> {
        self.boolean_literal
            .get(id)
            .map(|boolean_literal| boolean_literal.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanLiteral>`.
    ///
    pub fn iter_boolean_literal(&self) -> impl Iterator<Item = Arc<RwLock<BooleanLiteral>>> + '_ {
        self.boolean_literal
            .values()
            .map(|boolean_literal| boolean_literal.0.clone())
    }

    /// Get the timestamp for BooleanLiteral.
    ///
    pub fn boolean_literal_timestamp(&self, boolean_literal: &BooleanLiteral) -> SystemTime {
        self.boolean_literal
            .get(&boolean_literal.id())
            .map(|boolean_literal| boolean_literal.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Call`] into the store.
    ///
    pub fn inter_call(&mut self, call: Arc<RwLock<Call>>) {
        let read = call.read().unwrap();
        self.call.insert(read.id, (call.clone(), SystemTime::now()));
    }

    /// Exhume [`Call`] from the store.
    ///
    pub fn exhume_call(&self, id: &Uuid) -> Option<Arc<RwLock<Call>>> {
        self.call.get(id).map(|call| call.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Call>`.
    ///
    pub fn iter_call(&self) -> impl Iterator<Item = Arc<RwLock<Call>>> + '_ {
        self.call.values().map(|call| call.0.clone())
    }

    /// Get the timestamp for Call.
    ///
    pub fn call_timestamp(&self, call: &Call) -> SystemTime {
        self.call
            .get(&call.id)
            .map(|call| call.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`DwarfSourceFile`] into the store.
    ///
    pub fn inter_dwarf_source_file(&mut self, dwarf_source_file: Arc<RwLock<DwarfSourceFile>>) {
        let read = dwarf_source_file.read().unwrap();
        self.dwarf_source_file
            .insert(read.id, (dwarf_source_file.clone(), SystemTime::now()));
    }

    /// Exhume [`DwarfSourceFile`] from the store.
    ///
    pub fn exhume_dwarf_source_file(&self, id: &Uuid) -> Option<Arc<RwLock<DwarfSourceFile>>> {
        self.dwarf_source_file
            .get(id)
            .map(|dwarf_source_file| dwarf_source_file.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, DwarfSourceFile>`.
    ///
    pub fn iter_dwarf_source_file(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<DwarfSourceFile>>> + '_ {
        self.dwarf_source_file
            .values()
            .map(|dwarf_source_file| dwarf_source_file.0.clone())
    }

    /// Get the timestamp for DwarfSourceFile.
    ///
    pub fn dwarf_source_file_timestamp(&self, dwarf_source_file: &DwarfSourceFile) -> SystemTime {
        self.dwarf_source_file
            .get(&dwarf_source_file.id)
            .map(|dwarf_source_file| dwarf_source_file.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Error`] into the store.
    ///
    pub fn inter_error(&mut self, error: Arc<RwLock<Error>>) {
        let read = error.read().unwrap();
        self.error
            .insert(read.id(), (error.clone(), SystemTime::now()));
    }

    /// Exhume [`Error`] from the store.
    ///
    pub fn exhume_error(&self, id: &Uuid) -> Option<Arc<RwLock<Error>>> {
        self.error.get(id).map(|error| error.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Error>`.
    ///
    pub fn iter_error(&self) -> impl Iterator<Item = Arc<RwLock<Error>>> + '_ {
        self.error.values().map(|error| error.0.clone())
    }

    /// Get the timestamp for Error.
    ///
    pub fn error_timestamp(&self, error: &Error) -> SystemTime {
        self.error
            .get(&error.id())
            .map(|error| error.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`ErrorExpression`] into the store.
    ///
    pub fn inter_error_expression(&mut self, error_expression: Arc<RwLock<ErrorExpression>>) {
        let read = error_expression.read().unwrap();
        self.error_expression
            .insert(read.id, (error_expression.clone(), SystemTime::now()));
    }

    /// Exhume [`ErrorExpression`] from the store.
    ///
    pub fn exhume_error_expression(&self, id: &Uuid) -> Option<Arc<RwLock<ErrorExpression>>> {
        self.error_expression
            .get(id)
            .map(|error_expression| error_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ErrorExpression>`.
    ///
    pub fn iter_error_expression(&self) -> impl Iterator<Item = Arc<RwLock<ErrorExpression>>> + '_ {
        self.error_expression
            .values()
            .map(|error_expression| error_expression.0.clone())
    }

    /// Get the timestamp for ErrorExpression.
    ///
    pub fn error_expression_timestamp(&self, error_expression: &ErrorExpression) -> SystemTime {
        self.error_expression
            .get(&error_expression.id)
            .map(|error_expression| error_expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Expression`] into the store.
    ///
    pub fn inter_expression(&mut self, expression: Arc<RwLock<Expression>>) {
        let read = expression.read().unwrap();
        self.expression
            .insert(read.id(), (expression.clone(), SystemTime::now()));
    }

    /// Exhume [`Expression`] from the store.
    ///
    pub fn exhume_expression(&self, id: &Uuid) -> Option<Arc<RwLock<Expression>>> {
        self.expression
            .get(id)
            .map(|expression| expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Expression>`.
    ///
    pub fn iter_expression(&self) -> impl Iterator<Item = Arc<RwLock<Expression>>> + '_ {
        self.expression
            .values()
            .map(|expression| expression.0.clone())
    }

    /// Get the timestamp for Expression.
    ///
    pub fn expression_timestamp(&self, expression: &Expression) -> SystemTime {
        self.expression
            .get(&expression.id())
            .map(|expression| expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`ExpressionStatement`] into the store.
    ///
    pub fn inter_expression_statement(
        &mut self,
        expression_statement: Arc<RwLock<ExpressionStatement>>,
    ) {
        let read = expression_statement.read().unwrap();
        self.expression_statement
            .insert(read.id, (expression_statement.clone(), SystemTime::now()));
    }

    /// Exhume [`ExpressionStatement`] from the store.
    ///
    pub fn exhume_expression_statement(
        &self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<ExpressionStatement>>> {
        self.expression_statement
            .get(id)
            .map(|expression_statement| expression_statement.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ExpressionStatement>`.
    ///
    pub fn iter_expression_statement(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<ExpressionStatement>>> + '_ {
        self.expression_statement
            .values()
            .map(|expression_statement| expression_statement.0.clone())
    }

    /// Get the timestamp for ExpressionStatement.
    ///
    pub fn expression_statement_timestamp(
        &self,
        expression_statement: &ExpressionStatement,
    ) -> SystemTime {
        self.expression_statement
            .get(&expression_statement.id)
            .map(|expression_statement| expression_statement.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Field`] into the store.
    ///
    pub fn inter_field(&mut self, field: Arc<RwLock<Field>>) {
        let read = field.read().unwrap();
        self.field
            .insert(read.id, (field.clone(), SystemTime::now()));
    }

    /// Exhume [`Field`] from the store.
    ///
    pub fn exhume_field(&self, id: &Uuid) -> Option<Arc<RwLock<Field>>> {
        self.field.get(id).map(|field| field.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Field>`.
    ///
    pub fn iter_field(&self) -> impl Iterator<Item = Arc<RwLock<Field>>> + '_ {
        self.field.values().map(|field| field.0.clone())
    }

    /// Get the timestamp for Field.
    ///
    pub fn field_timestamp(&self, field: &Field) -> SystemTime {
        self.field
            .get(&field.id)
            .map(|field| field.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`FieldAccess`] into the store.
    ///
    pub fn inter_field_access(&mut self, field_access: Arc<RwLock<FieldAccess>>) {
        let read = field_access.read().unwrap();
        self.field_access
            .insert(read.id, (field_access.clone(), SystemTime::now()));
    }

    /// Exhume [`FieldAccess`] from the store.
    ///
    pub fn exhume_field_access(&self, id: &Uuid) -> Option<Arc<RwLock<FieldAccess>>> {
        self.field_access
            .get(id)
            .map(|field_access| field_access.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccess>`.
    ///
    pub fn iter_field_access(&self) -> impl Iterator<Item = Arc<RwLock<FieldAccess>>> + '_ {
        self.field_access
            .values()
            .map(|field_access| field_access.0.clone())
    }

    /// Get the timestamp for FieldAccess.
    ///
    pub fn field_access_timestamp(&self, field_access: &FieldAccess) -> SystemTime {
        self.field_access
            .get(&field_access.id)
            .map(|field_access| field_access.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`FieldExpression`] into the store.
    ///
    pub fn inter_field_expression(&mut self, field_expression: Arc<RwLock<FieldExpression>>) {
        let read = field_expression.read().unwrap();
        self.field_expression
            .insert(read.id, (field_expression.clone(), SystemTime::now()));
    }

    /// Exhume [`FieldExpression`] from the store.
    ///
    pub fn exhume_field_expression(&self, id: &Uuid) -> Option<Arc<RwLock<FieldExpression>>> {
        self.field_expression
            .get(id)
            .map(|field_expression| field_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldExpression>`.
    ///
    pub fn iter_field_expression(&self) -> impl Iterator<Item = Arc<RwLock<FieldExpression>>> + '_ {
        self.field_expression
            .values()
            .map(|field_expression| field_expression.0.clone())
    }

    /// Get the timestamp for FieldExpression.
    ///
    pub fn field_expression_timestamp(&self, field_expression: &FieldExpression) -> SystemTime {
        self.field_expression
            .get(&field_expression.id)
            .map(|field_expression| field_expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`FloatLiteral`] into the store.
    ///
    pub fn inter_float_literal(&mut self, float_literal: Arc<RwLock<FloatLiteral>>) {
        let read = float_literal.read().unwrap();
        self.float_literal
            .insert(read.id, (float_literal.clone(), SystemTime::now()));
    }

    /// Exhume [`FloatLiteral`] from the store.
    ///
    pub fn exhume_float_literal(&self, id: &Uuid) -> Option<Arc<RwLock<FloatLiteral>>> {
        self.float_literal
            .get(id)
            .map(|float_literal| float_literal.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FloatLiteral>`.
    ///
    pub fn iter_float_literal(&self) -> impl Iterator<Item = Arc<RwLock<FloatLiteral>>> + '_ {
        self.float_literal
            .values()
            .map(|float_literal| float_literal.0.clone())
    }

    /// Get the timestamp for FloatLiteral.
    ///
    pub fn float_literal_timestamp(&self, float_literal: &FloatLiteral) -> SystemTime {
        self.float_literal
            .get(&float_literal.id)
            .map(|float_literal| float_literal.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Function`] into the store.
    ///
    pub fn inter_function(&mut self, function: Arc<RwLock<Function>>) {
        let read = function.read().unwrap();
        self.function
            .insert(read.id, (function.clone(), SystemTime::now()));
    }

    /// Exhume [`Function`] from the store.
    ///
    pub fn exhume_function(&self, id: &Uuid) -> Option<Arc<RwLock<Function>>> {
        self.function.get(id).map(|function| function.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Function>`.
    ///
    pub fn iter_function(&self) -> impl Iterator<Item = Arc<RwLock<Function>>> + '_ {
        self.function.values().map(|function| function.0.clone())
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
    pub fn inter_implementation(&mut self, implementation: Arc<RwLock<Implementation>>) {
        let read = implementation.read().unwrap();
        self.implementation
            .insert(read.id, (implementation.clone(), SystemTime::now()));
    }

    /// Exhume [`Implementation`] from the store.
    ///
    pub fn exhume_implementation(&self, id: &Uuid) -> Option<Arc<RwLock<Implementation>>> {
        self.implementation
            .get(id)
            .map(|implementation| implementation.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Implementation>`.
    ///
    pub fn iter_implementation(&self) -> impl Iterator<Item = Arc<RwLock<Implementation>>> + '_ {
        self.implementation
            .values()
            .map(|implementation| implementation.0.clone())
    }

    /// Get the timestamp for Implementation.
    ///
    pub fn implementation_timestamp(&self, implementation: &Implementation) -> SystemTime {
        self.implementation
            .get(&implementation.id)
            .map(|implementation| implementation.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Import`] into the store.
    ///
    pub fn inter_import(&mut self, import: Arc<RwLock<Import>>) {
        let read = import.read().unwrap();
        self.import
            .insert(read.id, (import.clone(), SystemTime::now()));
    }

    /// Exhume [`Import`] from the store.
    ///
    pub fn exhume_import(&self, id: &Uuid) -> Option<Arc<RwLock<Import>>> {
        self.import.get(id).map(|import| import.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Import>`.
    ///
    pub fn iter_import(&self) -> impl Iterator<Item = Arc<RwLock<Import>>> + '_ {
        self.import.values().map(|import| import.0.clone())
    }

    /// Get the timestamp for Import.
    ///
    pub fn import_timestamp(&self, import: &Import) -> SystemTime {
        self.import
            .get(&import.id)
            .map(|import| import.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`IntegerLiteral`] into the store.
    ///
    pub fn inter_integer_literal(&mut self, integer_literal: Arc<RwLock<IntegerLiteral>>) {
        let read = integer_literal.read().unwrap();
        self.integer_literal
            .insert(read.id, (integer_literal.clone(), SystemTime::now()));
    }

    /// Exhume [`IntegerLiteral`] from the store.
    ///
    pub fn exhume_integer_literal(&self, id: &Uuid) -> Option<Arc<RwLock<IntegerLiteral>>> {
        self.integer_literal
            .get(id)
            .map(|integer_literal| integer_literal.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, IntegerLiteral>`.
    ///
    pub fn iter_integer_literal(&self) -> impl Iterator<Item = Arc<RwLock<IntegerLiteral>>> + '_ {
        self.integer_literal
            .values()
            .map(|integer_literal| integer_literal.0.clone())
    }

    /// Get the timestamp for IntegerLiteral.
    ///
    pub fn integer_literal_timestamp(&self, integer_literal: &IntegerLiteral) -> SystemTime {
        self.integer_literal
            .get(&integer_literal.id)
            .map(|integer_literal| integer_literal.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Item`] into the store.
    ///
    pub fn inter_item(&mut self, item: Arc<RwLock<Item>>) {
        let read = item.read().unwrap();
        self.item.insert(read.id, (item.clone(), SystemTime::now()));
    }

    /// Exhume [`Item`] from the store.
    ///
    pub fn exhume_item(&self, id: &Uuid) -> Option<Arc<RwLock<Item>>> {
        self.item.get(id).map(|item| item.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Item>`.
    ///
    pub fn iter_item(&self) -> impl Iterator<Item = Arc<RwLock<Item>>> + '_ {
        self.item.values().map(|item| item.0.clone())
    }

    /// Get the timestamp for Item.
    ///
    pub fn item_timestamp(&self, item: &Item) -> SystemTime {
        self.item
            .get(&item.id)
            .map(|item| item.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`LetStatement`] into the store.
    ///
    pub fn inter_let_statement(&mut self, let_statement: Arc<RwLock<LetStatement>>) {
        let read = let_statement.read().unwrap();
        self.let_statement
            .insert(read.id, (let_statement.clone(), SystemTime::now()));
    }

    /// Exhume [`LetStatement`] from the store.
    ///
    pub fn exhume_let_statement(&self, id: &Uuid) -> Option<Arc<RwLock<LetStatement>>> {
        self.let_statement
            .get(id)
            .map(|let_statement| let_statement.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LetStatement>`.
    ///
    pub fn iter_let_statement(&self) -> impl Iterator<Item = Arc<RwLock<LetStatement>>> + '_ {
        self.let_statement
            .values()
            .map(|let_statement| let_statement.0.clone())
    }

    /// Get the timestamp for LetStatement.
    ///
    pub fn let_statement_timestamp(&self, let_statement: &LetStatement) -> SystemTime {
        self.let_statement
            .get(&let_statement.id)
            .map(|let_statement| let_statement.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`List`] into the store.
    ///
    pub fn inter_list(&mut self, list: Arc<RwLock<List>>) {
        let read = list.read().unwrap();
        self.list.insert(read.id, (list.clone(), SystemTime::now()));
    }

    /// Exhume [`List`] from the store.
    ///
    pub fn exhume_list(&self, id: &Uuid) -> Option<Arc<RwLock<List>>> {
        self.list.get(id).map(|list| list.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, List>`.
    ///
    pub fn iter_list(&self) -> impl Iterator<Item = Arc<RwLock<List>>> + '_ {
        self.list.values().map(|list| list.0.clone())
    }

    /// Get the timestamp for List.
    ///
    pub fn list_timestamp(&self, list: &List) -> SystemTime {
        self.list
            .get(&list.id)
            .map(|list| list.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Literal`] into the store.
    ///
    pub fn inter_literal(&mut self, literal: Arc<RwLock<Literal>>) {
        let read = literal.read().unwrap();
        self.literal
            .insert(read.id(), (literal.clone(), SystemTime::now()));
    }

    /// Exhume [`Literal`] from the store.
    ///
    pub fn exhume_literal(&self, id: &Uuid) -> Option<Arc<RwLock<Literal>>> {
        self.literal.get(id).map(|literal| literal.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Literal>`.
    ///
    pub fn iter_literal(&self) -> impl Iterator<Item = Arc<RwLock<Literal>>> + '_ {
        self.literal.values().map(|literal| literal.0.clone())
    }

    /// Get the timestamp for Literal.
    ///
    pub fn literal_timestamp(&self, literal: &Literal) -> SystemTime {
        self.literal
            .get(&literal.id())
            .map(|literal| literal.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`LocalVariable`] into the store.
    ///
    pub fn inter_local_variable(&mut self, local_variable: Arc<RwLock<LocalVariable>>) {
        let read = local_variable.read().unwrap();
        self.local_variable
            .insert(read.id, (local_variable.clone(), SystemTime::now()));
    }

    /// Exhume [`LocalVariable`] from the store.
    ///
    pub fn exhume_local_variable(&self, id: &Uuid) -> Option<Arc<RwLock<LocalVariable>>> {
        self.local_variable
            .get(id)
            .map(|local_variable| local_variable.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LocalVariable>`.
    ///
    pub fn iter_local_variable(&self) -> impl Iterator<Item = Arc<RwLock<LocalVariable>>> + '_ {
        self.local_variable
            .values()
            .map(|local_variable| local_variable.0.clone())
    }

    /// Get the timestamp for LocalVariable.
    ///
    pub fn local_variable_timestamp(&self, local_variable: &LocalVariable) -> SystemTime {
        self.local_variable
            .get(&local_variable.id)
            .map(|local_variable| local_variable.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`MethodCall`] into the store.
    ///
    pub fn inter_method_call(&mut self, method_call: Arc<RwLock<MethodCall>>) {
        let read = method_call.read().unwrap();
        self.method_call
            .insert(read.id, (method_call.clone(), SystemTime::now()));
    }

    /// Exhume [`MethodCall`] from the store.
    ///
    pub fn exhume_method_call(&self, id: &Uuid) -> Option<Arc<RwLock<MethodCall>>> {
        self.method_call
            .get(id)
            .map(|method_call| method_call.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, MethodCall>`.
    ///
    pub fn iter_method_call(&self) -> impl Iterator<Item = Arc<RwLock<MethodCall>>> + '_ {
        self.method_call
            .values()
            .map(|method_call| method_call.0.clone())
    }

    /// Get the timestamp for MethodCall.
    ///
    pub fn method_call_timestamp(&self, method_call: &MethodCall) -> SystemTime {
        self.method_call
            .get(&method_call.id)
            .map(|method_call| method_call.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`ZObjectStore`] into the store.
    ///
    pub fn inter_z_object_store(&mut self, z_object_store: Arc<RwLock<ZObjectStore>>) {
        let read = z_object_store.read().unwrap();
        self.z_object_store
            .insert(read.id, (z_object_store.clone(), SystemTime::now()));
    }

    /// Exhume [`ZObjectStore`] from the store.
    ///
    pub fn exhume_z_object_store(&self, id: &Uuid) -> Option<Arc<RwLock<ZObjectStore>>> {
        self.z_object_store
            .get(id)
            .map(|z_object_store| z_object_store.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ZObjectStore>`.
    ///
    pub fn iter_z_object_store(&self) -> impl Iterator<Item = Arc<RwLock<ZObjectStore>>> + '_ {
        self.z_object_store
            .values()
            .map(|z_object_store| z_object_store.0.clone())
    }

    /// Get the timestamp for ZObjectStore.
    ///
    pub fn z_object_store_timestamp(&self, z_object_store: &ZObjectStore) -> SystemTime {
        self.z_object_store
            .get(&z_object_store.id)
            .map(|z_object_store| z_object_store.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`WoogOption`] into the store.
    ///
    pub fn inter_woog_option(&mut self, woog_option: Arc<RwLock<WoogOption>>) {
        let read = woog_option.read().unwrap();
        self.woog_option
            .insert(read.id, (woog_option.clone(), SystemTime::now()));
    }

    /// Exhume [`WoogOption`] from the store.
    ///
    pub fn exhume_woog_option(&self, id: &Uuid) -> Option<Arc<RwLock<WoogOption>>> {
        self.woog_option
            .get(id)
            .map(|woog_option| woog_option.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, WoogOption>`.
    ///
    pub fn iter_woog_option(&self) -> impl Iterator<Item = Arc<RwLock<WoogOption>>> + '_ {
        self.woog_option
            .values()
            .map(|woog_option| woog_option.0.clone())
    }

    /// Get the timestamp for WoogOption.
    ///
    pub fn woog_option_timestamp(&self, woog_option: &WoogOption) -> SystemTime {
        self.woog_option
            .get(&woog_option.id)
            .map(|woog_option| woog_option.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Parameter`] into the store.
    ///
    pub fn inter_parameter(&mut self, parameter: Arc<RwLock<Parameter>>) {
        let read = parameter.read().unwrap();
        self.parameter
            .insert(read.id, (parameter.clone(), SystemTime::now()));
    }

    /// Exhume [`Parameter`] from the store.
    ///
    pub fn exhume_parameter(&self, id: &Uuid) -> Option<Arc<RwLock<Parameter>>> {
        self.parameter.get(id).map(|parameter| parameter.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Parameter>`.
    ///
    pub fn iter_parameter(&self) -> impl Iterator<Item = Arc<RwLock<Parameter>>> + '_ {
        self.parameter.values().map(|parameter| parameter.0.clone())
    }

    /// Get the timestamp for Parameter.
    ///
    pub fn parameter_timestamp(&self, parameter: &Parameter) -> SystemTime {
        self.parameter
            .get(&parameter.id)
            .map(|parameter| parameter.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Print`] into the store.
    ///
    pub fn inter_print(&mut self, print: Arc<RwLock<Print>>) {
        let read = print.read().unwrap();
        self.print
            .insert(read.id, (print.clone(), SystemTime::now()));
    }

    /// Exhume [`Print`] from the store.
    ///
    pub fn exhume_print(&self, id: &Uuid) -> Option<Arc<RwLock<Print>>> {
        self.print.get(id).map(|print| print.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Print>`.
    ///
    pub fn iter_print(&self) -> impl Iterator<Item = Arc<RwLock<Print>>> + '_ {
        self.print.values().map(|print| print.0.clone())
    }

    /// Get the timestamp for Print.
    ///
    pub fn print_timestamp(&self, print: &Print) -> SystemTime {
        self.print
            .get(&print.id)
            .map(|print| print.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Reference`] into the store.
    ///
    pub fn inter_reference(&mut self, reference: Arc<RwLock<Reference>>) {
        let read = reference.read().unwrap();
        self.reference
            .insert(read.id, (reference.clone(), SystemTime::now()));
    }

    /// Exhume [`Reference`] from the store.
    ///
    pub fn exhume_reference(&self, id: &Uuid) -> Option<Arc<RwLock<Reference>>> {
        self.reference.get(id).map(|reference| reference.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Reference>`.
    ///
    pub fn iter_reference(&self) -> impl Iterator<Item = Arc<RwLock<Reference>>> + '_ {
        self.reference.values().map(|reference| reference.0.clone())
    }

    /// Get the timestamp for Reference.
    ///
    pub fn reference_timestamp(&self, reference: &Reference) -> SystemTime {
        self.reference
            .get(&reference.id)
            .map(|reference| reference.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`ResultStatement`] into the store.
    ///
    pub fn inter_result_statement(&mut self, result_statement: Arc<RwLock<ResultStatement>>) {
        let read = result_statement.read().unwrap();
        self.result_statement
            .insert(read.id, (result_statement.clone(), SystemTime::now()));
    }

    /// Exhume [`ResultStatement`] from the store.
    ///
    pub fn exhume_result_statement(&self, id: &Uuid) -> Option<Arc<RwLock<ResultStatement>>> {
        self.result_statement
            .get(id)
            .map(|result_statement| result_statement.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ResultStatement>`.
    ///
    pub fn iter_result_statement(&self) -> impl Iterator<Item = Arc<RwLock<ResultStatement>>> + '_ {
        self.result_statement
            .values()
            .map(|result_statement| result_statement.0.clone())
    }

    /// Get the timestamp for ResultStatement.
    ///
    pub fn result_statement_timestamp(&self, result_statement: &ResultStatement) -> SystemTime {
        self.result_statement
            .get(&result_statement.id)
            .map(|result_statement| result_statement.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`ZSome`] into the store.
    ///
    pub fn inter_z_some(&mut self, z_some: Arc<RwLock<ZSome>>) {
        let read = z_some.read().unwrap();
        self.z_some
            .insert(read.id, (z_some.clone(), SystemTime::now()));
    }

    /// Exhume [`ZSome`] from the store.
    ///
    pub fn exhume_z_some(&self, id: &Uuid) -> Option<Arc<RwLock<ZSome>>> {
        self.z_some.get(id).map(|z_some| z_some.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ZSome>`.
    ///
    pub fn iter_z_some(&self) -> impl Iterator<Item = Arc<RwLock<ZSome>>> + '_ {
        self.z_some.values().map(|z_some| z_some.0.clone())
    }

    /// Get the timestamp for ZSome.
    ///
    pub fn z_some_timestamp(&self, z_some: &ZSome) -> SystemTime {
        self.z_some
            .get(&z_some.id)
            .map(|z_some| z_some.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Statement`] into the store.
    ///
    pub fn inter_statement(&mut self, statement: Arc<RwLock<Statement>>) {
        let read = statement.read().unwrap();
        self.statement
            .insert(read.id, (statement.clone(), SystemTime::now()));
    }

    /// Exhume [`Statement`] from the store.
    ///
    pub fn exhume_statement(&self, id: &Uuid) -> Option<Arc<RwLock<Statement>>> {
        self.statement.get(id).map(|statement| statement.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Statement>`.
    ///
    pub fn iter_statement(&self) -> impl Iterator<Item = Arc<RwLock<Statement>>> + '_ {
        self.statement.values().map(|statement| statement.0.clone())
    }

    /// Get the timestamp for Statement.
    ///
    pub fn statement_timestamp(&self, statement: &Statement) -> SystemTime {
        self.statement
            .get(&statement.id)
            .map(|statement| statement.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`StaticMethodCall`] into the store.
    ///
    pub fn inter_static_method_call(&mut self, static_method_call: Arc<RwLock<StaticMethodCall>>) {
        let read = static_method_call.read().unwrap();
        self.static_method_call
            .insert(read.id, (static_method_call.clone(), SystemTime::now()));
    }

    /// Exhume [`StaticMethodCall`] from the store.
    ///
    pub fn exhume_static_method_call(&self, id: &Uuid) -> Option<Arc<RwLock<StaticMethodCall>>> {
        self.static_method_call
            .get(id)
            .map(|static_method_call| static_method_call.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StaticMethodCall>`.
    ///
    pub fn iter_static_method_call(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<StaticMethodCall>>> + '_ {
        self.static_method_call
            .values()
            .map(|static_method_call| static_method_call.0.clone())
    }

    /// Get the timestamp for StaticMethodCall.
    ///
    pub fn static_method_call_timestamp(
        &self,
        static_method_call: &StaticMethodCall,
    ) -> SystemTime {
        self.static_method_call
            .get(&static_method_call.id)
            .map(|static_method_call| static_method_call.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`StringLiteral`] into the store.
    ///
    pub fn inter_string_literal(&mut self, string_literal: Arc<RwLock<StringLiteral>>) {
        let read = string_literal.read().unwrap();
        self.string_literal
            .insert(read.id, (string_literal.clone(), SystemTime::now()));
    }

    /// Exhume [`StringLiteral`] from the store.
    ///
    pub fn exhume_string_literal(&self, id: &Uuid) -> Option<Arc<RwLock<StringLiteral>>> {
        self.string_literal
            .get(id)
            .map(|string_literal| string_literal.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StringLiteral>`.
    ///
    pub fn iter_string_literal(&self) -> impl Iterator<Item = Arc<RwLock<StringLiteral>>> + '_ {
        self.string_literal
            .values()
            .map(|string_literal| string_literal.0.clone())
    }

    /// Get the timestamp for StringLiteral.
    ///
    pub fn string_literal_timestamp(&self, string_literal: &StringLiteral) -> SystemTime {
        self.string_literal
            .get(&string_literal.id)
            .map(|string_literal| string_literal.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`WoogStruct`] into the store.
    ///
    pub fn inter_woog_struct(&mut self, woog_struct: Arc<RwLock<WoogStruct>>) {
        let read = woog_struct.read().unwrap();
        let value = (woog_struct.clone(), SystemTime::now());
        self.woog_struct_id_by_name
            .insert(read.name.to_upper_camel_case(), (read.id, value.1));
        self.woog_struct.insert(read.id, value);
    }

    /// Exhume [`WoogStruct`] from the store.
    ///
    pub fn exhume_woog_struct(&self, id: &Uuid) -> Option<Arc<RwLock<WoogStruct>>> {
        self.woog_struct
            .get(id)
            .map(|woog_struct| woog_struct.0.clone())
    }

    /// Exhume [`WoogStruct`] id from the store by name.
    ///
    pub fn exhume_woog_struct_id_by_name(&self, name: &str) -> Option<&Uuid> {
        self.woog_struct_id_by_name
            .get(name)
            .map(|woog_struct| &woog_struct.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, WoogStruct>`.
    ///
    pub fn iter_woog_struct(&self) -> impl Iterator<Item = Arc<RwLock<WoogStruct>>> + '_ {
        self.woog_struct
            .values()
            .map(|woog_struct| woog_struct.0.clone())
    }

    /// Get the timestamp for WoogStruct.
    ///
    pub fn woog_struct_timestamp(&self, woog_struct: &WoogStruct) -> SystemTime {
        self.woog_struct
            .get(&woog_struct.id)
            .map(|woog_struct| woog_struct.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`StructExpression`] into the store.
    ///
    pub fn inter_struct_expression(&mut self, struct_expression: Arc<RwLock<StructExpression>>) {
        let read = struct_expression.read().unwrap();
        self.struct_expression
            .insert(read.id, (struct_expression.clone(), SystemTime::now()));
    }

    /// Exhume [`StructExpression`] from the store.
    ///
    pub fn exhume_struct_expression(&self, id: &Uuid) -> Option<Arc<RwLock<StructExpression>>> {
        self.struct_expression
            .get(id)
            .map(|struct_expression| struct_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructExpression>`.
    ///
    pub fn iter_struct_expression(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<StructExpression>>> + '_ {
        self.struct_expression
            .values()
            .map(|struct_expression| struct_expression.0.clone())
    }

    /// Get the timestamp for StructExpression.
    ///
    pub fn struct_expression_timestamp(&self, struct_expression: &StructExpression) -> SystemTime {
        self.struct_expression
            .get(&struct_expression.id)
            .map(|struct_expression| struct_expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Value`] into the store.
    ///
    pub fn inter_value(&mut self, value: Arc<RwLock<Value>>) {
        let read = value.read().unwrap();
        self.value
            .insert(read.id, (value.clone(), SystemTime::now()));
    }

    /// Exhume [`Value`] from the store.
    ///
    pub fn exhume_value(&self, id: &Uuid) -> Option<Arc<RwLock<Value>>> {
        self.value.get(id).map(|value| value.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Value>`.
    ///
    pub fn iter_value(&self) -> impl Iterator<Item = Arc<RwLock<Value>>> + '_ {
        self.value.values().map(|value| value.0.clone())
    }

    /// Get the timestamp for Value.
    ///
    pub fn value_timestamp(&self, value: &Value) -> SystemTime {
        self.value
            .get(&value.id)
            .map(|value| value.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`ValueType`] into the store.
    ///
    pub fn inter_value_type(&mut self, value_type: Arc<RwLock<ValueType>>) {
        let read = value_type.read().unwrap();
        self.value_type
            .insert(read.id(), (value_type.clone(), SystemTime::now()));
    }

    /// Exhume [`ValueType`] from the store.
    ///
    pub fn exhume_value_type(&self, id: &Uuid) -> Option<Arc<RwLock<ValueType>>> {
        self.value_type
            .get(id)
            .map(|value_type| value_type.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ValueType>`.
    ///
    pub fn iter_value_type(&self) -> impl Iterator<Item = Arc<RwLock<ValueType>>> + '_ {
        self.value_type
            .values()
            .map(|value_type| value_type.0.clone())
    }

    /// Get the timestamp for ValueType.
    ///
    pub fn value_type_timestamp(&self, value_type: &ValueType) -> SystemTime {
        self.value_type
            .get(&value_type.id())
            .map(|value_type| value_type.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Variable`] into the store.
    ///
    pub fn inter_variable(&mut self, variable: Arc<RwLock<Variable>>) {
        let read = variable.read().unwrap();
        self.variable
            .insert(read.id, (variable.clone(), SystemTime::now()));
    }

    /// Exhume [`Variable`] from the store.
    ///
    pub fn exhume_variable(&self, id: &Uuid) -> Option<Arc<RwLock<Variable>>> {
        self.variable.get(id).map(|variable| variable.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Variable>`.
    ///
    pub fn iter_variable(&self) -> impl Iterator<Item = Arc<RwLock<Variable>>> + '_ {
        self.variable.values().map(|variable| variable.0.clone())
    }

    /// Get the timestamp for Variable.
    ///
    pub fn variable_timestamp(&self, variable: &Variable) -> SystemTime {
        self.variable
            .get(&variable.id)
            .map(|variable| variable.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`VariableExpression`] into the store.
    ///
    pub fn inter_variable_expression(
        &mut self,
        variable_expression: Arc<RwLock<VariableExpression>>,
    ) {
        let read = variable_expression.read().unwrap();
        self.variable_expression
            .insert(read.id, (variable_expression.clone(), SystemTime::now()));
    }

    /// Exhume [`VariableExpression`] from the store.
    ///
    pub fn exhume_variable_expression(&self, id: &Uuid) -> Option<Arc<RwLock<VariableExpression>>> {
        self.variable_expression
            .get(id)
            .map(|variable_expression| variable_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, VariableExpression>`.
    ///
    pub fn iter_variable_expression(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<VariableExpression>>> + '_ {
        self.variable_expression
            .values()
            .map(|variable_expression| variable_expression.0.clone())
    }

    /// Get the timestamp for VariableExpression.
    ///
    pub fn variable_expression_timestamp(
        &self,
        variable_expression: &VariableExpression,
    ) -> SystemTime {
        self.variable_expression
            .get(&variable_expression.id)
            .map(|variable_expression| variable_expression.1)
            .unwrap_or(SystemTime::now())
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog-object-store-persistence"}}}
    /// Persist the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automagic git integration as an option.
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        fs::create_dir_all(&path)?;

        let bin_path = path.clone().join("lu_dog.bin");
        let mut bin_file = fs::File::create(bin_path)?;
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        bin_file.write_all(&encoded)?;

        let path = path.join("lu_dog.json");
        fs::create_dir_all(&path)?;

        // Persist Argument.
        {
            let path = path.join("argument");
            fs::create_dir_all(&path)?;
            for argument_tuple in self.argument.values() {
                let path = path.join(format!("{}.json", argument_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Argument>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != argument_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &argument_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &argument_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.argument.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Block.
        {
            let path = path.join("block");
            fs::create_dir_all(&path)?;
            for block_tuple in self.block.values() {
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
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.block.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Boolean Literal.
        {
            let path = path.join("boolean_literal");
            fs::create_dir_all(&path)?;
            for boolean_literal_tuple in self.boolean_literal.values() {
                let path = path.join(format!(
                    "{}.json",
                    boolean_literal_tuple.0.read().unwrap().id()
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<BooleanLiteral>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != boolean_literal_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &boolean_literal_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &boolean_literal_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.boolean_literal.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Call.
        {
            let path = path.join("call");
            fs::create_dir_all(&path)?;
            for call_tuple in self.call.values() {
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
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.call.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Dwarf Source File.
        {
            let path = path.join("dwarf_source_file");
            fs::create_dir_all(&path)?;
            for dwarf_source_file_tuple in self.dwarf_source_file.values() {
                let path = path.join(format!(
                    "{}.json",
                    dwarf_source_file_tuple.0.read().unwrap().id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<DwarfSourceFile>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != dwarf_source_file_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &dwarf_source_file_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &dwarf_source_file_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.dwarf_source_file.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Error.
        {
            let path = path.join("error");
            fs::create_dir_all(&path)?;
            for error_tuple in self.error.values() {
                let path = path.join(format!("{}.json", error_tuple.0.read().unwrap().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Error>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != error_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &error_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &error_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.error.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Error Expression.
        {
            let path = path.join("error_expression");
            fs::create_dir_all(&path)?;
            for error_expression_tuple in self.error_expression.values() {
                let path = path.join(format!(
                    "{}.json",
                    error_expression_tuple.0.read().unwrap().id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<ErrorExpression>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != error_expression_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &error_expression_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &error_expression_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.error_expression.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Expression.
        {
            let path = path.join("expression");
            fs::create_dir_all(&path)?;
            for expression_tuple in self.expression.values() {
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
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.expression.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Expression Statement.
        {
            let path = path.join("expression_statement");
            fs::create_dir_all(&path)?;
            for expression_statement_tuple in self.expression_statement.values() {
                let path = path.join(format!(
                    "{}.json",
                    expression_statement_tuple.0.read().unwrap().id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<ExpressionStatement>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != expression_statement_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &expression_statement_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &expression_statement_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.expression_statement.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Field.
        {
            let path = path.join("field");
            fs::create_dir_all(&path)?;
            for field_tuple in self.field.values() {
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
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.field.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Field Access.
        {
            let path = path.join("field_access");
            fs::create_dir_all(&path)?;
            for field_access_tuple in self.field_access.values() {
                let path = path.join(format!("{}.json", field_access_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<FieldAccess>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != field_access_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &field_access_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &field_access_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.field_access.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Field Expression.
        {
            let path = path.join("field_expression");
            fs::create_dir_all(&path)?;
            for field_expression_tuple in self.field_expression.values() {
                let path = path.join(format!(
                    "{}.json",
                    field_expression_tuple.0.read().unwrap().id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<FieldExpression>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != field_expression_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &field_expression_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &field_expression_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.field_expression.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Float Literal.
        {
            let path = path.join("float_literal");
            fs::create_dir_all(&path)?;
            for float_literal_tuple in self.float_literal.values() {
                let path = path.join(format!("{}.json", float_literal_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<FloatLiteral>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != float_literal_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &float_literal_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &float_literal_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.float_literal.contains_key(&id) {
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
                let path = path.join(format!(
                    "{}.json",
                    implementation_tuple.0.read().unwrap().id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Implementation>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != implementation_tuple.0.read().unwrap().to_owned()
                    {
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

        // Persist Import.
        {
            let path = path.join("import");
            fs::create_dir_all(&path)?;
            for import_tuple in self.import.values() {
                let path = path.join(format!("{}.json", import_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Import>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != import_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &import_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &import_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.import.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Integer Literal.
        {
            let path = path.join("integer_literal");
            fs::create_dir_all(&path)?;
            for integer_literal_tuple in self.integer_literal.values() {
                let path = path.join(format!(
                    "{}.json",
                    integer_literal_tuple.0.read().unwrap().id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<IntegerLiteral>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != integer_literal_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &integer_literal_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &integer_literal_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.integer_literal.contains_key(&id) {
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
                let path = path.join(format!("{}.json", item_tuple.0.read().unwrap().id));
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
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.item.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Let Statement.
        {
            let path = path.join("let_statement");
            fs::create_dir_all(&path)?;
            for let_statement_tuple in self.let_statement.values() {
                let path = path.join(format!("{}.json", let_statement_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<LetStatement>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != let_statement_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &let_statement_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &let_statement_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.let_statement.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist List.
        {
            let path = path.join("list");
            fs::create_dir_all(&path)?;
            for list_tuple in self.list.values() {
                let path = path.join(format!("{}.json", list_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<List>>, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != list_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &list_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &list_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.list.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Literal.
        {
            let path = path.join("literal");
            fs::create_dir_all(&path)?;
            for literal_tuple in self.literal.values() {
                let path = path.join(format!("{}.json", literal_tuple.0.read().unwrap().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Literal>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != literal_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &literal_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &literal_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.literal.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Local Variable.
        {
            let path = path.join("local_variable");
            fs::create_dir_all(&path)?;
            for local_variable_tuple in self.local_variable.values() {
                let path = path.join(format!(
                    "{}.json",
                    local_variable_tuple.0.read().unwrap().id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<LocalVariable>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != local_variable_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &local_variable_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &local_variable_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.local_variable.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Method Call.
        {
            let path = path.join("method_call");
            fs::create_dir_all(&path)?;
            for method_call_tuple in self.method_call.values() {
                let path = path.join(format!("{}.json", method_call_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<MethodCall>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != method_call_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &method_call_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &method_call_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.method_call.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Object Store.
        {
            let path = path.join("z_object_store");
            fs::create_dir_all(&path)?;
            for z_object_store_tuple in self.z_object_store.values() {
                let path = path.join(format!(
                    "{}.json",
                    z_object_store_tuple.0.read().unwrap().id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<ZObjectStore>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != z_object_store_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &z_object_store_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &z_object_store_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.z_object_store.contains_key(&id) {
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
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.woog_option.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Parameter.
        {
            let path = path.join("parameter");
            fs::create_dir_all(&path)?;
            for parameter_tuple in self.parameter.values() {
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
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.parameter.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Print.
        {
            let path = path.join("print");
            fs::create_dir_all(&path)?;
            for print_tuple in self.print.values() {
                let path = path.join(format!("{}.json", print_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Print>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != print_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &print_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &print_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.print.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Reference.
        {
            let path = path.join("reference");
            fs::create_dir_all(&path)?;
            for reference_tuple in self.reference.values() {
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
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.reference.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Result Statement.
        {
            let path = path.join("result_statement");
            fs::create_dir_all(&path)?;
            for result_statement_tuple in self.result_statement.values() {
                let path = path.join(format!(
                    "{}.json",
                    result_statement_tuple.0.read().unwrap().id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<ResultStatement>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != result_statement_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &result_statement_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &result_statement_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.result_statement.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Some.
        {
            let path = path.join("z_some");
            fs::create_dir_all(&path)?;
            for z_some_tuple in self.z_some.values() {
                let path = path.join(format!("{}.json", z_some_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<ZSome>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != z_some_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &z_some_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &z_some_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.z_some.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Statement.
        {
            let path = path.join("statement");
            fs::create_dir_all(&path)?;
            for statement_tuple in self.statement.values() {
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
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.statement.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Static Method Call.
        {
            let path = path.join("static_method_call");
            fs::create_dir_all(&path)?;
            for static_method_call_tuple in self.static_method_call.values() {
                let path = path.join(format!(
                    "{}.json",
                    static_method_call_tuple.0.read().unwrap().id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<StaticMethodCall>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != static_method_call_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &static_method_call_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &static_method_call_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.static_method_call.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist String Literal.
        {
            let path = path.join("string_literal");
            fs::create_dir_all(&path)?;
            for string_literal_tuple in self.string_literal.values() {
                let path = path.join(format!(
                    "{}.json",
                    string_literal_tuple.0.read().unwrap().id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<StringLiteral>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != string_literal_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &string_literal_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &string_literal_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.string_literal.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Struct.
        {
            let path = path.join("woog_struct");
            fs::create_dir_all(&path)?;
            for woog_struct_tuple in self.woog_struct.values() {
                let path = path.join(format!("{}.json", woog_struct_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<WoogStruct>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != woog_struct_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &woog_struct_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &woog_struct_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.woog_struct.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Struct Expression.
        {
            let path = path.join("struct_expression");
            fs::create_dir_all(&path)?;
            for struct_expression_tuple in self.struct_expression.values() {
                let path = path.join(format!(
                    "{}.json",
                    struct_expression_tuple.0.read().unwrap().id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<StructExpression>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != struct_expression_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &struct_expression_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &struct_expression_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.struct_expression.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Value.
        {
            let path = path.join("value");
            fs::create_dir_all(&path)?;
            for value_tuple in self.value.values() {
                let path = path.join(format!("{}.json", value_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Value>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != value_tuple.0.read().unwrap().to_owned()
                    {
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
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.value.contains_key(&id) {
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
                let path = path.join(format!("{}.json", value_type_tuple.0.read().unwrap().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<ValueType>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != value_type_tuple.0.read().unwrap().to_owned()
                    {
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

        // Persist Variable.
        {
            let path = path.join("variable");
            fs::create_dir_all(&path)?;
            for variable_tuple in self.variable.values() {
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
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.variable.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Variable Expression.
        {
            let path = path.join("variable_expression");
            fs::create_dir_all(&path)?;
            for variable_expression_tuple in self.variable_expression.values() {
                let path = path.join(format!(
                    "{}.json",
                    variable_expression_tuple.0.read().unwrap().id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<VariableExpression>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != variable_expression_tuple.0.read().unwrap().to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &variable_expression_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &variable_expression_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.variable_expression.contains_key(&id) {
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
    /// In fact, I intend to add automagic git integration as an option.
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let path = path.join("lu_dog.json");

        let mut store = Self::new();

        // Load Argument.
        {
            let path = path.join("argument");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let argument: (Arc<RwLock<Argument>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .argument
                    .insert(argument.0.read().unwrap().id, argument.clone());
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
                let block: (Arc<RwLock<Block>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .block
                    .insert(block.0.read().unwrap().id, block.clone());
            }
        }

        // Load Boolean Literal.
        {
            let path = path.join("boolean_literal");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let boolean_literal: (Arc<RwLock<BooleanLiteral>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.boolean_literal.insert(
                    boolean_literal.0.read().unwrap().id(),
                    boolean_literal.clone(),
                );
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
                let call: (Arc<RwLock<Call>>, SystemTime) = serde_json::from_reader(reader)?;
                store.call.insert(call.0.read().unwrap().id, call.clone());
            }
        }

        // Load Dwarf Source File.
        {
            let path = path.join("dwarf_source_file");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let dwarf_source_file: (Arc<RwLock<DwarfSourceFile>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.dwarf_source_file.insert(
                    dwarf_source_file.0.read().unwrap().id,
                    dwarf_source_file.clone(),
                );
            }
        }

        // Load Error.
        {
            let path = path.join("error");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let error: (Arc<RwLock<Error>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .error
                    .insert(error.0.read().unwrap().id(), error.clone());
            }
        }

        // Load Error Expression.
        {
            let path = path.join("error_expression");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let error_expression: (Arc<RwLock<ErrorExpression>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.error_expression.insert(
                    error_expression.0.read().unwrap().id,
                    error_expression.clone(),
                );
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
                let expression: (Arc<RwLock<Expression>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .expression
                    .insert(expression.0.read().unwrap().id(), expression.clone());
            }
        }

        // Load Expression Statement.
        {
            let path = path.join("expression_statement");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let expression_statement: (Arc<RwLock<ExpressionStatement>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.expression_statement.insert(
                    expression_statement.0.read().unwrap().id,
                    expression_statement.clone(),
                );
            }
        }

        // Load Field.
        {
            let path = path.join("field");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let field: (Arc<RwLock<Field>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .field
                    .insert(field.0.read().unwrap().id, field.clone());
            }
        }

        // Load Field Access.
        {
            let path = path.join("field_access");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let field_access: (Arc<RwLock<FieldAccess>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .field_access
                    .insert(field_access.0.read().unwrap().id, field_access.clone());
            }
        }

        // Load Field Expression.
        {
            let path = path.join("field_expression");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let field_expression: (Arc<RwLock<FieldExpression>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.field_expression.insert(
                    field_expression.0.read().unwrap().id,
                    field_expression.clone(),
                );
            }
        }

        // Load Float Literal.
        {
            let path = path.join("float_literal");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let float_literal: (Arc<RwLock<FloatLiteral>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .float_literal
                    .insert(float_literal.0.read().unwrap().id, float_literal.clone());
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
                let function: (Arc<RwLock<Function>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .function
                    .insert(function.0.read().unwrap().id, function.clone());
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
                let implementation: (Arc<RwLock<Implementation>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .implementation
                    .insert(implementation.0.read().unwrap().id, implementation.clone());
            }
        }

        // Load Import.
        {
            let path = path.join("import");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let import: (Arc<RwLock<Import>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .import
                    .insert(import.0.read().unwrap().id, import.clone());
            }
        }

        // Load Integer Literal.
        {
            let path = path.join("integer_literal");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let integer_literal: (Arc<RwLock<IntegerLiteral>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.integer_literal.insert(
                    integer_literal.0.read().unwrap().id,
                    integer_literal.clone(),
                );
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
                let item: (Arc<RwLock<Item>>, SystemTime) = serde_json::from_reader(reader)?;
                store.item.insert(item.0.read().unwrap().id, item.clone());
            }
        }

        // Load Let Statement.
        {
            let path = path.join("let_statement");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let let_statement: (Arc<RwLock<LetStatement>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .let_statement
                    .insert(let_statement.0.read().unwrap().id, let_statement.clone());
            }
        }

        // Load List.
        {
            let path = path.join("list");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let list: (Arc<RwLock<List>>, SystemTime) = serde_json::from_reader(reader)?;
                store.list.insert(list.0.read().unwrap().id, list.clone());
            }
        }

        // Load Literal.
        {
            let path = path.join("literal");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let literal: (Arc<RwLock<Literal>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .literal
                    .insert(literal.0.read().unwrap().id(), literal.clone());
            }
        }

        // Load Local Variable.
        {
            let path = path.join("local_variable");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let local_variable: (Arc<RwLock<LocalVariable>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .local_variable
                    .insert(local_variable.0.read().unwrap().id, local_variable.clone());
            }
        }

        // Load Method Call.
        {
            let path = path.join("method_call");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let method_call: (Arc<RwLock<MethodCall>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .method_call
                    .insert(method_call.0.read().unwrap().id, method_call.clone());
            }
        }

        // Load Object Store.
        {
            let path = path.join("z_object_store");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let z_object_store: (Arc<RwLock<ZObjectStore>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .z_object_store
                    .insert(z_object_store.0.read().unwrap().id, z_object_store.clone());
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
                let woog_option: (Arc<RwLock<WoogOption>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .woog_option
                    .insert(woog_option.0.read().unwrap().id, woog_option.clone());
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
                let parameter: (Arc<RwLock<Parameter>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .parameter
                    .insert(parameter.0.read().unwrap().id, parameter.clone());
            }
        }

        // Load Print.
        {
            let path = path.join("print");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let print: (Arc<RwLock<Print>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .print
                    .insert(print.0.read().unwrap().id, print.clone());
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
                let reference: (Arc<RwLock<Reference>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .reference
                    .insert(reference.0.read().unwrap().id, reference.clone());
            }
        }

        // Load Result Statement.
        {
            let path = path.join("result_statement");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let result_statement: (Arc<RwLock<ResultStatement>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.result_statement.insert(
                    result_statement.0.read().unwrap().id,
                    result_statement.clone(),
                );
            }
        }

        // Load Some.
        {
            let path = path.join("z_some");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let z_some: (Arc<RwLock<ZSome>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .z_some
                    .insert(z_some.0.read().unwrap().id, z_some.clone());
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
                let statement: (Arc<RwLock<Statement>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .statement
                    .insert(statement.0.read().unwrap().id, statement.clone());
            }
        }

        // Load Static Method Call.
        {
            let path = path.join("static_method_call");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let static_method_call: (Arc<RwLock<StaticMethodCall>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.static_method_call.insert(
                    static_method_call.0.read().unwrap().id,
                    static_method_call.clone(),
                );
            }
        }

        // Load String Literal.
        {
            let path = path.join("string_literal");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let string_literal: (Arc<RwLock<StringLiteral>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .string_literal
                    .insert(string_literal.0.read().unwrap().id, string_literal.clone());
            }
        }

        // Load Struct.
        {
            let path = path.join("woog_struct");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let woog_struct: (Arc<RwLock<WoogStruct>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.woog_struct_id_by_name.insert(
                    woog_struct.0.read().unwrap().name.to_upper_camel_case(),
                    (woog_struct.0.read().unwrap().id, woog_struct.1),
                );
                store
                    .woog_struct
                    .insert(woog_struct.0.read().unwrap().id, woog_struct.clone());
            }
        }

        // Load Struct Expression.
        {
            let path = path.join("struct_expression");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let struct_expression: (Arc<RwLock<StructExpression>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.struct_expression.insert(
                    struct_expression.0.read().unwrap().id,
                    struct_expression.clone(),
                );
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
                let value: (Arc<RwLock<Value>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .value
                    .insert(value.0.read().unwrap().id, value.clone());
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
                let value_type: (Arc<RwLock<ValueType>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .value_type
                    .insert(value_type.0.read().unwrap().id(), value_type.clone());
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
                let variable: (Arc<RwLock<Variable>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .variable
                    .insert(variable.0.read().unwrap().id, variable.clone());
            }
        }

        // Load Variable Expression.
        {
            let path = path.join("variable_expression");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let variable_expression: (Arc<RwLock<VariableExpression>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.variable_expression.insert(
                    variable_expression.0.read().unwrap().id,
                    variable_expression.clone(),
                );
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
