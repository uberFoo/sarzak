//! v2::lu_dog_rwlock Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_rwlock-object-store-file"}}}
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_rwlock-object-store-definition"}}}
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

use crate::v2::lu_dog_rwlock::types::{
    Argument, Binary, Block, BooleanLiteral, BooleanOperator, Call, Comparison, DwarfSourceFile,
    Error, ErrorExpression, Expression, ExpressionStatement, Field, FieldAccess, FieldAccessTarget,
    FieldExpression, FloatLiteral, ForLoop, Function, Grouped, Implementation, Import, Index,
    IntegerLiteral, Item, LetStatement, List, ListElement, ListExpression, Literal, LocalVariable,
    MethodCall, Operator, Parameter, Print, RangeExpression, Reference, ResultStatement, Span,
    Statement, StaticMethodCall, StringLiteral, StructExpression, TypeCast, Unary, ValueType,
    Variable, VariableExpression, WoogOption, WoogStruct, XIf, XMacro, XReturn, XValue,
    ZObjectStore, ZSome, ADDITION, AND, ASSIGNMENT, DEBUGGER, DIVISION, EMPTY, EQUAL,
    FALSE_LITERAL, GREATER_THAN, GREATER_THAN_OR_EQUAL, LESS_THAN, LESS_THAN_OR_EQUAL,
    MULTIPLICATION, NEGATION, NOT, NOT_EQUAL, OR, RANGE, SUBTRACTION, TRUE_LITERAL, UNKNOWN,
    UNKNOWN_VARIABLE, Z_NONE,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    argument: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Argument>>, SystemTime)>>>,
    binary: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Binary>>, SystemTime)>>>,
    block: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Block>>, SystemTime)>>>,
    boolean_literal: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<BooleanLiteral>>, SystemTime)>>>,
    boolean_operator: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<BooleanOperator>>, SystemTime)>>>,
    call: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Call>>, SystemTime)>>>,
    comparison: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Comparison>>, SystemTime)>>>,
    dwarf_source_file: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<DwarfSourceFile>>, SystemTime)>>>,
    error: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Error>>, SystemTime)>>>,
    error_expression: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<ErrorExpression>>, SystemTime)>>>,
    expression: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Expression>>, SystemTime)>>>,
    expression_statement:
        Arc<RwLock<HashMap<Uuid, (Arc<RwLock<ExpressionStatement>>, SystemTime)>>>,
    field: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Field>>, SystemTime)>>>,
    field_id_by_name: Arc<RwLock<HashMap<String, (Uuid, SystemTime)>>>,
    field_access: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<FieldAccess>>, SystemTime)>>>,
    field_access_target: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<FieldAccessTarget>>, SystemTime)>>>,
    field_expression: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<FieldExpression>>, SystemTime)>>>,
    float_literal: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<FloatLiteral>>, SystemTime)>>>,
    for_loop: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<ForLoop>>, SystemTime)>>>,
    function: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Function>>, SystemTime)>>>,
    function_id_by_name: Arc<RwLock<HashMap<String, (Uuid, SystemTime)>>>,
    grouped: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Grouped>>, SystemTime)>>>,
    x_if: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<XIf>>, SystemTime)>>>,
    implementation: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Implementation>>, SystemTime)>>>,
    import: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Import>>, SystemTime)>>>,
    index: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Index>>, SystemTime)>>>,
    integer_literal: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<IntegerLiteral>>, SystemTime)>>>,
    item: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Item>>, SystemTime)>>>,
    let_statement: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<LetStatement>>, SystemTime)>>>,
    list: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<List>>, SystemTime)>>>,
    list_element: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<ListElement>>, SystemTime)>>>,
    list_expression: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<ListExpression>>, SystemTime)>>>,
    literal: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Literal>>, SystemTime)>>>,
    local_variable: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<LocalVariable>>, SystemTime)>>>,
    x_macro: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<XMacro>>, SystemTime)>>>,
    method_call: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<MethodCall>>, SystemTime)>>>,
    z_object_store: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<ZObjectStore>>, SystemTime)>>>,
    operator: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Operator>>, SystemTime)>>>,
    woog_option: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<WoogOption>>, SystemTime)>>>,
    parameter: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Parameter>>, SystemTime)>>>,
    print: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Print>>, SystemTime)>>>,
    range_expression: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<RangeExpression>>, SystemTime)>>>,
    reference: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Reference>>, SystemTime)>>>,
    result_statement: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<ResultStatement>>, SystemTime)>>>,
    x_return: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<XReturn>>, SystemTime)>>>,
    z_some: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<ZSome>>, SystemTime)>>>,
    span: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Span>>, SystemTime)>>>,
    statement: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Statement>>, SystemTime)>>>,
    static_method_call: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<StaticMethodCall>>, SystemTime)>>>,
    string_literal: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<StringLiteral>>, SystemTime)>>>,
    woog_struct: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<WoogStruct>>, SystemTime)>>>,
    woog_struct_id_by_name: Arc<RwLock<HashMap<String, (Uuid, SystemTime)>>>,
    struct_expression: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<StructExpression>>, SystemTime)>>>,
    type_cast: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<TypeCast>>, SystemTime)>>>,
    unary: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Unary>>, SystemTime)>>>,
    x_value: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<XValue>>, SystemTime)>>>,
    value_type: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<ValueType>>, SystemTime)>>>,
    variable: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Variable>>, SystemTime)>>>,
    variable_expression: Arc<RwLock<HashMap<Uuid, (Arc<RwLock<VariableExpression>>, SystemTime)>>>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            argument: Arc::new(RwLock::new(HashMap::default())),
            binary: Arc::new(RwLock::new(HashMap::default())),
            block: Arc::new(RwLock::new(HashMap::default())),
            boolean_literal: Arc::new(RwLock::new(HashMap::default())),
            boolean_operator: Arc::new(RwLock::new(HashMap::default())),
            call: Arc::new(RwLock::new(HashMap::default())),
            comparison: Arc::new(RwLock::new(HashMap::default())),
            dwarf_source_file: Arc::new(RwLock::new(HashMap::default())),
            error: Arc::new(RwLock::new(HashMap::default())),
            error_expression: Arc::new(RwLock::new(HashMap::default())),
            expression: Arc::new(RwLock::new(HashMap::default())),
            expression_statement: Arc::new(RwLock::new(HashMap::default())),
            field: Arc::new(RwLock::new(HashMap::default())),
            field_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            field_access: Arc::new(RwLock::new(HashMap::default())),
            field_access_target: Arc::new(RwLock::new(HashMap::default())),
            field_expression: Arc::new(RwLock::new(HashMap::default())),
            float_literal: Arc::new(RwLock::new(HashMap::default())),
            for_loop: Arc::new(RwLock::new(HashMap::default())),
            function: Arc::new(RwLock::new(HashMap::default())),
            function_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            grouped: Arc::new(RwLock::new(HashMap::default())),
            x_if: Arc::new(RwLock::new(HashMap::default())),
            implementation: Arc::new(RwLock::new(HashMap::default())),
            import: Arc::new(RwLock::new(HashMap::default())),
            index: Arc::new(RwLock::new(HashMap::default())),
            integer_literal: Arc::new(RwLock::new(HashMap::default())),
            item: Arc::new(RwLock::new(HashMap::default())),
            let_statement: Arc::new(RwLock::new(HashMap::default())),
            list: Arc::new(RwLock::new(HashMap::default())),
            list_element: Arc::new(RwLock::new(HashMap::default())),
            list_expression: Arc::new(RwLock::new(HashMap::default())),
            literal: Arc::new(RwLock::new(HashMap::default())),
            local_variable: Arc::new(RwLock::new(HashMap::default())),
            x_macro: Arc::new(RwLock::new(HashMap::default())),
            method_call: Arc::new(RwLock::new(HashMap::default())),
            z_object_store: Arc::new(RwLock::new(HashMap::default())),
            operator: Arc::new(RwLock::new(HashMap::default())),
            woog_option: Arc::new(RwLock::new(HashMap::default())),
            parameter: Arc::new(RwLock::new(HashMap::default())),
            print: Arc::new(RwLock::new(HashMap::default())),
            range_expression: Arc::new(RwLock::new(HashMap::default())),
            reference: Arc::new(RwLock::new(HashMap::default())),
            result_statement: Arc::new(RwLock::new(HashMap::default())),
            x_return: Arc::new(RwLock::new(HashMap::default())),
            z_some: Arc::new(RwLock::new(HashMap::default())),
            span: Arc::new(RwLock::new(HashMap::default())),
            statement: Arc::new(RwLock::new(HashMap::default())),
            static_method_call: Arc::new(RwLock::new(HashMap::default())),
            string_literal: Arc::new(RwLock::new(HashMap::default())),
            woog_struct: Arc::new(RwLock::new(HashMap::default())),
            woog_struct_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            struct_expression: Arc::new(RwLock::new(HashMap::default())),
            type_cast: Arc::new(RwLock::new(HashMap::default())),
            unary: Arc::new(RwLock::new(HashMap::default())),
            x_value: Arc::new(RwLock::new(HashMap::default())),
            value_type: Arc::new(RwLock::new(HashMap::default())),
            variable: Arc::new(RwLock::new(HashMap::default())),
            variable_expression: Arc::new(RwLock::new(HashMap::default())),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥
        store.inter_binary(Arc::new(RwLock::new(Binary::Addition(ADDITION))));
        store.inter_binary(Arc::new(RwLock::new(Binary::Assignment(ASSIGNMENT))));
        store.inter_binary(Arc::new(RwLock::new(Binary::BooleanOperator(
            BooleanOperator::And(AND).id(),
        ))));
        store.inter_binary(Arc::new(RwLock::new(Binary::BooleanOperator(
            BooleanOperator::Or(OR).id(),
        ))));
        store.inter_binary(Arc::new(RwLock::new(Binary::Division(DIVISION))));
        store.inter_binary(Arc::new(RwLock::new(Binary::Multiplication(
            MULTIPLICATION,
        ))));
        store.inter_binary(Arc::new(RwLock::new(Binary::Subtraction(SUBTRACTION))));
        store.inter_boolean_literal(Arc::new(RwLock::new(BooleanLiteral::FalseLiteral(
            FALSE_LITERAL,
        ))));
        store.inter_boolean_literal(Arc::new(RwLock::new(BooleanLiteral::TrueLiteral(
            TRUE_LITERAL,
        ))));
        store.inter_boolean_operator(Arc::new(RwLock::new(BooleanOperator::And(AND))));
        store.inter_boolean_operator(Arc::new(RwLock::new(BooleanOperator::Or(OR))));
        store.inter_comparison(Arc::new(RwLock::new(Comparison::Equal(EQUAL))));
        store.inter_comparison(Arc::new(RwLock::new(Comparison::GreaterThan(GREATER_THAN))));
        store.inter_comparison(Arc::new(RwLock::new(Comparison::GreaterThanOrEqual(
            GREATER_THAN_OR_EQUAL,
        ))));
        store.inter_comparison(Arc::new(RwLock::new(Comparison::LessThan(LESS_THAN))));
        store.inter_comparison(Arc::new(RwLock::new(Comparison::LessThanOrEqual(
            LESS_THAN_OR_EQUAL,
        ))));
        store.inter_comparison(Arc::new(RwLock::new(Comparison::NotEqual(NOT_EQUAL))));
        store.inter_error(Arc::new(RwLock::new(Error::UnknownVariable(
            UNKNOWN_VARIABLE,
        ))));
        store.inter_expression(Arc::new(RwLock::new(Expression::Debugger(DEBUGGER))));
        store.inter_expression(Arc::new(RwLock::new(Expression::Literal(
            Literal::BooleanLiteral(BooleanLiteral::FalseLiteral(FALSE_LITERAL).id()).id(),
        ))));
        store.inter_expression(Arc::new(RwLock::new(Expression::Literal(
            Literal::BooleanLiteral(BooleanLiteral::TrueLiteral(TRUE_LITERAL).id()).id(),
        ))));
        store.inter_expression(Arc::new(RwLock::new(Expression::ZNone(Z_NONE))));
        store.inter_literal(Arc::new(RwLock::new(Literal::BooleanLiteral(
            BooleanLiteral::FalseLiteral(FALSE_LITERAL).id(),
        ))));
        store.inter_literal(Arc::new(RwLock::new(Literal::BooleanLiteral(
            BooleanLiteral::TrueLiteral(TRUE_LITERAL).id(),
        ))));
        store.inter_unary(Arc::new(RwLock::new(Unary::Negation(NEGATION))));
        store.inter_unary(Arc::new(RwLock::new(Unary::Not(NOT))));
        store.inter_value_type(Arc::new(RwLock::new(ValueType::Empty(EMPTY))));
        store.inter_value_type(Arc::new(RwLock::new(ValueType::Error(
            Error::UnknownVariable(UNKNOWN_VARIABLE).id(),
        ))));
        store.inter_value_type(Arc::new(RwLock::new(ValueType::Range(RANGE))));
        store.inter_value_type(Arc::new(RwLock::new(ValueType::Unknown(UNKNOWN))));

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_rwlock-object-store-methods"}}}
    /// Inter (insert) [`Argument`] into the store.
    ///
    pub fn inter_argument(&mut self, argument: Arc<RwLock<Argument>>) {
        let read = argument.read().unwrap();
        self.argument
            .write()
            .unwrap()
            .insert(read.id, (argument.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Argument`] from the store.
    ///
    pub fn exhume_argument(&self, id: &Uuid) -> Option<Arc<RwLock<Argument>>> {
        self.argument
            .read()
            .unwrap()
            .get(id)
            .map(|argument| argument.0.clone())
    }

    /// Exorcise (remove) [`Argument`] from the store.
    ///
    pub fn exorcise_argument(&mut self, id: &Uuid) -> Option<Arc<RwLock<Argument>>> {
        self.argument
            .write()
            .unwrap()
            .remove(id)
            .map(|argument| argument.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Argument>`.
    ///
    pub fn iter_argument(&self) -> impl Iterator<Item = Arc<RwLock<Argument>>> + '_ {
        let values: Vec<Arc<RwLock<Argument>>> = self
            .argument
            .read()
            .unwrap()
            .values()
            .map(|argument| argument.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Argument.
    ///
    pub fn argument_timestamp(&self, argument: &Argument) -> SystemTime {
        self.argument
            .read()
            .unwrap()
            .get(&argument.id)
            .map(|argument| argument.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Binary`] into the store.
    ///
    pub fn inter_binary(&mut self, binary: Arc<RwLock<Binary>>) {
        let read = binary.read().unwrap();
        self.binary
            .write()
            .unwrap()
            .insert(read.id(), (binary.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Binary`] from the store.
    ///
    pub fn exhume_binary(&self, id: &Uuid) -> Option<Arc<RwLock<Binary>>> {
        self.binary
            .read()
            .unwrap()
            .get(id)
            .map(|binary| binary.0.clone())
    }

    /// Exorcise (remove) [`Binary`] from the store.
    ///
    pub fn exorcise_binary(&mut self, id: &Uuid) -> Option<Arc<RwLock<Binary>>> {
        self.binary
            .write()
            .unwrap()
            .remove(id)
            .map(|binary| binary.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Binary>`.
    ///
    pub fn iter_binary(&self) -> impl Iterator<Item = Arc<RwLock<Binary>>> + '_ {
        let values: Vec<Arc<RwLock<Binary>>> = self
            .binary
            .read()
            .unwrap()
            .values()
            .map(|binary| binary.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Binary.
    ///
    pub fn binary_timestamp(&self, binary: &Binary) -> SystemTime {
        self.binary
            .read()
            .unwrap()
            .get(&binary.id())
            .map(|binary| binary.1)
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

    /// Inter (insert) [`BooleanLiteral`] into the store.
    ///
    pub fn inter_boolean_literal(&mut self, boolean_literal: Arc<RwLock<BooleanLiteral>>) {
        let read = boolean_literal.read().unwrap();
        self.boolean_literal
            .write()
            .unwrap()
            .insert(read.id(), (boolean_literal.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`BooleanLiteral`] from the store.
    ///
    pub fn exhume_boolean_literal(&self, id: &Uuid) -> Option<Arc<RwLock<BooleanLiteral>>> {
        self.boolean_literal
            .read()
            .unwrap()
            .get(id)
            .map(|boolean_literal| boolean_literal.0.clone())
    }

    /// Exorcise (remove) [`BooleanLiteral`] from the store.
    ///
    pub fn exorcise_boolean_literal(&mut self, id: &Uuid) -> Option<Arc<RwLock<BooleanLiteral>>> {
        self.boolean_literal
            .write()
            .unwrap()
            .remove(id)
            .map(|boolean_literal| boolean_literal.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanLiteral>`.
    ///
    pub fn iter_boolean_literal(&self) -> impl Iterator<Item = Arc<RwLock<BooleanLiteral>>> + '_ {
        let values: Vec<Arc<RwLock<BooleanLiteral>>> = self
            .boolean_literal
            .read()
            .unwrap()
            .values()
            .map(|boolean_literal| boolean_literal.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for BooleanLiteral.
    ///
    pub fn boolean_literal_timestamp(&self, boolean_literal: &BooleanLiteral) -> SystemTime {
        self.boolean_literal
            .read()
            .unwrap()
            .get(&boolean_literal.id())
            .map(|boolean_literal| boolean_literal.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`BooleanOperator`] into the store.
    ///
    pub fn inter_boolean_operator(&mut self, boolean_operator: Arc<RwLock<BooleanOperator>>) {
        let read = boolean_operator.read().unwrap();
        self.boolean_operator
            .write()
            .unwrap()
            .insert(read.id(), (boolean_operator.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`BooleanOperator`] from the store.
    ///
    pub fn exhume_boolean_operator(&self, id: &Uuid) -> Option<Arc<RwLock<BooleanOperator>>> {
        self.boolean_operator
            .read()
            .unwrap()
            .get(id)
            .map(|boolean_operator| boolean_operator.0.clone())
    }

    /// Exorcise (remove) [`BooleanOperator`] from the store.
    ///
    pub fn exorcise_boolean_operator(&mut self, id: &Uuid) -> Option<Arc<RwLock<BooleanOperator>>> {
        self.boolean_operator
            .write()
            .unwrap()
            .remove(id)
            .map(|boolean_operator| boolean_operator.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanOperator>`.
    ///
    pub fn iter_boolean_operator(&self) -> impl Iterator<Item = Arc<RwLock<BooleanOperator>>> + '_ {
        let values: Vec<Arc<RwLock<BooleanOperator>>> = self
            .boolean_operator
            .read()
            .unwrap()
            .values()
            .map(|boolean_operator| boolean_operator.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for BooleanOperator.
    ///
    pub fn boolean_operator_timestamp(&self, boolean_operator: &BooleanOperator) -> SystemTime {
        self.boolean_operator
            .read()
            .unwrap()
            .get(&boolean_operator.id())
            .map(|boolean_operator| boolean_operator.1)
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

    /// Inter (insert) [`Comparison`] into the store.
    ///
    pub fn inter_comparison(&mut self, comparison: Arc<RwLock<Comparison>>) {
        let read = comparison.read().unwrap();
        self.comparison
            .write()
            .unwrap()
            .insert(read.id(), (comparison.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Comparison`] from the store.
    ///
    pub fn exhume_comparison(&self, id: &Uuid) -> Option<Arc<RwLock<Comparison>>> {
        self.comparison
            .read()
            .unwrap()
            .get(id)
            .map(|comparison| comparison.0.clone())
    }

    /// Exorcise (remove) [`Comparison`] from the store.
    ///
    pub fn exorcise_comparison(&mut self, id: &Uuid) -> Option<Arc<RwLock<Comparison>>> {
        self.comparison
            .write()
            .unwrap()
            .remove(id)
            .map(|comparison| comparison.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Comparison>`.
    ///
    pub fn iter_comparison(&self) -> impl Iterator<Item = Arc<RwLock<Comparison>>> + '_ {
        let values: Vec<Arc<RwLock<Comparison>>> = self
            .comparison
            .read()
            .unwrap()
            .values()
            .map(|comparison| comparison.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Comparison.
    ///
    pub fn comparison_timestamp(&self, comparison: &Comparison) -> SystemTime {
        self.comparison
            .read()
            .unwrap()
            .get(&comparison.id())
            .map(|comparison| comparison.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`DwarfSourceFile`] into the store.
    ///
    pub fn inter_dwarf_source_file(&mut self, dwarf_source_file: Arc<RwLock<DwarfSourceFile>>) {
        let read = dwarf_source_file.read().unwrap();
        self.dwarf_source_file
            .write()
            .unwrap()
            .insert(read.id, (dwarf_source_file.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`DwarfSourceFile`] from the store.
    ///
    pub fn exhume_dwarf_source_file(&self, id: &Uuid) -> Option<Arc<RwLock<DwarfSourceFile>>> {
        self.dwarf_source_file
            .read()
            .unwrap()
            .get(id)
            .map(|dwarf_source_file| dwarf_source_file.0.clone())
    }

    /// Exorcise (remove) [`DwarfSourceFile`] from the store.
    ///
    pub fn exorcise_dwarf_source_file(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<DwarfSourceFile>>> {
        self.dwarf_source_file
            .write()
            .unwrap()
            .remove(id)
            .map(|dwarf_source_file| dwarf_source_file.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, DwarfSourceFile>`.
    ///
    pub fn iter_dwarf_source_file(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<DwarfSourceFile>>> + '_ {
        let values: Vec<Arc<RwLock<DwarfSourceFile>>> = self
            .dwarf_source_file
            .read()
            .unwrap()
            .values()
            .map(|dwarf_source_file| dwarf_source_file.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for DwarfSourceFile.
    ///
    pub fn dwarf_source_file_timestamp(&self, dwarf_source_file: &DwarfSourceFile) -> SystemTime {
        self.dwarf_source_file
            .read()
            .unwrap()
            .get(&dwarf_source_file.id)
            .map(|dwarf_source_file| dwarf_source_file.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Error`] into the store.
    ///
    pub fn inter_error(&mut self, error: Arc<RwLock<Error>>) {
        let read = error.read().unwrap();
        self.error
            .write()
            .unwrap()
            .insert(read.id(), (error.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Error`] from the store.
    ///
    pub fn exhume_error(&self, id: &Uuid) -> Option<Arc<RwLock<Error>>> {
        self.error
            .read()
            .unwrap()
            .get(id)
            .map(|error| error.0.clone())
    }

    /// Exorcise (remove) [`Error`] from the store.
    ///
    pub fn exorcise_error(&mut self, id: &Uuid) -> Option<Arc<RwLock<Error>>> {
        self.error
            .write()
            .unwrap()
            .remove(id)
            .map(|error| error.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Error>`.
    ///
    pub fn iter_error(&self) -> impl Iterator<Item = Arc<RwLock<Error>>> + '_ {
        let values: Vec<Arc<RwLock<Error>>> = self
            .error
            .read()
            .unwrap()
            .values()
            .map(|error| error.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Error.
    ///
    pub fn error_timestamp(&self, error: &Error) -> SystemTime {
        self.error
            .read()
            .unwrap()
            .get(&error.id())
            .map(|error| error.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ErrorExpression`] into the store.
    ///
    pub fn inter_error_expression(&mut self, error_expression: Arc<RwLock<ErrorExpression>>) {
        let read = error_expression.read().unwrap();
        self.error_expression
            .write()
            .unwrap()
            .insert(read.id, (error_expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ErrorExpression`] from the store.
    ///
    pub fn exhume_error_expression(&self, id: &Uuid) -> Option<Arc<RwLock<ErrorExpression>>> {
        self.error_expression
            .read()
            .unwrap()
            .get(id)
            .map(|error_expression| error_expression.0.clone())
    }

    /// Exorcise (remove) [`ErrorExpression`] from the store.
    ///
    pub fn exorcise_error_expression(&mut self, id: &Uuid) -> Option<Arc<RwLock<ErrorExpression>>> {
        self.error_expression
            .write()
            .unwrap()
            .remove(id)
            .map(|error_expression| error_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ErrorExpression>`.
    ///
    pub fn iter_error_expression(&self) -> impl Iterator<Item = Arc<RwLock<ErrorExpression>>> + '_ {
        let values: Vec<Arc<RwLock<ErrorExpression>>> = self
            .error_expression
            .read()
            .unwrap()
            .values()
            .map(|error_expression| error_expression.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for ErrorExpression.
    ///
    pub fn error_expression_timestamp(&self, error_expression: &ErrorExpression) -> SystemTime {
        self.error_expression
            .read()
            .unwrap()
            .get(&error_expression.id)
            .map(|error_expression| error_expression.1)
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

    /// Inter (insert) [`ExpressionStatement`] into the store.
    ///
    pub fn inter_expression_statement(
        &mut self,
        expression_statement: Arc<RwLock<ExpressionStatement>>,
    ) {
        let read = expression_statement.read().unwrap();
        self.expression_statement
            .write()
            .unwrap()
            .insert(read.id, (expression_statement.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ExpressionStatement`] from the store.
    ///
    pub fn exhume_expression_statement(
        &self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<ExpressionStatement>>> {
        self.expression_statement
            .read()
            .unwrap()
            .get(id)
            .map(|expression_statement| expression_statement.0.clone())
    }

    /// Exorcise (remove) [`ExpressionStatement`] from the store.
    ///
    pub fn exorcise_expression_statement(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<ExpressionStatement>>> {
        self.expression_statement
            .write()
            .unwrap()
            .remove(id)
            .map(|expression_statement| expression_statement.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ExpressionStatement>`.
    ///
    pub fn iter_expression_statement(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<ExpressionStatement>>> + '_ {
        let values: Vec<Arc<RwLock<ExpressionStatement>>> = self
            .expression_statement
            .read()
            .unwrap()
            .values()
            .map(|expression_statement| expression_statement.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for ExpressionStatement.
    ///
    pub fn expression_statement_timestamp(
        &self,
        expression_statement: &ExpressionStatement,
    ) -> SystemTime {
        self.expression_statement
            .read()
            .unwrap()
            .get(&expression_statement.id)
            .map(|expression_statement| expression_statement.1)
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

    /// Inter (insert) [`FieldAccess`] into the store.
    ///
    pub fn inter_field_access(&mut self, field_access: Arc<RwLock<FieldAccess>>) {
        let read = field_access.read().unwrap();
        self.field_access
            .write()
            .unwrap()
            .insert(read.id, (field_access.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`FieldAccess`] from the store.
    ///
    pub fn exhume_field_access(&self, id: &Uuid) -> Option<Arc<RwLock<FieldAccess>>> {
        self.field_access
            .read()
            .unwrap()
            .get(id)
            .map(|field_access| field_access.0.clone())
    }

    /// Exorcise (remove) [`FieldAccess`] from the store.
    ///
    pub fn exorcise_field_access(&mut self, id: &Uuid) -> Option<Arc<RwLock<FieldAccess>>> {
        self.field_access
            .write()
            .unwrap()
            .remove(id)
            .map(|field_access| field_access.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccess>`.
    ///
    pub fn iter_field_access(&self) -> impl Iterator<Item = Arc<RwLock<FieldAccess>>> + '_ {
        let values: Vec<Arc<RwLock<FieldAccess>>> = self
            .field_access
            .read()
            .unwrap()
            .values()
            .map(|field_access| field_access.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for FieldAccess.
    ///
    pub fn field_access_timestamp(&self, field_access: &FieldAccess) -> SystemTime {
        self.field_access
            .read()
            .unwrap()
            .get(&field_access.id)
            .map(|field_access| field_access.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`FieldAccessTarget`] into the store.
    ///
    pub fn inter_field_access_target(
        &mut self,
        field_access_target: Arc<RwLock<FieldAccessTarget>>,
    ) {
        let read = field_access_target.read().unwrap();
        self.field_access_target
            .write()
            .unwrap()
            .insert(read.id(), (field_access_target.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`FieldAccessTarget`] from the store.
    ///
    pub fn exhume_field_access_target(&self, id: &Uuid) -> Option<Arc<RwLock<FieldAccessTarget>>> {
        self.field_access_target
            .read()
            .unwrap()
            .get(id)
            .map(|field_access_target| field_access_target.0.clone())
    }

    /// Exorcise (remove) [`FieldAccessTarget`] from the store.
    ///
    pub fn exorcise_field_access_target(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<FieldAccessTarget>>> {
        self.field_access_target
            .write()
            .unwrap()
            .remove(id)
            .map(|field_access_target| field_access_target.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccessTarget>`.
    ///
    pub fn iter_field_access_target(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<FieldAccessTarget>>> + '_ {
        let values: Vec<Arc<RwLock<FieldAccessTarget>>> = self
            .field_access_target
            .read()
            .unwrap()
            .values()
            .map(|field_access_target| field_access_target.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for FieldAccessTarget.
    ///
    pub fn field_access_target_timestamp(
        &self,
        field_access_target: &FieldAccessTarget,
    ) -> SystemTime {
        self.field_access_target
            .read()
            .unwrap()
            .get(&field_access_target.id())
            .map(|field_access_target| field_access_target.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`FieldExpression`] into the store.
    ///
    pub fn inter_field_expression(&mut self, field_expression: Arc<RwLock<FieldExpression>>) {
        let read = field_expression.read().unwrap();
        self.field_expression
            .write()
            .unwrap()
            .insert(read.id, (field_expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`FieldExpression`] from the store.
    ///
    pub fn exhume_field_expression(&self, id: &Uuid) -> Option<Arc<RwLock<FieldExpression>>> {
        self.field_expression
            .read()
            .unwrap()
            .get(id)
            .map(|field_expression| field_expression.0.clone())
    }

    /// Exorcise (remove) [`FieldExpression`] from the store.
    ///
    pub fn exorcise_field_expression(&mut self, id: &Uuid) -> Option<Arc<RwLock<FieldExpression>>> {
        self.field_expression
            .write()
            .unwrap()
            .remove(id)
            .map(|field_expression| field_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldExpression>`.
    ///
    pub fn iter_field_expression(&self) -> impl Iterator<Item = Arc<RwLock<FieldExpression>>> + '_ {
        let values: Vec<Arc<RwLock<FieldExpression>>> = self
            .field_expression
            .read()
            .unwrap()
            .values()
            .map(|field_expression| field_expression.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for FieldExpression.
    ///
    pub fn field_expression_timestamp(&self, field_expression: &FieldExpression) -> SystemTime {
        self.field_expression
            .read()
            .unwrap()
            .get(&field_expression.id)
            .map(|field_expression| field_expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`FloatLiteral`] into the store.
    ///
    pub fn inter_float_literal(&mut self, float_literal: Arc<RwLock<FloatLiteral>>) {
        let read = float_literal.read().unwrap();
        self.float_literal
            .write()
            .unwrap()
            .insert(read.id, (float_literal.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`FloatLiteral`] from the store.
    ///
    pub fn exhume_float_literal(&self, id: &Uuid) -> Option<Arc<RwLock<FloatLiteral>>> {
        self.float_literal
            .read()
            .unwrap()
            .get(id)
            .map(|float_literal| float_literal.0.clone())
    }

    /// Exorcise (remove) [`FloatLiteral`] from the store.
    ///
    pub fn exorcise_float_literal(&mut self, id: &Uuid) -> Option<Arc<RwLock<FloatLiteral>>> {
        self.float_literal
            .write()
            .unwrap()
            .remove(id)
            .map(|float_literal| float_literal.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FloatLiteral>`.
    ///
    pub fn iter_float_literal(&self) -> impl Iterator<Item = Arc<RwLock<FloatLiteral>>> + '_ {
        let values: Vec<Arc<RwLock<FloatLiteral>>> = self
            .float_literal
            .read()
            .unwrap()
            .values()
            .map(|float_literal| float_literal.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for FloatLiteral.
    ///
    pub fn float_literal_timestamp(&self, float_literal: &FloatLiteral) -> SystemTime {
        self.float_literal
            .read()
            .unwrap()
            .get(&float_literal.id)
            .map(|float_literal| float_literal.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ForLoop`] into the store.
    ///
    pub fn inter_for_loop(&mut self, for_loop: Arc<RwLock<ForLoop>>) {
        let read = for_loop.read().unwrap();
        self.for_loop
            .write()
            .unwrap()
            .insert(read.id, (for_loop.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ForLoop`] from the store.
    ///
    pub fn exhume_for_loop(&self, id: &Uuid) -> Option<Arc<RwLock<ForLoop>>> {
        self.for_loop
            .read()
            .unwrap()
            .get(id)
            .map(|for_loop| for_loop.0.clone())
    }

    /// Exorcise (remove) [`ForLoop`] from the store.
    ///
    pub fn exorcise_for_loop(&mut self, id: &Uuid) -> Option<Arc<RwLock<ForLoop>>> {
        self.for_loop
            .write()
            .unwrap()
            .remove(id)
            .map(|for_loop| for_loop.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ForLoop>`.
    ///
    pub fn iter_for_loop(&self) -> impl Iterator<Item = Arc<RwLock<ForLoop>>> + '_ {
        let values: Vec<Arc<RwLock<ForLoop>>> = self
            .for_loop
            .read()
            .unwrap()
            .values()
            .map(|for_loop| for_loop.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for ForLoop.
    ///
    pub fn for_loop_timestamp(&self, for_loop: &ForLoop) -> SystemTime {
        self.for_loop
            .read()
            .unwrap()
            .get(&for_loop.id)
            .map(|for_loop| for_loop.1)
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

    /// Inter (insert) [`Grouped`] into the store.
    ///
    pub fn inter_grouped(&mut self, grouped: Arc<RwLock<Grouped>>) {
        let read = grouped.read().unwrap();
        self.grouped
            .write()
            .unwrap()
            .insert(read.id, (grouped.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Grouped`] from the store.
    ///
    pub fn exhume_grouped(&self, id: &Uuid) -> Option<Arc<RwLock<Grouped>>> {
        self.grouped
            .read()
            .unwrap()
            .get(id)
            .map(|grouped| grouped.0.clone())
    }

    /// Exorcise (remove) [`Grouped`] from the store.
    ///
    pub fn exorcise_grouped(&mut self, id: &Uuid) -> Option<Arc<RwLock<Grouped>>> {
        self.grouped
            .write()
            .unwrap()
            .remove(id)
            .map(|grouped| grouped.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Grouped>`.
    ///
    pub fn iter_grouped(&self) -> impl Iterator<Item = Arc<RwLock<Grouped>>> + '_ {
        let values: Vec<Arc<RwLock<Grouped>>> = self
            .grouped
            .read()
            .unwrap()
            .values()
            .map(|grouped| grouped.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Grouped.
    ///
    pub fn grouped_timestamp(&self, grouped: &Grouped) -> SystemTime {
        self.grouped
            .read()
            .unwrap()
            .get(&grouped.id)
            .map(|grouped| grouped.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`XIf`] into the store.
    ///
    pub fn inter_x_if(&mut self, x_if: Arc<RwLock<XIf>>) {
        let read = x_if.read().unwrap();
        self.x_if
            .write()
            .unwrap()
            .insert(read.id, (x_if.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`XIf`] from the store.
    ///
    pub fn exhume_x_if(&self, id: &Uuid) -> Option<Arc<RwLock<XIf>>> {
        self.x_if.read().unwrap().get(id).map(|x_if| x_if.0.clone())
    }

    /// Exorcise (remove) [`XIf`] from the store.
    ///
    pub fn exorcise_x_if(&mut self, id: &Uuid) -> Option<Arc<RwLock<XIf>>> {
        self.x_if
            .write()
            .unwrap()
            .remove(id)
            .map(|x_if| x_if.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XIf>`.
    ///
    pub fn iter_x_if(&self) -> impl Iterator<Item = Arc<RwLock<XIf>>> + '_ {
        let values: Vec<Arc<RwLock<XIf>>> = self
            .x_if
            .read()
            .unwrap()
            .values()
            .map(|x_if| x_if.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for XIf.
    ///
    pub fn x_if_timestamp(&self, x_if: &XIf) -> SystemTime {
        self.x_if
            .read()
            .unwrap()
            .get(&x_if.id)
            .map(|x_if| x_if.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Implementation`] into the store.
    ///
    pub fn inter_implementation(&mut self, implementation: Arc<RwLock<Implementation>>) {
        let read = implementation.read().unwrap();
        self.implementation
            .write()
            .unwrap()
            .insert(read.id, (implementation.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Implementation`] from the store.
    ///
    pub fn exhume_implementation(&self, id: &Uuid) -> Option<Arc<RwLock<Implementation>>> {
        self.implementation
            .read()
            .unwrap()
            .get(id)
            .map(|implementation| implementation.0.clone())
    }

    /// Exorcise (remove) [`Implementation`] from the store.
    ///
    pub fn exorcise_implementation(&mut self, id: &Uuid) -> Option<Arc<RwLock<Implementation>>> {
        self.implementation
            .write()
            .unwrap()
            .remove(id)
            .map(|implementation| implementation.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Implementation>`.
    ///
    pub fn iter_implementation(&self) -> impl Iterator<Item = Arc<RwLock<Implementation>>> + '_ {
        let values: Vec<Arc<RwLock<Implementation>>> = self
            .implementation
            .read()
            .unwrap()
            .values()
            .map(|implementation| implementation.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Implementation.
    ///
    pub fn implementation_timestamp(&self, implementation: &Implementation) -> SystemTime {
        self.implementation
            .read()
            .unwrap()
            .get(&implementation.id)
            .map(|implementation| implementation.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Import`] into the store.
    ///
    pub fn inter_import(&mut self, import: Arc<RwLock<Import>>) {
        let read = import.read().unwrap();
        self.import
            .write()
            .unwrap()
            .insert(read.id, (import.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Import`] from the store.
    ///
    pub fn exhume_import(&self, id: &Uuid) -> Option<Arc<RwLock<Import>>> {
        self.import
            .read()
            .unwrap()
            .get(id)
            .map(|import| import.0.clone())
    }

    /// Exorcise (remove) [`Import`] from the store.
    ///
    pub fn exorcise_import(&mut self, id: &Uuid) -> Option<Arc<RwLock<Import>>> {
        self.import
            .write()
            .unwrap()
            .remove(id)
            .map(|import| import.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Import>`.
    ///
    pub fn iter_import(&self) -> impl Iterator<Item = Arc<RwLock<Import>>> + '_ {
        let values: Vec<Arc<RwLock<Import>>> = self
            .import
            .read()
            .unwrap()
            .values()
            .map(|import| import.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Import.
    ///
    pub fn import_timestamp(&self, import: &Import) -> SystemTime {
        self.import
            .read()
            .unwrap()
            .get(&import.id)
            .map(|import| import.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Index`] into the store.
    ///
    pub fn inter_index(&mut self, index: Arc<RwLock<Index>>) {
        let read = index.read().unwrap();
        self.index
            .write()
            .unwrap()
            .insert(read.id, (index.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Index`] from the store.
    ///
    pub fn exhume_index(&self, id: &Uuid) -> Option<Arc<RwLock<Index>>> {
        self.index
            .read()
            .unwrap()
            .get(id)
            .map(|index| index.0.clone())
    }

    /// Exorcise (remove) [`Index`] from the store.
    ///
    pub fn exorcise_index(&mut self, id: &Uuid) -> Option<Arc<RwLock<Index>>> {
        self.index
            .write()
            .unwrap()
            .remove(id)
            .map(|index| index.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Index>`.
    ///
    pub fn iter_index(&self) -> impl Iterator<Item = Arc<RwLock<Index>>> + '_ {
        let values: Vec<Arc<RwLock<Index>>> = self
            .index
            .read()
            .unwrap()
            .values()
            .map(|index| index.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Index.
    ///
    pub fn index_timestamp(&self, index: &Index) -> SystemTime {
        self.index
            .read()
            .unwrap()
            .get(&index.id)
            .map(|index| index.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`IntegerLiteral`] into the store.
    ///
    pub fn inter_integer_literal(&mut self, integer_literal: Arc<RwLock<IntegerLiteral>>) {
        let read = integer_literal.read().unwrap();
        self.integer_literal
            .write()
            .unwrap()
            .insert(read.id, (integer_literal.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`IntegerLiteral`] from the store.
    ///
    pub fn exhume_integer_literal(&self, id: &Uuid) -> Option<Arc<RwLock<IntegerLiteral>>> {
        self.integer_literal
            .read()
            .unwrap()
            .get(id)
            .map(|integer_literal| integer_literal.0.clone())
    }

    /// Exorcise (remove) [`IntegerLiteral`] from the store.
    ///
    pub fn exorcise_integer_literal(&mut self, id: &Uuid) -> Option<Arc<RwLock<IntegerLiteral>>> {
        self.integer_literal
            .write()
            .unwrap()
            .remove(id)
            .map(|integer_literal| integer_literal.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, IntegerLiteral>`.
    ///
    pub fn iter_integer_literal(&self) -> impl Iterator<Item = Arc<RwLock<IntegerLiteral>>> + '_ {
        let values: Vec<Arc<RwLock<IntegerLiteral>>> = self
            .integer_literal
            .read()
            .unwrap()
            .values()
            .map(|integer_literal| integer_literal.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for IntegerLiteral.
    ///
    pub fn integer_literal_timestamp(&self, integer_literal: &IntegerLiteral) -> SystemTime {
        self.integer_literal
            .read()
            .unwrap()
            .get(&integer_literal.id)
            .map(|integer_literal| integer_literal.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Item`] into the store.
    ///
    pub fn inter_item(&mut self, item: Arc<RwLock<Item>>) {
        let read = item.read().unwrap();
        self.item
            .write()
            .unwrap()
            .insert(read.id, (item.clone(), SystemTime::now()));
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
            .get(&item.id)
            .map(|item| item.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`LetStatement`] into the store.
    ///
    pub fn inter_let_statement(&mut self, let_statement: Arc<RwLock<LetStatement>>) {
        let read = let_statement.read().unwrap();
        self.let_statement
            .write()
            .unwrap()
            .insert(read.id, (let_statement.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`LetStatement`] from the store.
    ///
    pub fn exhume_let_statement(&self, id: &Uuid) -> Option<Arc<RwLock<LetStatement>>> {
        self.let_statement
            .read()
            .unwrap()
            .get(id)
            .map(|let_statement| let_statement.0.clone())
    }

    /// Exorcise (remove) [`LetStatement`] from the store.
    ///
    pub fn exorcise_let_statement(&mut self, id: &Uuid) -> Option<Arc<RwLock<LetStatement>>> {
        self.let_statement
            .write()
            .unwrap()
            .remove(id)
            .map(|let_statement| let_statement.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LetStatement>`.
    ///
    pub fn iter_let_statement(&self) -> impl Iterator<Item = Arc<RwLock<LetStatement>>> + '_ {
        let values: Vec<Arc<RwLock<LetStatement>>> = self
            .let_statement
            .read()
            .unwrap()
            .values()
            .map(|let_statement| let_statement.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for LetStatement.
    ///
    pub fn let_statement_timestamp(&self, let_statement: &LetStatement) -> SystemTime {
        self.let_statement
            .read()
            .unwrap()
            .get(&let_statement.id)
            .map(|let_statement| let_statement.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`List`] into the store.
    ///
    pub fn inter_list(&mut self, list: Arc<RwLock<List>>) {
        let read = list.read().unwrap();
        self.list
            .write()
            .unwrap()
            .insert(read.id, (list.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`List`] from the store.
    ///
    pub fn exhume_list(&self, id: &Uuid) -> Option<Arc<RwLock<List>>> {
        self.list.read().unwrap().get(id).map(|list| list.0.clone())
    }

    /// Exorcise (remove) [`List`] from the store.
    ///
    pub fn exorcise_list(&mut self, id: &Uuid) -> Option<Arc<RwLock<List>>> {
        self.list
            .write()
            .unwrap()
            .remove(id)
            .map(|list| list.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, List>`.
    ///
    pub fn iter_list(&self) -> impl Iterator<Item = Arc<RwLock<List>>> + '_ {
        let values: Vec<Arc<RwLock<List>>> = self
            .list
            .read()
            .unwrap()
            .values()
            .map(|list| list.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for List.
    ///
    pub fn list_timestamp(&self, list: &List) -> SystemTime {
        self.list
            .read()
            .unwrap()
            .get(&list.id)
            .map(|list| list.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ListElement`] into the store.
    ///
    pub fn inter_list_element(&mut self, list_element: Arc<RwLock<ListElement>>) {
        let read = list_element.read().unwrap();
        self.list_element
            .write()
            .unwrap()
            .insert(read.id, (list_element.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ListElement`] from the store.
    ///
    pub fn exhume_list_element(&self, id: &Uuid) -> Option<Arc<RwLock<ListElement>>> {
        self.list_element
            .read()
            .unwrap()
            .get(id)
            .map(|list_element| list_element.0.clone())
    }

    /// Exorcise (remove) [`ListElement`] from the store.
    ///
    pub fn exorcise_list_element(&mut self, id: &Uuid) -> Option<Arc<RwLock<ListElement>>> {
        self.list_element
            .write()
            .unwrap()
            .remove(id)
            .map(|list_element| list_element.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListElement>`.
    ///
    pub fn iter_list_element(&self) -> impl Iterator<Item = Arc<RwLock<ListElement>>> + '_ {
        let values: Vec<Arc<RwLock<ListElement>>> = self
            .list_element
            .read()
            .unwrap()
            .values()
            .map(|list_element| list_element.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for ListElement.
    ///
    pub fn list_element_timestamp(&self, list_element: &ListElement) -> SystemTime {
        self.list_element
            .read()
            .unwrap()
            .get(&list_element.id)
            .map(|list_element| list_element.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ListExpression`] into the store.
    ///
    pub fn inter_list_expression(&mut self, list_expression: Arc<RwLock<ListExpression>>) {
        let read = list_expression.read().unwrap();
        self.list_expression
            .write()
            .unwrap()
            .insert(read.id, (list_expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ListExpression`] from the store.
    ///
    pub fn exhume_list_expression(&self, id: &Uuid) -> Option<Arc<RwLock<ListExpression>>> {
        self.list_expression
            .read()
            .unwrap()
            .get(id)
            .map(|list_expression| list_expression.0.clone())
    }

    /// Exorcise (remove) [`ListExpression`] from the store.
    ///
    pub fn exorcise_list_expression(&mut self, id: &Uuid) -> Option<Arc<RwLock<ListExpression>>> {
        self.list_expression
            .write()
            .unwrap()
            .remove(id)
            .map(|list_expression| list_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListExpression>`.
    ///
    pub fn iter_list_expression(&self) -> impl Iterator<Item = Arc<RwLock<ListExpression>>> + '_ {
        let values: Vec<Arc<RwLock<ListExpression>>> = self
            .list_expression
            .read()
            .unwrap()
            .values()
            .map(|list_expression| list_expression.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for ListExpression.
    ///
    pub fn list_expression_timestamp(&self, list_expression: &ListExpression) -> SystemTime {
        self.list_expression
            .read()
            .unwrap()
            .get(&list_expression.id)
            .map(|list_expression| list_expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Literal`] into the store.
    ///
    pub fn inter_literal(&mut self, literal: Arc<RwLock<Literal>>) {
        let read = literal.read().unwrap();
        self.literal
            .write()
            .unwrap()
            .insert(read.id(), (literal.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Literal`] from the store.
    ///
    pub fn exhume_literal(&self, id: &Uuid) -> Option<Arc<RwLock<Literal>>> {
        self.literal
            .read()
            .unwrap()
            .get(id)
            .map(|literal| literal.0.clone())
    }

    /// Exorcise (remove) [`Literal`] from the store.
    ///
    pub fn exorcise_literal(&mut self, id: &Uuid) -> Option<Arc<RwLock<Literal>>> {
        self.literal
            .write()
            .unwrap()
            .remove(id)
            .map(|literal| literal.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Literal>`.
    ///
    pub fn iter_literal(&self) -> impl Iterator<Item = Arc<RwLock<Literal>>> + '_ {
        let values: Vec<Arc<RwLock<Literal>>> = self
            .literal
            .read()
            .unwrap()
            .values()
            .map(|literal| literal.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Literal.
    ///
    pub fn literal_timestamp(&self, literal: &Literal) -> SystemTime {
        self.literal
            .read()
            .unwrap()
            .get(&literal.id())
            .map(|literal| literal.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`LocalVariable`] into the store.
    ///
    pub fn inter_local_variable(&mut self, local_variable: Arc<RwLock<LocalVariable>>) {
        let read = local_variable.read().unwrap();
        self.local_variable
            .write()
            .unwrap()
            .insert(read.id, (local_variable.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`LocalVariable`] from the store.
    ///
    pub fn exhume_local_variable(&self, id: &Uuid) -> Option<Arc<RwLock<LocalVariable>>> {
        self.local_variable
            .read()
            .unwrap()
            .get(id)
            .map(|local_variable| local_variable.0.clone())
    }

    /// Exorcise (remove) [`LocalVariable`] from the store.
    ///
    pub fn exorcise_local_variable(&mut self, id: &Uuid) -> Option<Arc<RwLock<LocalVariable>>> {
        self.local_variable
            .write()
            .unwrap()
            .remove(id)
            .map(|local_variable| local_variable.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LocalVariable>`.
    ///
    pub fn iter_local_variable(&self) -> impl Iterator<Item = Arc<RwLock<LocalVariable>>> + '_ {
        let values: Vec<Arc<RwLock<LocalVariable>>> = self
            .local_variable
            .read()
            .unwrap()
            .values()
            .map(|local_variable| local_variable.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for LocalVariable.
    ///
    pub fn local_variable_timestamp(&self, local_variable: &LocalVariable) -> SystemTime {
        self.local_variable
            .read()
            .unwrap()
            .get(&local_variable.id)
            .map(|local_variable| local_variable.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`XMacro`] into the store.
    ///
    pub fn inter_x_macro(&mut self, x_macro: Arc<RwLock<XMacro>>) {
        let read = x_macro.read().unwrap();
        self.x_macro
            .write()
            .unwrap()
            .insert(read.id, (x_macro.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`XMacro`] from the store.
    ///
    pub fn exhume_x_macro(&self, id: &Uuid) -> Option<Arc<RwLock<XMacro>>> {
        self.x_macro
            .read()
            .unwrap()
            .get(id)
            .map(|x_macro| x_macro.0.clone())
    }

    /// Exorcise (remove) [`XMacro`] from the store.
    ///
    pub fn exorcise_x_macro(&mut self, id: &Uuid) -> Option<Arc<RwLock<XMacro>>> {
        self.x_macro
            .write()
            .unwrap()
            .remove(id)
            .map(|x_macro| x_macro.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XMacro>`.
    ///
    pub fn iter_x_macro(&self) -> impl Iterator<Item = Arc<RwLock<XMacro>>> + '_ {
        let values: Vec<Arc<RwLock<XMacro>>> = self
            .x_macro
            .read()
            .unwrap()
            .values()
            .map(|x_macro| x_macro.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for XMacro.
    ///
    pub fn x_macro_timestamp(&self, x_macro: &XMacro) -> SystemTime {
        self.x_macro
            .read()
            .unwrap()
            .get(&x_macro.id)
            .map(|x_macro| x_macro.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`MethodCall`] into the store.
    ///
    pub fn inter_method_call(&mut self, method_call: Arc<RwLock<MethodCall>>) {
        let read = method_call.read().unwrap();
        self.method_call
            .write()
            .unwrap()
            .insert(read.id, (method_call.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`MethodCall`] from the store.
    ///
    pub fn exhume_method_call(&self, id: &Uuid) -> Option<Arc<RwLock<MethodCall>>> {
        self.method_call
            .read()
            .unwrap()
            .get(id)
            .map(|method_call| method_call.0.clone())
    }

    /// Exorcise (remove) [`MethodCall`] from the store.
    ///
    pub fn exorcise_method_call(&mut self, id: &Uuid) -> Option<Arc<RwLock<MethodCall>>> {
        self.method_call
            .write()
            .unwrap()
            .remove(id)
            .map(|method_call| method_call.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, MethodCall>`.
    ///
    pub fn iter_method_call(&self) -> impl Iterator<Item = Arc<RwLock<MethodCall>>> + '_ {
        let values: Vec<Arc<RwLock<MethodCall>>> = self
            .method_call
            .read()
            .unwrap()
            .values()
            .map(|method_call| method_call.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for MethodCall.
    ///
    pub fn method_call_timestamp(&self, method_call: &MethodCall) -> SystemTime {
        self.method_call
            .read()
            .unwrap()
            .get(&method_call.id)
            .map(|method_call| method_call.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ZObjectStore`] into the store.
    ///
    pub fn inter_z_object_store(&mut self, z_object_store: Arc<RwLock<ZObjectStore>>) {
        let read = z_object_store.read().unwrap();
        self.z_object_store
            .write()
            .unwrap()
            .insert(read.id, (z_object_store.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ZObjectStore`] from the store.
    ///
    pub fn exhume_z_object_store(&self, id: &Uuid) -> Option<Arc<RwLock<ZObjectStore>>> {
        self.z_object_store
            .read()
            .unwrap()
            .get(id)
            .map(|z_object_store| z_object_store.0.clone())
    }

    /// Exorcise (remove) [`ZObjectStore`] from the store.
    ///
    pub fn exorcise_z_object_store(&mut self, id: &Uuid) -> Option<Arc<RwLock<ZObjectStore>>> {
        self.z_object_store
            .write()
            .unwrap()
            .remove(id)
            .map(|z_object_store| z_object_store.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ZObjectStore>`.
    ///
    pub fn iter_z_object_store(&self) -> impl Iterator<Item = Arc<RwLock<ZObjectStore>>> + '_ {
        let values: Vec<Arc<RwLock<ZObjectStore>>> = self
            .z_object_store
            .read()
            .unwrap()
            .values()
            .map(|z_object_store| z_object_store.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for ZObjectStore.
    ///
    pub fn z_object_store_timestamp(&self, z_object_store: &ZObjectStore) -> SystemTime {
        self.z_object_store
            .read()
            .unwrap()
            .get(&z_object_store.id)
            .map(|z_object_store| z_object_store.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Operator`] into the store.
    ///
    pub fn inter_operator(&mut self, operator: Arc<RwLock<Operator>>) {
        let read = operator.read().unwrap();
        self.operator
            .write()
            .unwrap()
            .insert(read.id, (operator.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Operator`] from the store.
    ///
    pub fn exhume_operator(&self, id: &Uuid) -> Option<Arc<RwLock<Operator>>> {
        self.operator
            .read()
            .unwrap()
            .get(id)
            .map(|operator| operator.0.clone())
    }

    /// Exorcise (remove) [`Operator`] from the store.
    ///
    pub fn exorcise_operator(&mut self, id: &Uuid) -> Option<Arc<RwLock<Operator>>> {
        self.operator
            .write()
            .unwrap()
            .remove(id)
            .map(|operator| operator.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Operator>`.
    ///
    pub fn iter_operator(&self) -> impl Iterator<Item = Arc<RwLock<Operator>>> + '_ {
        let values: Vec<Arc<RwLock<Operator>>> = self
            .operator
            .read()
            .unwrap()
            .values()
            .map(|operator| operator.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Operator.
    ///
    pub fn operator_timestamp(&self, operator: &Operator) -> SystemTime {
        self.operator
            .read()
            .unwrap()
            .get(&operator.id)
            .map(|operator| operator.1)
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

    /// Inter (insert) [`Print`] into the store.
    ///
    pub fn inter_print(&mut self, print: Arc<RwLock<Print>>) {
        let read = print.read().unwrap();
        self.print
            .write()
            .unwrap()
            .insert(read.id, (print.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Print`] from the store.
    ///
    pub fn exhume_print(&self, id: &Uuid) -> Option<Arc<RwLock<Print>>> {
        self.print
            .read()
            .unwrap()
            .get(id)
            .map(|print| print.0.clone())
    }

    /// Exorcise (remove) [`Print`] from the store.
    ///
    pub fn exorcise_print(&mut self, id: &Uuid) -> Option<Arc<RwLock<Print>>> {
        self.print
            .write()
            .unwrap()
            .remove(id)
            .map(|print| print.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Print>`.
    ///
    pub fn iter_print(&self) -> impl Iterator<Item = Arc<RwLock<Print>>> + '_ {
        let values: Vec<Arc<RwLock<Print>>> = self
            .print
            .read()
            .unwrap()
            .values()
            .map(|print| print.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Print.
    ///
    pub fn print_timestamp(&self, print: &Print) -> SystemTime {
        self.print
            .read()
            .unwrap()
            .get(&print.id)
            .map(|print| print.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`RangeExpression`] into the store.
    ///
    pub fn inter_range_expression(&mut self, range_expression: Arc<RwLock<RangeExpression>>) {
        let read = range_expression.read().unwrap();
        self.range_expression
            .write()
            .unwrap()
            .insert(read.id, (range_expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`RangeExpression`] from the store.
    ///
    pub fn exhume_range_expression(&self, id: &Uuid) -> Option<Arc<RwLock<RangeExpression>>> {
        self.range_expression
            .read()
            .unwrap()
            .get(id)
            .map(|range_expression| range_expression.0.clone())
    }

    /// Exorcise (remove) [`RangeExpression`] from the store.
    ///
    pub fn exorcise_range_expression(&mut self, id: &Uuid) -> Option<Arc<RwLock<RangeExpression>>> {
        self.range_expression
            .write()
            .unwrap()
            .remove(id)
            .map(|range_expression| range_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RangeExpression>`.
    ///
    pub fn iter_range_expression(&self) -> impl Iterator<Item = Arc<RwLock<RangeExpression>>> + '_ {
        let values: Vec<Arc<RwLock<RangeExpression>>> = self
            .range_expression
            .read()
            .unwrap()
            .values()
            .map(|range_expression| range_expression.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for RangeExpression.
    ///
    pub fn range_expression_timestamp(&self, range_expression: &RangeExpression) -> SystemTime {
        self.range_expression
            .read()
            .unwrap()
            .get(&range_expression.id)
            .map(|range_expression| range_expression.1)
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

    /// Inter (insert) [`ResultStatement`] into the store.
    ///
    pub fn inter_result_statement(&mut self, result_statement: Arc<RwLock<ResultStatement>>) {
        let read = result_statement.read().unwrap();
        self.result_statement
            .write()
            .unwrap()
            .insert(read.id, (result_statement.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ResultStatement`] from the store.
    ///
    pub fn exhume_result_statement(&self, id: &Uuid) -> Option<Arc<RwLock<ResultStatement>>> {
        self.result_statement
            .read()
            .unwrap()
            .get(id)
            .map(|result_statement| result_statement.0.clone())
    }

    /// Exorcise (remove) [`ResultStatement`] from the store.
    ///
    pub fn exorcise_result_statement(&mut self, id: &Uuid) -> Option<Arc<RwLock<ResultStatement>>> {
        self.result_statement
            .write()
            .unwrap()
            .remove(id)
            .map(|result_statement| result_statement.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ResultStatement>`.
    ///
    pub fn iter_result_statement(&self) -> impl Iterator<Item = Arc<RwLock<ResultStatement>>> + '_ {
        let values: Vec<Arc<RwLock<ResultStatement>>> = self
            .result_statement
            .read()
            .unwrap()
            .values()
            .map(|result_statement| result_statement.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for ResultStatement.
    ///
    pub fn result_statement_timestamp(&self, result_statement: &ResultStatement) -> SystemTime {
        self.result_statement
            .read()
            .unwrap()
            .get(&result_statement.id)
            .map(|result_statement| result_statement.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`XReturn`] into the store.
    ///
    pub fn inter_x_return(&mut self, x_return: Arc<RwLock<XReturn>>) {
        let read = x_return.read().unwrap();
        self.x_return
            .write()
            .unwrap()
            .insert(read.id, (x_return.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`XReturn`] from the store.
    ///
    pub fn exhume_x_return(&self, id: &Uuid) -> Option<Arc<RwLock<XReturn>>> {
        self.x_return
            .read()
            .unwrap()
            .get(id)
            .map(|x_return| x_return.0.clone())
    }

    /// Exorcise (remove) [`XReturn`] from the store.
    ///
    pub fn exorcise_x_return(&mut self, id: &Uuid) -> Option<Arc<RwLock<XReturn>>> {
        self.x_return
            .write()
            .unwrap()
            .remove(id)
            .map(|x_return| x_return.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XReturn>`.
    ///
    pub fn iter_x_return(&self) -> impl Iterator<Item = Arc<RwLock<XReturn>>> + '_ {
        let values: Vec<Arc<RwLock<XReturn>>> = self
            .x_return
            .read()
            .unwrap()
            .values()
            .map(|x_return| x_return.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for XReturn.
    ///
    pub fn x_return_timestamp(&self, x_return: &XReturn) -> SystemTime {
        self.x_return
            .read()
            .unwrap()
            .get(&x_return.id)
            .map(|x_return| x_return.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ZSome`] into the store.
    ///
    pub fn inter_z_some(&mut self, z_some: Arc<RwLock<ZSome>>) {
        let read = z_some.read().unwrap();
        self.z_some
            .write()
            .unwrap()
            .insert(read.id, (z_some.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ZSome`] from the store.
    ///
    pub fn exhume_z_some(&self, id: &Uuid) -> Option<Arc<RwLock<ZSome>>> {
        self.z_some
            .read()
            .unwrap()
            .get(id)
            .map(|z_some| z_some.0.clone())
    }

    /// Exorcise (remove) [`ZSome`] from the store.
    ///
    pub fn exorcise_z_some(&mut self, id: &Uuid) -> Option<Arc<RwLock<ZSome>>> {
        self.z_some
            .write()
            .unwrap()
            .remove(id)
            .map(|z_some| z_some.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ZSome>`.
    ///
    pub fn iter_z_some(&self) -> impl Iterator<Item = Arc<RwLock<ZSome>>> + '_ {
        let values: Vec<Arc<RwLock<ZSome>>> = self
            .z_some
            .read()
            .unwrap()
            .values()
            .map(|z_some| z_some.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for ZSome.
    ///
    pub fn z_some_timestamp(&self, z_some: &ZSome) -> SystemTime {
        self.z_some
            .read()
            .unwrap()
            .get(&z_some.id)
            .map(|z_some| z_some.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Span`] into the store.
    ///
    pub fn inter_span(&mut self, span: Arc<RwLock<Span>>) {
        let read = span.read().unwrap();
        self.span
            .write()
            .unwrap()
            .insert(read.id, (span.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Span`] from the store.
    ///
    pub fn exhume_span(&self, id: &Uuid) -> Option<Arc<RwLock<Span>>> {
        self.span.read().unwrap().get(id).map(|span| span.0.clone())
    }

    /// Exorcise (remove) [`Span`] from the store.
    ///
    pub fn exorcise_span(&mut self, id: &Uuid) -> Option<Arc<RwLock<Span>>> {
        self.span
            .write()
            .unwrap()
            .remove(id)
            .map(|span| span.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Span>`.
    ///
    pub fn iter_span(&self) -> impl Iterator<Item = Arc<RwLock<Span>>> + '_ {
        let values: Vec<Arc<RwLock<Span>>> = self
            .span
            .read()
            .unwrap()
            .values()
            .map(|span| span.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Span.
    ///
    pub fn span_timestamp(&self, span: &Span) -> SystemTime {
        self.span
            .read()
            .unwrap()
            .get(&span.id)
            .map(|span| span.1)
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

    /// Inter (insert) [`StaticMethodCall`] into the store.
    ///
    pub fn inter_static_method_call(&mut self, static_method_call: Arc<RwLock<StaticMethodCall>>) {
        let read = static_method_call.read().unwrap();
        self.static_method_call
            .write()
            .unwrap()
            .insert(read.id, (static_method_call.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`StaticMethodCall`] from the store.
    ///
    pub fn exhume_static_method_call(&self, id: &Uuid) -> Option<Arc<RwLock<StaticMethodCall>>> {
        self.static_method_call
            .read()
            .unwrap()
            .get(id)
            .map(|static_method_call| static_method_call.0.clone())
    }

    /// Exorcise (remove) [`StaticMethodCall`] from the store.
    ///
    pub fn exorcise_static_method_call(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<StaticMethodCall>>> {
        self.static_method_call
            .write()
            .unwrap()
            .remove(id)
            .map(|static_method_call| static_method_call.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StaticMethodCall>`.
    ///
    pub fn iter_static_method_call(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<StaticMethodCall>>> + '_ {
        let values: Vec<Arc<RwLock<StaticMethodCall>>> = self
            .static_method_call
            .read()
            .unwrap()
            .values()
            .map(|static_method_call| static_method_call.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for StaticMethodCall.
    ///
    pub fn static_method_call_timestamp(
        &self,
        static_method_call: &StaticMethodCall,
    ) -> SystemTime {
        self.static_method_call
            .read()
            .unwrap()
            .get(&static_method_call.id)
            .map(|static_method_call| static_method_call.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`StringLiteral`] into the store.
    ///
    pub fn inter_string_literal(&mut self, string_literal: Arc<RwLock<StringLiteral>>) {
        let read = string_literal.read().unwrap();
        self.string_literal
            .write()
            .unwrap()
            .insert(read.id, (string_literal.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`StringLiteral`] from the store.
    ///
    pub fn exhume_string_literal(&self, id: &Uuid) -> Option<Arc<RwLock<StringLiteral>>> {
        self.string_literal
            .read()
            .unwrap()
            .get(id)
            .map(|string_literal| string_literal.0.clone())
    }

    /// Exorcise (remove) [`StringLiteral`] from the store.
    ///
    pub fn exorcise_string_literal(&mut self, id: &Uuid) -> Option<Arc<RwLock<StringLiteral>>> {
        self.string_literal
            .write()
            .unwrap()
            .remove(id)
            .map(|string_literal| string_literal.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StringLiteral>`.
    ///
    pub fn iter_string_literal(&self) -> impl Iterator<Item = Arc<RwLock<StringLiteral>>> + '_ {
        let values: Vec<Arc<RwLock<StringLiteral>>> = self
            .string_literal
            .read()
            .unwrap()
            .values()
            .map(|string_literal| string_literal.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for StringLiteral.
    ///
    pub fn string_literal_timestamp(&self, string_literal: &StringLiteral) -> SystemTime {
        self.string_literal
            .read()
            .unwrap()
            .get(&string_literal.id)
            .map(|string_literal| string_literal.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`WoogStruct`] into the store.
    ///
    pub fn inter_woog_struct(&mut self, woog_struct: Arc<RwLock<WoogStruct>>) {
        let read = woog_struct.read().unwrap();
        let value = (woog_struct.clone(), SystemTime::now());
        self.woog_struct_id_by_name
            .write()
            .unwrap()
            .insert(read.name.to_upper_camel_case(), (read.id, value.1));
        self.woog_struct.write().unwrap().insert(read.id, value);
    }

    /// Exhume (get) [`WoogStruct`] from the store.
    ///
    pub fn exhume_woog_struct(&self, id: &Uuid) -> Option<Arc<RwLock<WoogStruct>>> {
        self.woog_struct
            .read()
            .unwrap()
            .get(id)
            .map(|woog_struct| woog_struct.0.clone())
    }

    /// Exorcise (remove) [`WoogStruct`] from the store.
    ///
    pub fn exorcise_woog_struct(&mut self, id: &Uuid) -> Option<Arc<RwLock<WoogStruct>>> {
        self.woog_struct
            .write()
            .unwrap()
            .remove(id)
            .map(|woog_struct| woog_struct.0.clone())
    }

    /// Exhume [`WoogStruct`] id from the store by name.
    ///
    pub fn exhume_woog_struct_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.woog_struct_id_by_name
            .read()
            .unwrap()
            .get(name)
            .map(|woog_struct| woog_struct.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, WoogStruct>`.
    ///
    pub fn iter_woog_struct(&self) -> impl Iterator<Item = Arc<RwLock<WoogStruct>>> + '_ {
        let values: Vec<Arc<RwLock<WoogStruct>>> = self
            .woog_struct
            .read()
            .unwrap()
            .values()
            .map(|woog_struct| woog_struct.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for WoogStruct.
    ///
    pub fn woog_struct_timestamp(&self, woog_struct: &WoogStruct) -> SystemTime {
        self.woog_struct
            .read()
            .unwrap()
            .get(&woog_struct.id)
            .map(|woog_struct| woog_struct.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`StructExpression`] into the store.
    ///
    pub fn inter_struct_expression(&mut self, struct_expression: Arc<RwLock<StructExpression>>) {
        let read = struct_expression.read().unwrap();
        self.struct_expression
            .write()
            .unwrap()
            .insert(read.id, (struct_expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`StructExpression`] from the store.
    ///
    pub fn exhume_struct_expression(&self, id: &Uuid) -> Option<Arc<RwLock<StructExpression>>> {
        self.struct_expression
            .read()
            .unwrap()
            .get(id)
            .map(|struct_expression| struct_expression.0.clone())
    }

    /// Exorcise (remove) [`StructExpression`] from the store.
    ///
    pub fn exorcise_struct_expression(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<StructExpression>>> {
        self.struct_expression
            .write()
            .unwrap()
            .remove(id)
            .map(|struct_expression| struct_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructExpression>`.
    ///
    pub fn iter_struct_expression(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<StructExpression>>> + '_ {
        let values: Vec<Arc<RwLock<StructExpression>>> = self
            .struct_expression
            .read()
            .unwrap()
            .values()
            .map(|struct_expression| struct_expression.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for StructExpression.
    ///
    pub fn struct_expression_timestamp(&self, struct_expression: &StructExpression) -> SystemTime {
        self.struct_expression
            .read()
            .unwrap()
            .get(&struct_expression.id)
            .map(|struct_expression| struct_expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`TypeCast`] into the store.
    ///
    pub fn inter_type_cast(&mut self, type_cast: Arc<RwLock<TypeCast>>) {
        let read = type_cast.read().unwrap();
        self.type_cast
            .write()
            .unwrap()
            .insert(read.id, (type_cast.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`TypeCast`] from the store.
    ///
    pub fn exhume_type_cast(&self, id: &Uuid) -> Option<Arc<RwLock<TypeCast>>> {
        self.type_cast
            .read()
            .unwrap()
            .get(id)
            .map(|type_cast| type_cast.0.clone())
    }

    /// Exorcise (remove) [`TypeCast`] from the store.
    ///
    pub fn exorcise_type_cast(&mut self, id: &Uuid) -> Option<Arc<RwLock<TypeCast>>> {
        self.type_cast
            .write()
            .unwrap()
            .remove(id)
            .map(|type_cast| type_cast.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, TypeCast>`.
    ///
    pub fn iter_type_cast(&self) -> impl Iterator<Item = Arc<RwLock<TypeCast>>> + '_ {
        let values: Vec<Arc<RwLock<TypeCast>>> = self
            .type_cast
            .read()
            .unwrap()
            .values()
            .map(|type_cast| type_cast.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for TypeCast.
    ///
    pub fn type_cast_timestamp(&self, type_cast: &TypeCast) -> SystemTime {
        self.type_cast
            .read()
            .unwrap()
            .get(&type_cast.id)
            .map(|type_cast| type_cast.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Unary`] into the store.
    ///
    pub fn inter_unary(&mut self, unary: Arc<RwLock<Unary>>) {
        let read = unary.read().unwrap();
        self.unary
            .write()
            .unwrap()
            .insert(read.id(), (unary.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Unary`] from the store.
    ///
    pub fn exhume_unary(&self, id: &Uuid) -> Option<Arc<RwLock<Unary>>> {
        self.unary
            .read()
            .unwrap()
            .get(id)
            .map(|unary| unary.0.clone())
    }

    /// Exorcise (remove) [`Unary`] from the store.
    ///
    pub fn exorcise_unary(&mut self, id: &Uuid) -> Option<Arc<RwLock<Unary>>> {
        self.unary
            .write()
            .unwrap()
            .remove(id)
            .map(|unary| unary.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Unary>`.
    ///
    pub fn iter_unary(&self) -> impl Iterator<Item = Arc<RwLock<Unary>>> + '_ {
        let values: Vec<Arc<RwLock<Unary>>> = self
            .unary
            .read()
            .unwrap()
            .values()
            .map(|unary| unary.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Unary.
    ///
    pub fn unary_timestamp(&self, unary: &Unary) -> SystemTime {
        self.unary
            .read()
            .unwrap()
            .get(&unary.id())
            .map(|unary| unary.1)
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

    /// Inter (insert) [`ValueType`] into the store.
    ///
    pub fn inter_value_type(&mut self, value_type: Arc<RwLock<ValueType>>) {
        let read = value_type.read().unwrap();
        self.value_type
            .write()
            .unwrap()
            .insert(read.id(), (value_type.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ValueType`] from the store.
    ///
    pub fn exhume_value_type(&self, id: &Uuid) -> Option<Arc<RwLock<ValueType>>> {
        self.value_type
            .read()
            .unwrap()
            .get(id)
            .map(|value_type| value_type.0.clone())
    }

    /// Exorcise (remove) [`ValueType`] from the store.
    ///
    pub fn exorcise_value_type(&mut self, id: &Uuid) -> Option<Arc<RwLock<ValueType>>> {
        self.value_type
            .write()
            .unwrap()
            .remove(id)
            .map(|value_type| value_type.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ValueType>`.
    ///
    pub fn iter_value_type(&self) -> impl Iterator<Item = Arc<RwLock<ValueType>>> + '_ {
        let values: Vec<Arc<RwLock<ValueType>>> = self
            .value_type
            .read()
            .unwrap()
            .values()
            .map(|value_type| value_type.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for ValueType.
    ///
    pub fn value_type_timestamp(&self, value_type: &ValueType) -> SystemTime {
        self.value_type
            .read()
            .unwrap()
            .get(&value_type.id())
            .map(|value_type| value_type.1)
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

    /// Inter (insert) [`VariableExpression`] into the store.
    ///
    pub fn inter_variable_expression(
        &mut self,
        variable_expression: Arc<RwLock<VariableExpression>>,
    ) {
        let read = variable_expression.read().unwrap();
        self.variable_expression
            .write()
            .unwrap()
            .insert(read.id, (variable_expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`VariableExpression`] from the store.
    ///
    pub fn exhume_variable_expression(&self, id: &Uuid) -> Option<Arc<RwLock<VariableExpression>>> {
        self.variable_expression
            .read()
            .unwrap()
            .get(id)
            .map(|variable_expression| variable_expression.0.clone())
    }

    /// Exorcise (remove) [`VariableExpression`] from the store.
    ///
    pub fn exorcise_variable_expression(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<VariableExpression>>> {
        self.variable_expression
            .write()
            .unwrap()
            .remove(id)
            .map(|variable_expression| variable_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, VariableExpression>`.
    ///
    pub fn iter_variable_expression(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<VariableExpression>>> + '_ {
        let values: Vec<Arc<RwLock<VariableExpression>>> = self
            .variable_expression
            .read()
            .unwrap()
            .values()
            .map(|variable_expression| variable_expression.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for VariableExpression.
    ///
    pub fn variable_expression_timestamp(
        &self,
        variable_expression: &VariableExpression,
    ) -> SystemTime {
        self.variable_expression
            .read()
            .unwrap()
            .get(&variable_expression.id)
            .map(|variable_expression| variable_expression.1)
            .unwrap_or(SystemTime::now())
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_rwlock-object-store-persistence"}}}
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
            for argument_tuple in self.argument.read().unwrap().values() {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.argument.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Binary.
        {
            let path = path.join("binary");
            fs::create_dir_all(&path)?;
            for binary_tuple in self.binary.read().unwrap().values() {
                let path = path.join(format!("{}.json", binary_tuple.0.read().unwrap().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Binary>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != binary_tuple.0.read().unwrap().to_owned()
                    {
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
                    if !self.binary.read().unwrap().contains_key(&id) {
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

        // Persist Boolean Literal.
        {
            let path = path.join("boolean_literal");
            fs::create_dir_all(&path)?;
            for boolean_literal_tuple in self.boolean_literal.read().unwrap().values() {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.boolean_literal.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Boolean Operator.
        {
            let path = path.join("boolean_operator");
            fs::create_dir_all(&path)?;
            for boolean_operator_tuple in self.boolean_operator.read().unwrap().values() {
                let path = path.join(format!(
                    "{}.json",
                    boolean_operator_tuple.0.read().unwrap().id()
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<BooleanOperator>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != boolean_operator_tuple.0.read().unwrap().to_owned()
                    {
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
                    if !self.boolean_operator.read().unwrap().contains_key(&id) {
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

        // Persist Comparison.
        {
            let path = path.join("comparison");
            fs::create_dir_all(&path)?;
            for comparison_tuple in self.comparison.read().unwrap().values() {
                let path = path.join(format!("{}.json", comparison_tuple.0.read().unwrap().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Comparison>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != comparison_tuple.0.read().unwrap().to_owned()
                    {
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
                    if !self.comparison.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Dwarf Source File.
        {
            let path = path.join("dwarf_source_file");
            fs::create_dir_all(&path)?;
            for dwarf_source_file_tuple in self.dwarf_source_file.read().unwrap().values() {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.dwarf_source_file.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Error.
        {
            let path = path.join("error");
            fs::create_dir_all(&path)?;
            for error_tuple in self.error.read().unwrap().values() {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.error.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Error Expression.
        {
            let path = path.join("error_expression");
            fs::create_dir_all(&path)?;
            for error_expression_tuple in self.error_expression.read().unwrap().values() {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.error_expression.read().unwrap().contains_key(&id) {
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

        // Persist Expression Statement.
        {
            let path = path.join("expression_statement");
            fs::create_dir_all(&path)?;
            for expression_statement_tuple in self.expression_statement.read().unwrap().values() {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.expression_statement.read().unwrap().contains_key(&id) {
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

        // Persist Field Access.
        {
            let path = path.join("field_access");
            fs::create_dir_all(&path)?;
            for field_access_tuple in self.field_access.read().unwrap().values() {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.field_access.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Field Access Target.
        {
            let path = path.join("field_access_target");
            fs::create_dir_all(&path)?;
            for field_access_target_tuple in self.field_access_target.read().unwrap().values() {
                let path = path.join(format!(
                    "{}.json",
                    field_access_target_tuple.0.read().unwrap().id()
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<FieldAccessTarget>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != field_access_target_tuple.0.read().unwrap().to_owned()
                    {
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
                    if !self.field_access_target.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Field Expression.
        {
            let path = path.join("field_expression");
            fs::create_dir_all(&path)?;
            for field_expression_tuple in self.field_expression.read().unwrap().values() {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.field_expression.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Float Literal.
        {
            let path = path.join("float_literal");
            fs::create_dir_all(&path)?;
            for float_literal_tuple in self.float_literal.read().unwrap().values() {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.float_literal.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist For Loop.
        {
            let path = path.join("for_loop");
            fs::create_dir_all(&path)?;
            for for_loop_tuple in self.for_loop.read().unwrap().values() {
                let path = path.join(format!("{}.json", for_loop_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<ForLoop>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != for_loop_tuple.0.read().unwrap().to_owned()
                    {
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
                    if !self.for_loop.read().unwrap().contains_key(&id) {
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

        // Persist Grouped.
        {
            let path = path.join("grouped");
            fs::create_dir_all(&path)?;
            for grouped_tuple in self.grouped.read().unwrap().values() {
                let path = path.join(format!("{}.json", grouped_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Grouped>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != grouped_tuple.0.read().unwrap().to_owned()
                    {
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
                    if !self.grouped.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist If.
        {
            let path = path.join("x_if");
            fs::create_dir_all(&path)?;
            for x_if_tuple in self.x_if.read().unwrap().values() {
                let path = path.join(format!("{}.json", x_if_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<XIf>>, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != x_if_tuple.0.read().unwrap().to_owned()
                    {
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
                    if !self.x_if.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Implementation.
        {
            let path = path.join("implementation");
            fs::create_dir_all(&path)?;
            for implementation_tuple in self.implementation.read().unwrap().values() {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.implementation.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Import.
        {
            let path = path.join("import");
            fs::create_dir_all(&path)?;
            for import_tuple in self.import.read().unwrap().values() {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.import.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Index.
        {
            let path = path.join("index");
            fs::create_dir_all(&path)?;
            for index_tuple in self.index.read().unwrap().values() {
                let path = path.join(format!("{}.json", index_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Index>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != index_tuple.0.read().unwrap().to_owned()
                    {
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
                    if !self.index.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Integer Literal.
        {
            let path = path.join("integer_literal");
            fs::create_dir_all(&path)?;
            for integer_literal_tuple in self.integer_literal.read().unwrap().values() {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.integer_literal.read().unwrap().contains_key(&id) {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.item.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Let Statement.
        {
            let path = path.join("let_statement");
            fs::create_dir_all(&path)?;
            for let_statement_tuple in self.let_statement.read().unwrap().values() {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.let_statement.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist List.
        {
            let path = path.join("list");
            fs::create_dir_all(&path)?;
            for list_tuple in self.list.read().unwrap().values() {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.list.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist List Element.
        {
            let path = path.join("list_element");
            fs::create_dir_all(&path)?;
            for list_element_tuple in self.list_element.read().unwrap().values() {
                let path = path.join(format!("{}.json", list_element_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<ListElement>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != list_element_tuple.0.read().unwrap().to_owned()
                    {
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
                    if !self.list_element.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist List Expression.
        {
            let path = path.join("list_expression");
            fs::create_dir_all(&path)?;
            for list_expression_tuple in self.list_expression.read().unwrap().values() {
                let path = path.join(format!(
                    "{}.json",
                    list_expression_tuple.0.read().unwrap().id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<ListExpression>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != list_expression_tuple.0.read().unwrap().to_owned()
                    {
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
                    if !self.list_expression.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Literal.
        {
            let path = path.join("literal");
            fs::create_dir_all(&path)?;
            for literal_tuple in self.literal.read().unwrap().values() {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.literal.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Local Variable.
        {
            let path = path.join("local_variable");
            fs::create_dir_all(&path)?;
            for local_variable_tuple in self.local_variable.read().unwrap().values() {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.local_variable.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Macro.
        {
            let path = path.join("x_macro");
            fs::create_dir_all(&path)?;
            for x_macro_tuple in self.x_macro.read().unwrap().values() {
                let path = path.join(format!("{}.json", x_macro_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<XMacro>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != x_macro_tuple.0.read().unwrap().to_owned()
                    {
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
                    if !self.x_macro.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Method Call.
        {
            let path = path.join("method_call");
            fs::create_dir_all(&path)?;
            for method_call_tuple in self.method_call.read().unwrap().values() {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.method_call.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Object Store.
        {
            let path = path.join("z_object_store");
            fs::create_dir_all(&path)?;
            for z_object_store_tuple in self.z_object_store.read().unwrap().values() {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.z_object_store.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Operator.
        {
            let path = path.join("operator");
            fs::create_dir_all(&path)?;
            for operator_tuple in self.operator.read().unwrap().values() {
                let path = path.join(format!("{}.json", operator_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Operator>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != operator_tuple.0.read().unwrap().to_owned()
                    {
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
                    if !self.operator.read().unwrap().contains_key(&id) {
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

        // Persist Print.
        {
            let path = path.join("print");
            fs::create_dir_all(&path)?;
            for print_tuple in self.print.read().unwrap().values() {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.print.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Range Expression.
        {
            let path = path.join("range_expression");
            fs::create_dir_all(&path)?;
            for range_expression_tuple in self.range_expression.read().unwrap().values() {
                let path = path.join(format!(
                    "{}.json",
                    range_expression_tuple.0.read().unwrap().id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<RangeExpression>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != range_expression_tuple.0.read().unwrap().to_owned()
                    {
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
                    if !self.range_expression.read().unwrap().contains_key(&id) {
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

        // Persist Result Statement.
        {
            let path = path.join("result_statement");
            fs::create_dir_all(&path)?;
            for result_statement_tuple in self.result_statement.read().unwrap().values() {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.result_statement.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Return.
        {
            let path = path.join("x_return");
            fs::create_dir_all(&path)?;
            for x_return_tuple in self.x_return.read().unwrap().values() {
                let path = path.join(format!("{}.json", x_return_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<XReturn>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != x_return_tuple.0.read().unwrap().to_owned()
                    {
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
                    if !self.x_return.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Some.
        {
            let path = path.join("z_some");
            fs::create_dir_all(&path)?;
            for z_some_tuple in self.z_some.read().unwrap().values() {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.z_some.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Span.
        {
            let path = path.join("span");
            fs::create_dir_all(&path)?;
            for span_tuple in self.span.read().unwrap().values() {
                let path = path.join(format!("{}.json", span_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Span>>, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != span_tuple.0.read().unwrap().to_owned()
                    {
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
                    if !self.span.read().unwrap().contains_key(&id) {
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

        // Persist Static Method Call.
        {
            let path = path.join("static_method_call");
            fs::create_dir_all(&path)?;
            for static_method_call_tuple in self.static_method_call.read().unwrap().values() {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.static_method_call.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist String Literal.
        {
            let path = path.join("string_literal");
            fs::create_dir_all(&path)?;
            for string_literal_tuple in self.string_literal.read().unwrap().values() {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.string_literal.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Struct.
        {
            let path = path.join("woog_struct");
            fs::create_dir_all(&path)?;
            for woog_struct_tuple in self.woog_struct.read().unwrap().values() {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.woog_struct.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Struct Expression.
        {
            let path = path.join("struct_expression");
            fs::create_dir_all(&path)?;
            for struct_expression_tuple in self.struct_expression.read().unwrap().values() {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.struct_expression.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Type Cast.
        {
            let path = path.join("type_cast");
            fs::create_dir_all(&path)?;
            for type_cast_tuple in self.type_cast.read().unwrap().values() {
                let path = path.join(format!("{}.json", type_cast_tuple.0.read().unwrap().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<TypeCast>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != type_cast_tuple.0.read().unwrap().to_owned()
                    {
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
                    if !self.type_cast.read().unwrap().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Unary.
        {
            let path = path.join("unary");
            fs::create_dir_all(&path)?;
            for unary_tuple in self.unary.read().unwrap().values() {
                let path = path.join(format!("{}.json", unary_tuple.0.read().unwrap().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Unary>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.read().unwrap().to_owned()
                        != unary_tuple.0.read().unwrap().to_owned()
                    {
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
                    if !self.unary.read().unwrap().contains_key(&id) {
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

        // Persist Value Type.
        {
            let path = path.join("value_type");
            fs::create_dir_all(&path)?;
            for value_type_tuple in self.value_type.read().unwrap().values() {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.value_type.read().unwrap().contains_key(&id) {
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

        // Persist Variable Expression.
        {
            let path = path.join("variable_expression");
            fs::create_dir_all(&path)?;
            for variable_expression_tuple in self.variable_expression.read().unwrap().values() {
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
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.variable_expression.read().unwrap().contains_key(&id) {
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

        let store = Self::new();

        // Load Argument.
        {
            let path = path.join("argument");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let argument: (Arc<RwLock<Argument>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .argument
                    .write()
                    .unwrap()
                    .insert(argument.0.read().unwrap().id, argument.clone());
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
                let binary: (Arc<RwLock<Binary>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .binary
                    .write()
                    .unwrap()
                    .insert(binary.0.read().unwrap().id(), binary.clone());
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

        // Load Boolean Literal.
        {
            let path = path.join("boolean_literal");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let boolean_literal: (Arc<RwLock<BooleanLiteral>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.boolean_literal.write().unwrap().insert(
                    boolean_literal.0.read().unwrap().id(),
                    boolean_literal.clone(),
                );
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
                let boolean_operator: (Arc<RwLock<BooleanOperator>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.boolean_operator.write().unwrap().insert(
                    boolean_operator.0.read().unwrap().id(),
                    boolean_operator.clone(),
                );
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

        // Load Comparison.
        {
            let path = path.join("comparison");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let comparison: (Arc<RwLock<Comparison>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .comparison
                    .write()
                    .unwrap()
                    .insert(comparison.0.read().unwrap().id(), comparison.clone());
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
                let dwarf_source_file: (Arc<RwLock<DwarfSourceFile>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.dwarf_source_file.write().unwrap().insert(
                    dwarf_source_file.0.read().unwrap().id,
                    dwarf_source_file.clone(),
                );
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
                let error: (Arc<RwLock<Error>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .error
                    .write()
                    .unwrap()
                    .insert(error.0.read().unwrap().id(), error.clone());
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
                let error_expression: (Arc<RwLock<ErrorExpression>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.error_expression.write().unwrap().insert(
                    error_expression.0.read().unwrap().id,
                    error_expression.clone(),
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

        // Load Expression Statement.
        {
            let path = path.join("expression_statement");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let expression_statement: (Arc<RwLock<ExpressionStatement>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.expression_statement.write().unwrap().insert(
                    expression_statement.0.read().unwrap().id,
                    expression_statement.clone(),
                );
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

        // Load Field Access.
        {
            let path = path.join("field_access");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let field_access: (Arc<RwLock<FieldAccess>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .field_access
                    .write()
                    .unwrap()
                    .insert(field_access.0.read().unwrap().id, field_access.clone());
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
                let field_access_target: (Arc<RwLock<FieldAccessTarget>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.field_access_target.write().unwrap().insert(
                    field_access_target.0.read().unwrap().id(),
                    field_access_target.clone(),
                );
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
                let field_expression: (Arc<RwLock<FieldExpression>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.field_expression.write().unwrap().insert(
                    field_expression.0.read().unwrap().id,
                    field_expression.clone(),
                );
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
                let float_literal: (Arc<RwLock<FloatLiteral>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .float_literal
                    .write()
                    .unwrap()
                    .insert(float_literal.0.read().unwrap().id, float_literal.clone());
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
                let for_loop: (Arc<RwLock<ForLoop>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .for_loop
                    .write()
                    .unwrap()
                    .insert(for_loop.0.read().unwrap().id, for_loop.clone());
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

        // Load Grouped.
        {
            let path = path.join("grouped");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let grouped: (Arc<RwLock<Grouped>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .grouped
                    .write()
                    .unwrap()
                    .insert(grouped.0.read().unwrap().id, grouped.clone());
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
                let x_if: (Arc<RwLock<XIf>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .x_if
                    .write()
                    .unwrap()
                    .insert(x_if.0.read().unwrap().id, x_if.clone());
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
                let implementation: (Arc<RwLock<Implementation>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .implementation
                    .write()
                    .unwrap()
                    .insert(implementation.0.read().unwrap().id, implementation.clone());
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
                let import: (Arc<RwLock<Import>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .import
                    .write()
                    .unwrap()
                    .insert(import.0.read().unwrap().id, import.clone());
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
                let index: (Arc<RwLock<Index>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .index
                    .write()
                    .unwrap()
                    .insert(index.0.read().unwrap().id, index.clone());
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
                let integer_literal: (Arc<RwLock<IntegerLiteral>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.integer_literal.write().unwrap().insert(
                    integer_literal.0.read().unwrap().id,
                    integer_literal.clone(),
                );
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
                    .insert(item.0.read().unwrap().id, item.clone());
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
                let let_statement: (Arc<RwLock<LetStatement>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .let_statement
                    .write()
                    .unwrap()
                    .insert(let_statement.0.read().unwrap().id, let_statement.clone());
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
                let list: (Arc<RwLock<List>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .list
                    .write()
                    .unwrap()
                    .insert(list.0.read().unwrap().id, list.clone());
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
                let list_element: (Arc<RwLock<ListElement>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .list_element
                    .write()
                    .unwrap()
                    .insert(list_element.0.read().unwrap().id, list_element.clone());
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
                let list_expression: (Arc<RwLock<ListExpression>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.list_expression.write().unwrap().insert(
                    list_expression.0.read().unwrap().id,
                    list_expression.clone(),
                );
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
                let literal: (Arc<RwLock<Literal>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .literal
                    .write()
                    .unwrap()
                    .insert(literal.0.read().unwrap().id(), literal.clone());
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
                let local_variable: (Arc<RwLock<LocalVariable>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .local_variable
                    .write()
                    .unwrap()
                    .insert(local_variable.0.read().unwrap().id, local_variable.clone());
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
                let x_macro: (Arc<RwLock<XMacro>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .x_macro
                    .write()
                    .unwrap()
                    .insert(x_macro.0.read().unwrap().id, x_macro.clone());
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
                let method_call: (Arc<RwLock<MethodCall>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .method_call
                    .write()
                    .unwrap()
                    .insert(method_call.0.read().unwrap().id, method_call.clone());
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
                let z_object_store: (Arc<RwLock<ZObjectStore>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .z_object_store
                    .write()
                    .unwrap()
                    .insert(z_object_store.0.read().unwrap().id, z_object_store.clone());
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
                let operator: (Arc<RwLock<Operator>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .operator
                    .write()
                    .unwrap()
                    .insert(operator.0.read().unwrap().id, operator.clone());
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

        // Load Print.
        {
            let path = path.join("print");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let print: (Arc<RwLock<Print>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .print
                    .write()
                    .unwrap()
                    .insert(print.0.read().unwrap().id, print.clone());
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
                let range_expression: (Arc<RwLock<RangeExpression>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.range_expression.write().unwrap().insert(
                    range_expression.0.read().unwrap().id,
                    range_expression.clone(),
                );
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

        // Load Result Statement.
        {
            let path = path.join("result_statement");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let result_statement: (Arc<RwLock<ResultStatement>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.result_statement.write().unwrap().insert(
                    result_statement.0.read().unwrap().id,
                    result_statement.clone(),
                );
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
                let x_return: (Arc<RwLock<XReturn>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .x_return
                    .write()
                    .unwrap()
                    .insert(x_return.0.read().unwrap().id, x_return.clone());
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
                let z_some: (Arc<RwLock<ZSome>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .z_some
                    .write()
                    .unwrap()
                    .insert(z_some.0.read().unwrap().id, z_some.clone());
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
                let span: (Arc<RwLock<Span>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .span
                    .write()
                    .unwrap()
                    .insert(span.0.read().unwrap().id, span.clone());
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

        // Load Static Method Call.
        {
            let path = path.join("static_method_call");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let static_method_call: (Arc<RwLock<StaticMethodCall>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.static_method_call.write().unwrap().insert(
                    static_method_call.0.read().unwrap().id,
                    static_method_call.clone(),
                );
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
                let string_literal: (Arc<RwLock<StringLiteral>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .string_literal
                    .write()
                    .unwrap()
                    .insert(string_literal.0.read().unwrap().id, string_literal.clone());
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
                let woog_struct: (Arc<RwLock<WoogStruct>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.woog_struct_id_by_name.write().unwrap().insert(
                    woog_struct.0.read().unwrap().name.to_upper_camel_case(),
                    (woog_struct.0.read().unwrap().id, woog_struct.1),
                );
                store
                    .woog_struct
                    .write()
                    .unwrap()
                    .insert(woog_struct.0.read().unwrap().id, woog_struct.clone());
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
                let struct_expression: (Arc<RwLock<StructExpression>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.struct_expression.write().unwrap().insert(
                    struct_expression.0.read().unwrap().id,
                    struct_expression.clone(),
                );
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
                let type_cast: (Arc<RwLock<TypeCast>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .type_cast
                    .write()
                    .unwrap()
                    .insert(type_cast.0.read().unwrap().id, type_cast.clone());
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
                let unary: (Arc<RwLock<Unary>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .unary
                    .write()
                    .unwrap()
                    .insert(unary.0.read().unwrap().id(), unary.clone());
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

        // Load Value Type.
        {
            let path = path.join("value_type");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let value_type: (Arc<RwLock<ValueType>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .value_type
                    .write()
                    .unwrap()
                    .insert(value_type.0.read().unwrap().id(), value_type.clone());
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

        // Load Variable Expression.
        {
            let path = path.join("variable_expression");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let variable_expression: (Arc<RwLock<VariableExpression>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.variable_expression.write().unwrap().insert(
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
