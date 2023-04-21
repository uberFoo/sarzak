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
//! * [`Error`]
//! * [`ErrorExpression`]
//! * [`Expression`]
//! * [`ExpressionStatement`]
//! * [`Field`]
//! * [`FieldAccess`]
//! * [`FieldExpression`]
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
//! * [`Some`]
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
    Argument, Block, BooleanLiteral, Call, Error, ErrorExpression, Expression, ExpressionStatement,
    Field, FieldAccess, FieldExpression, Function, Implementation, Import, IntegerLiteral, Item,
    LetStatement, List, Literal, LocalVariable, MethodCall, Parameter, Print, Reference, Some,
    Statement, StaticMethodCall, StringLiteral, StructExpression, Value, ValueType, Variable,
    VariableExpression, WoogOption, WoogStruct, ZObjectStore, EMPTY, FALSE_LITERAL, FLOAT_LITERAL,
    TRUE_LITERAL, UNKNOWN, UNKNOWN_VARIABLE,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    argument: HashMap<Uuid, (Argument, SystemTime)>,
    block: HashMap<Uuid, (Block, SystemTime)>,
    boolean_literal: HashMap<Uuid, (BooleanLiteral, SystemTime)>,
    call: HashMap<Uuid, (Call, SystemTime)>,
    error: HashMap<Uuid, (Error, SystemTime)>,
    error_expression: HashMap<Uuid, (ErrorExpression, SystemTime)>,
    expression: HashMap<Uuid, (Expression, SystemTime)>,
    expression_statement: HashMap<Uuid, (ExpressionStatement, SystemTime)>,
    field: HashMap<Uuid, (Field, SystemTime)>,
    field_access: HashMap<Uuid, (FieldAccess, SystemTime)>,
    field_expression: HashMap<Uuid, (FieldExpression, SystemTime)>,
    function: HashMap<Uuid, (Function, SystemTime)>,
    implementation: HashMap<Uuid, (Implementation, SystemTime)>,
    import: HashMap<Uuid, (Import, SystemTime)>,
    integer_literal: HashMap<Uuid, (IntegerLiteral, SystemTime)>,
    item: HashMap<Uuid, (Item, SystemTime)>,
    let_statement: HashMap<Uuid, (LetStatement, SystemTime)>,
    list: HashMap<Uuid, (List, SystemTime)>,
    literal: HashMap<Uuid, (Literal, SystemTime)>,
    local_variable: HashMap<Uuid, (LocalVariable, SystemTime)>,
    method_call: HashMap<Uuid, (MethodCall, SystemTime)>,
    z_object_store: HashMap<Uuid, (ZObjectStore, SystemTime)>,
    woog_option: HashMap<Uuid, (WoogOption, SystemTime)>,
    parameter: HashMap<Uuid, (Parameter, SystemTime)>,
    print: HashMap<Uuid, (Print, SystemTime)>,
    reference: HashMap<Uuid, (Reference, SystemTime)>,
    some: HashMap<Uuid, (Some, SystemTime)>,
    statement: HashMap<Uuid, (Statement, SystemTime)>,
    static_method_call: HashMap<Uuid, (StaticMethodCall, SystemTime)>,
    string_literal: HashMap<Uuid, (StringLiteral, SystemTime)>,
    woog_struct: HashMap<Uuid, (WoogStruct, SystemTime)>,
    woog_struct_id_by_name: HashMap<String, (Uuid, SystemTime)>,
    struct_expression: HashMap<Uuid, (StructExpression, SystemTime)>,
    value: HashMap<Uuid, (Value, SystemTime)>,
    value_type: HashMap<Uuid, (ValueType, SystemTime)>,
    variable: HashMap<Uuid, (Variable, SystemTime)>,
    variable_expression: HashMap<Uuid, (VariableExpression, SystemTime)>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            argument: HashMap::default(),
            block: HashMap::default(),
            boolean_literal: HashMap::default(),
            call: HashMap::default(),
            error: HashMap::default(),
            error_expression: HashMap::default(),
            expression: HashMap::default(),
            expression_statement: HashMap::default(),
            field: HashMap::default(),
            field_access: HashMap::default(),
            field_expression: HashMap::default(),
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
            some: HashMap::default(),
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
        store.inter_boolean_literal(BooleanLiteral::FalseLiteral(FALSE_LITERAL));
        store.inter_boolean_literal(BooleanLiteral::TrueLiteral(TRUE_LITERAL));
        store.inter_error(Error::UnknownVariable(UNKNOWN_VARIABLE));
        store.inter_expression(Expression::Literal(
            Literal::BooleanLiteral(BooleanLiteral::FalseLiteral(FALSE_LITERAL).id()).id(),
        ));
        store.inter_expression(Expression::Literal(
            Literal::BooleanLiteral(BooleanLiteral::TrueLiteral(TRUE_LITERAL).id()).id(),
        ));
        store.inter_expression(Expression::Literal(
            Literal::FloatLiteral(FLOAT_LITERAL).id(),
        ));
        store.inter_literal(Literal::BooleanLiteral(
            BooleanLiteral::FalseLiteral(FALSE_LITERAL).id(),
        ));
        store.inter_literal(Literal::BooleanLiteral(
            BooleanLiteral::TrueLiteral(TRUE_LITERAL).id(),
        ));
        store.inter_literal(Literal::FloatLiteral(FLOAT_LITERAL));
        store.inter_value_type(ValueType::Empty(EMPTY));
        store.inter_value_type(ValueType::Error(
            Error::UnknownVariable(UNKNOWN_VARIABLE).id(),
        ));
        store.inter_value_type(ValueType::Unknown(UNKNOWN));

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog-object-store-methods"}}}
    /// Inter [`Argument`] into the store.
    ///
    pub fn inter_argument(&mut self, argument: Argument) {
        self.argument
            .insert(argument.id, (argument, SystemTime::now()));
    }

    /// Exhume [`Argument`] from the store.
    ///
    pub fn exhume_argument(&self, id: &Uuid) -> Option<&Argument> {
        self.argument.get(id).map(|argument| &argument.0)
    }

    /// Exhume [`Argument`] from the store — mutably.
    ///
    pub fn exhume_argument_mut(&mut self, id: &Uuid) -> Option<&mut Argument> {
        self.argument.get_mut(id).map(|argument| &mut argument.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Argument>`.
    ///
    pub fn iter_argument(&self) -> impl Iterator<Item = &Argument> {
        self.argument.values().map(|argument| &argument.0)
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

    /// Inter [`BooleanLiteral`] into the store.
    ///
    pub fn inter_boolean_literal(&mut self, boolean_literal: BooleanLiteral) {
        self.boolean_literal
            .insert(boolean_literal.id(), (boolean_literal, SystemTime::now()));
    }

    /// Exhume [`BooleanLiteral`] from the store.
    ///
    pub fn exhume_boolean_literal(&self, id: &Uuid) -> Option<&BooleanLiteral> {
        self.boolean_literal
            .get(id)
            .map(|boolean_literal| &boolean_literal.0)
    }

    /// Exhume [`BooleanLiteral`] from the store — mutably.
    ///
    pub fn exhume_boolean_literal_mut(&mut self, id: &Uuid) -> Option<&mut BooleanLiteral> {
        self.boolean_literal
            .get_mut(id)
            .map(|boolean_literal| &mut boolean_literal.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanLiteral>`.
    ///
    pub fn iter_boolean_literal(&self) -> impl Iterator<Item = &BooleanLiteral> {
        self.boolean_literal
            .values()
            .map(|boolean_literal| &boolean_literal.0)
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

    /// Inter [`Error`] into the store.
    ///
    pub fn inter_error(&mut self, error: Error) {
        self.error.insert(error.id(), (error, SystemTime::now()));
    }

    /// Exhume [`Error`] from the store.
    ///
    pub fn exhume_error(&self, id: &Uuid) -> Option<&Error> {
        self.error.get(id).map(|error| &error.0)
    }

    /// Exhume [`Error`] from the store — mutably.
    ///
    pub fn exhume_error_mut(&mut self, id: &Uuid) -> Option<&mut Error> {
        self.error.get_mut(id).map(|error| &mut error.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Error>`.
    ///
    pub fn iter_error(&self) -> impl Iterator<Item = &Error> {
        self.error.values().map(|error| &error.0)
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
    pub fn inter_error_expression(&mut self, error_expression: ErrorExpression) {
        self.error_expression
            .insert(error_expression.id, (error_expression, SystemTime::now()));
    }

    /// Exhume [`ErrorExpression`] from the store.
    ///
    pub fn exhume_error_expression(&self, id: &Uuid) -> Option<&ErrorExpression> {
        self.error_expression
            .get(id)
            .map(|error_expression| &error_expression.0)
    }

    /// Exhume [`ErrorExpression`] from the store — mutably.
    ///
    pub fn exhume_error_expression_mut(&mut self, id: &Uuid) -> Option<&mut ErrorExpression> {
        self.error_expression
            .get_mut(id)
            .map(|error_expression| &mut error_expression.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ErrorExpression>`.
    ///
    pub fn iter_error_expression(&self) -> impl Iterator<Item = &ErrorExpression> {
        self.error_expression
            .values()
            .map(|error_expression| &error_expression.0)
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

    /// Inter [`ExpressionStatement`] into the store.
    ///
    pub fn inter_expression_statement(&mut self, expression_statement: ExpressionStatement) {
        self.expression_statement.insert(
            expression_statement.id,
            (expression_statement, SystemTime::now()),
        );
    }

    /// Exhume [`ExpressionStatement`] from the store.
    ///
    pub fn exhume_expression_statement(&self, id: &Uuid) -> Option<&ExpressionStatement> {
        self.expression_statement
            .get(id)
            .map(|expression_statement| &expression_statement.0)
    }

    /// Exhume [`ExpressionStatement`] from the store — mutably.
    ///
    pub fn exhume_expression_statement_mut(
        &mut self,
        id: &Uuid,
    ) -> Option<&mut ExpressionStatement> {
        self.expression_statement
            .get_mut(id)
            .map(|expression_statement| &mut expression_statement.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ExpressionStatement>`.
    ///
    pub fn iter_expression_statement(&self) -> impl Iterator<Item = &ExpressionStatement> {
        self.expression_statement
            .values()
            .map(|expression_statement| &expression_statement.0)
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
    pub fn inter_field(&mut self, field: Field) {
        self.field.insert(field.id, (field, SystemTime::now()));
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

    /// Inter [`FieldAccess`] into the store.
    ///
    pub fn inter_field_access(&mut self, field_access: FieldAccess) {
        self.field_access
            .insert(field_access.id, (field_access, SystemTime::now()));
    }

    /// Exhume [`FieldAccess`] from the store.
    ///
    pub fn exhume_field_access(&self, id: &Uuid) -> Option<&FieldAccess> {
        self.field_access
            .get(id)
            .map(|field_access| &field_access.0)
    }

    /// Exhume [`FieldAccess`] from the store — mutably.
    ///
    pub fn exhume_field_access_mut(&mut self, id: &Uuid) -> Option<&mut FieldAccess> {
        self.field_access
            .get_mut(id)
            .map(|field_access| &mut field_access.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccess>`.
    ///
    pub fn iter_field_access(&self) -> impl Iterator<Item = &FieldAccess> {
        self.field_access
            .values()
            .map(|field_access| &field_access.0)
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
    pub fn inter_field_expression(&mut self, field_expression: FieldExpression) {
        self.field_expression
            .insert(field_expression.id, (field_expression, SystemTime::now()));
    }

    /// Exhume [`FieldExpression`] from the store.
    ///
    pub fn exhume_field_expression(&self, id: &Uuid) -> Option<&FieldExpression> {
        self.field_expression
            .get(id)
            .map(|field_expression| &field_expression.0)
    }

    /// Exhume [`FieldExpression`] from the store — mutably.
    ///
    pub fn exhume_field_expression_mut(&mut self, id: &Uuid) -> Option<&mut FieldExpression> {
        self.field_expression
            .get_mut(id)
            .map(|field_expression| &mut field_expression.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldExpression>`.
    ///
    pub fn iter_field_expression(&self) -> impl Iterator<Item = &FieldExpression> {
        self.field_expression
            .values()
            .map(|field_expression| &field_expression.0)
    }

    /// Get the timestamp for FieldExpression.
    ///
    pub fn field_expression_timestamp(&self, field_expression: &FieldExpression) -> SystemTime {
        self.field_expression
            .get(&field_expression.id)
            .map(|field_expression| field_expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Function`] into the store.
    ///
    pub fn inter_function(&mut self, function: Function) {
        self.function
            .insert(function.id, (function, SystemTime::now()));
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

    /// Inter [`Import`] into the store.
    ///
    pub fn inter_import(&mut self, import: Import) {
        self.import.insert(import.id, (import, SystemTime::now()));
    }

    /// Exhume [`Import`] from the store.
    ///
    pub fn exhume_import(&self, id: &Uuid) -> Option<&Import> {
        self.import.get(id).map(|import| &import.0)
    }

    /// Exhume [`Import`] from the store — mutably.
    ///
    pub fn exhume_import_mut(&mut self, id: &Uuid) -> Option<&mut Import> {
        self.import.get_mut(id).map(|import| &mut import.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Import>`.
    ///
    pub fn iter_import(&self) -> impl Iterator<Item = &Import> {
        self.import.values().map(|import| &import.0)
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
    pub fn inter_integer_literal(&mut self, integer_literal: IntegerLiteral) {
        self.integer_literal
            .insert(integer_literal.id, (integer_literal, SystemTime::now()));
    }

    /// Exhume [`IntegerLiteral`] from the store.
    ///
    pub fn exhume_integer_literal(&self, id: &Uuid) -> Option<&IntegerLiteral> {
        self.integer_literal
            .get(id)
            .map(|integer_literal| &integer_literal.0)
    }

    /// Exhume [`IntegerLiteral`] from the store — mutably.
    ///
    pub fn exhume_integer_literal_mut(&mut self, id: &Uuid) -> Option<&mut IntegerLiteral> {
        self.integer_literal
            .get_mut(id)
            .map(|integer_literal| &mut integer_literal.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, IntegerLiteral>`.
    ///
    pub fn iter_integer_literal(&self) -> impl Iterator<Item = &IntegerLiteral> {
        self.integer_literal
            .values()
            .map(|integer_literal| &integer_literal.0)
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

    /// Inter [`LetStatement`] into the store.
    ///
    pub fn inter_let_statement(&mut self, let_statement: LetStatement) {
        self.let_statement
            .insert(let_statement.id, (let_statement, SystemTime::now()));
    }

    /// Exhume [`LetStatement`] from the store.
    ///
    pub fn exhume_let_statement(&self, id: &Uuid) -> Option<&LetStatement> {
        self.let_statement
            .get(id)
            .map(|let_statement| &let_statement.0)
    }

    /// Exhume [`LetStatement`] from the store — mutably.
    ///
    pub fn exhume_let_statement_mut(&mut self, id: &Uuid) -> Option<&mut LetStatement> {
        self.let_statement
            .get_mut(id)
            .map(|let_statement| &mut let_statement.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LetStatement>`.
    ///
    pub fn iter_let_statement(&self) -> impl Iterator<Item = &LetStatement> {
        self.let_statement
            .values()
            .map(|let_statement| &let_statement.0)
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
    pub fn inter_list(&mut self, list: List) {
        self.list.insert(list.id, (list, SystemTime::now()));
    }

    /// Exhume [`List`] from the store.
    ///
    pub fn exhume_list(&self, id: &Uuid) -> Option<&List> {
        self.list.get(id).map(|list| &list.0)
    }

    /// Exhume [`List`] from the store — mutably.
    ///
    pub fn exhume_list_mut(&mut self, id: &Uuid) -> Option<&mut List> {
        self.list.get_mut(id).map(|list| &mut list.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, List>`.
    ///
    pub fn iter_list(&self) -> impl Iterator<Item = &List> {
        self.list.values().map(|list| &list.0)
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
    pub fn inter_literal(&mut self, literal: Literal) {
        self.literal
            .insert(literal.id(), (literal, SystemTime::now()));
    }

    /// Exhume [`Literal`] from the store.
    ///
    pub fn exhume_literal(&self, id: &Uuid) -> Option<&Literal> {
        self.literal.get(id).map(|literal| &literal.0)
    }

    /// Exhume [`Literal`] from the store — mutably.
    ///
    pub fn exhume_literal_mut(&mut self, id: &Uuid) -> Option<&mut Literal> {
        self.literal.get_mut(id).map(|literal| &mut literal.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Literal>`.
    ///
    pub fn iter_literal(&self) -> impl Iterator<Item = &Literal> {
        self.literal.values().map(|literal| &literal.0)
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
    pub fn inter_local_variable(&mut self, local_variable: LocalVariable) {
        self.local_variable
            .insert(local_variable.id, (local_variable, SystemTime::now()));
    }

    /// Exhume [`LocalVariable`] from the store.
    ///
    pub fn exhume_local_variable(&self, id: &Uuid) -> Option<&LocalVariable> {
        self.local_variable
            .get(id)
            .map(|local_variable| &local_variable.0)
    }

    /// Exhume [`LocalVariable`] from the store — mutably.
    ///
    pub fn exhume_local_variable_mut(&mut self, id: &Uuid) -> Option<&mut LocalVariable> {
        self.local_variable
            .get_mut(id)
            .map(|local_variable| &mut local_variable.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LocalVariable>`.
    ///
    pub fn iter_local_variable(&self) -> impl Iterator<Item = &LocalVariable> {
        self.local_variable
            .values()
            .map(|local_variable| &local_variable.0)
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
    pub fn inter_method_call(&mut self, method_call: MethodCall) {
        self.method_call
            .insert(method_call.id, (method_call, SystemTime::now()));
    }

    /// Exhume [`MethodCall`] from the store.
    ///
    pub fn exhume_method_call(&self, id: &Uuid) -> Option<&MethodCall> {
        self.method_call.get(id).map(|method_call| &method_call.0)
    }

    /// Exhume [`MethodCall`] from the store — mutably.
    ///
    pub fn exhume_method_call_mut(&mut self, id: &Uuid) -> Option<&mut MethodCall> {
        self.method_call
            .get_mut(id)
            .map(|method_call| &mut method_call.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, MethodCall>`.
    ///
    pub fn iter_method_call(&self) -> impl Iterator<Item = &MethodCall> {
        self.method_call.values().map(|method_call| &method_call.0)
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
    pub fn inter_z_object_store(&mut self, z_object_store: ZObjectStore) {
        self.z_object_store
            .insert(z_object_store.id, (z_object_store, SystemTime::now()));
    }

    /// Exhume [`ZObjectStore`] from the store.
    ///
    pub fn exhume_z_object_store(&self, id: &Uuid) -> Option<&ZObjectStore> {
        self.z_object_store
            .get(id)
            .map(|z_object_store| &z_object_store.0)
    }

    /// Exhume [`ZObjectStore`] from the store — mutably.
    ///
    pub fn exhume_z_object_store_mut(&mut self, id: &Uuid) -> Option<&mut ZObjectStore> {
        self.z_object_store
            .get_mut(id)
            .map(|z_object_store| &mut z_object_store.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ZObjectStore>`.
    ///
    pub fn iter_z_object_store(&self) -> impl Iterator<Item = &ZObjectStore> {
        self.z_object_store
            .values()
            .map(|z_object_store| &z_object_store.0)
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

    /// Inter [`Print`] into the store.
    ///
    pub fn inter_print(&mut self, print: Print) {
        self.print.insert(print.id, (print, SystemTime::now()));
    }

    /// Exhume [`Print`] from the store.
    ///
    pub fn exhume_print(&self, id: &Uuid) -> Option<&Print> {
        self.print.get(id).map(|print| &print.0)
    }

    /// Exhume [`Print`] from the store — mutably.
    ///
    pub fn exhume_print_mut(&mut self, id: &Uuid) -> Option<&mut Print> {
        self.print.get_mut(id).map(|print| &mut print.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Print>`.
    ///
    pub fn iter_print(&self) -> impl Iterator<Item = &Print> {
        self.print.values().map(|print| &print.0)
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

    /// Inter [`StaticMethodCall`] into the store.
    ///
    pub fn inter_static_method_call(&mut self, static_method_call: StaticMethodCall) {
        self.static_method_call.insert(
            static_method_call.id,
            (static_method_call, SystemTime::now()),
        );
    }

    /// Exhume [`StaticMethodCall`] from the store.
    ///
    pub fn exhume_static_method_call(&self, id: &Uuid) -> Option<&StaticMethodCall> {
        self.static_method_call
            .get(id)
            .map(|static_method_call| &static_method_call.0)
    }

    /// Exhume [`StaticMethodCall`] from the store — mutably.
    ///
    pub fn exhume_static_method_call_mut(&mut self, id: &Uuid) -> Option<&mut StaticMethodCall> {
        self.static_method_call
            .get_mut(id)
            .map(|static_method_call| &mut static_method_call.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StaticMethodCall>`.
    ///
    pub fn iter_static_method_call(&self) -> impl Iterator<Item = &StaticMethodCall> {
        self.static_method_call
            .values()
            .map(|static_method_call| &static_method_call.0)
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
    pub fn inter_string_literal(&mut self, string_literal: StringLiteral) {
        self.string_literal
            .insert(string_literal.id, (string_literal, SystemTime::now()));
    }

    /// Exhume [`StringLiteral`] from the store.
    ///
    pub fn exhume_string_literal(&self, id: &Uuid) -> Option<&StringLiteral> {
        self.string_literal
            .get(id)
            .map(|string_literal| &string_literal.0)
    }

    /// Exhume [`StringLiteral`] from the store — mutably.
    ///
    pub fn exhume_string_literal_mut(&mut self, id: &Uuid) -> Option<&mut StringLiteral> {
        self.string_literal
            .get_mut(id)
            .map(|string_literal| &mut string_literal.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StringLiteral>`.
    ///
    pub fn iter_string_literal(&self) -> impl Iterator<Item = &StringLiteral> {
        self.string_literal
            .values()
            .map(|string_literal| &string_literal.0)
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
    pub fn inter_woog_struct(&mut self, woog_struct: WoogStruct) {
        let value = (woog_struct, SystemTime::now());
        self.woog_struct_id_by_name
            .insert(value.0.name.to_upper_camel_case(), (value.0.id, value.1));
        self.woog_struct.insert(value.0.id, value);
    }

    /// Exhume [`WoogStruct`] from the store.
    ///
    pub fn exhume_woog_struct(&self, id: &Uuid) -> Option<&WoogStruct> {
        self.woog_struct.get(id).map(|woog_struct| &woog_struct.0)
    }

    /// Exhume [`WoogStruct`] from the store — mutably.
    ///
    pub fn exhume_woog_struct_mut(&mut self, id: &Uuid) -> Option<&mut WoogStruct> {
        self.woog_struct
            .get_mut(id)
            .map(|woog_struct| &mut woog_struct.0)
    }

    /// Exhume [`WoogStruct`] from the store by name.
    ///
    pub fn exhume_woog_struct_id_by_name(&self, name: &str) -> Option<&Uuid> {
        self.woog_struct_id_by_name
            .get(name)
            .map(|woog_struct| &woog_struct.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, WoogStruct>`.
    ///
    pub fn iter_woog_struct(&self) -> impl Iterator<Item = &WoogStruct> {
        self.woog_struct.values().map(|woog_struct| &woog_struct.0)
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
    pub fn inter_struct_expression(&mut self, struct_expression: StructExpression) {
        self.struct_expression
            .insert(struct_expression.id, (struct_expression, SystemTime::now()));
    }

    /// Exhume [`StructExpression`] from the store.
    ///
    pub fn exhume_struct_expression(&self, id: &Uuid) -> Option<&StructExpression> {
        self.struct_expression
            .get(id)
            .map(|struct_expression| &struct_expression.0)
    }

    /// Exhume [`StructExpression`] from the store — mutably.
    ///
    pub fn exhume_struct_expression_mut(&mut self, id: &Uuid) -> Option<&mut StructExpression> {
        self.struct_expression
            .get_mut(id)
            .map(|struct_expression| &mut struct_expression.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructExpression>`.
    ///
    pub fn iter_struct_expression(&self) -> impl Iterator<Item = &StructExpression> {
        self.struct_expression
            .values()
            .map(|struct_expression| &struct_expression.0)
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

    /// Inter [`VariableExpression`] into the store.
    ///
    pub fn inter_variable_expression(&mut self, variable_expression: VariableExpression) {
        self.variable_expression.insert(
            variable_expression.id,
            (variable_expression, SystemTime::now()),
        );
    }

    /// Exhume [`VariableExpression`] from the store.
    ///
    pub fn exhume_variable_expression(&self, id: &Uuid) -> Option<&VariableExpression> {
        self.variable_expression
            .get(id)
            .map(|variable_expression| &variable_expression.0)
    }

    /// Exhume [`VariableExpression`] from the store — mutably.
    ///
    pub fn exhume_variable_expression_mut(&mut self, id: &Uuid) -> Option<&mut VariableExpression> {
        self.variable_expression
            .get_mut(id)
            .map(|variable_expression| &mut variable_expression.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, VariableExpression>`.
    ///
    pub fn iter_variable_expression(&self) -> impl Iterator<Item = &VariableExpression> {
        self.variable_expression
            .values()
            .map(|variable_expression| &variable_expression.0)
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
                let path = path.join(format!("{}.json", argument_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Argument, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != argument_tuple.0 {
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
                let path = path.join(format!("{}.json", boolean_literal_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (BooleanLiteral, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != boolean_literal_tuple.0 {
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

        // Persist Error.
        {
            let path = path.join("error");
            fs::create_dir_all(&path)?;
            for error_tuple in self.error.values() {
                let path = path.join(format!("{}.json", error_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Error, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != error_tuple.0 {
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
                let path = path.join(format!("{}.json", error_expression_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (ErrorExpression, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != error_expression_tuple.0 {
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
                let path = path.join(format!("{}.json", expression_statement_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (ExpressionStatement, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0 != expression_statement_tuple.0 {
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

        // Persist Field Access.
        {
            let path = path.join("field_access");
            fs::create_dir_all(&path)?;
            for field_access_tuple in self.field_access.values() {
                let path = path.join(format!("{}.json", field_access_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (FieldAccess, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != field_access_tuple.0 {
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
                let path = path.join(format!("{}.json", field_expression_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (FieldExpression, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != field_expression_tuple.0 {
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

        // Persist Import.
        {
            let path = path.join("import");
            fs::create_dir_all(&path)?;
            for import_tuple in self.import.values() {
                let path = path.join(format!("{}.json", import_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Import, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != import_tuple.0 {
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
                let path = path.join(format!("{}.json", integer_literal_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (IntegerLiteral, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != integer_literal_tuple.0 {
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

        // Persist Let Statement.
        {
            let path = path.join("let_statement");
            fs::create_dir_all(&path)?;
            for let_statement_tuple in self.let_statement.values() {
                let path = path.join(format!("{}.json", let_statement_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (LetStatement, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != let_statement_tuple.0 {
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
                let path = path.join(format!("{}.json", list_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (List, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != list_tuple.0 {
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
                let path = path.join(format!("{}.json", literal_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Literal, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != literal_tuple.0 {
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
                let path = path.join(format!("{}.json", local_variable_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (LocalVariable, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != local_variable_tuple.0 {
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
                let path = path.join(format!("{}.json", method_call_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (MethodCall, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != method_call_tuple.0 {
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
                let path = path.join(format!("{}.json", z_object_store_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (ZObjectStore, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != z_object_store_tuple.0 {
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
                let path = path.join(format!("{}.json", print_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Print, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != print_tuple.0 {
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
                let path = path.join(format!("{}.json", static_method_call_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (StaticMethodCall, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != static_method_call_tuple.0 {
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
                let path = path.join(format!("{}.json", string_literal_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (StringLiteral, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != string_literal_tuple.0 {
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
                let path = path.join(format!("{}.json", woog_struct_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (WoogStruct, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != woog_struct_tuple.0 {
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
                let path = path.join(format!("{}.json", struct_expression_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (StructExpression, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != struct_expression_tuple.0 {
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
                let path = path.join(format!("{}.json", variable_expression_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (VariableExpression, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0 != variable_expression_tuple.0 {
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
    /// In fact, I intend to add automaagic git integration as an option.
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
                let argument: (Argument, SystemTime) = serde_json::from_reader(reader)?;
                store.argument.insert(argument.0.id, argument);
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

        // Load Boolean Literal.
        {
            let path = path.join("boolean_literal");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let boolean_literal: (BooleanLiteral, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .boolean_literal
                    .insert(boolean_literal.0.id(), boolean_literal);
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

        // Load Error.
        {
            let path = path.join("error");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let error: (Error, SystemTime) = serde_json::from_reader(reader)?;
                store.error.insert(error.0.id(), error);
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
                let error_expression: (ErrorExpression, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .error_expression
                    .insert(error_expression.0.id, error_expression);
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

        // Load Expression Statement.
        {
            let path = path.join("expression_statement");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let expression_statement: (ExpressionStatement, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .expression_statement
                    .insert(expression_statement.0.id, expression_statement);
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
                let field: (Field, SystemTime) = serde_json::from_reader(reader)?;
                store.field.insert(field.0.id, field);
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
                let field_access: (FieldAccess, SystemTime) = serde_json::from_reader(reader)?;
                store.field_access.insert(field_access.0.id, field_access);
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
                let field_expression: (FieldExpression, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .field_expression
                    .insert(field_expression.0.id, field_expression);
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

        // Load Import.
        {
            let path = path.join("import");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let import: (Import, SystemTime) = serde_json::from_reader(reader)?;
                store.import.insert(import.0.id, import);
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
                let integer_literal: (IntegerLiteral, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .integer_literal
                    .insert(integer_literal.0.id, integer_literal);
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

        // Load Let Statement.
        {
            let path = path.join("let_statement");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let let_statement: (LetStatement, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .let_statement
                    .insert(let_statement.0.id, let_statement);
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
                let list: (List, SystemTime) = serde_json::from_reader(reader)?;
                store.list.insert(list.0.id, list);
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
                let literal: (Literal, SystemTime) = serde_json::from_reader(reader)?;
                store.literal.insert(literal.0.id(), literal);
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
                let local_variable: (LocalVariable, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .local_variable
                    .insert(local_variable.0.id, local_variable);
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
                let method_call: (MethodCall, SystemTime) = serde_json::from_reader(reader)?;
                store.method_call.insert(method_call.0.id, method_call);
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
                let z_object_store: (ZObjectStore, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .z_object_store
                    .insert(z_object_store.0.id, z_object_store);
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

        // Load Print.
        {
            let path = path.join("print");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let print: (Print, SystemTime) = serde_json::from_reader(reader)?;
                store.print.insert(print.0.id, print);
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

        // Load Static Method Call.
        {
            let path = path.join("static_method_call");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let static_method_call: (StaticMethodCall, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .static_method_call
                    .insert(static_method_call.0.id, static_method_call);
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
                let string_literal: (StringLiteral, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .string_literal
                    .insert(string_literal.0.id, string_literal);
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
                let woog_struct: (WoogStruct, SystemTime) = serde_json::from_reader(reader)?;
                store.woog_struct_id_by_name.insert(
                    woog_struct.0.name.to_upper_camel_case(),
                    (woog_struct.0.id, woog_struct.1),
                );
                store.woog_struct.insert(woog_struct.0.id, woog_struct);
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
                let struct_expression: (StructExpression, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .struct_expression
                    .insert(struct_expression.0.id, struct_expression);
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

        // Load Variable Expression.
        {
            let path = path.join("variable_expression");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let variable_expression: (VariableExpression, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .variable_expression
                    .insert(variable_expression.0.id, variable_expression);
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
