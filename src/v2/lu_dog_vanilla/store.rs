//! v2::lu_dog_vanilla Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_vanilla-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`Argument`]
//! * [`Binary`]
//! * [`Block`]
//! * [`BooleanLiteral`]
//! * [`BooleanOperator`]
//! * [`Call`]
//! * [`Comparison`]
//! * [`DwarfSourceFile`]
//! * [`Error`]
//! * [`ErrorExpression`]
//! * [`Expression`]
//! * [`ExpressionStatement`]
//! * [`Field`]
//! * [`FieldAccess`]
//! * [`FieldAccessTarget`]
//! * [`FieldExpression`]
//! * [`FloatLiteral`]
//! * [`ForLoop`]
//! * [`Function`]
//! * [`Grouped`]
//! * [`XIf`]
//! * [`Implementation`]
//! * [`Import`]
//! * [`Index`]
//! * [`IntegerLiteral`]
//! * [`Item`]
//! * [`LetStatement`]
//! * [`List`]
//! * [`ListElement`]
//! * [`ListExpression`]
//! * [`Literal`]
//! * [`LocalVariable`]
//! * [`XMacro`]
//! * [`MethodCall`]
//! * [`ZObjectStore`]
//! * [`Operator`]
//! * [`WoogOption`]
//! * [`Parameter`]
//! * [`Print`]
//! * [`RangeExpression`]
//! * [`Reference`]
//! * [`ResultStatement`]
//! * [`XReturn`]
//! * [`ZSome`]
//! * [`Span`]
//! * [`Statement`]
//! * [`StaticMethodCall`]
//! * [`StringLiteral`]
//! * [`WoogStruct`]
//! * [`StructExpression`]
//! * [`TypeCast`]
//! * [`Unary`]
//! * [`XValue`]
//! * [`ValueType`]
//! * [`Variable`]
//! * [`VariableExpression`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_vanilla-object-store-definition"}}}
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

use crate::v2::lu_dog_vanilla::types::{
    Argument, Binary, Block, BooleanLiteral, BooleanOperator, Call, Comparison, DwarfSourceFile,
    Error, ErrorExpression, Expression, ExpressionStatement, Field, FieldAccess, FieldAccessTarget,
    FieldExpression, FloatLiteral, ForLoop, Function, Grouped, Implementation, Import, Index,
    IntegerLiteral, Item, LetStatement, List, ListElement, ListExpression, Literal, LocalVariable,
    MethodCall, Operator, Parameter, Print, RangeExpression, Reference, ResultStatement, Span,
    Statement, StaticMethodCall, StringLiteral, StructExpression, TypeCast, Unary, ValueType,
    Variable, VariableExpression, WoogOption, WoogStruct, XIf, XMacro, XReturn, XValue,
    ZObjectStore, ZSome, ADDITION, AND, ASSIGNMENT, DEBUGGER, DIVISION, EMPTY, EQUAL,
    FALSE_LITERAL, GREATER_THAN, GREATER_THAN_OR_EQUAL, LESS_THAN_OR_EQUAL, MULTIPLICATION,
    NEGATION, NOT, NOT_EQUAL, RANGE, SUBTRACTION, TRUE_LITERAL, UNKNOWN, UNKNOWN_VARIABLE, Z_NONE,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    argument: HashMap<Uuid, (Argument, SystemTime)>,
    binary: HashMap<Uuid, (Binary, SystemTime)>,
    block: HashMap<Uuid, (Block, SystemTime)>,
    boolean_literal: HashMap<Uuid, (BooleanLiteral, SystemTime)>,
    boolean_operator: HashMap<Uuid, (BooleanOperator, SystemTime)>,
    call: HashMap<Uuid, (Call, SystemTime)>,
    comparison: HashMap<Uuid, (Comparison, SystemTime)>,
    dwarf_source_file: HashMap<Uuid, (DwarfSourceFile, SystemTime)>,
    error: HashMap<Uuid, (Error, SystemTime)>,
    error_expression: HashMap<Uuid, (ErrorExpression, SystemTime)>,
    expression: HashMap<Uuid, (Expression, SystemTime)>,
    expression_statement: HashMap<Uuid, (ExpressionStatement, SystemTime)>,
    field: HashMap<Uuid, (Field, SystemTime)>,
    field_id_by_name: HashMap<String, (Uuid, SystemTime)>,
    field_access: HashMap<Uuid, (FieldAccess, SystemTime)>,
    field_access_target: HashMap<Uuid, (FieldAccessTarget, SystemTime)>,
    field_expression: HashMap<Uuid, (FieldExpression, SystemTime)>,
    float_literal: HashMap<Uuid, (FloatLiteral, SystemTime)>,
    for_loop: HashMap<Uuid, (ForLoop, SystemTime)>,
    function: HashMap<Uuid, (Function, SystemTime)>,
    function_id_by_name: HashMap<String, (Uuid, SystemTime)>,
    grouped: HashMap<Uuid, (Grouped, SystemTime)>,
    x_if: HashMap<Uuid, (XIf, SystemTime)>,
    implementation: HashMap<Uuid, (Implementation, SystemTime)>,
    import: HashMap<Uuid, (Import, SystemTime)>,
    index: HashMap<Uuid, (Index, SystemTime)>,
    integer_literal: HashMap<Uuid, (IntegerLiteral, SystemTime)>,
    item: HashMap<Uuid, (Item, SystemTime)>,
    let_statement: HashMap<Uuid, (LetStatement, SystemTime)>,
    list: HashMap<Uuid, (List, SystemTime)>,
    list_element: HashMap<Uuid, (ListElement, SystemTime)>,
    list_expression: HashMap<Uuid, (ListExpression, SystemTime)>,
    literal: HashMap<Uuid, (Literal, SystemTime)>,
    local_variable: HashMap<Uuid, (LocalVariable, SystemTime)>,
    x_macro: HashMap<Uuid, (XMacro, SystemTime)>,
    method_call: HashMap<Uuid, (MethodCall, SystemTime)>,
    z_object_store: HashMap<Uuid, (ZObjectStore, SystemTime)>,
    operator: HashMap<Uuid, (Operator, SystemTime)>,
    woog_option: HashMap<Uuid, (WoogOption, SystemTime)>,
    parameter: HashMap<Uuid, (Parameter, SystemTime)>,
    print: HashMap<Uuid, (Print, SystemTime)>,
    range_expression: HashMap<Uuid, (RangeExpression, SystemTime)>,
    reference: HashMap<Uuid, (Reference, SystemTime)>,
    result_statement: HashMap<Uuid, (ResultStatement, SystemTime)>,
    x_return: HashMap<Uuid, (XReturn, SystemTime)>,
    z_some: HashMap<Uuid, (ZSome, SystemTime)>,
    span: HashMap<Uuid, (Span, SystemTime)>,
    statement: HashMap<Uuid, (Statement, SystemTime)>,
    static_method_call: HashMap<Uuid, (StaticMethodCall, SystemTime)>,
    string_literal: HashMap<Uuid, (StringLiteral, SystemTime)>,
    woog_struct: HashMap<Uuid, (WoogStruct, SystemTime)>,
    woog_struct_id_by_name: HashMap<String, (Uuid, SystemTime)>,
    struct_expression: HashMap<Uuid, (StructExpression, SystemTime)>,
    type_cast: HashMap<Uuid, (TypeCast, SystemTime)>,
    unary: HashMap<Uuid, (Unary, SystemTime)>,
    x_value: HashMap<Uuid, (XValue, SystemTime)>,
    value_type: HashMap<Uuid, (ValueType, SystemTime)>,
    variable: HashMap<Uuid, (Variable, SystemTime)>,
    variable_expression: HashMap<Uuid, (VariableExpression, SystemTime)>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            argument: HashMap::default(),
            binary: HashMap::default(),
            block: HashMap::default(),
            boolean_literal: HashMap::default(),
            boolean_operator: HashMap::default(),
            call: HashMap::default(),
            comparison: HashMap::default(),
            dwarf_source_file: HashMap::default(),
            error: HashMap::default(),
            error_expression: HashMap::default(),
            expression: HashMap::default(),
            expression_statement: HashMap::default(),
            field: HashMap::default(),
            field_id_by_name: HashMap::default(),
            field_access: HashMap::default(),
            field_access_target: HashMap::default(),
            field_expression: HashMap::default(),
            float_literal: HashMap::default(),
            for_loop: HashMap::default(),
            function: HashMap::default(),
            function_id_by_name: HashMap::default(),
            grouped: HashMap::default(),
            x_if: HashMap::default(),
            implementation: HashMap::default(),
            import: HashMap::default(),
            index: HashMap::default(),
            integer_literal: HashMap::default(),
            item: HashMap::default(),
            let_statement: HashMap::default(),
            list: HashMap::default(),
            list_element: HashMap::default(),
            list_expression: HashMap::default(),
            literal: HashMap::default(),
            local_variable: HashMap::default(),
            x_macro: HashMap::default(),
            method_call: HashMap::default(),
            z_object_store: HashMap::default(),
            operator: HashMap::default(),
            woog_option: HashMap::default(),
            parameter: HashMap::default(),
            print: HashMap::default(),
            range_expression: HashMap::default(),
            reference: HashMap::default(),
            result_statement: HashMap::default(),
            x_return: HashMap::default(),
            z_some: HashMap::default(),
            span: HashMap::default(),
            statement: HashMap::default(),
            static_method_call: HashMap::default(),
            string_literal: HashMap::default(),
            woog_struct: HashMap::default(),
            woog_struct_id_by_name: HashMap::default(),
            struct_expression: HashMap::default(),
            type_cast: HashMap::default(),
            unary: HashMap::default(),
            x_value: HashMap::default(),
            value_type: HashMap::default(),
            variable: HashMap::default(),
            variable_expression: HashMap::default(),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥
        store.inter_binary(Binary::Addition(ADDITION));
        store.inter_binary(Binary::Assignment(ASSIGNMENT));
        store.inter_binary(Binary::BooleanOperator(BooleanOperator::And(AND).id()));
        store.inter_binary(Binary::Division(DIVISION));
        store.inter_binary(Binary::Multiplication(MULTIPLICATION));
        store.inter_binary(Binary::Subtraction(SUBTRACTION));
        store.inter_boolean_literal(BooleanLiteral::FalseLiteral(FALSE_LITERAL));
        store.inter_boolean_literal(BooleanLiteral::TrueLiteral(TRUE_LITERAL));
        store.inter_boolean_operator(BooleanOperator::And(AND));
        store.inter_comparison(Comparison::Equal(EQUAL));
        store.inter_comparison(Comparison::GreaterThan(GREATER_THAN));
        store.inter_comparison(Comparison::GreaterThanOrEqual(GREATER_THAN_OR_EQUAL));
        store.inter_comparison(Comparison::LessThanOrEqual(LESS_THAN_OR_EQUAL));
        store.inter_comparison(Comparison::NotEqual(NOT_EQUAL));
        store.inter_error(Error::UnknownVariable(UNKNOWN_VARIABLE));
        store.inter_expression(Expression::Debugger(DEBUGGER));
        store.inter_expression(Expression::Literal(
            Literal::BooleanLiteral(BooleanLiteral::FalseLiteral(FALSE_LITERAL).id()).id(),
        ));
        store.inter_expression(Expression::Literal(
            Literal::BooleanLiteral(BooleanLiteral::TrueLiteral(TRUE_LITERAL).id()).id(),
        ));
        store.inter_expression(Expression::ZNone(Z_NONE));
        store.inter_literal(Literal::BooleanLiteral(
            BooleanLiteral::FalseLiteral(FALSE_LITERAL).id(),
        ));
        store.inter_literal(Literal::BooleanLiteral(
            BooleanLiteral::TrueLiteral(TRUE_LITERAL).id(),
        ));
        store.inter_unary(Unary::Negation(NEGATION));
        store.inter_unary(Unary::Not(NOT));
        store.inter_value_type(ValueType::Empty(EMPTY));
        store.inter_value_type(ValueType::Error(
            Error::UnknownVariable(UNKNOWN_VARIABLE).id(),
        ));
        store.inter_value_type(ValueType::Range(RANGE));
        store.inter_value_type(ValueType::Unknown(UNKNOWN));

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_vanilla-object-store-methods"}}}
    /// Inter (insert) [`Argument`] into the store.
    ///
    pub fn inter_argument(&mut self, argument: Argument) {
        self.argument
            .insert(argument.id, (argument, SystemTime::now()));
    }

    /// Exhume (get) [`Argument`] from the store.
    ///
    pub fn exhume_argument(&self, id: &Uuid) -> Option<&Argument> {
        self.argument.get(id).map(|argument| &argument.0)
    }

    /// Exorcise (remove) [`Argument`] from the store.
    ///
    pub fn exorcise_argument(&mut self, id: &Uuid) -> Option<Argument> {
        self.argument.remove(id).map(|argument| argument.0)
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

    /// Inter (insert) [`Binary`] into the store.
    ///
    pub fn inter_binary(&mut self, binary: Binary) {
        self.binary.insert(binary.id(), (binary, SystemTime::now()));
    }

    /// Exhume (get) [`Binary`] from the store.
    ///
    pub fn exhume_binary(&self, id: &Uuid) -> Option<&Binary> {
        self.binary.get(id).map(|binary| &binary.0)
    }

    /// Exorcise (remove) [`Binary`] from the store.
    ///
    pub fn exorcise_binary(&mut self, id: &Uuid) -> Option<Binary> {
        self.binary.remove(id).map(|binary| binary.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Binary>`.
    ///
    pub fn iter_binary(&self) -> impl Iterator<Item = &Binary> {
        self.binary.values().map(|binary| &binary.0)
    }

    /// Get the timestamp for Binary.
    ///
    pub fn binary_timestamp(&self, binary: &Binary) -> SystemTime {
        self.binary
            .get(&binary.id())
            .map(|binary| binary.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Block`] into the store.
    ///
    pub fn inter_block(&mut self, block: Block) {
        self.block.insert(block.id, (block, SystemTime::now()));
    }

    /// Exhume (get) [`Block`] from the store.
    ///
    pub fn exhume_block(&self, id: &Uuid) -> Option<&Block> {
        self.block.get(id).map(|block| &block.0)
    }

    /// Exorcise (remove) [`Block`] from the store.
    ///
    pub fn exorcise_block(&mut self, id: &Uuid) -> Option<Block> {
        self.block.remove(id).map(|block| block.0)
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

    /// Inter (insert) [`BooleanLiteral`] into the store.
    ///
    pub fn inter_boolean_literal(&mut self, boolean_literal: BooleanLiteral) {
        self.boolean_literal
            .insert(boolean_literal.id(), (boolean_literal, SystemTime::now()));
    }

    /// Exhume (get) [`BooleanLiteral`] from the store.
    ///
    pub fn exhume_boolean_literal(&self, id: &Uuid) -> Option<&BooleanLiteral> {
        self.boolean_literal
            .get(id)
            .map(|boolean_literal| &boolean_literal.0)
    }

    /// Exorcise (remove) [`BooleanLiteral`] from the store.
    ///
    pub fn exorcise_boolean_literal(&mut self, id: &Uuid) -> Option<BooleanLiteral> {
        self.boolean_literal
            .remove(id)
            .map(|boolean_literal| boolean_literal.0)
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

    /// Inter (insert) [`BooleanOperator`] into the store.
    ///
    pub fn inter_boolean_operator(&mut self, boolean_operator: BooleanOperator) {
        self.boolean_operator
            .insert(boolean_operator.id(), (boolean_operator, SystemTime::now()));
    }

    /// Exhume (get) [`BooleanOperator`] from the store.
    ///
    pub fn exhume_boolean_operator(&self, id: &Uuid) -> Option<&BooleanOperator> {
        self.boolean_operator
            .get(id)
            .map(|boolean_operator| &boolean_operator.0)
    }

    /// Exorcise (remove) [`BooleanOperator`] from the store.
    ///
    pub fn exorcise_boolean_operator(&mut self, id: &Uuid) -> Option<BooleanOperator> {
        self.boolean_operator
            .remove(id)
            .map(|boolean_operator| boolean_operator.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanOperator>`.
    ///
    pub fn iter_boolean_operator(&self) -> impl Iterator<Item = &BooleanOperator> {
        self.boolean_operator
            .values()
            .map(|boolean_operator| &boolean_operator.0)
    }

    /// Get the timestamp for BooleanOperator.
    ///
    pub fn boolean_operator_timestamp(&self, boolean_operator: &BooleanOperator) -> SystemTime {
        self.boolean_operator
            .get(&boolean_operator.id())
            .map(|boolean_operator| boolean_operator.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Call`] into the store.
    ///
    pub fn inter_call(&mut self, call: Call) {
        self.call.insert(call.id, (call, SystemTime::now()));
    }

    /// Exhume (get) [`Call`] from the store.
    ///
    pub fn exhume_call(&self, id: &Uuid) -> Option<&Call> {
        self.call.get(id).map(|call| &call.0)
    }

    /// Exorcise (remove) [`Call`] from the store.
    ///
    pub fn exorcise_call(&mut self, id: &Uuid) -> Option<Call> {
        self.call.remove(id).map(|call| call.0)
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

    /// Inter (insert) [`Comparison`] into the store.
    ///
    pub fn inter_comparison(&mut self, comparison: Comparison) {
        self.comparison
            .insert(comparison.id(), (comparison, SystemTime::now()));
    }

    /// Exhume (get) [`Comparison`] from the store.
    ///
    pub fn exhume_comparison(&self, id: &Uuid) -> Option<&Comparison> {
        self.comparison.get(id).map(|comparison| &comparison.0)
    }

    /// Exorcise (remove) [`Comparison`] from the store.
    ///
    pub fn exorcise_comparison(&mut self, id: &Uuid) -> Option<Comparison> {
        self.comparison.remove(id).map(|comparison| comparison.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Comparison>`.
    ///
    pub fn iter_comparison(&self) -> impl Iterator<Item = &Comparison> {
        self.comparison.values().map(|comparison| &comparison.0)
    }

    /// Get the timestamp for Comparison.
    ///
    pub fn comparison_timestamp(&self, comparison: &Comparison) -> SystemTime {
        self.comparison
            .get(&comparison.id())
            .map(|comparison| comparison.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`DwarfSourceFile`] into the store.
    ///
    pub fn inter_dwarf_source_file(&mut self, dwarf_source_file: DwarfSourceFile) {
        self.dwarf_source_file
            .insert(dwarf_source_file.id, (dwarf_source_file, SystemTime::now()));
    }

    /// Exhume (get) [`DwarfSourceFile`] from the store.
    ///
    pub fn exhume_dwarf_source_file(&self, id: &Uuid) -> Option<&DwarfSourceFile> {
        self.dwarf_source_file
            .get(id)
            .map(|dwarf_source_file| &dwarf_source_file.0)
    }

    /// Exorcise (remove) [`DwarfSourceFile`] from the store.
    ///
    pub fn exorcise_dwarf_source_file(&mut self, id: &Uuid) -> Option<DwarfSourceFile> {
        self.dwarf_source_file
            .remove(id)
            .map(|dwarf_source_file| dwarf_source_file.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, DwarfSourceFile>`.
    ///
    pub fn iter_dwarf_source_file(&self) -> impl Iterator<Item = &DwarfSourceFile> {
        self.dwarf_source_file
            .values()
            .map(|dwarf_source_file| &dwarf_source_file.0)
    }

    /// Get the timestamp for DwarfSourceFile.
    ///
    pub fn dwarf_source_file_timestamp(&self, dwarf_source_file: &DwarfSourceFile) -> SystemTime {
        self.dwarf_source_file
            .get(&dwarf_source_file.id)
            .map(|dwarf_source_file| dwarf_source_file.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Error`] into the store.
    ///
    pub fn inter_error(&mut self, error: Error) {
        self.error.insert(error.id(), (error, SystemTime::now()));
    }

    /// Exhume (get) [`Error`] from the store.
    ///
    pub fn exhume_error(&self, id: &Uuid) -> Option<&Error> {
        self.error.get(id).map(|error| &error.0)
    }

    /// Exorcise (remove) [`Error`] from the store.
    ///
    pub fn exorcise_error(&mut self, id: &Uuid) -> Option<Error> {
        self.error.remove(id).map(|error| error.0)
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

    /// Inter (insert) [`ErrorExpression`] into the store.
    ///
    pub fn inter_error_expression(&mut self, error_expression: ErrorExpression) {
        self.error_expression
            .insert(error_expression.id, (error_expression, SystemTime::now()));
    }

    /// Exhume (get) [`ErrorExpression`] from the store.
    ///
    pub fn exhume_error_expression(&self, id: &Uuid) -> Option<&ErrorExpression> {
        self.error_expression
            .get(id)
            .map(|error_expression| &error_expression.0)
    }

    /// Exorcise (remove) [`ErrorExpression`] from the store.
    ///
    pub fn exorcise_error_expression(&mut self, id: &Uuid) -> Option<ErrorExpression> {
        self.error_expression
            .remove(id)
            .map(|error_expression| error_expression.0)
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

    /// Inter (insert) [`Expression`] into the store.
    ///
    pub fn inter_expression(&mut self, expression: Expression) {
        self.expression
            .insert(expression.id(), (expression, SystemTime::now()));
    }

    /// Exhume (get) [`Expression`] from the store.
    ///
    pub fn exhume_expression(&self, id: &Uuid) -> Option<&Expression> {
        self.expression.get(id).map(|expression| &expression.0)
    }

    /// Exorcise (remove) [`Expression`] from the store.
    ///
    pub fn exorcise_expression(&mut self, id: &Uuid) -> Option<Expression> {
        self.expression.remove(id).map(|expression| expression.0)
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

    /// Inter (insert) [`ExpressionStatement`] into the store.
    ///
    pub fn inter_expression_statement(&mut self, expression_statement: ExpressionStatement) {
        self.expression_statement.insert(
            expression_statement.id,
            (expression_statement, SystemTime::now()),
        );
    }

    /// Exhume (get) [`ExpressionStatement`] from the store.
    ///
    pub fn exhume_expression_statement(&self, id: &Uuid) -> Option<&ExpressionStatement> {
        self.expression_statement
            .get(id)
            .map(|expression_statement| &expression_statement.0)
    }

    /// Exorcise (remove) [`ExpressionStatement`] from the store.
    ///
    pub fn exorcise_expression_statement(&mut self, id: &Uuid) -> Option<ExpressionStatement> {
        self.expression_statement
            .remove(id)
            .map(|expression_statement| expression_statement.0)
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

    /// Inter (insert) [`Field`] into the store.
    ///
    pub fn inter_field(&mut self, field: Field) {
        let value = (field, SystemTime::now());
        self.field_id_by_name
            .insert(value.0.name.to_upper_camel_case(), (value.0.id, value.1));
        self.field.insert(value.0.id, value);
    }

    /// Exhume (get) [`Field`] from the store.
    ///
    pub fn exhume_field(&self, id: &Uuid) -> Option<&Field> {
        self.field.get(id).map(|field| &field.0)
    }

    /// Exorcise (remove) [`Field`] from the store.
    ///
    pub fn exorcise_field(&mut self, id: &Uuid) -> Option<Field> {
        self.field.remove(id).map(|field| field.0)
    }

    /// Exhume [`Field`] id from the store by name.
    ///
    pub fn exhume_field_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.field_id_by_name.get(name).map(|field| field.0)
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

    /// Inter (insert) [`FieldAccess`] into the store.
    ///
    pub fn inter_field_access(&mut self, field_access: FieldAccess) {
        self.field_access
            .insert(field_access.id, (field_access, SystemTime::now()));
    }

    /// Exhume (get) [`FieldAccess`] from the store.
    ///
    pub fn exhume_field_access(&self, id: &Uuid) -> Option<&FieldAccess> {
        self.field_access
            .get(id)
            .map(|field_access| &field_access.0)
    }

    /// Exorcise (remove) [`FieldAccess`] from the store.
    ///
    pub fn exorcise_field_access(&mut self, id: &Uuid) -> Option<FieldAccess> {
        self.field_access
            .remove(id)
            .map(|field_access| field_access.0)
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

    /// Inter (insert) [`FieldAccessTarget`] into the store.
    ///
    pub fn inter_field_access_target(&mut self, field_access_target: FieldAccessTarget) {
        self.field_access_target.insert(
            field_access_target.id(),
            (field_access_target, SystemTime::now()),
        );
    }

    /// Exhume (get) [`FieldAccessTarget`] from the store.
    ///
    pub fn exhume_field_access_target(&self, id: &Uuid) -> Option<&FieldAccessTarget> {
        self.field_access_target
            .get(id)
            .map(|field_access_target| &field_access_target.0)
    }

    /// Exorcise (remove) [`FieldAccessTarget`] from the store.
    ///
    pub fn exorcise_field_access_target(&mut self, id: &Uuid) -> Option<FieldAccessTarget> {
        self.field_access_target
            .remove(id)
            .map(|field_access_target| field_access_target.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccessTarget>`.
    ///
    pub fn iter_field_access_target(&self) -> impl Iterator<Item = &FieldAccessTarget> {
        self.field_access_target
            .values()
            .map(|field_access_target| &field_access_target.0)
    }

    /// Get the timestamp for FieldAccessTarget.
    ///
    pub fn field_access_target_timestamp(
        &self,
        field_access_target: &FieldAccessTarget,
    ) -> SystemTime {
        self.field_access_target
            .get(&field_access_target.id())
            .map(|field_access_target| field_access_target.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`FieldExpression`] into the store.
    ///
    pub fn inter_field_expression(&mut self, field_expression: FieldExpression) {
        self.field_expression
            .insert(field_expression.id, (field_expression, SystemTime::now()));
    }

    /// Exhume (get) [`FieldExpression`] from the store.
    ///
    pub fn exhume_field_expression(&self, id: &Uuid) -> Option<&FieldExpression> {
        self.field_expression
            .get(id)
            .map(|field_expression| &field_expression.0)
    }

    /// Exorcise (remove) [`FieldExpression`] from the store.
    ///
    pub fn exorcise_field_expression(&mut self, id: &Uuid) -> Option<FieldExpression> {
        self.field_expression
            .remove(id)
            .map(|field_expression| field_expression.0)
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

    /// Inter (insert) [`FloatLiteral`] into the store.
    ///
    pub fn inter_float_literal(&mut self, float_literal: FloatLiteral) {
        self.float_literal
            .insert(float_literal.id, (float_literal, SystemTime::now()));
    }

    /// Exhume (get) [`FloatLiteral`] from the store.
    ///
    pub fn exhume_float_literal(&self, id: &Uuid) -> Option<&FloatLiteral> {
        self.float_literal
            .get(id)
            .map(|float_literal| &float_literal.0)
    }

    /// Exorcise (remove) [`FloatLiteral`] from the store.
    ///
    pub fn exorcise_float_literal(&mut self, id: &Uuid) -> Option<FloatLiteral> {
        self.float_literal
            .remove(id)
            .map(|float_literal| float_literal.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FloatLiteral>`.
    ///
    pub fn iter_float_literal(&self) -> impl Iterator<Item = &FloatLiteral> {
        self.float_literal
            .values()
            .map(|float_literal| &float_literal.0)
    }

    /// Get the timestamp for FloatLiteral.
    ///
    pub fn float_literal_timestamp(&self, float_literal: &FloatLiteral) -> SystemTime {
        self.float_literal
            .get(&float_literal.id)
            .map(|float_literal| float_literal.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ForLoop`] into the store.
    ///
    pub fn inter_for_loop(&mut self, for_loop: ForLoop) {
        self.for_loop
            .insert(for_loop.id, (for_loop, SystemTime::now()));
    }

    /// Exhume (get) [`ForLoop`] from the store.
    ///
    pub fn exhume_for_loop(&self, id: &Uuid) -> Option<&ForLoop> {
        self.for_loop.get(id).map(|for_loop| &for_loop.0)
    }

    /// Exorcise (remove) [`ForLoop`] from the store.
    ///
    pub fn exorcise_for_loop(&mut self, id: &Uuid) -> Option<ForLoop> {
        self.for_loop.remove(id).map(|for_loop| for_loop.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ForLoop>`.
    ///
    pub fn iter_for_loop(&self) -> impl Iterator<Item = &ForLoop> {
        self.for_loop.values().map(|for_loop| &for_loop.0)
    }

    /// Get the timestamp for ForLoop.
    ///
    pub fn for_loop_timestamp(&self, for_loop: &ForLoop) -> SystemTime {
        self.for_loop
            .get(&for_loop.id)
            .map(|for_loop| for_loop.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Function`] into the store.
    ///
    pub fn inter_function(&mut self, function: Function) {
        let value = (function, SystemTime::now());
        self.function_id_by_name
            .insert(value.0.name.to_upper_camel_case(), (value.0.id, value.1));
        self.function.insert(value.0.id, value);
    }

    /// Exhume (get) [`Function`] from the store.
    ///
    pub fn exhume_function(&self, id: &Uuid) -> Option<&Function> {
        self.function.get(id).map(|function| &function.0)
    }

    /// Exorcise (remove) [`Function`] from the store.
    ///
    pub fn exorcise_function(&mut self, id: &Uuid) -> Option<Function> {
        self.function.remove(id).map(|function| function.0)
    }

    /// Exhume [`Function`] id from the store by name.
    ///
    pub fn exhume_function_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.function_id_by_name
            .get(name)
            .map(|function| function.0)
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

    /// Inter (insert) [`Grouped`] into the store.
    ///
    pub fn inter_grouped(&mut self, grouped: Grouped) {
        self.grouped
            .insert(grouped.id, (grouped, SystemTime::now()));
    }

    /// Exhume (get) [`Grouped`] from the store.
    ///
    pub fn exhume_grouped(&self, id: &Uuid) -> Option<&Grouped> {
        self.grouped.get(id).map(|grouped| &grouped.0)
    }

    /// Exorcise (remove) [`Grouped`] from the store.
    ///
    pub fn exorcise_grouped(&mut self, id: &Uuid) -> Option<Grouped> {
        self.grouped.remove(id).map(|grouped| grouped.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Grouped>`.
    ///
    pub fn iter_grouped(&self) -> impl Iterator<Item = &Grouped> {
        self.grouped.values().map(|grouped| &grouped.0)
    }

    /// Get the timestamp for Grouped.
    ///
    pub fn grouped_timestamp(&self, grouped: &Grouped) -> SystemTime {
        self.grouped
            .get(&grouped.id)
            .map(|grouped| grouped.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`XIf`] into the store.
    ///
    pub fn inter_x_if(&mut self, x_if: XIf) {
        self.x_if.insert(x_if.id, (x_if, SystemTime::now()));
    }

    /// Exhume (get) [`XIf`] from the store.
    ///
    pub fn exhume_x_if(&self, id: &Uuid) -> Option<&XIf> {
        self.x_if.get(id).map(|x_if| &x_if.0)
    }

    /// Exorcise (remove) [`XIf`] from the store.
    ///
    pub fn exorcise_x_if(&mut self, id: &Uuid) -> Option<XIf> {
        self.x_if.remove(id).map(|x_if| x_if.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XIf>`.
    ///
    pub fn iter_x_if(&self) -> impl Iterator<Item = &XIf> {
        self.x_if.values().map(|x_if| &x_if.0)
    }

    /// Get the timestamp for XIf.
    ///
    pub fn x_if_timestamp(&self, x_if: &XIf) -> SystemTime {
        self.x_if
            .get(&x_if.id)
            .map(|x_if| x_if.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Implementation`] into the store.
    ///
    pub fn inter_implementation(&mut self, implementation: Implementation) {
        self.implementation
            .insert(implementation.id, (implementation, SystemTime::now()));
    }

    /// Exhume (get) [`Implementation`] from the store.
    ///
    pub fn exhume_implementation(&self, id: &Uuid) -> Option<&Implementation> {
        self.implementation
            .get(id)
            .map(|implementation| &implementation.0)
    }

    /// Exorcise (remove) [`Implementation`] from the store.
    ///
    pub fn exorcise_implementation(&mut self, id: &Uuid) -> Option<Implementation> {
        self.implementation
            .remove(id)
            .map(|implementation| implementation.0)
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

    /// Inter (insert) [`Import`] into the store.
    ///
    pub fn inter_import(&mut self, import: Import) {
        self.import.insert(import.id, (import, SystemTime::now()));
    }

    /// Exhume (get) [`Import`] from the store.
    ///
    pub fn exhume_import(&self, id: &Uuid) -> Option<&Import> {
        self.import.get(id).map(|import| &import.0)
    }

    /// Exorcise (remove) [`Import`] from the store.
    ///
    pub fn exorcise_import(&mut self, id: &Uuid) -> Option<Import> {
        self.import.remove(id).map(|import| import.0)
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

    /// Inter (insert) [`Index`] into the store.
    ///
    pub fn inter_index(&mut self, index: Index) {
        self.index.insert(index.id, (index, SystemTime::now()));
    }

    /// Exhume (get) [`Index`] from the store.
    ///
    pub fn exhume_index(&self, id: &Uuid) -> Option<&Index> {
        self.index.get(id).map(|index| &index.0)
    }

    /// Exorcise (remove) [`Index`] from the store.
    ///
    pub fn exorcise_index(&mut self, id: &Uuid) -> Option<Index> {
        self.index.remove(id).map(|index| index.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Index>`.
    ///
    pub fn iter_index(&self) -> impl Iterator<Item = &Index> {
        self.index.values().map(|index| &index.0)
    }

    /// Get the timestamp for Index.
    ///
    pub fn index_timestamp(&self, index: &Index) -> SystemTime {
        self.index
            .get(&index.id)
            .map(|index| index.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`IntegerLiteral`] into the store.
    ///
    pub fn inter_integer_literal(&mut self, integer_literal: IntegerLiteral) {
        self.integer_literal
            .insert(integer_literal.id, (integer_literal, SystemTime::now()));
    }

    /// Exhume (get) [`IntegerLiteral`] from the store.
    ///
    pub fn exhume_integer_literal(&self, id: &Uuid) -> Option<&IntegerLiteral> {
        self.integer_literal
            .get(id)
            .map(|integer_literal| &integer_literal.0)
    }

    /// Exorcise (remove) [`IntegerLiteral`] from the store.
    ///
    pub fn exorcise_integer_literal(&mut self, id: &Uuid) -> Option<IntegerLiteral> {
        self.integer_literal
            .remove(id)
            .map(|integer_literal| integer_literal.0)
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

    /// Inter (insert) [`Item`] into the store.
    ///
    pub fn inter_item(&mut self, item: Item) {
        self.item.insert(item.id, (item, SystemTime::now()));
    }

    /// Exhume (get) [`Item`] from the store.
    ///
    pub fn exhume_item(&self, id: &Uuid) -> Option<&Item> {
        self.item.get(id).map(|item| &item.0)
    }

    /// Exorcise (remove) [`Item`] from the store.
    ///
    pub fn exorcise_item(&mut self, id: &Uuid) -> Option<Item> {
        self.item.remove(id).map(|item| item.0)
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
            .get(&item.id)
            .map(|item| item.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`LetStatement`] into the store.
    ///
    pub fn inter_let_statement(&mut self, let_statement: LetStatement) {
        self.let_statement
            .insert(let_statement.id, (let_statement, SystemTime::now()));
    }

    /// Exhume (get) [`LetStatement`] from the store.
    ///
    pub fn exhume_let_statement(&self, id: &Uuid) -> Option<&LetStatement> {
        self.let_statement
            .get(id)
            .map(|let_statement| &let_statement.0)
    }

    /// Exorcise (remove) [`LetStatement`] from the store.
    ///
    pub fn exorcise_let_statement(&mut self, id: &Uuid) -> Option<LetStatement> {
        self.let_statement
            .remove(id)
            .map(|let_statement| let_statement.0)
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

    /// Inter (insert) [`List`] into the store.
    ///
    pub fn inter_list(&mut self, list: List) {
        self.list.insert(list.id, (list, SystemTime::now()));
    }

    /// Exhume (get) [`List`] from the store.
    ///
    pub fn exhume_list(&self, id: &Uuid) -> Option<&List> {
        self.list.get(id).map(|list| &list.0)
    }

    /// Exorcise (remove) [`List`] from the store.
    ///
    pub fn exorcise_list(&mut self, id: &Uuid) -> Option<List> {
        self.list.remove(id).map(|list| list.0)
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

    /// Inter (insert) [`ListElement`] into the store.
    ///
    pub fn inter_list_element(&mut self, list_element: ListElement) {
        self.list_element
            .insert(list_element.id, (list_element, SystemTime::now()));
    }

    /// Exhume (get) [`ListElement`] from the store.
    ///
    pub fn exhume_list_element(&self, id: &Uuid) -> Option<&ListElement> {
        self.list_element
            .get(id)
            .map(|list_element| &list_element.0)
    }

    /// Exorcise (remove) [`ListElement`] from the store.
    ///
    pub fn exorcise_list_element(&mut self, id: &Uuid) -> Option<ListElement> {
        self.list_element
            .remove(id)
            .map(|list_element| list_element.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListElement>`.
    ///
    pub fn iter_list_element(&self) -> impl Iterator<Item = &ListElement> {
        self.list_element
            .values()
            .map(|list_element| &list_element.0)
    }

    /// Get the timestamp for ListElement.
    ///
    pub fn list_element_timestamp(&self, list_element: &ListElement) -> SystemTime {
        self.list_element
            .get(&list_element.id)
            .map(|list_element| list_element.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ListExpression`] into the store.
    ///
    pub fn inter_list_expression(&mut self, list_expression: ListExpression) {
        self.list_expression
            .insert(list_expression.id, (list_expression, SystemTime::now()));
    }

    /// Exhume (get) [`ListExpression`] from the store.
    ///
    pub fn exhume_list_expression(&self, id: &Uuid) -> Option<&ListExpression> {
        self.list_expression
            .get(id)
            .map(|list_expression| &list_expression.0)
    }

    /// Exorcise (remove) [`ListExpression`] from the store.
    ///
    pub fn exorcise_list_expression(&mut self, id: &Uuid) -> Option<ListExpression> {
        self.list_expression
            .remove(id)
            .map(|list_expression| list_expression.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListExpression>`.
    ///
    pub fn iter_list_expression(&self) -> impl Iterator<Item = &ListExpression> {
        self.list_expression
            .values()
            .map(|list_expression| &list_expression.0)
    }

    /// Get the timestamp for ListExpression.
    ///
    pub fn list_expression_timestamp(&self, list_expression: &ListExpression) -> SystemTime {
        self.list_expression
            .get(&list_expression.id)
            .map(|list_expression| list_expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Literal`] into the store.
    ///
    pub fn inter_literal(&mut self, literal: Literal) {
        self.literal
            .insert(literal.id(), (literal, SystemTime::now()));
    }

    /// Exhume (get) [`Literal`] from the store.
    ///
    pub fn exhume_literal(&self, id: &Uuid) -> Option<&Literal> {
        self.literal.get(id).map(|literal| &literal.0)
    }

    /// Exorcise (remove) [`Literal`] from the store.
    ///
    pub fn exorcise_literal(&mut self, id: &Uuid) -> Option<Literal> {
        self.literal.remove(id).map(|literal| literal.0)
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

    /// Inter (insert) [`LocalVariable`] into the store.
    ///
    pub fn inter_local_variable(&mut self, local_variable: LocalVariable) {
        self.local_variable
            .insert(local_variable.id, (local_variable, SystemTime::now()));
    }

    /// Exhume (get) [`LocalVariable`] from the store.
    ///
    pub fn exhume_local_variable(&self, id: &Uuid) -> Option<&LocalVariable> {
        self.local_variable
            .get(id)
            .map(|local_variable| &local_variable.0)
    }

    /// Exorcise (remove) [`LocalVariable`] from the store.
    ///
    pub fn exorcise_local_variable(&mut self, id: &Uuid) -> Option<LocalVariable> {
        self.local_variable
            .remove(id)
            .map(|local_variable| local_variable.0)
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

    /// Inter (insert) [`XMacro`] into the store.
    ///
    pub fn inter_x_macro(&mut self, x_macro: XMacro) {
        self.x_macro
            .insert(x_macro.id, (x_macro, SystemTime::now()));
    }

    /// Exhume (get) [`XMacro`] from the store.
    ///
    pub fn exhume_x_macro(&self, id: &Uuid) -> Option<&XMacro> {
        self.x_macro.get(id).map(|x_macro| &x_macro.0)
    }

    /// Exorcise (remove) [`XMacro`] from the store.
    ///
    pub fn exorcise_x_macro(&mut self, id: &Uuid) -> Option<XMacro> {
        self.x_macro.remove(id).map(|x_macro| x_macro.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XMacro>`.
    ///
    pub fn iter_x_macro(&self) -> impl Iterator<Item = &XMacro> {
        self.x_macro.values().map(|x_macro| &x_macro.0)
    }

    /// Get the timestamp for XMacro.
    ///
    pub fn x_macro_timestamp(&self, x_macro: &XMacro) -> SystemTime {
        self.x_macro
            .get(&x_macro.id)
            .map(|x_macro| x_macro.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`MethodCall`] into the store.
    ///
    pub fn inter_method_call(&mut self, method_call: MethodCall) {
        self.method_call
            .insert(method_call.id, (method_call, SystemTime::now()));
    }

    /// Exhume (get) [`MethodCall`] from the store.
    ///
    pub fn exhume_method_call(&self, id: &Uuid) -> Option<&MethodCall> {
        self.method_call.get(id).map(|method_call| &method_call.0)
    }

    /// Exorcise (remove) [`MethodCall`] from the store.
    ///
    pub fn exorcise_method_call(&mut self, id: &Uuid) -> Option<MethodCall> {
        self.method_call.remove(id).map(|method_call| method_call.0)
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

    /// Inter (insert) [`ZObjectStore`] into the store.
    ///
    pub fn inter_z_object_store(&mut self, z_object_store: ZObjectStore) {
        self.z_object_store
            .insert(z_object_store.id, (z_object_store, SystemTime::now()));
    }

    /// Exhume (get) [`ZObjectStore`] from the store.
    ///
    pub fn exhume_z_object_store(&self, id: &Uuid) -> Option<&ZObjectStore> {
        self.z_object_store
            .get(id)
            .map(|z_object_store| &z_object_store.0)
    }

    /// Exorcise (remove) [`ZObjectStore`] from the store.
    ///
    pub fn exorcise_z_object_store(&mut self, id: &Uuid) -> Option<ZObjectStore> {
        self.z_object_store
            .remove(id)
            .map(|z_object_store| z_object_store.0)
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

    /// Inter (insert) [`Operator`] into the store.
    ///
    pub fn inter_operator(&mut self, operator: Operator) {
        self.operator
            .insert(operator.id, (operator, SystemTime::now()));
    }

    /// Exhume (get) [`Operator`] from the store.
    ///
    pub fn exhume_operator(&self, id: &Uuid) -> Option<&Operator> {
        self.operator.get(id).map(|operator| &operator.0)
    }

    /// Exorcise (remove) [`Operator`] from the store.
    ///
    pub fn exorcise_operator(&mut self, id: &Uuid) -> Option<Operator> {
        self.operator.remove(id).map(|operator| operator.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Operator>`.
    ///
    pub fn iter_operator(&self) -> impl Iterator<Item = &Operator> {
        self.operator.values().map(|operator| &operator.0)
    }

    /// Get the timestamp for Operator.
    ///
    pub fn operator_timestamp(&self, operator: &Operator) -> SystemTime {
        self.operator
            .get(&operator.id)
            .map(|operator| operator.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`WoogOption`] into the store.
    ///
    pub fn inter_woog_option(&mut self, woog_option: WoogOption) {
        self.woog_option
            .insert(woog_option.id, (woog_option, SystemTime::now()));
    }

    /// Exhume (get) [`WoogOption`] from the store.
    ///
    pub fn exhume_woog_option(&self, id: &Uuid) -> Option<&WoogOption> {
        self.woog_option.get(id).map(|woog_option| &woog_option.0)
    }

    /// Exorcise (remove) [`WoogOption`] from the store.
    ///
    pub fn exorcise_woog_option(&mut self, id: &Uuid) -> Option<WoogOption> {
        self.woog_option.remove(id).map(|woog_option| woog_option.0)
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

    /// Inter (insert) [`Parameter`] into the store.
    ///
    pub fn inter_parameter(&mut self, parameter: Parameter) {
        self.parameter
            .insert(parameter.id, (parameter, SystemTime::now()));
    }

    /// Exhume (get) [`Parameter`] from the store.
    ///
    pub fn exhume_parameter(&self, id: &Uuid) -> Option<&Parameter> {
        self.parameter.get(id).map(|parameter| &parameter.0)
    }

    /// Exorcise (remove) [`Parameter`] from the store.
    ///
    pub fn exorcise_parameter(&mut self, id: &Uuid) -> Option<Parameter> {
        self.parameter.remove(id).map(|parameter| parameter.0)
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

    /// Inter (insert) [`Print`] into the store.
    ///
    pub fn inter_print(&mut self, print: Print) {
        self.print.insert(print.id, (print, SystemTime::now()));
    }

    /// Exhume (get) [`Print`] from the store.
    ///
    pub fn exhume_print(&self, id: &Uuid) -> Option<&Print> {
        self.print.get(id).map(|print| &print.0)
    }

    /// Exorcise (remove) [`Print`] from the store.
    ///
    pub fn exorcise_print(&mut self, id: &Uuid) -> Option<Print> {
        self.print.remove(id).map(|print| print.0)
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

    /// Inter (insert) [`RangeExpression`] into the store.
    ///
    pub fn inter_range_expression(&mut self, range_expression: RangeExpression) {
        self.range_expression
            .insert(range_expression.id, (range_expression, SystemTime::now()));
    }

    /// Exhume (get) [`RangeExpression`] from the store.
    ///
    pub fn exhume_range_expression(&self, id: &Uuid) -> Option<&RangeExpression> {
        self.range_expression
            .get(id)
            .map(|range_expression| &range_expression.0)
    }

    /// Exorcise (remove) [`RangeExpression`] from the store.
    ///
    pub fn exorcise_range_expression(&mut self, id: &Uuid) -> Option<RangeExpression> {
        self.range_expression
            .remove(id)
            .map(|range_expression| range_expression.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RangeExpression>`.
    ///
    pub fn iter_range_expression(&self) -> impl Iterator<Item = &RangeExpression> {
        self.range_expression
            .values()
            .map(|range_expression| &range_expression.0)
    }

    /// Get the timestamp for RangeExpression.
    ///
    pub fn range_expression_timestamp(&self, range_expression: &RangeExpression) -> SystemTime {
        self.range_expression
            .get(&range_expression.id)
            .map(|range_expression| range_expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Reference`] into the store.
    ///
    pub fn inter_reference(&mut self, reference: Reference) {
        self.reference
            .insert(reference.id, (reference, SystemTime::now()));
    }

    /// Exhume (get) [`Reference`] from the store.
    ///
    pub fn exhume_reference(&self, id: &Uuid) -> Option<&Reference> {
        self.reference.get(id).map(|reference| &reference.0)
    }

    /// Exorcise (remove) [`Reference`] from the store.
    ///
    pub fn exorcise_reference(&mut self, id: &Uuid) -> Option<Reference> {
        self.reference.remove(id).map(|reference| reference.0)
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

    /// Inter (insert) [`ResultStatement`] into the store.
    ///
    pub fn inter_result_statement(&mut self, result_statement: ResultStatement) {
        self.result_statement
            .insert(result_statement.id, (result_statement, SystemTime::now()));
    }

    /// Exhume (get) [`ResultStatement`] from the store.
    ///
    pub fn exhume_result_statement(&self, id: &Uuid) -> Option<&ResultStatement> {
        self.result_statement
            .get(id)
            .map(|result_statement| &result_statement.0)
    }

    /// Exorcise (remove) [`ResultStatement`] from the store.
    ///
    pub fn exorcise_result_statement(&mut self, id: &Uuid) -> Option<ResultStatement> {
        self.result_statement
            .remove(id)
            .map(|result_statement| result_statement.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ResultStatement>`.
    ///
    pub fn iter_result_statement(&self) -> impl Iterator<Item = &ResultStatement> {
        self.result_statement
            .values()
            .map(|result_statement| &result_statement.0)
    }

    /// Get the timestamp for ResultStatement.
    ///
    pub fn result_statement_timestamp(&self, result_statement: &ResultStatement) -> SystemTime {
        self.result_statement
            .get(&result_statement.id)
            .map(|result_statement| result_statement.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`XReturn`] into the store.
    ///
    pub fn inter_x_return(&mut self, x_return: XReturn) {
        self.x_return
            .insert(x_return.id, (x_return, SystemTime::now()));
    }

    /// Exhume (get) [`XReturn`] from the store.
    ///
    pub fn exhume_x_return(&self, id: &Uuid) -> Option<&XReturn> {
        self.x_return.get(id).map(|x_return| &x_return.0)
    }

    /// Exorcise (remove) [`XReturn`] from the store.
    ///
    pub fn exorcise_x_return(&mut self, id: &Uuid) -> Option<XReturn> {
        self.x_return.remove(id).map(|x_return| x_return.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XReturn>`.
    ///
    pub fn iter_x_return(&self) -> impl Iterator<Item = &XReturn> {
        self.x_return.values().map(|x_return| &x_return.0)
    }

    /// Get the timestamp for XReturn.
    ///
    pub fn x_return_timestamp(&self, x_return: &XReturn) -> SystemTime {
        self.x_return
            .get(&x_return.id)
            .map(|x_return| x_return.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ZSome`] into the store.
    ///
    pub fn inter_z_some(&mut self, z_some: ZSome) {
        self.z_some.insert(z_some.id, (z_some, SystemTime::now()));
    }

    /// Exhume (get) [`ZSome`] from the store.
    ///
    pub fn exhume_z_some(&self, id: &Uuid) -> Option<&ZSome> {
        self.z_some.get(id).map(|z_some| &z_some.0)
    }

    /// Exorcise (remove) [`ZSome`] from the store.
    ///
    pub fn exorcise_z_some(&mut self, id: &Uuid) -> Option<ZSome> {
        self.z_some.remove(id).map(|z_some| z_some.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ZSome>`.
    ///
    pub fn iter_z_some(&self) -> impl Iterator<Item = &ZSome> {
        self.z_some.values().map(|z_some| &z_some.0)
    }

    /// Get the timestamp for ZSome.
    ///
    pub fn z_some_timestamp(&self, z_some: &ZSome) -> SystemTime {
        self.z_some
            .get(&z_some.id)
            .map(|z_some| z_some.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Span`] into the store.
    ///
    pub fn inter_span(&mut self, span: Span) {
        self.span.insert(span.id, (span, SystemTime::now()));
    }

    /// Exhume (get) [`Span`] from the store.
    ///
    pub fn exhume_span(&self, id: &Uuid) -> Option<&Span> {
        self.span.get(id).map(|span| &span.0)
    }

    /// Exorcise (remove) [`Span`] from the store.
    ///
    pub fn exorcise_span(&mut self, id: &Uuid) -> Option<Span> {
        self.span.remove(id).map(|span| span.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Span>`.
    ///
    pub fn iter_span(&self) -> impl Iterator<Item = &Span> {
        self.span.values().map(|span| &span.0)
    }

    /// Get the timestamp for Span.
    ///
    pub fn span_timestamp(&self, span: &Span) -> SystemTime {
        self.span
            .get(&span.id)
            .map(|span| span.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Statement`] into the store.
    ///
    pub fn inter_statement(&mut self, statement: Statement) {
        self.statement
            .insert(statement.id, (statement, SystemTime::now()));
    }

    /// Exhume (get) [`Statement`] from the store.
    ///
    pub fn exhume_statement(&self, id: &Uuid) -> Option<&Statement> {
        self.statement.get(id).map(|statement| &statement.0)
    }

    /// Exorcise (remove) [`Statement`] from the store.
    ///
    pub fn exorcise_statement(&mut self, id: &Uuid) -> Option<Statement> {
        self.statement.remove(id).map(|statement| statement.0)
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

    /// Inter (insert) [`StaticMethodCall`] into the store.
    ///
    pub fn inter_static_method_call(&mut self, static_method_call: StaticMethodCall) {
        self.static_method_call.insert(
            static_method_call.id,
            (static_method_call, SystemTime::now()),
        );
    }

    /// Exhume (get) [`StaticMethodCall`] from the store.
    ///
    pub fn exhume_static_method_call(&self, id: &Uuid) -> Option<&StaticMethodCall> {
        self.static_method_call
            .get(id)
            .map(|static_method_call| &static_method_call.0)
    }

    /// Exorcise (remove) [`StaticMethodCall`] from the store.
    ///
    pub fn exorcise_static_method_call(&mut self, id: &Uuid) -> Option<StaticMethodCall> {
        self.static_method_call
            .remove(id)
            .map(|static_method_call| static_method_call.0)
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

    /// Inter (insert) [`StringLiteral`] into the store.
    ///
    pub fn inter_string_literal(&mut self, string_literal: StringLiteral) {
        self.string_literal
            .insert(string_literal.id, (string_literal, SystemTime::now()));
    }

    /// Exhume (get) [`StringLiteral`] from the store.
    ///
    pub fn exhume_string_literal(&self, id: &Uuid) -> Option<&StringLiteral> {
        self.string_literal
            .get(id)
            .map(|string_literal| &string_literal.0)
    }

    /// Exorcise (remove) [`StringLiteral`] from the store.
    ///
    pub fn exorcise_string_literal(&mut self, id: &Uuid) -> Option<StringLiteral> {
        self.string_literal
            .remove(id)
            .map(|string_literal| string_literal.0)
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

    /// Inter (insert) [`WoogStruct`] into the store.
    ///
    pub fn inter_woog_struct(&mut self, woog_struct: WoogStruct) {
        let value = (woog_struct, SystemTime::now());
        self.woog_struct_id_by_name
            .insert(value.0.name.to_upper_camel_case(), (value.0.id, value.1));
        self.woog_struct.insert(value.0.id, value);
    }

    /// Exhume (get) [`WoogStruct`] from the store.
    ///
    pub fn exhume_woog_struct(&self, id: &Uuid) -> Option<&WoogStruct> {
        self.woog_struct.get(id).map(|woog_struct| &woog_struct.0)
    }

    /// Exorcise (remove) [`WoogStruct`] from the store.
    ///
    pub fn exorcise_woog_struct(&mut self, id: &Uuid) -> Option<WoogStruct> {
        self.woog_struct.remove(id).map(|woog_struct| woog_struct.0)
    }

    /// Exhume [`WoogStruct`] id from the store by name.
    ///
    pub fn exhume_woog_struct_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.woog_struct_id_by_name
            .get(name)
            .map(|woog_struct| woog_struct.0)
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

    /// Inter (insert) [`StructExpression`] into the store.
    ///
    pub fn inter_struct_expression(&mut self, struct_expression: StructExpression) {
        self.struct_expression
            .insert(struct_expression.id, (struct_expression, SystemTime::now()));
    }

    /// Exhume (get) [`StructExpression`] from the store.
    ///
    pub fn exhume_struct_expression(&self, id: &Uuid) -> Option<&StructExpression> {
        self.struct_expression
            .get(id)
            .map(|struct_expression| &struct_expression.0)
    }

    /// Exorcise (remove) [`StructExpression`] from the store.
    ///
    pub fn exorcise_struct_expression(&mut self, id: &Uuid) -> Option<StructExpression> {
        self.struct_expression
            .remove(id)
            .map(|struct_expression| struct_expression.0)
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

    /// Inter (insert) [`TypeCast`] into the store.
    ///
    pub fn inter_type_cast(&mut self, type_cast: TypeCast) {
        self.type_cast
            .insert(type_cast.id, (type_cast, SystemTime::now()));
    }

    /// Exhume (get) [`TypeCast`] from the store.
    ///
    pub fn exhume_type_cast(&self, id: &Uuid) -> Option<&TypeCast> {
        self.type_cast.get(id).map(|type_cast| &type_cast.0)
    }

    /// Exorcise (remove) [`TypeCast`] from the store.
    ///
    pub fn exorcise_type_cast(&mut self, id: &Uuid) -> Option<TypeCast> {
        self.type_cast.remove(id).map(|type_cast| type_cast.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, TypeCast>`.
    ///
    pub fn iter_type_cast(&self) -> impl Iterator<Item = &TypeCast> {
        self.type_cast.values().map(|type_cast| &type_cast.0)
    }

    /// Get the timestamp for TypeCast.
    ///
    pub fn type_cast_timestamp(&self, type_cast: &TypeCast) -> SystemTime {
        self.type_cast
            .get(&type_cast.id)
            .map(|type_cast| type_cast.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Unary`] into the store.
    ///
    pub fn inter_unary(&mut self, unary: Unary) {
        self.unary.insert(unary.id(), (unary, SystemTime::now()));
    }

    /// Exhume (get) [`Unary`] from the store.
    ///
    pub fn exhume_unary(&self, id: &Uuid) -> Option<&Unary> {
        self.unary.get(id).map(|unary| &unary.0)
    }

    /// Exorcise (remove) [`Unary`] from the store.
    ///
    pub fn exorcise_unary(&mut self, id: &Uuid) -> Option<Unary> {
        self.unary.remove(id).map(|unary| unary.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Unary>`.
    ///
    pub fn iter_unary(&self) -> impl Iterator<Item = &Unary> {
        self.unary.values().map(|unary| &unary.0)
    }

    /// Get the timestamp for Unary.
    ///
    pub fn unary_timestamp(&self, unary: &Unary) -> SystemTime {
        self.unary
            .get(&unary.id())
            .map(|unary| unary.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`XValue`] into the store.
    ///
    pub fn inter_x_value(&mut self, x_value: XValue) {
        self.x_value
            .insert(x_value.id, (x_value, SystemTime::now()));
    }

    /// Exhume (get) [`XValue`] from the store.
    ///
    pub fn exhume_x_value(&self, id: &Uuid) -> Option<&XValue> {
        self.x_value.get(id).map(|x_value| &x_value.0)
    }

    /// Exorcise (remove) [`XValue`] from the store.
    ///
    pub fn exorcise_x_value(&mut self, id: &Uuid) -> Option<XValue> {
        self.x_value.remove(id).map(|x_value| x_value.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XValue>`.
    ///
    pub fn iter_x_value(&self) -> impl Iterator<Item = &XValue> {
        self.x_value.values().map(|x_value| &x_value.0)
    }

    /// Get the timestamp for XValue.
    ///
    pub fn x_value_timestamp(&self, x_value: &XValue) -> SystemTime {
        self.x_value
            .get(&x_value.id)
            .map(|x_value| x_value.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ValueType`] into the store.
    ///
    pub fn inter_value_type(&mut self, value_type: ValueType) {
        self.value_type
            .insert(value_type.id(), (value_type, SystemTime::now()));
    }

    /// Exhume (get) [`ValueType`] from the store.
    ///
    pub fn exhume_value_type(&self, id: &Uuid) -> Option<&ValueType> {
        self.value_type.get(id).map(|value_type| &value_type.0)
    }

    /// Exorcise (remove) [`ValueType`] from the store.
    ///
    pub fn exorcise_value_type(&mut self, id: &Uuid) -> Option<ValueType> {
        self.value_type.remove(id).map(|value_type| value_type.0)
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

    /// Inter (insert) [`Variable`] into the store.
    ///
    pub fn inter_variable(&mut self, variable: Variable) {
        self.variable
            .insert(variable.id, (variable, SystemTime::now()));
    }

    /// Exhume (get) [`Variable`] from the store.
    ///
    pub fn exhume_variable(&self, id: &Uuid) -> Option<&Variable> {
        self.variable.get(id).map(|variable| &variable.0)
    }

    /// Exorcise (remove) [`Variable`] from the store.
    ///
    pub fn exorcise_variable(&mut self, id: &Uuid) -> Option<Variable> {
        self.variable.remove(id).map(|variable| variable.0)
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

    /// Inter (insert) [`VariableExpression`] into the store.
    ///
    pub fn inter_variable_expression(&mut self, variable_expression: VariableExpression) {
        self.variable_expression.insert(
            variable_expression.id,
            (variable_expression, SystemTime::now()),
        );
    }

    /// Exhume (get) [`VariableExpression`] from the store.
    ///
    pub fn exhume_variable_expression(&self, id: &Uuid) -> Option<&VariableExpression> {
        self.variable_expression
            .get(id)
            .map(|variable_expression| &variable_expression.0)
    }

    /// Exorcise (remove) [`VariableExpression`] from the store.
    ///
    pub fn exorcise_variable_expression(&mut self, id: &Uuid) -> Option<VariableExpression> {
        self.variable_expression
            .remove(id)
            .map(|variable_expression| variable_expression.0)
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

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_vanilla-object-store-persistence"}}}
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.argument.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Binary.
        {
            let path = path.join("binary");
            fs::create_dir_all(&path)?;
            for binary_tuple in self.binary.values() {
                let path = path.join(format!("{}.json", binary_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Binary, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != binary_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &binary_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &binary_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.binary.contains_key(&id) {
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.boolean_literal.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Boolean Operator.
        {
            let path = path.join("boolean_operator");
            fs::create_dir_all(&path)?;
            for boolean_operator_tuple in self.boolean_operator.values() {
                let path = path.join(format!("{}.json", boolean_operator_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (BooleanOperator, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != boolean_operator_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &boolean_operator_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &boolean_operator_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.boolean_operator.contains_key(&id) {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.call.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Comparison.
        {
            let path = path.join("comparison");
            fs::create_dir_all(&path)?;
            for comparison_tuple in self.comparison.values() {
                let path = path.join(format!("{}.json", comparison_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Comparison, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != comparison_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &comparison_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &comparison_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.comparison.contains_key(&id) {
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
                let path = path.join(format!("{}.json", dwarf_source_file_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (DwarfSourceFile, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != dwarf_source_file_tuple.0 {
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.field_access.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Field Access Target.
        {
            let path = path.join("field_access_target");
            fs::create_dir_all(&path)?;
            for field_access_target_tuple in self.field_access_target.values() {
                let path = path.join(format!("{}.json", field_access_target_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (FieldAccessTarget, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != field_access_target_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &field_access_target_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &field_access_target_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.field_access_target.contains_key(&id) {
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
                let id = file_name.split('.').next().unwrap();
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
                let path = path.join(format!("{}.json", float_literal_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (FloatLiteral, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != float_literal_tuple.0 {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.float_literal.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist For Loop.
        {
            let path = path.join("for_loop");
            fs::create_dir_all(&path)?;
            for for_loop_tuple in self.for_loop.values() {
                let path = path.join(format!("{}.json", for_loop_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (ForLoop, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != for_loop_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &for_loop_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &for_loop_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.for_loop.contains_key(&id) {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.function.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Grouped.
        {
            let path = path.join("grouped");
            fs::create_dir_all(&path)?;
            for grouped_tuple in self.grouped.values() {
                let path = path.join(format!("{}.json", grouped_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Grouped, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != grouped_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &grouped_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &grouped_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.grouped.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist If.
        {
            let path = path.join("x_if");
            fs::create_dir_all(&path)?;
            for x_if_tuple in self.x_if.values() {
                let path = path.join(format!("{}.json", x_if_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (XIf, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != x_if_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &x_if_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &x_if_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.x_if.contains_key(&id) {
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.import.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Index.
        {
            let path = path.join("index");
            fs::create_dir_all(&path)?;
            for index_tuple in self.index.values() {
                let path = path.join(format!("{}.json", index_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Index, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != index_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &index_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &index_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.index.contains_key(&id) {
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
                let id = file_name.split('.').next().unwrap();
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
                let path = path.join(format!("{}.json", item_tuple.0.id));
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.list.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist List Element.
        {
            let path = path.join("list_element");
            fs::create_dir_all(&path)?;
            for list_element_tuple in self.list_element.values() {
                let path = path.join(format!("{}.json", list_element_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (ListElement, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != list_element_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &list_element_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &list_element_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.list_element.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist List Expression.
        {
            let path = path.join("list_expression");
            fs::create_dir_all(&path)?;
            for list_expression_tuple in self.list_expression.values() {
                let path = path.join(format!("{}.json", list_expression_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (ListExpression, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != list_expression_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &list_expression_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &list_expression_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.list_expression.contains_key(&id) {
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.local_variable.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Macro.
        {
            let path = path.join("x_macro");
            fs::create_dir_all(&path)?;
            for x_macro_tuple in self.x_macro.values() {
                let path = path.join(format!("{}.json", x_macro_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (XMacro, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != x_macro_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &x_macro_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &x_macro_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.x_macro.contains_key(&id) {
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.z_object_store.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Operator.
        {
            let path = path.join("operator");
            fs::create_dir_all(&path)?;
            for operator_tuple in self.operator.values() {
                let path = path.join(format!("{}.json", operator_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Operator, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != operator_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &operator_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &operator_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.operator.contains_key(&id) {
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.print.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Range Expression.
        {
            let path = path.join("range_expression");
            fs::create_dir_all(&path)?;
            for range_expression_tuple in self.range_expression.values() {
                let path = path.join(format!("{}.json", range_expression_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (RangeExpression, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != range_expression_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &range_expression_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &range_expression_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.range_expression.contains_key(&id) {
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
                let id = file_name.split('.').next().unwrap();
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
                let path = path.join(format!("{}.json", result_statement_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (ResultStatement, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != result_statement_tuple.0 {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.result_statement.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Return.
        {
            let path = path.join("x_return");
            fs::create_dir_all(&path)?;
            for x_return_tuple in self.x_return.values() {
                let path = path.join(format!("{}.json", x_return_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (XReturn, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != x_return_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &x_return_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &x_return_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.x_return.contains_key(&id) {
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
                let path = path.join(format!("{}.json", z_some_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (ZSome, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != z_some_tuple.0 {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.z_some.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Span.
        {
            let path = path.join("span");
            fs::create_dir_all(&path)?;
            for span_tuple in self.span.values() {
                let path = path.join(format!("{}.json", span_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Span, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != span_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &span_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &span_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.span.contains_key(&id) {
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.struct_expression.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Type Cast.
        {
            let path = path.join("type_cast");
            fs::create_dir_all(&path)?;
            for type_cast_tuple in self.type_cast.values() {
                let path = path.join(format!("{}.json", type_cast_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (TypeCast, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != type_cast_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &type_cast_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &type_cast_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.type_cast.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Unary.
        {
            let path = path.join("unary");
            fs::create_dir_all(&path)?;
            for unary_tuple in self.unary.values() {
                let path = path.join(format!("{}.json", unary_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Unary, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != unary_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &unary_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &unary_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.unary.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Value.
        {
            let path = path.join("x_value");
            fs::create_dir_all(&path)?;
            for x_value_tuple in self.x_value.values() {
                let path = path.join(format!("{}.json", x_value_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (XValue, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != x_value_tuple.0 {
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
                    if !self.x_value.contains_key(&id) {
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
        let path = path.join("lu_dog.json");

        let mut store = Self::new();

        // Load Argument.
        {
            let path = path.join("argument");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let argument: (Argument, SystemTime) = serde_json::from_reader(reader)?;
                store.argument.insert(argument.0.id, argument);
            }
        }

        // Load Binary.
        {
            let path = path.join("binary");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let binary: (Binary, SystemTime) = serde_json::from_reader(reader)?;
                store.binary.insert(binary.0.id(), binary);
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
                let block: (Block, SystemTime) = serde_json::from_reader(reader)?;
                store.block.insert(block.0.id, block);
            }
        }

        // Load Boolean Literal.
        {
            let path = path.join("boolean_literal");
            let entries = fs::read_dir(path)?;
            for entry in entries {
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

        // Load Boolean Operator.
        {
            let path = path.join("boolean_operator");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let boolean_operator: (BooleanOperator, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .boolean_operator
                    .insert(boolean_operator.0.id(), boolean_operator);
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
                let call: (Call, SystemTime) = serde_json::from_reader(reader)?;
                store.call.insert(call.0.id, call);
            }
        }

        // Load Comparison.
        {
            let path = path.join("comparison");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let comparison: (Comparison, SystemTime) = serde_json::from_reader(reader)?;
                store.comparison.insert(comparison.0.id(), comparison);
            }
        }

        // Load Dwarf Source File.
        {
            let path = path.join("dwarf_source_file");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let dwarf_source_file: (DwarfSourceFile, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .dwarf_source_file
                    .insert(dwarf_source_file.0.id, dwarf_source_file);
            }
        }

        // Load Error.
        {
            let path = path.join("error");
            let entries = fs::read_dir(path)?;
            for entry in entries {
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
            let entries = fs::read_dir(path)?;
            for entry in entries {
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
            let entries = fs::read_dir(path)?;
            for entry in entries {
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
            let entries = fs::read_dir(path)?;
            for entry in entries {
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
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let field: (Field, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .field_id_by_name
                    .insert(field.0.name.to_upper_camel_case(), (field.0.id, field.1));
                store.field.insert(field.0.id, field);
            }
        }

        // Load Field Access.
        {
            let path = path.join("field_access");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let field_access: (FieldAccess, SystemTime) = serde_json::from_reader(reader)?;
                store.field_access.insert(field_access.0.id, field_access);
            }
        }

        // Load Field Access Target.
        {
            let path = path.join("field_access_target");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let field_access_target: (FieldAccessTarget, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .field_access_target
                    .insert(field_access_target.0.id(), field_access_target);
            }
        }

        // Load Field Expression.
        {
            let path = path.join("field_expression");
            let entries = fs::read_dir(path)?;
            for entry in entries {
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

        // Load Float Literal.
        {
            let path = path.join("float_literal");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let float_literal: (FloatLiteral, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .float_literal
                    .insert(float_literal.0.id, float_literal);
            }
        }

        // Load For Loop.
        {
            let path = path.join("for_loop");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let for_loop: (ForLoop, SystemTime) = serde_json::from_reader(reader)?;
                store.for_loop.insert(for_loop.0.id, for_loop);
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
                let function: (Function, SystemTime) = serde_json::from_reader(reader)?;
                store.function_id_by_name.insert(
                    function.0.name.to_upper_camel_case(),
                    (function.0.id, function.1),
                );
                store.function.insert(function.0.id, function);
            }
        }

        // Load Grouped.
        {
            let path = path.join("grouped");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let grouped: (Grouped, SystemTime) = serde_json::from_reader(reader)?;
                store.grouped.insert(grouped.0.id, grouped);
            }
        }

        // Load If.
        {
            let path = path.join("x_if");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let x_if: (XIf, SystemTime) = serde_json::from_reader(reader)?;
                store.x_if.insert(x_if.0.id, x_if);
            }
        }

        // Load Implementation.
        {
            let path = path.join("implementation");
            let entries = fs::read_dir(path)?;
            for entry in entries {
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
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let import: (Import, SystemTime) = serde_json::from_reader(reader)?;
                store.import.insert(import.0.id, import);
            }
        }

        // Load Index.
        {
            let path = path.join("index");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let index: (Index, SystemTime) = serde_json::from_reader(reader)?;
                store.index.insert(index.0.id, index);
            }
        }

        // Load Integer Literal.
        {
            let path = path.join("integer_literal");
            let entries = fs::read_dir(path)?;
            for entry in entries {
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
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let item: (Item, SystemTime) = serde_json::from_reader(reader)?;
                store.item.insert(item.0.id, item);
            }
        }

        // Load Let Statement.
        {
            let path = path.join("let_statement");
            let entries = fs::read_dir(path)?;
            for entry in entries {
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
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let list: (List, SystemTime) = serde_json::from_reader(reader)?;
                store.list.insert(list.0.id, list);
            }
        }

        // Load List Element.
        {
            let path = path.join("list_element");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let list_element: (ListElement, SystemTime) = serde_json::from_reader(reader)?;
                store.list_element.insert(list_element.0.id, list_element);
            }
        }

        // Load List Expression.
        {
            let path = path.join("list_expression");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let list_expression: (ListExpression, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .list_expression
                    .insert(list_expression.0.id, list_expression);
            }
        }

        // Load Literal.
        {
            let path = path.join("literal");
            let entries = fs::read_dir(path)?;
            for entry in entries {
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
            let entries = fs::read_dir(path)?;
            for entry in entries {
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

        // Load Macro.
        {
            let path = path.join("x_macro");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let x_macro: (XMacro, SystemTime) = serde_json::from_reader(reader)?;
                store.x_macro.insert(x_macro.0.id, x_macro);
            }
        }

        // Load Method Call.
        {
            let path = path.join("method_call");
            let entries = fs::read_dir(path)?;
            for entry in entries {
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
            let entries = fs::read_dir(path)?;
            for entry in entries {
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

        // Load Operator.
        {
            let path = path.join("operator");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let operator: (Operator, SystemTime) = serde_json::from_reader(reader)?;
                store.operator.insert(operator.0.id, operator);
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
                let woog_option: (WoogOption, SystemTime) = serde_json::from_reader(reader)?;
                store.woog_option.insert(woog_option.0.id, woog_option);
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
                let parameter: (Parameter, SystemTime) = serde_json::from_reader(reader)?;
                store.parameter.insert(parameter.0.id, parameter);
            }
        }

        // Load Print.
        {
            let path = path.join("print");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let print: (Print, SystemTime) = serde_json::from_reader(reader)?;
                store.print.insert(print.0.id, print);
            }
        }

        // Load Range Expression.
        {
            let path = path.join("range_expression");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let range_expression: (RangeExpression, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .range_expression
                    .insert(range_expression.0.id, range_expression);
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
                let reference: (Reference, SystemTime) = serde_json::from_reader(reader)?;
                store.reference.insert(reference.0.id, reference);
            }
        }

        // Load Result Statement.
        {
            let path = path.join("result_statement");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let result_statement: (ResultStatement, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .result_statement
                    .insert(result_statement.0.id, result_statement);
            }
        }

        // Load Return.
        {
            let path = path.join("x_return");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let x_return: (XReturn, SystemTime) = serde_json::from_reader(reader)?;
                store.x_return.insert(x_return.0.id, x_return);
            }
        }

        // Load Some.
        {
            let path = path.join("z_some");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let z_some: (ZSome, SystemTime) = serde_json::from_reader(reader)?;
                store.z_some.insert(z_some.0.id, z_some);
            }
        }

        // Load Span.
        {
            let path = path.join("span");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let span: (Span, SystemTime) = serde_json::from_reader(reader)?;
                store.span.insert(span.0.id, span);
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
                let statement: (Statement, SystemTime) = serde_json::from_reader(reader)?;
                store.statement.insert(statement.0.id, statement);
            }
        }

        // Load Static Method Call.
        {
            let path = path.join("static_method_call");
            let entries = fs::read_dir(path)?;
            for entry in entries {
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
            let entries = fs::read_dir(path)?;
            for entry in entries {
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
            let entries = fs::read_dir(path)?;
            for entry in entries {
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
            let entries = fs::read_dir(path)?;
            for entry in entries {
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

        // Load Type Cast.
        {
            let path = path.join("type_cast");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let type_cast: (TypeCast, SystemTime) = serde_json::from_reader(reader)?;
                store.type_cast.insert(type_cast.0.id, type_cast);
            }
        }

        // Load Unary.
        {
            let path = path.join("unary");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let unary: (Unary, SystemTime) = serde_json::from_reader(reader)?;
                store.unary.insert(unary.0.id(), unary);
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
                let x_value: (XValue, SystemTime) = serde_json::from_reader(reader)?;
                store.x_value.insert(x_value.0.id, x_value);
            }
        }

        // Load Value Type.
        {
            let path = path.join("value_type");
            let entries = fs::read_dir(path)?;
            for entry in entries {
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
            let entries = fs::read_dir(path)?;
            for entry in entries {
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
            let entries = fs::read_dir(path)?;
            for entry in entries {
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
