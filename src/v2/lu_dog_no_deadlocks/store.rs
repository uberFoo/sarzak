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
use no_deadlocks::RwLock;
use std::sync::Arc;
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

use crate::v2::lu_dog_rwlock::types::{
    Argument, Binary, Block, BooleanLiteral, BooleanOperator, Call, Comparison, DwarfSourceFile,
    Error, ErrorExpression, Expression, ExpressionStatement, Field, FieldAccess, FieldAccessTarget,
    FieldExpression, FloatLiteral, ForLoop, Function, Grouped, Implementation, Import, Index,
    IntegerLiteral, Item, LetStatement, List, ListElement, ListExpression, Literal, LocalVariable,
    MethodCall, Operator, Parameter, Print, RangeExpression, Reference, ResultStatement, Span,
    Statement, StaticMethodCall, StringLiteral, StructExpression, TypeCast, Unary, ValueType,
    Variable, VariableExpression, WoogOption, WoogStruct, XIf, XReturn, XValue, ZObjectStore,
    ZSome, ADDITION, AND, ASSIGNMENT, DEBUGGER, DIVISION, EMPTY, EQUAL, FALSE_LITERAL,
    GREATER_THAN, GREATER_THAN_OR_EQUAL, LESS_THAN_OR_EQUAL, MULTIPLICATION, NEGATION, NOT, RANGE,
    SUBTRACTION, TRUE_LITERAL, UNKNOWN, UNKNOWN_VARIABLE, Z_NONE,
};

#[derive(Clone, Debug)]
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
        store.inter_comparison(Arc::new(RwLock::new(Comparison::Equal(EQUAL))));
        store.inter_comparison(Arc::new(RwLock::new(Comparison::GreaterThan(GREATER_THAN))));
        store.inter_comparison(Arc::new(RwLock::new(Comparison::GreaterThanOrEqual(
            GREATER_THAN_OR_EQUAL,
        ))));
        store.inter_comparison(Arc::new(RwLock::new(Comparison::LessThanOrEqual(
            LESS_THAN_OR_EQUAL,
        ))));
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
}
