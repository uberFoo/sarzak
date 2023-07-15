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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog-object-store-definition"}}}
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

use crate::v2::lu_dog::types::{
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
    argument: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Argument>>, SystemTime)>>>,
    binary: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Binary>>, SystemTime)>>>,
    block: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Block>>, SystemTime)>>>,
    boolean_literal: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<BooleanLiteral>>, SystemTime)>>>,
    boolean_operator: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<BooleanOperator>>, SystemTime)>>>,
    call: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Call>>, SystemTime)>>>,
    comparison: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Comparison>>, SystemTime)>>>,
    dwarf_source_file: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<DwarfSourceFile>>, SystemTime)>>>,
    error: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Error>>, SystemTime)>>>,
    error_expression: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<ErrorExpression>>, SystemTime)>>>,
    expression: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Expression>>, SystemTime)>>>,
    expression_statement:
        Rc<RefCell<HashMap<Uuid, (Rc<RefCell<ExpressionStatement>>, SystemTime)>>>,
    field: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Field>>, SystemTime)>>>,
    field_id_by_name: Rc<RefCell<HashMap<String, (Uuid, SystemTime)>>>,
    field_access: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<FieldAccess>>, SystemTime)>>>,
    field_access_target: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<FieldAccessTarget>>, SystemTime)>>>,
    field_expression: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<FieldExpression>>, SystemTime)>>>,
    float_literal: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<FloatLiteral>>, SystemTime)>>>,
    for_loop: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<ForLoop>>, SystemTime)>>>,
    function: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Function>>, SystemTime)>>>,
    function_id_by_name: Rc<RefCell<HashMap<String, (Uuid, SystemTime)>>>,
    grouped: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Grouped>>, SystemTime)>>>,
    x_if: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<XIf>>, SystemTime)>>>,
    implementation: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Implementation>>, SystemTime)>>>,
    import: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Import>>, SystemTime)>>>,
    index: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Index>>, SystemTime)>>>,
    integer_literal: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<IntegerLiteral>>, SystemTime)>>>,
    item: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Item>>, SystemTime)>>>,
    let_statement: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<LetStatement>>, SystemTime)>>>,
    list: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<List>>, SystemTime)>>>,
    list_element: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<ListElement>>, SystemTime)>>>,
    list_expression: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<ListExpression>>, SystemTime)>>>,
    literal: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Literal>>, SystemTime)>>>,
    local_variable: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<LocalVariable>>, SystemTime)>>>,
    x_macro: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<XMacro>>, SystemTime)>>>,
    method_call: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<MethodCall>>, SystemTime)>>>,
    z_object_store: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<ZObjectStore>>, SystemTime)>>>,
    operator: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Operator>>, SystemTime)>>>,
    woog_option: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<WoogOption>>, SystemTime)>>>,
    parameter: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Parameter>>, SystemTime)>>>,
    print: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Print>>, SystemTime)>>>,
    range_expression: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<RangeExpression>>, SystemTime)>>>,
    reference: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Reference>>, SystemTime)>>>,
    result_statement: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<ResultStatement>>, SystemTime)>>>,
    x_return: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<XReturn>>, SystemTime)>>>,
    z_some: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<ZSome>>, SystemTime)>>>,
    span: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Span>>, SystemTime)>>>,
    statement: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Statement>>, SystemTime)>>>,
    static_method_call: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<StaticMethodCall>>, SystemTime)>>>,
    string_literal: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<StringLiteral>>, SystemTime)>>>,
    woog_struct: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<WoogStruct>>, SystemTime)>>>,
    woog_struct_id_by_name: Rc<RefCell<HashMap<String, (Uuid, SystemTime)>>>,
    struct_expression: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<StructExpression>>, SystemTime)>>>,
    type_cast: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<TypeCast>>, SystemTime)>>>,
    unary: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Unary>>, SystemTime)>>>,
    x_value: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<XValue>>, SystemTime)>>>,
    value_type: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<ValueType>>, SystemTime)>>>,
    variable: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<Variable>>, SystemTime)>>>,
    variable_expression: Rc<RefCell<HashMap<Uuid, (Rc<RefCell<VariableExpression>>, SystemTime)>>>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            argument: Rc::new(RefCell::new(HashMap::default())),
            binary: Rc::new(RefCell::new(HashMap::default())),
            block: Rc::new(RefCell::new(HashMap::default())),
            boolean_literal: Rc::new(RefCell::new(HashMap::default())),
            boolean_operator: Rc::new(RefCell::new(HashMap::default())),
            call: Rc::new(RefCell::new(HashMap::default())),
            comparison: Rc::new(RefCell::new(HashMap::default())),
            dwarf_source_file: Rc::new(RefCell::new(HashMap::default())),
            error: Rc::new(RefCell::new(HashMap::default())),
            error_expression: Rc::new(RefCell::new(HashMap::default())),
            expression: Rc::new(RefCell::new(HashMap::default())),
            expression_statement: Rc::new(RefCell::new(HashMap::default())),
            field: Rc::new(RefCell::new(HashMap::default())),
            field_id_by_name: Rc::new(RefCell::new(HashMap::default())),
            field_access: Rc::new(RefCell::new(HashMap::default())),
            field_access_target: Rc::new(RefCell::new(HashMap::default())),
            field_expression: Rc::new(RefCell::new(HashMap::default())),
            float_literal: Rc::new(RefCell::new(HashMap::default())),
            for_loop: Rc::new(RefCell::new(HashMap::default())),
            function: Rc::new(RefCell::new(HashMap::default())),
            function_id_by_name: Rc::new(RefCell::new(HashMap::default())),
            grouped: Rc::new(RefCell::new(HashMap::default())),
            x_if: Rc::new(RefCell::new(HashMap::default())),
            implementation: Rc::new(RefCell::new(HashMap::default())),
            import: Rc::new(RefCell::new(HashMap::default())),
            index: Rc::new(RefCell::new(HashMap::default())),
            integer_literal: Rc::new(RefCell::new(HashMap::default())),
            item: Rc::new(RefCell::new(HashMap::default())),
            let_statement: Rc::new(RefCell::new(HashMap::default())),
            list: Rc::new(RefCell::new(HashMap::default())),
            list_element: Rc::new(RefCell::new(HashMap::default())),
            list_expression: Rc::new(RefCell::new(HashMap::default())),
            literal: Rc::new(RefCell::new(HashMap::default())),
            local_variable: Rc::new(RefCell::new(HashMap::default())),
            x_macro: Rc::new(RefCell::new(HashMap::default())),
            method_call: Rc::new(RefCell::new(HashMap::default())),
            z_object_store: Rc::new(RefCell::new(HashMap::default())),
            operator: Rc::new(RefCell::new(HashMap::default())),
            woog_option: Rc::new(RefCell::new(HashMap::default())),
            parameter: Rc::new(RefCell::new(HashMap::default())),
            print: Rc::new(RefCell::new(HashMap::default())),
            range_expression: Rc::new(RefCell::new(HashMap::default())),
            reference: Rc::new(RefCell::new(HashMap::default())),
            result_statement: Rc::new(RefCell::new(HashMap::default())),
            x_return: Rc::new(RefCell::new(HashMap::default())),
            z_some: Rc::new(RefCell::new(HashMap::default())),
            span: Rc::new(RefCell::new(HashMap::default())),
            statement: Rc::new(RefCell::new(HashMap::default())),
            static_method_call: Rc::new(RefCell::new(HashMap::default())),
            string_literal: Rc::new(RefCell::new(HashMap::default())),
            woog_struct: Rc::new(RefCell::new(HashMap::default())),
            woog_struct_id_by_name: Rc::new(RefCell::new(HashMap::default())),
            struct_expression: Rc::new(RefCell::new(HashMap::default())),
            type_cast: Rc::new(RefCell::new(HashMap::default())),
            unary: Rc::new(RefCell::new(HashMap::default())),
            x_value: Rc::new(RefCell::new(HashMap::default())),
            value_type: Rc::new(RefCell::new(HashMap::default())),
            variable: Rc::new(RefCell::new(HashMap::default())),
            variable_expression: Rc::new(RefCell::new(HashMap::default())),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥
        store.inter_binary(Rc::new(RefCell::new(Binary::Addition(ADDITION))));
        store.inter_binary(Rc::new(RefCell::new(Binary::Assignment(ASSIGNMENT))));
        store.inter_binary(Rc::new(RefCell::new(Binary::BooleanOperator(
            BooleanOperator::And(AND).id(),
        ))));
        store.inter_binary(Rc::new(RefCell::new(Binary::BooleanOperator(
            BooleanOperator::Or(OR).id(),
        ))));
        store.inter_binary(Rc::new(RefCell::new(Binary::Division(DIVISION))));
        store.inter_binary(Rc::new(RefCell::new(Binary::Multiplication(
            MULTIPLICATION,
        ))));
        store.inter_binary(Rc::new(RefCell::new(Binary::Subtraction(SUBTRACTION))));
        store.inter_boolean_literal(Rc::new(RefCell::new(BooleanLiteral::FalseLiteral(
            FALSE_LITERAL,
        ))));
        store.inter_boolean_literal(Rc::new(RefCell::new(BooleanLiteral::TrueLiteral(
            TRUE_LITERAL,
        ))));
        store.inter_boolean_operator(Rc::new(RefCell::new(BooleanOperator::And(AND))));
        store.inter_boolean_operator(Rc::new(RefCell::new(BooleanOperator::Or(OR))));
        store.inter_comparison(Rc::new(RefCell::new(Comparison::Equal(EQUAL))));
        store.inter_comparison(Rc::new(RefCell::new(Comparison::GreaterThan(GREATER_THAN))));
        store.inter_comparison(Rc::new(RefCell::new(Comparison::GreaterThanOrEqual(
            GREATER_THAN_OR_EQUAL,
        ))));
        store.inter_comparison(Rc::new(RefCell::new(Comparison::LessThan(LESS_THAN))));
        store.inter_comparison(Rc::new(RefCell::new(Comparison::LessThanOrEqual(
            LESS_THAN_OR_EQUAL,
        ))));
        store.inter_comparison(Rc::new(RefCell::new(Comparison::NotEqual(NOT_EQUAL))));
        store.inter_error(Rc::new(RefCell::new(Error::UnknownVariable(
            UNKNOWN_VARIABLE,
        ))));
        store.inter_expression(Rc::new(RefCell::new(Expression::Debugger(DEBUGGER))));
        store.inter_expression(Rc::new(RefCell::new(Expression::Literal(
            Literal::BooleanLiteral(BooleanLiteral::FalseLiteral(FALSE_LITERAL).id()).id(),
        ))));
        store.inter_expression(Rc::new(RefCell::new(Expression::Literal(
            Literal::BooleanLiteral(BooleanLiteral::TrueLiteral(TRUE_LITERAL).id()).id(),
        ))));
        store.inter_expression(Rc::new(RefCell::new(Expression::ZNone(Z_NONE))));
        store.inter_literal(Rc::new(RefCell::new(Literal::BooleanLiteral(
            BooleanLiteral::FalseLiteral(FALSE_LITERAL).id(),
        ))));
        store.inter_literal(Rc::new(RefCell::new(Literal::BooleanLiteral(
            BooleanLiteral::TrueLiteral(TRUE_LITERAL).id(),
        ))));
        store.inter_unary(Rc::new(RefCell::new(Unary::Negation(NEGATION))));
        store.inter_unary(Rc::new(RefCell::new(Unary::Not(NOT))));
        store.inter_value_type(Rc::new(RefCell::new(ValueType::Empty(EMPTY))));
        store.inter_value_type(Rc::new(RefCell::new(ValueType::Error(
            Error::UnknownVariable(UNKNOWN_VARIABLE).id(),
        ))));
        store.inter_value_type(Rc::new(RefCell::new(ValueType::Range(RANGE))));
        store.inter_value_type(Rc::new(RefCell::new(ValueType::Unknown(UNKNOWN))));

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog-object-store-methods"}}}
    /// Inter (insert) [`Argument`] into the store.
    ///
    pub fn inter_argument(&mut self, argument: Rc<RefCell<Argument>>) {
        let read = argument.borrow();
        self.argument
            .borrow_mut()
            .insert(read.id, (argument.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Argument`] from the store.
    ///
    pub fn exhume_argument(&self, id: &Uuid) -> Option<Rc<RefCell<Argument>>> {
        self.argument
            .borrow()
            .get(id)
            .map(|argument| argument.0.clone())
    }

    /// Exorcise (remove) [`Argument`] from the store.
    ///
    pub fn exorcise_argument(&mut self, id: &Uuid) -> Option<Rc<RefCell<Argument>>> {
        self.argument
            .borrow_mut()
            .remove(id)
            .map(|argument| argument.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Argument>`.
    ///
    pub fn iter_argument(&self) -> impl Iterator<Item = Rc<RefCell<Argument>>> + '_ {
        let values: Vec<Rc<RefCell<Argument>>> = self
            .argument
            .borrow()
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
            .borrow()
            .get(&argument.id)
            .map(|argument| argument.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Binary`] into the store.
    ///
    pub fn inter_binary(&mut self, binary: Rc<RefCell<Binary>>) {
        let read = binary.borrow();
        self.binary
            .borrow_mut()
            .insert(read.id(), (binary.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Binary`] from the store.
    ///
    pub fn exhume_binary(&self, id: &Uuid) -> Option<Rc<RefCell<Binary>>> {
        self.binary.borrow().get(id).map(|binary| binary.0.clone())
    }

    /// Exorcise (remove) [`Binary`] from the store.
    ///
    pub fn exorcise_binary(&mut self, id: &Uuid) -> Option<Rc<RefCell<Binary>>> {
        self.binary
            .borrow_mut()
            .remove(id)
            .map(|binary| binary.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Binary>`.
    ///
    pub fn iter_binary(&self) -> impl Iterator<Item = Rc<RefCell<Binary>>> + '_ {
        let values: Vec<Rc<RefCell<Binary>>> = self
            .binary
            .borrow()
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
            .borrow()
            .get(&binary.id())
            .map(|binary| binary.1)
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

    /// Inter (insert) [`BooleanLiteral`] into the store.
    ///
    pub fn inter_boolean_literal(&mut self, boolean_literal: Rc<RefCell<BooleanLiteral>>) {
        let read = boolean_literal.borrow();
        self.boolean_literal
            .borrow_mut()
            .insert(read.id(), (boolean_literal.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`BooleanLiteral`] from the store.
    ///
    pub fn exhume_boolean_literal(&self, id: &Uuid) -> Option<Rc<RefCell<BooleanLiteral>>> {
        self.boolean_literal
            .borrow()
            .get(id)
            .map(|boolean_literal| boolean_literal.0.clone())
    }

    /// Exorcise (remove) [`BooleanLiteral`] from the store.
    ///
    pub fn exorcise_boolean_literal(&mut self, id: &Uuid) -> Option<Rc<RefCell<BooleanLiteral>>> {
        self.boolean_literal
            .borrow_mut()
            .remove(id)
            .map(|boolean_literal| boolean_literal.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanLiteral>`.
    ///
    pub fn iter_boolean_literal(&self) -> impl Iterator<Item = Rc<RefCell<BooleanLiteral>>> + '_ {
        let values: Vec<Rc<RefCell<BooleanLiteral>>> = self
            .boolean_literal
            .borrow()
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
            .borrow()
            .get(&boolean_literal.id())
            .map(|boolean_literal| boolean_literal.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`BooleanOperator`] into the store.
    ///
    pub fn inter_boolean_operator(&mut self, boolean_operator: Rc<RefCell<BooleanOperator>>) {
        let read = boolean_operator.borrow();
        self.boolean_operator
            .borrow_mut()
            .insert(read.id(), (boolean_operator.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`BooleanOperator`] from the store.
    ///
    pub fn exhume_boolean_operator(&self, id: &Uuid) -> Option<Rc<RefCell<BooleanOperator>>> {
        self.boolean_operator
            .borrow()
            .get(id)
            .map(|boolean_operator| boolean_operator.0.clone())
    }

    /// Exorcise (remove) [`BooleanOperator`] from the store.
    ///
    pub fn exorcise_boolean_operator(&mut self, id: &Uuid) -> Option<Rc<RefCell<BooleanOperator>>> {
        self.boolean_operator
            .borrow_mut()
            .remove(id)
            .map(|boolean_operator| boolean_operator.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanOperator>`.
    ///
    pub fn iter_boolean_operator(&self) -> impl Iterator<Item = Rc<RefCell<BooleanOperator>>> + '_ {
        let values: Vec<Rc<RefCell<BooleanOperator>>> = self
            .boolean_operator
            .borrow()
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
            .borrow()
            .get(&boolean_operator.id())
            .map(|boolean_operator| boolean_operator.1)
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

    /// Inter (insert) [`Comparison`] into the store.
    ///
    pub fn inter_comparison(&mut self, comparison: Rc<RefCell<Comparison>>) {
        let read = comparison.borrow();
        self.comparison
            .borrow_mut()
            .insert(read.id(), (comparison.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Comparison`] from the store.
    ///
    pub fn exhume_comparison(&self, id: &Uuid) -> Option<Rc<RefCell<Comparison>>> {
        self.comparison
            .borrow()
            .get(id)
            .map(|comparison| comparison.0.clone())
    }

    /// Exorcise (remove) [`Comparison`] from the store.
    ///
    pub fn exorcise_comparison(&mut self, id: &Uuid) -> Option<Rc<RefCell<Comparison>>> {
        self.comparison
            .borrow_mut()
            .remove(id)
            .map(|comparison| comparison.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Comparison>`.
    ///
    pub fn iter_comparison(&self) -> impl Iterator<Item = Rc<RefCell<Comparison>>> + '_ {
        let values: Vec<Rc<RefCell<Comparison>>> = self
            .comparison
            .borrow()
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
            .borrow()
            .get(&comparison.id())
            .map(|comparison| comparison.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`DwarfSourceFile`] into the store.
    ///
    pub fn inter_dwarf_source_file(&mut self, dwarf_source_file: Rc<RefCell<DwarfSourceFile>>) {
        let read = dwarf_source_file.borrow();
        self.dwarf_source_file
            .borrow_mut()
            .insert(read.id, (dwarf_source_file.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`DwarfSourceFile`] from the store.
    ///
    pub fn exhume_dwarf_source_file(&self, id: &Uuid) -> Option<Rc<RefCell<DwarfSourceFile>>> {
        self.dwarf_source_file
            .borrow()
            .get(id)
            .map(|dwarf_source_file| dwarf_source_file.0.clone())
    }

    /// Exorcise (remove) [`DwarfSourceFile`] from the store.
    ///
    pub fn exorcise_dwarf_source_file(
        &mut self,
        id: &Uuid,
    ) -> Option<Rc<RefCell<DwarfSourceFile>>> {
        self.dwarf_source_file
            .borrow_mut()
            .remove(id)
            .map(|dwarf_source_file| dwarf_source_file.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, DwarfSourceFile>`.
    ///
    pub fn iter_dwarf_source_file(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<DwarfSourceFile>>> + '_ {
        let values: Vec<Rc<RefCell<DwarfSourceFile>>> = self
            .dwarf_source_file
            .borrow()
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
            .borrow()
            .get(&dwarf_source_file.id)
            .map(|dwarf_source_file| dwarf_source_file.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Error`] into the store.
    ///
    pub fn inter_error(&mut self, error: Rc<RefCell<Error>>) {
        let read = error.borrow();
        self.error
            .borrow_mut()
            .insert(read.id(), (error.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Error`] from the store.
    ///
    pub fn exhume_error(&self, id: &Uuid) -> Option<Rc<RefCell<Error>>> {
        self.error.borrow().get(id).map(|error| error.0.clone())
    }

    /// Exorcise (remove) [`Error`] from the store.
    ///
    pub fn exorcise_error(&mut self, id: &Uuid) -> Option<Rc<RefCell<Error>>> {
        self.error
            .borrow_mut()
            .remove(id)
            .map(|error| error.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Error>`.
    ///
    pub fn iter_error(&self) -> impl Iterator<Item = Rc<RefCell<Error>>> + '_ {
        let values: Vec<Rc<RefCell<Error>>> = self
            .error
            .borrow()
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
            .borrow()
            .get(&error.id())
            .map(|error| error.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ErrorExpression`] into the store.
    ///
    pub fn inter_error_expression(&mut self, error_expression: Rc<RefCell<ErrorExpression>>) {
        let read = error_expression.borrow();
        self.error_expression
            .borrow_mut()
            .insert(read.id, (error_expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ErrorExpression`] from the store.
    ///
    pub fn exhume_error_expression(&self, id: &Uuid) -> Option<Rc<RefCell<ErrorExpression>>> {
        self.error_expression
            .borrow()
            .get(id)
            .map(|error_expression| error_expression.0.clone())
    }

    /// Exorcise (remove) [`ErrorExpression`] from the store.
    ///
    pub fn exorcise_error_expression(&mut self, id: &Uuid) -> Option<Rc<RefCell<ErrorExpression>>> {
        self.error_expression
            .borrow_mut()
            .remove(id)
            .map(|error_expression| error_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ErrorExpression>`.
    ///
    pub fn iter_error_expression(&self) -> impl Iterator<Item = Rc<RefCell<ErrorExpression>>> + '_ {
        let values: Vec<Rc<RefCell<ErrorExpression>>> = self
            .error_expression
            .borrow()
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
            .borrow()
            .get(&error_expression.id)
            .map(|error_expression| error_expression.1)
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

    /// Inter (insert) [`ExpressionStatement`] into the store.
    ///
    pub fn inter_expression_statement(
        &mut self,
        expression_statement: Rc<RefCell<ExpressionStatement>>,
    ) {
        let read = expression_statement.borrow();
        self.expression_statement
            .borrow_mut()
            .insert(read.id, (expression_statement.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ExpressionStatement`] from the store.
    ///
    pub fn exhume_expression_statement(
        &self,
        id: &Uuid,
    ) -> Option<Rc<RefCell<ExpressionStatement>>> {
        self.expression_statement
            .borrow()
            .get(id)
            .map(|expression_statement| expression_statement.0.clone())
    }

    /// Exorcise (remove) [`ExpressionStatement`] from the store.
    ///
    pub fn exorcise_expression_statement(
        &mut self,
        id: &Uuid,
    ) -> Option<Rc<RefCell<ExpressionStatement>>> {
        self.expression_statement
            .borrow_mut()
            .remove(id)
            .map(|expression_statement| expression_statement.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ExpressionStatement>`.
    ///
    pub fn iter_expression_statement(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<ExpressionStatement>>> + '_ {
        let values: Vec<Rc<RefCell<ExpressionStatement>>> = self
            .expression_statement
            .borrow()
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
            .borrow()
            .get(&expression_statement.id)
            .map(|expression_statement| expression_statement.1)
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

    /// Inter (insert) [`FieldAccess`] into the store.
    ///
    pub fn inter_field_access(&mut self, field_access: Rc<RefCell<FieldAccess>>) {
        let read = field_access.borrow();
        self.field_access
            .borrow_mut()
            .insert(read.id, (field_access.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`FieldAccess`] from the store.
    ///
    pub fn exhume_field_access(&self, id: &Uuid) -> Option<Rc<RefCell<FieldAccess>>> {
        self.field_access
            .borrow()
            .get(id)
            .map(|field_access| field_access.0.clone())
    }

    /// Exorcise (remove) [`FieldAccess`] from the store.
    ///
    pub fn exorcise_field_access(&mut self, id: &Uuid) -> Option<Rc<RefCell<FieldAccess>>> {
        self.field_access
            .borrow_mut()
            .remove(id)
            .map(|field_access| field_access.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccess>`.
    ///
    pub fn iter_field_access(&self) -> impl Iterator<Item = Rc<RefCell<FieldAccess>>> + '_ {
        let values: Vec<Rc<RefCell<FieldAccess>>> = self
            .field_access
            .borrow()
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
            .borrow()
            .get(&field_access.id)
            .map(|field_access| field_access.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`FieldAccessTarget`] into the store.
    ///
    pub fn inter_field_access_target(
        &mut self,
        field_access_target: Rc<RefCell<FieldAccessTarget>>,
    ) {
        let read = field_access_target.borrow();
        self.field_access_target
            .borrow_mut()
            .insert(read.id(), (field_access_target.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`FieldAccessTarget`] from the store.
    ///
    pub fn exhume_field_access_target(&self, id: &Uuid) -> Option<Rc<RefCell<FieldAccessTarget>>> {
        self.field_access_target
            .borrow()
            .get(id)
            .map(|field_access_target| field_access_target.0.clone())
    }

    /// Exorcise (remove) [`FieldAccessTarget`] from the store.
    ///
    pub fn exorcise_field_access_target(
        &mut self,
        id: &Uuid,
    ) -> Option<Rc<RefCell<FieldAccessTarget>>> {
        self.field_access_target
            .borrow_mut()
            .remove(id)
            .map(|field_access_target| field_access_target.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccessTarget>`.
    ///
    pub fn iter_field_access_target(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<FieldAccessTarget>>> + '_ {
        let values: Vec<Rc<RefCell<FieldAccessTarget>>> = self
            .field_access_target
            .borrow()
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
            .borrow()
            .get(&field_access_target.id())
            .map(|field_access_target| field_access_target.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`FieldExpression`] into the store.
    ///
    pub fn inter_field_expression(&mut self, field_expression: Rc<RefCell<FieldExpression>>) {
        let read = field_expression.borrow();
        self.field_expression
            .borrow_mut()
            .insert(read.id, (field_expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`FieldExpression`] from the store.
    ///
    pub fn exhume_field_expression(&self, id: &Uuid) -> Option<Rc<RefCell<FieldExpression>>> {
        self.field_expression
            .borrow()
            .get(id)
            .map(|field_expression| field_expression.0.clone())
    }

    /// Exorcise (remove) [`FieldExpression`] from the store.
    ///
    pub fn exorcise_field_expression(&mut self, id: &Uuid) -> Option<Rc<RefCell<FieldExpression>>> {
        self.field_expression
            .borrow_mut()
            .remove(id)
            .map(|field_expression| field_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldExpression>`.
    ///
    pub fn iter_field_expression(&self) -> impl Iterator<Item = Rc<RefCell<FieldExpression>>> + '_ {
        let values: Vec<Rc<RefCell<FieldExpression>>> = self
            .field_expression
            .borrow()
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
            .borrow()
            .get(&field_expression.id)
            .map(|field_expression| field_expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`FloatLiteral`] into the store.
    ///
    pub fn inter_float_literal(&mut self, float_literal: Rc<RefCell<FloatLiteral>>) {
        let read = float_literal.borrow();
        self.float_literal
            .borrow_mut()
            .insert(read.id, (float_literal.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`FloatLiteral`] from the store.
    ///
    pub fn exhume_float_literal(&self, id: &Uuid) -> Option<Rc<RefCell<FloatLiteral>>> {
        self.float_literal
            .borrow()
            .get(id)
            .map(|float_literal| float_literal.0.clone())
    }

    /// Exorcise (remove) [`FloatLiteral`] from the store.
    ///
    pub fn exorcise_float_literal(&mut self, id: &Uuid) -> Option<Rc<RefCell<FloatLiteral>>> {
        self.float_literal
            .borrow_mut()
            .remove(id)
            .map(|float_literal| float_literal.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FloatLiteral>`.
    ///
    pub fn iter_float_literal(&self) -> impl Iterator<Item = Rc<RefCell<FloatLiteral>>> + '_ {
        let values: Vec<Rc<RefCell<FloatLiteral>>> = self
            .float_literal
            .borrow()
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
            .borrow()
            .get(&float_literal.id)
            .map(|float_literal| float_literal.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ForLoop`] into the store.
    ///
    pub fn inter_for_loop(&mut self, for_loop: Rc<RefCell<ForLoop>>) {
        let read = for_loop.borrow();
        self.for_loop
            .borrow_mut()
            .insert(read.id, (for_loop.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ForLoop`] from the store.
    ///
    pub fn exhume_for_loop(&self, id: &Uuid) -> Option<Rc<RefCell<ForLoop>>> {
        self.for_loop
            .borrow()
            .get(id)
            .map(|for_loop| for_loop.0.clone())
    }

    /// Exorcise (remove) [`ForLoop`] from the store.
    ///
    pub fn exorcise_for_loop(&mut self, id: &Uuid) -> Option<Rc<RefCell<ForLoop>>> {
        self.for_loop
            .borrow_mut()
            .remove(id)
            .map(|for_loop| for_loop.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ForLoop>`.
    ///
    pub fn iter_for_loop(&self) -> impl Iterator<Item = Rc<RefCell<ForLoop>>> + '_ {
        let values: Vec<Rc<RefCell<ForLoop>>> = self
            .for_loop
            .borrow()
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
            .borrow()
            .get(&for_loop.id)
            .map(|for_loop| for_loop.1)
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

    /// Inter (insert) [`Grouped`] into the store.
    ///
    pub fn inter_grouped(&mut self, grouped: Rc<RefCell<Grouped>>) {
        let read = grouped.borrow();
        self.grouped
            .borrow_mut()
            .insert(read.id, (grouped.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Grouped`] from the store.
    ///
    pub fn exhume_grouped(&self, id: &Uuid) -> Option<Rc<RefCell<Grouped>>> {
        self.grouped
            .borrow()
            .get(id)
            .map(|grouped| grouped.0.clone())
    }

    /// Exorcise (remove) [`Grouped`] from the store.
    ///
    pub fn exorcise_grouped(&mut self, id: &Uuid) -> Option<Rc<RefCell<Grouped>>> {
        self.grouped
            .borrow_mut()
            .remove(id)
            .map(|grouped| grouped.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Grouped>`.
    ///
    pub fn iter_grouped(&self) -> impl Iterator<Item = Rc<RefCell<Grouped>>> + '_ {
        let values: Vec<Rc<RefCell<Grouped>>> = self
            .grouped
            .borrow()
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
            .borrow()
            .get(&grouped.id)
            .map(|grouped| grouped.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`XIf`] into the store.
    ///
    pub fn inter_x_if(&mut self, x_if: Rc<RefCell<XIf>>) {
        let read = x_if.borrow();
        self.x_if
            .borrow_mut()
            .insert(read.id, (x_if.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`XIf`] from the store.
    ///
    pub fn exhume_x_if(&self, id: &Uuid) -> Option<Rc<RefCell<XIf>>> {
        self.x_if.borrow().get(id).map(|x_if| x_if.0.clone())
    }

    /// Exorcise (remove) [`XIf`] from the store.
    ///
    pub fn exorcise_x_if(&mut self, id: &Uuid) -> Option<Rc<RefCell<XIf>>> {
        self.x_if.borrow_mut().remove(id).map(|x_if| x_if.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XIf>`.
    ///
    pub fn iter_x_if(&self) -> impl Iterator<Item = Rc<RefCell<XIf>>> + '_ {
        let values: Vec<Rc<RefCell<XIf>>> = self
            .x_if
            .borrow()
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
            .borrow()
            .get(&x_if.id)
            .map(|x_if| x_if.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Implementation`] into the store.
    ///
    pub fn inter_implementation(&mut self, implementation: Rc<RefCell<Implementation>>) {
        let read = implementation.borrow();
        self.implementation
            .borrow_mut()
            .insert(read.id, (implementation.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Implementation`] from the store.
    ///
    pub fn exhume_implementation(&self, id: &Uuid) -> Option<Rc<RefCell<Implementation>>> {
        self.implementation
            .borrow()
            .get(id)
            .map(|implementation| implementation.0.clone())
    }

    /// Exorcise (remove) [`Implementation`] from the store.
    ///
    pub fn exorcise_implementation(&mut self, id: &Uuid) -> Option<Rc<RefCell<Implementation>>> {
        self.implementation
            .borrow_mut()
            .remove(id)
            .map(|implementation| implementation.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Implementation>`.
    ///
    pub fn iter_implementation(&self) -> impl Iterator<Item = Rc<RefCell<Implementation>>> + '_ {
        let values: Vec<Rc<RefCell<Implementation>>> = self
            .implementation
            .borrow()
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
            .borrow()
            .get(&implementation.id)
            .map(|implementation| implementation.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Import`] into the store.
    ///
    pub fn inter_import(&mut self, import: Rc<RefCell<Import>>) {
        let read = import.borrow();
        self.import
            .borrow_mut()
            .insert(read.id, (import.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Import`] from the store.
    ///
    pub fn exhume_import(&self, id: &Uuid) -> Option<Rc<RefCell<Import>>> {
        self.import.borrow().get(id).map(|import| import.0.clone())
    }

    /// Exorcise (remove) [`Import`] from the store.
    ///
    pub fn exorcise_import(&mut self, id: &Uuid) -> Option<Rc<RefCell<Import>>> {
        self.import
            .borrow_mut()
            .remove(id)
            .map(|import| import.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Import>`.
    ///
    pub fn iter_import(&self) -> impl Iterator<Item = Rc<RefCell<Import>>> + '_ {
        let values: Vec<Rc<RefCell<Import>>> = self
            .import
            .borrow()
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
            .borrow()
            .get(&import.id)
            .map(|import| import.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Index`] into the store.
    ///
    pub fn inter_index(&mut self, index: Rc<RefCell<Index>>) {
        let read = index.borrow();
        self.index
            .borrow_mut()
            .insert(read.id, (index.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Index`] from the store.
    ///
    pub fn exhume_index(&self, id: &Uuid) -> Option<Rc<RefCell<Index>>> {
        self.index.borrow().get(id).map(|index| index.0.clone())
    }

    /// Exorcise (remove) [`Index`] from the store.
    ///
    pub fn exorcise_index(&mut self, id: &Uuid) -> Option<Rc<RefCell<Index>>> {
        self.index
            .borrow_mut()
            .remove(id)
            .map(|index| index.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Index>`.
    ///
    pub fn iter_index(&self) -> impl Iterator<Item = Rc<RefCell<Index>>> + '_ {
        let values: Vec<Rc<RefCell<Index>>> = self
            .index
            .borrow()
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
            .borrow()
            .get(&index.id)
            .map(|index| index.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`IntegerLiteral`] into the store.
    ///
    pub fn inter_integer_literal(&mut self, integer_literal: Rc<RefCell<IntegerLiteral>>) {
        let read = integer_literal.borrow();
        self.integer_literal
            .borrow_mut()
            .insert(read.id, (integer_literal.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`IntegerLiteral`] from the store.
    ///
    pub fn exhume_integer_literal(&self, id: &Uuid) -> Option<Rc<RefCell<IntegerLiteral>>> {
        self.integer_literal
            .borrow()
            .get(id)
            .map(|integer_literal| integer_literal.0.clone())
    }

    /// Exorcise (remove) [`IntegerLiteral`] from the store.
    ///
    pub fn exorcise_integer_literal(&mut self, id: &Uuid) -> Option<Rc<RefCell<IntegerLiteral>>> {
        self.integer_literal
            .borrow_mut()
            .remove(id)
            .map(|integer_literal| integer_literal.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, IntegerLiteral>`.
    ///
    pub fn iter_integer_literal(&self) -> impl Iterator<Item = Rc<RefCell<IntegerLiteral>>> + '_ {
        let values: Vec<Rc<RefCell<IntegerLiteral>>> = self
            .integer_literal
            .borrow()
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
            .borrow()
            .get(&integer_literal.id)
            .map(|integer_literal| integer_literal.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Item`] into the store.
    ///
    pub fn inter_item(&mut self, item: Rc<RefCell<Item>>) {
        let read = item.borrow();
        self.item
            .borrow_mut()
            .insert(read.id, (item.clone(), SystemTime::now()));
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
            .get(&item.id)
            .map(|item| item.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`LetStatement`] into the store.
    ///
    pub fn inter_let_statement(&mut self, let_statement: Rc<RefCell<LetStatement>>) {
        let read = let_statement.borrow();
        self.let_statement
            .borrow_mut()
            .insert(read.id, (let_statement.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`LetStatement`] from the store.
    ///
    pub fn exhume_let_statement(&self, id: &Uuid) -> Option<Rc<RefCell<LetStatement>>> {
        self.let_statement
            .borrow()
            .get(id)
            .map(|let_statement| let_statement.0.clone())
    }

    /// Exorcise (remove) [`LetStatement`] from the store.
    ///
    pub fn exorcise_let_statement(&mut self, id: &Uuid) -> Option<Rc<RefCell<LetStatement>>> {
        self.let_statement
            .borrow_mut()
            .remove(id)
            .map(|let_statement| let_statement.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LetStatement>`.
    ///
    pub fn iter_let_statement(&self) -> impl Iterator<Item = Rc<RefCell<LetStatement>>> + '_ {
        let values: Vec<Rc<RefCell<LetStatement>>> = self
            .let_statement
            .borrow()
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
            .borrow()
            .get(&let_statement.id)
            .map(|let_statement| let_statement.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`List`] into the store.
    ///
    pub fn inter_list(&mut self, list: Rc<RefCell<List>>) {
        let read = list.borrow();
        self.list
            .borrow_mut()
            .insert(read.id, (list.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`List`] from the store.
    ///
    pub fn exhume_list(&self, id: &Uuid) -> Option<Rc<RefCell<List>>> {
        self.list.borrow().get(id).map(|list| list.0.clone())
    }

    /// Exorcise (remove) [`List`] from the store.
    ///
    pub fn exorcise_list(&mut self, id: &Uuid) -> Option<Rc<RefCell<List>>> {
        self.list.borrow_mut().remove(id).map(|list| list.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, List>`.
    ///
    pub fn iter_list(&self) -> impl Iterator<Item = Rc<RefCell<List>>> + '_ {
        let values: Vec<Rc<RefCell<List>>> = self
            .list
            .borrow()
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
            .borrow()
            .get(&list.id)
            .map(|list| list.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ListElement`] into the store.
    ///
    pub fn inter_list_element(&mut self, list_element: Rc<RefCell<ListElement>>) {
        let read = list_element.borrow();
        self.list_element
            .borrow_mut()
            .insert(read.id, (list_element.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ListElement`] from the store.
    ///
    pub fn exhume_list_element(&self, id: &Uuid) -> Option<Rc<RefCell<ListElement>>> {
        self.list_element
            .borrow()
            .get(id)
            .map(|list_element| list_element.0.clone())
    }

    /// Exorcise (remove) [`ListElement`] from the store.
    ///
    pub fn exorcise_list_element(&mut self, id: &Uuid) -> Option<Rc<RefCell<ListElement>>> {
        self.list_element
            .borrow_mut()
            .remove(id)
            .map(|list_element| list_element.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListElement>`.
    ///
    pub fn iter_list_element(&self) -> impl Iterator<Item = Rc<RefCell<ListElement>>> + '_ {
        let values: Vec<Rc<RefCell<ListElement>>> = self
            .list_element
            .borrow()
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
            .borrow()
            .get(&list_element.id)
            .map(|list_element| list_element.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ListExpression`] into the store.
    ///
    pub fn inter_list_expression(&mut self, list_expression: Rc<RefCell<ListExpression>>) {
        let read = list_expression.borrow();
        self.list_expression
            .borrow_mut()
            .insert(read.id, (list_expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ListExpression`] from the store.
    ///
    pub fn exhume_list_expression(&self, id: &Uuid) -> Option<Rc<RefCell<ListExpression>>> {
        self.list_expression
            .borrow()
            .get(id)
            .map(|list_expression| list_expression.0.clone())
    }

    /// Exorcise (remove) [`ListExpression`] from the store.
    ///
    pub fn exorcise_list_expression(&mut self, id: &Uuid) -> Option<Rc<RefCell<ListExpression>>> {
        self.list_expression
            .borrow_mut()
            .remove(id)
            .map(|list_expression| list_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListExpression>`.
    ///
    pub fn iter_list_expression(&self) -> impl Iterator<Item = Rc<RefCell<ListExpression>>> + '_ {
        let values: Vec<Rc<RefCell<ListExpression>>> = self
            .list_expression
            .borrow()
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
            .borrow()
            .get(&list_expression.id)
            .map(|list_expression| list_expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Literal`] into the store.
    ///
    pub fn inter_literal(&mut self, literal: Rc<RefCell<Literal>>) {
        let read = literal.borrow();
        self.literal
            .borrow_mut()
            .insert(read.id(), (literal.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Literal`] from the store.
    ///
    pub fn exhume_literal(&self, id: &Uuid) -> Option<Rc<RefCell<Literal>>> {
        self.literal
            .borrow()
            .get(id)
            .map(|literal| literal.0.clone())
    }

    /// Exorcise (remove) [`Literal`] from the store.
    ///
    pub fn exorcise_literal(&mut self, id: &Uuid) -> Option<Rc<RefCell<Literal>>> {
        self.literal
            .borrow_mut()
            .remove(id)
            .map(|literal| literal.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Literal>`.
    ///
    pub fn iter_literal(&self) -> impl Iterator<Item = Rc<RefCell<Literal>>> + '_ {
        let values: Vec<Rc<RefCell<Literal>>> = self
            .literal
            .borrow()
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
            .borrow()
            .get(&literal.id())
            .map(|literal| literal.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`LocalVariable`] into the store.
    ///
    pub fn inter_local_variable(&mut self, local_variable: Rc<RefCell<LocalVariable>>) {
        let read = local_variable.borrow();
        self.local_variable
            .borrow_mut()
            .insert(read.id, (local_variable.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`LocalVariable`] from the store.
    ///
    pub fn exhume_local_variable(&self, id: &Uuid) -> Option<Rc<RefCell<LocalVariable>>> {
        self.local_variable
            .borrow()
            .get(id)
            .map(|local_variable| local_variable.0.clone())
    }

    /// Exorcise (remove) [`LocalVariable`] from the store.
    ///
    pub fn exorcise_local_variable(&mut self, id: &Uuid) -> Option<Rc<RefCell<LocalVariable>>> {
        self.local_variable
            .borrow_mut()
            .remove(id)
            .map(|local_variable| local_variable.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LocalVariable>`.
    ///
    pub fn iter_local_variable(&self) -> impl Iterator<Item = Rc<RefCell<LocalVariable>>> + '_ {
        let values: Vec<Rc<RefCell<LocalVariable>>> = self
            .local_variable
            .borrow()
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
            .borrow()
            .get(&local_variable.id)
            .map(|local_variable| local_variable.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`XMacro`] into the store.
    ///
    pub fn inter_x_macro(&mut self, x_macro: Rc<RefCell<XMacro>>) {
        let read = x_macro.borrow();
        self.x_macro
            .borrow_mut()
            .insert(read.id, (x_macro.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`XMacro`] from the store.
    ///
    pub fn exhume_x_macro(&self, id: &Uuid) -> Option<Rc<RefCell<XMacro>>> {
        self.x_macro
            .borrow()
            .get(id)
            .map(|x_macro| x_macro.0.clone())
    }

    /// Exorcise (remove) [`XMacro`] from the store.
    ///
    pub fn exorcise_x_macro(&mut self, id: &Uuid) -> Option<Rc<RefCell<XMacro>>> {
        self.x_macro
            .borrow_mut()
            .remove(id)
            .map(|x_macro| x_macro.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XMacro>`.
    ///
    pub fn iter_x_macro(&self) -> impl Iterator<Item = Rc<RefCell<XMacro>>> + '_ {
        let values: Vec<Rc<RefCell<XMacro>>> = self
            .x_macro
            .borrow()
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
            .borrow()
            .get(&x_macro.id)
            .map(|x_macro| x_macro.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`MethodCall`] into the store.
    ///
    pub fn inter_method_call(&mut self, method_call: Rc<RefCell<MethodCall>>) {
        let read = method_call.borrow();
        self.method_call
            .borrow_mut()
            .insert(read.id, (method_call.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`MethodCall`] from the store.
    ///
    pub fn exhume_method_call(&self, id: &Uuid) -> Option<Rc<RefCell<MethodCall>>> {
        self.method_call
            .borrow()
            .get(id)
            .map(|method_call| method_call.0.clone())
    }

    /// Exorcise (remove) [`MethodCall`] from the store.
    ///
    pub fn exorcise_method_call(&mut self, id: &Uuid) -> Option<Rc<RefCell<MethodCall>>> {
        self.method_call
            .borrow_mut()
            .remove(id)
            .map(|method_call| method_call.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, MethodCall>`.
    ///
    pub fn iter_method_call(&self) -> impl Iterator<Item = Rc<RefCell<MethodCall>>> + '_ {
        let values: Vec<Rc<RefCell<MethodCall>>> = self
            .method_call
            .borrow()
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
            .borrow()
            .get(&method_call.id)
            .map(|method_call| method_call.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ZObjectStore`] into the store.
    ///
    pub fn inter_z_object_store(&mut self, z_object_store: Rc<RefCell<ZObjectStore>>) {
        let read = z_object_store.borrow();
        self.z_object_store
            .borrow_mut()
            .insert(read.id, (z_object_store.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ZObjectStore`] from the store.
    ///
    pub fn exhume_z_object_store(&self, id: &Uuid) -> Option<Rc<RefCell<ZObjectStore>>> {
        self.z_object_store
            .borrow()
            .get(id)
            .map(|z_object_store| z_object_store.0.clone())
    }

    /// Exorcise (remove) [`ZObjectStore`] from the store.
    ///
    pub fn exorcise_z_object_store(&mut self, id: &Uuid) -> Option<Rc<RefCell<ZObjectStore>>> {
        self.z_object_store
            .borrow_mut()
            .remove(id)
            .map(|z_object_store| z_object_store.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ZObjectStore>`.
    ///
    pub fn iter_z_object_store(&self) -> impl Iterator<Item = Rc<RefCell<ZObjectStore>>> + '_ {
        let values: Vec<Rc<RefCell<ZObjectStore>>> = self
            .z_object_store
            .borrow()
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
            .borrow()
            .get(&z_object_store.id)
            .map(|z_object_store| z_object_store.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Operator`] into the store.
    ///
    pub fn inter_operator(&mut self, operator: Rc<RefCell<Operator>>) {
        let read = operator.borrow();
        self.operator
            .borrow_mut()
            .insert(read.id, (operator.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Operator`] from the store.
    ///
    pub fn exhume_operator(&self, id: &Uuid) -> Option<Rc<RefCell<Operator>>> {
        self.operator
            .borrow()
            .get(id)
            .map(|operator| operator.0.clone())
    }

    /// Exorcise (remove) [`Operator`] from the store.
    ///
    pub fn exorcise_operator(&mut self, id: &Uuid) -> Option<Rc<RefCell<Operator>>> {
        self.operator
            .borrow_mut()
            .remove(id)
            .map(|operator| operator.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Operator>`.
    ///
    pub fn iter_operator(&self) -> impl Iterator<Item = Rc<RefCell<Operator>>> + '_ {
        let values: Vec<Rc<RefCell<Operator>>> = self
            .operator
            .borrow()
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
            .borrow()
            .get(&operator.id)
            .map(|operator| operator.1)
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

    /// Inter (insert) [`Print`] into the store.
    ///
    pub fn inter_print(&mut self, print: Rc<RefCell<Print>>) {
        let read = print.borrow();
        self.print
            .borrow_mut()
            .insert(read.id, (print.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Print`] from the store.
    ///
    pub fn exhume_print(&self, id: &Uuid) -> Option<Rc<RefCell<Print>>> {
        self.print.borrow().get(id).map(|print| print.0.clone())
    }

    /// Exorcise (remove) [`Print`] from the store.
    ///
    pub fn exorcise_print(&mut self, id: &Uuid) -> Option<Rc<RefCell<Print>>> {
        self.print
            .borrow_mut()
            .remove(id)
            .map(|print| print.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Print>`.
    ///
    pub fn iter_print(&self) -> impl Iterator<Item = Rc<RefCell<Print>>> + '_ {
        let values: Vec<Rc<RefCell<Print>>> = self
            .print
            .borrow()
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
            .borrow()
            .get(&print.id)
            .map(|print| print.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`RangeExpression`] into the store.
    ///
    pub fn inter_range_expression(&mut self, range_expression: Rc<RefCell<RangeExpression>>) {
        let read = range_expression.borrow();
        self.range_expression
            .borrow_mut()
            .insert(read.id, (range_expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`RangeExpression`] from the store.
    ///
    pub fn exhume_range_expression(&self, id: &Uuid) -> Option<Rc<RefCell<RangeExpression>>> {
        self.range_expression
            .borrow()
            .get(id)
            .map(|range_expression| range_expression.0.clone())
    }

    /// Exorcise (remove) [`RangeExpression`] from the store.
    ///
    pub fn exorcise_range_expression(&mut self, id: &Uuid) -> Option<Rc<RefCell<RangeExpression>>> {
        self.range_expression
            .borrow_mut()
            .remove(id)
            .map(|range_expression| range_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RangeExpression>`.
    ///
    pub fn iter_range_expression(&self) -> impl Iterator<Item = Rc<RefCell<RangeExpression>>> + '_ {
        let values: Vec<Rc<RefCell<RangeExpression>>> = self
            .range_expression
            .borrow()
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
            .borrow()
            .get(&range_expression.id)
            .map(|range_expression| range_expression.1)
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

    /// Inter (insert) [`ResultStatement`] into the store.
    ///
    pub fn inter_result_statement(&mut self, result_statement: Rc<RefCell<ResultStatement>>) {
        let read = result_statement.borrow();
        self.result_statement
            .borrow_mut()
            .insert(read.id, (result_statement.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ResultStatement`] from the store.
    ///
    pub fn exhume_result_statement(&self, id: &Uuid) -> Option<Rc<RefCell<ResultStatement>>> {
        self.result_statement
            .borrow()
            .get(id)
            .map(|result_statement| result_statement.0.clone())
    }

    /// Exorcise (remove) [`ResultStatement`] from the store.
    ///
    pub fn exorcise_result_statement(&mut self, id: &Uuid) -> Option<Rc<RefCell<ResultStatement>>> {
        self.result_statement
            .borrow_mut()
            .remove(id)
            .map(|result_statement| result_statement.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ResultStatement>`.
    ///
    pub fn iter_result_statement(&self) -> impl Iterator<Item = Rc<RefCell<ResultStatement>>> + '_ {
        let values: Vec<Rc<RefCell<ResultStatement>>> = self
            .result_statement
            .borrow()
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
            .borrow()
            .get(&result_statement.id)
            .map(|result_statement| result_statement.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`XReturn`] into the store.
    ///
    pub fn inter_x_return(&mut self, x_return: Rc<RefCell<XReturn>>) {
        let read = x_return.borrow();
        self.x_return
            .borrow_mut()
            .insert(read.id, (x_return.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`XReturn`] from the store.
    ///
    pub fn exhume_x_return(&self, id: &Uuid) -> Option<Rc<RefCell<XReturn>>> {
        self.x_return
            .borrow()
            .get(id)
            .map(|x_return| x_return.0.clone())
    }

    /// Exorcise (remove) [`XReturn`] from the store.
    ///
    pub fn exorcise_x_return(&mut self, id: &Uuid) -> Option<Rc<RefCell<XReturn>>> {
        self.x_return
            .borrow_mut()
            .remove(id)
            .map(|x_return| x_return.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XReturn>`.
    ///
    pub fn iter_x_return(&self) -> impl Iterator<Item = Rc<RefCell<XReturn>>> + '_ {
        let values: Vec<Rc<RefCell<XReturn>>> = self
            .x_return
            .borrow()
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
            .borrow()
            .get(&x_return.id)
            .map(|x_return| x_return.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ZSome`] into the store.
    ///
    pub fn inter_z_some(&mut self, z_some: Rc<RefCell<ZSome>>) {
        let read = z_some.borrow();
        self.z_some
            .borrow_mut()
            .insert(read.id, (z_some.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ZSome`] from the store.
    ///
    pub fn exhume_z_some(&self, id: &Uuid) -> Option<Rc<RefCell<ZSome>>> {
        self.z_some.borrow().get(id).map(|z_some| z_some.0.clone())
    }

    /// Exorcise (remove) [`ZSome`] from the store.
    ///
    pub fn exorcise_z_some(&mut self, id: &Uuid) -> Option<Rc<RefCell<ZSome>>> {
        self.z_some
            .borrow_mut()
            .remove(id)
            .map(|z_some| z_some.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ZSome>`.
    ///
    pub fn iter_z_some(&self) -> impl Iterator<Item = Rc<RefCell<ZSome>>> + '_ {
        let values: Vec<Rc<RefCell<ZSome>>> = self
            .z_some
            .borrow()
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
            .borrow()
            .get(&z_some.id)
            .map(|z_some| z_some.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Span`] into the store.
    ///
    pub fn inter_span(&mut self, span: Rc<RefCell<Span>>) {
        let read = span.borrow();
        self.span
            .borrow_mut()
            .insert(read.id, (span.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Span`] from the store.
    ///
    pub fn exhume_span(&self, id: &Uuid) -> Option<Rc<RefCell<Span>>> {
        self.span.borrow().get(id).map(|span| span.0.clone())
    }

    /// Exorcise (remove) [`Span`] from the store.
    ///
    pub fn exorcise_span(&mut self, id: &Uuid) -> Option<Rc<RefCell<Span>>> {
        self.span.borrow_mut().remove(id).map(|span| span.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Span>`.
    ///
    pub fn iter_span(&self) -> impl Iterator<Item = Rc<RefCell<Span>>> + '_ {
        let values: Vec<Rc<RefCell<Span>>> = self
            .span
            .borrow()
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
            .borrow()
            .get(&span.id)
            .map(|span| span.1)
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

    /// Inter (insert) [`StaticMethodCall`] into the store.
    ///
    pub fn inter_static_method_call(&mut self, static_method_call: Rc<RefCell<StaticMethodCall>>) {
        let read = static_method_call.borrow();
        self.static_method_call
            .borrow_mut()
            .insert(read.id, (static_method_call.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`StaticMethodCall`] from the store.
    ///
    pub fn exhume_static_method_call(&self, id: &Uuid) -> Option<Rc<RefCell<StaticMethodCall>>> {
        self.static_method_call
            .borrow()
            .get(id)
            .map(|static_method_call| static_method_call.0.clone())
    }

    /// Exorcise (remove) [`StaticMethodCall`] from the store.
    ///
    pub fn exorcise_static_method_call(
        &mut self,
        id: &Uuid,
    ) -> Option<Rc<RefCell<StaticMethodCall>>> {
        self.static_method_call
            .borrow_mut()
            .remove(id)
            .map(|static_method_call| static_method_call.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StaticMethodCall>`.
    ///
    pub fn iter_static_method_call(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<StaticMethodCall>>> + '_ {
        let values: Vec<Rc<RefCell<StaticMethodCall>>> = self
            .static_method_call
            .borrow()
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
            .borrow()
            .get(&static_method_call.id)
            .map(|static_method_call| static_method_call.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`StringLiteral`] into the store.
    ///
    pub fn inter_string_literal(&mut self, string_literal: Rc<RefCell<StringLiteral>>) {
        let read = string_literal.borrow();
        self.string_literal
            .borrow_mut()
            .insert(read.id, (string_literal.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`StringLiteral`] from the store.
    ///
    pub fn exhume_string_literal(&self, id: &Uuid) -> Option<Rc<RefCell<StringLiteral>>> {
        self.string_literal
            .borrow()
            .get(id)
            .map(|string_literal| string_literal.0.clone())
    }

    /// Exorcise (remove) [`StringLiteral`] from the store.
    ///
    pub fn exorcise_string_literal(&mut self, id: &Uuid) -> Option<Rc<RefCell<StringLiteral>>> {
        self.string_literal
            .borrow_mut()
            .remove(id)
            .map(|string_literal| string_literal.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StringLiteral>`.
    ///
    pub fn iter_string_literal(&self) -> impl Iterator<Item = Rc<RefCell<StringLiteral>>> + '_ {
        let values: Vec<Rc<RefCell<StringLiteral>>> = self
            .string_literal
            .borrow()
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
            .borrow()
            .get(&string_literal.id)
            .map(|string_literal| string_literal.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`WoogStruct`] into the store.
    ///
    pub fn inter_woog_struct(&mut self, woog_struct: Rc<RefCell<WoogStruct>>) {
        let read = woog_struct.borrow();
        let value = (woog_struct.clone(), SystemTime::now());
        self.woog_struct_id_by_name
            .borrow_mut()
            .insert(read.name.to_upper_camel_case(), (read.id, value.1));
        self.woog_struct.borrow_mut().insert(read.id, value);
    }

    /// Exhume (get) [`WoogStruct`] from the store.
    ///
    pub fn exhume_woog_struct(&self, id: &Uuid) -> Option<Rc<RefCell<WoogStruct>>> {
        self.woog_struct
            .borrow()
            .get(id)
            .map(|woog_struct| woog_struct.0.clone())
    }

    /// Exorcise (remove) [`WoogStruct`] from the store.
    ///
    pub fn exorcise_woog_struct(&mut self, id: &Uuid) -> Option<Rc<RefCell<WoogStruct>>> {
        self.woog_struct
            .borrow_mut()
            .remove(id)
            .map(|woog_struct| woog_struct.0.clone())
    }

    /// Exhume [`WoogStruct`] id from the store by name.
    ///
    pub fn exhume_woog_struct_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.woog_struct_id_by_name
            .borrow()
            .get(name)
            .map(|woog_struct| woog_struct.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, WoogStruct>`.
    ///
    pub fn iter_woog_struct(&self) -> impl Iterator<Item = Rc<RefCell<WoogStruct>>> + '_ {
        let values: Vec<Rc<RefCell<WoogStruct>>> = self
            .woog_struct
            .borrow()
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
            .borrow()
            .get(&woog_struct.id)
            .map(|woog_struct| woog_struct.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`StructExpression`] into the store.
    ///
    pub fn inter_struct_expression(&mut self, struct_expression: Rc<RefCell<StructExpression>>) {
        let read = struct_expression.borrow();
        self.struct_expression
            .borrow_mut()
            .insert(read.id, (struct_expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`StructExpression`] from the store.
    ///
    pub fn exhume_struct_expression(&self, id: &Uuid) -> Option<Rc<RefCell<StructExpression>>> {
        self.struct_expression
            .borrow()
            .get(id)
            .map(|struct_expression| struct_expression.0.clone())
    }

    /// Exorcise (remove) [`StructExpression`] from the store.
    ///
    pub fn exorcise_struct_expression(
        &mut self,
        id: &Uuid,
    ) -> Option<Rc<RefCell<StructExpression>>> {
        self.struct_expression
            .borrow_mut()
            .remove(id)
            .map(|struct_expression| struct_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructExpression>`.
    ///
    pub fn iter_struct_expression(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<StructExpression>>> + '_ {
        let values: Vec<Rc<RefCell<StructExpression>>> = self
            .struct_expression
            .borrow()
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
            .borrow()
            .get(&struct_expression.id)
            .map(|struct_expression| struct_expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`TypeCast`] into the store.
    ///
    pub fn inter_type_cast(&mut self, type_cast: Rc<RefCell<TypeCast>>) {
        let read = type_cast.borrow();
        self.type_cast
            .borrow_mut()
            .insert(read.id, (type_cast.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`TypeCast`] from the store.
    ///
    pub fn exhume_type_cast(&self, id: &Uuid) -> Option<Rc<RefCell<TypeCast>>> {
        self.type_cast
            .borrow()
            .get(id)
            .map(|type_cast| type_cast.0.clone())
    }

    /// Exorcise (remove) [`TypeCast`] from the store.
    ///
    pub fn exorcise_type_cast(&mut self, id: &Uuid) -> Option<Rc<RefCell<TypeCast>>> {
        self.type_cast
            .borrow_mut()
            .remove(id)
            .map(|type_cast| type_cast.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, TypeCast>`.
    ///
    pub fn iter_type_cast(&self) -> impl Iterator<Item = Rc<RefCell<TypeCast>>> + '_ {
        let values: Vec<Rc<RefCell<TypeCast>>> = self
            .type_cast
            .borrow()
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
            .borrow()
            .get(&type_cast.id)
            .map(|type_cast| type_cast.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Unary`] into the store.
    ///
    pub fn inter_unary(&mut self, unary: Rc<RefCell<Unary>>) {
        let read = unary.borrow();
        self.unary
            .borrow_mut()
            .insert(read.id(), (unary.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Unary`] from the store.
    ///
    pub fn exhume_unary(&self, id: &Uuid) -> Option<Rc<RefCell<Unary>>> {
        self.unary.borrow().get(id).map(|unary| unary.0.clone())
    }

    /// Exorcise (remove) [`Unary`] from the store.
    ///
    pub fn exorcise_unary(&mut self, id: &Uuid) -> Option<Rc<RefCell<Unary>>> {
        self.unary
            .borrow_mut()
            .remove(id)
            .map(|unary| unary.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Unary>`.
    ///
    pub fn iter_unary(&self) -> impl Iterator<Item = Rc<RefCell<Unary>>> + '_ {
        let values: Vec<Rc<RefCell<Unary>>> = self
            .unary
            .borrow()
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
            .borrow()
            .get(&unary.id())
            .map(|unary| unary.1)
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

    /// Inter (insert) [`ValueType`] into the store.
    ///
    pub fn inter_value_type(&mut self, value_type: Rc<RefCell<ValueType>>) {
        let read = value_type.borrow();
        self.value_type
            .borrow_mut()
            .insert(read.id(), (value_type.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ValueType`] from the store.
    ///
    pub fn exhume_value_type(&self, id: &Uuid) -> Option<Rc<RefCell<ValueType>>> {
        self.value_type
            .borrow()
            .get(id)
            .map(|value_type| value_type.0.clone())
    }

    /// Exorcise (remove) [`ValueType`] from the store.
    ///
    pub fn exorcise_value_type(&mut self, id: &Uuid) -> Option<Rc<RefCell<ValueType>>> {
        self.value_type
            .borrow_mut()
            .remove(id)
            .map(|value_type| value_type.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ValueType>`.
    ///
    pub fn iter_value_type(&self) -> impl Iterator<Item = Rc<RefCell<ValueType>>> + '_ {
        let values: Vec<Rc<RefCell<ValueType>>> = self
            .value_type
            .borrow()
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
            .borrow()
            .get(&value_type.id())
            .map(|value_type| value_type.1)
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

    /// Inter (insert) [`VariableExpression`] into the store.
    ///
    pub fn inter_variable_expression(
        &mut self,
        variable_expression: Rc<RefCell<VariableExpression>>,
    ) {
        let read = variable_expression.borrow();
        self.variable_expression
            .borrow_mut()
            .insert(read.id, (variable_expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`VariableExpression`] from the store.
    ///
    pub fn exhume_variable_expression(&self, id: &Uuid) -> Option<Rc<RefCell<VariableExpression>>> {
        self.variable_expression
            .borrow()
            .get(id)
            .map(|variable_expression| variable_expression.0.clone())
    }

    /// Exorcise (remove) [`VariableExpression`] from the store.
    ///
    pub fn exorcise_variable_expression(
        &mut self,
        id: &Uuid,
    ) -> Option<Rc<RefCell<VariableExpression>>> {
        self.variable_expression
            .borrow_mut()
            .remove(id)
            .map(|variable_expression| variable_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, VariableExpression>`.
    ///
    pub fn iter_variable_expression(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<VariableExpression>>> + '_ {
        let values: Vec<Rc<RefCell<VariableExpression>>> = self
            .variable_expression
            .borrow()
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
            .borrow()
            .get(&variable_expression.id)
            .map(|variable_expression| variable_expression.1)
            .unwrap_or(SystemTime::now())
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog-object-store-persistence"}}}
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
            for argument_tuple in self.argument.borrow().values() {
                let path = path.join(format!("{}.json", argument_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Argument>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != argument_tuple.0.borrow().to_owned() {
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
                    if !self.argument.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Binary.
        {
            let path = path.join("binary");
            fs::create_dir_all(&path)?;
            for binary_tuple in self.binary.borrow().values() {
                let path = path.join(format!("{}.json", binary_tuple.0.borrow().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Binary>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != binary_tuple.0.borrow().to_owned() {
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
                    if !self.binary.borrow().contains_key(&id) {
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

        // Persist Boolean Literal.
        {
            let path = path.join("boolean_literal");
            fs::create_dir_all(&path)?;
            for boolean_literal_tuple in self.boolean_literal.borrow().values() {
                let path = path.join(format!("{}.json", boolean_literal_tuple.0.borrow().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<BooleanLiteral>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != boolean_literal_tuple.0.borrow().to_owned()
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
                    if !self.boolean_literal.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Boolean Operator.
        {
            let path = path.join("boolean_operator");
            fs::create_dir_all(&path)?;
            for boolean_operator_tuple in self.boolean_operator.borrow().values() {
                let path = path.join(format!("{}.json", boolean_operator_tuple.0.borrow().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<BooleanOperator>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != boolean_operator_tuple.0.borrow().to_owned()
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
                    if !self.boolean_operator.borrow().contains_key(&id) {
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

        // Persist Comparison.
        {
            let path = path.join("comparison");
            fs::create_dir_all(&path)?;
            for comparison_tuple in self.comparison.borrow().values() {
                let path = path.join(format!("{}.json", comparison_tuple.0.borrow().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Comparison>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != comparison_tuple.0.borrow().to_owned() {
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
                    if !self.comparison.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Dwarf Source File.
        {
            let path = path.join("dwarf_source_file");
            fs::create_dir_all(&path)?;
            for dwarf_source_file_tuple in self.dwarf_source_file.borrow().values() {
                let path = path.join(format!("{}.json", dwarf_source_file_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<DwarfSourceFile>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned()
                        != dwarf_source_file_tuple.0.borrow().to_owned()
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
                    if !self.dwarf_source_file.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Error.
        {
            let path = path.join("error");
            fs::create_dir_all(&path)?;
            for error_tuple in self.error.borrow().values() {
                let path = path.join(format!("{}.json", error_tuple.0.borrow().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Error>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != error_tuple.0.borrow().to_owned() {
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
                    if !self.error.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Error Expression.
        {
            let path = path.join("error_expression");
            fs::create_dir_all(&path)?;
            for error_expression_tuple in self.error_expression.borrow().values() {
                let path = path.join(format!("{}.json", error_expression_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<ErrorExpression>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != error_expression_tuple.0.borrow().to_owned()
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
                    if !self.error_expression.borrow().contains_key(&id) {
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

        // Persist Expression Statement.
        {
            let path = path.join("expression_statement");
            fs::create_dir_all(&path)?;
            for expression_statement_tuple in self.expression_statement.borrow().values() {
                let path = path.join(format!("{}.json", expression_statement_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<ExpressionStatement>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned()
                        != expression_statement_tuple.0.borrow().to_owned()
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
                    if !self.expression_statement.borrow().contains_key(&id) {
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

        // Persist Field Access.
        {
            let path = path.join("field_access");
            fs::create_dir_all(&path)?;
            for field_access_tuple in self.field_access.borrow().values() {
                let path = path.join(format!("{}.json", field_access_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<FieldAccess>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != field_access_tuple.0.borrow().to_owned() {
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
                    if !self.field_access.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Field Access Target.
        {
            let path = path.join("field_access_target");
            fs::create_dir_all(&path)?;
            for field_access_target_tuple in self.field_access_target.borrow().values() {
                let path = path.join(format!(
                    "{}.json",
                    field_access_target_tuple.0.borrow().id()
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<FieldAccessTarget>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned()
                        != field_access_target_tuple.0.borrow().to_owned()
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
                    if !self.field_access_target.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Field Expression.
        {
            let path = path.join("field_expression");
            fs::create_dir_all(&path)?;
            for field_expression_tuple in self.field_expression.borrow().values() {
                let path = path.join(format!("{}.json", field_expression_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<FieldExpression>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != field_expression_tuple.0.borrow().to_owned()
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
                    if !self.field_expression.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Float Literal.
        {
            let path = path.join("float_literal");
            fs::create_dir_all(&path)?;
            for float_literal_tuple in self.float_literal.borrow().values() {
                let path = path.join(format!("{}.json", float_literal_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<FloatLiteral>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != float_literal_tuple.0.borrow().to_owned() {
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
                    if !self.float_literal.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist For Loop.
        {
            let path = path.join("for_loop");
            fs::create_dir_all(&path)?;
            for for_loop_tuple in self.for_loop.borrow().values() {
                let path = path.join(format!("{}.json", for_loop_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<ForLoop>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != for_loop_tuple.0.borrow().to_owned() {
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
                    if !self.for_loop.borrow().contains_key(&id) {
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

        // Persist Grouped.
        {
            let path = path.join("grouped");
            fs::create_dir_all(&path)?;
            for grouped_tuple in self.grouped.borrow().values() {
                let path = path.join(format!("{}.json", grouped_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Grouped>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != grouped_tuple.0.borrow().to_owned() {
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
                    if !self.grouped.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist If.
        {
            let path = path.join("x_if");
            fs::create_dir_all(&path)?;
            for x_if_tuple in self.x_if.borrow().values() {
                let path = path.join(format!("{}.json", x_if_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<XIf>>, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != x_if_tuple.0.borrow().to_owned() {
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
                    if !self.x_if.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Implementation.
        {
            let path = path.join("implementation");
            fs::create_dir_all(&path)?;
            for implementation_tuple in self.implementation.borrow().values() {
                let path = path.join(format!("{}.json", implementation_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Implementation>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != implementation_tuple.0.borrow().to_owned() {
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
                    if !self.implementation.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Import.
        {
            let path = path.join("import");
            fs::create_dir_all(&path)?;
            for import_tuple in self.import.borrow().values() {
                let path = path.join(format!("{}.json", import_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Import>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != import_tuple.0.borrow().to_owned() {
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
                    if !self.import.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Index.
        {
            let path = path.join("index");
            fs::create_dir_all(&path)?;
            for index_tuple in self.index.borrow().values() {
                let path = path.join(format!("{}.json", index_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Index>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != index_tuple.0.borrow().to_owned() {
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
                    if !self.index.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Integer Literal.
        {
            let path = path.join("integer_literal");
            fs::create_dir_all(&path)?;
            for integer_literal_tuple in self.integer_literal.borrow().values() {
                let path = path.join(format!("{}.json", integer_literal_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<IntegerLiteral>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != integer_literal_tuple.0.borrow().to_owned()
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
                    if !self.integer_literal.borrow().contains_key(&id) {
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
                let path = path.join(format!("{}.json", item_tuple.0.borrow().id));
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

        // Persist Let Statement.
        {
            let path = path.join("let_statement");
            fs::create_dir_all(&path)?;
            for let_statement_tuple in self.let_statement.borrow().values() {
                let path = path.join(format!("{}.json", let_statement_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<LetStatement>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != let_statement_tuple.0.borrow().to_owned() {
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
                    if !self.let_statement.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist List.
        {
            let path = path.join("list");
            fs::create_dir_all(&path)?;
            for list_tuple in self.list.borrow().values() {
                let path = path.join(format!("{}.json", list_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<List>>, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != list_tuple.0.borrow().to_owned() {
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
                    if !self.list.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist List Element.
        {
            let path = path.join("list_element");
            fs::create_dir_all(&path)?;
            for list_element_tuple in self.list_element.borrow().values() {
                let path = path.join(format!("{}.json", list_element_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<ListElement>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != list_element_tuple.0.borrow().to_owned() {
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
                    if !self.list_element.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist List Expression.
        {
            let path = path.join("list_expression");
            fs::create_dir_all(&path)?;
            for list_expression_tuple in self.list_expression.borrow().values() {
                let path = path.join(format!("{}.json", list_expression_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<ListExpression>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != list_expression_tuple.0.borrow().to_owned()
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
                    if !self.list_expression.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Literal.
        {
            let path = path.join("literal");
            fs::create_dir_all(&path)?;
            for literal_tuple in self.literal.borrow().values() {
                let path = path.join(format!("{}.json", literal_tuple.0.borrow().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Literal>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != literal_tuple.0.borrow().to_owned() {
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
                    if !self.literal.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Local Variable.
        {
            let path = path.join("local_variable");
            fs::create_dir_all(&path)?;
            for local_variable_tuple in self.local_variable.borrow().values() {
                let path = path.join(format!("{}.json", local_variable_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<LocalVariable>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != local_variable_tuple.0.borrow().to_owned() {
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
                    if !self.local_variable.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Macro.
        {
            let path = path.join("x_macro");
            fs::create_dir_all(&path)?;
            for x_macro_tuple in self.x_macro.borrow().values() {
                let path = path.join(format!("{}.json", x_macro_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<XMacro>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != x_macro_tuple.0.borrow().to_owned() {
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
                    if !self.x_macro.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Method Call.
        {
            let path = path.join("method_call");
            fs::create_dir_all(&path)?;
            for method_call_tuple in self.method_call.borrow().values() {
                let path = path.join(format!("{}.json", method_call_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<MethodCall>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != method_call_tuple.0.borrow().to_owned() {
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
                    if !self.method_call.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Object Store.
        {
            let path = path.join("z_object_store");
            fs::create_dir_all(&path)?;
            for z_object_store_tuple in self.z_object_store.borrow().values() {
                let path = path.join(format!("{}.json", z_object_store_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<ZObjectStore>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != z_object_store_tuple.0.borrow().to_owned() {
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
                    if !self.z_object_store.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Operator.
        {
            let path = path.join("operator");
            fs::create_dir_all(&path)?;
            for operator_tuple in self.operator.borrow().values() {
                let path = path.join(format!("{}.json", operator_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Operator>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != operator_tuple.0.borrow().to_owned() {
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
                    if !self.operator.borrow().contains_key(&id) {
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

        // Persist Print.
        {
            let path = path.join("print");
            fs::create_dir_all(&path)?;
            for print_tuple in self.print.borrow().values() {
                let path = path.join(format!("{}.json", print_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Print>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != print_tuple.0.borrow().to_owned() {
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
                    if !self.print.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Range Expression.
        {
            let path = path.join("range_expression");
            fs::create_dir_all(&path)?;
            for range_expression_tuple in self.range_expression.borrow().values() {
                let path = path.join(format!("{}.json", range_expression_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<RangeExpression>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != range_expression_tuple.0.borrow().to_owned()
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
                    if !self.range_expression.borrow().contains_key(&id) {
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

        // Persist Result Statement.
        {
            let path = path.join("result_statement");
            fs::create_dir_all(&path)?;
            for result_statement_tuple in self.result_statement.borrow().values() {
                let path = path.join(format!("{}.json", result_statement_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<ResultStatement>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != result_statement_tuple.0.borrow().to_owned()
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
                    if !self.result_statement.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Return.
        {
            let path = path.join("x_return");
            fs::create_dir_all(&path)?;
            for x_return_tuple in self.x_return.borrow().values() {
                let path = path.join(format!("{}.json", x_return_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<XReturn>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != x_return_tuple.0.borrow().to_owned() {
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
                    if !self.x_return.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Some.
        {
            let path = path.join("z_some");
            fs::create_dir_all(&path)?;
            for z_some_tuple in self.z_some.borrow().values() {
                let path = path.join(format!("{}.json", z_some_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<ZSome>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != z_some_tuple.0.borrow().to_owned() {
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
                    if !self.z_some.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Span.
        {
            let path = path.join("span");
            fs::create_dir_all(&path)?;
            for span_tuple in self.span.borrow().values() {
                let path = path.join(format!("{}.json", span_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Span>>, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != span_tuple.0.borrow().to_owned() {
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
                    if !self.span.borrow().contains_key(&id) {
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

        // Persist Static Method Call.
        {
            let path = path.join("static_method_call");
            fs::create_dir_all(&path)?;
            for static_method_call_tuple in self.static_method_call.borrow().values() {
                let path = path.join(format!("{}.json", static_method_call_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<StaticMethodCall>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned()
                        != static_method_call_tuple.0.borrow().to_owned()
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
                    if !self.static_method_call.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist String Literal.
        {
            let path = path.join("string_literal");
            fs::create_dir_all(&path)?;
            for string_literal_tuple in self.string_literal.borrow().values() {
                let path = path.join(format!("{}.json", string_literal_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<StringLiteral>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != string_literal_tuple.0.borrow().to_owned() {
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
                    if !self.string_literal.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Struct.
        {
            let path = path.join("woog_struct");
            fs::create_dir_all(&path)?;
            for woog_struct_tuple in self.woog_struct.borrow().values() {
                let path = path.join(format!("{}.json", woog_struct_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<WoogStruct>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != woog_struct_tuple.0.borrow().to_owned() {
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
                    if !self.woog_struct.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Struct Expression.
        {
            let path = path.join("struct_expression");
            fs::create_dir_all(&path)?;
            for struct_expression_tuple in self.struct_expression.borrow().values() {
                let path = path.join(format!("{}.json", struct_expression_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<StructExpression>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned()
                        != struct_expression_tuple.0.borrow().to_owned()
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
                    if !self.struct_expression.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Type Cast.
        {
            let path = path.join("type_cast");
            fs::create_dir_all(&path)?;
            for type_cast_tuple in self.type_cast.borrow().values() {
                let path = path.join(format!("{}.json", type_cast_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<TypeCast>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != type_cast_tuple.0.borrow().to_owned() {
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
                    if !self.type_cast.borrow().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Unary.
        {
            let path = path.join("unary");
            fs::create_dir_all(&path)?;
            for unary_tuple in self.unary.borrow().values() {
                let path = path.join(format!("{}.json", unary_tuple.0.borrow().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<Unary>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != unary_tuple.0.borrow().to_owned() {
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
                    if !self.unary.borrow().contains_key(&id) {
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

        // Persist Value Type.
        {
            let path = path.join("value_type");
            fs::create_dir_all(&path)?;
            for value_type_tuple in self.value_type.borrow().values() {
                let path = path.join(format!("{}.json", value_type_tuple.0.borrow().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<ValueType>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned() != value_type_tuple.0.borrow().to_owned() {
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
                    if !self.value_type.borrow().contains_key(&id) {
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

        // Persist Variable Expression.
        {
            let path = path.join("variable_expression");
            fs::create_dir_all(&path)?;
            for variable_expression_tuple in self.variable_expression.borrow().values() {
                let path = path.join(format!("{}.json", variable_expression_tuple.0.borrow().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Rc<RefCell<VariableExpression>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.borrow().to_owned()
                        != variable_expression_tuple.0.borrow().to_owned()
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
                    if !self.variable_expression.borrow().contains_key(&id) {
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
                let argument: (Rc<RefCell<Argument>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .argument
                    .borrow_mut()
                    .insert(argument.0.borrow().id, argument.clone());
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
                let binary: (Rc<RefCell<Binary>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .binary
                    .borrow_mut()
                    .insert(binary.0.borrow().id(), binary.clone());
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

        // Load Boolean Literal.
        {
            let path = path.join("boolean_literal");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let boolean_literal: (Rc<RefCell<BooleanLiteral>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .boolean_literal
                    .borrow_mut()
                    .insert(boolean_literal.0.borrow().id(), boolean_literal.clone());
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
                let boolean_operator: (Rc<RefCell<BooleanOperator>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .boolean_operator
                    .borrow_mut()
                    .insert(boolean_operator.0.borrow().id(), boolean_operator.clone());
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

        // Load Comparison.
        {
            let path = path.join("comparison");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let comparison: (Rc<RefCell<Comparison>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .comparison
                    .borrow_mut()
                    .insert(comparison.0.borrow().id(), comparison.clone());
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
                let dwarf_source_file: (Rc<RefCell<DwarfSourceFile>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .dwarf_source_file
                    .borrow_mut()
                    .insert(dwarf_source_file.0.borrow().id, dwarf_source_file.clone());
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
                let error: (Rc<RefCell<Error>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .error
                    .borrow_mut()
                    .insert(error.0.borrow().id(), error.clone());
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
                let error_expression: (Rc<RefCell<ErrorExpression>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .error_expression
                    .borrow_mut()
                    .insert(error_expression.0.borrow().id, error_expression.clone());
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

        // Load Expression Statement.
        {
            let path = path.join("expression_statement");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let expression_statement: (Rc<RefCell<ExpressionStatement>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.expression_statement.borrow_mut().insert(
                    expression_statement.0.borrow().id,
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

        // Load Field Access.
        {
            let path = path.join("field_access");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let field_access: (Rc<RefCell<FieldAccess>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .field_access
                    .borrow_mut()
                    .insert(field_access.0.borrow().id, field_access.clone());
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
                let field_access_target: (Rc<RefCell<FieldAccessTarget>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.field_access_target.borrow_mut().insert(
                    field_access_target.0.borrow().id(),
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
                let field_expression: (Rc<RefCell<FieldExpression>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .field_expression
                    .borrow_mut()
                    .insert(field_expression.0.borrow().id, field_expression.clone());
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
                let float_literal: (Rc<RefCell<FloatLiteral>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .float_literal
                    .borrow_mut()
                    .insert(float_literal.0.borrow().id, float_literal.clone());
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
                let for_loop: (Rc<RefCell<ForLoop>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .for_loop
                    .borrow_mut()
                    .insert(for_loop.0.borrow().id, for_loop.clone());
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

        // Load Grouped.
        {
            let path = path.join("grouped");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let grouped: (Rc<RefCell<Grouped>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .grouped
                    .borrow_mut()
                    .insert(grouped.0.borrow().id, grouped.clone());
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
                let x_if: (Rc<RefCell<XIf>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .x_if
                    .borrow_mut()
                    .insert(x_if.0.borrow().id, x_if.clone());
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
                let implementation: (Rc<RefCell<Implementation>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .implementation
                    .borrow_mut()
                    .insert(implementation.0.borrow().id, implementation.clone());
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
                let import: (Rc<RefCell<Import>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .import
                    .borrow_mut()
                    .insert(import.0.borrow().id, import.clone());
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
                let index: (Rc<RefCell<Index>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .index
                    .borrow_mut()
                    .insert(index.0.borrow().id, index.clone());
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
                let integer_literal: (Rc<RefCell<IntegerLiteral>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .integer_literal
                    .borrow_mut()
                    .insert(integer_literal.0.borrow().id, integer_literal.clone());
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
                    .insert(item.0.borrow().id, item.clone());
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
                let let_statement: (Rc<RefCell<LetStatement>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .let_statement
                    .borrow_mut()
                    .insert(let_statement.0.borrow().id, let_statement.clone());
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
                let list: (Rc<RefCell<List>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .list
                    .borrow_mut()
                    .insert(list.0.borrow().id, list.clone());
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
                let list_element: (Rc<RefCell<ListElement>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .list_element
                    .borrow_mut()
                    .insert(list_element.0.borrow().id, list_element.clone());
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
                let list_expression: (Rc<RefCell<ListExpression>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .list_expression
                    .borrow_mut()
                    .insert(list_expression.0.borrow().id, list_expression.clone());
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
                let literal: (Rc<RefCell<Literal>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .literal
                    .borrow_mut()
                    .insert(literal.0.borrow().id(), literal.clone());
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
                let local_variable: (Rc<RefCell<LocalVariable>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .local_variable
                    .borrow_mut()
                    .insert(local_variable.0.borrow().id, local_variable.clone());
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
                let x_macro: (Rc<RefCell<XMacro>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .x_macro
                    .borrow_mut()
                    .insert(x_macro.0.borrow().id, x_macro.clone());
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
                let method_call: (Rc<RefCell<MethodCall>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .method_call
                    .borrow_mut()
                    .insert(method_call.0.borrow().id, method_call.clone());
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
                let z_object_store: (Rc<RefCell<ZObjectStore>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .z_object_store
                    .borrow_mut()
                    .insert(z_object_store.0.borrow().id, z_object_store.clone());
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
                let operator: (Rc<RefCell<Operator>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .operator
                    .borrow_mut()
                    .insert(operator.0.borrow().id, operator.clone());
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

        // Load Print.
        {
            let path = path.join("print");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let print: (Rc<RefCell<Print>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .print
                    .borrow_mut()
                    .insert(print.0.borrow().id, print.clone());
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
                let range_expression: (Rc<RefCell<RangeExpression>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .range_expression
                    .borrow_mut()
                    .insert(range_expression.0.borrow().id, range_expression.clone());
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

        // Load Result Statement.
        {
            let path = path.join("result_statement");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let result_statement: (Rc<RefCell<ResultStatement>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .result_statement
                    .borrow_mut()
                    .insert(result_statement.0.borrow().id, result_statement.clone());
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
                let x_return: (Rc<RefCell<XReturn>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .x_return
                    .borrow_mut()
                    .insert(x_return.0.borrow().id, x_return.clone());
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
                let z_some: (Rc<RefCell<ZSome>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .z_some
                    .borrow_mut()
                    .insert(z_some.0.borrow().id, z_some.clone());
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
                let span: (Rc<RefCell<Span>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .span
                    .borrow_mut()
                    .insert(span.0.borrow().id, span.clone());
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

        // Load Static Method Call.
        {
            let path = path.join("static_method_call");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let static_method_call: (Rc<RefCell<StaticMethodCall>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .static_method_call
                    .borrow_mut()
                    .insert(static_method_call.0.borrow().id, static_method_call.clone());
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
                let string_literal: (Rc<RefCell<StringLiteral>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .string_literal
                    .borrow_mut()
                    .insert(string_literal.0.borrow().id, string_literal.clone());
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
                let woog_struct: (Rc<RefCell<WoogStruct>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.woog_struct_id_by_name.borrow_mut().insert(
                    woog_struct.0.borrow().name.to_upper_camel_case(),
                    (woog_struct.0.borrow().id, woog_struct.1),
                );
                store
                    .woog_struct
                    .borrow_mut()
                    .insert(woog_struct.0.borrow().id, woog_struct.clone());
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
                let struct_expression: (Rc<RefCell<StructExpression>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .struct_expression
                    .borrow_mut()
                    .insert(struct_expression.0.borrow().id, struct_expression.clone());
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
                let type_cast: (Rc<RefCell<TypeCast>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .type_cast
                    .borrow_mut()
                    .insert(type_cast.0.borrow().id, type_cast.clone());
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
                let unary: (Rc<RefCell<Unary>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .unary
                    .borrow_mut()
                    .insert(unary.0.borrow().id(), unary.clone());
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

        // Load Value Type.
        {
            let path = path.join("value_type");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let value_type: (Rc<RefCell<ValueType>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .value_type
                    .borrow_mut()
                    .insert(value_type.0.borrow().id(), value_type.clone());
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

        // Load Variable Expression.
        {
            let path = path.join("variable_expression");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let variable_expression: (Rc<RefCell<VariableExpression>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.variable_expression.borrow_mut().insert(
                    variable_expression.0.borrow().id,
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
