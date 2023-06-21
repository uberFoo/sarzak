//! v2::lu_dog_async Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_async-object-store-file"}}}
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_async-object-store-definition"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    ser::SerializeMap,
    Deserializer, Serializer,
};
use std::fmt;
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

use crate::v2::lu_dog_async::types::{
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

impl Serialize for ObjectStore {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let argument = (*futures::executor::block_on(async { self.argument.read().await })).clone();
        let mut map = serializer.serialize_map(Some(argument.len()))?;
        for (k, v) in argument {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let binary = (*futures::executor::block_on(async { self.binary.read().await })).clone();
        let mut map = serializer.serialize_map(Some(binary.len()))?;
        for (k, v) in binary {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let block = (*futures::executor::block_on(async { self.block.read().await })).clone();
        let mut map = serializer.serialize_map(Some(block.len()))?;
        for (k, v) in block {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let boolean_literal =
            (*futures::executor::block_on(async { self.boolean_literal.read().await })).clone();
        let mut map = serializer.serialize_map(Some(boolean_literal.len()))?;
        for (k, v) in boolean_literal {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let boolean_operator =
            (*futures::executor::block_on(async { self.boolean_operator.read().await })).clone();
        let mut map = serializer.serialize_map(Some(boolean_operator.len()))?;
        for (k, v) in boolean_operator {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let call = (*futures::executor::block_on(async { self.call.read().await })).clone();
        let mut map = serializer.serialize_map(Some(call.len()))?;
        for (k, v) in call {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let comparison =
            (*futures::executor::block_on(async { self.comparison.read().await })).clone();
        let mut map = serializer.serialize_map(Some(comparison.len()))?;
        for (k, v) in comparison {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let dwarf_source_file =
            (*futures::executor::block_on(async { self.dwarf_source_file.read().await })).clone();
        let mut map = serializer.serialize_map(Some(dwarf_source_file.len()))?;
        for (k, v) in dwarf_source_file {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let error = (*futures::executor::block_on(async { self.error.read().await })).clone();
        let mut map = serializer.serialize_map(Some(error.len()))?;
        for (k, v) in error {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let error_expression =
            (*futures::executor::block_on(async { self.error_expression.read().await })).clone();
        let mut map = serializer.serialize_map(Some(error_expression.len()))?;
        for (k, v) in error_expression {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let expression =
            (*futures::executor::block_on(async { self.expression.read().await })).clone();
        let mut map = serializer.serialize_map(Some(expression.len()))?;
        for (k, v) in expression {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let expression_statement =
            (*futures::executor::block_on(async { self.expression_statement.read().await }))
                .clone();
        let mut map = serializer.serialize_map(Some(expression_statement.len()))?;
        for (k, v) in expression_statement {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let field = (*futures::executor::block_on(async { self.field.read().await })).clone();
        let mut map = serializer.serialize_map(Some(field.len()))?;
        for (k, v) in field {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let field_access =
            (*futures::executor::block_on(async { self.field_access.read().await })).clone();
        let mut map = serializer.serialize_map(Some(field_access.len()))?;
        for (k, v) in field_access {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let field_access_target =
            (*futures::executor::block_on(async { self.field_access_target.read().await })).clone();
        let mut map = serializer.serialize_map(Some(field_access_target.len()))?;
        for (k, v) in field_access_target {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let field_expression =
            (*futures::executor::block_on(async { self.field_expression.read().await })).clone();
        let mut map = serializer.serialize_map(Some(field_expression.len()))?;
        for (k, v) in field_expression {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let float_literal =
            (*futures::executor::block_on(async { self.float_literal.read().await })).clone();
        let mut map = serializer.serialize_map(Some(float_literal.len()))?;
        for (k, v) in float_literal {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let for_loop = (*futures::executor::block_on(async { self.for_loop.read().await })).clone();
        let mut map = serializer.serialize_map(Some(for_loop.len()))?;
        for (k, v) in for_loop {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let function = (*futures::executor::block_on(async { self.function.read().await })).clone();
        let mut map = serializer.serialize_map(Some(function.len()))?;
        for (k, v) in function {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let grouped = (*futures::executor::block_on(async { self.grouped.read().await })).clone();
        let mut map = serializer.serialize_map(Some(grouped.len()))?;
        for (k, v) in grouped {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let x_if = (*futures::executor::block_on(async { self.x_if.read().await })).clone();
        let mut map = serializer.serialize_map(Some(x_if.len()))?;
        for (k, v) in x_if {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let implementation =
            (*futures::executor::block_on(async { self.implementation.read().await })).clone();
        let mut map = serializer.serialize_map(Some(implementation.len()))?;
        for (k, v) in implementation {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let import = (*futures::executor::block_on(async { self.import.read().await })).clone();
        let mut map = serializer.serialize_map(Some(import.len()))?;
        for (k, v) in import {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let index = (*futures::executor::block_on(async { self.index.read().await })).clone();
        let mut map = serializer.serialize_map(Some(index.len()))?;
        for (k, v) in index {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let integer_literal =
            (*futures::executor::block_on(async { self.integer_literal.read().await })).clone();
        let mut map = serializer.serialize_map(Some(integer_literal.len()))?;
        for (k, v) in integer_literal {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let item = (*futures::executor::block_on(async { self.item.read().await })).clone();
        let mut map = serializer.serialize_map(Some(item.len()))?;
        for (k, v) in item {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let let_statement =
            (*futures::executor::block_on(async { self.let_statement.read().await })).clone();
        let mut map = serializer.serialize_map(Some(let_statement.len()))?;
        for (k, v) in let_statement {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let list = (*futures::executor::block_on(async { self.list.read().await })).clone();
        let mut map = serializer.serialize_map(Some(list.len()))?;
        for (k, v) in list {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let list_element =
            (*futures::executor::block_on(async { self.list_element.read().await })).clone();
        let mut map = serializer.serialize_map(Some(list_element.len()))?;
        for (k, v) in list_element {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let list_expression =
            (*futures::executor::block_on(async { self.list_expression.read().await })).clone();
        let mut map = serializer.serialize_map(Some(list_expression.len()))?;
        for (k, v) in list_expression {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let literal = (*futures::executor::block_on(async { self.literal.read().await })).clone();
        let mut map = serializer.serialize_map(Some(literal.len()))?;
        for (k, v) in literal {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let local_variable =
            (*futures::executor::block_on(async { self.local_variable.read().await })).clone();
        let mut map = serializer.serialize_map(Some(local_variable.len()))?;
        for (k, v) in local_variable {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let x_macro = (*futures::executor::block_on(async { self.x_macro.read().await })).clone();
        let mut map = serializer.serialize_map(Some(x_macro.len()))?;
        for (k, v) in x_macro {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let method_call =
            (*futures::executor::block_on(async { self.method_call.read().await })).clone();
        let mut map = serializer.serialize_map(Some(method_call.len()))?;
        for (k, v) in method_call {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let z_object_store =
            (*futures::executor::block_on(async { self.z_object_store.read().await })).clone();
        let mut map = serializer.serialize_map(Some(z_object_store.len()))?;
        for (k, v) in z_object_store {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let operator = (*futures::executor::block_on(async { self.operator.read().await })).clone();
        let mut map = serializer.serialize_map(Some(operator.len()))?;
        for (k, v) in operator {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let woog_option =
            (*futures::executor::block_on(async { self.woog_option.read().await })).clone();
        let mut map = serializer.serialize_map(Some(woog_option.len()))?;
        for (k, v) in woog_option {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let parameter =
            (*futures::executor::block_on(async { self.parameter.read().await })).clone();
        let mut map = serializer.serialize_map(Some(parameter.len()))?;
        for (k, v) in parameter {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let print = (*futures::executor::block_on(async { self.print.read().await })).clone();
        let mut map = serializer.serialize_map(Some(print.len()))?;
        for (k, v) in print {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let range_expression =
            (*futures::executor::block_on(async { self.range_expression.read().await })).clone();
        let mut map = serializer.serialize_map(Some(range_expression.len()))?;
        for (k, v) in range_expression {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let reference =
            (*futures::executor::block_on(async { self.reference.read().await })).clone();
        let mut map = serializer.serialize_map(Some(reference.len()))?;
        for (k, v) in reference {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let result_statement =
            (*futures::executor::block_on(async { self.result_statement.read().await })).clone();
        let mut map = serializer.serialize_map(Some(result_statement.len()))?;
        for (k, v) in result_statement {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let x_return = (*futures::executor::block_on(async { self.x_return.read().await })).clone();
        let mut map = serializer.serialize_map(Some(x_return.len()))?;
        for (k, v) in x_return {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let z_some = (*futures::executor::block_on(async { self.z_some.read().await })).clone();
        let mut map = serializer.serialize_map(Some(z_some.len()))?;
        for (k, v) in z_some {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let span = (*futures::executor::block_on(async { self.span.read().await })).clone();
        let mut map = serializer.serialize_map(Some(span.len()))?;
        for (k, v) in span {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let statement =
            (*futures::executor::block_on(async { self.statement.read().await })).clone();
        let mut map = serializer.serialize_map(Some(statement.len()))?;
        for (k, v) in statement {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let static_method_call =
            (*futures::executor::block_on(async { self.static_method_call.read().await })).clone();
        let mut map = serializer.serialize_map(Some(static_method_call.len()))?;
        for (k, v) in static_method_call {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let string_literal =
            (*futures::executor::block_on(async { self.string_literal.read().await })).clone();
        let mut map = serializer.serialize_map(Some(string_literal.len()))?;
        for (k, v) in string_literal {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let woog_struct =
            (*futures::executor::block_on(async { self.woog_struct.read().await })).clone();
        let mut map = serializer.serialize_map(Some(woog_struct.len()))?;
        for (k, v) in woog_struct {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let struct_expression =
            (*futures::executor::block_on(async { self.struct_expression.read().await })).clone();
        let mut map = serializer.serialize_map(Some(struct_expression.len()))?;
        for (k, v) in struct_expression {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let type_cast =
            (*futures::executor::block_on(async { self.type_cast.read().await })).clone();
        let mut map = serializer.serialize_map(Some(type_cast.len()))?;
        for (k, v) in type_cast {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let unary = (*futures::executor::block_on(async { self.unary.read().await })).clone();
        let mut map = serializer.serialize_map(Some(unary.len()))?;
        for (k, v) in unary {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let x_value = (*futures::executor::block_on(async { self.x_value.read().await })).clone();
        let mut map = serializer.serialize_map(Some(x_value.len()))?;
        for (k, v) in x_value {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let value_type =
            (*futures::executor::block_on(async { self.value_type.read().await })).clone();
        let mut map = serializer.serialize_map(Some(value_type.len()))?;
        for (k, v) in value_type {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let variable = (*futures::executor::block_on(async { self.variable.read().await })).clone();
        let mut map = serializer.serialize_map(Some(variable.len()))?;
        for (k, v) in variable {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        let variable_expression =
            (*futures::executor::block_on(async { self.variable_expression.read().await })).clone();
        let mut map = serializer.serialize_map(Some(variable_expression.len()))?;
        for (k, v) in variable_expression {
            map.serialize_entry(
                &k,
                &(
                    (*futures::executor::block_on(async { v.0.read().await })).clone(),
                    v.1,
                ),
            )?;
        }
        let result = map.end();

        result
    }
}

impl<'de> Deserialize<'de> for ObjectStore {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Argument,
            Binary,
            Block,
            BooleanLiteral,
            BooleanOperator,
            Call,
            Comparison,
            DwarfSourceFile,
            Error,
            ErrorExpression,
            Expression,
            ExpressionStatement,
            Field,
            FieldAccess,
            FieldAccessTarget,
            FieldExpression,
            FloatLiteral,
            ForLoop,
            Function,
            Grouped,
            XIf,
            Implementation,
            Import,
            Index,
            IntegerLiteral,
            Item,
            LetStatement,
            List,
            ListElement,
            ListExpression,
            Literal,
            LocalVariable,
            XMacro,
            MethodCall,
            ZObjectStore,
            Operator,
            WoogOption,
            Parameter,
            Print,
            RangeExpression,
            Reference,
            ResultStatement,
            XReturn,
            ZSome,
            Span,
            Statement,
            StaticMethodCall,
            StringLiteral,
            WoogStruct,
            StructExpression,
            TypeCast,
            Unary,
            XValue,
            ValueType,
            Variable,
            VariableExpression,
        }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;
                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;
                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("field identifier")
                    }
                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "argument" => Ok(Field::Argument),
                            "binary" => Ok(Field::Binary),
                            "block" => Ok(Field::Block),
                            "boolean_literal" => Ok(Field::BooleanLiteral),
                            "boolean_operator" => Ok(Field::BooleanOperator),
                            "call" => Ok(Field::Call),
                            "comparison" => Ok(Field::Comparison),
                            "dwarf_source_file" => Ok(Field::DwarfSourceFile),
                            "error" => Ok(Field::Error),
                            "error_expression" => Ok(Field::ErrorExpression),
                            "expression" => Ok(Field::Expression),
                            "expression_statement" => Ok(Field::ExpressionStatement),
                            "field" => Ok(Field::Field),
                            "field_access" => Ok(Field::FieldAccess),
                            "field_access_target" => Ok(Field::FieldAccessTarget),
                            "field_expression" => Ok(Field::FieldExpression),
                            "float_literal" => Ok(Field::FloatLiteral),
                            "for_loop" => Ok(Field::ForLoop),
                            "function" => Ok(Field::Function),
                            "grouped" => Ok(Field::Grouped),
                            "x_if" => Ok(Field::XIf),
                            "implementation" => Ok(Field::Implementation),
                            "import" => Ok(Field::Import),
                            "index" => Ok(Field::Index),
                            "integer_literal" => Ok(Field::IntegerLiteral),
                            "item" => Ok(Field::Item),
                            "let_statement" => Ok(Field::LetStatement),
                            "list" => Ok(Field::List),
                            "list_element" => Ok(Field::ListElement),
                            "list_expression" => Ok(Field::ListExpression),
                            "literal" => Ok(Field::Literal),
                            "local_variable" => Ok(Field::LocalVariable),
                            "x_macro" => Ok(Field::XMacro),
                            "method_call" => Ok(Field::MethodCall),
                            "z_object_store" => Ok(Field::ZObjectStore),
                            "operator" => Ok(Field::Operator),
                            "woog_option" => Ok(Field::WoogOption),
                            "parameter" => Ok(Field::Parameter),
                            "print" => Ok(Field::Print),
                            "range_expression" => Ok(Field::RangeExpression),
                            "reference" => Ok(Field::Reference),
                            "result_statement" => Ok(Field::ResultStatement),
                            "x_return" => Ok(Field::XReturn),
                            "z_some" => Ok(Field::ZSome),
                            "span" => Ok(Field::Span),
                            "statement" => Ok(Field::Statement),
                            "static_method_call" => Ok(Field::StaticMethodCall),
                            "string_literal" => Ok(Field::StringLiteral),
                            "woog_struct" => Ok(Field::WoogStruct),
                            "struct_expression" => Ok(Field::StructExpression),
                            "type_cast" => Ok(Field::TypeCast),
                            "unary" => Ok(Field::Unary),
                            "x_value" => Ok(Field::XValue),
                            "value_type" => Ok(Field::ValueType),
                            "variable" => Ok(Field::Variable),
                            "variable_expression" => Ok(Field::VariableExpression),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        struct ObjectStoreVisitor;
        impl<'de> Visitor<'de> for ObjectStoreVisitor {
            type Value = ObjectStore;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ObjectStore")
            }
            fn visit_map<A>(self, mut map: A) -> Result<ObjectStore, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut result = ObjectStore::new();
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Argument => result.argument = map.next_value()?,
                        Field::Binary => result.binary = map.next_value()?,
                        Field::Block => result.block = map.next_value()?,
                        Field::BooleanLiteral => result.boolean_literal = map.next_value()?,
                        Field::BooleanOperator => result.boolean_operator = map.next_value()?,
                        Field::Call => result.call = map.next_value()?,
                        Field::Comparison => result.comparison = map.next_value()?,
                        Field::DwarfSourceFile => result.dwarf_source_file = map.next_value()?,
                        Field::Error => result.error = map.next_value()?,
                        Field::ErrorExpression => result.error_expression = map.next_value()?,
                        Field::Expression => result.expression = map.next_value()?,
                        Field::ExpressionStatement => {
                            result.expression_statement = map.next_value()?
                        }
                        Field::Field => result.field = map.next_value()?,
                        Field::FieldAccess => result.field_access = map.next_value()?,
                        Field::FieldAccessTarget => {
                            result.field_access_target = map.next_value()?
                        }
                        Field::FieldExpression => result.field_expression = map.next_value()?,
                        Field::FloatLiteral => result.float_literal = map.next_value()?,
                        Field::ForLoop => result.for_loop = map.next_value()?,
                        Field::Function => result.function = map.next_value()?,
                        Field::Grouped => result.grouped = map.next_value()?,
                        Field::XIf => result.x_if = map.next_value()?,
                        Field::Implementation => result.implementation = map.next_value()?,
                        Field::Import => result.import = map.next_value()?,
                        Field::Index => result.index = map.next_value()?,
                        Field::IntegerLiteral => result.integer_literal = map.next_value()?,
                        Field::Item => result.item = map.next_value()?,
                        Field::LetStatement => result.let_statement = map.next_value()?,
                        Field::List => result.list = map.next_value()?,
                        Field::ListElement => result.list_element = map.next_value()?,
                        Field::ListExpression => result.list_expression = map.next_value()?,
                        Field::Literal => result.literal = map.next_value()?,
                        Field::LocalVariable => result.local_variable = map.next_value()?,
                        Field::XMacro => result.x_macro = map.next_value()?,
                        Field::MethodCall => result.method_call = map.next_value()?,
                        Field::ZObjectStore => result.z_object_store = map.next_value()?,
                        Field::Operator => result.operator = map.next_value()?,
                        Field::WoogOption => result.woog_option = map.next_value()?,
                        Field::Parameter => result.parameter = map.next_value()?,
                        Field::Print => result.print = map.next_value()?,
                        Field::RangeExpression => result.range_expression = map.next_value()?,
                        Field::Reference => result.reference = map.next_value()?,
                        Field::ResultStatement => result.result_statement = map.next_value()?,
                        Field::XReturn => result.x_return = map.next_value()?,
                        Field::ZSome => result.z_some = map.next_value()?,
                        Field::Span => result.span = map.next_value()?,
                        Field::Statement => result.statement = map.next_value()?,
                        Field::StaticMethodCall => result.static_method_call = map.next_value()?,
                        Field::StringLiteral => result.string_literal = map.next_value()?,
                        Field::WoogStruct => result.woog_struct = map.next_value()?,
                        Field::StructExpression => result.struct_expression = map.next_value()?,
                        Field::TypeCast => result.type_cast = map.next_value()?,
                        Field::Unary => result.unary = map.next_value()?,
                        Field::XValue => result.x_value = map.next_value()?,
                        Field::ValueType => result.value_type = map.next_value()?,
                        Field::Variable => result.variable = map.next_value()?,
                        Field::VariableExpression => {
                            result.variable_expression = map.next_value()?
                        }
                    }
                }
                Ok(result)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<ObjectStore, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut result = ObjectStore::new();
                result.argument = Arc::new(RwLock::new(seq.next_element()?))
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                Ok(result)
            }
        }

        struct ArgumentVisitor;
        impl<'de> Visitor<'de> for ArgumentVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Argument>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Argument map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (Argument, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct BinaryVisitor;
        impl<'de> Visitor<'de> for BinaryVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Binary>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Binary map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, (Binary, SystemTime)>()? {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct BlockVisitor;
        impl<'de> Visitor<'de> for BlockVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Block>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Block map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, (Block, SystemTime)>()? {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct BooleanLiteralVisitor;
        impl<'de> Visitor<'de> for BooleanLiteralVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<BooleanLiteral>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("BooleanLiteral map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (BooleanLiteral, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct BooleanOperatorVisitor;
        impl<'de> Visitor<'de> for BooleanOperatorVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<BooleanOperator>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("BooleanOperator map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (BooleanOperator, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct CallVisitor;
        impl<'de> Visitor<'de> for CallVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Call>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Call map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, (Call, SystemTime)>()? {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ComparisonVisitor;
        impl<'de> Visitor<'de> for ComparisonVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Comparison>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Comparison map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (Comparison, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct DwarfSourceFileVisitor;
        impl<'de> Visitor<'de> for DwarfSourceFileVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<DwarfSourceFile>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("DwarfSourceFile map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (DwarfSourceFile, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ErrorVisitor;
        impl<'de> Visitor<'de> for ErrorVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Error>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Error map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, (Error, SystemTime)>()? {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ErrorExpressionVisitor;
        impl<'de> Visitor<'de> for ErrorExpressionVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<ErrorExpression>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ErrorExpression map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (ErrorExpression, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ExpressionVisitor;
        impl<'de> Visitor<'de> for ExpressionVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Expression>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Expression map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (Expression, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ExpressionStatementVisitor;
        impl<'de> Visitor<'de> for ExpressionStatementVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<ExpressionStatement>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ExpressionStatement map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (ExpressionStatement, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct FieldVisitor;
        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Field>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Field map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, (Field, SystemTime)>()? {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct FieldAccessVisitor;
        impl<'de> Visitor<'de> for FieldAccessVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<FieldAccess>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("FieldAccess map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (FieldAccess, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct FieldAccessTargetVisitor;
        impl<'de> Visitor<'de> for FieldAccessTargetVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<FieldAccessTarget>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("FieldAccessTarget map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (FieldAccessTarget, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct FieldExpressionVisitor;
        impl<'de> Visitor<'de> for FieldExpressionVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<FieldExpression>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("FieldExpression map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (FieldExpression, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct FloatLiteralVisitor;
        impl<'de> Visitor<'de> for FloatLiteralVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<FloatLiteral>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("FloatLiteral map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (FloatLiteral, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ForLoopVisitor;
        impl<'de> Visitor<'de> for ForLoopVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<ForLoop>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ForLoop map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, (ForLoop, SystemTime)>()? {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct FunctionVisitor;
        impl<'de> Visitor<'de> for FunctionVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Function>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Function map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (Function, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct GroupedVisitor;
        impl<'de> Visitor<'de> for GroupedVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Grouped>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Grouped map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, (Grouped, SystemTime)>()? {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct XIfVisitor;
        impl<'de> Visitor<'de> for XIfVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<XIf>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("XIf map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, (XIf, SystemTime)>()? {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ImplementationVisitor;
        impl<'de> Visitor<'de> for ImplementationVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Implementation>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Implementation map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (Implementation, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ImportVisitor;
        impl<'de> Visitor<'de> for ImportVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Import>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Import map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, (Import, SystemTime)>()? {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct IndexVisitor;
        impl<'de> Visitor<'de> for IndexVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Index>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Index map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, (Index, SystemTime)>()? {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct IntegerLiteralVisitor;
        impl<'de> Visitor<'de> for IntegerLiteralVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<IntegerLiteral>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("IntegerLiteral map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (IntegerLiteral, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ItemVisitor;
        impl<'de> Visitor<'de> for ItemVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Item>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Item map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, (Item, SystemTime)>()? {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct LetStatementVisitor;
        impl<'de> Visitor<'de> for LetStatementVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<LetStatement>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("LetStatement map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (LetStatement, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ListVisitor;
        impl<'de> Visitor<'de> for ListVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<List>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("List map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, (List, SystemTime)>()? {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ListElementVisitor;
        impl<'de> Visitor<'de> for ListElementVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<ListElement>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ListElement map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (ListElement, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ListExpressionVisitor;
        impl<'de> Visitor<'de> for ListExpressionVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<ListExpression>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ListExpression map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (ListExpression, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct LiteralVisitor;
        impl<'de> Visitor<'de> for LiteralVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Literal>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Literal map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, (Literal, SystemTime)>()? {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct LocalVariableVisitor;
        impl<'de> Visitor<'de> for LocalVariableVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<LocalVariable>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("LocalVariable map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (LocalVariable, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct XMacroVisitor;
        impl<'de> Visitor<'de> for XMacroVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<XMacro>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("XMacro map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, (XMacro, SystemTime)>()? {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct MethodCallVisitor;
        impl<'de> Visitor<'de> for MethodCallVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<MethodCall>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("MethodCall map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (MethodCall, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ZObjectStoreVisitor;
        impl<'de> Visitor<'de> for ZObjectStoreVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<ZObjectStore>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ZObjectStore map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (ZObjectStore, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct OperatorVisitor;
        impl<'de> Visitor<'de> for OperatorVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Operator>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Operator map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (Operator, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct WoogOptionVisitor;
        impl<'de> Visitor<'de> for WoogOptionVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<WoogOption>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("WoogOption map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (WoogOption, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ParameterVisitor;
        impl<'de> Visitor<'de> for ParameterVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Parameter>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Parameter map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (Parameter, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct PrintVisitor;
        impl<'de> Visitor<'de> for PrintVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Print>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Print map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, (Print, SystemTime)>()? {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct RangeExpressionVisitor;
        impl<'de> Visitor<'de> for RangeExpressionVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<RangeExpression>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("RangeExpression map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (RangeExpression, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ReferenceVisitor;
        impl<'de> Visitor<'de> for ReferenceVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Reference>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Reference map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (Reference, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ResultStatementVisitor;
        impl<'de> Visitor<'de> for ResultStatementVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<ResultStatement>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ResultStatement map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (ResultStatement, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct XReturnVisitor;
        impl<'de> Visitor<'de> for XReturnVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<XReturn>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("XReturn map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, (XReturn, SystemTime)>()? {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ZSomeVisitor;
        impl<'de> Visitor<'de> for ZSomeVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<ZSome>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ZSome map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, (ZSome, SystemTime)>()? {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct SpanVisitor;
        impl<'de> Visitor<'de> for SpanVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Span>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Span map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, (Span, SystemTime)>()? {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct StatementVisitor;
        impl<'de> Visitor<'de> for StatementVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Statement>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Statement map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (Statement, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct StaticMethodCallVisitor;
        impl<'de> Visitor<'de> for StaticMethodCallVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<StaticMethodCall>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("StaticMethodCall map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (StaticMethodCall, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct StringLiteralVisitor;
        impl<'de> Visitor<'de> for StringLiteralVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<StringLiteral>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("StringLiteral map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (StringLiteral, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct WoogStructVisitor;
        impl<'de> Visitor<'de> for WoogStructVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<WoogStruct>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("WoogStruct map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (WoogStruct, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct StructExpressionVisitor;
        impl<'de> Visitor<'de> for StructExpressionVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<StructExpression>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("StructExpression map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (StructExpression, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct TypeCastVisitor;
        impl<'de> Visitor<'de> for TypeCastVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<TypeCast>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("TypeCast map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (TypeCast, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct UnaryVisitor;
        impl<'de> Visitor<'de> for UnaryVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Unary>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Unary map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, (Unary, SystemTime)>()? {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct XValueVisitor;
        impl<'de> Visitor<'de> for XValueVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<XValue>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("XValue map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, (XValue, SystemTime)>()? {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ValueTypeVisitor;
        impl<'de> Visitor<'de> for ValueTypeVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<ValueType>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ValueType map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (ValueType, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct VariableVisitor;
        impl<'de> Visitor<'de> for VariableVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<Variable>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Variable map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (Variable, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct VariableExpressionVisitor;
        impl<'de> Visitor<'de> for VariableExpressionVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<VariableExpression>>, SystemTime)>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("VariableExpression map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, (VariableExpression, SystemTime)>()?
                {
                    map.insert(key, (Arc::new(RwLock::new(value.0)), value.1));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        const FIELDS: &'static [&'static str] = &[
            "argument",
            "binary",
            "block",
            "boolean_literal",
            "boolean_operator",
            "call",
            "comparison",
            "dwarf_source_file",
            "error",
            "error_expression",
            "expression",
            "expression_statement",
            "field",
            "field_access",
            "field_access_target",
            "field_expression",
            "float_literal",
            "for_loop",
            "function",
            "grouped",
            "x_if",
            "implementation",
            "import",
            "index",
            "integer_literal",
            "item",
            "let_statement",
            "list",
            "list_element",
            "list_expression",
            "literal",
            "local_variable",
            "x_macro",
            "method_call",
            "z_object_store",
            "operator",
            "woog_option",
            "parameter",
            "print",
            "range_expression",
            "reference",
            "result_statement",
            "x_return",
            "z_some",
            "span",
            "statement",
            "static_method_call",
            "string_literal",
            "woog_struct",
            "struct_expression",
            "type_cast",
            "unary",
            "x_value",
            "value_type",
            "variable",
            "variable_expression",
        ];
        deserializer.deserialize_struct("ObjectStore", FIELDS, ObjectStoreVisitor)
    }
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
        store
            .inter_binary(Arc::new(RwLock::new(Binary::Addition(ADDITION))))
            .await;
        store
            .inter_binary(Arc::new(RwLock::new(Binary::Assignment(ASSIGNMENT))))
            .await;
        store
            .inter_binary(Arc::new(RwLock::new(Binary::BooleanOperator(
                BooleanOperator::And(AND).id(),
            ))))
            .await;
        store
            .inter_binary(Arc::new(RwLock::new(Binary::Division(DIVISION))))
            .await;
        store
            .inter_binary(Arc::new(RwLock::new(Binary::Multiplication(
                MULTIPLICATION,
            ))))
            .await;
        store
            .inter_binary(Arc::new(RwLock::new(Binary::Subtraction(SUBTRACTION))))
            .await;
        store
            .inter_boolean_literal(Arc::new(RwLock::new(BooleanLiteral::FalseLiteral(
                FALSE_LITERAL,
            ))))
            .await;
        store
            .inter_boolean_literal(Arc::new(RwLock::new(BooleanLiteral::TrueLiteral(
                TRUE_LITERAL,
            ))))
            .await;
        store
            .inter_boolean_operator(Arc::new(RwLock::new(BooleanOperator::And(AND))))
            .await;
        store
            .inter_comparison(Arc::new(RwLock::new(Comparison::Equal(EQUAL))))
            .await;
        store
            .inter_comparison(Arc::new(RwLock::new(Comparison::GreaterThan(GREATER_THAN))))
            .await;
        store
            .inter_comparison(Arc::new(RwLock::new(Comparison::GreaterThanOrEqual(
                GREATER_THAN_OR_EQUAL,
            ))))
            .await;
        store
            .inter_comparison(Arc::new(RwLock::new(Comparison::LessThanOrEqual(
                LESS_THAN_OR_EQUAL,
            ))))
            .await;
        store
            .inter_comparison(Arc::new(RwLock::new(Comparison::NotEqual(NOT_EQUAL))))
            .await;
        store
            .inter_error(Arc::new(RwLock::new(Error::UnknownVariable(
                UNKNOWN_VARIABLE,
            ))))
            .await;
        store
            .inter_expression(Arc::new(RwLock::new(Expression::Debugger(DEBUGGER))))
            .await;
        store
            .inter_expression(Arc::new(RwLock::new(Expression::Literal(
                Literal::BooleanLiteral(BooleanLiteral::FalseLiteral(FALSE_LITERAL).id()).id(),
            ))))
            .await;
        store
            .inter_expression(Arc::new(RwLock::new(Expression::Literal(
                Literal::BooleanLiteral(BooleanLiteral::TrueLiteral(TRUE_LITERAL).id()).id(),
            ))))
            .await;
        store
            .inter_expression(Arc::new(RwLock::new(Expression::ZNone(Z_NONE))))
            .await;
        store
            .inter_literal(Arc::new(RwLock::new(Literal::BooleanLiteral(
                BooleanLiteral::FalseLiteral(FALSE_LITERAL).id(),
            ))))
            .await;
        store
            .inter_literal(Arc::new(RwLock::new(Literal::BooleanLiteral(
                BooleanLiteral::TrueLiteral(TRUE_LITERAL).id(),
            ))))
            .await;
        store
            .inter_unary(Arc::new(RwLock::new(Unary::Negation(NEGATION))))
            .await;
        store
            .inter_unary(Arc::new(RwLock::new(Unary::Not(NOT))))
            .await;
        store
            .inter_value_type(Arc::new(RwLock::new(ValueType::Empty(EMPTY))))
            .await;
        store
            .inter_value_type(Arc::new(RwLock::new(ValueType::Error(
                Error::UnknownVariable(UNKNOWN_VARIABLE).id(),
            ))))
            .await;
        store
            .inter_value_type(Arc::new(RwLock::new(ValueType::Range(RANGE))))
            .await;
        store
            .inter_value_type(Arc::new(RwLock::new(ValueType::Unknown(UNKNOWN))))
            .await;

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_async-object-store-methods"}}}
    /// Inter (insert) [`Argument`] into the store.
    ///
    pub async fn inter_argument(&mut self, argument: Arc<RwLock<Argument>>) {
        let read = argument.read().await;
        self.argument
            .write()
            .await
            .insert(read.id, (argument.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Argument`] from the store.
    ///
    pub async fn exhume_argument(&self, id: &Uuid) -> Option<Arc<RwLock<Argument>>> {
        self.argument
            .read()
            .await
            .get(id)
            .map(|argument| argument.0.clone())
    }

    /// Exorcise (remove) [`Argument`] from the store.
    ///
    pub async fn exorcise_argument(&mut self, id: &Uuid) -> Option<Arc<RwLock<Argument>>> {
        self.argument
            .write()
            .await
            .remove(id)
            .map(|argument| argument.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Argument>`.
    ///
    pub async fn iter_argument(&self) -> impl Iterator<Item = Arc<RwLock<Argument>>> + '_ {
        let values: Vec<Arc<RwLock<Argument>>> = self
            .argument
            .read()
            .await
            .values()
            .map(|argument| argument.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Argument.
    ///
    pub async fn argument_timestamp(&self, argument: &Argument) -> SystemTime {
        self.argument
            .read()
            .await
            .get(&argument.id)
            .map(|argument| argument.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Binary`] into the store.
    ///
    pub async fn inter_binary(&mut self, binary: Arc<RwLock<Binary>>) {
        let read = binary.read().await;
        self.binary
            .write()
            .await
            .insert(read.id(), (binary.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Binary`] from the store.
    ///
    pub async fn exhume_binary(&self, id: &Uuid) -> Option<Arc<RwLock<Binary>>> {
        self.binary
            .read()
            .await
            .get(id)
            .map(|binary| binary.0.clone())
    }

    /// Exorcise (remove) [`Binary`] from the store.
    ///
    pub async fn exorcise_binary(&mut self, id: &Uuid) -> Option<Arc<RwLock<Binary>>> {
        self.binary
            .write()
            .await
            .remove(id)
            .map(|binary| binary.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Binary>`.
    ///
    pub async fn iter_binary(&self) -> impl Iterator<Item = Arc<RwLock<Binary>>> + '_ {
        let values: Vec<Arc<RwLock<Binary>>> = self
            .binary
            .read()
            .await
            .values()
            .map(|binary| binary.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Binary.
    ///
    pub async fn binary_timestamp(&self, binary: &Binary) -> SystemTime {
        self.binary
            .read()
            .await
            .get(&binary.id())
            .map(|binary| binary.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Block`] into the store.
    ///
    pub async fn inter_block(&mut self, block: Arc<RwLock<Block>>) {
        let read = block.read().await;
        self.block
            .write()
            .await
            .insert(read.id, (block.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Block`] from the store.
    ///
    pub async fn exhume_block(&self, id: &Uuid) -> Option<Arc<RwLock<Block>>> {
        self.block.read().await.get(id).map(|block| block.0.clone())
    }

    /// Exorcise (remove) [`Block`] from the store.
    ///
    pub async fn exorcise_block(&mut self, id: &Uuid) -> Option<Arc<RwLock<Block>>> {
        self.block
            .write()
            .await
            .remove(id)
            .map(|block| block.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Block>`.
    ///
    pub async fn iter_block(&self) -> impl Iterator<Item = Arc<RwLock<Block>>> + '_ {
        let values: Vec<Arc<RwLock<Block>>> = self
            .block
            .read()
            .await
            .values()
            .map(|block| block.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Block.
    ///
    pub async fn block_timestamp(&self, block: &Block) -> SystemTime {
        self.block
            .read()
            .await
            .get(&block.id)
            .map(|block| block.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`BooleanLiteral`] into the store.
    ///
    pub async fn inter_boolean_literal(&mut self, boolean_literal: Arc<RwLock<BooleanLiteral>>) {
        let read = boolean_literal.read().await;
        self.boolean_literal
            .write()
            .await
            .insert(read.id(), (boolean_literal.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`BooleanLiteral`] from the store.
    ///
    pub async fn exhume_boolean_literal(&self, id: &Uuid) -> Option<Arc<RwLock<BooleanLiteral>>> {
        self.boolean_literal
            .read()
            .await
            .get(id)
            .map(|boolean_literal| boolean_literal.0.clone())
    }

    /// Exorcise (remove) [`BooleanLiteral`] from the store.
    ///
    pub async fn exorcise_boolean_literal(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<BooleanLiteral>>> {
        self.boolean_literal
            .write()
            .await
            .remove(id)
            .map(|boolean_literal| boolean_literal.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanLiteral>`.
    ///
    pub async fn iter_boolean_literal(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<BooleanLiteral>>> + '_ {
        let values: Vec<Arc<RwLock<BooleanLiteral>>> = self
            .boolean_literal
            .read()
            .await
            .values()
            .map(|boolean_literal| boolean_literal.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for BooleanLiteral.
    ///
    pub async fn boolean_literal_timestamp(&self, boolean_literal: &BooleanLiteral) -> SystemTime {
        self.boolean_literal
            .read()
            .await
            .get(&boolean_literal.id())
            .map(|boolean_literal| boolean_literal.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`BooleanOperator`] into the store.
    ///
    pub async fn inter_boolean_operator(&mut self, boolean_operator: Arc<RwLock<BooleanOperator>>) {
        let read = boolean_operator.read().await;
        self.boolean_operator
            .write()
            .await
            .insert(read.id(), (boolean_operator.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`BooleanOperator`] from the store.
    ///
    pub async fn exhume_boolean_operator(&self, id: &Uuid) -> Option<Arc<RwLock<BooleanOperator>>> {
        self.boolean_operator
            .read()
            .await
            .get(id)
            .map(|boolean_operator| boolean_operator.0.clone())
    }

    /// Exorcise (remove) [`BooleanOperator`] from the store.
    ///
    pub async fn exorcise_boolean_operator(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<BooleanOperator>>> {
        self.boolean_operator
            .write()
            .await
            .remove(id)
            .map(|boolean_operator| boolean_operator.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanOperator>`.
    ///
    pub async fn iter_boolean_operator(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<BooleanOperator>>> + '_ {
        let values: Vec<Arc<RwLock<BooleanOperator>>> = self
            .boolean_operator
            .read()
            .await
            .values()
            .map(|boolean_operator| boolean_operator.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for BooleanOperator.
    ///
    pub async fn boolean_operator_timestamp(
        &self,
        boolean_operator: &BooleanOperator,
    ) -> SystemTime {
        self.boolean_operator
            .read()
            .await
            .get(&boolean_operator.id())
            .map(|boolean_operator| boolean_operator.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Call`] into the store.
    ///
    pub async fn inter_call(&mut self, call: Arc<RwLock<Call>>) {
        let read = call.read().await;
        self.call
            .write()
            .await
            .insert(read.id, (call.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Call`] from the store.
    ///
    pub async fn exhume_call(&self, id: &Uuid) -> Option<Arc<RwLock<Call>>> {
        self.call.read().await.get(id).map(|call| call.0.clone())
    }

    /// Exorcise (remove) [`Call`] from the store.
    ///
    pub async fn exorcise_call(&mut self, id: &Uuid) -> Option<Arc<RwLock<Call>>> {
        self.call
            .write()
            .await
            .remove(id)
            .map(|call| call.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Call>`.
    ///
    pub async fn iter_call(&self) -> impl Iterator<Item = Arc<RwLock<Call>>> + '_ {
        let values: Vec<Arc<RwLock<Call>>> = self
            .call
            .read()
            .await
            .values()
            .map(|call| call.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Call.
    ///
    pub async fn call_timestamp(&self, call: &Call) -> SystemTime {
        self.call
            .read()
            .await
            .get(&call.id)
            .map(|call| call.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Comparison`] into the store.
    ///
    pub async fn inter_comparison(&mut self, comparison: Arc<RwLock<Comparison>>) {
        let read = comparison.read().await;
        self.comparison
            .write()
            .await
            .insert(read.id(), (comparison.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Comparison`] from the store.
    ///
    pub async fn exhume_comparison(&self, id: &Uuid) -> Option<Arc<RwLock<Comparison>>> {
        self.comparison
            .read()
            .await
            .get(id)
            .map(|comparison| comparison.0.clone())
    }

    /// Exorcise (remove) [`Comparison`] from the store.
    ///
    pub async fn exorcise_comparison(&mut self, id: &Uuid) -> Option<Arc<RwLock<Comparison>>> {
        self.comparison
            .write()
            .await
            .remove(id)
            .map(|comparison| comparison.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Comparison>`.
    ///
    pub async fn iter_comparison(&self) -> impl Iterator<Item = Arc<RwLock<Comparison>>> + '_ {
        let values: Vec<Arc<RwLock<Comparison>>> = self
            .comparison
            .read()
            .await
            .values()
            .map(|comparison| comparison.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Comparison.
    ///
    pub async fn comparison_timestamp(&self, comparison: &Comparison) -> SystemTime {
        self.comparison
            .read()
            .await
            .get(&comparison.id())
            .map(|comparison| comparison.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`DwarfSourceFile`] into the store.
    ///
    pub async fn inter_dwarf_source_file(
        &mut self,
        dwarf_source_file: Arc<RwLock<DwarfSourceFile>>,
    ) {
        let read = dwarf_source_file.read().await;
        self.dwarf_source_file
            .write()
            .await
            .insert(read.id, (dwarf_source_file.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`DwarfSourceFile`] from the store.
    ///
    pub async fn exhume_dwarf_source_file(
        &self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<DwarfSourceFile>>> {
        self.dwarf_source_file
            .read()
            .await
            .get(id)
            .map(|dwarf_source_file| dwarf_source_file.0.clone())
    }

    /// Exorcise (remove) [`DwarfSourceFile`] from the store.
    ///
    pub async fn exorcise_dwarf_source_file(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<DwarfSourceFile>>> {
        self.dwarf_source_file
            .write()
            .await
            .remove(id)
            .map(|dwarf_source_file| dwarf_source_file.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, DwarfSourceFile>`.
    ///
    pub async fn iter_dwarf_source_file(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<DwarfSourceFile>>> + '_ {
        let values: Vec<Arc<RwLock<DwarfSourceFile>>> = self
            .dwarf_source_file
            .read()
            .await
            .values()
            .map(|dwarf_source_file| dwarf_source_file.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for DwarfSourceFile.
    ///
    pub async fn dwarf_source_file_timestamp(
        &self,
        dwarf_source_file: &DwarfSourceFile,
    ) -> SystemTime {
        self.dwarf_source_file
            .read()
            .await
            .get(&dwarf_source_file.id)
            .map(|dwarf_source_file| dwarf_source_file.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Error`] into the store.
    ///
    pub async fn inter_error(&mut self, error: Arc<RwLock<Error>>) {
        let read = error.read().await;
        self.error
            .write()
            .await
            .insert(read.id(), (error.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Error`] from the store.
    ///
    pub async fn exhume_error(&self, id: &Uuid) -> Option<Arc<RwLock<Error>>> {
        self.error.read().await.get(id).map(|error| error.0.clone())
    }

    /// Exorcise (remove) [`Error`] from the store.
    ///
    pub async fn exorcise_error(&mut self, id: &Uuid) -> Option<Arc<RwLock<Error>>> {
        self.error
            .write()
            .await
            .remove(id)
            .map(|error| error.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Error>`.
    ///
    pub async fn iter_error(&self) -> impl Iterator<Item = Arc<RwLock<Error>>> + '_ {
        let values: Vec<Arc<RwLock<Error>>> = self
            .error
            .read()
            .await
            .values()
            .map(|error| error.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Error.
    ///
    pub async fn error_timestamp(&self, error: &Error) -> SystemTime {
        self.error
            .read()
            .await
            .get(&error.id())
            .map(|error| error.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ErrorExpression`] into the store.
    ///
    pub async fn inter_error_expression(&mut self, error_expression: Arc<RwLock<ErrorExpression>>) {
        let read = error_expression.read().await;
        self.error_expression
            .write()
            .await
            .insert(read.id, (error_expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ErrorExpression`] from the store.
    ///
    pub async fn exhume_error_expression(&self, id: &Uuid) -> Option<Arc<RwLock<ErrorExpression>>> {
        self.error_expression
            .read()
            .await
            .get(id)
            .map(|error_expression| error_expression.0.clone())
    }

    /// Exorcise (remove) [`ErrorExpression`] from the store.
    ///
    pub async fn exorcise_error_expression(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<ErrorExpression>>> {
        self.error_expression
            .write()
            .await
            .remove(id)
            .map(|error_expression| error_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ErrorExpression>`.
    ///
    pub async fn iter_error_expression(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<ErrorExpression>>> + '_ {
        let values: Vec<Arc<RwLock<ErrorExpression>>> = self
            .error_expression
            .read()
            .await
            .values()
            .map(|error_expression| error_expression.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for ErrorExpression.
    ///
    pub async fn error_expression_timestamp(
        &self,
        error_expression: &ErrorExpression,
    ) -> SystemTime {
        self.error_expression
            .read()
            .await
            .get(&error_expression.id)
            .map(|error_expression| error_expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Expression`] into the store.
    ///
    pub async fn inter_expression(&mut self, expression: Arc<RwLock<Expression>>) {
        let read = expression.read().await;
        self.expression
            .write()
            .await
            .insert(read.id(), (expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Expression`] from the store.
    ///
    pub async fn exhume_expression(&self, id: &Uuid) -> Option<Arc<RwLock<Expression>>> {
        self.expression
            .read()
            .await
            .get(id)
            .map(|expression| expression.0.clone())
    }

    /// Exorcise (remove) [`Expression`] from the store.
    ///
    pub async fn exorcise_expression(&mut self, id: &Uuid) -> Option<Arc<RwLock<Expression>>> {
        self.expression
            .write()
            .await
            .remove(id)
            .map(|expression| expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Expression>`.
    ///
    pub async fn iter_expression(&self) -> impl Iterator<Item = Arc<RwLock<Expression>>> + '_ {
        let values: Vec<Arc<RwLock<Expression>>> = self
            .expression
            .read()
            .await
            .values()
            .map(|expression| expression.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Expression.
    ///
    pub async fn expression_timestamp(&self, expression: &Expression) -> SystemTime {
        self.expression
            .read()
            .await
            .get(&expression.id())
            .map(|expression| expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ExpressionStatement`] into the store.
    ///
    pub async fn inter_expression_statement(
        &mut self,
        expression_statement: Arc<RwLock<ExpressionStatement>>,
    ) {
        let read = expression_statement.read().await;
        self.expression_statement
            .write()
            .await
            .insert(read.id, (expression_statement.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ExpressionStatement`] from the store.
    ///
    pub async fn exhume_expression_statement(
        &self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<ExpressionStatement>>> {
        self.expression_statement
            .read()
            .await
            .get(id)
            .map(|expression_statement| expression_statement.0.clone())
    }

    /// Exorcise (remove) [`ExpressionStatement`] from the store.
    ///
    pub async fn exorcise_expression_statement(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<ExpressionStatement>>> {
        self.expression_statement
            .write()
            .await
            .remove(id)
            .map(|expression_statement| expression_statement.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ExpressionStatement>`.
    ///
    pub async fn iter_expression_statement(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<ExpressionStatement>>> + '_ {
        let values: Vec<Arc<RwLock<ExpressionStatement>>> = self
            .expression_statement
            .read()
            .await
            .values()
            .map(|expression_statement| expression_statement.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for ExpressionStatement.
    ///
    pub async fn expression_statement_timestamp(
        &self,
        expression_statement: &ExpressionStatement,
    ) -> SystemTime {
        self.expression_statement
            .read()
            .await
            .get(&expression_statement.id)
            .map(|expression_statement| expression_statement.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Field`] into the store.
    ///
    pub async fn inter_field(&mut self, field: Arc<RwLock<Field>>) {
        let read = field.read().await;
        let value = (field.clone(), SystemTime::now());
        self.field_id_by_name
            .write()
            .await
            .insert(read.name.to_upper_camel_case(), (read.id, value.1));
        self.field.write().await.insert(read.id, value);
    }

    /// Exhume (get) [`Field`] from the store.
    ///
    pub async fn exhume_field(&self, id: &Uuid) -> Option<Arc<RwLock<Field>>> {
        self.field.read().await.get(id).map(|field| field.0.clone())
    }

    /// Exorcise (remove) [`Field`] from the store.
    ///
    pub async fn exorcise_field(&mut self, id: &Uuid) -> Option<Arc<RwLock<Field>>> {
        self.field
            .write()
            .await
            .remove(id)
            .map(|field| field.0.clone())
    }

    /// Exhume [`Field`] id from the store by name.
    ///
    pub async fn exhume_field_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.field_id_by_name
            .read()
            .await
            .get(name)
            .map(|field| field.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Field>`.
    ///
    pub async fn iter_field(&self) -> impl Iterator<Item = Arc<RwLock<Field>>> + '_ {
        let values: Vec<Arc<RwLock<Field>>> = self
            .field
            .read()
            .await
            .values()
            .map(|field| field.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Field.
    ///
    pub async fn field_timestamp(&self, field: &Field) -> SystemTime {
        self.field
            .read()
            .await
            .get(&field.id)
            .map(|field| field.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`FieldAccess`] into the store.
    ///
    pub async fn inter_field_access(&mut self, field_access: Arc<RwLock<FieldAccess>>) {
        let read = field_access.read().await;
        self.field_access
            .write()
            .await
            .insert(read.id, (field_access.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`FieldAccess`] from the store.
    ///
    pub async fn exhume_field_access(&self, id: &Uuid) -> Option<Arc<RwLock<FieldAccess>>> {
        self.field_access
            .read()
            .await
            .get(id)
            .map(|field_access| field_access.0.clone())
    }

    /// Exorcise (remove) [`FieldAccess`] from the store.
    ///
    pub async fn exorcise_field_access(&mut self, id: &Uuid) -> Option<Arc<RwLock<FieldAccess>>> {
        self.field_access
            .write()
            .await
            .remove(id)
            .map(|field_access| field_access.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccess>`.
    ///
    pub async fn iter_field_access(&self) -> impl Iterator<Item = Arc<RwLock<FieldAccess>>> + '_ {
        let values: Vec<Arc<RwLock<FieldAccess>>> = self
            .field_access
            .read()
            .await
            .values()
            .map(|field_access| field_access.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for FieldAccess.
    ///
    pub async fn field_access_timestamp(&self, field_access: &FieldAccess) -> SystemTime {
        self.field_access
            .read()
            .await
            .get(&field_access.id)
            .map(|field_access| field_access.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`FieldAccessTarget`] into the store.
    ///
    pub async fn inter_field_access_target(
        &mut self,
        field_access_target: Arc<RwLock<FieldAccessTarget>>,
    ) {
        let read = field_access_target.read().await;
        self.field_access_target
            .write()
            .await
            .insert(read.id(), (field_access_target.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`FieldAccessTarget`] from the store.
    ///
    pub async fn exhume_field_access_target(
        &self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<FieldAccessTarget>>> {
        self.field_access_target
            .read()
            .await
            .get(id)
            .map(|field_access_target| field_access_target.0.clone())
    }

    /// Exorcise (remove) [`FieldAccessTarget`] from the store.
    ///
    pub async fn exorcise_field_access_target(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<FieldAccessTarget>>> {
        self.field_access_target
            .write()
            .await
            .remove(id)
            .map(|field_access_target| field_access_target.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccessTarget>`.
    ///
    pub async fn iter_field_access_target(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<FieldAccessTarget>>> + '_ {
        let values: Vec<Arc<RwLock<FieldAccessTarget>>> = self
            .field_access_target
            .read()
            .await
            .values()
            .map(|field_access_target| field_access_target.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for FieldAccessTarget.
    ///
    pub async fn field_access_target_timestamp(
        &self,
        field_access_target: &FieldAccessTarget,
    ) -> SystemTime {
        self.field_access_target
            .read()
            .await
            .get(&field_access_target.id())
            .map(|field_access_target| field_access_target.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`FieldExpression`] into the store.
    ///
    pub async fn inter_field_expression(&mut self, field_expression: Arc<RwLock<FieldExpression>>) {
        let read = field_expression.read().await;
        self.field_expression
            .write()
            .await
            .insert(read.id, (field_expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`FieldExpression`] from the store.
    ///
    pub async fn exhume_field_expression(&self, id: &Uuid) -> Option<Arc<RwLock<FieldExpression>>> {
        self.field_expression
            .read()
            .await
            .get(id)
            .map(|field_expression| field_expression.0.clone())
    }

    /// Exorcise (remove) [`FieldExpression`] from the store.
    ///
    pub async fn exorcise_field_expression(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<FieldExpression>>> {
        self.field_expression
            .write()
            .await
            .remove(id)
            .map(|field_expression| field_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldExpression>`.
    ///
    pub async fn iter_field_expression(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<FieldExpression>>> + '_ {
        let values: Vec<Arc<RwLock<FieldExpression>>> = self
            .field_expression
            .read()
            .await
            .values()
            .map(|field_expression| field_expression.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for FieldExpression.
    ///
    pub async fn field_expression_timestamp(
        &self,
        field_expression: &FieldExpression,
    ) -> SystemTime {
        self.field_expression
            .read()
            .await
            .get(&field_expression.id)
            .map(|field_expression| field_expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`FloatLiteral`] into the store.
    ///
    pub async fn inter_float_literal(&mut self, float_literal: Arc<RwLock<FloatLiteral>>) {
        let read = float_literal.read().await;
        self.float_literal
            .write()
            .await
            .insert(read.id, (float_literal.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`FloatLiteral`] from the store.
    ///
    pub async fn exhume_float_literal(&self, id: &Uuid) -> Option<Arc<RwLock<FloatLiteral>>> {
        self.float_literal
            .read()
            .await
            .get(id)
            .map(|float_literal| float_literal.0.clone())
    }

    /// Exorcise (remove) [`FloatLiteral`] from the store.
    ///
    pub async fn exorcise_float_literal(&mut self, id: &Uuid) -> Option<Arc<RwLock<FloatLiteral>>> {
        self.float_literal
            .write()
            .await
            .remove(id)
            .map(|float_literal| float_literal.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FloatLiteral>`.
    ///
    pub async fn iter_float_literal(&self) -> impl Iterator<Item = Arc<RwLock<FloatLiteral>>> + '_ {
        let values: Vec<Arc<RwLock<FloatLiteral>>> = self
            .float_literal
            .read()
            .await
            .values()
            .map(|float_literal| float_literal.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for FloatLiteral.
    ///
    pub async fn float_literal_timestamp(&self, float_literal: &FloatLiteral) -> SystemTime {
        self.float_literal
            .read()
            .await
            .get(&float_literal.id)
            .map(|float_literal| float_literal.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ForLoop`] into the store.
    ///
    pub async fn inter_for_loop(&mut self, for_loop: Arc<RwLock<ForLoop>>) {
        let read = for_loop.read().await;
        self.for_loop
            .write()
            .await
            .insert(read.id, (for_loop.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ForLoop`] from the store.
    ///
    pub async fn exhume_for_loop(&self, id: &Uuid) -> Option<Arc<RwLock<ForLoop>>> {
        self.for_loop
            .read()
            .await
            .get(id)
            .map(|for_loop| for_loop.0.clone())
    }

    /// Exorcise (remove) [`ForLoop`] from the store.
    ///
    pub async fn exorcise_for_loop(&mut self, id: &Uuid) -> Option<Arc<RwLock<ForLoop>>> {
        self.for_loop
            .write()
            .await
            .remove(id)
            .map(|for_loop| for_loop.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ForLoop>`.
    ///
    pub async fn iter_for_loop(&self) -> impl Iterator<Item = Arc<RwLock<ForLoop>>> + '_ {
        let values: Vec<Arc<RwLock<ForLoop>>> = self
            .for_loop
            .read()
            .await
            .values()
            .map(|for_loop| for_loop.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for ForLoop.
    ///
    pub async fn for_loop_timestamp(&self, for_loop: &ForLoop) -> SystemTime {
        self.for_loop
            .read()
            .await
            .get(&for_loop.id)
            .map(|for_loop| for_loop.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Function`] into the store.
    ///
    pub async fn inter_function(&mut self, function: Arc<RwLock<Function>>) {
        let read = function.read().await;
        let value = (function.clone(), SystemTime::now());
        self.function_id_by_name
            .write()
            .await
            .insert(read.name.to_upper_camel_case(), (read.id, value.1));
        self.function.write().await.insert(read.id, value);
    }

    /// Exhume (get) [`Function`] from the store.
    ///
    pub async fn exhume_function(&self, id: &Uuid) -> Option<Arc<RwLock<Function>>> {
        self.function
            .read()
            .await
            .get(id)
            .map(|function| function.0.clone())
    }

    /// Exorcise (remove) [`Function`] from the store.
    ///
    pub async fn exorcise_function(&mut self, id: &Uuid) -> Option<Arc<RwLock<Function>>> {
        self.function
            .write()
            .await
            .remove(id)
            .map(|function| function.0.clone())
    }

    /// Exhume [`Function`] id from the store by name.
    ///
    pub async fn exhume_function_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.function_id_by_name
            .read()
            .await
            .get(name)
            .map(|function| function.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Function>`.
    ///
    pub async fn iter_function(&self) -> impl Iterator<Item = Arc<RwLock<Function>>> + '_ {
        let values: Vec<Arc<RwLock<Function>>> = self
            .function
            .read()
            .await
            .values()
            .map(|function| function.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Function.
    ///
    pub async fn function_timestamp(&self, function: &Function) -> SystemTime {
        self.function
            .read()
            .await
            .get(&function.id)
            .map(|function| function.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Grouped`] into the store.
    ///
    pub async fn inter_grouped(&mut self, grouped: Arc<RwLock<Grouped>>) {
        let read = grouped.read().await;
        self.grouped
            .write()
            .await
            .insert(read.id, (grouped.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Grouped`] from the store.
    ///
    pub async fn exhume_grouped(&self, id: &Uuid) -> Option<Arc<RwLock<Grouped>>> {
        self.grouped
            .read()
            .await
            .get(id)
            .map(|grouped| grouped.0.clone())
    }

    /// Exorcise (remove) [`Grouped`] from the store.
    ///
    pub async fn exorcise_grouped(&mut self, id: &Uuid) -> Option<Arc<RwLock<Grouped>>> {
        self.grouped
            .write()
            .await
            .remove(id)
            .map(|grouped| grouped.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Grouped>`.
    ///
    pub async fn iter_grouped(&self) -> impl Iterator<Item = Arc<RwLock<Grouped>>> + '_ {
        let values: Vec<Arc<RwLock<Grouped>>> = self
            .grouped
            .read()
            .await
            .values()
            .map(|grouped| grouped.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Grouped.
    ///
    pub async fn grouped_timestamp(&self, grouped: &Grouped) -> SystemTime {
        self.grouped
            .read()
            .await
            .get(&grouped.id)
            .map(|grouped| grouped.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`XIf`] into the store.
    ///
    pub async fn inter_x_if(&mut self, x_if: Arc<RwLock<XIf>>) {
        let read = x_if.read().await;
        self.x_if
            .write()
            .await
            .insert(read.id, (x_if.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`XIf`] from the store.
    ///
    pub async fn exhume_x_if(&self, id: &Uuid) -> Option<Arc<RwLock<XIf>>> {
        self.x_if.read().await.get(id).map(|x_if| x_if.0.clone())
    }

    /// Exorcise (remove) [`XIf`] from the store.
    ///
    pub async fn exorcise_x_if(&mut self, id: &Uuid) -> Option<Arc<RwLock<XIf>>> {
        self.x_if
            .write()
            .await
            .remove(id)
            .map(|x_if| x_if.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XIf>`.
    ///
    pub async fn iter_x_if(&self) -> impl Iterator<Item = Arc<RwLock<XIf>>> + '_ {
        let values: Vec<Arc<RwLock<XIf>>> = self
            .x_if
            .read()
            .await
            .values()
            .map(|x_if| x_if.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for XIf.
    ///
    pub async fn x_if_timestamp(&self, x_if: &XIf) -> SystemTime {
        self.x_if
            .read()
            .await
            .get(&x_if.id)
            .map(|x_if| x_if.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Implementation`] into the store.
    ///
    pub async fn inter_implementation(&mut self, implementation: Arc<RwLock<Implementation>>) {
        let read = implementation.read().await;
        self.implementation
            .write()
            .await
            .insert(read.id, (implementation.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Implementation`] from the store.
    ///
    pub async fn exhume_implementation(&self, id: &Uuid) -> Option<Arc<RwLock<Implementation>>> {
        self.implementation
            .read()
            .await
            .get(id)
            .map(|implementation| implementation.0.clone())
    }

    /// Exorcise (remove) [`Implementation`] from the store.
    ///
    pub async fn exorcise_implementation(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<Implementation>>> {
        self.implementation
            .write()
            .await
            .remove(id)
            .map(|implementation| implementation.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Implementation>`.
    ///
    pub async fn iter_implementation(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<Implementation>>> + '_ {
        let values: Vec<Arc<RwLock<Implementation>>> = self
            .implementation
            .read()
            .await
            .values()
            .map(|implementation| implementation.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Implementation.
    ///
    pub async fn implementation_timestamp(&self, implementation: &Implementation) -> SystemTime {
        self.implementation
            .read()
            .await
            .get(&implementation.id)
            .map(|implementation| implementation.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Import`] into the store.
    ///
    pub async fn inter_import(&mut self, import: Arc<RwLock<Import>>) {
        let read = import.read().await;
        self.import
            .write()
            .await
            .insert(read.id, (import.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Import`] from the store.
    ///
    pub async fn exhume_import(&self, id: &Uuid) -> Option<Arc<RwLock<Import>>> {
        self.import
            .read()
            .await
            .get(id)
            .map(|import| import.0.clone())
    }

    /// Exorcise (remove) [`Import`] from the store.
    ///
    pub async fn exorcise_import(&mut self, id: &Uuid) -> Option<Arc<RwLock<Import>>> {
        self.import
            .write()
            .await
            .remove(id)
            .map(|import| import.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Import>`.
    ///
    pub async fn iter_import(&self) -> impl Iterator<Item = Arc<RwLock<Import>>> + '_ {
        let values: Vec<Arc<RwLock<Import>>> = self
            .import
            .read()
            .await
            .values()
            .map(|import| import.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Import.
    ///
    pub async fn import_timestamp(&self, import: &Import) -> SystemTime {
        self.import
            .read()
            .await
            .get(&import.id)
            .map(|import| import.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Index`] into the store.
    ///
    pub async fn inter_index(&mut self, index: Arc<RwLock<Index>>) {
        let read = index.read().await;
        self.index
            .write()
            .await
            .insert(read.id, (index.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Index`] from the store.
    ///
    pub async fn exhume_index(&self, id: &Uuid) -> Option<Arc<RwLock<Index>>> {
        self.index.read().await.get(id).map(|index| index.0.clone())
    }

    /// Exorcise (remove) [`Index`] from the store.
    ///
    pub async fn exorcise_index(&mut self, id: &Uuid) -> Option<Arc<RwLock<Index>>> {
        self.index
            .write()
            .await
            .remove(id)
            .map(|index| index.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Index>`.
    ///
    pub async fn iter_index(&self) -> impl Iterator<Item = Arc<RwLock<Index>>> + '_ {
        let values: Vec<Arc<RwLock<Index>>> = self
            .index
            .read()
            .await
            .values()
            .map(|index| index.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Index.
    ///
    pub async fn index_timestamp(&self, index: &Index) -> SystemTime {
        self.index
            .read()
            .await
            .get(&index.id)
            .map(|index| index.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`IntegerLiteral`] into the store.
    ///
    pub async fn inter_integer_literal(&mut self, integer_literal: Arc<RwLock<IntegerLiteral>>) {
        let read = integer_literal.read().await;
        self.integer_literal
            .write()
            .await
            .insert(read.id, (integer_literal.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`IntegerLiteral`] from the store.
    ///
    pub async fn exhume_integer_literal(&self, id: &Uuid) -> Option<Arc<RwLock<IntegerLiteral>>> {
        self.integer_literal
            .read()
            .await
            .get(id)
            .map(|integer_literal| integer_literal.0.clone())
    }

    /// Exorcise (remove) [`IntegerLiteral`] from the store.
    ///
    pub async fn exorcise_integer_literal(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<IntegerLiteral>>> {
        self.integer_literal
            .write()
            .await
            .remove(id)
            .map(|integer_literal| integer_literal.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, IntegerLiteral>`.
    ///
    pub async fn iter_integer_literal(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<IntegerLiteral>>> + '_ {
        let values: Vec<Arc<RwLock<IntegerLiteral>>> = self
            .integer_literal
            .read()
            .await
            .values()
            .map(|integer_literal| integer_literal.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for IntegerLiteral.
    ///
    pub async fn integer_literal_timestamp(&self, integer_literal: &IntegerLiteral) -> SystemTime {
        self.integer_literal
            .read()
            .await
            .get(&integer_literal.id)
            .map(|integer_literal| integer_literal.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Item`] into the store.
    ///
    pub async fn inter_item(&mut self, item: Arc<RwLock<Item>>) {
        let read = item.read().await;
        self.item
            .write()
            .await
            .insert(read.id, (item.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Item`] from the store.
    ///
    pub async fn exhume_item(&self, id: &Uuid) -> Option<Arc<RwLock<Item>>> {
        self.item.read().await.get(id).map(|item| item.0.clone())
    }

    /// Exorcise (remove) [`Item`] from the store.
    ///
    pub async fn exorcise_item(&mut self, id: &Uuid) -> Option<Arc<RwLock<Item>>> {
        self.item
            .write()
            .await
            .remove(id)
            .map(|item| item.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Item>`.
    ///
    pub async fn iter_item(&self) -> impl Iterator<Item = Arc<RwLock<Item>>> + '_ {
        let values: Vec<Arc<RwLock<Item>>> = self
            .item
            .read()
            .await
            .values()
            .map(|item| item.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Item.
    ///
    pub async fn item_timestamp(&self, item: &Item) -> SystemTime {
        self.item
            .read()
            .await
            .get(&item.id)
            .map(|item| item.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`LetStatement`] into the store.
    ///
    pub async fn inter_let_statement(&mut self, let_statement: Arc<RwLock<LetStatement>>) {
        let read = let_statement.read().await;
        self.let_statement
            .write()
            .await
            .insert(read.id, (let_statement.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`LetStatement`] from the store.
    ///
    pub async fn exhume_let_statement(&self, id: &Uuid) -> Option<Arc<RwLock<LetStatement>>> {
        self.let_statement
            .read()
            .await
            .get(id)
            .map(|let_statement| let_statement.0.clone())
    }

    /// Exorcise (remove) [`LetStatement`] from the store.
    ///
    pub async fn exorcise_let_statement(&mut self, id: &Uuid) -> Option<Arc<RwLock<LetStatement>>> {
        self.let_statement
            .write()
            .await
            .remove(id)
            .map(|let_statement| let_statement.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LetStatement>`.
    ///
    pub async fn iter_let_statement(&self) -> impl Iterator<Item = Arc<RwLock<LetStatement>>> + '_ {
        let values: Vec<Arc<RwLock<LetStatement>>> = self
            .let_statement
            .read()
            .await
            .values()
            .map(|let_statement| let_statement.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for LetStatement.
    ///
    pub async fn let_statement_timestamp(&self, let_statement: &LetStatement) -> SystemTime {
        self.let_statement
            .read()
            .await
            .get(&let_statement.id)
            .map(|let_statement| let_statement.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`List`] into the store.
    ///
    pub async fn inter_list(&mut self, list: Arc<RwLock<List>>) {
        let read = list.read().await;
        self.list
            .write()
            .await
            .insert(read.id, (list.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`List`] from the store.
    ///
    pub async fn exhume_list(&self, id: &Uuid) -> Option<Arc<RwLock<List>>> {
        self.list.read().await.get(id).map(|list| list.0.clone())
    }

    /// Exorcise (remove) [`List`] from the store.
    ///
    pub async fn exorcise_list(&mut self, id: &Uuid) -> Option<Arc<RwLock<List>>> {
        self.list
            .write()
            .await
            .remove(id)
            .map(|list| list.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, List>`.
    ///
    pub async fn iter_list(&self) -> impl Iterator<Item = Arc<RwLock<List>>> + '_ {
        let values: Vec<Arc<RwLock<List>>> = self
            .list
            .read()
            .await
            .values()
            .map(|list| list.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for List.
    ///
    pub async fn list_timestamp(&self, list: &List) -> SystemTime {
        self.list
            .read()
            .await
            .get(&list.id)
            .map(|list| list.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ListElement`] into the store.
    ///
    pub async fn inter_list_element(&mut self, list_element: Arc<RwLock<ListElement>>) {
        let read = list_element.read().await;
        self.list_element
            .write()
            .await
            .insert(read.id, (list_element.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ListElement`] from the store.
    ///
    pub async fn exhume_list_element(&self, id: &Uuid) -> Option<Arc<RwLock<ListElement>>> {
        self.list_element
            .read()
            .await
            .get(id)
            .map(|list_element| list_element.0.clone())
    }

    /// Exorcise (remove) [`ListElement`] from the store.
    ///
    pub async fn exorcise_list_element(&mut self, id: &Uuid) -> Option<Arc<RwLock<ListElement>>> {
        self.list_element
            .write()
            .await
            .remove(id)
            .map(|list_element| list_element.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListElement>`.
    ///
    pub async fn iter_list_element(&self) -> impl Iterator<Item = Arc<RwLock<ListElement>>> + '_ {
        let values: Vec<Arc<RwLock<ListElement>>> = self
            .list_element
            .read()
            .await
            .values()
            .map(|list_element| list_element.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for ListElement.
    ///
    pub async fn list_element_timestamp(&self, list_element: &ListElement) -> SystemTime {
        self.list_element
            .read()
            .await
            .get(&list_element.id)
            .map(|list_element| list_element.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ListExpression`] into the store.
    ///
    pub async fn inter_list_expression(&mut self, list_expression: Arc<RwLock<ListExpression>>) {
        let read = list_expression.read().await;
        self.list_expression
            .write()
            .await
            .insert(read.id, (list_expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ListExpression`] from the store.
    ///
    pub async fn exhume_list_expression(&self, id: &Uuid) -> Option<Arc<RwLock<ListExpression>>> {
        self.list_expression
            .read()
            .await
            .get(id)
            .map(|list_expression| list_expression.0.clone())
    }

    /// Exorcise (remove) [`ListExpression`] from the store.
    ///
    pub async fn exorcise_list_expression(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<ListExpression>>> {
        self.list_expression
            .write()
            .await
            .remove(id)
            .map(|list_expression| list_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListExpression>`.
    ///
    pub async fn iter_list_expression(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<ListExpression>>> + '_ {
        let values: Vec<Arc<RwLock<ListExpression>>> = self
            .list_expression
            .read()
            .await
            .values()
            .map(|list_expression| list_expression.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for ListExpression.
    ///
    pub async fn list_expression_timestamp(&self, list_expression: &ListExpression) -> SystemTime {
        self.list_expression
            .read()
            .await
            .get(&list_expression.id)
            .map(|list_expression| list_expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Literal`] into the store.
    ///
    pub async fn inter_literal(&mut self, literal: Arc<RwLock<Literal>>) {
        let read = literal.read().await;
        self.literal
            .write()
            .await
            .insert(read.id(), (literal.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Literal`] from the store.
    ///
    pub async fn exhume_literal(&self, id: &Uuid) -> Option<Arc<RwLock<Literal>>> {
        self.literal
            .read()
            .await
            .get(id)
            .map(|literal| literal.0.clone())
    }

    /// Exorcise (remove) [`Literal`] from the store.
    ///
    pub async fn exorcise_literal(&mut self, id: &Uuid) -> Option<Arc<RwLock<Literal>>> {
        self.literal
            .write()
            .await
            .remove(id)
            .map(|literal| literal.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Literal>`.
    ///
    pub async fn iter_literal(&self) -> impl Iterator<Item = Arc<RwLock<Literal>>> + '_ {
        let values: Vec<Arc<RwLock<Literal>>> = self
            .literal
            .read()
            .await
            .values()
            .map(|literal| literal.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Literal.
    ///
    pub async fn literal_timestamp(&self, literal: &Literal) -> SystemTime {
        self.literal
            .read()
            .await
            .get(&literal.id())
            .map(|literal| literal.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`LocalVariable`] into the store.
    ///
    pub async fn inter_local_variable(&mut self, local_variable: Arc<RwLock<LocalVariable>>) {
        let read = local_variable.read().await;
        self.local_variable
            .write()
            .await
            .insert(read.id, (local_variable.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`LocalVariable`] from the store.
    ///
    pub async fn exhume_local_variable(&self, id: &Uuid) -> Option<Arc<RwLock<LocalVariable>>> {
        self.local_variable
            .read()
            .await
            .get(id)
            .map(|local_variable| local_variable.0.clone())
    }

    /// Exorcise (remove) [`LocalVariable`] from the store.
    ///
    pub async fn exorcise_local_variable(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<LocalVariable>>> {
        self.local_variable
            .write()
            .await
            .remove(id)
            .map(|local_variable| local_variable.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LocalVariable>`.
    ///
    pub async fn iter_local_variable(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<LocalVariable>>> + '_ {
        let values: Vec<Arc<RwLock<LocalVariable>>> = self
            .local_variable
            .read()
            .await
            .values()
            .map(|local_variable| local_variable.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for LocalVariable.
    ///
    pub async fn local_variable_timestamp(&self, local_variable: &LocalVariable) -> SystemTime {
        self.local_variable
            .read()
            .await
            .get(&local_variable.id)
            .map(|local_variable| local_variable.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`XMacro`] into the store.
    ///
    pub async fn inter_x_macro(&mut self, x_macro: Arc<RwLock<XMacro>>) {
        let read = x_macro.read().await;
        self.x_macro
            .write()
            .await
            .insert(read.id, (x_macro.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`XMacro`] from the store.
    ///
    pub async fn exhume_x_macro(&self, id: &Uuid) -> Option<Arc<RwLock<XMacro>>> {
        self.x_macro
            .read()
            .await
            .get(id)
            .map(|x_macro| x_macro.0.clone())
    }

    /// Exorcise (remove) [`XMacro`] from the store.
    ///
    pub async fn exorcise_x_macro(&mut self, id: &Uuid) -> Option<Arc<RwLock<XMacro>>> {
        self.x_macro
            .write()
            .await
            .remove(id)
            .map(|x_macro| x_macro.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XMacro>`.
    ///
    pub async fn iter_x_macro(&self) -> impl Iterator<Item = Arc<RwLock<XMacro>>> + '_ {
        let values: Vec<Arc<RwLock<XMacro>>> = self
            .x_macro
            .read()
            .await
            .values()
            .map(|x_macro| x_macro.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for XMacro.
    ///
    pub async fn x_macro_timestamp(&self, x_macro: &XMacro) -> SystemTime {
        self.x_macro
            .read()
            .await
            .get(&x_macro.id)
            .map(|x_macro| x_macro.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`MethodCall`] into the store.
    ///
    pub async fn inter_method_call(&mut self, method_call: Arc<RwLock<MethodCall>>) {
        let read = method_call.read().await;
        self.method_call
            .write()
            .await
            .insert(read.id, (method_call.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`MethodCall`] from the store.
    ///
    pub async fn exhume_method_call(&self, id: &Uuid) -> Option<Arc<RwLock<MethodCall>>> {
        self.method_call
            .read()
            .await
            .get(id)
            .map(|method_call| method_call.0.clone())
    }

    /// Exorcise (remove) [`MethodCall`] from the store.
    ///
    pub async fn exorcise_method_call(&mut self, id: &Uuid) -> Option<Arc<RwLock<MethodCall>>> {
        self.method_call
            .write()
            .await
            .remove(id)
            .map(|method_call| method_call.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, MethodCall>`.
    ///
    pub async fn iter_method_call(&self) -> impl Iterator<Item = Arc<RwLock<MethodCall>>> + '_ {
        let values: Vec<Arc<RwLock<MethodCall>>> = self
            .method_call
            .read()
            .await
            .values()
            .map(|method_call| method_call.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for MethodCall.
    ///
    pub async fn method_call_timestamp(&self, method_call: &MethodCall) -> SystemTime {
        self.method_call
            .read()
            .await
            .get(&method_call.id)
            .map(|method_call| method_call.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ZObjectStore`] into the store.
    ///
    pub async fn inter_z_object_store(&mut self, z_object_store: Arc<RwLock<ZObjectStore>>) {
        let read = z_object_store.read().await;
        self.z_object_store
            .write()
            .await
            .insert(read.id, (z_object_store.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ZObjectStore`] from the store.
    ///
    pub async fn exhume_z_object_store(&self, id: &Uuid) -> Option<Arc<RwLock<ZObjectStore>>> {
        self.z_object_store
            .read()
            .await
            .get(id)
            .map(|z_object_store| z_object_store.0.clone())
    }

    /// Exorcise (remove) [`ZObjectStore`] from the store.
    ///
    pub async fn exorcise_z_object_store(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<ZObjectStore>>> {
        self.z_object_store
            .write()
            .await
            .remove(id)
            .map(|z_object_store| z_object_store.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ZObjectStore>`.
    ///
    pub async fn iter_z_object_store(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<ZObjectStore>>> + '_ {
        let values: Vec<Arc<RwLock<ZObjectStore>>> = self
            .z_object_store
            .read()
            .await
            .values()
            .map(|z_object_store| z_object_store.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for ZObjectStore.
    ///
    pub async fn z_object_store_timestamp(&self, z_object_store: &ZObjectStore) -> SystemTime {
        self.z_object_store
            .read()
            .await
            .get(&z_object_store.id)
            .map(|z_object_store| z_object_store.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Operator`] into the store.
    ///
    pub async fn inter_operator(&mut self, operator: Arc<RwLock<Operator>>) {
        let read = operator.read().await;
        self.operator
            .write()
            .await
            .insert(read.id, (operator.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Operator`] from the store.
    ///
    pub async fn exhume_operator(&self, id: &Uuid) -> Option<Arc<RwLock<Operator>>> {
        self.operator
            .read()
            .await
            .get(id)
            .map(|operator| operator.0.clone())
    }

    /// Exorcise (remove) [`Operator`] from the store.
    ///
    pub async fn exorcise_operator(&mut self, id: &Uuid) -> Option<Arc<RwLock<Operator>>> {
        self.operator
            .write()
            .await
            .remove(id)
            .map(|operator| operator.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Operator>`.
    ///
    pub async fn iter_operator(&self) -> impl Iterator<Item = Arc<RwLock<Operator>>> + '_ {
        let values: Vec<Arc<RwLock<Operator>>> = self
            .operator
            .read()
            .await
            .values()
            .map(|operator| operator.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Operator.
    ///
    pub async fn operator_timestamp(&self, operator: &Operator) -> SystemTime {
        self.operator
            .read()
            .await
            .get(&operator.id)
            .map(|operator| operator.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`WoogOption`] into the store.
    ///
    pub async fn inter_woog_option(&mut self, woog_option: Arc<RwLock<WoogOption>>) {
        let read = woog_option.read().await;
        self.woog_option
            .write()
            .await
            .insert(read.id, (woog_option.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`WoogOption`] from the store.
    ///
    pub async fn exhume_woog_option(&self, id: &Uuid) -> Option<Arc<RwLock<WoogOption>>> {
        self.woog_option
            .read()
            .await
            .get(id)
            .map(|woog_option| woog_option.0.clone())
    }

    /// Exorcise (remove) [`WoogOption`] from the store.
    ///
    pub async fn exorcise_woog_option(&mut self, id: &Uuid) -> Option<Arc<RwLock<WoogOption>>> {
        self.woog_option
            .write()
            .await
            .remove(id)
            .map(|woog_option| woog_option.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, WoogOption>`.
    ///
    pub async fn iter_woog_option(&self) -> impl Iterator<Item = Arc<RwLock<WoogOption>>> + '_ {
        let values: Vec<Arc<RwLock<WoogOption>>> = self
            .woog_option
            .read()
            .await
            .values()
            .map(|woog_option| woog_option.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for WoogOption.
    ///
    pub async fn woog_option_timestamp(&self, woog_option: &WoogOption) -> SystemTime {
        self.woog_option
            .read()
            .await
            .get(&woog_option.id)
            .map(|woog_option| woog_option.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Parameter`] into the store.
    ///
    pub async fn inter_parameter(&mut self, parameter: Arc<RwLock<Parameter>>) {
        let read = parameter.read().await;
        self.parameter
            .write()
            .await
            .insert(read.id, (parameter.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Parameter`] from the store.
    ///
    pub async fn exhume_parameter(&self, id: &Uuid) -> Option<Arc<RwLock<Parameter>>> {
        self.parameter
            .read()
            .await
            .get(id)
            .map(|parameter| parameter.0.clone())
    }

    /// Exorcise (remove) [`Parameter`] from the store.
    ///
    pub async fn exorcise_parameter(&mut self, id: &Uuid) -> Option<Arc<RwLock<Parameter>>> {
        self.parameter
            .write()
            .await
            .remove(id)
            .map(|parameter| parameter.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Parameter>`.
    ///
    pub async fn iter_parameter(&self) -> impl Iterator<Item = Arc<RwLock<Parameter>>> + '_ {
        let values: Vec<Arc<RwLock<Parameter>>> = self
            .parameter
            .read()
            .await
            .values()
            .map(|parameter| parameter.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Parameter.
    ///
    pub async fn parameter_timestamp(&self, parameter: &Parameter) -> SystemTime {
        self.parameter
            .read()
            .await
            .get(&parameter.id)
            .map(|parameter| parameter.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Print`] into the store.
    ///
    pub async fn inter_print(&mut self, print: Arc<RwLock<Print>>) {
        let read = print.read().await;
        self.print
            .write()
            .await
            .insert(read.id, (print.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Print`] from the store.
    ///
    pub async fn exhume_print(&self, id: &Uuid) -> Option<Arc<RwLock<Print>>> {
        self.print.read().await.get(id).map(|print| print.0.clone())
    }

    /// Exorcise (remove) [`Print`] from the store.
    ///
    pub async fn exorcise_print(&mut self, id: &Uuid) -> Option<Arc<RwLock<Print>>> {
        self.print
            .write()
            .await
            .remove(id)
            .map(|print| print.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Print>`.
    ///
    pub async fn iter_print(&self) -> impl Iterator<Item = Arc<RwLock<Print>>> + '_ {
        let values: Vec<Arc<RwLock<Print>>> = self
            .print
            .read()
            .await
            .values()
            .map(|print| print.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Print.
    ///
    pub async fn print_timestamp(&self, print: &Print) -> SystemTime {
        self.print
            .read()
            .await
            .get(&print.id)
            .map(|print| print.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`RangeExpression`] into the store.
    ///
    pub async fn inter_range_expression(&mut self, range_expression: Arc<RwLock<RangeExpression>>) {
        let read = range_expression.read().await;
        self.range_expression
            .write()
            .await
            .insert(read.id, (range_expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`RangeExpression`] from the store.
    ///
    pub async fn exhume_range_expression(&self, id: &Uuid) -> Option<Arc<RwLock<RangeExpression>>> {
        self.range_expression
            .read()
            .await
            .get(id)
            .map(|range_expression| range_expression.0.clone())
    }

    /// Exorcise (remove) [`RangeExpression`] from the store.
    ///
    pub async fn exorcise_range_expression(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<RangeExpression>>> {
        self.range_expression
            .write()
            .await
            .remove(id)
            .map(|range_expression| range_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RangeExpression>`.
    ///
    pub async fn iter_range_expression(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<RangeExpression>>> + '_ {
        let values: Vec<Arc<RwLock<RangeExpression>>> = self
            .range_expression
            .read()
            .await
            .values()
            .map(|range_expression| range_expression.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for RangeExpression.
    ///
    pub async fn range_expression_timestamp(
        &self,
        range_expression: &RangeExpression,
    ) -> SystemTime {
        self.range_expression
            .read()
            .await
            .get(&range_expression.id)
            .map(|range_expression| range_expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Reference`] into the store.
    ///
    pub async fn inter_reference(&mut self, reference: Arc<RwLock<Reference>>) {
        let read = reference.read().await;
        self.reference
            .write()
            .await
            .insert(read.id, (reference.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Reference`] from the store.
    ///
    pub async fn exhume_reference(&self, id: &Uuid) -> Option<Arc<RwLock<Reference>>> {
        self.reference
            .read()
            .await
            .get(id)
            .map(|reference| reference.0.clone())
    }

    /// Exorcise (remove) [`Reference`] from the store.
    ///
    pub async fn exorcise_reference(&mut self, id: &Uuid) -> Option<Arc<RwLock<Reference>>> {
        self.reference
            .write()
            .await
            .remove(id)
            .map(|reference| reference.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Reference>`.
    ///
    pub async fn iter_reference(&self) -> impl Iterator<Item = Arc<RwLock<Reference>>> + '_ {
        let values: Vec<Arc<RwLock<Reference>>> = self
            .reference
            .read()
            .await
            .values()
            .map(|reference| reference.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Reference.
    ///
    pub async fn reference_timestamp(&self, reference: &Reference) -> SystemTime {
        self.reference
            .read()
            .await
            .get(&reference.id)
            .map(|reference| reference.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ResultStatement`] into the store.
    ///
    pub async fn inter_result_statement(&mut self, result_statement: Arc<RwLock<ResultStatement>>) {
        let read = result_statement.read().await;
        self.result_statement
            .write()
            .await
            .insert(read.id, (result_statement.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ResultStatement`] from the store.
    ///
    pub async fn exhume_result_statement(&self, id: &Uuid) -> Option<Arc<RwLock<ResultStatement>>> {
        self.result_statement
            .read()
            .await
            .get(id)
            .map(|result_statement| result_statement.0.clone())
    }

    /// Exorcise (remove) [`ResultStatement`] from the store.
    ///
    pub async fn exorcise_result_statement(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<ResultStatement>>> {
        self.result_statement
            .write()
            .await
            .remove(id)
            .map(|result_statement| result_statement.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ResultStatement>`.
    ///
    pub async fn iter_result_statement(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<ResultStatement>>> + '_ {
        let values: Vec<Arc<RwLock<ResultStatement>>> = self
            .result_statement
            .read()
            .await
            .values()
            .map(|result_statement| result_statement.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for ResultStatement.
    ///
    pub async fn result_statement_timestamp(
        &self,
        result_statement: &ResultStatement,
    ) -> SystemTime {
        self.result_statement
            .read()
            .await
            .get(&result_statement.id)
            .map(|result_statement| result_statement.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`XReturn`] into the store.
    ///
    pub async fn inter_x_return(&mut self, x_return: Arc<RwLock<XReturn>>) {
        let read = x_return.read().await;
        self.x_return
            .write()
            .await
            .insert(read.id, (x_return.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`XReturn`] from the store.
    ///
    pub async fn exhume_x_return(&self, id: &Uuid) -> Option<Arc<RwLock<XReturn>>> {
        self.x_return
            .read()
            .await
            .get(id)
            .map(|x_return| x_return.0.clone())
    }

    /// Exorcise (remove) [`XReturn`] from the store.
    ///
    pub async fn exorcise_x_return(&mut self, id: &Uuid) -> Option<Arc<RwLock<XReturn>>> {
        self.x_return
            .write()
            .await
            .remove(id)
            .map(|x_return| x_return.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XReturn>`.
    ///
    pub async fn iter_x_return(&self) -> impl Iterator<Item = Arc<RwLock<XReturn>>> + '_ {
        let values: Vec<Arc<RwLock<XReturn>>> = self
            .x_return
            .read()
            .await
            .values()
            .map(|x_return| x_return.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for XReturn.
    ///
    pub async fn x_return_timestamp(&self, x_return: &XReturn) -> SystemTime {
        self.x_return
            .read()
            .await
            .get(&x_return.id)
            .map(|x_return| x_return.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ZSome`] into the store.
    ///
    pub async fn inter_z_some(&mut self, z_some: Arc<RwLock<ZSome>>) {
        let read = z_some.read().await;
        self.z_some
            .write()
            .await
            .insert(read.id, (z_some.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ZSome`] from the store.
    ///
    pub async fn exhume_z_some(&self, id: &Uuid) -> Option<Arc<RwLock<ZSome>>> {
        self.z_some
            .read()
            .await
            .get(id)
            .map(|z_some| z_some.0.clone())
    }

    /// Exorcise (remove) [`ZSome`] from the store.
    ///
    pub async fn exorcise_z_some(&mut self, id: &Uuid) -> Option<Arc<RwLock<ZSome>>> {
        self.z_some
            .write()
            .await
            .remove(id)
            .map(|z_some| z_some.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ZSome>`.
    ///
    pub async fn iter_z_some(&self) -> impl Iterator<Item = Arc<RwLock<ZSome>>> + '_ {
        let values: Vec<Arc<RwLock<ZSome>>> = self
            .z_some
            .read()
            .await
            .values()
            .map(|z_some| z_some.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for ZSome.
    ///
    pub async fn z_some_timestamp(&self, z_some: &ZSome) -> SystemTime {
        self.z_some
            .read()
            .await
            .get(&z_some.id)
            .map(|z_some| z_some.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Span`] into the store.
    ///
    pub async fn inter_span(&mut self, span: Arc<RwLock<Span>>) {
        let read = span.read().await;
        self.span
            .write()
            .await
            .insert(read.id, (span.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Span`] from the store.
    ///
    pub async fn exhume_span(&self, id: &Uuid) -> Option<Arc<RwLock<Span>>> {
        self.span.read().await.get(id).map(|span| span.0.clone())
    }

    /// Exorcise (remove) [`Span`] from the store.
    ///
    pub async fn exorcise_span(&mut self, id: &Uuid) -> Option<Arc<RwLock<Span>>> {
        self.span
            .write()
            .await
            .remove(id)
            .map(|span| span.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Span>`.
    ///
    pub async fn iter_span(&self) -> impl Iterator<Item = Arc<RwLock<Span>>> + '_ {
        let values: Vec<Arc<RwLock<Span>>> = self
            .span
            .read()
            .await
            .values()
            .map(|span| span.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Span.
    ///
    pub async fn span_timestamp(&self, span: &Span) -> SystemTime {
        self.span
            .read()
            .await
            .get(&span.id)
            .map(|span| span.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Statement`] into the store.
    ///
    pub async fn inter_statement(&mut self, statement: Arc<RwLock<Statement>>) {
        let read = statement.read().await;
        self.statement
            .write()
            .await
            .insert(read.id, (statement.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Statement`] from the store.
    ///
    pub async fn exhume_statement(&self, id: &Uuid) -> Option<Arc<RwLock<Statement>>> {
        self.statement
            .read()
            .await
            .get(id)
            .map(|statement| statement.0.clone())
    }

    /// Exorcise (remove) [`Statement`] from the store.
    ///
    pub async fn exorcise_statement(&mut self, id: &Uuid) -> Option<Arc<RwLock<Statement>>> {
        self.statement
            .write()
            .await
            .remove(id)
            .map(|statement| statement.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Statement>`.
    ///
    pub async fn iter_statement(&self) -> impl Iterator<Item = Arc<RwLock<Statement>>> + '_ {
        let values: Vec<Arc<RwLock<Statement>>> = self
            .statement
            .read()
            .await
            .values()
            .map(|statement| statement.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Statement.
    ///
    pub async fn statement_timestamp(&self, statement: &Statement) -> SystemTime {
        self.statement
            .read()
            .await
            .get(&statement.id)
            .map(|statement| statement.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`StaticMethodCall`] into the store.
    ///
    pub async fn inter_static_method_call(
        &mut self,
        static_method_call: Arc<RwLock<StaticMethodCall>>,
    ) {
        let read = static_method_call.read().await;
        self.static_method_call
            .write()
            .await
            .insert(read.id, (static_method_call.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`StaticMethodCall`] from the store.
    ///
    pub async fn exhume_static_method_call(
        &self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<StaticMethodCall>>> {
        self.static_method_call
            .read()
            .await
            .get(id)
            .map(|static_method_call| static_method_call.0.clone())
    }

    /// Exorcise (remove) [`StaticMethodCall`] from the store.
    ///
    pub async fn exorcise_static_method_call(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<StaticMethodCall>>> {
        self.static_method_call
            .write()
            .await
            .remove(id)
            .map(|static_method_call| static_method_call.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StaticMethodCall>`.
    ///
    pub async fn iter_static_method_call(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<StaticMethodCall>>> + '_ {
        let values: Vec<Arc<RwLock<StaticMethodCall>>> = self
            .static_method_call
            .read()
            .await
            .values()
            .map(|static_method_call| static_method_call.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for StaticMethodCall.
    ///
    pub async fn static_method_call_timestamp(
        &self,
        static_method_call: &StaticMethodCall,
    ) -> SystemTime {
        self.static_method_call
            .read()
            .await
            .get(&static_method_call.id)
            .map(|static_method_call| static_method_call.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`StringLiteral`] into the store.
    ///
    pub async fn inter_string_literal(&mut self, string_literal: Arc<RwLock<StringLiteral>>) {
        let read = string_literal.read().await;
        self.string_literal
            .write()
            .await
            .insert(read.id, (string_literal.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`StringLiteral`] from the store.
    ///
    pub async fn exhume_string_literal(&self, id: &Uuid) -> Option<Arc<RwLock<StringLiteral>>> {
        self.string_literal
            .read()
            .await
            .get(id)
            .map(|string_literal| string_literal.0.clone())
    }

    /// Exorcise (remove) [`StringLiteral`] from the store.
    ///
    pub async fn exorcise_string_literal(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<StringLiteral>>> {
        self.string_literal
            .write()
            .await
            .remove(id)
            .map(|string_literal| string_literal.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StringLiteral>`.
    ///
    pub async fn iter_string_literal(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<StringLiteral>>> + '_ {
        let values: Vec<Arc<RwLock<StringLiteral>>> = self
            .string_literal
            .read()
            .await
            .values()
            .map(|string_literal| string_literal.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for StringLiteral.
    ///
    pub async fn string_literal_timestamp(&self, string_literal: &StringLiteral) -> SystemTime {
        self.string_literal
            .read()
            .await
            .get(&string_literal.id)
            .map(|string_literal| string_literal.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`WoogStruct`] into the store.
    ///
    pub async fn inter_woog_struct(&mut self, woog_struct: Arc<RwLock<WoogStruct>>) {
        let read = woog_struct.read().await;
        let value = (woog_struct.clone(), SystemTime::now());
        self.woog_struct_id_by_name
            .write()
            .await
            .insert(read.name.to_upper_camel_case(), (read.id, value.1));
        self.woog_struct.write().await.insert(read.id, value);
    }

    /// Exhume (get) [`WoogStruct`] from the store.
    ///
    pub async fn exhume_woog_struct(&self, id: &Uuid) -> Option<Arc<RwLock<WoogStruct>>> {
        self.woog_struct
            .read()
            .await
            .get(id)
            .map(|woog_struct| woog_struct.0.clone())
    }

    /// Exorcise (remove) [`WoogStruct`] from the store.
    ///
    pub async fn exorcise_woog_struct(&mut self, id: &Uuid) -> Option<Arc<RwLock<WoogStruct>>> {
        self.woog_struct
            .write()
            .await
            .remove(id)
            .map(|woog_struct| woog_struct.0.clone())
    }

    /// Exhume [`WoogStruct`] id from the store by name.
    ///
    pub async fn exhume_woog_struct_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.woog_struct_id_by_name
            .read()
            .await
            .get(name)
            .map(|woog_struct| woog_struct.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, WoogStruct>`.
    ///
    pub async fn iter_woog_struct(&self) -> impl Iterator<Item = Arc<RwLock<WoogStruct>>> + '_ {
        let values: Vec<Arc<RwLock<WoogStruct>>> = self
            .woog_struct
            .read()
            .await
            .values()
            .map(|woog_struct| woog_struct.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for WoogStruct.
    ///
    pub async fn woog_struct_timestamp(&self, woog_struct: &WoogStruct) -> SystemTime {
        self.woog_struct
            .read()
            .await
            .get(&woog_struct.id)
            .map(|woog_struct| woog_struct.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`StructExpression`] into the store.
    ///
    pub async fn inter_struct_expression(
        &mut self,
        struct_expression: Arc<RwLock<StructExpression>>,
    ) {
        let read = struct_expression.read().await;
        self.struct_expression
            .write()
            .await
            .insert(read.id, (struct_expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`StructExpression`] from the store.
    ///
    pub async fn exhume_struct_expression(
        &self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<StructExpression>>> {
        self.struct_expression
            .read()
            .await
            .get(id)
            .map(|struct_expression| struct_expression.0.clone())
    }

    /// Exorcise (remove) [`StructExpression`] from the store.
    ///
    pub async fn exorcise_struct_expression(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<StructExpression>>> {
        self.struct_expression
            .write()
            .await
            .remove(id)
            .map(|struct_expression| struct_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructExpression>`.
    ///
    pub async fn iter_struct_expression(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<StructExpression>>> + '_ {
        let values: Vec<Arc<RwLock<StructExpression>>> = self
            .struct_expression
            .read()
            .await
            .values()
            .map(|struct_expression| struct_expression.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for StructExpression.
    ///
    pub async fn struct_expression_timestamp(
        &self,
        struct_expression: &StructExpression,
    ) -> SystemTime {
        self.struct_expression
            .read()
            .await
            .get(&struct_expression.id)
            .map(|struct_expression| struct_expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`TypeCast`] into the store.
    ///
    pub async fn inter_type_cast(&mut self, type_cast: Arc<RwLock<TypeCast>>) {
        let read = type_cast.read().await;
        self.type_cast
            .write()
            .await
            .insert(read.id, (type_cast.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`TypeCast`] from the store.
    ///
    pub async fn exhume_type_cast(&self, id: &Uuid) -> Option<Arc<RwLock<TypeCast>>> {
        self.type_cast
            .read()
            .await
            .get(id)
            .map(|type_cast| type_cast.0.clone())
    }

    /// Exorcise (remove) [`TypeCast`] from the store.
    ///
    pub async fn exorcise_type_cast(&mut self, id: &Uuid) -> Option<Arc<RwLock<TypeCast>>> {
        self.type_cast
            .write()
            .await
            .remove(id)
            .map(|type_cast| type_cast.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, TypeCast>`.
    ///
    pub async fn iter_type_cast(&self) -> impl Iterator<Item = Arc<RwLock<TypeCast>>> + '_ {
        let values: Vec<Arc<RwLock<TypeCast>>> = self
            .type_cast
            .read()
            .await
            .values()
            .map(|type_cast| type_cast.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for TypeCast.
    ///
    pub async fn type_cast_timestamp(&self, type_cast: &TypeCast) -> SystemTime {
        self.type_cast
            .read()
            .await
            .get(&type_cast.id)
            .map(|type_cast| type_cast.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Unary`] into the store.
    ///
    pub async fn inter_unary(&mut self, unary: Arc<RwLock<Unary>>) {
        let read = unary.read().await;
        self.unary
            .write()
            .await
            .insert(read.id(), (unary.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Unary`] from the store.
    ///
    pub async fn exhume_unary(&self, id: &Uuid) -> Option<Arc<RwLock<Unary>>> {
        self.unary.read().await.get(id).map(|unary| unary.0.clone())
    }

    /// Exorcise (remove) [`Unary`] from the store.
    ///
    pub async fn exorcise_unary(&mut self, id: &Uuid) -> Option<Arc<RwLock<Unary>>> {
        self.unary
            .write()
            .await
            .remove(id)
            .map(|unary| unary.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Unary>`.
    ///
    pub async fn iter_unary(&self) -> impl Iterator<Item = Arc<RwLock<Unary>>> + '_ {
        let values: Vec<Arc<RwLock<Unary>>> = self
            .unary
            .read()
            .await
            .values()
            .map(|unary| unary.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Unary.
    ///
    pub async fn unary_timestamp(&self, unary: &Unary) -> SystemTime {
        self.unary
            .read()
            .await
            .get(&unary.id())
            .map(|unary| unary.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`XValue`] into the store.
    ///
    pub async fn inter_x_value(&mut self, x_value: Arc<RwLock<XValue>>) {
        let read = x_value.read().await;
        self.x_value
            .write()
            .await
            .insert(read.id, (x_value.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`XValue`] from the store.
    ///
    pub async fn exhume_x_value(&self, id: &Uuid) -> Option<Arc<RwLock<XValue>>> {
        self.x_value
            .read()
            .await
            .get(id)
            .map(|x_value| x_value.0.clone())
    }

    /// Exorcise (remove) [`XValue`] from the store.
    ///
    pub async fn exorcise_x_value(&mut self, id: &Uuid) -> Option<Arc<RwLock<XValue>>> {
        self.x_value
            .write()
            .await
            .remove(id)
            .map(|x_value| x_value.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XValue>`.
    ///
    pub async fn iter_x_value(&self) -> impl Iterator<Item = Arc<RwLock<XValue>>> + '_ {
        let values: Vec<Arc<RwLock<XValue>>> = self
            .x_value
            .read()
            .await
            .values()
            .map(|x_value| x_value.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for XValue.
    ///
    pub async fn x_value_timestamp(&self, x_value: &XValue) -> SystemTime {
        self.x_value
            .read()
            .await
            .get(&x_value.id)
            .map(|x_value| x_value.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ValueType`] into the store.
    ///
    pub async fn inter_value_type(&mut self, value_type: Arc<RwLock<ValueType>>) {
        let read = value_type.read().await;
        self.value_type
            .write()
            .await
            .insert(read.id(), (value_type.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ValueType`] from the store.
    ///
    pub async fn exhume_value_type(&self, id: &Uuid) -> Option<Arc<RwLock<ValueType>>> {
        self.value_type
            .read()
            .await
            .get(id)
            .map(|value_type| value_type.0.clone())
    }

    /// Exorcise (remove) [`ValueType`] from the store.
    ///
    pub async fn exorcise_value_type(&mut self, id: &Uuid) -> Option<Arc<RwLock<ValueType>>> {
        self.value_type
            .write()
            .await
            .remove(id)
            .map(|value_type| value_type.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ValueType>`.
    ///
    pub async fn iter_value_type(&self) -> impl Iterator<Item = Arc<RwLock<ValueType>>> + '_ {
        let values: Vec<Arc<RwLock<ValueType>>> = self
            .value_type
            .read()
            .await
            .values()
            .map(|value_type| value_type.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for ValueType.
    ///
    pub async fn value_type_timestamp(&self, value_type: &ValueType) -> SystemTime {
        self.value_type
            .read()
            .await
            .get(&value_type.id())
            .map(|value_type| value_type.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Variable`] into the store.
    ///
    pub async fn inter_variable(&mut self, variable: Arc<RwLock<Variable>>) {
        let read = variable.read().await;
        self.variable
            .write()
            .await
            .insert(read.id, (variable.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Variable`] from the store.
    ///
    pub async fn exhume_variable(&self, id: &Uuid) -> Option<Arc<RwLock<Variable>>> {
        self.variable
            .read()
            .await
            .get(id)
            .map(|variable| variable.0.clone())
    }

    /// Exorcise (remove) [`Variable`] from the store.
    ///
    pub async fn exorcise_variable(&mut self, id: &Uuid) -> Option<Arc<RwLock<Variable>>> {
        self.variable
            .write()
            .await
            .remove(id)
            .map(|variable| variable.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Variable>`.
    ///
    pub async fn iter_variable(&self) -> impl Iterator<Item = Arc<RwLock<Variable>>> + '_ {
        let values: Vec<Arc<RwLock<Variable>>> = self
            .variable
            .read()
            .await
            .values()
            .map(|variable| variable.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Variable.
    ///
    pub async fn variable_timestamp(&self, variable: &Variable) -> SystemTime {
        self.variable
            .read()
            .await
            .get(&variable.id)
            .map(|variable| variable.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`VariableExpression`] into the store.
    ///
    pub async fn inter_variable_expression(
        &mut self,
        variable_expression: Arc<RwLock<VariableExpression>>,
    ) {
        let read = variable_expression.read().await;
        self.variable_expression
            .write()
            .await
            .insert(read.id, (variable_expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`VariableExpression`] from the store.
    ///
    pub async fn exhume_variable_expression(
        &self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<VariableExpression>>> {
        self.variable_expression
            .read()
            .await
            .get(id)
            .map(|variable_expression| variable_expression.0.clone())
    }

    /// Exorcise (remove) [`VariableExpression`] from the store.
    ///
    pub async fn exorcise_variable_expression(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<VariableExpression>>> {
        self.variable_expression
            .write()
            .await
            .remove(id)
            .map(|variable_expression| variable_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, VariableExpression>`.
    ///
    pub async fn iter_variable_expression(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<VariableExpression>>> + '_ {
        let values: Vec<Arc<RwLock<VariableExpression>>> = self
            .variable_expression
            .read()
            .await
            .values()
            .map(|variable_expression| variable_expression.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for VariableExpression.
    ///
    pub async fn variable_expression_timestamp(
        &self,
        variable_expression: &VariableExpression,
    ) -> SystemTime {
        self.variable_expression
            .read()
            .await
            .get(&variable_expression.id)
            .map(|variable_expression| variable_expression.1)
            .unwrap_or(SystemTime::now())
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_async-object-store-persistence"}}}
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
    pub async fn persist<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        fs::create_dir_all(path)?;

        let path = path.join("lu_dog.json");
        fs::create_dir_all(&path)?;

        // Persist Argument.
        {
            let path = path.join("argument");
            fs::create_dir_all(&path)?;
            for argument_tuple in self.argument.read().await.values() {
                let path = path.join(format!("{}.json", argument_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Argument>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned() != argument_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(&argument_tuple.0.read().await.to_owned(), &argument_tuple.1),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(&argument_tuple.0.read().await.to_owned(), &argument_tuple.1),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.argument.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Binary.
        {
            let path = path.join("binary");
            fs::create_dir_all(&path)?;
            for binary_tuple in self.binary.read().await.values() {
                let path = path.join(format!("{}.json", binary_tuple.0.read().await.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Binary>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned() != binary_tuple.0.read().await.to_owned() {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(&binary_tuple.0.read().await.to_owned(), &binary_tuple.1),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(&binary_tuple.0.read().await.to_owned(), &binary_tuple.1),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.binary.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Block.
        {
            let path = path.join("block");
            fs::create_dir_all(&path)?;
            for block_tuple in self.block.read().await.values() {
                let path = path.join(format!("{}.json", block_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Block>>, SystemTime) = serde_json::from_reader(reader)
                        .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned() != block_tuple.0.read().await.to_owned() {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(&block_tuple.0.read().await.to_owned(), &block_tuple.1),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(&block_tuple.0.read().await.to_owned(), &block_tuple.1),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.block.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Boolean Literal.
        {
            let path = path.join("boolean_literal");
            fs::create_dir_all(&path)?;
            for boolean_literal_tuple in self.boolean_literal.read().await.values() {
                let path = path.join(format!(
                    "{}.json",
                    boolean_literal_tuple.0.read().await.id()
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<BooleanLiteral>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != boolean_literal_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &boolean_literal_tuple.0.read().await.to_owned(),
                                &boolean_literal_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &boolean_literal_tuple.0.read().await.to_owned(),
                            &boolean_literal_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.boolean_literal.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Boolean Operator.
        {
            let path = path.join("boolean_operator");
            fs::create_dir_all(&path)?;
            for boolean_operator_tuple in self.boolean_operator.read().await.values() {
                let path = path.join(format!(
                    "{}.json",
                    boolean_operator_tuple.0.read().await.id()
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<BooleanOperator>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != boolean_operator_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &boolean_operator_tuple.0.read().await.to_owned(),
                                &boolean_operator_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &boolean_operator_tuple.0.read().await.to_owned(),
                            &boolean_operator_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.boolean_operator.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Call.
        {
            let path = path.join("call");
            fs::create_dir_all(&path)?;
            for call_tuple in self.call.read().await.values() {
                let path = path.join(format!("{}.json", call_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Call>>, SystemTime) = serde_json::from_reader(reader)
                        .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned() != call_tuple.0.read().await.to_owned() {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(&call_tuple.0.read().await.to_owned(), &call_tuple.1),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(&call_tuple.0.read().await.to_owned(), &call_tuple.1),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.call.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Comparison.
        {
            let path = path.join("comparison");
            fs::create_dir_all(&path)?;
            for comparison_tuple in self.comparison.read().await.values() {
                let path = path.join(format!("{}.json", comparison_tuple.0.read().await.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Comparison>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != comparison_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &comparison_tuple.0.read().await.to_owned(),
                                &comparison_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &comparison_tuple.0.read().await.to_owned(),
                            &comparison_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.comparison.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Dwarf Source File.
        {
            let path = path.join("dwarf_source_file");
            fs::create_dir_all(&path)?;
            for dwarf_source_file_tuple in self.dwarf_source_file.read().await.values() {
                let path = path.join(format!(
                    "{}.json",
                    dwarf_source_file_tuple.0.read().await.id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<DwarfSourceFile>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != dwarf_source_file_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &dwarf_source_file_tuple.0.read().await.to_owned(),
                                &dwarf_source_file_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &dwarf_source_file_tuple.0.read().await.to_owned(),
                            &dwarf_source_file_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.dwarf_source_file.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Error.
        {
            let path = path.join("error");
            fs::create_dir_all(&path)?;
            for error_tuple in self.error.read().await.values() {
                let path = path.join(format!("{}.json", error_tuple.0.read().await.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Error>>, SystemTime) = serde_json::from_reader(reader)
                        .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned() != error_tuple.0.read().await.to_owned() {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(&error_tuple.0.read().await.to_owned(), &error_tuple.1),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(&error_tuple.0.read().await.to_owned(), &error_tuple.1),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.error.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Error Expression.
        {
            let path = path.join("error_expression");
            fs::create_dir_all(&path)?;
            for error_expression_tuple in self.error_expression.read().await.values() {
                let path = path.join(format!("{}.json", error_expression_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<ErrorExpression>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != error_expression_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &error_expression_tuple.0.read().await.to_owned(),
                                &error_expression_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &error_expression_tuple.0.read().await.to_owned(),
                            &error_expression_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.error_expression.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Expression.
        {
            let path = path.join("expression");
            fs::create_dir_all(&path)?;
            for expression_tuple in self.expression.read().await.values() {
                let path = path.join(format!("{}.json", expression_tuple.0.read().await.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Expression>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != expression_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &expression_tuple.0.read().await.to_owned(),
                                &expression_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &expression_tuple.0.read().await.to_owned(),
                            &expression_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.expression.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Expression Statement.
        {
            let path = path.join("expression_statement");
            fs::create_dir_all(&path)?;
            for expression_statement_tuple in self.expression_statement.read().await.values() {
                let path = path.join(format!(
                    "{}.json",
                    expression_statement_tuple.0.read().await.id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<ExpressionStatement>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != expression_statement_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &expression_statement_tuple.0.read().await.to_owned(),
                                &expression_statement_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &expression_statement_tuple.0.read().await.to_owned(),
                            &expression_statement_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.expression_statement.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Field.
        {
            let path = path.join("field");
            fs::create_dir_all(&path)?;
            for field_tuple in self.field.read().await.values() {
                let path = path.join(format!("{}.json", field_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Field>>, SystemTime) = serde_json::from_reader(reader)
                        .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned() != field_tuple.0.read().await.to_owned() {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(&field_tuple.0.read().await.to_owned(), &field_tuple.1),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(&field_tuple.0.read().await.to_owned(), &field_tuple.1),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.field.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Field Access.
        {
            let path = path.join("field_access");
            fs::create_dir_all(&path)?;
            for field_access_tuple in self.field_access.read().await.values() {
                let path = path.join(format!("{}.json", field_access_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<FieldAccess>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != field_access_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &field_access_tuple.0.read().await.to_owned(),
                                &field_access_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &field_access_tuple.0.read().await.to_owned(),
                            &field_access_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.field_access.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Field Access Target.
        {
            let path = path.join("field_access_target");
            fs::create_dir_all(&path)?;
            for field_access_target_tuple in self.field_access_target.read().await.values() {
                let path = path.join(format!(
                    "{}.json",
                    field_access_target_tuple.0.read().await.id()
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<FieldAccessTarget>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != field_access_target_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &field_access_target_tuple.0.read().await.to_owned(),
                                &field_access_target_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &field_access_target_tuple.0.read().await.to_owned(),
                            &field_access_target_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.field_access_target.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Field Expression.
        {
            let path = path.join("field_expression");
            fs::create_dir_all(&path)?;
            for field_expression_tuple in self.field_expression.read().await.values() {
                let path = path.join(format!("{}.json", field_expression_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<FieldExpression>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != field_expression_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &field_expression_tuple.0.read().await.to_owned(),
                                &field_expression_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &field_expression_tuple.0.read().await.to_owned(),
                            &field_expression_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.field_expression.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Float Literal.
        {
            let path = path.join("float_literal");
            fs::create_dir_all(&path)?;
            for float_literal_tuple in self.float_literal.read().await.values() {
                let path = path.join(format!("{}.json", float_literal_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<FloatLiteral>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != float_literal_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &float_literal_tuple.0.read().await.to_owned(),
                                &float_literal_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &float_literal_tuple.0.read().await.to_owned(),
                            &float_literal_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.float_literal.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist For Loop.
        {
            let path = path.join("for_loop");
            fs::create_dir_all(&path)?;
            for for_loop_tuple in self.for_loop.read().await.values() {
                let path = path.join(format!("{}.json", for_loop_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<ForLoop>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned() != for_loop_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(&for_loop_tuple.0.read().await.to_owned(), &for_loop_tuple.1),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(&for_loop_tuple.0.read().await.to_owned(), &for_loop_tuple.1),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.for_loop.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Function.
        {
            let path = path.join("function");
            fs::create_dir_all(&path)?;
            for function_tuple in self.function.read().await.values() {
                let path = path.join(format!("{}.json", function_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Function>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned() != function_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(&function_tuple.0.read().await.to_owned(), &function_tuple.1),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(&function_tuple.0.read().await.to_owned(), &function_tuple.1),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.function.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Grouped.
        {
            let path = path.join("grouped");
            fs::create_dir_all(&path)?;
            for grouped_tuple in self.grouped.read().await.values() {
                let path = path.join(format!("{}.json", grouped_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Grouped>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned() != grouped_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(&grouped_tuple.0.read().await.to_owned(), &grouped_tuple.1),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(&grouped_tuple.0.read().await.to_owned(), &grouped_tuple.1),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.grouped.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist If.
        {
            let path = path.join("x_if");
            fs::create_dir_all(&path)?;
            for x_if_tuple in self.x_if.read().await.values() {
                let path = path.join(format!("{}.json", x_if_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<XIf>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned() != x_if_tuple.0.read().await.to_owned() {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(&x_if_tuple.0.read().await.to_owned(), &x_if_tuple.1),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(&x_if_tuple.0.read().await.to_owned(), &x_if_tuple.1),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.x_if.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Implementation.
        {
            let path = path.join("implementation");
            fs::create_dir_all(&path)?;
            for implementation_tuple in self.implementation.read().await.values() {
                let path = path.join(format!("{}.json", implementation_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Implementation>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != implementation_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &implementation_tuple.0.read().await.to_owned(),
                                &implementation_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &implementation_tuple.0.read().await.to_owned(),
                            &implementation_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.implementation.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Import.
        {
            let path = path.join("import");
            fs::create_dir_all(&path)?;
            for import_tuple in self.import.read().await.values() {
                let path = path.join(format!("{}.json", import_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Import>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned() != import_tuple.0.read().await.to_owned() {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(&import_tuple.0.read().await.to_owned(), &import_tuple.1),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(&import_tuple.0.read().await.to_owned(), &import_tuple.1),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.import.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Index.
        {
            let path = path.join("index");
            fs::create_dir_all(&path)?;
            for index_tuple in self.index.read().await.values() {
                let path = path.join(format!("{}.json", index_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Index>>, SystemTime) = serde_json::from_reader(reader)
                        .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned() != index_tuple.0.read().await.to_owned() {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(&index_tuple.0.read().await.to_owned(), &index_tuple.1),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(&index_tuple.0.read().await.to_owned(), &index_tuple.1),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.index.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Integer Literal.
        {
            let path = path.join("integer_literal");
            fs::create_dir_all(&path)?;
            for integer_literal_tuple in self.integer_literal.read().await.values() {
                let path = path.join(format!("{}.json", integer_literal_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<IntegerLiteral>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != integer_literal_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &integer_literal_tuple.0.read().await.to_owned(),
                                &integer_literal_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &integer_literal_tuple.0.read().await.to_owned(),
                            &integer_literal_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.integer_literal.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Item.
        {
            let path = path.join("item");
            fs::create_dir_all(&path)?;
            for item_tuple in self.item.read().await.values() {
                let path = path.join(format!("{}.json", item_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Item>>, SystemTime) = serde_json::from_reader(reader)
                        .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned() != item_tuple.0.read().await.to_owned() {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(&item_tuple.0.read().await.to_owned(), &item_tuple.1),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(&item_tuple.0.read().await.to_owned(), &item_tuple.1),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.item.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Let Statement.
        {
            let path = path.join("let_statement");
            fs::create_dir_all(&path)?;
            for let_statement_tuple in self.let_statement.read().await.values() {
                let path = path.join(format!("{}.json", let_statement_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<LetStatement>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != let_statement_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &let_statement_tuple.0.read().await.to_owned(),
                                &let_statement_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &let_statement_tuple.0.read().await.to_owned(),
                            &let_statement_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.let_statement.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist List.
        {
            let path = path.join("list");
            fs::create_dir_all(&path)?;
            for list_tuple in self.list.read().await.values() {
                let path = path.join(format!("{}.json", list_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<List>>, SystemTime) = serde_json::from_reader(reader)
                        .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned() != list_tuple.0.read().await.to_owned() {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(&list_tuple.0.read().await.to_owned(), &list_tuple.1),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(&list_tuple.0.read().await.to_owned(), &list_tuple.1),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.list.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist List Element.
        {
            let path = path.join("list_element");
            fs::create_dir_all(&path)?;
            for list_element_tuple in self.list_element.read().await.values() {
                let path = path.join(format!("{}.json", list_element_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<ListElement>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != list_element_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &list_element_tuple.0.read().await.to_owned(),
                                &list_element_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &list_element_tuple.0.read().await.to_owned(),
                            &list_element_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.list_element.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist List Expression.
        {
            let path = path.join("list_expression");
            fs::create_dir_all(&path)?;
            for list_expression_tuple in self.list_expression.read().await.values() {
                let path = path.join(format!("{}.json", list_expression_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<ListExpression>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != list_expression_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &list_expression_tuple.0.read().await.to_owned(),
                                &list_expression_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &list_expression_tuple.0.read().await.to_owned(),
                            &list_expression_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.list_expression.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Literal.
        {
            let path = path.join("literal");
            fs::create_dir_all(&path)?;
            for literal_tuple in self.literal.read().await.values() {
                let path = path.join(format!("{}.json", literal_tuple.0.read().await.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Literal>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned() != literal_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(&literal_tuple.0.read().await.to_owned(), &literal_tuple.1),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(&literal_tuple.0.read().await.to_owned(), &literal_tuple.1),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.literal.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Local Variable.
        {
            let path = path.join("local_variable");
            fs::create_dir_all(&path)?;
            for local_variable_tuple in self.local_variable.read().await.values() {
                let path = path.join(format!("{}.json", local_variable_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<LocalVariable>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != local_variable_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &local_variable_tuple.0.read().await.to_owned(),
                                &local_variable_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &local_variable_tuple.0.read().await.to_owned(),
                            &local_variable_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.local_variable.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Macro.
        {
            let path = path.join("x_macro");
            fs::create_dir_all(&path)?;
            for x_macro_tuple in self.x_macro.read().await.values() {
                let path = path.join(format!("{}.json", x_macro_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<XMacro>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned() != x_macro_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(&x_macro_tuple.0.read().await.to_owned(), &x_macro_tuple.1),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(&x_macro_tuple.0.read().await.to_owned(), &x_macro_tuple.1),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.x_macro.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Method Call.
        {
            let path = path.join("method_call");
            fs::create_dir_all(&path)?;
            for method_call_tuple in self.method_call.read().await.values() {
                let path = path.join(format!("{}.json", method_call_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<MethodCall>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != method_call_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &method_call_tuple.0.read().await.to_owned(),
                                &method_call_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &method_call_tuple.0.read().await.to_owned(),
                            &method_call_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.method_call.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Object Store.
        {
            let path = path.join("z_object_store");
            fs::create_dir_all(&path)?;
            for z_object_store_tuple in self.z_object_store.read().await.values() {
                let path = path.join(format!("{}.json", z_object_store_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<ZObjectStore>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != z_object_store_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &z_object_store_tuple.0.read().await.to_owned(),
                                &z_object_store_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &z_object_store_tuple.0.read().await.to_owned(),
                            &z_object_store_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.z_object_store.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Operator.
        {
            let path = path.join("operator");
            fs::create_dir_all(&path)?;
            for operator_tuple in self.operator.read().await.values() {
                let path = path.join(format!("{}.json", operator_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Operator>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned() != operator_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(&operator_tuple.0.read().await.to_owned(), &operator_tuple.1),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(&operator_tuple.0.read().await.to_owned(), &operator_tuple.1),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.operator.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Option.
        {
            let path = path.join("woog_option");
            fs::create_dir_all(&path)?;
            for woog_option_tuple in self.woog_option.read().await.values() {
                let path = path.join(format!("{}.json", woog_option_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<WoogOption>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != woog_option_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &woog_option_tuple.0.read().await.to_owned(),
                                &woog_option_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &woog_option_tuple.0.read().await.to_owned(),
                            &woog_option_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.woog_option.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Parameter.
        {
            let path = path.join("parameter");
            fs::create_dir_all(&path)?;
            for parameter_tuple in self.parameter.read().await.values() {
                let path = path.join(format!("{}.json", parameter_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Parameter>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != parameter_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &parameter_tuple.0.read().await.to_owned(),
                                &parameter_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &parameter_tuple.0.read().await.to_owned(),
                            &parameter_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.parameter.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Print.
        {
            let path = path.join("print");
            fs::create_dir_all(&path)?;
            for print_tuple in self.print.read().await.values() {
                let path = path.join(format!("{}.json", print_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Print>>, SystemTime) = serde_json::from_reader(reader)
                        .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned() != print_tuple.0.read().await.to_owned() {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(&print_tuple.0.read().await.to_owned(), &print_tuple.1),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(&print_tuple.0.read().await.to_owned(), &print_tuple.1),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.print.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Range Expression.
        {
            let path = path.join("range_expression");
            fs::create_dir_all(&path)?;
            for range_expression_tuple in self.range_expression.read().await.values() {
                let path = path.join(format!("{}.json", range_expression_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<RangeExpression>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != range_expression_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &range_expression_tuple.0.read().await.to_owned(),
                                &range_expression_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &range_expression_tuple.0.read().await.to_owned(),
                            &range_expression_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.range_expression.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Reference.
        {
            let path = path.join("reference");
            fs::create_dir_all(&path)?;
            for reference_tuple in self.reference.read().await.values() {
                let path = path.join(format!("{}.json", reference_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Reference>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != reference_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &reference_tuple.0.read().await.to_owned(),
                                &reference_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &reference_tuple.0.read().await.to_owned(),
                            &reference_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.reference.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Result Statement.
        {
            let path = path.join("result_statement");
            fs::create_dir_all(&path)?;
            for result_statement_tuple in self.result_statement.read().await.values() {
                let path = path.join(format!("{}.json", result_statement_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<ResultStatement>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != result_statement_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &result_statement_tuple.0.read().await.to_owned(),
                                &result_statement_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &result_statement_tuple.0.read().await.to_owned(),
                            &result_statement_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.result_statement.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Return.
        {
            let path = path.join("x_return");
            fs::create_dir_all(&path)?;
            for x_return_tuple in self.x_return.read().await.values() {
                let path = path.join(format!("{}.json", x_return_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<XReturn>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned() != x_return_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(&x_return_tuple.0.read().await.to_owned(), &x_return_tuple.1),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(&x_return_tuple.0.read().await.to_owned(), &x_return_tuple.1),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.x_return.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Some.
        {
            let path = path.join("z_some");
            fs::create_dir_all(&path)?;
            for z_some_tuple in self.z_some.read().await.values() {
                let path = path.join(format!("{}.json", z_some_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<ZSome>>, SystemTime) = serde_json::from_reader(reader)
                        .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned() != z_some_tuple.0.read().await.to_owned() {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(&z_some_tuple.0.read().await.to_owned(), &z_some_tuple.1),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(&z_some_tuple.0.read().await.to_owned(), &z_some_tuple.1),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.z_some.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Span.
        {
            let path = path.join("span");
            fs::create_dir_all(&path)?;
            for span_tuple in self.span.read().await.values() {
                let path = path.join(format!("{}.json", span_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Span>>, SystemTime) = serde_json::from_reader(reader)
                        .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned() != span_tuple.0.read().await.to_owned() {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(&span_tuple.0.read().await.to_owned(), &span_tuple.1),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(&span_tuple.0.read().await.to_owned(), &span_tuple.1),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.span.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Statement.
        {
            let path = path.join("statement");
            fs::create_dir_all(&path)?;
            for statement_tuple in self.statement.read().await.values() {
                let path = path.join(format!("{}.json", statement_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Statement>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != statement_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &statement_tuple.0.read().await.to_owned(),
                                &statement_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &statement_tuple.0.read().await.to_owned(),
                            &statement_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.statement.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Static Method Call.
        {
            let path = path.join("static_method_call");
            fs::create_dir_all(&path)?;
            for static_method_call_tuple in self.static_method_call.read().await.values() {
                let path = path.join(format!(
                    "{}.json",
                    static_method_call_tuple.0.read().await.id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<StaticMethodCall>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != static_method_call_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &static_method_call_tuple.0.read().await.to_owned(),
                                &static_method_call_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &static_method_call_tuple.0.read().await.to_owned(),
                            &static_method_call_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.static_method_call.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist String Literal.
        {
            let path = path.join("string_literal");
            fs::create_dir_all(&path)?;
            for string_literal_tuple in self.string_literal.read().await.values() {
                let path = path.join(format!("{}.json", string_literal_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<StringLiteral>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != string_literal_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &string_literal_tuple.0.read().await.to_owned(),
                                &string_literal_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &string_literal_tuple.0.read().await.to_owned(),
                            &string_literal_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.string_literal.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Struct.
        {
            let path = path.join("woog_struct");
            fs::create_dir_all(&path)?;
            for woog_struct_tuple in self.woog_struct.read().await.values() {
                let path = path.join(format!("{}.json", woog_struct_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<WoogStruct>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != woog_struct_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &woog_struct_tuple.0.read().await.to_owned(),
                                &woog_struct_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &woog_struct_tuple.0.read().await.to_owned(),
                            &woog_struct_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.woog_struct.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Struct Expression.
        {
            let path = path.join("struct_expression");
            fs::create_dir_all(&path)?;
            for struct_expression_tuple in self.struct_expression.read().await.values() {
                let path = path.join(format!(
                    "{}.json",
                    struct_expression_tuple.0.read().await.id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<StructExpression>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != struct_expression_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &struct_expression_tuple.0.read().await.to_owned(),
                                &struct_expression_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &struct_expression_tuple.0.read().await.to_owned(),
                            &struct_expression_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.struct_expression.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Type Cast.
        {
            let path = path.join("type_cast");
            fs::create_dir_all(&path)?;
            for type_cast_tuple in self.type_cast.read().await.values() {
                let path = path.join(format!("{}.json", type_cast_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<TypeCast>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != type_cast_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &type_cast_tuple.0.read().await.to_owned(),
                                &type_cast_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &type_cast_tuple.0.read().await.to_owned(),
                            &type_cast_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.type_cast.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Unary.
        {
            let path = path.join("unary");
            fs::create_dir_all(&path)?;
            for unary_tuple in self.unary.read().await.values() {
                let path = path.join(format!("{}.json", unary_tuple.0.read().await.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Unary>>, SystemTime) = serde_json::from_reader(reader)
                        .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned() != unary_tuple.0.read().await.to_owned() {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(&unary_tuple.0.read().await.to_owned(), &unary_tuple.1),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(&unary_tuple.0.read().await.to_owned(), &unary_tuple.1),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.unary.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Value.
        {
            let path = path.join("x_value");
            fs::create_dir_all(&path)?;
            for x_value_tuple in self.x_value.read().await.values() {
                let path = path.join(format!("{}.json", x_value_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<XValue>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned() != x_value_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(&x_value_tuple.0.read().await.to_owned(), &x_value_tuple.1),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(&x_value_tuple.0.read().await.to_owned(), &x_value_tuple.1),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.x_value.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Value Type.
        {
            let path = path.join("value_type");
            fs::create_dir_all(&path)?;
            for value_type_tuple in self.value_type.read().await.values() {
                let path = path.join(format!("{}.json", value_type_tuple.0.read().await.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<ValueType>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != value_type_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &value_type_tuple.0.read().await.to_owned(),
                                &value_type_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &value_type_tuple.0.read().await.to_owned(),
                            &value_type_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.value_type.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Variable.
        {
            let path = path.join("variable");
            fs::create_dir_all(&path)?;
            for variable_tuple in self.variable.read().await.values() {
                let path = path.join(format!("{}.json", variable_tuple.0.read().await.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<Variable>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned() != variable_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(&variable_tuple.0.read().await.to_owned(), &variable_tuple.1),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(&variable_tuple.0.read().await.to_owned(), &variable_tuple.1),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.variable.read().await.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Variable Expression.
        {
            let path = path.join("variable_expression");
            fs::create_dir_all(&path)?;
            for variable_expression_tuple in self.variable_expression.read().await.values() {
                let path = path.join(format!(
                    "{}.json",
                    variable_expression_tuple.0.read().await.id
                ));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<RwLock<VariableExpression>>, SystemTime) =
                        serde_json::from_reader(reader)
                            .map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                    if on_disk.0.read().await.to_owned()
                        != variable_expression_tuple.0.read().await.to_owned()
                    {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(
                            &mut writer,
                            &(
                                &variable_expression_tuple.0.read().await.to_owned(),
                                &variable_expression_tuple.1,
                            ),
                        )?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(
                            &variable_expression_tuple.0.read().await.to_owned(),
                            &variable_expression_tuple.1,
                        ),
                    )?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.variable_expression.read().await.contains_key(&id) {
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
    pub async fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
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
                let argument: (Arc<RwLock<Argument>>, SystemTime) =
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .argument
                    .write()
                    .await
                    .insert(argument.0.read().await.id, argument.clone());
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
                let binary: (Arc<RwLock<Binary>>, SystemTime) =
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .binary
                    .write()
                    .await
                    .insert(binary.0.read().await.id(), binary.clone());
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
                let block: (Arc<RwLock<Block>>, SystemTime) =
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .block
                    .write()
                    .await
                    .insert(block.0.read().await.id, block.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .boolean_literal
                    .write()
                    .await
                    .insert(boolean_literal.0.read().await.id(), boolean_literal.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store.boolean_operator.write().await.insert(
                    boolean_operator.0.read().await.id(),
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
                let call: (Arc<RwLock<Call>>, SystemTime) =
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .call
                    .write()
                    .await
                    .insert(call.0.read().await.id, call.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .comparison
                    .write()
                    .await
                    .insert(comparison.0.read().await.id(), comparison.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store.dwarf_source_file.write().await.insert(
                    dwarf_source_file.0.read().await.id,
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
                let error: (Arc<RwLock<Error>>, SystemTime) =
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .error
                    .write()
                    .await
                    .insert(error.0.read().await.id(), error.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .error_expression
                    .write()
                    .await
                    .insert(error_expression.0.read().await.id, error_expression.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .expression
                    .write()
                    .await
                    .insert(expression.0.read().await.id(), expression.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store.expression_statement.write().await.insert(
                    expression_statement.0.read().await.id,
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
                let field: (Arc<RwLock<Field>>, SystemTime) =
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store.field_id_by_name.write().await.insert(
                    field.0.read().await.name.to_upper_camel_case(),
                    (field.0.read().await.id, field.1),
                );
                store
                    .field
                    .write()
                    .await
                    .insert(field.0.read().await.id, field.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .field_access
                    .write()
                    .await
                    .insert(field_access.0.read().await.id, field_access.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store.field_access_target.write().await.insert(
                    field_access_target.0.read().await.id(),
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .field_expression
                    .write()
                    .await
                    .insert(field_expression.0.read().await.id, field_expression.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .float_literal
                    .write()
                    .await
                    .insert(float_literal.0.read().await.id, float_literal.clone());
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
                let for_loop: (Arc<RwLock<ForLoop>>, SystemTime) =
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .for_loop
                    .write()
                    .await
                    .insert(for_loop.0.read().await.id, for_loop.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store.function_id_by_name.write().await.insert(
                    function.0.read().await.name.to_upper_camel_case(),
                    (function.0.read().await.id, function.1),
                );
                store
                    .function
                    .write()
                    .await
                    .insert(function.0.read().await.id, function.clone());
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
                let grouped: (Arc<RwLock<Grouped>>, SystemTime) =
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .grouped
                    .write()
                    .await
                    .insert(grouped.0.read().await.id, grouped.clone());
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
                let x_if: (Arc<RwLock<XIf>>, SystemTime) =
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .x_if
                    .write()
                    .await
                    .insert(x_if.0.read().await.id, x_if.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .implementation
                    .write()
                    .await
                    .insert(implementation.0.read().await.id, implementation.clone());
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
                let import: (Arc<RwLock<Import>>, SystemTime) =
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .import
                    .write()
                    .await
                    .insert(import.0.read().await.id, import.clone());
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
                let index: (Arc<RwLock<Index>>, SystemTime) =
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .index
                    .write()
                    .await
                    .insert(index.0.read().await.id, index.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .integer_literal
                    .write()
                    .await
                    .insert(integer_literal.0.read().await.id, integer_literal.clone());
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
                let item: (Arc<RwLock<Item>>, SystemTime) =
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .item
                    .write()
                    .await
                    .insert(item.0.read().await.id, item.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .let_statement
                    .write()
                    .await
                    .insert(let_statement.0.read().await.id, let_statement.clone());
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
                let list: (Arc<RwLock<List>>, SystemTime) =
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .list
                    .write()
                    .await
                    .insert(list.0.read().await.id, list.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .list_element
                    .write()
                    .await
                    .insert(list_element.0.read().await.id, list_element.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .list_expression
                    .write()
                    .await
                    .insert(list_expression.0.read().await.id, list_expression.clone());
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
                let literal: (Arc<RwLock<Literal>>, SystemTime) =
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .literal
                    .write()
                    .await
                    .insert(literal.0.read().await.id(), literal.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .local_variable
                    .write()
                    .await
                    .insert(local_variable.0.read().await.id, local_variable.clone());
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
                let x_macro: (Arc<RwLock<XMacro>>, SystemTime) =
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .x_macro
                    .write()
                    .await
                    .insert(x_macro.0.read().await.id, x_macro.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .method_call
                    .write()
                    .await
                    .insert(method_call.0.read().await.id, method_call.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .z_object_store
                    .write()
                    .await
                    .insert(z_object_store.0.read().await.id, z_object_store.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .operator
                    .write()
                    .await
                    .insert(operator.0.read().await.id, operator.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .woog_option
                    .write()
                    .await
                    .insert(woog_option.0.read().await.id, woog_option.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .parameter
                    .write()
                    .await
                    .insert(parameter.0.read().await.id, parameter.clone());
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
                let print: (Arc<RwLock<Print>>, SystemTime) =
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .print
                    .write()
                    .await
                    .insert(print.0.read().await.id, print.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .range_expression
                    .write()
                    .await
                    .insert(range_expression.0.read().await.id, range_expression.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .reference
                    .write()
                    .await
                    .insert(reference.0.read().await.id, reference.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .result_statement
                    .write()
                    .await
                    .insert(result_statement.0.read().await.id, result_statement.clone());
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
                let x_return: (Arc<RwLock<XReturn>>, SystemTime) =
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .x_return
                    .write()
                    .await
                    .insert(x_return.0.read().await.id, x_return.clone());
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
                let z_some: (Arc<RwLock<ZSome>>, SystemTime) =
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .z_some
                    .write()
                    .await
                    .insert(z_some.0.read().await.id, z_some.clone());
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
                let span: (Arc<RwLock<Span>>, SystemTime) =
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .span
                    .write()
                    .await
                    .insert(span.0.read().await.id, span.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .statement
                    .write()
                    .await
                    .insert(statement.0.read().await.id, statement.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store.static_method_call.write().await.insert(
                    static_method_call.0.read().await.id,
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .string_literal
                    .write()
                    .await
                    .insert(string_literal.0.read().await.id, string_literal.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store.woog_struct_id_by_name.write().await.insert(
                    woog_struct.0.read().await.name.to_upper_camel_case(),
                    (woog_struct.0.read().await.id, woog_struct.1),
                );
                store
                    .woog_struct
                    .write()
                    .await
                    .insert(woog_struct.0.read().await.id, woog_struct.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store.struct_expression.write().await.insert(
                    struct_expression.0.read().await.id,
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .type_cast
                    .write()
                    .await
                    .insert(type_cast.0.read().await.id, type_cast.clone());
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
                let unary: (Arc<RwLock<Unary>>, SystemTime) =
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .unary
                    .write()
                    .await
                    .insert(unary.0.read().await.id(), unary.clone());
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
                let x_value: (Arc<RwLock<XValue>>, SystemTime) =
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .x_value
                    .write()
                    .await
                    .insert(x_value.0.read().await.id, x_value.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .value_type
                    .write()
                    .await
                    .insert(value_type.0.read().await.id(), value_type.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store
                    .variable
                    .write()
                    .await
                    .insert(variable.0.read().await.id, variable.clone());
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
                    serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;
                store.variable_expression.write().await.insert(
                    variable_expression.0.read().await.id,
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
