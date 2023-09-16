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
//! * [`Body`]
//! * [`BooleanLiteral`]
//! * [`BooleanOperator`]
//! * [`Call`]
//! * [`Comparison`]
//! * [`DwarfSourceFile`]
//! * [`EnumField`]
//! * [`Enumeration`]
//! * [`XError`]
//! * [`ErrorExpression`]
//! * [`Expression`]
//! * [`ExpressionStatement`]
//! * [`ExternalImplementation`]
//! * [`Field`]
//! * [`FieldAccess`]
//! * [`FieldAccessTarget`]
//! * [`FieldExpression`]
//! * [`FloatLiteral`]
//! * [`ForLoop`]
//! * [`Function`]
//! * [`Generic`]
//! * [`Grouped`]
//! * [`XIf`]
//! * [`ImplementationBlock`]
//! * [`Import`]
//! * [`Index`]
//! * [`IntegerLiteral`]
//! * [`Item`]
//! * [`Lambda`]
//! * [`LambdaParameter`]
//! * [`LetStatement`]
//! * [`List`]
//! * [`ListElement`]
//! * [`ListExpression`]
//! * [`Literal`]
//! * [`LocalVariable`]
//! * [`XMacro`]
//! * [`XMatch`]
//! * [`MethodCall`]
//! * [`ZObjectStore`]
//! * [`ObjectWrapper`]
//! * [`Operator`]
//! * [`WoogOption`]
//! * [`Parameter`]
//! * [`Pattern`]
//! * [`Plain`]
//! * [`XPrint`]
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
//! * [`StructField`]
//! * [`TupleField`]
//! * [`TypeCast`]
//! * [`Unary`]
//! * [`XValue`]
//! * [`ValueType`]
//! * [`Variable`]
//! * [`VariableExpression`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_async-object-store-definition"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use serde::{
    de::{self, MapAccess, Visitor},
    ser::SerializeStruct,
    Deserializer, Serializer,
};
use std::fmt;
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
};

use heck::ToUpperCamelCase;
use rustc_hash::FxHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v2::lu_dog_async::types::{
    Argument, Binary, Block, Body, BooleanLiteral, BooleanOperator, Call, Comparison,
    DwarfSourceFile, EnumField, Enumeration, ErrorExpression, Expression, ExpressionStatement,
    ExternalImplementation, Field, FieldAccess, FieldAccessTarget, FieldExpression, FloatLiteral,
    ForLoop, Function, Generic, Grouped, ImplementationBlock, Import, Index, IntegerLiteral, Item,
    Lambda, LambdaParameter, LetStatement, List, ListElement, ListExpression, Literal,
    LocalVariable, MethodCall, ObjectWrapper, Operator, Parameter, Pattern, Plain, RangeExpression,
    Reference, ResultStatement, Span, Statement, StaticMethodCall, StringLiteral, StructExpression,
    StructField, TupleField, TypeCast, Unary, ValueType, Variable, VariableExpression, WoogOption,
    WoogStruct, XError, XIf, XMacro, XMatch, XPrint, XReturn, XValue, ZObjectStore, ZSome,
    ADDITION, AND, ASSIGNMENT, CHAR, DEBUGGER, DIVISION, EMPTY, EQUAL, FALSE_LITERAL, FROM, FULL,
    FUNCTION_CALL, GREATER_THAN, GREATER_THAN_OR_EQUAL, INCLUSIVE, ITEM_STATEMENT, LESS_THAN,
    LESS_THAN_OR_EQUAL, MACRO_CALL, MULTIPLICATION, NEGATION, NOT, NOT_EQUAL, OR, RANGE,
    SUBTRACTION, TO, TO_INCLUSIVE, TRUE_LITERAL, UNKNOWN, UNKNOWN_VARIABLE, Z_NONE,
};

#[derive(Debug)]
pub struct ObjectStore {
    argument_free_list: async_std::sync::Mutex<Vec<usize>>,
    argument: Arc<RwLock<Vec<Option<Arc<RwLock<Argument>>>>>>,
    binary_free_list: async_std::sync::Mutex<Vec<usize>>,
    binary: Arc<RwLock<Vec<Option<Arc<RwLock<Binary>>>>>>,
    block_free_list: async_std::sync::Mutex<Vec<usize>>,
    block: Arc<RwLock<Vec<Option<Arc<RwLock<Block>>>>>>,
    body_free_list: async_std::sync::Mutex<Vec<usize>>,
    body: Arc<RwLock<Vec<Option<Arc<RwLock<Body>>>>>>,
    boolean_literal_free_list: async_std::sync::Mutex<Vec<usize>>,
    boolean_literal: Arc<RwLock<Vec<Option<Arc<RwLock<BooleanLiteral>>>>>>,
    boolean_operator_free_list: async_std::sync::Mutex<Vec<usize>>,
    boolean_operator: Arc<RwLock<Vec<Option<Arc<RwLock<BooleanOperator>>>>>>,
    call_free_list: async_std::sync::Mutex<Vec<usize>>,
    call: Arc<RwLock<Vec<Option<Arc<RwLock<Call>>>>>>,
    comparison_free_list: async_std::sync::Mutex<Vec<usize>>,
    comparison: Arc<RwLock<Vec<Option<Arc<RwLock<Comparison>>>>>>,
    dwarf_source_file_free_list: async_std::sync::Mutex<Vec<usize>>,
    dwarf_source_file: Arc<RwLock<Vec<Option<Arc<RwLock<DwarfSourceFile>>>>>>,
    enum_field_free_list: async_std::sync::Mutex<Vec<usize>>,
    enum_field: Arc<RwLock<Vec<Option<Arc<RwLock<EnumField>>>>>>,
    enumeration_free_list: async_std::sync::Mutex<Vec<usize>>,
    enumeration: Arc<RwLock<Vec<Option<Arc<RwLock<Enumeration>>>>>>,
    enumeration_id_by_name: Arc<RwLock<HashMap<String, usize>>>,
    x_error_free_list: async_std::sync::Mutex<Vec<usize>>,
    x_error: Arc<RwLock<Vec<Option<Arc<RwLock<XError>>>>>>,
    error_expression_free_list: async_std::sync::Mutex<Vec<usize>>,
    error_expression: Arc<RwLock<Vec<Option<Arc<RwLock<ErrorExpression>>>>>>,
    expression_free_list: async_std::sync::Mutex<Vec<usize>>,
    expression: Arc<RwLock<Vec<Option<Arc<RwLock<Expression>>>>>>,
    expression_statement_free_list: async_std::sync::Mutex<Vec<usize>>,
    expression_statement: Arc<RwLock<Vec<Option<Arc<RwLock<ExpressionStatement>>>>>>,
    external_implementation_free_list: async_std::sync::Mutex<Vec<usize>>,
    external_implementation: Arc<RwLock<Vec<Option<Arc<RwLock<ExternalImplementation>>>>>>,
    field_free_list: async_std::sync::Mutex<Vec<usize>>,
    field: Arc<RwLock<Vec<Option<Arc<RwLock<Field>>>>>>,
    field_id_by_name: Arc<RwLock<HashMap<String, usize>>>,
    field_access_free_list: async_std::sync::Mutex<Vec<usize>>,
    field_access: Arc<RwLock<Vec<Option<Arc<RwLock<FieldAccess>>>>>>,
    field_access_target_free_list: async_std::sync::Mutex<Vec<usize>>,
    field_access_target: Arc<RwLock<Vec<Option<Arc<RwLock<FieldAccessTarget>>>>>>,
    field_expression_free_list: async_std::sync::Mutex<Vec<usize>>,
    field_expression: Arc<RwLock<Vec<Option<Arc<RwLock<FieldExpression>>>>>>,
    float_literal_free_list: async_std::sync::Mutex<Vec<usize>>,
    float_literal: Arc<RwLock<Vec<Option<Arc<RwLock<FloatLiteral>>>>>>,
    for_loop_free_list: async_std::sync::Mutex<Vec<usize>>,
    for_loop: Arc<RwLock<Vec<Option<Arc<RwLock<ForLoop>>>>>>,
    function_free_list: async_std::sync::Mutex<Vec<usize>>,
    function: Arc<RwLock<Vec<Option<Arc<RwLock<Function>>>>>>,
    function_id_by_name: Arc<RwLock<HashMap<String, usize>>>,
    generic_free_list: async_std::sync::Mutex<Vec<usize>>,
    generic: Arc<RwLock<Vec<Option<Arc<RwLock<Generic>>>>>>,
    grouped_free_list: async_std::sync::Mutex<Vec<usize>>,
    grouped: Arc<RwLock<Vec<Option<Arc<RwLock<Grouped>>>>>>,
    x_if_free_list: async_std::sync::Mutex<Vec<usize>>,
    x_if: Arc<RwLock<Vec<Option<Arc<RwLock<XIf>>>>>>,
    implementation_block_free_list: async_std::sync::Mutex<Vec<usize>>,
    implementation_block: Arc<RwLock<Vec<Option<Arc<RwLock<ImplementationBlock>>>>>>,
    import_free_list: async_std::sync::Mutex<Vec<usize>>,
    import: Arc<RwLock<Vec<Option<Arc<RwLock<Import>>>>>>,
    index_free_list: async_std::sync::Mutex<Vec<usize>>,
    index: Arc<RwLock<Vec<Option<Arc<RwLock<Index>>>>>>,
    integer_literal_free_list: async_std::sync::Mutex<Vec<usize>>,
    integer_literal: Arc<RwLock<Vec<Option<Arc<RwLock<IntegerLiteral>>>>>>,
    item_free_list: async_std::sync::Mutex<Vec<usize>>,
    item: Arc<RwLock<Vec<Option<Arc<RwLock<Item>>>>>>,
    lambda_free_list: async_std::sync::Mutex<Vec<usize>>,
    lambda: Arc<RwLock<Vec<Option<Arc<RwLock<Lambda>>>>>>,
    lambda_parameter_free_list: async_std::sync::Mutex<Vec<usize>>,
    lambda_parameter: Arc<RwLock<Vec<Option<Arc<RwLock<LambdaParameter>>>>>>,
    let_statement_free_list: async_std::sync::Mutex<Vec<usize>>,
    let_statement: Arc<RwLock<Vec<Option<Arc<RwLock<LetStatement>>>>>>,
    list_free_list: async_std::sync::Mutex<Vec<usize>>,
    list: Arc<RwLock<Vec<Option<Arc<RwLock<List>>>>>>,
    list_element_free_list: async_std::sync::Mutex<Vec<usize>>,
    list_element: Arc<RwLock<Vec<Option<Arc<RwLock<ListElement>>>>>>,
    list_expression_free_list: async_std::sync::Mutex<Vec<usize>>,
    list_expression: Arc<RwLock<Vec<Option<Arc<RwLock<ListExpression>>>>>>,
    literal_free_list: async_std::sync::Mutex<Vec<usize>>,
    literal: Arc<RwLock<Vec<Option<Arc<RwLock<Literal>>>>>>,
    local_variable_free_list: async_std::sync::Mutex<Vec<usize>>,
    local_variable: Arc<RwLock<Vec<Option<Arc<RwLock<LocalVariable>>>>>>,
    x_macro_free_list: async_std::sync::Mutex<Vec<usize>>,
    x_macro: Arc<RwLock<Vec<Option<Arc<RwLock<XMacro>>>>>>,
    x_match_free_list: async_std::sync::Mutex<Vec<usize>>,
    x_match: Arc<RwLock<Vec<Option<Arc<RwLock<XMatch>>>>>>,
    method_call_free_list: async_std::sync::Mutex<Vec<usize>>,
    method_call: Arc<RwLock<Vec<Option<Arc<RwLock<MethodCall>>>>>>,
    z_object_store_free_list: async_std::sync::Mutex<Vec<usize>>,
    z_object_store: Arc<RwLock<Vec<Option<Arc<RwLock<ZObjectStore>>>>>>,
    z_object_store_id_by_name: Arc<RwLock<HashMap<String, usize>>>,
    object_wrapper_free_list: async_std::sync::Mutex<Vec<usize>>,
    object_wrapper: Arc<RwLock<Vec<Option<Arc<RwLock<ObjectWrapper>>>>>>,
    operator_free_list: async_std::sync::Mutex<Vec<usize>>,
    operator: Arc<RwLock<Vec<Option<Arc<RwLock<Operator>>>>>>,
    woog_option_free_list: async_std::sync::Mutex<Vec<usize>>,
    woog_option: Arc<RwLock<Vec<Option<Arc<RwLock<WoogOption>>>>>>,
    parameter_free_list: async_std::sync::Mutex<Vec<usize>>,
    parameter: Arc<RwLock<Vec<Option<Arc<RwLock<Parameter>>>>>>,
    pattern_free_list: async_std::sync::Mutex<Vec<usize>>,
    pattern: Arc<RwLock<Vec<Option<Arc<RwLock<Pattern>>>>>>,
    plain_free_list: async_std::sync::Mutex<Vec<usize>>,
    plain: Arc<RwLock<Vec<Option<Arc<RwLock<Plain>>>>>>,
    x_print_free_list: async_std::sync::Mutex<Vec<usize>>,
    x_print: Arc<RwLock<Vec<Option<Arc<RwLock<XPrint>>>>>>,
    range_expression_free_list: async_std::sync::Mutex<Vec<usize>>,
    range_expression: Arc<RwLock<Vec<Option<Arc<RwLock<RangeExpression>>>>>>,
    reference_free_list: async_std::sync::Mutex<Vec<usize>>,
    reference: Arc<RwLock<Vec<Option<Arc<RwLock<Reference>>>>>>,
    result_statement_free_list: async_std::sync::Mutex<Vec<usize>>,
    result_statement: Arc<RwLock<Vec<Option<Arc<RwLock<ResultStatement>>>>>>,
    x_return_free_list: async_std::sync::Mutex<Vec<usize>>,
    x_return: Arc<RwLock<Vec<Option<Arc<RwLock<XReturn>>>>>>,
    z_some_free_list: async_std::sync::Mutex<Vec<usize>>,
    z_some: Arc<RwLock<Vec<Option<Arc<RwLock<ZSome>>>>>>,
    span_free_list: async_std::sync::Mutex<Vec<usize>>,
    span: Arc<RwLock<Vec<Option<Arc<RwLock<Span>>>>>>,
    statement_free_list: async_std::sync::Mutex<Vec<usize>>,
    statement: Arc<RwLock<Vec<Option<Arc<RwLock<Statement>>>>>>,
    static_method_call_free_list: async_std::sync::Mutex<Vec<usize>>,
    static_method_call: Arc<RwLock<Vec<Option<Arc<RwLock<StaticMethodCall>>>>>>,
    string_literal_free_list: async_std::sync::Mutex<Vec<usize>>,
    string_literal: Arc<RwLock<Vec<Option<Arc<RwLock<StringLiteral>>>>>>,
    woog_struct_free_list: async_std::sync::Mutex<Vec<usize>>,
    woog_struct: Arc<RwLock<Vec<Option<Arc<RwLock<WoogStruct>>>>>>,
    woog_struct_id_by_name: Arc<RwLock<HashMap<String, usize>>>,
    struct_expression_free_list: async_std::sync::Mutex<Vec<usize>>,
    struct_expression: Arc<RwLock<Vec<Option<Arc<RwLock<StructExpression>>>>>>,
    struct_field_free_list: async_std::sync::Mutex<Vec<usize>>,
    struct_field: Arc<RwLock<Vec<Option<Arc<RwLock<StructField>>>>>>,
    tuple_field_free_list: async_std::sync::Mutex<Vec<usize>>,
    tuple_field: Arc<RwLock<Vec<Option<Arc<RwLock<TupleField>>>>>>,
    type_cast_free_list: async_std::sync::Mutex<Vec<usize>>,
    type_cast: Arc<RwLock<Vec<Option<Arc<RwLock<TypeCast>>>>>>,
    unary_free_list: async_std::sync::Mutex<Vec<usize>>,
    unary: Arc<RwLock<Vec<Option<Arc<RwLock<Unary>>>>>>,
    x_value_free_list: async_std::sync::Mutex<Vec<usize>>,
    x_value: Arc<RwLock<Vec<Option<Arc<RwLock<XValue>>>>>>,
    value_type_free_list: async_std::sync::Mutex<Vec<usize>>,
    value_type: Arc<RwLock<Vec<Option<Arc<RwLock<ValueType>>>>>>,
    variable_free_list: async_std::sync::Mutex<Vec<usize>>,
    variable: Arc<RwLock<Vec<Option<Arc<RwLock<Variable>>>>>>,
    variable_expression_free_list: async_std::sync::Mutex<Vec<usize>>,
    variable_expression: Arc<RwLock<Vec<Option<Arc<RwLock<VariableExpression>>>>>>,
}

impl Serialize for ObjectStore {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_struct("ObjectStore", 69)?;

        let argument = futures::executor::block_on(async { self.argument.read().await }).clone();
        let values: Vec<Argument> = argument
            .into_iter()
            .filter_map(|argument| {
                if let Some(argument) = argument {
                    Some(futures::executor::block_on(async { argument.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("argument", &values)?;

        let binary = futures::executor::block_on(async { self.binary.read().await }).clone();
        let values: Vec<Binary> = binary
            .into_iter()
            .filter_map(|binary| {
                if let Some(binary) = binary {
                    Some(futures::executor::block_on(async { binary.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("binary", &values)?;

        let block = futures::executor::block_on(async { self.block.read().await }).clone();
        let values: Vec<Block> = block
            .into_iter()
            .filter_map(|block| {
                if let Some(block) = block {
                    Some(futures::executor::block_on(async { block.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("block", &values)?;

        let body = futures::executor::block_on(async { self.body.read().await }).clone();
        let values: Vec<Body> = body
            .into_iter()
            .filter_map(|body| {
                if let Some(body) = body {
                    Some(futures::executor::block_on(async { body.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("body", &values)?;

        let boolean_literal =
            futures::executor::block_on(async { self.boolean_literal.read().await }).clone();
        let values: Vec<BooleanLiteral> = boolean_literal
            .into_iter()
            .filter_map(|boolean_literal| {
                if let Some(boolean_literal) = boolean_literal {
                    Some(
                        futures::executor::block_on(async { boolean_literal.read().await }).clone(),
                    )
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("boolean_literal", &values)?;

        let boolean_operator =
            futures::executor::block_on(async { self.boolean_operator.read().await }).clone();
        let values: Vec<BooleanOperator> = boolean_operator
            .into_iter()
            .filter_map(|boolean_operator| {
                if let Some(boolean_operator) = boolean_operator {
                    Some(
                        futures::executor::block_on(async { boolean_operator.read().await })
                            .clone(),
                    )
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("boolean_operator", &values)?;

        let call = futures::executor::block_on(async { self.call.read().await }).clone();
        let values: Vec<Call> = call
            .into_iter()
            .filter_map(|call| {
                if let Some(call) = call {
                    Some(futures::executor::block_on(async { call.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("call", &values)?;

        let comparison =
            futures::executor::block_on(async { self.comparison.read().await }).clone();
        let values: Vec<Comparison> = comparison
            .into_iter()
            .filter_map(|comparison| {
                if let Some(comparison) = comparison {
                    Some(futures::executor::block_on(async { comparison.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("comparison", &values)?;

        let dwarf_source_file =
            futures::executor::block_on(async { self.dwarf_source_file.read().await }).clone();
        let values: Vec<DwarfSourceFile> = dwarf_source_file
            .into_iter()
            .filter_map(|dwarf_source_file| {
                if let Some(dwarf_source_file) = dwarf_source_file {
                    Some(
                        futures::executor::block_on(async { dwarf_source_file.read().await })
                            .clone(),
                    )
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("dwarf_source_file", &values)?;

        let enum_field =
            futures::executor::block_on(async { self.enum_field.read().await }).clone();
        let values: Vec<EnumField> = enum_field
            .into_iter()
            .filter_map(|enum_field| {
                if let Some(enum_field) = enum_field {
                    Some(futures::executor::block_on(async { enum_field.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("enum_field", &values)?;

        let enumeration =
            futures::executor::block_on(async { self.enumeration.read().await }).clone();
        let values: Vec<Enumeration> = enumeration
            .into_iter()
            .filter_map(|enumeration| {
                if let Some(enumeration) = enumeration {
                    Some(futures::executor::block_on(async { enumeration.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("enumeration", &values)?;

        let x_error = futures::executor::block_on(async { self.x_error.read().await }).clone();
        let values: Vec<XError> = x_error
            .into_iter()
            .filter_map(|x_error| {
                if let Some(x_error) = x_error {
                    Some(futures::executor::block_on(async { x_error.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("x_error", &values)?;

        let error_expression =
            futures::executor::block_on(async { self.error_expression.read().await }).clone();
        let values: Vec<ErrorExpression> = error_expression
            .into_iter()
            .filter_map(|error_expression| {
                if let Some(error_expression) = error_expression {
                    Some(
                        futures::executor::block_on(async { error_expression.read().await })
                            .clone(),
                    )
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("error_expression", &values)?;

        let expression =
            futures::executor::block_on(async { self.expression.read().await }).clone();
        let values: Vec<Expression> = expression
            .into_iter()
            .filter_map(|expression| {
                if let Some(expression) = expression {
                    Some(futures::executor::block_on(async { expression.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("expression", &values)?;

        let expression_statement =
            futures::executor::block_on(async { self.expression_statement.read().await }).clone();
        let values: Vec<ExpressionStatement> = expression_statement
            .into_iter()
            .filter_map(|expression_statement| {
                if let Some(expression_statement) = expression_statement {
                    Some(
                        futures::executor::block_on(async { expression_statement.read().await })
                            .clone(),
                    )
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("expression_statement", &values)?;

        let external_implementation =
            futures::executor::block_on(async { self.external_implementation.read().await })
                .clone();
        let values: Vec<ExternalImplementation> = external_implementation
            .into_iter()
            .filter_map(|external_implementation| {
                if let Some(external_implementation) = external_implementation {
                    Some(
                        futures::executor::block_on(async { external_implementation.read().await })
                            .clone(),
                    )
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("external_implementation", &values)?;

        let field = futures::executor::block_on(async { self.field.read().await }).clone();
        let values: Vec<Field> = field
            .into_iter()
            .filter_map(|field| {
                if let Some(field) = field {
                    Some(futures::executor::block_on(async { field.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("field", &values)?;

        let field_access =
            futures::executor::block_on(async { self.field_access.read().await }).clone();
        let values: Vec<FieldAccess> = field_access
            .into_iter()
            .filter_map(|field_access| {
                if let Some(field_access) = field_access {
                    Some(futures::executor::block_on(async { field_access.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("field_access", &values)?;

        let field_access_target =
            futures::executor::block_on(async { self.field_access_target.read().await }).clone();
        let values: Vec<FieldAccessTarget> = field_access_target
            .into_iter()
            .filter_map(|field_access_target| {
                if let Some(field_access_target) = field_access_target {
                    Some(
                        futures::executor::block_on(async { field_access_target.read().await })
                            .clone(),
                    )
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("field_access_target", &values)?;

        let field_expression =
            futures::executor::block_on(async { self.field_expression.read().await }).clone();
        let values: Vec<FieldExpression> = field_expression
            .into_iter()
            .filter_map(|field_expression| {
                if let Some(field_expression) = field_expression {
                    Some(
                        futures::executor::block_on(async { field_expression.read().await })
                            .clone(),
                    )
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("field_expression", &values)?;

        let float_literal =
            futures::executor::block_on(async { self.float_literal.read().await }).clone();
        let values: Vec<FloatLiteral> = float_literal
            .into_iter()
            .filter_map(|float_literal| {
                if let Some(float_literal) = float_literal {
                    Some(futures::executor::block_on(async { float_literal.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("float_literal", &values)?;

        let for_loop = futures::executor::block_on(async { self.for_loop.read().await }).clone();
        let values: Vec<ForLoop> = for_loop
            .into_iter()
            .filter_map(|for_loop| {
                if let Some(for_loop) = for_loop {
                    Some(futures::executor::block_on(async { for_loop.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("for_loop", &values)?;

        let function = futures::executor::block_on(async { self.function.read().await }).clone();
        let values: Vec<Function> = function
            .into_iter()
            .filter_map(|function| {
                if let Some(function) = function {
                    Some(futures::executor::block_on(async { function.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("function", &values)?;

        let generic = futures::executor::block_on(async { self.generic.read().await }).clone();
        let values: Vec<Generic> = generic
            .into_iter()
            .filter_map(|generic| {
                if let Some(generic) = generic {
                    Some(futures::executor::block_on(async { generic.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("generic", &values)?;

        let grouped = futures::executor::block_on(async { self.grouped.read().await }).clone();
        let values: Vec<Grouped> = grouped
            .into_iter()
            .filter_map(|grouped| {
                if let Some(grouped) = grouped {
                    Some(futures::executor::block_on(async { grouped.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("grouped", &values)?;

        let x_if = futures::executor::block_on(async { self.x_if.read().await }).clone();
        let values: Vec<XIf> = x_if
            .into_iter()
            .filter_map(|x_if| {
                if let Some(x_if) = x_if {
                    Some(futures::executor::block_on(async { x_if.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("x_if", &values)?;

        let implementation_block =
            futures::executor::block_on(async { self.implementation_block.read().await }).clone();
        let values: Vec<ImplementationBlock> = implementation_block
            .into_iter()
            .filter_map(|implementation_block| {
                if let Some(implementation_block) = implementation_block {
                    Some(
                        futures::executor::block_on(async { implementation_block.read().await })
                            .clone(),
                    )
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("implementation_block", &values)?;

        let import = futures::executor::block_on(async { self.import.read().await }).clone();
        let values: Vec<Import> = import
            .into_iter()
            .filter_map(|import| {
                if let Some(import) = import {
                    Some(futures::executor::block_on(async { import.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("import", &values)?;

        let index = futures::executor::block_on(async { self.index.read().await }).clone();
        let values: Vec<Index> = index
            .into_iter()
            .filter_map(|index| {
                if let Some(index) = index {
                    Some(futures::executor::block_on(async { index.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("index", &values)?;

        let integer_literal =
            futures::executor::block_on(async { self.integer_literal.read().await }).clone();
        let values: Vec<IntegerLiteral> = integer_literal
            .into_iter()
            .filter_map(|integer_literal| {
                if let Some(integer_literal) = integer_literal {
                    Some(
                        futures::executor::block_on(async { integer_literal.read().await }).clone(),
                    )
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("integer_literal", &values)?;

        let item = futures::executor::block_on(async { self.item.read().await }).clone();
        let values: Vec<Item> = item
            .into_iter()
            .filter_map(|item| {
                if let Some(item) = item {
                    Some(futures::executor::block_on(async { item.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("item", &values)?;

        let lambda = futures::executor::block_on(async { self.lambda.read().await }).clone();
        let values: Vec<Lambda> = lambda
            .into_iter()
            .filter_map(|lambda| {
                if let Some(lambda) = lambda {
                    Some(futures::executor::block_on(async { lambda.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("lambda", &values)?;

        let lambda_parameter =
            futures::executor::block_on(async { self.lambda_parameter.read().await }).clone();
        let values: Vec<LambdaParameter> = lambda_parameter
            .into_iter()
            .filter_map(|lambda_parameter| {
                if let Some(lambda_parameter) = lambda_parameter {
                    Some(
                        futures::executor::block_on(async { lambda_parameter.read().await })
                            .clone(),
                    )
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("lambda_parameter", &values)?;

        let let_statement =
            futures::executor::block_on(async { self.let_statement.read().await }).clone();
        let values: Vec<LetStatement> = let_statement
            .into_iter()
            .filter_map(|let_statement| {
                if let Some(let_statement) = let_statement {
                    Some(futures::executor::block_on(async { let_statement.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("let_statement", &values)?;

        let list = futures::executor::block_on(async { self.list.read().await }).clone();
        let values: Vec<List> = list
            .into_iter()
            .filter_map(|list| {
                if let Some(list) = list {
                    Some(futures::executor::block_on(async { list.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("list", &values)?;

        let list_element =
            futures::executor::block_on(async { self.list_element.read().await }).clone();
        let values: Vec<ListElement> = list_element
            .into_iter()
            .filter_map(|list_element| {
                if let Some(list_element) = list_element {
                    Some(futures::executor::block_on(async { list_element.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("list_element", &values)?;

        let list_expression =
            futures::executor::block_on(async { self.list_expression.read().await }).clone();
        let values: Vec<ListExpression> = list_expression
            .into_iter()
            .filter_map(|list_expression| {
                if let Some(list_expression) = list_expression {
                    Some(
                        futures::executor::block_on(async { list_expression.read().await }).clone(),
                    )
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("list_expression", &values)?;

        let literal = futures::executor::block_on(async { self.literal.read().await }).clone();
        let values: Vec<Literal> = literal
            .into_iter()
            .filter_map(|literal| {
                if let Some(literal) = literal {
                    Some(futures::executor::block_on(async { literal.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("literal", &values)?;

        let local_variable =
            futures::executor::block_on(async { self.local_variable.read().await }).clone();
        let values: Vec<LocalVariable> = local_variable
            .into_iter()
            .filter_map(|local_variable| {
                if let Some(local_variable) = local_variable {
                    Some(futures::executor::block_on(async { local_variable.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("local_variable", &values)?;

        let x_macro = futures::executor::block_on(async { self.x_macro.read().await }).clone();
        let values: Vec<XMacro> = x_macro
            .into_iter()
            .filter_map(|x_macro| {
                if let Some(x_macro) = x_macro {
                    Some(futures::executor::block_on(async { x_macro.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("x_macro", &values)?;

        let x_match = futures::executor::block_on(async { self.x_match.read().await }).clone();
        let values: Vec<XMatch> = x_match
            .into_iter()
            .filter_map(|x_match| {
                if let Some(x_match) = x_match {
                    Some(futures::executor::block_on(async { x_match.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("x_match", &values)?;

        let method_call =
            futures::executor::block_on(async { self.method_call.read().await }).clone();
        let values: Vec<MethodCall> = method_call
            .into_iter()
            .filter_map(|method_call| {
                if let Some(method_call) = method_call {
                    Some(futures::executor::block_on(async { method_call.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("method_call", &values)?;

        let z_object_store =
            futures::executor::block_on(async { self.z_object_store.read().await }).clone();
        let values: Vec<ZObjectStore> = z_object_store
            .into_iter()
            .filter_map(|z_object_store| {
                if let Some(z_object_store) = z_object_store {
                    Some(futures::executor::block_on(async { z_object_store.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("z_object_store", &values)?;

        let object_wrapper =
            futures::executor::block_on(async { self.object_wrapper.read().await }).clone();
        let values: Vec<ObjectWrapper> = object_wrapper
            .into_iter()
            .filter_map(|object_wrapper| {
                if let Some(object_wrapper) = object_wrapper {
                    Some(futures::executor::block_on(async { object_wrapper.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("object_wrapper", &values)?;

        let operator = futures::executor::block_on(async { self.operator.read().await }).clone();
        let values: Vec<Operator> = operator
            .into_iter()
            .filter_map(|operator| {
                if let Some(operator) = operator {
                    Some(futures::executor::block_on(async { operator.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("operator", &values)?;

        let woog_option =
            futures::executor::block_on(async { self.woog_option.read().await }).clone();
        let values: Vec<WoogOption> = woog_option
            .into_iter()
            .filter_map(|woog_option| {
                if let Some(woog_option) = woog_option {
                    Some(futures::executor::block_on(async { woog_option.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("woog_option", &values)?;

        let parameter = futures::executor::block_on(async { self.parameter.read().await }).clone();
        let values: Vec<Parameter> = parameter
            .into_iter()
            .filter_map(|parameter| {
                if let Some(parameter) = parameter {
                    Some(futures::executor::block_on(async { parameter.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("parameter", &values)?;

        let pattern = futures::executor::block_on(async { self.pattern.read().await }).clone();
        let values: Vec<Pattern> = pattern
            .into_iter()
            .filter_map(|pattern| {
                if let Some(pattern) = pattern {
                    Some(futures::executor::block_on(async { pattern.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("pattern", &values)?;

        let plain = futures::executor::block_on(async { self.plain.read().await }).clone();
        let values: Vec<Plain> = plain
            .into_iter()
            .filter_map(|plain| {
                if let Some(plain) = plain {
                    Some(futures::executor::block_on(async { plain.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("plain", &values)?;

        let x_print = futures::executor::block_on(async { self.x_print.read().await }).clone();
        let values: Vec<XPrint> = x_print
            .into_iter()
            .filter_map(|x_print| {
                if let Some(x_print) = x_print {
                    Some(futures::executor::block_on(async { x_print.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("x_print", &values)?;

        let range_expression =
            futures::executor::block_on(async { self.range_expression.read().await }).clone();
        let values: Vec<RangeExpression> = range_expression
            .into_iter()
            .filter_map(|range_expression| {
                if let Some(range_expression) = range_expression {
                    Some(
                        futures::executor::block_on(async { range_expression.read().await })
                            .clone(),
                    )
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("range_expression", &values)?;

        let reference = futures::executor::block_on(async { self.reference.read().await }).clone();
        let values: Vec<Reference> = reference
            .into_iter()
            .filter_map(|reference| {
                if let Some(reference) = reference {
                    Some(futures::executor::block_on(async { reference.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("reference", &values)?;

        let result_statement =
            futures::executor::block_on(async { self.result_statement.read().await }).clone();
        let values: Vec<ResultStatement> = result_statement
            .into_iter()
            .filter_map(|result_statement| {
                if let Some(result_statement) = result_statement {
                    Some(
                        futures::executor::block_on(async { result_statement.read().await })
                            .clone(),
                    )
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("result_statement", &values)?;

        let x_return = futures::executor::block_on(async { self.x_return.read().await }).clone();
        let values: Vec<XReturn> = x_return
            .into_iter()
            .filter_map(|x_return| {
                if let Some(x_return) = x_return {
                    Some(futures::executor::block_on(async { x_return.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("x_return", &values)?;

        let z_some = futures::executor::block_on(async { self.z_some.read().await }).clone();
        let values: Vec<ZSome> = z_some
            .into_iter()
            .filter_map(|z_some| {
                if let Some(z_some) = z_some {
                    Some(futures::executor::block_on(async { z_some.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("z_some", &values)?;

        let span = futures::executor::block_on(async { self.span.read().await }).clone();
        let values: Vec<Span> = span
            .into_iter()
            .filter_map(|span| {
                if let Some(span) = span {
                    Some(futures::executor::block_on(async { span.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("span", &values)?;

        let statement = futures::executor::block_on(async { self.statement.read().await }).clone();
        let values: Vec<Statement> = statement
            .into_iter()
            .filter_map(|statement| {
                if let Some(statement) = statement {
                    Some(futures::executor::block_on(async { statement.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("statement", &values)?;

        let static_method_call =
            futures::executor::block_on(async { self.static_method_call.read().await }).clone();
        let values: Vec<StaticMethodCall> = static_method_call
            .into_iter()
            .filter_map(|static_method_call| {
                if let Some(static_method_call) = static_method_call {
                    Some(
                        futures::executor::block_on(async { static_method_call.read().await })
                            .clone(),
                    )
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("static_method_call", &values)?;

        let string_literal =
            futures::executor::block_on(async { self.string_literal.read().await }).clone();
        let values: Vec<StringLiteral> = string_literal
            .into_iter()
            .filter_map(|string_literal| {
                if let Some(string_literal) = string_literal {
                    Some(futures::executor::block_on(async { string_literal.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("string_literal", &values)?;

        let woog_struct =
            futures::executor::block_on(async { self.woog_struct.read().await }).clone();
        let values: Vec<WoogStruct> = woog_struct
            .into_iter()
            .filter_map(|woog_struct| {
                if let Some(woog_struct) = woog_struct {
                    Some(futures::executor::block_on(async { woog_struct.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("woog_struct", &values)?;

        let struct_expression =
            futures::executor::block_on(async { self.struct_expression.read().await }).clone();
        let values: Vec<StructExpression> = struct_expression
            .into_iter()
            .filter_map(|struct_expression| {
                if let Some(struct_expression) = struct_expression {
                    Some(
                        futures::executor::block_on(async { struct_expression.read().await })
                            .clone(),
                    )
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("struct_expression", &values)?;

        let struct_field =
            futures::executor::block_on(async { self.struct_field.read().await }).clone();
        let values: Vec<StructField> = struct_field
            .into_iter()
            .filter_map(|struct_field| {
                if let Some(struct_field) = struct_field {
                    Some(futures::executor::block_on(async { struct_field.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("struct_field", &values)?;

        let tuple_field =
            futures::executor::block_on(async { self.tuple_field.read().await }).clone();
        let values: Vec<TupleField> = tuple_field
            .into_iter()
            .filter_map(|tuple_field| {
                if let Some(tuple_field) = tuple_field {
                    Some(futures::executor::block_on(async { tuple_field.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("tuple_field", &values)?;

        let type_cast = futures::executor::block_on(async { self.type_cast.read().await }).clone();
        let values: Vec<TypeCast> = type_cast
            .into_iter()
            .filter_map(|type_cast| {
                if let Some(type_cast) = type_cast {
                    Some(futures::executor::block_on(async { type_cast.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("type_cast", &values)?;

        let unary = futures::executor::block_on(async { self.unary.read().await }).clone();
        let values: Vec<Unary> = unary
            .into_iter()
            .filter_map(|unary| {
                if let Some(unary) = unary {
                    Some(futures::executor::block_on(async { unary.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("unary", &values)?;

        let x_value = futures::executor::block_on(async { self.x_value.read().await }).clone();
        let values: Vec<XValue> = x_value
            .into_iter()
            .filter_map(|x_value| {
                if let Some(x_value) = x_value {
                    Some(futures::executor::block_on(async { x_value.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("x_value", &values)?;

        let value_type =
            futures::executor::block_on(async { self.value_type.read().await }).clone();
        let values: Vec<ValueType> = value_type
            .into_iter()
            .filter_map(|value_type| {
                if let Some(value_type) = value_type {
                    Some(futures::executor::block_on(async { value_type.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("value_type", &values)?;

        let variable = futures::executor::block_on(async { self.variable.read().await }).clone();
        let values: Vec<Variable> = variable
            .into_iter()
            .filter_map(|variable| {
                if let Some(variable) = variable {
                    Some(futures::executor::block_on(async { variable.read().await }).clone())
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("variable", &values)?;

        let variable_expression =
            futures::executor::block_on(async { self.variable_expression.read().await }).clone();
        let values: Vec<VariableExpression> = variable_expression
            .into_iter()
            .filter_map(|variable_expression| {
                if let Some(variable_expression) = variable_expression {
                    Some(
                        futures::executor::block_on(async { variable_expression.read().await })
                            .clone(),
                    )
                } else {
                    None
                }
            })
            .collect();
        map.serialize_field("variable_expression", &values)?;

        map.end()
    }
}

impl<'de> Deserialize<'de> for ObjectStore {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum SerdeField {
            Argument,
            Binary,
            Block,
            Body,
            BooleanLiteral,
            BooleanOperator,
            Call,
            Comparison,
            DwarfSourceFile,
            EnumField,
            Enumeration,
            XError,
            ErrorExpression,
            Expression,
            ExpressionStatement,
            ExternalImplementation,
            Field,
            FieldAccess,
            FieldAccessTarget,
            FieldExpression,
            FloatLiteral,
            ForLoop,
            Function,
            Generic,
            Grouped,
            XIf,
            ImplementationBlock,
            Import,
            Index,
            IntegerLiteral,
            Item,
            Lambda,
            LambdaParameter,
            LetStatement,
            List,
            ListElement,
            ListExpression,
            Literal,
            LocalVariable,
            XMacro,
            XMatch,
            MethodCall,
            ZObjectStore,
            ObjectWrapper,
            Operator,
            WoogOption,
            Parameter,
            Pattern,
            Plain,
            XPrint,
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
            StructField,
            TupleField,
            TypeCast,
            Unary,
            XValue,
            ValueType,
            Variable,
            VariableExpression,
        }
        impl<'de> Deserialize<'de> for SerdeField {
            fn deserialize<D>(deserializer: D) -> Result<SerdeField, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;
                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = SerdeField;
                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("field identifier")
                    }
                    fn visit_str<E>(self, value: &str) -> Result<SerdeField, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "argument" => Ok(SerdeField::Argument),
                            "binary" => Ok(SerdeField::Binary),
                            "block" => Ok(SerdeField::Block),
                            "body" => Ok(SerdeField::Body),
                            "boolean_literal" => Ok(SerdeField::BooleanLiteral),
                            "boolean_operator" => Ok(SerdeField::BooleanOperator),
                            "call" => Ok(SerdeField::Call),
                            "comparison" => Ok(SerdeField::Comparison),
                            "dwarf_source_file" => Ok(SerdeField::DwarfSourceFile),
                            "enum_field" => Ok(SerdeField::EnumField),
                            "enumeration" => Ok(SerdeField::Enumeration),
                            "x_error" => Ok(SerdeField::XError),
                            "error_expression" => Ok(SerdeField::ErrorExpression),
                            "expression" => Ok(SerdeField::Expression),
                            "expression_statement" => Ok(SerdeField::ExpressionStatement),
                            "external_implementation" => Ok(SerdeField::ExternalImplementation),
                            "field" => Ok(SerdeField::Field),
                            "field_access" => Ok(SerdeField::FieldAccess),
                            "field_access_target" => Ok(SerdeField::FieldAccessTarget),
                            "field_expression" => Ok(SerdeField::FieldExpression),
                            "float_literal" => Ok(SerdeField::FloatLiteral),
                            "for_loop" => Ok(SerdeField::ForLoop),
                            "function" => Ok(SerdeField::Function),
                            "generic" => Ok(SerdeField::Generic),
                            "grouped" => Ok(SerdeField::Grouped),
                            "x_if" => Ok(SerdeField::XIf),
                            "implementation_block" => Ok(SerdeField::ImplementationBlock),
                            "import" => Ok(SerdeField::Import),
                            "index" => Ok(SerdeField::Index),
                            "integer_literal" => Ok(SerdeField::IntegerLiteral),
                            "item" => Ok(SerdeField::Item),
                            "lambda" => Ok(SerdeField::Lambda),
                            "lambda_parameter" => Ok(SerdeField::LambdaParameter),
                            "let_statement" => Ok(SerdeField::LetStatement),
                            "list" => Ok(SerdeField::List),
                            "list_element" => Ok(SerdeField::ListElement),
                            "list_expression" => Ok(SerdeField::ListExpression),
                            "literal" => Ok(SerdeField::Literal),
                            "local_variable" => Ok(SerdeField::LocalVariable),
                            "x_macro" => Ok(SerdeField::XMacro),
                            "x_match" => Ok(SerdeField::XMatch),
                            "method_call" => Ok(SerdeField::MethodCall),
                            "z_object_store" => Ok(SerdeField::ZObjectStore),
                            "object_wrapper" => Ok(SerdeField::ObjectWrapper),
                            "operator" => Ok(SerdeField::Operator),
                            "woog_option" => Ok(SerdeField::WoogOption),
                            "parameter" => Ok(SerdeField::Parameter),
                            "pattern" => Ok(SerdeField::Pattern),
                            "plain" => Ok(SerdeField::Plain),
                            "x_print" => Ok(SerdeField::XPrint),
                            "range_expression" => Ok(SerdeField::RangeExpression),
                            "reference" => Ok(SerdeField::Reference),
                            "result_statement" => Ok(SerdeField::ResultStatement),
                            "x_return" => Ok(SerdeField::XReturn),
                            "z_some" => Ok(SerdeField::ZSome),
                            "span" => Ok(SerdeField::Span),
                            "statement" => Ok(SerdeField::Statement),
                            "static_method_call" => Ok(SerdeField::StaticMethodCall),
                            "string_literal" => Ok(SerdeField::StringLiteral),
                            "woog_struct" => Ok(SerdeField::WoogStruct),
                            "struct_expression" => Ok(SerdeField::StructExpression),
                            "struct_field" => Ok(SerdeField::StructField),
                            "tuple_field" => Ok(SerdeField::TupleField),
                            "type_cast" => Ok(SerdeField::TypeCast),
                            "unary" => Ok(SerdeField::Unary),
                            "x_value" => Ok(SerdeField::XValue),
                            "value_type" => Ok(SerdeField::ValueType),
                            "variable" => Ok(SerdeField::Variable),
                            "variable_expression" => Ok(SerdeField::VariableExpression),
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
                let result = ObjectStore::new();
                let mut result = futures::executor::block_on(async { result.await });
                while let Some(key) = map.next_key()? {
                    match key {
                        SerdeField::Argument => {
                            let mut guard = futures::executor::block_on(result.argument.write());
                            let values: Vec<Argument> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::Binary => {
                            let mut guard = futures::executor::block_on(result.binary.write());
                            let values: Vec<Binary> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::Block => {
                            let mut guard = futures::executor::block_on(result.block.write());
                            let values: Vec<Block> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::Body => {
                            let mut guard = futures::executor::block_on(result.body.write());
                            let values: Vec<Body> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::BooleanLiteral => {
                            let mut guard =
                                futures::executor::block_on(result.boolean_literal.write());
                            let values: Vec<BooleanLiteral> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::BooleanOperator => {
                            let mut guard =
                                futures::executor::block_on(result.boolean_operator.write());
                            let values: Vec<BooleanOperator> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::Call => {
                            let mut guard = futures::executor::block_on(result.call.write());
                            let values: Vec<Call> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::Comparison => {
                            let mut guard = futures::executor::block_on(result.comparison.write());
                            let values: Vec<Comparison> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::DwarfSourceFile => {
                            let mut guard =
                                futures::executor::block_on(result.dwarf_source_file.write());
                            let values: Vec<DwarfSourceFile> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::EnumField => {
                            let mut guard = futures::executor::block_on(result.enum_field.write());
                            let values: Vec<EnumField> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::Enumeration => {
                            let mut guard = futures::executor::block_on(result.enumeration.write());
                            let values: Vec<Enumeration> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::XError => {
                            let mut guard = futures::executor::block_on(result.x_error.write());
                            let values: Vec<XError> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::ErrorExpression => {
                            let mut guard =
                                futures::executor::block_on(result.error_expression.write());
                            let values: Vec<ErrorExpression> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::Expression => {
                            let mut guard = futures::executor::block_on(result.expression.write());
                            let values: Vec<Expression> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::ExpressionStatement => {
                            let mut guard =
                                futures::executor::block_on(result.expression_statement.write());
                            let values: Vec<ExpressionStatement> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::ExternalImplementation => {
                            let mut guard =
                                futures::executor::block_on(result.external_implementation.write());
                            let values: Vec<ExternalImplementation> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::Field => {
                            let mut guard = futures::executor::block_on(result.field.write());
                            let values: Vec<Field> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::FieldAccess => {
                            let mut guard =
                                futures::executor::block_on(result.field_access.write());
                            let values: Vec<FieldAccess> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::FieldAccessTarget => {
                            let mut guard =
                                futures::executor::block_on(result.field_access_target.write());
                            let values: Vec<FieldAccessTarget> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::FieldExpression => {
                            let mut guard =
                                futures::executor::block_on(result.field_expression.write());
                            let values: Vec<FieldExpression> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::FloatLiteral => {
                            let mut guard =
                                futures::executor::block_on(result.float_literal.write());
                            let values: Vec<FloatLiteral> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::ForLoop => {
                            let mut guard = futures::executor::block_on(result.for_loop.write());
                            let values: Vec<ForLoop> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::Function => {
                            let mut guard = futures::executor::block_on(result.function.write());
                            let values: Vec<Function> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::Generic => {
                            let mut guard = futures::executor::block_on(result.generic.write());
                            let values: Vec<Generic> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::Grouped => {
                            let mut guard = futures::executor::block_on(result.grouped.write());
                            let values: Vec<Grouped> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::XIf => {
                            let mut guard = futures::executor::block_on(result.x_if.write());
                            let values: Vec<XIf> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::ImplementationBlock => {
                            let mut guard =
                                futures::executor::block_on(result.implementation_block.write());
                            let values: Vec<ImplementationBlock> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::Import => {
                            let mut guard = futures::executor::block_on(result.import.write());
                            let values: Vec<Import> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::Index => {
                            let mut guard = futures::executor::block_on(result.index.write());
                            let values: Vec<Index> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::IntegerLiteral => {
                            let mut guard =
                                futures::executor::block_on(result.integer_literal.write());
                            let values: Vec<IntegerLiteral> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::Item => {
                            let mut guard = futures::executor::block_on(result.item.write());
                            let values: Vec<Item> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::Lambda => {
                            let mut guard = futures::executor::block_on(result.lambda.write());
                            let values: Vec<Lambda> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::LambdaParameter => {
                            let mut guard =
                                futures::executor::block_on(result.lambda_parameter.write());
                            let values: Vec<LambdaParameter> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::LetStatement => {
                            let mut guard =
                                futures::executor::block_on(result.let_statement.write());
                            let values: Vec<LetStatement> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::List => {
                            let mut guard = futures::executor::block_on(result.list.write());
                            let values: Vec<List> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::ListElement => {
                            let mut guard =
                                futures::executor::block_on(result.list_element.write());
                            let values: Vec<ListElement> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::ListExpression => {
                            let mut guard =
                                futures::executor::block_on(result.list_expression.write());
                            let values: Vec<ListExpression> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::Literal => {
                            let mut guard = futures::executor::block_on(result.literal.write());
                            let values: Vec<Literal> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::LocalVariable => {
                            let mut guard =
                                futures::executor::block_on(result.local_variable.write());
                            let values: Vec<LocalVariable> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::XMacro => {
                            let mut guard = futures::executor::block_on(result.x_macro.write());
                            let values: Vec<XMacro> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::XMatch => {
                            let mut guard = futures::executor::block_on(result.x_match.write());
                            let values: Vec<XMatch> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::MethodCall => {
                            let mut guard = futures::executor::block_on(result.method_call.write());
                            let values: Vec<MethodCall> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::ZObjectStore => {
                            let mut guard =
                                futures::executor::block_on(result.z_object_store.write());
                            let values: Vec<ZObjectStore> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::ObjectWrapper => {
                            let mut guard =
                                futures::executor::block_on(result.object_wrapper.write());
                            let values: Vec<ObjectWrapper> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::Operator => {
                            let mut guard = futures::executor::block_on(result.operator.write());
                            let values: Vec<Operator> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::WoogOption => {
                            let mut guard = futures::executor::block_on(result.woog_option.write());
                            let values: Vec<WoogOption> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::Parameter => {
                            let mut guard = futures::executor::block_on(result.parameter.write());
                            let values: Vec<Parameter> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::Pattern => {
                            let mut guard = futures::executor::block_on(result.pattern.write());
                            let values: Vec<Pattern> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::Plain => {
                            let mut guard = futures::executor::block_on(result.plain.write());
                            let values: Vec<Plain> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::XPrint => {
                            let mut guard = futures::executor::block_on(result.x_print.write());
                            let values: Vec<XPrint> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::RangeExpression => {
                            let mut guard =
                                futures::executor::block_on(result.range_expression.write());
                            let values: Vec<RangeExpression> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::Reference => {
                            let mut guard = futures::executor::block_on(result.reference.write());
                            let values: Vec<Reference> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::ResultStatement => {
                            let mut guard =
                                futures::executor::block_on(result.result_statement.write());
                            let values: Vec<ResultStatement> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::XReturn => {
                            let mut guard = futures::executor::block_on(result.x_return.write());
                            let values: Vec<XReturn> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::ZSome => {
                            let mut guard = futures::executor::block_on(result.z_some.write());
                            let values: Vec<ZSome> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::Span => {
                            let mut guard = futures::executor::block_on(result.span.write());
                            let values: Vec<Span> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::Statement => {
                            let mut guard = futures::executor::block_on(result.statement.write());
                            let values: Vec<Statement> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::StaticMethodCall => {
                            let mut guard =
                                futures::executor::block_on(result.static_method_call.write());
                            let values: Vec<StaticMethodCall> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::StringLiteral => {
                            let mut guard =
                                futures::executor::block_on(result.string_literal.write());
                            let values: Vec<StringLiteral> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::WoogStruct => {
                            let mut guard = futures::executor::block_on(result.woog_struct.write());
                            let values: Vec<WoogStruct> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::StructExpression => {
                            let mut guard =
                                futures::executor::block_on(result.struct_expression.write());
                            let values: Vec<StructExpression> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::StructField => {
                            let mut guard =
                                futures::executor::block_on(result.struct_field.write());
                            let values: Vec<StructField> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::TupleField => {
                            let mut guard = futures::executor::block_on(result.tuple_field.write());
                            let values: Vec<TupleField> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::TypeCast => {
                            let mut guard = futures::executor::block_on(result.type_cast.write());
                            let values: Vec<TypeCast> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::Unary => {
                            let mut guard = futures::executor::block_on(result.unary.write());
                            let values: Vec<Unary> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::XValue => {
                            let mut guard = futures::executor::block_on(result.x_value.write());
                            let values: Vec<XValue> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::ValueType => {
                            let mut guard = futures::executor::block_on(result.value_type.write());
                            let values: Vec<ValueType> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::Variable => {
                            let mut guard = futures::executor::block_on(result.variable.write());
                            let values: Vec<Variable> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                        SerdeField::VariableExpression => {
                            let mut guard =
                                futures::executor::block_on(result.variable_expression.write());
                            let values: Vec<VariableExpression> = map.next_value()?;
                            for value in values {
                                guard.push(Some(Arc::new(RwLock::new(value))));
                            }
                        }
                    }
                }
                Ok(result)
            }
        }

        struct ArgumentVisitor;
        impl<'de> Visitor<'de> for ArgumentVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<Argument>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Argument map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, Argument>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct BinaryVisitor;
        impl<'de> Visitor<'de> for BinaryVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<Binary>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Binary map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, Binary>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct BlockVisitor;
        impl<'de> Visitor<'de> for BlockVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<Block>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Block map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, Block>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct BodyVisitor;
        impl<'de> Visitor<'de> for BodyVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<Body>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Body map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, Body>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct BooleanLiteralVisitor;
        impl<'de> Visitor<'de> for BooleanLiteralVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<BooleanLiteral>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("BooleanLiteral map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, BooleanLiteral>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct BooleanOperatorVisitor;
        impl<'de> Visitor<'de> for BooleanOperatorVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<BooleanOperator>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("BooleanOperator map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, BooleanOperator>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct CallVisitor;
        impl<'de> Visitor<'de> for CallVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<Call>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Call map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, Call>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ComparisonVisitor;
        impl<'de> Visitor<'de> for ComparisonVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<Comparison>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Comparison map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, Comparison>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct DwarfSourceFileVisitor;
        impl<'de> Visitor<'de> for DwarfSourceFileVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<DwarfSourceFile>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("DwarfSourceFile map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, DwarfSourceFile>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct EnumFieldVisitor;
        impl<'de> Visitor<'de> for EnumFieldVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<EnumField>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("EnumField map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, EnumField>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct EnumerationVisitor;
        impl<'de> Visitor<'de> for EnumerationVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<Enumeration>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Enumeration map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, Enumeration>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct XErrorVisitor;
        impl<'de> Visitor<'de> for XErrorVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<XError>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("XError map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, XError>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ErrorExpressionVisitor;
        impl<'de> Visitor<'de> for ErrorExpressionVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<ErrorExpression>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ErrorExpression map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, ErrorExpression>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ExpressionVisitor;
        impl<'de> Visitor<'de> for ExpressionVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<Expression>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Expression map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, Expression>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ExpressionStatementVisitor;
        impl<'de> Visitor<'de> for ExpressionStatementVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<ExpressionStatement>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ExpressionStatement map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, ExpressionStatement>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ExternalImplementationVisitor;
        impl<'de> Visitor<'de> for ExternalImplementationVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<ExternalImplementation>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ExternalImplementation map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) =
                    access.next_entry::<Uuid, ExternalImplementation>()?
                {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct FieldVisitor;
        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<Field>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Field map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, Field>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct FieldAccessVisitor;
        impl<'de> Visitor<'de> for FieldAccessVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<FieldAccess>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("FieldAccess map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, FieldAccess>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct FieldAccessTargetVisitor;
        impl<'de> Visitor<'de> for FieldAccessTargetVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<FieldAccessTarget>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("FieldAccessTarget map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, FieldAccessTarget>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct FieldExpressionVisitor;
        impl<'de> Visitor<'de> for FieldExpressionVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<FieldExpression>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("FieldExpression map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, FieldExpression>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct FloatLiteralVisitor;
        impl<'de> Visitor<'de> for FloatLiteralVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<FloatLiteral>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("FloatLiteral map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, FloatLiteral>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ForLoopVisitor;
        impl<'de> Visitor<'de> for ForLoopVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<ForLoop>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ForLoop map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, ForLoop>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct FunctionVisitor;
        impl<'de> Visitor<'de> for FunctionVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<Function>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Function map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, Function>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct GenericVisitor;
        impl<'de> Visitor<'de> for GenericVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<Generic>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Generic map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, Generic>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct GroupedVisitor;
        impl<'de> Visitor<'de> for GroupedVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<Grouped>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Grouped map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, Grouped>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct XIfVisitor;
        impl<'de> Visitor<'de> for XIfVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<XIf>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("XIf map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, XIf>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ImplementationBlockVisitor;
        impl<'de> Visitor<'de> for ImplementationBlockVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<ImplementationBlock>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ImplementationBlock map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, ImplementationBlock>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ImportVisitor;
        impl<'de> Visitor<'de> for ImportVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<Import>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Import map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, Import>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct IndexVisitor;
        impl<'de> Visitor<'de> for IndexVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<Index>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Index map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, Index>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct IntegerLiteralVisitor;
        impl<'de> Visitor<'de> for IntegerLiteralVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<IntegerLiteral>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("IntegerLiteral map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, IntegerLiteral>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ItemVisitor;
        impl<'de> Visitor<'de> for ItemVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<Item>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Item map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, Item>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct LambdaVisitor;
        impl<'de> Visitor<'de> for LambdaVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<Lambda>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Lambda map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, Lambda>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct LambdaParameterVisitor;
        impl<'de> Visitor<'de> for LambdaParameterVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<LambdaParameter>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("LambdaParameter map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, LambdaParameter>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct LetStatementVisitor;
        impl<'de> Visitor<'de> for LetStatementVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<LetStatement>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("LetStatement map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, LetStatement>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ListVisitor;
        impl<'de> Visitor<'de> for ListVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<List>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("List map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, List>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ListElementVisitor;
        impl<'de> Visitor<'de> for ListElementVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<ListElement>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ListElement map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, ListElement>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ListExpressionVisitor;
        impl<'de> Visitor<'de> for ListExpressionVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<ListExpression>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ListExpression map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, ListExpression>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct LiteralVisitor;
        impl<'de> Visitor<'de> for LiteralVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<Literal>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Literal map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, Literal>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct LocalVariableVisitor;
        impl<'de> Visitor<'de> for LocalVariableVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<LocalVariable>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("LocalVariable map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, LocalVariable>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct XMacroVisitor;
        impl<'de> Visitor<'de> for XMacroVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<XMacro>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("XMacro map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, XMacro>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct XMatchVisitor;
        impl<'de> Visitor<'de> for XMatchVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<XMatch>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("XMatch map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, XMatch>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct MethodCallVisitor;
        impl<'de> Visitor<'de> for MethodCallVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<MethodCall>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("MethodCall map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, MethodCall>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ZObjectStoreVisitor;
        impl<'de> Visitor<'de> for ZObjectStoreVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<ZObjectStore>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ZObjectStore map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, ZObjectStore>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ObjectWrapperVisitor;
        impl<'de> Visitor<'de> for ObjectWrapperVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<ObjectWrapper>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ObjectWrapper map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, ObjectWrapper>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct OperatorVisitor;
        impl<'de> Visitor<'de> for OperatorVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<Operator>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Operator map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, Operator>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct WoogOptionVisitor;
        impl<'de> Visitor<'de> for WoogOptionVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<WoogOption>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("WoogOption map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, WoogOption>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ParameterVisitor;
        impl<'de> Visitor<'de> for ParameterVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<Parameter>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Parameter map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, Parameter>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct PatternVisitor;
        impl<'de> Visitor<'de> for PatternVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<Pattern>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Pattern map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, Pattern>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct PlainVisitor;
        impl<'de> Visitor<'de> for PlainVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<Plain>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Plain map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, Plain>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct XPrintVisitor;
        impl<'de> Visitor<'de> for XPrintVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<XPrint>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("XPrint map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, XPrint>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct RangeExpressionVisitor;
        impl<'de> Visitor<'de> for RangeExpressionVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<RangeExpression>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("RangeExpression map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, RangeExpression>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ReferenceVisitor;
        impl<'de> Visitor<'de> for ReferenceVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<Reference>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Reference map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, Reference>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ResultStatementVisitor;
        impl<'de> Visitor<'de> for ResultStatementVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<ResultStatement>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ResultStatement map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, ResultStatement>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct XReturnVisitor;
        impl<'de> Visitor<'de> for XReturnVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<XReturn>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("XReturn map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, XReturn>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ZSomeVisitor;
        impl<'de> Visitor<'de> for ZSomeVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<ZSome>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ZSome map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, ZSome>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct SpanVisitor;
        impl<'de> Visitor<'de> for SpanVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<Span>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Span map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, Span>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct StatementVisitor;
        impl<'de> Visitor<'de> for StatementVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<Statement>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Statement map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, Statement>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct StaticMethodCallVisitor;
        impl<'de> Visitor<'de> for StaticMethodCallVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<StaticMethodCall>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("StaticMethodCall map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, StaticMethodCall>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct StringLiteralVisitor;
        impl<'de> Visitor<'de> for StringLiteralVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<StringLiteral>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("StringLiteral map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, StringLiteral>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct WoogStructVisitor;
        impl<'de> Visitor<'de> for WoogStructVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<WoogStruct>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("WoogStruct map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, WoogStruct>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct StructExpressionVisitor;
        impl<'de> Visitor<'de> for StructExpressionVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<StructExpression>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("StructExpression map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, StructExpression>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct StructFieldVisitor;
        impl<'de> Visitor<'de> for StructFieldVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<StructField>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("StructField map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, StructField>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct TupleFieldVisitor;
        impl<'de> Visitor<'de> for TupleFieldVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<TupleField>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("TupleField map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, TupleField>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct TypeCastVisitor;
        impl<'de> Visitor<'de> for TypeCastVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<TypeCast>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("TypeCast map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, TypeCast>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct UnaryVisitor;
        impl<'de> Visitor<'de> for UnaryVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<Unary>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Unary map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, Unary>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct XValueVisitor;
        impl<'de> Visitor<'de> for XValueVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<XValue>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("XValue map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, XValue>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct ValueTypeVisitor;
        impl<'de> Visitor<'de> for ValueTypeVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<ValueType>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ValueType map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, ValueType>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct VariableVisitor;
        impl<'de> Visitor<'de> for VariableVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<Variable>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Variable map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, Variable>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        struct VariableExpressionVisitor;
        impl<'de> Visitor<'de> for VariableExpressionVisitor {
            type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<VariableExpression>>>>>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("VariableExpression map")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::default();
                while let Some((key, value)) = access.next_entry::<Uuid, VariableExpression>()? {
                    map.insert(key, Arc::new(RwLock::new(value)));
                }
                Ok(Arc::new(RwLock::new(map)))
            }
        }

        const FIELDS: &'static [&'static str] = &[
            "argument",
            "binary",
            "block",
            "body",
            "boolean_literal",
            "boolean_operator",
            "call",
            "comparison",
            "dwarf_source_file",
            "enum_field",
            "enumeration",
            "x_error",
            "error_expression",
            "expression",
            "expression_statement",
            "external_implementation",
            "field",
            "field_access",
            "field_access_target",
            "field_expression",
            "float_literal",
            "for_loop",
            "function",
            "generic",
            "grouped",
            "x_if",
            "implementation_block",
            "import",
            "index",
            "integer_literal",
            "item",
            "lambda",
            "lambda_parameter",
            "let_statement",
            "list",
            "list_element",
            "list_expression",
            "literal",
            "local_variable",
            "x_macro",
            "x_match",
            "method_call",
            "z_object_store",
            "object_wrapper",
            "operator",
            "woog_option",
            "parameter",
            "pattern",
            "plain",
            "x_print",
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
            "struct_field",
            "tuple_field",
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
    pub async fn new() -> Self {
        let mut store = Self {
            argument_free_list: async_std::sync::Mutex::new(Vec::new()),
            argument: Arc::new(RwLock::new(Vec::new())),
            binary_free_list: async_std::sync::Mutex::new(Vec::new()),
            binary: Arc::new(RwLock::new(Vec::new())),
            block_free_list: async_std::sync::Mutex::new(Vec::new()),
            block: Arc::new(RwLock::new(Vec::new())),
            body_free_list: async_std::sync::Mutex::new(Vec::new()),
            body: Arc::new(RwLock::new(Vec::new())),
            boolean_literal_free_list: async_std::sync::Mutex::new(Vec::new()),
            boolean_literal: Arc::new(RwLock::new(Vec::new())),
            boolean_operator_free_list: async_std::sync::Mutex::new(Vec::new()),
            boolean_operator: Arc::new(RwLock::new(Vec::new())),
            call_free_list: async_std::sync::Mutex::new(Vec::new()),
            call: Arc::new(RwLock::new(Vec::new())),
            comparison_free_list: async_std::sync::Mutex::new(Vec::new()),
            comparison: Arc::new(RwLock::new(Vec::new())),
            dwarf_source_file_free_list: async_std::sync::Mutex::new(Vec::new()),
            dwarf_source_file: Arc::new(RwLock::new(Vec::new())),
            enum_field_free_list: async_std::sync::Mutex::new(Vec::new()),
            enum_field: Arc::new(RwLock::new(Vec::new())),
            enumeration_free_list: async_std::sync::Mutex::new(Vec::new()),
            enumeration: Arc::new(RwLock::new(Vec::new())),
            enumeration_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            x_error_free_list: async_std::sync::Mutex::new(Vec::new()),
            x_error: Arc::new(RwLock::new(Vec::new())),
            error_expression_free_list: async_std::sync::Mutex::new(Vec::new()),
            error_expression: Arc::new(RwLock::new(Vec::new())),
            expression_free_list: async_std::sync::Mutex::new(Vec::new()),
            expression: Arc::new(RwLock::new(Vec::new())),
            expression_statement_free_list: async_std::sync::Mutex::new(Vec::new()),
            expression_statement: Arc::new(RwLock::new(Vec::new())),
            external_implementation_free_list: async_std::sync::Mutex::new(Vec::new()),
            external_implementation: Arc::new(RwLock::new(Vec::new())),
            field_free_list: async_std::sync::Mutex::new(Vec::new()),
            field: Arc::new(RwLock::new(Vec::new())),
            field_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            field_access_free_list: async_std::sync::Mutex::new(Vec::new()),
            field_access: Arc::new(RwLock::new(Vec::new())),
            field_access_target_free_list: async_std::sync::Mutex::new(Vec::new()),
            field_access_target: Arc::new(RwLock::new(Vec::new())),
            field_expression_free_list: async_std::sync::Mutex::new(Vec::new()),
            field_expression: Arc::new(RwLock::new(Vec::new())),
            float_literal_free_list: async_std::sync::Mutex::new(Vec::new()),
            float_literal: Arc::new(RwLock::new(Vec::new())),
            for_loop_free_list: async_std::sync::Mutex::new(Vec::new()),
            for_loop: Arc::new(RwLock::new(Vec::new())),
            function_free_list: async_std::sync::Mutex::new(Vec::new()),
            function: Arc::new(RwLock::new(Vec::new())),
            function_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            generic_free_list: async_std::sync::Mutex::new(Vec::new()),
            generic: Arc::new(RwLock::new(Vec::new())),
            grouped_free_list: async_std::sync::Mutex::new(Vec::new()),
            grouped: Arc::new(RwLock::new(Vec::new())),
            x_if_free_list: async_std::sync::Mutex::new(Vec::new()),
            x_if: Arc::new(RwLock::new(Vec::new())),
            implementation_block_free_list: async_std::sync::Mutex::new(Vec::new()),
            implementation_block: Arc::new(RwLock::new(Vec::new())),
            import_free_list: async_std::sync::Mutex::new(Vec::new()),
            import: Arc::new(RwLock::new(Vec::new())),
            index_free_list: async_std::sync::Mutex::new(Vec::new()),
            index: Arc::new(RwLock::new(Vec::new())),
            integer_literal_free_list: async_std::sync::Mutex::new(Vec::new()),
            integer_literal: Arc::new(RwLock::new(Vec::new())),
            item_free_list: async_std::sync::Mutex::new(Vec::new()),
            item: Arc::new(RwLock::new(Vec::new())),
            lambda_free_list: async_std::sync::Mutex::new(Vec::new()),
            lambda: Arc::new(RwLock::new(Vec::new())),
            lambda_parameter_free_list: async_std::sync::Mutex::new(Vec::new()),
            lambda_parameter: Arc::new(RwLock::new(Vec::new())),
            let_statement_free_list: async_std::sync::Mutex::new(Vec::new()),
            let_statement: Arc::new(RwLock::new(Vec::new())),
            list_free_list: async_std::sync::Mutex::new(Vec::new()),
            list: Arc::new(RwLock::new(Vec::new())),
            list_element_free_list: async_std::sync::Mutex::new(Vec::new()),
            list_element: Arc::new(RwLock::new(Vec::new())),
            list_expression_free_list: async_std::sync::Mutex::new(Vec::new()),
            list_expression: Arc::new(RwLock::new(Vec::new())),
            literal_free_list: async_std::sync::Mutex::new(Vec::new()),
            literal: Arc::new(RwLock::new(Vec::new())),
            local_variable_free_list: async_std::sync::Mutex::new(Vec::new()),
            local_variable: Arc::new(RwLock::new(Vec::new())),
            x_macro_free_list: async_std::sync::Mutex::new(Vec::new()),
            x_macro: Arc::new(RwLock::new(Vec::new())),
            x_match_free_list: async_std::sync::Mutex::new(Vec::new()),
            x_match: Arc::new(RwLock::new(Vec::new())),
            method_call_free_list: async_std::sync::Mutex::new(Vec::new()),
            method_call: Arc::new(RwLock::new(Vec::new())),
            z_object_store_free_list: async_std::sync::Mutex::new(Vec::new()),
            z_object_store: Arc::new(RwLock::new(Vec::new())),
            z_object_store_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            object_wrapper_free_list: async_std::sync::Mutex::new(Vec::new()),
            object_wrapper: Arc::new(RwLock::new(Vec::new())),
            operator_free_list: async_std::sync::Mutex::new(Vec::new()),
            operator: Arc::new(RwLock::new(Vec::new())),
            woog_option_free_list: async_std::sync::Mutex::new(Vec::new()),
            woog_option: Arc::new(RwLock::new(Vec::new())),
            parameter_free_list: async_std::sync::Mutex::new(Vec::new()),
            parameter: Arc::new(RwLock::new(Vec::new())),
            pattern_free_list: async_std::sync::Mutex::new(Vec::new()),
            pattern: Arc::new(RwLock::new(Vec::new())),
            plain_free_list: async_std::sync::Mutex::new(Vec::new()),
            plain: Arc::new(RwLock::new(Vec::new())),
            x_print_free_list: async_std::sync::Mutex::new(Vec::new()),
            x_print: Arc::new(RwLock::new(Vec::new())),
            range_expression_free_list: async_std::sync::Mutex::new(Vec::new()),
            range_expression: Arc::new(RwLock::new(Vec::new())),
            reference_free_list: async_std::sync::Mutex::new(Vec::new()),
            reference: Arc::new(RwLock::new(Vec::new())),
            result_statement_free_list: async_std::sync::Mutex::new(Vec::new()),
            result_statement: Arc::new(RwLock::new(Vec::new())),
            x_return_free_list: async_std::sync::Mutex::new(Vec::new()),
            x_return: Arc::new(RwLock::new(Vec::new())),
            z_some_free_list: async_std::sync::Mutex::new(Vec::new()),
            z_some: Arc::new(RwLock::new(Vec::new())),
            span_free_list: async_std::sync::Mutex::new(Vec::new()),
            span: Arc::new(RwLock::new(Vec::new())),
            statement_free_list: async_std::sync::Mutex::new(Vec::new()),
            statement: Arc::new(RwLock::new(Vec::new())),
            static_method_call_free_list: async_std::sync::Mutex::new(Vec::new()),
            static_method_call: Arc::new(RwLock::new(Vec::new())),
            string_literal_free_list: async_std::sync::Mutex::new(Vec::new()),
            string_literal: Arc::new(RwLock::new(Vec::new())),
            woog_struct_free_list: async_std::sync::Mutex::new(Vec::new()),
            woog_struct: Arc::new(RwLock::new(Vec::new())),
            woog_struct_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            struct_expression_free_list: async_std::sync::Mutex::new(Vec::new()),
            struct_expression: Arc::new(RwLock::new(Vec::new())),
            struct_field_free_list: async_std::sync::Mutex::new(Vec::new()),
            struct_field: Arc::new(RwLock::new(Vec::new())),
            tuple_field_free_list: async_std::sync::Mutex::new(Vec::new()),
            tuple_field: Arc::new(RwLock::new(Vec::new())),
            type_cast_free_list: async_std::sync::Mutex::new(Vec::new()),
            type_cast: Arc::new(RwLock::new(Vec::new())),
            unary_free_list: async_std::sync::Mutex::new(Vec::new()),
            unary: Arc::new(RwLock::new(Vec::new())),
            x_value_free_list: async_std::sync::Mutex::new(Vec::new()),
            x_value: Arc::new(RwLock::new(Vec::new())),
            value_type_free_list: async_std::sync::Mutex::new(Vec::new()),
            value_type: Arc::new(RwLock::new(Vec::new())),
            variable_free_list: async_std::sync::Mutex::new(Vec::new()),
            variable: Arc::new(RwLock::new(Vec::new())),
            variable_expression_free_list: async_std::sync::Mutex::new(Vec::new()),
            variable_expression: Arc::new(RwLock::new(Vec::new())),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥
        store
            .inter_binary(|id| {
                Arc::new(RwLock::new(Binary {
                    subtype: super::BinaryEnum::Addition(ADDITION),
                    id,
                }))
            })
            .await;
        store
            .inter_binary(|id| {
                Arc::new(RwLock::new(Binary {
                    subtype: super::BinaryEnum::Assignment(ASSIGNMENT),
                    id,
                }))
            })
            .await;
        store
            .inter_binary(|id| {
                Arc::new(RwLock::new(Binary {
                    subtype: super::BinaryEnum::Division(DIVISION),
                    id,
                }))
            })
            .await;
        store
            .inter_binary(|id| {
                Arc::new(RwLock::new(Binary {
                    subtype: super::BinaryEnum::Multiplication(MULTIPLICATION),
                    id,
                }))
            })
            .await;
        store
            .inter_binary(|id| {
                Arc::new(RwLock::new(Binary {
                    subtype: super::BinaryEnum::Subtraction(SUBTRACTION),
                    id,
                }))
            })
            .await;
        store
            .inter_boolean_literal(|id| {
                Arc::new(RwLock::new(BooleanLiteral {
                    subtype: super::BooleanLiteralEnum::FalseLiteral(FALSE_LITERAL),
                    id,
                }))
            })
            .await;
        store
            .inter_boolean_literal(|id| {
                Arc::new(RwLock::new(BooleanLiteral {
                    subtype: super::BooleanLiteralEnum::TrueLiteral(TRUE_LITERAL),
                    id,
                }))
            })
            .await;
        store
            .inter_boolean_operator(|id| {
                Arc::new(RwLock::new(BooleanOperator {
                    subtype: super::BooleanOperatorEnum::And(AND),
                    id,
                }))
            })
            .await;
        store
            .inter_boolean_operator(|id| {
                Arc::new(RwLock::new(BooleanOperator {
                    subtype: super::BooleanOperatorEnum::Or(OR),
                    id,
                }))
            })
            .await;
        store
            .inter_comparison(|id| {
                Arc::new(RwLock::new(Comparison {
                    subtype: super::ComparisonEnum::Equal(EQUAL),
                    id,
                }))
            })
            .await;
        store
            .inter_comparison(|id| {
                Arc::new(RwLock::new(Comparison {
                    subtype: super::ComparisonEnum::GreaterThan(GREATER_THAN),
                    id,
                }))
            })
            .await;
        store
            .inter_comparison(|id| {
                Arc::new(RwLock::new(Comparison {
                    subtype: super::ComparisonEnum::GreaterThanOrEqual(GREATER_THAN_OR_EQUAL),
                    id,
                }))
            })
            .await;
        store
            .inter_comparison(|id| {
                Arc::new(RwLock::new(Comparison {
                    subtype: super::ComparisonEnum::LessThan(LESS_THAN),
                    id,
                }))
            })
            .await;
        store
            .inter_comparison(|id| {
                Arc::new(RwLock::new(Comparison {
                    subtype: super::ComparisonEnum::LessThanOrEqual(LESS_THAN_OR_EQUAL),
                    id,
                }))
            })
            .await;
        store
            .inter_comparison(|id| {
                Arc::new(RwLock::new(Comparison {
                    subtype: super::ComparisonEnum::NotEqual(NOT_EQUAL),
                    id,
                }))
            })
            .await;
        store
            .inter_x_error(|id| {
                Arc::new(RwLock::new(XError {
                    subtype: super::XErrorEnum::UnknownVariable(UNKNOWN_VARIABLE),
                    id,
                }))
            })
            .await;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    subtype: super::ExpressionEnum::Debugger(DEBUGGER),
                    id,
                }))
            })
            .await;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    subtype: super::ExpressionEnum::ZNone(Z_NONE),
                    id,
                }))
            })
            .await;
        store
            .inter_unary(|id| {
                Arc::new(RwLock::new(Unary {
                    subtype: super::UnaryEnum::Negation(NEGATION),
                    id,
                }))
            })
            .await;
        store
            .inter_unary(|id| {
                Arc::new(RwLock::new(Unary {
                    subtype: super::UnaryEnum::Not(NOT),
                    id,
                }))
            })
            .await;
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    subtype: super::ValueTypeEnum::Char(CHAR),
                    id,
                }))
            })
            .await;
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    subtype: super::ValueTypeEnum::Empty(EMPTY),
                    id,
                }))
            })
            .await;
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    subtype: super::ValueTypeEnum::Range(RANGE),
                    id,
                }))
            })
            .await;
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    subtype: super::ValueTypeEnum::Unknown(UNKNOWN),
                    id,
                }))
            })
            .await;

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_async-object-store-methods"}}}
    /// Inter (insert) [`Argument`] into the store.
    ///
    pub async fn inter_argument<F>(&mut self, argument: F) -> Arc<RwLock<Argument>>
    where
        F: Fn(usize) -> Arc<RwLock<Argument>>,
    {
        let _index = if let Some(_index) = self.argument_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.argument.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.argument.write().await.push(None);
            _index
        };

        let argument = argument(_index);

        let iter = self.argument.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *argument.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(argument) = found {
            log::debug!(target: "store", "found duplicate {argument:?}.");
            self.argument_free_list.lock().await.push(_index);
            argument.clone()
        } else {
            log::debug!(target: "store", "interring {argument:?}.");
            self.argument.write().await[_index] = Some(argument.clone());
            argument
        }
    }

    /// Exhume (get) [`Argument`] from the store.
    ///
    pub async fn exhume_argument(&self, id: &usize) -> Option<Arc<RwLock<Argument>>> {
        match self.argument.read().await.get(*id) {
            Some(argument) => argument.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Argument`] from the store.
    ///
    pub async fn exorcise_argument(&mut self, id: &usize) -> Option<Arc<RwLock<Argument>>> {
        log::debug!(target: "store", "exorcising argument slot: {id}.");
        let result = self.argument.write().await[*id].take();
        self.argument_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Argument>`.
    ///
    pub async fn iter_argument(&self) -> impl stream::Stream<Item = Arc<RwLock<Argument>>> + '_ {
        let len = self.argument.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.argument.read().await[i].is_some() {
                self.argument.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`Binary`] into the store.
    ///
    pub async fn inter_binary<F>(&mut self, binary: F) -> Arc<RwLock<Binary>>
    where
        F: Fn(usize) -> Arc<RwLock<Binary>>,
    {
        let _index = if let Some(_index) = self.binary_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.binary.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.binary.write().await.push(None);
            _index
        };

        let binary = binary(_index);

        let iter = self.binary.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *binary.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(binary) = found {
            log::debug!(target: "store", "found duplicate {binary:?}.");
            self.binary_free_list.lock().await.push(_index);
            binary.clone()
        } else {
            log::debug!(target: "store", "interring {binary:?}.");
            self.binary.write().await[_index] = Some(binary.clone());
            binary
        }
    }

    /// Exhume (get) [`Binary`] from the store.
    ///
    pub async fn exhume_binary(&self, id: &usize) -> Option<Arc<RwLock<Binary>>> {
        match self.binary.read().await.get(*id) {
            Some(binary) => binary.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Binary`] from the store.
    ///
    pub async fn exorcise_binary(&mut self, id: &usize) -> Option<Arc<RwLock<Binary>>> {
        log::debug!(target: "store", "exorcising binary slot: {id}.");
        let result = self.binary.write().await[*id].take();
        self.binary_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Binary>`.
    ///
    pub async fn iter_binary(&self) -> impl stream::Stream<Item = Arc<RwLock<Binary>>> + '_ {
        let len = self.binary.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.binary.read().await[i].is_some() {
                self.binary.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`Block`] into the store.
    ///
    pub async fn inter_block<F>(&mut self, block: F) -> Arc<RwLock<Block>>
    where
        F: Fn(usize) -> Arc<RwLock<Block>>,
    {
        let _index = if let Some(_index) = self.block_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.block.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.block.write().await.push(None);
            _index
        };

        let block = block(_index);

        let iter = self.block.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *block.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(block) = found {
            log::debug!(target: "store", "found duplicate {block:?}.");
            self.block_free_list.lock().await.push(_index);
            block.clone()
        } else {
            log::debug!(target: "store", "interring {block:?}.");
            self.block.write().await[_index] = Some(block.clone());
            block
        }
    }

    /// Exhume (get) [`Block`] from the store.
    ///
    pub async fn exhume_block(&self, id: &usize) -> Option<Arc<RwLock<Block>>> {
        match self.block.read().await.get(*id) {
            Some(block) => block.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Block`] from the store.
    ///
    pub async fn exorcise_block(&mut self, id: &usize) -> Option<Arc<RwLock<Block>>> {
        log::debug!(target: "store", "exorcising block slot: {id}.");
        let result = self.block.write().await[*id].take();
        self.block_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Block>`.
    ///
    pub async fn iter_block(&self) -> impl stream::Stream<Item = Arc<RwLock<Block>>> + '_ {
        let len = self.block.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.block.read().await[i].is_some() {
                self.block.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`Body`] into the store.
    ///
    pub async fn inter_body<F>(&mut self, body: F) -> Arc<RwLock<Body>>
    where
        F: Fn(usize) -> Arc<RwLock<Body>>,
    {
        let _index = if let Some(_index) = self.body_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.body.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.body.write().await.push(None);
            _index
        };

        let body = body(_index);

        let iter = self.body.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *body.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(body) = found {
            log::debug!(target: "store", "found duplicate {body:?}.");
            self.body_free_list.lock().await.push(_index);
            body.clone()
        } else {
            log::debug!(target: "store", "interring {body:?}.");
            self.body.write().await[_index] = Some(body.clone());
            body
        }
    }

    /// Exhume (get) [`Body`] from the store.
    ///
    pub async fn exhume_body(&self, id: &usize) -> Option<Arc<RwLock<Body>>> {
        match self.body.read().await.get(*id) {
            Some(body) => body.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Body`] from the store.
    ///
    pub async fn exorcise_body(&mut self, id: &usize) -> Option<Arc<RwLock<Body>>> {
        log::debug!(target: "store", "exorcising body slot: {id}.");
        let result = self.body.write().await[*id].take();
        self.body_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Body>`.
    ///
    pub async fn iter_body(&self) -> impl stream::Stream<Item = Arc<RwLock<Body>>> + '_ {
        let len = self.body.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.body.read().await[i].is_some() {
                self.body.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`BooleanLiteral`] into the store.
    ///
    pub async fn inter_boolean_literal<F>(
        &mut self,
        boolean_literal: F,
    ) -> Arc<RwLock<BooleanLiteral>>
    where
        F: Fn(usize) -> Arc<RwLock<BooleanLiteral>>,
    {
        let _index = if let Some(_index) = self.boolean_literal_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.boolean_literal.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.boolean_literal.write().await.push(None);
            _index
        };

        let boolean_literal = boolean_literal(_index);

        let iter = self.boolean_literal.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *boolean_literal.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(boolean_literal) = found {
            log::debug!(target: "store", "found duplicate {boolean_literal:?}.");
            self.boolean_literal_free_list.lock().await.push(_index);
            boolean_literal.clone()
        } else {
            log::debug!(target: "store", "interring {boolean_literal:?}.");
            self.boolean_literal.write().await[_index] = Some(boolean_literal.clone());
            boolean_literal
        }
    }

    /// Exhume (get) [`BooleanLiteral`] from the store.
    ///
    pub async fn exhume_boolean_literal(&self, id: &usize) -> Option<Arc<RwLock<BooleanLiteral>>> {
        match self.boolean_literal.read().await.get(*id) {
            Some(boolean_literal) => boolean_literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`BooleanLiteral`] from the store.
    ///
    pub async fn exorcise_boolean_literal(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<BooleanLiteral>>> {
        log::debug!(target: "store", "exorcising boolean_literal slot: {id}.");
        let result = self.boolean_literal.write().await[*id].take();
        self.boolean_literal_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanLiteral>`.
    ///
    pub async fn iter_boolean_literal(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<BooleanLiteral>>> + '_ {
        let len = self.boolean_literal.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.boolean_literal.read().await[i].is_some() {
                self.boolean_literal.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`BooleanOperator`] into the store.
    ///
    pub async fn inter_boolean_operator<F>(
        &mut self,
        boolean_operator: F,
    ) -> Arc<RwLock<BooleanOperator>>
    where
        F: Fn(usize) -> Arc<RwLock<BooleanOperator>>,
    {
        let _index = if let Some(_index) = self.boolean_operator_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.boolean_operator.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.boolean_operator.write().await.push(None);
            _index
        };

        let boolean_operator = boolean_operator(_index);

        let iter = self.boolean_operator.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *boolean_operator.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(boolean_operator) = found {
            log::debug!(target: "store", "found duplicate {boolean_operator:?}.");
            self.boolean_operator_free_list.lock().await.push(_index);
            boolean_operator.clone()
        } else {
            log::debug!(target: "store", "interring {boolean_operator:?}.");
            self.boolean_operator.write().await[_index] = Some(boolean_operator.clone());
            boolean_operator
        }
    }

    /// Exhume (get) [`BooleanOperator`] from the store.
    ///
    pub async fn exhume_boolean_operator(
        &self,
        id: &usize,
    ) -> Option<Arc<RwLock<BooleanOperator>>> {
        match self.boolean_operator.read().await.get(*id) {
            Some(boolean_operator) => boolean_operator.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`BooleanOperator`] from the store.
    ///
    pub async fn exorcise_boolean_operator(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<BooleanOperator>>> {
        log::debug!(target: "store", "exorcising boolean_operator slot: {id}.");
        let result = self.boolean_operator.write().await[*id].take();
        self.boolean_operator_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanOperator>`.
    ///
    pub async fn iter_boolean_operator(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<BooleanOperator>>> + '_ {
        let len = self.boolean_operator.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.boolean_operator.read().await[i].is_some() {
                self.boolean_operator.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`Call`] into the store.
    ///
    pub async fn inter_call<F>(&mut self, call: F) -> Arc<RwLock<Call>>
    where
        F: Fn(usize) -> Arc<RwLock<Call>>,
    {
        let _index = if let Some(_index) = self.call_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.call.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.call.write().await.push(None);
            _index
        };

        let call = call(_index);

        let iter = self.call.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *call.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(call) = found {
            log::debug!(target: "store", "found duplicate {call:?}.");
            self.call_free_list.lock().await.push(_index);
            call.clone()
        } else {
            log::debug!(target: "store", "interring {call:?}.");
            self.call.write().await[_index] = Some(call.clone());
            call
        }
    }

    /// Exhume (get) [`Call`] from the store.
    ///
    pub async fn exhume_call(&self, id: &usize) -> Option<Arc<RwLock<Call>>> {
        match self.call.read().await.get(*id) {
            Some(call) => call.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Call`] from the store.
    ///
    pub async fn exorcise_call(&mut self, id: &usize) -> Option<Arc<RwLock<Call>>> {
        log::debug!(target: "store", "exorcising call slot: {id}.");
        let result = self.call.write().await[*id].take();
        self.call_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Call>`.
    ///
    pub async fn iter_call(&self) -> impl stream::Stream<Item = Arc<RwLock<Call>>> + '_ {
        let len = self.call.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.call.read().await[i].is_some() {
                self.call.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`Comparison`] into the store.
    ///
    pub async fn inter_comparison<F>(&mut self, comparison: F) -> Arc<RwLock<Comparison>>
    where
        F: Fn(usize) -> Arc<RwLock<Comparison>>,
    {
        let _index = if let Some(_index) = self.comparison_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.comparison.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.comparison.write().await.push(None);
            _index
        };

        let comparison = comparison(_index);

        let iter = self.comparison.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *comparison.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(comparison) = found {
            log::debug!(target: "store", "found duplicate {comparison:?}.");
            self.comparison_free_list.lock().await.push(_index);
            comparison.clone()
        } else {
            log::debug!(target: "store", "interring {comparison:?}.");
            self.comparison.write().await[_index] = Some(comparison.clone());
            comparison
        }
    }

    /// Exhume (get) [`Comparison`] from the store.
    ///
    pub async fn exhume_comparison(&self, id: &usize) -> Option<Arc<RwLock<Comparison>>> {
        match self.comparison.read().await.get(*id) {
            Some(comparison) => comparison.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Comparison`] from the store.
    ///
    pub async fn exorcise_comparison(&mut self, id: &usize) -> Option<Arc<RwLock<Comparison>>> {
        log::debug!(target: "store", "exorcising comparison slot: {id}.");
        let result = self.comparison.write().await[*id].take();
        self.comparison_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Comparison>`.
    ///
    pub async fn iter_comparison(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<Comparison>>> + '_ {
        let len = self.comparison.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.comparison.read().await[i].is_some() {
                self.comparison.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`DwarfSourceFile`] into the store.
    ///
    pub async fn inter_dwarf_source_file<F>(
        &mut self,
        dwarf_source_file: F,
    ) -> Arc<RwLock<DwarfSourceFile>>
    where
        F: Fn(usize) -> Arc<RwLock<DwarfSourceFile>>,
    {
        let _index = if let Some(_index) = self.dwarf_source_file_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.dwarf_source_file.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.dwarf_source_file.write().await.push(None);
            _index
        };

        let dwarf_source_file = dwarf_source_file(_index);

        let iter = self.dwarf_source_file.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *dwarf_source_file.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(dwarf_source_file) = found {
            log::debug!(target: "store", "found duplicate {dwarf_source_file:?}.");
            self.dwarf_source_file_free_list.lock().await.push(_index);
            dwarf_source_file.clone()
        } else {
            log::debug!(target: "store", "interring {dwarf_source_file:?}.");
            self.dwarf_source_file.write().await[_index] = Some(dwarf_source_file.clone());
            dwarf_source_file
        }
    }

    /// Exhume (get) [`DwarfSourceFile`] from the store.
    ///
    pub async fn exhume_dwarf_source_file(
        &self,
        id: &usize,
    ) -> Option<Arc<RwLock<DwarfSourceFile>>> {
        match self.dwarf_source_file.read().await.get(*id) {
            Some(dwarf_source_file) => dwarf_source_file.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`DwarfSourceFile`] from the store.
    ///
    pub async fn exorcise_dwarf_source_file(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<DwarfSourceFile>>> {
        log::debug!(target: "store", "exorcising dwarf_source_file slot: {id}.");
        let result = self.dwarf_source_file.write().await[*id].take();
        self.dwarf_source_file_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, DwarfSourceFile>`.
    ///
    pub async fn iter_dwarf_source_file(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<DwarfSourceFile>>> + '_ {
        let len = self.dwarf_source_file.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.dwarf_source_file.read().await[i].is_some() {
                self.dwarf_source_file.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`EnumField`] into the store.
    ///
    pub async fn inter_enum_field<F>(&mut self, enum_field: F) -> Arc<RwLock<EnumField>>
    where
        F: Fn(usize) -> Arc<RwLock<EnumField>>,
    {
        let _index = if let Some(_index) = self.enum_field_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.enum_field.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.enum_field.write().await.push(None);
            _index
        };

        let enum_field = enum_field(_index);

        let iter = self.enum_field.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *enum_field.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(enum_field) = found {
            log::debug!(target: "store", "found duplicate {enum_field:?}.");
            self.enum_field_free_list.lock().await.push(_index);
            enum_field.clone()
        } else {
            log::debug!(target: "store", "interring {enum_field:?}.");
            self.enum_field.write().await[_index] = Some(enum_field.clone());
            enum_field
        }
    }

    /// Exhume (get) [`EnumField`] from the store.
    ///
    pub async fn exhume_enum_field(&self, id: &usize) -> Option<Arc<RwLock<EnumField>>> {
        match self.enum_field.read().await.get(*id) {
            Some(enum_field) => enum_field.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`EnumField`] from the store.
    ///
    pub async fn exorcise_enum_field(&mut self, id: &usize) -> Option<Arc<RwLock<EnumField>>> {
        log::debug!(target: "store", "exorcising enum_field slot: {id}.");
        let result = self.enum_field.write().await[*id].take();
        self.enum_field_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, EnumField>`.
    ///
    pub async fn iter_enum_field(&self) -> impl stream::Stream<Item = Arc<RwLock<EnumField>>> + '_ {
        let len = self.enum_field.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.enum_field.read().await[i].is_some() {
                self.enum_field.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`Enumeration`] into the store.
    ///
    pub async fn inter_enumeration<F>(&mut self, enumeration: F) -> Arc<RwLock<Enumeration>>
    where
        F: Fn(usize) -> Arc<RwLock<Enumeration>>,
    {
        let _index = if let Some(_index) = self.enumeration_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.enumeration.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.enumeration.write().await.push(None);
            _index
        };

        let enumeration = enumeration(_index);

        let iter = self.enumeration.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *enumeration.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        let enumeration = if let Some(enumeration) = found {
            log::debug!(target: "store", "found duplicate {enumeration:?}.");
            self.enumeration_free_list.lock().await.push(_index);
            enumeration.clone()
        } else {
            log::debug!(target: "store", "interring {enumeration:?}.");
            self.enumeration.write().await[_index] = Some(enumeration.clone());
            enumeration
        };
        self.enumeration_id_by_name.write().await.insert(
            enumeration.read().await.name.to_owned(),
            enumeration.read().await.id,
        );
        enumeration
    }

    /// Exhume (get) [`Enumeration`] from the store.
    ///
    pub async fn exhume_enumeration(&self, id: &usize) -> Option<Arc<RwLock<Enumeration>>> {
        match self.enumeration.read().await.get(*id) {
            Some(enumeration) => enumeration.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Enumeration`] from the store.
    ///
    pub async fn exorcise_enumeration(&mut self, id: &usize) -> Option<Arc<RwLock<Enumeration>>> {
        log::debug!(target: "store", "exorcising enumeration slot: {id}.");
        let result = self.enumeration.write().await[*id].take();
        self.enumeration_free_list.lock().await.push(*id);
        result
    }

    /// Exorcise [`Enumeration`] id from the store by name.
    ///
    pub async fn exhume_enumeration_id_by_name(&self, name: &str) -> Option<usize> {
        self.enumeration_id_by_name
            .read()
            .await
            .get(name)
            .map(|enumeration| *enumeration)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Enumeration>`.
    ///
    pub async fn iter_enumeration(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<Enumeration>>> + '_ {
        let len = self.enumeration.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.enumeration.read().await[i].is_some() {
                self.enumeration.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`XError`] into the store.
    ///
    pub async fn inter_x_error<F>(&mut self, x_error: F) -> Arc<RwLock<XError>>
    where
        F: Fn(usize) -> Arc<RwLock<XError>>,
    {
        let _index = if let Some(_index) = self.x_error_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_error.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.x_error.write().await.push(None);
            _index
        };

        let x_error = x_error(_index);

        let iter = self.x_error.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *x_error.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(x_error) = found {
            log::debug!(target: "store", "found duplicate {x_error:?}.");
            self.x_error_free_list.lock().await.push(_index);
            x_error.clone()
        } else {
            log::debug!(target: "store", "interring {x_error:?}.");
            self.x_error.write().await[_index] = Some(x_error.clone());
            x_error
        }
    }

    /// Exhume (get) [`XError`] from the store.
    ///
    pub async fn exhume_x_error(&self, id: &usize) -> Option<Arc<RwLock<XError>>> {
        match self.x_error.read().await.get(*id) {
            Some(x_error) => x_error.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XError`] from the store.
    ///
    pub async fn exorcise_x_error(&mut self, id: &usize) -> Option<Arc<RwLock<XError>>> {
        log::debug!(target: "store", "exorcising x_error slot: {id}.");
        let result = self.x_error.write().await[*id].take();
        self.x_error_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XError>`.
    ///
    pub async fn iter_x_error(&self) -> impl stream::Stream<Item = Arc<RwLock<XError>>> + '_ {
        let len = self.x_error.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.x_error.read().await[i].is_some() {
                self.x_error.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`ErrorExpression`] into the store.
    ///
    pub async fn inter_error_expression<F>(
        &mut self,
        error_expression: F,
    ) -> Arc<RwLock<ErrorExpression>>
    where
        F: Fn(usize) -> Arc<RwLock<ErrorExpression>>,
    {
        let _index = if let Some(_index) = self.error_expression_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.error_expression.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.error_expression.write().await.push(None);
            _index
        };

        let error_expression = error_expression(_index);

        let iter = self.error_expression.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *error_expression.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(error_expression) = found {
            log::debug!(target: "store", "found duplicate {error_expression:?}.");
            self.error_expression_free_list.lock().await.push(_index);
            error_expression.clone()
        } else {
            log::debug!(target: "store", "interring {error_expression:?}.");
            self.error_expression.write().await[_index] = Some(error_expression.clone());
            error_expression
        }
    }

    /// Exhume (get) [`ErrorExpression`] from the store.
    ///
    pub async fn exhume_error_expression(
        &self,
        id: &usize,
    ) -> Option<Arc<RwLock<ErrorExpression>>> {
        match self.error_expression.read().await.get(*id) {
            Some(error_expression) => error_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ErrorExpression`] from the store.
    ///
    pub async fn exorcise_error_expression(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<ErrorExpression>>> {
        log::debug!(target: "store", "exorcising error_expression slot: {id}.");
        let result = self.error_expression.write().await[*id].take();
        self.error_expression_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ErrorExpression>`.
    ///
    pub async fn iter_error_expression(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<ErrorExpression>>> + '_ {
        let len = self.error_expression.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.error_expression.read().await[i].is_some() {
                self.error_expression.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`Expression`] into the store.
    ///
    pub async fn inter_expression<F>(&mut self, expression: F) -> Arc<RwLock<Expression>>
    where
        F: Fn(usize) -> Arc<RwLock<Expression>>,
    {
        let _index = if let Some(_index) = self.expression_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.expression.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.expression.write().await.push(None);
            _index
        };

        let expression = expression(_index);

        let iter = self.expression.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *expression.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(expression) = found {
            log::debug!(target: "store", "found duplicate {expression:?}.");
            self.expression_free_list.lock().await.push(_index);
            expression.clone()
        } else {
            log::debug!(target: "store", "interring {expression:?}.");
            self.expression.write().await[_index] = Some(expression.clone());
            expression
        }
    }

    /// Exhume (get) [`Expression`] from the store.
    ///
    pub async fn exhume_expression(&self, id: &usize) -> Option<Arc<RwLock<Expression>>> {
        match self.expression.read().await.get(*id) {
            Some(expression) => expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Expression`] from the store.
    ///
    pub async fn exorcise_expression(&mut self, id: &usize) -> Option<Arc<RwLock<Expression>>> {
        log::debug!(target: "store", "exorcising expression slot: {id}.");
        let result = self.expression.write().await[*id].take();
        self.expression_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Expression>`.
    ///
    pub async fn iter_expression(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<Expression>>> + '_ {
        let len = self.expression.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.expression.read().await[i].is_some() {
                self.expression.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`ExpressionStatement`] into the store.
    ///
    pub async fn inter_expression_statement<F>(
        &mut self,
        expression_statement: F,
    ) -> Arc<RwLock<ExpressionStatement>>
    where
        F: Fn(usize) -> Arc<RwLock<ExpressionStatement>>,
    {
        let _index = if let Some(_index) = self.expression_statement_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.expression_statement.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.expression_statement.write().await.push(None);
            _index
        };

        let expression_statement = expression_statement(_index);

        let iter = self.expression_statement.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *expression_statement.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(expression_statement) = found {
            log::debug!(target: "store", "found duplicate {expression_statement:?}.");
            self.expression_statement_free_list
                .lock()
                .await
                .push(_index);
            expression_statement.clone()
        } else {
            log::debug!(target: "store", "interring {expression_statement:?}.");
            self.expression_statement.write().await[_index] = Some(expression_statement.clone());
            expression_statement
        }
    }

    /// Exhume (get) [`ExpressionStatement`] from the store.
    ///
    pub async fn exhume_expression_statement(
        &self,
        id: &usize,
    ) -> Option<Arc<RwLock<ExpressionStatement>>> {
        match self.expression_statement.read().await.get(*id) {
            Some(expression_statement) => expression_statement.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ExpressionStatement`] from the store.
    ///
    pub async fn exorcise_expression_statement(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<ExpressionStatement>>> {
        log::debug!(target: "store", "exorcising expression_statement slot: {id}.");
        let result = self.expression_statement.write().await[*id].take();
        self.expression_statement_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ExpressionStatement>`.
    ///
    pub async fn iter_expression_statement(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<ExpressionStatement>>> + '_ {
        let len = self.expression_statement.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.expression_statement.read().await[i].is_some() {
                self.expression_statement.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`ExternalImplementation`] into the store.
    ///
    pub async fn inter_external_implementation<F>(
        &mut self,
        external_implementation: F,
    ) -> Arc<RwLock<ExternalImplementation>>
    where
        F: Fn(usize) -> Arc<RwLock<ExternalImplementation>>,
    {
        let _index = if let Some(_index) = self.external_implementation_free_list.lock().await.pop()
        {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.external_implementation.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.external_implementation.write().await.push(None);
            _index
        };

        let external_implementation = external_implementation(_index);

        let iter = self.external_implementation.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *external_implementation.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(external_implementation) = found {
            log::debug!(target: "store", "found duplicate {external_implementation:?}.");
            self.external_implementation_free_list
                .lock()
                .await
                .push(_index);
            external_implementation.clone()
        } else {
            log::debug!(target: "store", "interring {external_implementation:?}.");
            self.external_implementation.write().await[_index] =
                Some(external_implementation.clone());
            external_implementation
        }
    }

    /// Exhume (get) [`ExternalImplementation`] from the store.
    ///
    pub async fn exhume_external_implementation(
        &self,
        id: &usize,
    ) -> Option<Arc<RwLock<ExternalImplementation>>> {
        match self.external_implementation.read().await.get(*id) {
            Some(external_implementation) => external_implementation.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ExternalImplementation`] from the store.
    ///
    pub async fn exorcise_external_implementation(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<ExternalImplementation>>> {
        log::debug!(target: "store", "exorcising external_implementation slot: {id}.");
        let result = self.external_implementation.write().await[*id].take();
        self.external_implementation_free_list
            .lock()
            .await
            .push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ExternalImplementation>`.
    ///
    pub async fn iter_external_implementation(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<ExternalImplementation>>> + '_ {
        let len = self.external_implementation.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.external_implementation.read().await[i].is_some() {
                self.external_implementation.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`Field`] into the store.
    ///
    pub async fn inter_field<F>(&mut self, field: F) -> Arc<RwLock<Field>>
    where
        F: Fn(usize) -> Arc<RwLock<Field>>,
    {
        let _index = if let Some(_index) = self.field_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.field.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.field.write().await.push(None);
            _index
        };

        let field = field(_index);

        let iter = self.field.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *field.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        let field = if let Some(field) = found {
            log::debug!(target: "store", "found duplicate {field:?}.");
            self.field_free_list.lock().await.push(_index);
            field.clone()
        } else {
            log::debug!(target: "store", "interring {field:?}.");
            self.field.write().await[_index] = Some(field.clone());
            field
        };
        self.field_id_by_name
            .write()
            .await
            .insert(field.read().await.name.to_owned(), field.read().await.id);
        field
    }

    /// Exhume (get) [`Field`] from the store.
    ///
    pub async fn exhume_field(&self, id: &usize) -> Option<Arc<RwLock<Field>>> {
        match self.field.read().await.get(*id) {
            Some(field) => field.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Field`] from the store.
    ///
    pub async fn exorcise_field(&mut self, id: &usize) -> Option<Arc<RwLock<Field>>> {
        log::debug!(target: "store", "exorcising field slot: {id}.");
        let result = self.field.write().await[*id].take();
        self.field_free_list.lock().await.push(*id);
        result
    }

    /// Exorcise [`Field`] id from the store by name.
    ///
    pub async fn exhume_field_id_by_name(&self, name: &str) -> Option<usize> {
        self.field_id_by_name
            .read()
            .await
            .get(name)
            .map(|field| *field)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Field>`.
    ///
    pub async fn iter_field(&self) -> impl stream::Stream<Item = Arc<RwLock<Field>>> + '_ {
        let len = self.field.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.field.read().await[i].is_some() {
                self.field.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`FieldAccess`] into the store.
    ///
    pub async fn inter_field_access<F>(&mut self, field_access: F) -> Arc<RwLock<FieldAccess>>
    where
        F: Fn(usize) -> Arc<RwLock<FieldAccess>>,
    {
        let _index = if let Some(_index) = self.field_access_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.field_access.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.field_access.write().await.push(None);
            _index
        };

        let field_access = field_access(_index);

        let iter = self.field_access.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *field_access.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(field_access) = found {
            log::debug!(target: "store", "found duplicate {field_access:?}.");
            self.field_access_free_list.lock().await.push(_index);
            field_access.clone()
        } else {
            log::debug!(target: "store", "interring {field_access:?}.");
            self.field_access.write().await[_index] = Some(field_access.clone());
            field_access
        }
    }

    /// Exhume (get) [`FieldAccess`] from the store.
    ///
    pub async fn exhume_field_access(&self, id: &usize) -> Option<Arc<RwLock<FieldAccess>>> {
        match self.field_access.read().await.get(*id) {
            Some(field_access) => field_access.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FieldAccess`] from the store.
    ///
    pub async fn exorcise_field_access(&mut self, id: &usize) -> Option<Arc<RwLock<FieldAccess>>> {
        log::debug!(target: "store", "exorcising field_access slot: {id}.");
        let result = self.field_access.write().await[*id].take();
        self.field_access_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccess>`.
    ///
    pub async fn iter_field_access(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<FieldAccess>>> + '_ {
        let len = self.field_access.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.field_access.read().await[i].is_some() {
                self.field_access.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`FieldAccessTarget`] into the store.
    ///
    pub async fn inter_field_access_target<F>(
        &mut self,
        field_access_target: F,
    ) -> Arc<RwLock<FieldAccessTarget>>
    where
        F: Fn(usize) -> Arc<RwLock<FieldAccessTarget>>,
    {
        let _index = if let Some(_index) = self.field_access_target_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.field_access_target.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.field_access_target.write().await.push(None);
            _index
        };

        let field_access_target = field_access_target(_index);

        let iter = self.field_access_target.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *field_access_target.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(field_access_target) = found {
            log::debug!(target: "store", "found duplicate {field_access_target:?}.");
            self.field_access_target_free_list.lock().await.push(_index);
            field_access_target.clone()
        } else {
            log::debug!(target: "store", "interring {field_access_target:?}.");
            self.field_access_target.write().await[_index] = Some(field_access_target.clone());
            field_access_target
        }
    }

    /// Exhume (get) [`FieldAccessTarget`] from the store.
    ///
    pub async fn exhume_field_access_target(
        &self,
        id: &usize,
    ) -> Option<Arc<RwLock<FieldAccessTarget>>> {
        match self.field_access_target.read().await.get(*id) {
            Some(field_access_target) => field_access_target.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FieldAccessTarget`] from the store.
    ///
    pub async fn exorcise_field_access_target(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<FieldAccessTarget>>> {
        log::debug!(target: "store", "exorcising field_access_target slot: {id}.");
        let result = self.field_access_target.write().await[*id].take();
        self.field_access_target_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccessTarget>`.
    ///
    pub async fn iter_field_access_target(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<FieldAccessTarget>>> + '_ {
        let len = self.field_access_target.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.field_access_target.read().await[i].is_some() {
                self.field_access_target.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`FieldExpression`] into the store.
    ///
    pub async fn inter_field_expression<F>(
        &mut self,
        field_expression: F,
    ) -> Arc<RwLock<FieldExpression>>
    where
        F: Fn(usize) -> Arc<RwLock<FieldExpression>>,
    {
        let _index = if let Some(_index) = self.field_expression_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.field_expression.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.field_expression.write().await.push(None);
            _index
        };

        let field_expression = field_expression(_index);

        let iter = self.field_expression.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *field_expression.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(field_expression) = found {
            log::debug!(target: "store", "found duplicate {field_expression:?}.");
            self.field_expression_free_list.lock().await.push(_index);
            field_expression.clone()
        } else {
            log::debug!(target: "store", "interring {field_expression:?}.");
            self.field_expression.write().await[_index] = Some(field_expression.clone());
            field_expression
        }
    }

    /// Exhume (get) [`FieldExpression`] from the store.
    ///
    pub async fn exhume_field_expression(
        &self,
        id: &usize,
    ) -> Option<Arc<RwLock<FieldExpression>>> {
        match self.field_expression.read().await.get(*id) {
            Some(field_expression) => field_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FieldExpression`] from the store.
    ///
    pub async fn exorcise_field_expression(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<FieldExpression>>> {
        log::debug!(target: "store", "exorcising field_expression slot: {id}.");
        let result = self.field_expression.write().await[*id].take();
        self.field_expression_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldExpression>`.
    ///
    pub async fn iter_field_expression(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<FieldExpression>>> + '_ {
        let len = self.field_expression.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.field_expression.read().await[i].is_some() {
                self.field_expression.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`FloatLiteral`] into the store.
    ///
    pub async fn inter_float_literal<F>(&mut self, float_literal: F) -> Arc<RwLock<FloatLiteral>>
    where
        F: Fn(usize) -> Arc<RwLock<FloatLiteral>>,
    {
        let _index = if let Some(_index) = self.float_literal_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.float_literal.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.float_literal.write().await.push(None);
            _index
        };

        let float_literal = float_literal(_index);

        let iter = self.float_literal.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *float_literal.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(float_literal) = found {
            log::debug!(target: "store", "found duplicate {float_literal:?}.");
            self.float_literal_free_list.lock().await.push(_index);
            float_literal.clone()
        } else {
            log::debug!(target: "store", "interring {float_literal:?}.");
            self.float_literal.write().await[_index] = Some(float_literal.clone());
            float_literal
        }
    }

    /// Exhume (get) [`FloatLiteral`] from the store.
    ///
    pub async fn exhume_float_literal(&self, id: &usize) -> Option<Arc<RwLock<FloatLiteral>>> {
        match self.float_literal.read().await.get(*id) {
            Some(float_literal) => float_literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FloatLiteral`] from the store.
    ///
    pub async fn exorcise_float_literal(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<FloatLiteral>>> {
        log::debug!(target: "store", "exorcising float_literal slot: {id}.");
        let result = self.float_literal.write().await[*id].take();
        self.float_literal_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FloatLiteral>`.
    ///
    pub async fn iter_float_literal(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<FloatLiteral>>> + '_ {
        let len = self.float_literal.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.float_literal.read().await[i].is_some() {
                self.float_literal.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`ForLoop`] into the store.
    ///
    pub async fn inter_for_loop<F>(&mut self, for_loop: F) -> Arc<RwLock<ForLoop>>
    where
        F: Fn(usize) -> Arc<RwLock<ForLoop>>,
    {
        let _index = if let Some(_index) = self.for_loop_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.for_loop.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.for_loop.write().await.push(None);
            _index
        };

        let for_loop = for_loop(_index);

        let iter = self.for_loop.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *for_loop.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(for_loop) = found {
            log::debug!(target: "store", "found duplicate {for_loop:?}.");
            self.for_loop_free_list.lock().await.push(_index);
            for_loop.clone()
        } else {
            log::debug!(target: "store", "interring {for_loop:?}.");
            self.for_loop.write().await[_index] = Some(for_loop.clone());
            for_loop
        }
    }

    /// Exhume (get) [`ForLoop`] from the store.
    ///
    pub async fn exhume_for_loop(&self, id: &usize) -> Option<Arc<RwLock<ForLoop>>> {
        match self.for_loop.read().await.get(*id) {
            Some(for_loop) => for_loop.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ForLoop`] from the store.
    ///
    pub async fn exorcise_for_loop(&mut self, id: &usize) -> Option<Arc<RwLock<ForLoop>>> {
        log::debug!(target: "store", "exorcising for_loop slot: {id}.");
        let result = self.for_loop.write().await[*id].take();
        self.for_loop_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ForLoop>`.
    ///
    pub async fn iter_for_loop(&self) -> impl stream::Stream<Item = Arc<RwLock<ForLoop>>> + '_ {
        let len = self.for_loop.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.for_loop.read().await[i].is_some() {
                self.for_loop.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`Function`] into the store.
    ///
    pub async fn inter_function<F>(&mut self, function: F) -> Arc<RwLock<Function>>
    where
        F: Fn(usize) -> Arc<RwLock<Function>>,
    {
        let _index = if let Some(_index) = self.function_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.function.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.function.write().await.push(None);
            _index
        };

        let function = function(_index);

        let iter = self.function.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *function.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        let function = if let Some(function) = found {
            log::debug!(target: "store", "found duplicate {function:?}.");
            self.function_free_list.lock().await.push(_index);
            function.clone()
        } else {
            log::debug!(target: "store", "interring {function:?}.");
            self.function.write().await[_index] = Some(function.clone());
            function
        };
        self.function_id_by_name.write().await.insert(
            function.read().await.name.to_owned(),
            function.read().await.id,
        );
        function
    }

    /// Exhume (get) [`Function`] from the store.
    ///
    pub async fn exhume_function(&self, id: &usize) -> Option<Arc<RwLock<Function>>> {
        match self.function.read().await.get(*id) {
            Some(function) => function.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Function`] from the store.
    ///
    pub async fn exorcise_function(&mut self, id: &usize) -> Option<Arc<RwLock<Function>>> {
        log::debug!(target: "store", "exorcising function slot: {id}.");
        let result = self.function.write().await[*id].take();
        self.function_free_list.lock().await.push(*id);
        result
    }

    /// Exorcise [`Function`] id from the store by name.
    ///
    pub async fn exhume_function_id_by_name(&self, name: &str) -> Option<usize> {
        self.function_id_by_name
            .read()
            .await
            .get(name)
            .map(|function| *function)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Function>`.
    ///
    pub async fn iter_function(&self) -> impl stream::Stream<Item = Arc<RwLock<Function>>> + '_ {
        let len = self.function.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.function.read().await[i].is_some() {
                self.function.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`Generic`] into the store.
    ///
    pub async fn inter_generic<F>(&mut self, generic: F) -> Arc<RwLock<Generic>>
    where
        F: Fn(usize) -> Arc<RwLock<Generic>>,
    {
        let _index = if let Some(_index) = self.generic_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.generic.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.generic.write().await.push(None);
            _index
        };

        let generic = generic(_index);

        let iter = self.generic.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *generic.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(generic) = found {
            log::debug!(target: "store", "found duplicate {generic:?}.");
            self.generic_free_list.lock().await.push(_index);
            generic.clone()
        } else {
            log::debug!(target: "store", "interring {generic:?}.");
            self.generic.write().await[_index] = Some(generic.clone());
            generic
        }
    }

    /// Exhume (get) [`Generic`] from the store.
    ///
    pub async fn exhume_generic(&self, id: &usize) -> Option<Arc<RwLock<Generic>>> {
        match self.generic.read().await.get(*id) {
            Some(generic) => generic.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Generic`] from the store.
    ///
    pub async fn exorcise_generic(&mut self, id: &usize) -> Option<Arc<RwLock<Generic>>> {
        log::debug!(target: "store", "exorcising generic slot: {id}.");
        let result = self.generic.write().await[*id].take();
        self.generic_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Generic>`.
    ///
    pub async fn iter_generic(&self) -> impl stream::Stream<Item = Arc<RwLock<Generic>>> + '_ {
        let len = self.generic.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.generic.read().await[i].is_some() {
                self.generic.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`Grouped`] into the store.
    ///
    pub async fn inter_grouped<F>(&mut self, grouped: F) -> Arc<RwLock<Grouped>>
    where
        F: Fn(usize) -> Arc<RwLock<Grouped>>,
    {
        let _index = if let Some(_index) = self.grouped_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.grouped.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.grouped.write().await.push(None);
            _index
        };

        let grouped = grouped(_index);

        let iter = self.grouped.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *grouped.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(grouped) = found {
            log::debug!(target: "store", "found duplicate {grouped:?}.");
            self.grouped_free_list.lock().await.push(_index);
            grouped.clone()
        } else {
            log::debug!(target: "store", "interring {grouped:?}.");
            self.grouped.write().await[_index] = Some(grouped.clone());
            grouped
        }
    }

    /// Exhume (get) [`Grouped`] from the store.
    ///
    pub async fn exhume_grouped(&self, id: &usize) -> Option<Arc<RwLock<Grouped>>> {
        match self.grouped.read().await.get(*id) {
            Some(grouped) => grouped.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Grouped`] from the store.
    ///
    pub async fn exorcise_grouped(&mut self, id: &usize) -> Option<Arc<RwLock<Grouped>>> {
        log::debug!(target: "store", "exorcising grouped slot: {id}.");
        let result = self.grouped.write().await[*id].take();
        self.grouped_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Grouped>`.
    ///
    pub async fn iter_grouped(&self) -> impl stream::Stream<Item = Arc<RwLock<Grouped>>> + '_ {
        let len = self.grouped.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.grouped.read().await[i].is_some() {
                self.grouped.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`XIf`] into the store.
    ///
    pub async fn inter_x_if<F>(&mut self, x_if: F) -> Arc<RwLock<XIf>>
    where
        F: Fn(usize) -> Arc<RwLock<XIf>>,
    {
        let _index = if let Some(_index) = self.x_if_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_if.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.x_if.write().await.push(None);
            _index
        };

        let x_if = x_if(_index);

        let iter = self.x_if.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *x_if.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(x_if) = found {
            log::debug!(target: "store", "found duplicate {x_if:?}.");
            self.x_if_free_list.lock().await.push(_index);
            x_if.clone()
        } else {
            log::debug!(target: "store", "interring {x_if:?}.");
            self.x_if.write().await[_index] = Some(x_if.clone());
            x_if
        }
    }

    /// Exhume (get) [`XIf`] from the store.
    ///
    pub async fn exhume_x_if(&self, id: &usize) -> Option<Arc<RwLock<XIf>>> {
        match self.x_if.read().await.get(*id) {
            Some(x_if) => x_if.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XIf`] from the store.
    ///
    pub async fn exorcise_x_if(&mut self, id: &usize) -> Option<Arc<RwLock<XIf>>> {
        log::debug!(target: "store", "exorcising x_if slot: {id}.");
        let result = self.x_if.write().await[*id].take();
        self.x_if_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XIf>`.
    ///
    pub async fn iter_x_if(&self) -> impl stream::Stream<Item = Arc<RwLock<XIf>>> + '_ {
        let len = self.x_if.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.x_if.read().await[i].is_some() {
                self.x_if.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`ImplementationBlock`] into the store.
    ///
    pub async fn inter_implementation_block<F>(
        &mut self,
        implementation_block: F,
    ) -> Arc<RwLock<ImplementationBlock>>
    where
        F: Fn(usize) -> Arc<RwLock<ImplementationBlock>>,
    {
        let _index = if let Some(_index) = self.implementation_block_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.implementation_block.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.implementation_block.write().await.push(None);
            _index
        };

        let implementation_block = implementation_block(_index);

        let iter = self.implementation_block.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *implementation_block.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(implementation_block) = found {
            log::debug!(target: "store", "found duplicate {implementation_block:?}.");
            self.implementation_block_free_list
                .lock()
                .await
                .push(_index);
            implementation_block.clone()
        } else {
            log::debug!(target: "store", "interring {implementation_block:?}.");
            self.implementation_block.write().await[_index] = Some(implementation_block.clone());
            implementation_block
        }
    }

    /// Exhume (get) [`ImplementationBlock`] from the store.
    ///
    pub async fn exhume_implementation_block(
        &self,
        id: &usize,
    ) -> Option<Arc<RwLock<ImplementationBlock>>> {
        match self.implementation_block.read().await.get(*id) {
            Some(implementation_block) => implementation_block.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ImplementationBlock`] from the store.
    ///
    pub async fn exorcise_implementation_block(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<ImplementationBlock>>> {
        log::debug!(target: "store", "exorcising implementation_block slot: {id}.");
        let result = self.implementation_block.write().await[*id].take();
        self.implementation_block_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ImplementationBlock>`.
    ///
    pub async fn iter_implementation_block(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<ImplementationBlock>>> + '_ {
        let len = self.implementation_block.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.implementation_block.read().await[i].is_some() {
                self.implementation_block.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`Import`] into the store.
    ///
    pub async fn inter_import<F>(&mut self, import: F) -> Arc<RwLock<Import>>
    where
        F: Fn(usize) -> Arc<RwLock<Import>>,
    {
        let _index = if let Some(_index) = self.import_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.import.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.import.write().await.push(None);
            _index
        };

        let import = import(_index);

        let iter = self.import.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *import.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(import) = found {
            log::debug!(target: "store", "found duplicate {import:?}.");
            self.import_free_list.lock().await.push(_index);
            import.clone()
        } else {
            log::debug!(target: "store", "interring {import:?}.");
            self.import.write().await[_index] = Some(import.clone());
            import
        }
    }

    /// Exhume (get) [`Import`] from the store.
    ///
    pub async fn exhume_import(&self, id: &usize) -> Option<Arc<RwLock<Import>>> {
        match self.import.read().await.get(*id) {
            Some(import) => import.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Import`] from the store.
    ///
    pub async fn exorcise_import(&mut self, id: &usize) -> Option<Arc<RwLock<Import>>> {
        log::debug!(target: "store", "exorcising import slot: {id}.");
        let result = self.import.write().await[*id].take();
        self.import_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Import>`.
    ///
    pub async fn iter_import(&self) -> impl stream::Stream<Item = Arc<RwLock<Import>>> + '_ {
        let len = self.import.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.import.read().await[i].is_some() {
                self.import.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`Index`] into the store.
    ///
    pub async fn inter_index<F>(&mut self, index: F) -> Arc<RwLock<Index>>
    where
        F: Fn(usize) -> Arc<RwLock<Index>>,
    {
        let _index = if let Some(_index) = self.index_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.index.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.index.write().await.push(None);
            _index
        };

        let index = index(_index);

        let iter = self.index.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *index.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(index) = found {
            log::debug!(target: "store", "found duplicate {index:?}.");
            self.index_free_list.lock().await.push(_index);
            index.clone()
        } else {
            log::debug!(target: "store", "interring {index:?}.");
            self.index.write().await[_index] = Some(index.clone());
            index
        }
    }

    /// Exhume (get) [`Index`] from the store.
    ///
    pub async fn exhume_index(&self, id: &usize) -> Option<Arc<RwLock<Index>>> {
        match self.index.read().await.get(*id) {
            Some(index) => index.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Index`] from the store.
    ///
    pub async fn exorcise_index(&mut self, id: &usize) -> Option<Arc<RwLock<Index>>> {
        log::debug!(target: "store", "exorcising index slot: {id}.");
        let result = self.index.write().await[*id].take();
        self.index_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Index>`.
    ///
    pub async fn iter_index(&self) -> impl stream::Stream<Item = Arc<RwLock<Index>>> + '_ {
        let len = self.index.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.index.read().await[i].is_some() {
                self.index.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`IntegerLiteral`] into the store.
    ///
    pub async fn inter_integer_literal<F>(
        &mut self,
        integer_literal: F,
    ) -> Arc<RwLock<IntegerLiteral>>
    where
        F: Fn(usize) -> Arc<RwLock<IntegerLiteral>>,
    {
        let _index = if let Some(_index) = self.integer_literal_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.integer_literal.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.integer_literal.write().await.push(None);
            _index
        };

        let integer_literal = integer_literal(_index);

        let iter = self.integer_literal.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *integer_literal.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(integer_literal) = found {
            log::debug!(target: "store", "found duplicate {integer_literal:?}.");
            self.integer_literal_free_list.lock().await.push(_index);
            integer_literal.clone()
        } else {
            log::debug!(target: "store", "interring {integer_literal:?}.");
            self.integer_literal.write().await[_index] = Some(integer_literal.clone());
            integer_literal
        }
    }

    /// Exhume (get) [`IntegerLiteral`] from the store.
    ///
    pub async fn exhume_integer_literal(&self, id: &usize) -> Option<Arc<RwLock<IntegerLiteral>>> {
        match self.integer_literal.read().await.get(*id) {
            Some(integer_literal) => integer_literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`IntegerLiteral`] from the store.
    ///
    pub async fn exorcise_integer_literal(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<IntegerLiteral>>> {
        log::debug!(target: "store", "exorcising integer_literal slot: {id}.");
        let result = self.integer_literal.write().await[*id].take();
        self.integer_literal_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, IntegerLiteral>`.
    ///
    pub async fn iter_integer_literal(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<IntegerLiteral>>> + '_ {
        let len = self.integer_literal.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.integer_literal.read().await[i].is_some() {
                self.integer_literal.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`Item`] into the store.
    ///
    pub async fn inter_item<F>(&mut self, item: F) -> Arc<RwLock<Item>>
    where
        F: Fn(usize) -> Arc<RwLock<Item>>,
    {
        let _index = if let Some(_index) = self.item_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.item.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.item.write().await.push(None);
            _index
        };

        let item = item(_index);

        let iter = self.item.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *item.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(item) = found {
            log::debug!(target: "store", "found duplicate {item:?}.");
            self.item_free_list.lock().await.push(_index);
            item.clone()
        } else {
            log::debug!(target: "store", "interring {item:?}.");
            self.item.write().await[_index] = Some(item.clone());
            item
        }
    }

    /// Exhume (get) [`Item`] from the store.
    ///
    pub async fn exhume_item(&self, id: &usize) -> Option<Arc<RwLock<Item>>> {
        match self.item.read().await.get(*id) {
            Some(item) => item.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Item`] from the store.
    ///
    pub async fn exorcise_item(&mut self, id: &usize) -> Option<Arc<RwLock<Item>>> {
        log::debug!(target: "store", "exorcising item slot: {id}.");
        let result = self.item.write().await[*id].take();
        self.item_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Item>`.
    ///
    pub async fn iter_item(&self) -> impl stream::Stream<Item = Arc<RwLock<Item>>> + '_ {
        let len = self.item.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.item.read().await[i].is_some() {
                self.item.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`Lambda`] into the store.
    ///
    pub async fn inter_lambda<F>(&mut self, lambda: F) -> Arc<RwLock<Lambda>>
    where
        F: Fn(usize) -> Arc<RwLock<Lambda>>,
    {
        let _index = if let Some(_index) = self.lambda_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.lambda.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.lambda.write().await.push(None);
            _index
        };

        let lambda = lambda(_index);

        let iter = self.lambda.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *lambda.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(lambda) = found {
            log::debug!(target: "store", "found duplicate {lambda:?}.");
            self.lambda_free_list.lock().await.push(_index);
            lambda.clone()
        } else {
            log::debug!(target: "store", "interring {lambda:?}.");
            self.lambda.write().await[_index] = Some(lambda.clone());
            lambda
        }
    }

    /// Exhume (get) [`Lambda`] from the store.
    ///
    pub async fn exhume_lambda(&self, id: &usize) -> Option<Arc<RwLock<Lambda>>> {
        match self.lambda.read().await.get(*id) {
            Some(lambda) => lambda.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Lambda`] from the store.
    ///
    pub async fn exorcise_lambda(&mut self, id: &usize) -> Option<Arc<RwLock<Lambda>>> {
        log::debug!(target: "store", "exorcising lambda slot: {id}.");
        let result = self.lambda.write().await[*id].take();
        self.lambda_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Lambda>`.
    ///
    pub async fn iter_lambda(&self) -> impl stream::Stream<Item = Arc<RwLock<Lambda>>> + '_ {
        let len = self.lambda.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.lambda.read().await[i].is_some() {
                self.lambda.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`LambdaParameter`] into the store.
    ///
    pub async fn inter_lambda_parameter<F>(
        &mut self,
        lambda_parameter: F,
    ) -> Arc<RwLock<LambdaParameter>>
    where
        F: Fn(usize) -> Arc<RwLock<LambdaParameter>>,
    {
        let _index = if let Some(_index) = self.lambda_parameter_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.lambda_parameter.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.lambda_parameter.write().await.push(None);
            _index
        };

        let lambda_parameter = lambda_parameter(_index);

        let iter = self.lambda_parameter.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *lambda_parameter.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(lambda_parameter) = found {
            log::debug!(target: "store", "found duplicate {lambda_parameter:?}.");
            self.lambda_parameter_free_list.lock().await.push(_index);
            lambda_parameter.clone()
        } else {
            log::debug!(target: "store", "interring {lambda_parameter:?}.");
            self.lambda_parameter.write().await[_index] = Some(lambda_parameter.clone());
            lambda_parameter
        }
    }

    /// Exhume (get) [`LambdaParameter`] from the store.
    ///
    pub async fn exhume_lambda_parameter(
        &self,
        id: &usize,
    ) -> Option<Arc<RwLock<LambdaParameter>>> {
        match self.lambda_parameter.read().await.get(*id) {
            Some(lambda_parameter) => lambda_parameter.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`LambdaParameter`] from the store.
    ///
    pub async fn exorcise_lambda_parameter(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<LambdaParameter>>> {
        log::debug!(target: "store", "exorcising lambda_parameter slot: {id}.");
        let result = self.lambda_parameter.write().await[*id].take();
        self.lambda_parameter_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LambdaParameter>`.
    ///
    pub async fn iter_lambda_parameter(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<LambdaParameter>>> + '_ {
        let len = self.lambda_parameter.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.lambda_parameter.read().await[i].is_some() {
                self.lambda_parameter.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`LetStatement`] into the store.
    ///
    pub async fn inter_let_statement<F>(&mut self, let_statement: F) -> Arc<RwLock<LetStatement>>
    where
        F: Fn(usize) -> Arc<RwLock<LetStatement>>,
    {
        let _index = if let Some(_index) = self.let_statement_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.let_statement.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.let_statement.write().await.push(None);
            _index
        };

        let let_statement = let_statement(_index);

        let iter = self.let_statement.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *let_statement.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(let_statement) = found {
            log::debug!(target: "store", "found duplicate {let_statement:?}.");
            self.let_statement_free_list.lock().await.push(_index);
            let_statement.clone()
        } else {
            log::debug!(target: "store", "interring {let_statement:?}.");
            self.let_statement.write().await[_index] = Some(let_statement.clone());
            let_statement
        }
    }

    /// Exhume (get) [`LetStatement`] from the store.
    ///
    pub async fn exhume_let_statement(&self, id: &usize) -> Option<Arc<RwLock<LetStatement>>> {
        match self.let_statement.read().await.get(*id) {
            Some(let_statement) => let_statement.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`LetStatement`] from the store.
    ///
    pub async fn exorcise_let_statement(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<LetStatement>>> {
        log::debug!(target: "store", "exorcising let_statement slot: {id}.");
        let result = self.let_statement.write().await[*id].take();
        self.let_statement_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LetStatement>`.
    ///
    pub async fn iter_let_statement(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<LetStatement>>> + '_ {
        let len = self.let_statement.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.let_statement.read().await[i].is_some() {
                self.let_statement.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`List`] into the store.
    ///
    pub async fn inter_list<F>(&mut self, list: F) -> Arc<RwLock<List>>
    where
        F: Fn(usize) -> Arc<RwLock<List>>,
    {
        let _index = if let Some(_index) = self.list_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.list.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.list.write().await.push(None);
            _index
        };

        let list = list(_index);

        let iter = self.list.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *list.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(list) = found {
            log::debug!(target: "store", "found duplicate {list:?}.");
            self.list_free_list.lock().await.push(_index);
            list.clone()
        } else {
            log::debug!(target: "store", "interring {list:?}.");
            self.list.write().await[_index] = Some(list.clone());
            list
        }
    }

    /// Exhume (get) [`List`] from the store.
    ///
    pub async fn exhume_list(&self, id: &usize) -> Option<Arc<RwLock<List>>> {
        match self.list.read().await.get(*id) {
            Some(list) => list.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`List`] from the store.
    ///
    pub async fn exorcise_list(&mut self, id: &usize) -> Option<Arc<RwLock<List>>> {
        log::debug!(target: "store", "exorcising list slot: {id}.");
        let result = self.list.write().await[*id].take();
        self.list_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, List>`.
    ///
    pub async fn iter_list(&self) -> impl stream::Stream<Item = Arc<RwLock<List>>> + '_ {
        let len = self.list.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.list.read().await[i].is_some() {
                self.list.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`ListElement`] into the store.
    ///
    pub async fn inter_list_element<F>(&mut self, list_element: F) -> Arc<RwLock<ListElement>>
    where
        F: Fn(usize) -> Arc<RwLock<ListElement>>,
    {
        let _index = if let Some(_index) = self.list_element_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.list_element.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.list_element.write().await.push(None);
            _index
        };

        let list_element = list_element(_index);

        let iter = self.list_element.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *list_element.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(list_element) = found {
            log::debug!(target: "store", "found duplicate {list_element:?}.");
            self.list_element_free_list.lock().await.push(_index);
            list_element.clone()
        } else {
            log::debug!(target: "store", "interring {list_element:?}.");
            self.list_element.write().await[_index] = Some(list_element.clone());
            list_element
        }
    }

    /// Exhume (get) [`ListElement`] from the store.
    ///
    pub async fn exhume_list_element(&self, id: &usize) -> Option<Arc<RwLock<ListElement>>> {
        match self.list_element.read().await.get(*id) {
            Some(list_element) => list_element.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ListElement`] from the store.
    ///
    pub async fn exorcise_list_element(&mut self, id: &usize) -> Option<Arc<RwLock<ListElement>>> {
        log::debug!(target: "store", "exorcising list_element slot: {id}.");
        let result = self.list_element.write().await[*id].take();
        self.list_element_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListElement>`.
    ///
    pub async fn iter_list_element(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<ListElement>>> + '_ {
        let len = self.list_element.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.list_element.read().await[i].is_some() {
                self.list_element.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`ListExpression`] into the store.
    ///
    pub async fn inter_list_expression<F>(
        &mut self,
        list_expression: F,
    ) -> Arc<RwLock<ListExpression>>
    where
        F: Fn(usize) -> Arc<RwLock<ListExpression>>,
    {
        let _index = if let Some(_index) = self.list_expression_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.list_expression.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.list_expression.write().await.push(None);
            _index
        };

        let list_expression = list_expression(_index);

        let iter = self.list_expression.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *list_expression.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(list_expression) = found {
            log::debug!(target: "store", "found duplicate {list_expression:?}.");
            self.list_expression_free_list.lock().await.push(_index);
            list_expression.clone()
        } else {
            log::debug!(target: "store", "interring {list_expression:?}.");
            self.list_expression.write().await[_index] = Some(list_expression.clone());
            list_expression
        }
    }

    /// Exhume (get) [`ListExpression`] from the store.
    ///
    pub async fn exhume_list_expression(&self, id: &usize) -> Option<Arc<RwLock<ListExpression>>> {
        match self.list_expression.read().await.get(*id) {
            Some(list_expression) => list_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ListExpression`] from the store.
    ///
    pub async fn exorcise_list_expression(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<ListExpression>>> {
        log::debug!(target: "store", "exorcising list_expression slot: {id}.");
        let result = self.list_expression.write().await[*id].take();
        self.list_expression_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListExpression>`.
    ///
    pub async fn iter_list_expression(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<ListExpression>>> + '_ {
        let len = self.list_expression.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.list_expression.read().await[i].is_some() {
                self.list_expression.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`Literal`] into the store.
    ///
    pub async fn inter_literal<F>(&mut self, literal: F) -> Arc<RwLock<Literal>>
    where
        F: Fn(usize) -> Arc<RwLock<Literal>>,
    {
        let _index = if let Some(_index) = self.literal_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.literal.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.literal.write().await.push(None);
            _index
        };

        let literal = literal(_index);

        let iter = self.literal.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *literal.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(literal) = found {
            log::debug!(target: "store", "found duplicate {literal:?}.");
            self.literal_free_list.lock().await.push(_index);
            literal.clone()
        } else {
            log::debug!(target: "store", "interring {literal:?}.");
            self.literal.write().await[_index] = Some(literal.clone());
            literal
        }
    }

    /// Exhume (get) [`Literal`] from the store.
    ///
    pub async fn exhume_literal(&self, id: &usize) -> Option<Arc<RwLock<Literal>>> {
        match self.literal.read().await.get(*id) {
            Some(literal) => literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Literal`] from the store.
    ///
    pub async fn exorcise_literal(&mut self, id: &usize) -> Option<Arc<RwLock<Literal>>> {
        log::debug!(target: "store", "exorcising literal slot: {id}.");
        let result = self.literal.write().await[*id].take();
        self.literal_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Literal>`.
    ///
    pub async fn iter_literal(&self) -> impl stream::Stream<Item = Arc<RwLock<Literal>>> + '_ {
        let len = self.literal.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.literal.read().await[i].is_some() {
                self.literal.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`LocalVariable`] into the store.
    ///
    pub async fn inter_local_variable<F>(&mut self, local_variable: F) -> Arc<RwLock<LocalVariable>>
    where
        F: Fn(usize) -> Arc<RwLock<LocalVariable>>,
    {
        let _index = if let Some(_index) = self.local_variable_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.local_variable.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.local_variable.write().await.push(None);
            _index
        };

        let local_variable = local_variable(_index);

        let iter = self.local_variable.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *local_variable.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(local_variable) = found {
            log::debug!(target: "store", "found duplicate {local_variable:?}.");
            self.local_variable_free_list.lock().await.push(_index);
            local_variable.clone()
        } else {
            log::debug!(target: "store", "interring {local_variable:?}.");
            self.local_variable.write().await[_index] = Some(local_variable.clone());
            local_variable
        }
    }

    /// Exhume (get) [`LocalVariable`] from the store.
    ///
    pub async fn exhume_local_variable(&self, id: &usize) -> Option<Arc<RwLock<LocalVariable>>> {
        match self.local_variable.read().await.get(*id) {
            Some(local_variable) => local_variable.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`LocalVariable`] from the store.
    ///
    pub async fn exorcise_local_variable(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<LocalVariable>>> {
        log::debug!(target: "store", "exorcising local_variable slot: {id}.");
        let result = self.local_variable.write().await[*id].take();
        self.local_variable_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LocalVariable>`.
    ///
    pub async fn iter_local_variable(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<LocalVariable>>> + '_ {
        let len = self.local_variable.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.local_variable.read().await[i].is_some() {
                self.local_variable.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`XMacro`] into the store.
    ///
    pub async fn inter_x_macro<F>(&mut self, x_macro: F) -> Arc<RwLock<XMacro>>
    where
        F: Fn(usize) -> Arc<RwLock<XMacro>>,
    {
        let _index = if let Some(_index) = self.x_macro_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_macro.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.x_macro.write().await.push(None);
            _index
        };

        let x_macro = x_macro(_index);

        let iter = self.x_macro.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *x_macro.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(x_macro) = found {
            log::debug!(target: "store", "found duplicate {x_macro:?}.");
            self.x_macro_free_list.lock().await.push(_index);
            x_macro.clone()
        } else {
            log::debug!(target: "store", "interring {x_macro:?}.");
            self.x_macro.write().await[_index] = Some(x_macro.clone());
            x_macro
        }
    }

    /// Exhume (get) [`XMacro`] from the store.
    ///
    pub async fn exhume_x_macro(&self, id: &usize) -> Option<Arc<RwLock<XMacro>>> {
        match self.x_macro.read().await.get(*id) {
            Some(x_macro) => x_macro.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XMacro`] from the store.
    ///
    pub async fn exorcise_x_macro(&mut self, id: &usize) -> Option<Arc<RwLock<XMacro>>> {
        log::debug!(target: "store", "exorcising x_macro slot: {id}.");
        let result = self.x_macro.write().await[*id].take();
        self.x_macro_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XMacro>`.
    ///
    pub async fn iter_x_macro(&self) -> impl stream::Stream<Item = Arc<RwLock<XMacro>>> + '_ {
        let len = self.x_macro.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.x_macro.read().await[i].is_some() {
                self.x_macro.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`XMatch`] into the store.
    ///
    pub async fn inter_x_match<F>(&mut self, x_match: F) -> Arc<RwLock<XMatch>>
    where
        F: Fn(usize) -> Arc<RwLock<XMatch>>,
    {
        let _index = if let Some(_index) = self.x_match_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_match.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.x_match.write().await.push(None);
            _index
        };

        let x_match = x_match(_index);

        let iter = self.x_match.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *x_match.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(x_match) = found {
            log::debug!(target: "store", "found duplicate {x_match:?}.");
            self.x_match_free_list.lock().await.push(_index);
            x_match.clone()
        } else {
            log::debug!(target: "store", "interring {x_match:?}.");
            self.x_match.write().await[_index] = Some(x_match.clone());
            x_match
        }
    }

    /// Exhume (get) [`XMatch`] from the store.
    ///
    pub async fn exhume_x_match(&self, id: &usize) -> Option<Arc<RwLock<XMatch>>> {
        match self.x_match.read().await.get(*id) {
            Some(x_match) => x_match.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XMatch`] from the store.
    ///
    pub async fn exorcise_x_match(&mut self, id: &usize) -> Option<Arc<RwLock<XMatch>>> {
        log::debug!(target: "store", "exorcising x_match slot: {id}.");
        let result = self.x_match.write().await[*id].take();
        self.x_match_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XMatch>`.
    ///
    pub async fn iter_x_match(&self) -> impl stream::Stream<Item = Arc<RwLock<XMatch>>> + '_ {
        let len = self.x_match.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.x_match.read().await[i].is_some() {
                self.x_match.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`MethodCall`] into the store.
    ///
    pub async fn inter_method_call<F>(&mut self, method_call: F) -> Arc<RwLock<MethodCall>>
    where
        F: Fn(usize) -> Arc<RwLock<MethodCall>>,
    {
        let _index = if let Some(_index) = self.method_call_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.method_call.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.method_call.write().await.push(None);
            _index
        };

        let method_call = method_call(_index);

        let iter = self.method_call.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *method_call.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(method_call) = found {
            log::debug!(target: "store", "found duplicate {method_call:?}.");
            self.method_call_free_list.lock().await.push(_index);
            method_call.clone()
        } else {
            log::debug!(target: "store", "interring {method_call:?}.");
            self.method_call.write().await[_index] = Some(method_call.clone());
            method_call
        }
    }

    /// Exhume (get) [`MethodCall`] from the store.
    ///
    pub async fn exhume_method_call(&self, id: &usize) -> Option<Arc<RwLock<MethodCall>>> {
        match self.method_call.read().await.get(*id) {
            Some(method_call) => method_call.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`MethodCall`] from the store.
    ///
    pub async fn exorcise_method_call(&mut self, id: &usize) -> Option<Arc<RwLock<MethodCall>>> {
        log::debug!(target: "store", "exorcising method_call slot: {id}.");
        let result = self.method_call.write().await[*id].take();
        self.method_call_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, MethodCall>`.
    ///
    pub async fn iter_method_call(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<MethodCall>>> + '_ {
        let len = self.method_call.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.method_call.read().await[i].is_some() {
                self.method_call.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`ZObjectStore`] into the store.
    ///
    pub async fn inter_z_object_store<F>(&mut self, z_object_store: F) -> Arc<RwLock<ZObjectStore>>
    where
        F: Fn(usize) -> Arc<RwLock<ZObjectStore>>,
    {
        let _index = if let Some(_index) = self.z_object_store_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.z_object_store.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.z_object_store.write().await.push(None);
            _index
        };

        let z_object_store = z_object_store(_index);

        let iter = self.z_object_store.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *z_object_store.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        let z_object_store = if let Some(z_object_store) = found {
            log::debug!(target: "store", "found duplicate {z_object_store:?}.");
            self.z_object_store_free_list.lock().await.push(_index);
            z_object_store.clone()
        } else {
            log::debug!(target: "store", "interring {z_object_store:?}.");
            self.z_object_store.write().await[_index] = Some(z_object_store.clone());
            z_object_store
        };
        self.z_object_store_id_by_name.write().await.insert(
            z_object_store.read().await.name.to_owned(),
            z_object_store.read().await.id,
        );
        z_object_store
    }

    /// Exhume (get) [`ZObjectStore`] from the store.
    ///
    pub async fn exhume_z_object_store(&self, id: &usize) -> Option<Arc<RwLock<ZObjectStore>>> {
        match self.z_object_store.read().await.get(*id) {
            Some(z_object_store) => z_object_store.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ZObjectStore`] from the store.
    ///
    pub async fn exorcise_z_object_store(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<ZObjectStore>>> {
        log::debug!(target: "store", "exorcising z_object_store slot: {id}.");
        let result = self.z_object_store.write().await[*id].take();
        self.z_object_store_free_list.lock().await.push(*id);
        result
    }

    /// Exorcise [`ZObjectStore`] id from the store by name.
    ///
    pub async fn exhume_z_object_store_id_by_name(&self, name: &str) -> Option<usize> {
        self.z_object_store_id_by_name
            .read()
            .await
            .get(name)
            .map(|z_object_store| *z_object_store)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ZObjectStore>`.
    ///
    pub async fn iter_z_object_store(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<ZObjectStore>>> + '_ {
        let len = self.z_object_store.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.z_object_store.read().await[i].is_some() {
                self.z_object_store.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`ObjectWrapper`] into the store.
    ///
    pub async fn inter_object_wrapper<F>(&mut self, object_wrapper: F) -> Arc<RwLock<ObjectWrapper>>
    where
        F: Fn(usize) -> Arc<RwLock<ObjectWrapper>>,
    {
        let _index = if let Some(_index) = self.object_wrapper_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.object_wrapper.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.object_wrapper.write().await.push(None);
            _index
        };

        let object_wrapper = object_wrapper(_index);

        let iter = self.object_wrapper.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *object_wrapper.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(object_wrapper) = found {
            log::debug!(target: "store", "found duplicate {object_wrapper:?}.");
            self.object_wrapper_free_list.lock().await.push(_index);
            object_wrapper.clone()
        } else {
            log::debug!(target: "store", "interring {object_wrapper:?}.");
            self.object_wrapper.write().await[_index] = Some(object_wrapper.clone());
            object_wrapper
        }
    }

    /// Exhume (get) [`ObjectWrapper`] from the store.
    ///
    pub async fn exhume_object_wrapper(&self, id: &usize) -> Option<Arc<RwLock<ObjectWrapper>>> {
        match self.object_wrapper.read().await.get(*id) {
            Some(object_wrapper) => object_wrapper.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ObjectWrapper`] from the store.
    ///
    pub async fn exorcise_object_wrapper(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<ObjectWrapper>>> {
        log::debug!(target: "store", "exorcising object_wrapper slot: {id}.");
        let result = self.object_wrapper.write().await[*id].take();
        self.object_wrapper_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ObjectWrapper>`.
    ///
    pub async fn iter_object_wrapper(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<ObjectWrapper>>> + '_ {
        let len = self.object_wrapper.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.object_wrapper.read().await[i].is_some() {
                self.object_wrapper.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`Operator`] into the store.
    ///
    pub async fn inter_operator<F>(&mut self, operator: F) -> Arc<RwLock<Operator>>
    where
        F: Fn(usize) -> Arc<RwLock<Operator>>,
    {
        let _index = if let Some(_index) = self.operator_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.operator.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.operator.write().await.push(None);
            _index
        };

        let operator = operator(_index);

        let iter = self.operator.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *operator.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(operator) = found {
            log::debug!(target: "store", "found duplicate {operator:?}.");
            self.operator_free_list.lock().await.push(_index);
            operator.clone()
        } else {
            log::debug!(target: "store", "interring {operator:?}.");
            self.operator.write().await[_index] = Some(operator.clone());
            operator
        }
    }

    /// Exhume (get) [`Operator`] from the store.
    ///
    pub async fn exhume_operator(&self, id: &usize) -> Option<Arc<RwLock<Operator>>> {
        match self.operator.read().await.get(*id) {
            Some(operator) => operator.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Operator`] from the store.
    ///
    pub async fn exorcise_operator(&mut self, id: &usize) -> Option<Arc<RwLock<Operator>>> {
        log::debug!(target: "store", "exorcising operator slot: {id}.");
        let result = self.operator.write().await[*id].take();
        self.operator_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Operator>`.
    ///
    pub async fn iter_operator(&self) -> impl stream::Stream<Item = Arc<RwLock<Operator>>> + '_ {
        let len = self.operator.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.operator.read().await[i].is_some() {
                self.operator.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`WoogOption`] into the store.
    ///
    pub async fn inter_woog_option<F>(&mut self, woog_option: F) -> Arc<RwLock<WoogOption>>
    where
        F: Fn(usize) -> Arc<RwLock<WoogOption>>,
    {
        let _index = if let Some(_index) = self.woog_option_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.woog_option.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.woog_option.write().await.push(None);
            _index
        };

        let woog_option = woog_option(_index);

        let iter = self.woog_option.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *woog_option.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(woog_option) = found {
            log::debug!(target: "store", "found duplicate {woog_option:?}.");
            self.woog_option_free_list.lock().await.push(_index);
            woog_option.clone()
        } else {
            log::debug!(target: "store", "interring {woog_option:?}.");
            self.woog_option.write().await[_index] = Some(woog_option.clone());
            woog_option
        }
    }

    /// Exhume (get) [`WoogOption`] from the store.
    ///
    pub async fn exhume_woog_option(&self, id: &usize) -> Option<Arc<RwLock<WoogOption>>> {
        match self.woog_option.read().await.get(*id) {
            Some(woog_option) => woog_option.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`WoogOption`] from the store.
    ///
    pub async fn exorcise_woog_option(&mut self, id: &usize) -> Option<Arc<RwLock<WoogOption>>> {
        log::debug!(target: "store", "exorcising woog_option slot: {id}.");
        let result = self.woog_option.write().await[*id].take();
        self.woog_option_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, WoogOption>`.
    ///
    pub async fn iter_woog_option(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<WoogOption>>> + '_ {
        let len = self.woog_option.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.woog_option.read().await[i].is_some() {
                self.woog_option.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`Parameter`] into the store.
    ///
    pub async fn inter_parameter<F>(&mut self, parameter: F) -> Arc<RwLock<Parameter>>
    where
        F: Fn(usize) -> Arc<RwLock<Parameter>>,
    {
        let _index = if let Some(_index) = self.parameter_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.parameter.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.parameter.write().await.push(None);
            _index
        };

        let parameter = parameter(_index);

        let iter = self.parameter.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *parameter.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(parameter) = found {
            log::debug!(target: "store", "found duplicate {parameter:?}.");
            self.parameter_free_list.lock().await.push(_index);
            parameter.clone()
        } else {
            log::debug!(target: "store", "interring {parameter:?}.");
            self.parameter.write().await[_index] = Some(parameter.clone());
            parameter
        }
    }

    /// Exhume (get) [`Parameter`] from the store.
    ///
    pub async fn exhume_parameter(&self, id: &usize) -> Option<Arc<RwLock<Parameter>>> {
        match self.parameter.read().await.get(*id) {
            Some(parameter) => parameter.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Parameter`] from the store.
    ///
    pub async fn exorcise_parameter(&mut self, id: &usize) -> Option<Arc<RwLock<Parameter>>> {
        log::debug!(target: "store", "exorcising parameter slot: {id}.");
        let result = self.parameter.write().await[*id].take();
        self.parameter_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Parameter>`.
    ///
    pub async fn iter_parameter(&self) -> impl stream::Stream<Item = Arc<RwLock<Parameter>>> + '_ {
        let len = self.parameter.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.parameter.read().await[i].is_some() {
                self.parameter.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`Pattern`] into the store.
    ///
    pub async fn inter_pattern<F>(&mut self, pattern: F) -> Arc<RwLock<Pattern>>
    where
        F: Fn(usize) -> Arc<RwLock<Pattern>>,
    {
        let _index = if let Some(_index) = self.pattern_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.pattern.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.pattern.write().await.push(None);
            _index
        };

        let pattern = pattern(_index);

        let iter = self.pattern.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *pattern.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(pattern) = found {
            log::debug!(target: "store", "found duplicate {pattern:?}.");
            self.pattern_free_list.lock().await.push(_index);
            pattern.clone()
        } else {
            log::debug!(target: "store", "interring {pattern:?}.");
            self.pattern.write().await[_index] = Some(pattern.clone());
            pattern
        }
    }

    /// Exhume (get) [`Pattern`] from the store.
    ///
    pub async fn exhume_pattern(&self, id: &usize) -> Option<Arc<RwLock<Pattern>>> {
        match self.pattern.read().await.get(*id) {
            Some(pattern) => pattern.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Pattern`] from the store.
    ///
    pub async fn exorcise_pattern(&mut self, id: &usize) -> Option<Arc<RwLock<Pattern>>> {
        log::debug!(target: "store", "exorcising pattern slot: {id}.");
        let result = self.pattern.write().await[*id].take();
        self.pattern_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Pattern>`.
    ///
    pub async fn iter_pattern(&self) -> impl stream::Stream<Item = Arc<RwLock<Pattern>>> + '_ {
        let len = self.pattern.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.pattern.read().await[i].is_some() {
                self.pattern.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`Plain`] into the store.
    ///
    pub async fn inter_plain<F>(&mut self, plain: F) -> Arc<RwLock<Plain>>
    where
        F: Fn(usize) -> Arc<RwLock<Plain>>,
    {
        let _index = if let Some(_index) = self.plain_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.plain.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.plain.write().await.push(None);
            _index
        };

        let plain = plain(_index);

        let iter = self.plain.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *plain.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(plain) = found {
            log::debug!(target: "store", "found duplicate {plain:?}.");
            self.plain_free_list.lock().await.push(_index);
            plain.clone()
        } else {
            log::debug!(target: "store", "interring {plain:?}.");
            self.plain.write().await[_index] = Some(plain.clone());
            plain
        }
    }

    /// Exhume (get) [`Plain`] from the store.
    ///
    pub async fn exhume_plain(&self, id: &usize) -> Option<Arc<RwLock<Plain>>> {
        match self.plain.read().await.get(*id) {
            Some(plain) => plain.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Plain`] from the store.
    ///
    pub async fn exorcise_plain(&mut self, id: &usize) -> Option<Arc<RwLock<Plain>>> {
        log::debug!(target: "store", "exorcising plain slot: {id}.");
        let result = self.plain.write().await[*id].take();
        self.plain_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Plain>`.
    ///
    pub async fn iter_plain(&self) -> impl stream::Stream<Item = Arc<RwLock<Plain>>> + '_ {
        let len = self.plain.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.plain.read().await[i].is_some() {
                self.plain.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`XPrint`] into the store.
    ///
    pub async fn inter_x_print<F>(&mut self, x_print: F) -> Arc<RwLock<XPrint>>
    where
        F: Fn(usize) -> Arc<RwLock<XPrint>>,
    {
        let _index = if let Some(_index) = self.x_print_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_print.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.x_print.write().await.push(None);
            _index
        };

        let x_print = x_print(_index);

        let iter = self.x_print.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *x_print.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(x_print) = found {
            log::debug!(target: "store", "found duplicate {x_print:?}.");
            self.x_print_free_list.lock().await.push(_index);
            x_print.clone()
        } else {
            log::debug!(target: "store", "interring {x_print:?}.");
            self.x_print.write().await[_index] = Some(x_print.clone());
            x_print
        }
    }

    /// Exhume (get) [`XPrint`] from the store.
    ///
    pub async fn exhume_x_print(&self, id: &usize) -> Option<Arc<RwLock<XPrint>>> {
        match self.x_print.read().await.get(*id) {
            Some(x_print) => x_print.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XPrint`] from the store.
    ///
    pub async fn exorcise_x_print(&mut self, id: &usize) -> Option<Arc<RwLock<XPrint>>> {
        log::debug!(target: "store", "exorcising x_print slot: {id}.");
        let result = self.x_print.write().await[*id].take();
        self.x_print_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XPrint>`.
    ///
    pub async fn iter_x_print(&self) -> impl stream::Stream<Item = Arc<RwLock<XPrint>>> + '_ {
        let len = self.x_print.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.x_print.read().await[i].is_some() {
                self.x_print.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`RangeExpression`] into the store.
    ///
    pub async fn inter_range_expression<F>(
        &mut self,
        range_expression: F,
    ) -> Arc<RwLock<RangeExpression>>
    where
        F: Fn(usize) -> Arc<RwLock<RangeExpression>>,
    {
        let _index = if let Some(_index) = self.range_expression_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.range_expression.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.range_expression.write().await.push(None);
            _index
        };

        let range_expression = range_expression(_index);

        let iter = self.range_expression.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *range_expression.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(range_expression) = found {
            log::debug!(target: "store", "found duplicate {range_expression:?}.");
            self.range_expression_free_list.lock().await.push(_index);
            range_expression.clone()
        } else {
            log::debug!(target: "store", "interring {range_expression:?}.");
            self.range_expression.write().await[_index] = Some(range_expression.clone());
            range_expression
        }
    }

    /// Exhume (get) [`RangeExpression`] from the store.
    ///
    pub async fn exhume_range_expression(
        &self,
        id: &usize,
    ) -> Option<Arc<RwLock<RangeExpression>>> {
        match self.range_expression.read().await.get(*id) {
            Some(range_expression) => range_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`RangeExpression`] from the store.
    ///
    pub async fn exorcise_range_expression(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<RangeExpression>>> {
        log::debug!(target: "store", "exorcising range_expression slot: {id}.");
        let result = self.range_expression.write().await[*id].take();
        self.range_expression_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RangeExpression>`.
    ///
    pub async fn iter_range_expression(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<RangeExpression>>> + '_ {
        let len = self.range_expression.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.range_expression.read().await[i].is_some() {
                self.range_expression.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`Reference`] into the store.
    ///
    pub async fn inter_reference<F>(&mut self, reference: F) -> Arc<RwLock<Reference>>
    where
        F: Fn(usize) -> Arc<RwLock<Reference>>,
    {
        let _index = if let Some(_index) = self.reference_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.reference.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.reference.write().await.push(None);
            _index
        };

        let reference = reference(_index);

        let iter = self.reference.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *reference.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(reference) = found {
            log::debug!(target: "store", "found duplicate {reference:?}.");
            self.reference_free_list.lock().await.push(_index);
            reference.clone()
        } else {
            log::debug!(target: "store", "interring {reference:?}.");
            self.reference.write().await[_index] = Some(reference.clone());
            reference
        }
    }

    /// Exhume (get) [`Reference`] from the store.
    ///
    pub async fn exhume_reference(&self, id: &usize) -> Option<Arc<RwLock<Reference>>> {
        match self.reference.read().await.get(*id) {
            Some(reference) => reference.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Reference`] from the store.
    ///
    pub async fn exorcise_reference(&mut self, id: &usize) -> Option<Arc<RwLock<Reference>>> {
        log::debug!(target: "store", "exorcising reference slot: {id}.");
        let result = self.reference.write().await[*id].take();
        self.reference_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Reference>`.
    ///
    pub async fn iter_reference(&self) -> impl stream::Stream<Item = Arc<RwLock<Reference>>> + '_ {
        let len = self.reference.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.reference.read().await[i].is_some() {
                self.reference.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`ResultStatement`] into the store.
    ///
    pub async fn inter_result_statement<F>(
        &mut self,
        result_statement: F,
    ) -> Arc<RwLock<ResultStatement>>
    where
        F: Fn(usize) -> Arc<RwLock<ResultStatement>>,
    {
        let _index = if let Some(_index) = self.result_statement_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.result_statement.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.result_statement.write().await.push(None);
            _index
        };

        let result_statement = result_statement(_index);

        let iter = self.result_statement.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *result_statement.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(result_statement) = found {
            log::debug!(target: "store", "found duplicate {result_statement:?}.");
            self.result_statement_free_list.lock().await.push(_index);
            result_statement.clone()
        } else {
            log::debug!(target: "store", "interring {result_statement:?}.");
            self.result_statement.write().await[_index] = Some(result_statement.clone());
            result_statement
        }
    }

    /// Exhume (get) [`ResultStatement`] from the store.
    ///
    pub async fn exhume_result_statement(
        &self,
        id: &usize,
    ) -> Option<Arc<RwLock<ResultStatement>>> {
        match self.result_statement.read().await.get(*id) {
            Some(result_statement) => result_statement.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ResultStatement`] from the store.
    ///
    pub async fn exorcise_result_statement(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<ResultStatement>>> {
        log::debug!(target: "store", "exorcising result_statement slot: {id}.");
        let result = self.result_statement.write().await[*id].take();
        self.result_statement_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ResultStatement>`.
    ///
    pub async fn iter_result_statement(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<ResultStatement>>> + '_ {
        let len = self.result_statement.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.result_statement.read().await[i].is_some() {
                self.result_statement.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`XReturn`] into the store.
    ///
    pub async fn inter_x_return<F>(&mut self, x_return: F) -> Arc<RwLock<XReturn>>
    where
        F: Fn(usize) -> Arc<RwLock<XReturn>>,
    {
        let _index = if let Some(_index) = self.x_return_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_return.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.x_return.write().await.push(None);
            _index
        };

        let x_return = x_return(_index);

        let iter = self.x_return.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *x_return.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(x_return) = found {
            log::debug!(target: "store", "found duplicate {x_return:?}.");
            self.x_return_free_list.lock().await.push(_index);
            x_return.clone()
        } else {
            log::debug!(target: "store", "interring {x_return:?}.");
            self.x_return.write().await[_index] = Some(x_return.clone());
            x_return
        }
    }

    /// Exhume (get) [`XReturn`] from the store.
    ///
    pub async fn exhume_x_return(&self, id: &usize) -> Option<Arc<RwLock<XReturn>>> {
        match self.x_return.read().await.get(*id) {
            Some(x_return) => x_return.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XReturn`] from the store.
    ///
    pub async fn exorcise_x_return(&mut self, id: &usize) -> Option<Arc<RwLock<XReturn>>> {
        log::debug!(target: "store", "exorcising x_return slot: {id}.");
        let result = self.x_return.write().await[*id].take();
        self.x_return_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XReturn>`.
    ///
    pub async fn iter_x_return(&self) -> impl stream::Stream<Item = Arc<RwLock<XReturn>>> + '_ {
        let len = self.x_return.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.x_return.read().await[i].is_some() {
                self.x_return.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`ZSome`] into the store.
    ///
    pub async fn inter_z_some<F>(&mut self, z_some: F) -> Arc<RwLock<ZSome>>
    where
        F: Fn(usize) -> Arc<RwLock<ZSome>>,
    {
        let _index = if let Some(_index) = self.z_some_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.z_some.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.z_some.write().await.push(None);
            _index
        };

        let z_some = z_some(_index);

        let iter = self.z_some.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *z_some.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(z_some) = found {
            log::debug!(target: "store", "found duplicate {z_some:?}.");
            self.z_some_free_list.lock().await.push(_index);
            z_some.clone()
        } else {
            log::debug!(target: "store", "interring {z_some:?}.");
            self.z_some.write().await[_index] = Some(z_some.clone());
            z_some
        }
    }

    /// Exhume (get) [`ZSome`] from the store.
    ///
    pub async fn exhume_z_some(&self, id: &usize) -> Option<Arc<RwLock<ZSome>>> {
        match self.z_some.read().await.get(*id) {
            Some(z_some) => z_some.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ZSome`] from the store.
    ///
    pub async fn exorcise_z_some(&mut self, id: &usize) -> Option<Arc<RwLock<ZSome>>> {
        log::debug!(target: "store", "exorcising z_some slot: {id}.");
        let result = self.z_some.write().await[*id].take();
        self.z_some_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ZSome>`.
    ///
    pub async fn iter_z_some(&self) -> impl stream::Stream<Item = Arc<RwLock<ZSome>>> + '_ {
        let len = self.z_some.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.z_some.read().await[i].is_some() {
                self.z_some.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`Span`] into the store.
    ///
    pub async fn inter_span<F>(&mut self, span: F) -> Arc<RwLock<Span>>
    where
        F: Fn(usize) -> Arc<RwLock<Span>>,
    {
        let _index = if let Some(_index) = self.span_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.span.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.span.write().await.push(None);
            _index
        };

        let span = span(_index);

        let iter = self.span.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *span.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(span) = found {
            log::debug!(target: "store", "found duplicate {span:?}.");
            self.span_free_list.lock().await.push(_index);
            span.clone()
        } else {
            log::debug!(target: "store", "interring {span:?}.");
            self.span.write().await[_index] = Some(span.clone());
            span
        }
    }

    /// Exhume (get) [`Span`] from the store.
    ///
    pub async fn exhume_span(&self, id: &usize) -> Option<Arc<RwLock<Span>>> {
        match self.span.read().await.get(*id) {
            Some(span) => span.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Span`] from the store.
    ///
    pub async fn exorcise_span(&mut self, id: &usize) -> Option<Arc<RwLock<Span>>> {
        log::debug!(target: "store", "exorcising span slot: {id}.");
        let result = self.span.write().await[*id].take();
        self.span_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Span>`.
    ///
    pub async fn iter_span(&self) -> impl stream::Stream<Item = Arc<RwLock<Span>>> + '_ {
        let len = self.span.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.span.read().await[i].is_some() {
                self.span.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`Statement`] into the store.
    ///
    pub async fn inter_statement<F>(&mut self, statement: F) -> Arc<RwLock<Statement>>
    where
        F: Fn(usize) -> Arc<RwLock<Statement>>,
    {
        let _index = if let Some(_index) = self.statement_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.statement.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.statement.write().await.push(None);
            _index
        };

        let statement = statement(_index);

        let iter = self.statement.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *statement.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(statement) = found {
            log::debug!(target: "store", "found duplicate {statement:?}.");
            self.statement_free_list.lock().await.push(_index);
            statement.clone()
        } else {
            log::debug!(target: "store", "interring {statement:?}.");
            self.statement.write().await[_index] = Some(statement.clone());
            statement
        }
    }

    /// Exhume (get) [`Statement`] from the store.
    ///
    pub async fn exhume_statement(&self, id: &usize) -> Option<Arc<RwLock<Statement>>> {
        match self.statement.read().await.get(*id) {
            Some(statement) => statement.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Statement`] from the store.
    ///
    pub async fn exorcise_statement(&mut self, id: &usize) -> Option<Arc<RwLock<Statement>>> {
        log::debug!(target: "store", "exorcising statement slot: {id}.");
        let result = self.statement.write().await[*id].take();
        self.statement_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Statement>`.
    ///
    pub async fn iter_statement(&self) -> impl stream::Stream<Item = Arc<RwLock<Statement>>> + '_ {
        let len = self.statement.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.statement.read().await[i].is_some() {
                self.statement.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`StaticMethodCall`] into the store.
    ///
    pub async fn inter_static_method_call<F>(
        &mut self,
        static_method_call: F,
    ) -> Arc<RwLock<StaticMethodCall>>
    where
        F: Fn(usize) -> Arc<RwLock<StaticMethodCall>>,
    {
        let _index = if let Some(_index) = self.static_method_call_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.static_method_call.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.static_method_call.write().await.push(None);
            _index
        };

        let static_method_call = static_method_call(_index);

        let iter = self.static_method_call.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *static_method_call.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(static_method_call) = found {
            log::debug!(target: "store", "found duplicate {static_method_call:?}.");
            self.static_method_call_free_list.lock().await.push(_index);
            static_method_call.clone()
        } else {
            log::debug!(target: "store", "interring {static_method_call:?}.");
            self.static_method_call.write().await[_index] = Some(static_method_call.clone());
            static_method_call
        }
    }

    /// Exhume (get) [`StaticMethodCall`] from the store.
    ///
    pub async fn exhume_static_method_call(
        &self,
        id: &usize,
    ) -> Option<Arc<RwLock<StaticMethodCall>>> {
        match self.static_method_call.read().await.get(*id) {
            Some(static_method_call) => static_method_call.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`StaticMethodCall`] from the store.
    ///
    pub async fn exorcise_static_method_call(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<StaticMethodCall>>> {
        log::debug!(target: "store", "exorcising static_method_call slot: {id}.");
        let result = self.static_method_call.write().await[*id].take();
        self.static_method_call_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StaticMethodCall>`.
    ///
    pub async fn iter_static_method_call(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<StaticMethodCall>>> + '_ {
        let len = self.static_method_call.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.static_method_call.read().await[i].is_some() {
                self.static_method_call.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`StringLiteral`] into the store.
    ///
    pub async fn inter_string_literal<F>(&mut self, string_literal: F) -> Arc<RwLock<StringLiteral>>
    where
        F: Fn(usize) -> Arc<RwLock<StringLiteral>>,
    {
        let _index = if let Some(_index) = self.string_literal_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.string_literal.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.string_literal.write().await.push(None);
            _index
        };

        let string_literal = string_literal(_index);

        let iter = self.string_literal.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *string_literal.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(string_literal) = found {
            log::debug!(target: "store", "found duplicate {string_literal:?}.");
            self.string_literal_free_list.lock().await.push(_index);
            string_literal.clone()
        } else {
            log::debug!(target: "store", "interring {string_literal:?}.");
            self.string_literal.write().await[_index] = Some(string_literal.clone());
            string_literal
        }
    }

    /// Exhume (get) [`StringLiteral`] from the store.
    ///
    pub async fn exhume_string_literal(&self, id: &usize) -> Option<Arc<RwLock<StringLiteral>>> {
        match self.string_literal.read().await.get(*id) {
            Some(string_literal) => string_literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`StringLiteral`] from the store.
    ///
    pub async fn exorcise_string_literal(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<StringLiteral>>> {
        log::debug!(target: "store", "exorcising string_literal slot: {id}.");
        let result = self.string_literal.write().await[*id].take();
        self.string_literal_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StringLiteral>`.
    ///
    pub async fn iter_string_literal(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<StringLiteral>>> + '_ {
        let len = self.string_literal.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.string_literal.read().await[i].is_some() {
                self.string_literal.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`WoogStruct`] into the store.
    ///
    pub async fn inter_woog_struct<F>(&mut self, woog_struct: F) -> Arc<RwLock<WoogStruct>>
    where
        F: Fn(usize) -> Arc<RwLock<WoogStruct>>,
    {
        let _index = if let Some(_index) = self.woog_struct_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.woog_struct.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.woog_struct.write().await.push(None);
            _index
        };

        let woog_struct = woog_struct(_index);

        let iter = self.woog_struct.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *woog_struct.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        let woog_struct = if let Some(woog_struct) = found {
            log::debug!(target: "store", "found duplicate {woog_struct:?}.");
            self.woog_struct_free_list.lock().await.push(_index);
            woog_struct.clone()
        } else {
            log::debug!(target: "store", "interring {woog_struct:?}.");
            self.woog_struct.write().await[_index] = Some(woog_struct.clone());
            woog_struct
        };
        self.woog_struct_id_by_name.write().await.insert(
            woog_struct.read().await.name.to_owned(),
            woog_struct.read().await.id,
        );
        woog_struct
    }

    /// Exhume (get) [`WoogStruct`] from the store.
    ///
    pub async fn exhume_woog_struct(&self, id: &usize) -> Option<Arc<RwLock<WoogStruct>>> {
        match self.woog_struct.read().await.get(*id) {
            Some(woog_struct) => woog_struct.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`WoogStruct`] from the store.
    ///
    pub async fn exorcise_woog_struct(&mut self, id: &usize) -> Option<Arc<RwLock<WoogStruct>>> {
        log::debug!(target: "store", "exorcising woog_struct slot: {id}.");
        let result = self.woog_struct.write().await[*id].take();
        self.woog_struct_free_list.lock().await.push(*id);
        result
    }

    /// Exorcise [`WoogStruct`] id from the store by name.
    ///
    pub async fn exhume_woog_struct_id_by_name(&self, name: &str) -> Option<usize> {
        self.woog_struct_id_by_name
            .read()
            .await
            .get(name)
            .map(|woog_struct| *woog_struct)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, WoogStruct>`.
    ///
    pub async fn iter_woog_struct(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<WoogStruct>>> + '_ {
        let len = self.woog_struct.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.woog_struct.read().await[i].is_some() {
                self.woog_struct.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`StructExpression`] into the store.
    ///
    pub async fn inter_struct_expression<F>(
        &mut self,
        struct_expression: F,
    ) -> Arc<RwLock<StructExpression>>
    where
        F: Fn(usize) -> Arc<RwLock<StructExpression>>,
    {
        let _index = if let Some(_index) = self.struct_expression_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.struct_expression.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.struct_expression.write().await.push(None);
            _index
        };

        let struct_expression = struct_expression(_index);

        let iter = self.struct_expression.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *struct_expression.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(struct_expression) = found {
            log::debug!(target: "store", "found duplicate {struct_expression:?}.");
            self.struct_expression_free_list.lock().await.push(_index);
            struct_expression.clone()
        } else {
            log::debug!(target: "store", "interring {struct_expression:?}.");
            self.struct_expression.write().await[_index] = Some(struct_expression.clone());
            struct_expression
        }
    }

    /// Exhume (get) [`StructExpression`] from the store.
    ///
    pub async fn exhume_struct_expression(
        &self,
        id: &usize,
    ) -> Option<Arc<RwLock<StructExpression>>> {
        match self.struct_expression.read().await.get(*id) {
            Some(struct_expression) => struct_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`StructExpression`] from the store.
    ///
    pub async fn exorcise_struct_expression(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<StructExpression>>> {
        log::debug!(target: "store", "exorcising struct_expression slot: {id}.");
        let result = self.struct_expression.write().await[*id].take();
        self.struct_expression_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructExpression>`.
    ///
    pub async fn iter_struct_expression(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<StructExpression>>> + '_ {
        let len = self.struct_expression.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.struct_expression.read().await[i].is_some() {
                self.struct_expression.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`StructField`] into the store.
    ///
    pub async fn inter_struct_field<F>(&mut self, struct_field: F) -> Arc<RwLock<StructField>>
    where
        F: Fn(usize) -> Arc<RwLock<StructField>>,
    {
        let _index = if let Some(_index) = self.struct_field_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.struct_field.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.struct_field.write().await.push(None);
            _index
        };

        let struct_field = struct_field(_index);

        let iter = self.struct_field.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *struct_field.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(struct_field) = found {
            log::debug!(target: "store", "found duplicate {struct_field:?}.");
            self.struct_field_free_list.lock().await.push(_index);
            struct_field.clone()
        } else {
            log::debug!(target: "store", "interring {struct_field:?}.");
            self.struct_field.write().await[_index] = Some(struct_field.clone());
            struct_field
        }
    }

    /// Exhume (get) [`StructField`] from the store.
    ///
    pub async fn exhume_struct_field(&self, id: &usize) -> Option<Arc<RwLock<StructField>>> {
        match self.struct_field.read().await.get(*id) {
            Some(struct_field) => struct_field.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`StructField`] from the store.
    ///
    pub async fn exorcise_struct_field(&mut self, id: &usize) -> Option<Arc<RwLock<StructField>>> {
        log::debug!(target: "store", "exorcising struct_field slot: {id}.");
        let result = self.struct_field.write().await[*id].take();
        self.struct_field_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructField>`.
    ///
    pub async fn iter_struct_field(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<StructField>>> + '_ {
        let len = self.struct_field.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.struct_field.read().await[i].is_some() {
                self.struct_field.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`TupleField`] into the store.
    ///
    pub async fn inter_tuple_field<F>(&mut self, tuple_field: F) -> Arc<RwLock<TupleField>>
    where
        F: Fn(usize) -> Arc<RwLock<TupleField>>,
    {
        let _index = if let Some(_index) = self.tuple_field_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.tuple_field.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.tuple_field.write().await.push(None);
            _index
        };

        let tuple_field = tuple_field(_index);

        let iter = self.tuple_field.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *tuple_field.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(tuple_field) = found {
            log::debug!(target: "store", "found duplicate {tuple_field:?}.");
            self.tuple_field_free_list.lock().await.push(_index);
            tuple_field.clone()
        } else {
            log::debug!(target: "store", "interring {tuple_field:?}.");
            self.tuple_field.write().await[_index] = Some(tuple_field.clone());
            tuple_field
        }
    }

    /// Exhume (get) [`TupleField`] from the store.
    ///
    pub async fn exhume_tuple_field(&self, id: &usize) -> Option<Arc<RwLock<TupleField>>> {
        match self.tuple_field.read().await.get(*id) {
            Some(tuple_field) => tuple_field.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`TupleField`] from the store.
    ///
    pub async fn exorcise_tuple_field(&mut self, id: &usize) -> Option<Arc<RwLock<TupleField>>> {
        log::debug!(target: "store", "exorcising tuple_field slot: {id}.");
        let result = self.tuple_field.write().await[*id].take();
        self.tuple_field_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, TupleField>`.
    ///
    pub async fn iter_tuple_field(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<TupleField>>> + '_ {
        let len = self.tuple_field.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.tuple_field.read().await[i].is_some() {
                self.tuple_field.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`TypeCast`] into the store.
    ///
    pub async fn inter_type_cast<F>(&mut self, type_cast: F) -> Arc<RwLock<TypeCast>>
    where
        F: Fn(usize) -> Arc<RwLock<TypeCast>>,
    {
        let _index = if let Some(_index) = self.type_cast_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.type_cast.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.type_cast.write().await.push(None);
            _index
        };

        let type_cast = type_cast(_index);

        let iter = self.type_cast.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *type_cast.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(type_cast) = found {
            log::debug!(target: "store", "found duplicate {type_cast:?}.");
            self.type_cast_free_list.lock().await.push(_index);
            type_cast.clone()
        } else {
            log::debug!(target: "store", "interring {type_cast:?}.");
            self.type_cast.write().await[_index] = Some(type_cast.clone());
            type_cast
        }
    }

    /// Exhume (get) [`TypeCast`] from the store.
    ///
    pub async fn exhume_type_cast(&self, id: &usize) -> Option<Arc<RwLock<TypeCast>>> {
        match self.type_cast.read().await.get(*id) {
            Some(type_cast) => type_cast.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`TypeCast`] from the store.
    ///
    pub async fn exorcise_type_cast(&mut self, id: &usize) -> Option<Arc<RwLock<TypeCast>>> {
        log::debug!(target: "store", "exorcising type_cast slot: {id}.");
        let result = self.type_cast.write().await[*id].take();
        self.type_cast_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, TypeCast>`.
    ///
    pub async fn iter_type_cast(&self) -> impl stream::Stream<Item = Arc<RwLock<TypeCast>>> + '_ {
        let len = self.type_cast.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.type_cast.read().await[i].is_some() {
                self.type_cast.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`Unary`] into the store.
    ///
    pub async fn inter_unary<F>(&mut self, unary: F) -> Arc<RwLock<Unary>>
    where
        F: Fn(usize) -> Arc<RwLock<Unary>>,
    {
        let _index = if let Some(_index) = self.unary_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.unary.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.unary.write().await.push(None);
            _index
        };

        let unary = unary(_index);

        let iter = self.unary.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *unary.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(unary) = found {
            log::debug!(target: "store", "found duplicate {unary:?}.");
            self.unary_free_list.lock().await.push(_index);
            unary.clone()
        } else {
            log::debug!(target: "store", "interring {unary:?}.");
            self.unary.write().await[_index] = Some(unary.clone());
            unary
        }
    }

    /// Exhume (get) [`Unary`] from the store.
    ///
    pub async fn exhume_unary(&self, id: &usize) -> Option<Arc<RwLock<Unary>>> {
        match self.unary.read().await.get(*id) {
            Some(unary) => unary.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Unary`] from the store.
    ///
    pub async fn exorcise_unary(&mut self, id: &usize) -> Option<Arc<RwLock<Unary>>> {
        log::debug!(target: "store", "exorcising unary slot: {id}.");
        let result = self.unary.write().await[*id].take();
        self.unary_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Unary>`.
    ///
    pub async fn iter_unary(&self) -> impl stream::Stream<Item = Arc<RwLock<Unary>>> + '_ {
        let len = self.unary.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.unary.read().await[i].is_some() {
                self.unary.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`XValue`] into the store.
    ///
    pub async fn inter_x_value<F>(&mut self, x_value: F) -> Arc<RwLock<XValue>>
    where
        F: Fn(usize) -> Arc<RwLock<XValue>>,
    {
        let _index = if let Some(_index) = self.x_value_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_value.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.x_value.write().await.push(None);
            _index
        };

        let x_value = x_value(_index);

        let iter = self.x_value.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *x_value.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(x_value) = found {
            log::debug!(target: "store", "found duplicate {x_value:?}.");
            self.x_value_free_list.lock().await.push(_index);
            x_value.clone()
        } else {
            log::debug!(target: "store", "interring {x_value:?}.");
            self.x_value.write().await[_index] = Some(x_value.clone());
            x_value
        }
    }

    /// Exhume (get) [`XValue`] from the store.
    ///
    pub async fn exhume_x_value(&self, id: &usize) -> Option<Arc<RwLock<XValue>>> {
        match self.x_value.read().await.get(*id) {
            Some(x_value) => x_value.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XValue`] from the store.
    ///
    pub async fn exorcise_x_value(&mut self, id: &usize) -> Option<Arc<RwLock<XValue>>> {
        log::debug!(target: "store", "exorcising x_value slot: {id}.");
        let result = self.x_value.write().await[*id].take();
        self.x_value_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XValue>`.
    ///
    pub async fn iter_x_value(&self) -> impl stream::Stream<Item = Arc<RwLock<XValue>>> + '_ {
        let len = self.x_value.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.x_value.read().await[i].is_some() {
                self.x_value.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`ValueType`] into the store.
    ///
    pub async fn inter_value_type<F>(&mut self, value_type: F) -> Arc<RwLock<ValueType>>
    where
        F: Fn(usize) -> Arc<RwLock<ValueType>>,
    {
        let _index = if let Some(_index) = self.value_type_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.value_type.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.value_type.write().await.push(None);
            _index
        };

        let value_type = value_type(_index);

        let iter = self.value_type.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *value_type.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(value_type) = found {
            log::debug!(target: "store", "found duplicate {value_type:?}.");
            self.value_type_free_list.lock().await.push(_index);
            value_type.clone()
        } else {
            log::debug!(target: "store", "interring {value_type:?}.");
            self.value_type.write().await[_index] = Some(value_type.clone());
            value_type
        }
    }

    /// Exhume (get) [`ValueType`] from the store.
    ///
    pub async fn exhume_value_type(&self, id: &usize) -> Option<Arc<RwLock<ValueType>>> {
        match self.value_type.read().await.get(*id) {
            Some(value_type) => value_type.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ValueType`] from the store.
    ///
    pub async fn exorcise_value_type(&mut self, id: &usize) -> Option<Arc<RwLock<ValueType>>> {
        log::debug!(target: "store", "exorcising value_type slot: {id}.");
        let result = self.value_type.write().await[*id].take();
        self.value_type_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ValueType>`.
    ///
    pub async fn iter_value_type(&self) -> impl stream::Stream<Item = Arc<RwLock<ValueType>>> + '_ {
        let len = self.value_type.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.value_type.read().await[i].is_some() {
                self.value_type.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`Variable`] into the store.
    ///
    pub async fn inter_variable<F>(&mut self, variable: F) -> Arc<RwLock<Variable>>
    where
        F: Fn(usize) -> Arc<RwLock<Variable>>,
    {
        let _index = if let Some(_index) = self.variable_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.variable.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.variable.write().await.push(None);
            _index
        };

        let variable = variable(_index);

        let iter = self.variable.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *variable.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(variable) = found {
            log::debug!(target: "store", "found duplicate {variable:?}.");
            self.variable_free_list.lock().await.push(_index);
            variable.clone()
        } else {
            log::debug!(target: "store", "interring {variable:?}.");
            self.variable.write().await[_index] = Some(variable.clone());
            variable
        }
    }

    /// Exhume (get) [`Variable`] from the store.
    ///
    pub async fn exhume_variable(&self, id: &usize) -> Option<Arc<RwLock<Variable>>> {
        match self.variable.read().await.get(*id) {
            Some(variable) => variable.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Variable`] from the store.
    ///
    pub async fn exorcise_variable(&mut self, id: &usize) -> Option<Arc<RwLock<Variable>>> {
        log::debug!(target: "store", "exorcising variable slot: {id}.");
        let result = self.variable.write().await[*id].take();
        self.variable_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Variable>`.
    ///
    pub async fn iter_variable(&self) -> impl stream::Stream<Item = Arc<RwLock<Variable>>> + '_ {
        let len = self.variable.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.variable.read().await[i].is_some() {
                self.variable.read().await[i].clone()
            } else {
                None
            }
        })
    }

    /// Inter (insert) [`VariableExpression`] into the store.
    ///
    pub async fn inter_variable_expression<F>(
        &mut self,
        variable_expression: F,
    ) -> Arc<RwLock<VariableExpression>>
    where
        F: Fn(usize) -> Arc<RwLock<VariableExpression>>,
    {
        let _index = if let Some(_index) = self.variable_expression_free_list.lock().await.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.variable_expression.read().await.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.variable_expression.write().await.push(None);
            _index
        };

        let variable_expression = variable_expression(_index);

        let iter = self.variable_expression.read().await;
        let iter = iter.iter();
        let iter = stream::iter(iter);
        let found = iter
            .filter_map(|stored| {
                Box::pin({
                    let stored = stored.clone();
                    async {
                        if let Some(stored) = stored {
                            if *stored.read().await == *variable_expression.read().await {
                                Some(stored)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .next()
            .await;

        if let Some(variable_expression) = found {
            log::debug!(target: "store", "found duplicate {variable_expression:?}.");
            self.variable_expression_free_list.lock().await.push(_index);
            variable_expression.clone()
        } else {
            log::debug!(target: "store", "interring {variable_expression:?}.");
            self.variable_expression.write().await[_index] = Some(variable_expression.clone());
            variable_expression
        }
    }

    /// Exhume (get) [`VariableExpression`] from the store.
    ///
    pub async fn exhume_variable_expression(
        &self,
        id: &usize,
    ) -> Option<Arc<RwLock<VariableExpression>>> {
        match self.variable_expression.read().await.get(*id) {
            Some(variable_expression) => variable_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`VariableExpression`] from the store.
    ///
    pub async fn exorcise_variable_expression(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<VariableExpression>>> {
        log::debug!(target: "store", "exorcising variable_expression slot: {id}.");
        let result = self.variable_expression.write().await[*id].take();
        self.variable_expression_free_list.lock().await.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, VariableExpression>`.
    ///
    pub async fn iter_variable_expression(
        &self,
    ) -> impl stream::Stream<Item = Arc<RwLock<VariableExpression>>> + '_ {
        let len = self.variable_expression.read().await.len();
        stream::iter((0..len)).filter_map(move |i| async move {
            if self.variable_expression.read().await[i].is_some() {
                self.variable_expression.read().await[i].clone()
            } else {
                None
            }
        })
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
            for argument in &*self.argument.read().await {
                if let Some(argument) = argument {
                    let path = path.join(format!("{}.json", argument.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(argument.read().await).clone())?;
                }
            }
        }

        // Persist Binary.
        {
            let path = path.join("binary");
            fs::create_dir_all(&path)?;
            for binary in &*self.binary.read().await {
                if let Some(binary) = binary {
                    let path = path.join(format!("{}.json", binary.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(binary.read().await).clone())?;
                }
            }
        }

        // Persist Block.
        {
            let path = path.join("block");
            fs::create_dir_all(&path)?;
            for block in &*self.block.read().await {
                if let Some(block) = block {
                    let path = path.join(format!("{}.json", block.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(block.read().await).clone())?;
                }
            }
        }

        // Persist Body.
        {
            let path = path.join("body");
            fs::create_dir_all(&path)?;
            for body in &*self.body.read().await {
                if let Some(body) = body {
                    let path = path.join(format!("{}.json", body.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(body.read().await).clone())?;
                }
            }
        }

        // Persist Boolean Literal.
        {
            let path = path.join("boolean_literal");
            fs::create_dir_all(&path)?;
            for boolean_literal in &*self.boolean_literal.read().await {
                if let Some(boolean_literal) = boolean_literal {
                    let path = path.join(format!("{}.json", boolean_literal.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(boolean_literal.read().await).clone(),
                    )?;
                }
            }
        }

        // Persist Boolean Operator.
        {
            let path = path.join("boolean_operator");
            fs::create_dir_all(&path)?;
            for boolean_operator in &*self.boolean_operator.read().await {
                if let Some(boolean_operator) = boolean_operator {
                    let path = path.join(format!("{}.json", boolean_operator.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(boolean_operator.read().await).clone(),
                    )?;
                }
            }
        }

        // Persist Call.
        {
            let path = path.join("call");
            fs::create_dir_all(&path)?;
            for call in &*self.call.read().await {
                if let Some(call) = call {
                    let path = path.join(format!("{}.json", call.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(call.read().await).clone())?;
                }
            }
        }

        // Persist Comparison.
        {
            let path = path.join("comparison");
            fs::create_dir_all(&path)?;
            for comparison in &*self.comparison.read().await {
                if let Some(comparison) = comparison {
                    let path = path.join(format!("{}.json", comparison.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(comparison.read().await).clone())?;
                }
            }
        }

        // Persist Dwarf Source File.
        {
            let path = path.join("dwarf_source_file");
            fs::create_dir_all(&path)?;
            for dwarf_source_file in &*self.dwarf_source_file.read().await {
                if let Some(dwarf_source_file) = dwarf_source_file {
                    let path = path.join(format!("{}.json", dwarf_source_file.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(dwarf_source_file.read().await).clone(),
                    )?;
                }
            }
        }

        // Persist Enum Field.
        {
            let path = path.join("enum_field");
            fs::create_dir_all(&path)?;
            for enum_field in &*self.enum_field.read().await {
                if let Some(enum_field) = enum_field {
                    let path = path.join(format!("{}.json", enum_field.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(enum_field.read().await).clone())?;
                }
            }
        }

        // Persist Enumeration.
        {
            let path = path.join("enumeration");
            fs::create_dir_all(&path)?;
            for enumeration in &*self.enumeration.read().await {
                if let Some(enumeration) = enumeration {
                    let path = path.join(format!("{}.json", enumeration.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(enumeration.read().await).clone())?;
                }
            }
        }

        // Persist Error.
        {
            let path = path.join("x_error");
            fs::create_dir_all(&path)?;
            for x_error in &*self.x_error.read().await {
                if let Some(x_error) = x_error {
                    let path = path.join(format!("{}.json", x_error.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(x_error.read().await).clone())?;
                }
            }
        }

        // Persist Error Expression.
        {
            let path = path.join("error_expression");
            fs::create_dir_all(&path)?;
            for error_expression in &*self.error_expression.read().await {
                if let Some(error_expression) = error_expression {
                    let path = path.join(format!("{}.json", error_expression.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(error_expression.read().await).clone(),
                    )?;
                }
            }
        }

        // Persist Expression.
        {
            let path = path.join("expression");
            fs::create_dir_all(&path)?;
            for expression in &*self.expression.read().await {
                if let Some(expression) = expression {
                    let path = path.join(format!("{}.json", expression.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(expression.read().await).clone())?;
                }
            }
        }

        // Persist Expression Statement.
        {
            let path = path.join("expression_statement");
            fs::create_dir_all(&path)?;
            for expression_statement in &*self.expression_statement.read().await {
                if let Some(expression_statement) = expression_statement {
                    let path = path.join(format!("{}.json", expression_statement.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(expression_statement.read().await).clone(),
                    )?;
                }
            }
        }

        // Persist External Implementation.
        {
            let path = path.join("external_implementation");
            fs::create_dir_all(&path)?;
            for external_implementation in &*self.external_implementation.read().await {
                if let Some(external_implementation) = external_implementation {
                    let path =
                        path.join(format!("{}.json", external_implementation.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(external_implementation.read().await).clone(),
                    )?;
                }
            }
        }

        // Persist Field.
        {
            let path = path.join("field");
            fs::create_dir_all(&path)?;
            for field in &*self.field.read().await {
                if let Some(field) = field {
                    let path = path.join(format!("{}.json", field.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(field.read().await).clone())?;
                }
            }
        }

        // Persist Field Access.
        {
            let path = path.join("field_access");
            fs::create_dir_all(&path)?;
            for field_access in &*self.field_access.read().await {
                if let Some(field_access) = field_access {
                    let path = path.join(format!("{}.json", field_access.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(field_access.read().await).clone(),
                    )?;
                }
            }
        }

        // Persist Field Access Target.
        {
            let path = path.join("field_access_target");
            fs::create_dir_all(&path)?;
            for field_access_target in &*self.field_access_target.read().await {
                if let Some(field_access_target) = field_access_target {
                    let path = path.join(format!("{}.json", field_access_target.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(field_access_target.read().await).clone(),
                    )?;
                }
            }
        }

        // Persist Field Expression.
        {
            let path = path.join("field_expression");
            fs::create_dir_all(&path)?;
            for field_expression in &*self.field_expression.read().await {
                if let Some(field_expression) = field_expression {
                    let path = path.join(format!("{}.json", field_expression.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(field_expression.read().await).clone(),
                    )?;
                }
            }
        }

        // Persist Float Literal.
        {
            let path = path.join("float_literal");
            fs::create_dir_all(&path)?;
            for float_literal in &*self.float_literal.read().await {
                if let Some(float_literal) = float_literal {
                    let path = path.join(format!("{}.json", float_literal.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(float_literal.read().await).clone(),
                    )?;
                }
            }
        }

        // Persist For Loop.
        {
            let path = path.join("for_loop");
            fs::create_dir_all(&path)?;
            for for_loop in &*self.for_loop.read().await {
                if let Some(for_loop) = for_loop {
                    let path = path.join(format!("{}.json", for_loop.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(for_loop.read().await).clone())?;
                }
            }
        }

        // Persist Function.
        {
            let path = path.join("function");
            fs::create_dir_all(&path)?;
            for function in &*self.function.read().await {
                if let Some(function) = function {
                    let path = path.join(format!("{}.json", function.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(function.read().await).clone())?;
                }
            }
        }

        // Persist Generic.
        {
            let path = path.join("generic");
            fs::create_dir_all(&path)?;
            for generic in &*self.generic.read().await {
                if let Some(generic) = generic {
                    let path = path.join(format!("{}.json", generic.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(generic.read().await).clone())?;
                }
            }
        }

        // Persist Grouped.
        {
            let path = path.join("grouped");
            fs::create_dir_all(&path)?;
            for grouped in &*self.grouped.read().await {
                if let Some(grouped) = grouped {
                    let path = path.join(format!("{}.json", grouped.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(grouped.read().await).clone())?;
                }
            }
        }

        // Persist If.
        {
            let path = path.join("x_if");
            fs::create_dir_all(&path)?;
            for x_if in &*self.x_if.read().await {
                if let Some(x_if) = x_if {
                    let path = path.join(format!("{}.json", x_if.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(x_if.read().await).clone())?;
                }
            }
        }

        // Persist Implementation Block.
        {
            let path = path.join("implementation_block");
            fs::create_dir_all(&path)?;
            for implementation_block in &*self.implementation_block.read().await {
                if let Some(implementation_block) = implementation_block {
                    let path = path.join(format!("{}.json", implementation_block.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(implementation_block.read().await).clone(),
                    )?;
                }
            }
        }

        // Persist Import.
        {
            let path = path.join("import");
            fs::create_dir_all(&path)?;
            for import in &*self.import.read().await {
                if let Some(import) = import {
                    let path = path.join(format!("{}.json", import.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(import.read().await).clone())?;
                }
            }
        }

        // Persist Index.
        {
            let path = path.join("index");
            fs::create_dir_all(&path)?;
            for index in &*self.index.read().await {
                if let Some(index) = index {
                    let path = path.join(format!("{}.json", index.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(index.read().await).clone())?;
                }
            }
        }

        // Persist Integer Literal.
        {
            let path = path.join("integer_literal");
            fs::create_dir_all(&path)?;
            for integer_literal in &*self.integer_literal.read().await {
                if let Some(integer_literal) = integer_literal {
                    let path = path.join(format!("{}.json", integer_literal.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(integer_literal.read().await).clone(),
                    )?;
                }
            }
        }

        // Persist Item.
        {
            let path = path.join("item");
            fs::create_dir_all(&path)?;
            for item in &*self.item.read().await {
                if let Some(item) = item {
                    let path = path.join(format!("{}.json", item.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(item.read().await).clone())?;
                }
            }
        }

        // Persist Lambda.
        {
            let path = path.join("lambda");
            fs::create_dir_all(&path)?;
            for lambda in &*self.lambda.read().await {
                if let Some(lambda) = lambda {
                    let path = path.join(format!("{}.json", lambda.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(lambda.read().await).clone())?;
                }
            }
        }

        // Persist Lambda Parameter.
        {
            let path = path.join("lambda_parameter");
            fs::create_dir_all(&path)?;
            for lambda_parameter in &*self.lambda_parameter.read().await {
                if let Some(lambda_parameter) = lambda_parameter {
                    let path = path.join(format!("{}.json", lambda_parameter.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(lambda_parameter.read().await).clone(),
                    )?;
                }
            }
        }

        // Persist Let Statement.
        {
            let path = path.join("let_statement");
            fs::create_dir_all(&path)?;
            for let_statement in &*self.let_statement.read().await {
                if let Some(let_statement) = let_statement {
                    let path = path.join(format!("{}.json", let_statement.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(let_statement.read().await).clone(),
                    )?;
                }
            }
        }

        // Persist List.
        {
            let path = path.join("list");
            fs::create_dir_all(&path)?;
            for list in &*self.list.read().await {
                if let Some(list) = list {
                    let path = path.join(format!("{}.json", list.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(list.read().await).clone())?;
                }
            }
        }

        // Persist List Element.
        {
            let path = path.join("list_element");
            fs::create_dir_all(&path)?;
            for list_element in &*self.list_element.read().await {
                if let Some(list_element) = list_element {
                    let path = path.join(format!("{}.json", list_element.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(list_element.read().await).clone(),
                    )?;
                }
            }
        }

        // Persist List Expression.
        {
            let path = path.join("list_expression");
            fs::create_dir_all(&path)?;
            for list_expression in &*self.list_expression.read().await {
                if let Some(list_expression) = list_expression {
                    let path = path.join(format!("{}.json", list_expression.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(list_expression.read().await).clone(),
                    )?;
                }
            }
        }

        // Persist Literal.
        {
            let path = path.join("literal");
            fs::create_dir_all(&path)?;
            for literal in &*self.literal.read().await {
                if let Some(literal) = literal {
                    let path = path.join(format!("{}.json", literal.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(literal.read().await).clone())?;
                }
            }
        }

        // Persist Local Variable.
        {
            let path = path.join("local_variable");
            fs::create_dir_all(&path)?;
            for local_variable in &*self.local_variable.read().await {
                if let Some(local_variable) = local_variable {
                    let path = path.join(format!("{}.json", local_variable.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(local_variable.read().await).clone(),
                    )?;
                }
            }
        }

        // Persist Macro.
        {
            let path = path.join("x_macro");
            fs::create_dir_all(&path)?;
            for x_macro in &*self.x_macro.read().await {
                if let Some(x_macro) = x_macro {
                    let path = path.join(format!("{}.json", x_macro.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(x_macro.read().await).clone())?;
                }
            }
        }

        // Persist Match.
        {
            let path = path.join("x_match");
            fs::create_dir_all(&path)?;
            for x_match in &*self.x_match.read().await {
                if let Some(x_match) = x_match {
                    let path = path.join(format!("{}.json", x_match.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(x_match.read().await).clone())?;
                }
            }
        }

        // Persist Method Call.
        {
            let path = path.join("method_call");
            fs::create_dir_all(&path)?;
            for method_call in &*self.method_call.read().await {
                if let Some(method_call) = method_call {
                    let path = path.join(format!("{}.json", method_call.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(method_call.read().await).clone())?;
                }
            }
        }

        // Persist Object Store.
        {
            let path = path.join("z_object_store");
            fs::create_dir_all(&path)?;
            for z_object_store in &*self.z_object_store.read().await {
                if let Some(z_object_store) = z_object_store {
                    let path = path.join(format!("{}.json", z_object_store.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(z_object_store.read().await).clone(),
                    )?;
                }
            }
        }

        // Persist Object Wrapper.
        {
            let path = path.join("object_wrapper");
            fs::create_dir_all(&path)?;
            for object_wrapper in &*self.object_wrapper.read().await {
                if let Some(object_wrapper) = object_wrapper {
                    let path = path.join(format!("{}.json", object_wrapper.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(object_wrapper.read().await).clone(),
                    )?;
                }
            }
        }

        // Persist Operator.
        {
            let path = path.join("operator");
            fs::create_dir_all(&path)?;
            for operator in &*self.operator.read().await {
                if let Some(operator) = operator {
                    let path = path.join(format!("{}.json", operator.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(operator.read().await).clone())?;
                }
            }
        }

        // Persist Option.
        {
            let path = path.join("woog_option");
            fs::create_dir_all(&path)?;
            for woog_option in &*self.woog_option.read().await {
                if let Some(woog_option) = woog_option {
                    let path = path.join(format!("{}.json", woog_option.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(woog_option.read().await).clone())?;
                }
            }
        }

        // Persist Parameter.
        {
            let path = path.join("parameter");
            fs::create_dir_all(&path)?;
            for parameter in &*self.parameter.read().await {
                if let Some(parameter) = parameter {
                    let path = path.join(format!("{}.json", parameter.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(parameter.read().await).clone())?;
                }
            }
        }

        // Persist Pattern.
        {
            let path = path.join("pattern");
            fs::create_dir_all(&path)?;
            for pattern in &*self.pattern.read().await {
                if let Some(pattern) = pattern {
                    let path = path.join(format!("{}.json", pattern.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(pattern.read().await).clone())?;
                }
            }
        }

        // Persist Plain.
        {
            let path = path.join("plain");
            fs::create_dir_all(&path)?;
            for plain in &*self.plain.read().await {
                if let Some(plain) = plain {
                    let path = path.join(format!("{}.json", plain.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(plain.read().await).clone())?;
                }
            }
        }

        // Persist Print.
        {
            let path = path.join("x_print");
            fs::create_dir_all(&path)?;
            for x_print in &*self.x_print.read().await {
                if let Some(x_print) = x_print {
                    let path = path.join(format!("{}.json", x_print.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(x_print.read().await).clone())?;
                }
            }
        }

        // Persist Range Expression.
        {
            let path = path.join("range_expression");
            fs::create_dir_all(&path)?;
            for range_expression in &*self.range_expression.read().await {
                if let Some(range_expression) = range_expression {
                    let path = path.join(format!("{}.json", range_expression.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(range_expression.read().await).clone(),
                    )?;
                }
            }
        }

        // Persist Reference.
        {
            let path = path.join("reference");
            fs::create_dir_all(&path)?;
            for reference in &*self.reference.read().await {
                if let Some(reference) = reference {
                    let path = path.join(format!("{}.json", reference.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(reference.read().await).clone())?;
                }
            }
        }

        // Persist Result Statement.
        {
            let path = path.join("result_statement");
            fs::create_dir_all(&path)?;
            for result_statement in &*self.result_statement.read().await {
                if let Some(result_statement) = result_statement {
                    let path = path.join(format!("{}.json", result_statement.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(result_statement.read().await).clone(),
                    )?;
                }
            }
        }

        // Persist Return.
        {
            let path = path.join("x_return");
            fs::create_dir_all(&path)?;
            for x_return in &*self.x_return.read().await {
                if let Some(x_return) = x_return {
                    let path = path.join(format!("{}.json", x_return.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(x_return.read().await).clone())?;
                }
            }
        }

        // Persist Some.
        {
            let path = path.join("z_some");
            fs::create_dir_all(&path)?;
            for z_some in &*self.z_some.read().await {
                if let Some(z_some) = z_some {
                    let path = path.join(format!("{}.json", z_some.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(z_some.read().await).clone())?;
                }
            }
        }

        // Persist Span.
        {
            let path = path.join("span");
            fs::create_dir_all(&path)?;
            for span in &*self.span.read().await {
                if let Some(span) = span {
                    let path = path.join(format!("{}.json", span.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(span.read().await).clone())?;
                }
            }
        }

        // Persist Statement.
        {
            let path = path.join("statement");
            fs::create_dir_all(&path)?;
            for statement in &*self.statement.read().await {
                if let Some(statement) = statement {
                    let path = path.join(format!("{}.json", statement.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(statement.read().await).clone())?;
                }
            }
        }

        // Persist Static Method Call.
        {
            let path = path.join("static_method_call");
            fs::create_dir_all(&path)?;
            for static_method_call in &*self.static_method_call.read().await {
                if let Some(static_method_call) = static_method_call {
                    let path = path.join(format!("{}.json", static_method_call.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(static_method_call.read().await).clone(),
                    )?;
                }
            }
        }

        // Persist String Literal.
        {
            let path = path.join("string_literal");
            fs::create_dir_all(&path)?;
            for string_literal in &*self.string_literal.read().await {
                if let Some(string_literal) = string_literal {
                    let path = path.join(format!("{}.json", string_literal.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(string_literal.read().await).clone(),
                    )?;
                }
            }
        }

        // Persist Struct.
        {
            let path = path.join("woog_struct");
            fs::create_dir_all(&path)?;
            for woog_struct in &*self.woog_struct.read().await {
                if let Some(woog_struct) = woog_struct {
                    let path = path.join(format!("{}.json", woog_struct.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(woog_struct.read().await).clone())?;
                }
            }
        }

        // Persist Struct Expression.
        {
            let path = path.join("struct_expression");
            fs::create_dir_all(&path)?;
            for struct_expression in &*self.struct_expression.read().await {
                if let Some(struct_expression) = struct_expression {
                    let path = path.join(format!("{}.json", struct_expression.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(struct_expression.read().await).clone(),
                    )?;
                }
            }
        }

        // Persist Struct Field.
        {
            let path = path.join("struct_field");
            fs::create_dir_all(&path)?;
            for struct_field in &*self.struct_field.read().await {
                if let Some(struct_field) = struct_field {
                    let path = path.join(format!("{}.json", struct_field.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(struct_field.read().await).clone(),
                    )?;
                }
            }
        }

        // Persist Tuple Field.
        {
            let path = path.join("tuple_field");
            fs::create_dir_all(&path)?;
            for tuple_field in &*self.tuple_field.read().await {
                if let Some(tuple_field) = tuple_field {
                    let path = path.join(format!("{}.json", tuple_field.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(tuple_field.read().await).clone())?;
                }
            }
        }

        // Persist Type Cast.
        {
            let path = path.join("type_cast");
            fs::create_dir_all(&path)?;
            for type_cast in &*self.type_cast.read().await {
                if let Some(type_cast) = type_cast {
                    let path = path.join(format!("{}.json", type_cast.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(type_cast.read().await).clone())?;
                }
            }
        }

        // Persist Unary.
        {
            let path = path.join("unary");
            fs::create_dir_all(&path)?;
            for unary in &*self.unary.read().await {
                if let Some(unary) = unary {
                    let path = path.join(format!("{}.json", unary.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(unary.read().await).clone())?;
                }
            }
        }

        // Persist Value.
        {
            let path = path.join("x_value");
            fs::create_dir_all(&path)?;
            for x_value in &*self.x_value.read().await {
                if let Some(x_value) = x_value {
                    let path = path.join(format!("{}.json", x_value.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(x_value.read().await).clone())?;
                }
            }
        }

        // Persist Value Type.
        {
            let path = path.join("value_type");
            fs::create_dir_all(&path)?;
            for value_type in &*self.value_type.read().await {
                if let Some(value_type) = value_type {
                    let path = path.join(format!("{}.json", value_type.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(value_type.read().await).clone())?;
                }
            }
        }

        // Persist Variable.
        {
            let path = path.join("variable");
            fs::create_dir_all(&path)?;
            for variable in &*self.variable.read().await {
                if let Some(variable) = variable {
                    let path = path.join(format!("{}.json", variable.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &(variable.read().await).clone())?;
                }
            }
        }

        // Persist Variable Expression.
        {
            let path = path.join("variable_expression");
            fs::create_dir_all(&path)?;
            for variable_expression in &*self.variable_expression.read().await {
                if let Some(variable_expression) = variable_expression {
                    let path = path.join(format!("{}.json", variable_expression.read().await.id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(
                        &mut writer,
                        &(variable_expression.read().await).clone(),
                    )?;
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
        let mut store = store.await;

        // Load Argument.
        {
            let path = path.join("argument");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let argument: Arc<RwLock<Argument>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .argument
                    .write()
                    .await
                    .insert(argument.read().await.id, Some(argument.clone()));
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
                let binary: Arc<RwLock<Binary>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .binary
                    .write()
                    .await
                    .insert(binary.read().await.id, Some(binary.clone()));
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
                let block: Arc<RwLock<Block>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .block
                    .write()
                    .await
                    .insert(block.read().await.id, Some(block.clone()));
            }
        }

        // Load Body.
        {
            let path = path.join("body");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let body: Arc<RwLock<Body>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .body
                    .write()
                    .await
                    .insert(body.read().await.id, Some(body.clone()));
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
                let boolean_literal: Arc<RwLock<BooleanLiteral>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store.boolean_literal.write().await.insert(
                    boolean_literal.read().await.id,
                    Some(boolean_literal.clone()),
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
                let boolean_operator: Arc<RwLock<BooleanOperator>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store.boolean_operator.write().await.insert(
                    boolean_operator.read().await.id,
                    Some(boolean_operator.clone()),
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
                let call: Arc<RwLock<Call>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .call
                    .write()
                    .await
                    .insert(call.read().await.id, Some(call.clone()));
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
                let comparison: Arc<RwLock<Comparison>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .comparison
                    .write()
                    .await
                    .insert(comparison.read().await.id, Some(comparison.clone()));
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
                let dwarf_source_file: Arc<RwLock<DwarfSourceFile>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store.dwarf_source_file.write().await.insert(
                    dwarf_source_file.read().await.id,
                    Some(dwarf_source_file.clone()),
                );
            }
        }

        // Load Enum Field.
        {
            let path = path.join("enum_field");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let enum_field: Arc<RwLock<EnumField>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .enum_field
                    .write()
                    .await
                    .insert(enum_field.read().await.id, Some(enum_field.clone()));
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
                let enumeration: Arc<RwLock<Enumeration>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store.enumeration_id_by_name.write().await.insert(
                    enumeration.read().await.name.to_owned(),
                    enumeration.read().await.id,
                );
                store
                    .enumeration
                    .write()
                    .await
                    .insert(enumeration.read().await.id, Some(enumeration.clone()));
            }
        }

        // Load Error.
        {
            let path = path.join("x_error");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let x_error: Arc<RwLock<XError>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .x_error
                    .write()
                    .await
                    .insert(x_error.read().await.id, Some(x_error.clone()));
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
                let error_expression: Arc<RwLock<ErrorExpression>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store.error_expression.write().await.insert(
                    error_expression.read().await.id,
                    Some(error_expression.clone()),
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
                let expression: Arc<RwLock<Expression>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .expression
                    .write()
                    .await
                    .insert(expression.read().await.id, Some(expression.clone()));
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
                let expression_statement: Arc<RwLock<ExpressionStatement>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store.expression_statement.write().await.insert(
                    expression_statement.read().await.id,
                    Some(expression_statement.clone()),
                );
            }
        }

        // Load External Implementation.
        {
            let path = path.join("external_implementation");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let external_implementation: Arc<RwLock<ExternalImplementation>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store.external_implementation.write().await.insert(
                    external_implementation.read().await.id,
                    Some(external_implementation.clone()),
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
                let field: Arc<RwLock<Field>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .field_id_by_name
                    .write()
                    .await
                    .insert(field.read().await.name.to_owned(), field.read().await.id);
                store
                    .field
                    .write()
                    .await
                    .insert(field.read().await.id, Some(field.clone()));
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
                let field_access: Arc<RwLock<FieldAccess>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .field_access
                    .write()
                    .await
                    .insert(field_access.read().await.id, Some(field_access.clone()));
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
                let field_access_target: Arc<RwLock<FieldAccessTarget>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store.field_access_target.write().await.insert(
                    field_access_target.read().await.id,
                    Some(field_access_target.clone()),
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
                let field_expression: Arc<RwLock<FieldExpression>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store.field_expression.write().await.insert(
                    field_expression.read().await.id,
                    Some(field_expression.clone()),
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
                let float_literal: Arc<RwLock<FloatLiteral>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .float_literal
                    .write()
                    .await
                    .insert(float_literal.read().await.id, Some(float_literal.clone()));
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
                let for_loop: Arc<RwLock<ForLoop>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .for_loop
                    .write()
                    .await
                    .insert(for_loop.read().await.id, Some(for_loop.clone()));
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
                let function: Arc<RwLock<Function>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store.function_id_by_name.write().await.insert(
                    function.read().await.name.to_owned(),
                    function.read().await.id,
                );
                store
                    .function
                    .write()
                    .await
                    .insert(function.read().await.id, Some(function.clone()));
            }
        }

        // Load Generic.
        {
            let path = path.join("generic");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let generic: Arc<RwLock<Generic>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .generic
                    .write()
                    .await
                    .insert(generic.read().await.id, Some(generic.clone()));
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
                let grouped: Arc<RwLock<Grouped>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .grouped
                    .write()
                    .await
                    .insert(grouped.read().await.id, Some(grouped.clone()));
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
                let x_if: Arc<RwLock<XIf>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .x_if
                    .write()
                    .await
                    .insert(x_if.read().await.id, Some(x_if.clone()));
            }
        }

        // Load Implementation Block.
        {
            let path = path.join("implementation_block");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let implementation_block: Arc<RwLock<ImplementationBlock>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store.implementation_block.write().await.insert(
                    implementation_block.read().await.id,
                    Some(implementation_block.clone()),
                );
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
                let import: Arc<RwLock<Import>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .import
                    .write()
                    .await
                    .insert(import.read().await.id, Some(import.clone()));
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
                let index: Arc<RwLock<Index>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .index
                    .write()
                    .await
                    .insert(index.read().await.id, Some(index.clone()));
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
                let integer_literal: Arc<RwLock<IntegerLiteral>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store.integer_literal.write().await.insert(
                    integer_literal.read().await.id,
                    Some(integer_literal.clone()),
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
                let item: Arc<RwLock<Item>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .item
                    .write()
                    .await
                    .insert(item.read().await.id, Some(item.clone()));
            }
        }

        // Load Lambda.
        {
            let path = path.join("lambda");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let lambda: Arc<RwLock<Lambda>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .lambda
                    .write()
                    .await
                    .insert(lambda.read().await.id, Some(lambda.clone()));
            }
        }

        // Load Lambda Parameter.
        {
            let path = path.join("lambda_parameter");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let lambda_parameter: Arc<RwLock<LambdaParameter>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store.lambda_parameter.write().await.insert(
                    lambda_parameter.read().await.id,
                    Some(lambda_parameter.clone()),
                );
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
                let let_statement: Arc<RwLock<LetStatement>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .let_statement
                    .write()
                    .await
                    .insert(let_statement.read().await.id, Some(let_statement.clone()));
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
                let list: Arc<RwLock<List>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .list
                    .write()
                    .await
                    .insert(list.read().await.id, Some(list.clone()));
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
                let list_element: Arc<RwLock<ListElement>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .list_element
                    .write()
                    .await
                    .insert(list_element.read().await.id, Some(list_element.clone()));
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
                let list_expression: Arc<RwLock<ListExpression>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store.list_expression.write().await.insert(
                    list_expression.read().await.id,
                    Some(list_expression.clone()),
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
                let literal: Arc<RwLock<Literal>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .literal
                    .write()
                    .await
                    .insert(literal.read().await.id, Some(literal.clone()));
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
                let local_variable: Arc<RwLock<LocalVariable>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .local_variable
                    .write()
                    .await
                    .insert(local_variable.read().await.id, Some(local_variable.clone()));
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
                let x_macro: Arc<RwLock<XMacro>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .x_macro
                    .write()
                    .await
                    .insert(x_macro.read().await.id, Some(x_macro.clone()));
            }
        }

        // Load Match.
        {
            let path = path.join("x_match");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let x_match: Arc<RwLock<XMatch>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .x_match
                    .write()
                    .await
                    .insert(x_match.read().await.id, Some(x_match.clone()));
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
                let method_call: Arc<RwLock<MethodCall>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .method_call
                    .write()
                    .await
                    .insert(method_call.read().await.id, Some(method_call.clone()));
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
                let z_object_store: Arc<RwLock<ZObjectStore>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store.z_object_store_id_by_name.write().await.insert(
                    z_object_store.read().await.name.to_owned(),
                    z_object_store.read().await.id,
                );
                store
                    .z_object_store
                    .write()
                    .await
                    .insert(z_object_store.read().await.id, Some(z_object_store.clone()));
            }
        }

        // Load Object Wrapper.
        {
            let path = path.join("object_wrapper");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let object_wrapper: Arc<RwLock<ObjectWrapper>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .object_wrapper
                    .write()
                    .await
                    .insert(object_wrapper.read().await.id, Some(object_wrapper.clone()));
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
                let operator: Arc<RwLock<Operator>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .operator
                    .write()
                    .await
                    .insert(operator.read().await.id, Some(operator.clone()));
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
                let woog_option: Arc<RwLock<WoogOption>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .woog_option
                    .write()
                    .await
                    .insert(woog_option.read().await.id, Some(woog_option.clone()));
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
                let parameter: Arc<RwLock<Parameter>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .parameter
                    .write()
                    .await
                    .insert(parameter.read().await.id, Some(parameter.clone()));
            }
        }

        // Load Pattern.
        {
            let path = path.join("pattern");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let pattern: Arc<RwLock<Pattern>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .pattern
                    .write()
                    .await
                    .insert(pattern.read().await.id, Some(pattern.clone()));
            }
        }

        // Load Plain.
        {
            let path = path.join("plain");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let plain: Arc<RwLock<Plain>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .plain
                    .write()
                    .await
                    .insert(plain.read().await.id, Some(plain.clone()));
            }
        }

        // Load Print.
        {
            let path = path.join("x_print");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let x_print: Arc<RwLock<XPrint>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .x_print
                    .write()
                    .await
                    .insert(x_print.read().await.id, Some(x_print.clone()));
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
                let range_expression: Arc<RwLock<RangeExpression>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store.range_expression.write().await.insert(
                    range_expression.read().await.id,
                    Some(range_expression.clone()),
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
                let reference: Arc<RwLock<Reference>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .reference
                    .write()
                    .await
                    .insert(reference.read().await.id, Some(reference.clone()));
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
                let result_statement: Arc<RwLock<ResultStatement>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store.result_statement.write().await.insert(
                    result_statement.read().await.id,
                    Some(result_statement.clone()),
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
                let x_return: Arc<RwLock<XReturn>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .x_return
                    .write()
                    .await
                    .insert(x_return.read().await.id, Some(x_return.clone()));
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
                let z_some: Arc<RwLock<ZSome>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .z_some
                    .write()
                    .await
                    .insert(z_some.read().await.id, Some(z_some.clone()));
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
                let span: Arc<RwLock<Span>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .span
                    .write()
                    .await
                    .insert(span.read().await.id, Some(span.clone()));
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
                let statement: Arc<RwLock<Statement>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .statement
                    .write()
                    .await
                    .insert(statement.read().await.id, Some(statement.clone()));
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
                let static_method_call: Arc<RwLock<StaticMethodCall>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store.static_method_call.write().await.insert(
                    static_method_call.read().await.id,
                    Some(static_method_call.clone()),
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
                let string_literal: Arc<RwLock<StringLiteral>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .string_literal
                    .write()
                    .await
                    .insert(string_literal.read().await.id, Some(string_literal.clone()));
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
                let woog_struct: Arc<RwLock<WoogStruct>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store.woog_struct_id_by_name.write().await.insert(
                    woog_struct.read().await.name.to_owned(),
                    woog_struct.read().await.id,
                );
                store
                    .woog_struct
                    .write()
                    .await
                    .insert(woog_struct.read().await.id, Some(woog_struct.clone()));
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
                let struct_expression: Arc<RwLock<StructExpression>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store.struct_expression.write().await.insert(
                    struct_expression.read().await.id,
                    Some(struct_expression.clone()),
                );
            }
        }

        // Load Struct Field.
        {
            let path = path.join("struct_field");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let struct_field: Arc<RwLock<StructField>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .struct_field
                    .write()
                    .await
                    .insert(struct_field.read().await.id, Some(struct_field.clone()));
            }
        }

        // Load Tuple Field.
        {
            let path = path.join("tuple_field");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let tuple_field: Arc<RwLock<TupleField>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .tuple_field
                    .write()
                    .await
                    .insert(tuple_field.read().await.id, Some(tuple_field.clone()));
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
                let type_cast: Arc<RwLock<TypeCast>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .type_cast
                    .write()
                    .await
                    .insert(type_cast.read().await.id, Some(type_cast.clone()));
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
                let unary: Arc<RwLock<Unary>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .unary
                    .write()
                    .await
                    .insert(unary.read().await.id, Some(unary.clone()));
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
                let x_value: Arc<RwLock<XValue>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .x_value
                    .write()
                    .await
                    .insert(x_value.read().await.id, Some(x_value.clone()));
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
                let value_type: Arc<RwLock<ValueType>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .value_type
                    .write()
                    .await
                    .insert(value_type.read().await.id, Some(value_type.clone()));
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
                let variable: Arc<RwLock<Variable>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store
                    .variable
                    .write()
                    .await
                    .insert(variable.read().await.id, Some(variable.clone()));
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
                let variable_expression: Arc<RwLock<VariableExpression>> =
                    serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;
                store.variable_expression.write().await.insert(
                    variable_expression.read().await.id,
                    Some(variable_expression.clone()),
                );
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
