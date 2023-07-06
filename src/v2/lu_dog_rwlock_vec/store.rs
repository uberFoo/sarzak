//! v2::lu_dog_rwlock_vec Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_rwlock_vec-object-store-file"}}}
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
//! * [`Lambda`]
//! * [`LambdaParameter`]
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_rwlock_vec-object-store-definition"}}}
use std::sync::Arc;
use std::sync::RwLock;
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
};

use fnv::FnvHashMap as HashMap;
use heck::ToUpperCamelCase;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v2::lu_dog_rwlock_vec::types::{
    Argument, Binary, Block, BooleanLiteral, BooleanOperator, Call, Comparison, DwarfSourceFile,
    Error, ErrorExpression, Expression, ExpressionStatement, Field, FieldAccess, FieldAccessTarget,
    FieldExpression, FloatLiteral, ForLoop, Function, Grouped, Implementation, Import, Index,
    IntegerLiteral, Item, Lambda, LambdaParameter, LetStatement, List, ListElement, ListExpression,
    Literal, LocalVariable, MethodCall, Operator, Parameter, Print, RangeExpression, Reference,
    ResultStatement, Span, Statement, StaticMethodCall, StringLiteral, StructExpression, TypeCast,
    Unary, ValueType, Variable, VariableExpression, WoogOption, WoogStruct, XIf, XMacro, XReturn,
    XValue, ZObjectStore, ZSome, ADDITION, AND, ASSIGNMENT, CHAR, DEBUGGER, DIVISION, EMPTY, EQUAL,
    FALSE_LITERAL, FROM, FULL, FUNCTION_CALL, GREATER_THAN, GREATER_THAN_OR_EQUAL, INCLUSIVE,
    ITEM_STATEMENT, LESS_THAN, LESS_THAN_OR_EQUAL, MACRO_CALL, MULTIPLICATION, NEGATION, NOT,
    NOT_EQUAL, OR, RANGE, SUBTRACTION, TO, TO_INCLUSIVE, TRUE_LITERAL, UNKNOWN, UNKNOWN_VARIABLE,
    Z_NONE,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    argument_free_list: std::sync::Mutex<Vec<usize>>,
    argument: Arc<RwLock<Vec<Option<Arc<RwLock<Argument>>>>>>,
    binary_free_list: std::sync::Mutex<Vec<usize>>,
    binary: Arc<RwLock<Vec<Option<Arc<RwLock<Binary>>>>>>,
    block_free_list: std::sync::Mutex<Vec<usize>>,
    block: Arc<RwLock<Vec<Option<Arc<RwLock<Block>>>>>>,
    boolean_literal_free_list: std::sync::Mutex<Vec<usize>>,
    boolean_literal: Arc<RwLock<Vec<Option<Arc<RwLock<BooleanLiteral>>>>>>,
    boolean_operator_free_list: std::sync::Mutex<Vec<usize>>,
    boolean_operator: Arc<RwLock<Vec<Option<Arc<RwLock<BooleanOperator>>>>>>,
    call_free_list: std::sync::Mutex<Vec<usize>>,
    call: Arc<RwLock<Vec<Option<Arc<RwLock<Call>>>>>>,
    comparison_free_list: std::sync::Mutex<Vec<usize>>,
    comparison: Arc<RwLock<Vec<Option<Arc<RwLock<Comparison>>>>>>,
    dwarf_source_file_free_list: std::sync::Mutex<Vec<usize>>,
    dwarf_source_file: Arc<RwLock<Vec<Option<Arc<RwLock<DwarfSourceFile>>>>>>,
    error_free_list: std::sync::Mutex<Vec<usize>>,
    error: Arc<RwLock<Vec<Option<Arc<RwLock<Error>>>>>>,
    error_expression_free_list: std::sync::Mutex<Vec<usize>>,
    error_expression: Arc<RwLock<Vec<Option<Arc<RwLock<ErrorExpression>>>>>>,
    expression_free_list: std::sync::Mutex<Vec<usize>>,
    expression: Arc<RwLock<Vec<Option<Arc<RwLock<Expression>>>>>>,
    expression_statement_free_list: std::sync::Mutex<Vec<usize>>,
    expression_statement: Arc<RwLock<Vec<Option<Arc<RwLock<ExpressionStatement>>>>>>,
    field_free_list: std::sync::Mutex<Vec<usize>>,
    field: Arc<RwLock<Vec<Option<Arc<RwLock<Field>>>>>>,
    field_id_by_name: Arc<RwLock<HashMap<String, usize>>>,
    field_access_free_list: std::sync::Mutex<Vec<usize>>,
    field_access: Arc<RwLock<Vec<Option<Arc<RwLock<FieldAccess>>>>>>,
    field_access_target_free_list: std::sync::Mutex<Vec<usize>>,
    field_access_target: Arc<RwLock<Vec<Option<Arc<RwLock<FieldAccessTarget>>>>>>,
    field_expression_free_list: std::sync::Mutex<Vec<usize>>,
    field_expression: Arc<RwLock<Vec<Option<Arc<RwLock<FieldExpression>>>>>>,
    float_literal_free_list: std::sync::Mutex<Vec<usize>>,
    float_literal: Arc<RwLock<Vec<Option<Arc<RwLock<FloatLiteral>>>>>>,
    for_loop_free_list: std::sync::Mutex<Vec<usize>>,
    for_loop: Arc<RwLock<Vec<Option<Arc<RwLock<ForLoop>>>>>>,
    function_free_list: std::sync::Mutex<Vec<usize>>,
    function: Arc<RwLock<Vec<Option<Arc<RwLock<Function>>>>>>,
    function_id_by_name: Arc<RwLock<HashMap<String, usize>>>,
    grouped_free_list: std::sync::Mutex<Vec<usize>>,
    grouped: Arc<RwLock<Vec<Option<Arc<RwLock<Grouped>>>>>>,
    x_if_free_list: std::sync::Mutex<Vec<usize>>,
    x_if: Arc<RwLock<Vec<Option<Arc<RwLock<XIf>>>>>>,
    implementation_free_list: std::sync::Mutex<Vec<usize>>,
    implementation: Arc<RwLock<Vec<Option<Arc<RwLock<Implementation>>>>>>,
    import_free_list: std::sync::Mutex<Vec<usize>>,
    import: Arc<RwLock<Vec<Option<Arc<RwLock<Import>>>>>>,
    index_free_list: std::sync::Mutex<Vec<usize>>,
    index: Arc<RwLock<Vec<Option<Arc<RwLock<Index>>>>>>,
    integer_literal_free_list: std::sync::Mutex<Vec<usize>>,
    integer_literal: Arc<RwLock<Vec<Option<Arc<RwLock<IntegerLiteral>>>>>>,
    item_free_list: std::sync::Mutex<Vec<usize>>,
    item: Arc<RwLock<Vec<Option<Arc<RwLock<Item>>>>>>,
    lambda_free_list: std::sync::Mutex<Vec<usize>>,
    lambda: Arc<RwLock<Vec<Option<Arc<RwLock<Lambda>>>>>>,
    lambda_parameter_free_list: std::sync::Mutex<Vec<usize>>,
    lambda_parameter: Arc<RwLock<Vec<Option<Arc<RwLock<LambdaParameter>>>>>>,
    let_statement_free_list: std::sync::Mutex<Vec<usize>>,
    let_statement: Arc<RwLock<Vec<Option<Arc<RwLock<LetStatement>>>>>>,
    list_free_list: std::sync::Mutex<Vec<usize>>,
    list: Arc<RwLock<Vec<Option<Arc<RwLock<List>>>>>>,
    list_element_free_list: std::sync::Mutex<Vec<usize>>,
    list_element: Arc<RwLock<Vec<Option<Arc<RwLock<ListElement>>>>>>,
    list_expression_free_list: std::sync::Mutex<Vec<usize>>,
    list_expression: Arc<RwLock<Vec<Option<Arc<RwLock<ListExpression>>>>>>,
    literal_free_list: std::sync::Mutex<Vec<usize>>,
    literal: Arc<RwLock<Vec<Option<Arc<RwLock<Literal>>>>>>,
    local_variable_free_list: std::sync::Mutex<Vec<usize>>,
    local_variable: Arc<RwLock<Vec<Option<Arc<RwLock<LocalVariable>>>>>>,
    x_macro_free_list: std::sync::Mutex<Vec<usize>>,
    x_macro: Arc<RwLock<Vec<Option<Arc<RwLock<XMacro>>>>>>,
    method_call_free_list: std::sync::Mutex<Vec<usize>>,
    method_call: Arc<RwLock<Vec<Option<Arc<RwLock<MethodCall>>>>>>,
    z_object_store_free_list: std::sync::Mutex<Vec<usize>>,
    z_object_store: Arc<RwLock<Vec<Option<Arc<RwLock<ZObjectStore>>>>>>,
    operator_free_list: std::sync::Mutex<Vec<usize>>,
    operator: Arc<RwLock<Vec<Option<Arc<RwLock<Operator>>>>>>,
    woog_option_free_list: std::sync::Mutex<Vec<usize>>,
    woog_option: Arc<RwLock<Vec<Option<Arc<RwLock<WoogOption>>>>>>,
    parameter_free_list: std::sync::Mutex<Vec<usize>>,
    parameter: Arc<RwLock<Vec<Option<Arc<RwLock<Parameter>>>>>>,
    print_free_list: std::sync::Mutex<Vec<usize>>,
    print: Arc<RwLock<Vec<Option<Arc<RwLock<Print>>>>>>,
    range_expression_free_list: std::sync::Mutex<Vec<usize>>,
    range_expression: Arc<RwLock<Vec<Option<Arc<RwLock<RangeExpression>>>>>>,
    reference_free_list: std::sync::Mutex<Vec<usize>>,
    reference: Arc<RwLock<Vec<Option<Arc<RwLock<Reference>>>>>>,
    result_statement_free_list: std::sync::Mutex<Vec<usize>>,
    result_statement: Arc<RwLock<Vec<Option<Arc<RwLock<ResultStatement>>>>>>,
    x_return_free_list: std::sync::Mutex<Vec<usize>>,
    x_return: Arc<RwLock<Vec<Option<Arc<RwLock<XReturn>>>>>>,
    z_some_free_list: std::sync::Mutex<Vec<usize>>,
    z_some: Arc<RwLock<Vec<Option<Arc<RwLock<ZSome>>>>>>,
    span_free_list: std::sync::Mutex<Vec<usize>>,
    span: Arc<RwLock<Vec<Option<Arc<RwLock<Span>>>>>>,
    statement_free_list: std::sync::Mutex<Vec<usize>>,
    statement: Arc<RwLock<Vec<Option<Arc<RwLock<Statement>>>>>>,
    static_method_call_free_list: std::sync::Mutex<Vec<usize>>,
    static_method_call: Arc<RwLock<Vec<Option<Arc<RwLock<StaticMethodCall>>>>>>,
    string_literal_free_list: std::sync::Mutex<Vec<usize>>,
    string_literal: Arc<RwLock<Vec<Option<Arc<RwLock<StringLiteral>>>>>>,
    woog_struct_free_list: std::sync::Mutex<Vec<usize>>,
    woog_struct: Arc<RwLock<Vec<Option<Arc<RwLock<WoogStruct>>>>>>,
    woog_struct_id_by_name: Arc<RwLock<HashMap<String, usize>>>,
    struct_expression_free_list: std::sync::Mutex<Vec<usize>>,
    struct_expression: Arc<RwLock<Vec<Option<Arc<RwLock<StructExpression>>>>>>,
    type_cast_free_list: std::sync::Mutex<Vec<usize>>,
    type_cast: Arc<RwLock<Vec<Option<Arc<RwLock<TypeCast>>>>>>,
    unary_free_list: std::sync::Mutex<Vec<usize>>,
    unary: Arc<RwLock<Vec<Option<Arc<RwLock<Unary>>>>>>,
    x_value_free_list: std::sync::Mutex<Vec<usize>>,
    x_value: Arc<RwLock<Vec<Option<Arc<RwLock<XValue>>>>>>,
    value_type_free_list: std::sync::Mutex<Vec<usize>>,
    value_type: Arc<RwLock<Vec<Option<Arc<RwLock<ValueType>>>>>>,
    variable_free_list: std::sync::Mutex<Vec<usize>>,
    variable: Arc<RwLock<Vec<Option<Arc<RwLock<Variable>>>>>>,
    variable_expression_free_list: std::sync::Mutex<Vec<usize>>,
    variable_expression: Arc<RwLock<Vec<Option<Arc<RwLock<VariableExpression>>>>>>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            argument_free_list: std::sync::Mutex::new(Vec::new()),
            argument: Arc::new(RwLock::new(Vec::new())),
            binary_free_list: std::sync::Mutex::new(Vec::new()),
            binary: Arc::new(RwLock::new(Vec::new())),
            block_free_list: std::sync::Mutex::new(Vec::new()),
            block: Arc::new(RwLock::new(Vec::new())),
            boolean_literal_free_list: std::sync::Mutex::new(Vec::new()),
            boolean_literal: Arc::new(RwLock::new(Vec::new())),
            boolean_operator_free_list: std::sync::Mutex::new(Vec::new()),
            boolean_operator: Arc::new(RwLock::new(Vec::new())),
            call_free_list: std::sync::Mutex::new(Vec::new()),
            call: Arc::new(RwLock::new(Vec::new())),
            comparison_free_list: std::sync::Mutex::new(Vec::new()),
            comparison: Arc::new(RwLock::new(Vec::new())),
            dwarf_source_file_free_list: std::sync::Mutex::new(Vec::new()),
            dwarf_source_file: Arc::new(RwLock::new(Vec::new())),
            error_free_list: std::sync::Mutex::new(Vec::new()),
            error: Arc::new(RwLock::new(Vec::new())),
            error_expression_free_list: std::sync::Mutex::new(Vec::new()),
            error_expression: Arc::new(RwLock::new(Vec::new())),
            expression_free_list: std::sync::Mutex::new(Vec::new()),
            expression: Arc::new(RwLock::new(Vec::new())),
            expression_statement_free_list: std::sync::Mutex::new(Vec::new()),
            expression_statement: Arc::new(RwLock::new(Vec::new())),
            field_free_list: std::sync::Mutex::new(Vec::new()),
            field: Arc::new(RwLock::new(Vec::new())),
            field_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            field_access_free_list: std::sync::Mutex::new(Vec::new()),
            field_access: Arc::new(RwLock::new(Vec::new())),
            field_access_target_free_list: std::sync::Mutex::new(Vec::new()),
            field_access_target: Arc::new(RwLock::new(Vec::new())),
            field_expression_free_list: std::sync::Mutex::new(Vec::new()),
            field_expression: Arc::new(RwLock::new(Vec::new())),
            float_literal_free_list: std::sync::Mutex::new(Vec::new()),
            float_literal: Arc::new(RwLock::new(Vec::new())),
            for_loop_free_list: std::sync::Mutex::new(Vec::new()),
            for_loop: Arc::new(RwLock::new(Vec::new())),
            function_free_list: std::sync::Mutex::new(Vec::new()),
            function: Arc::new(RwLock::new(Vec::new())),
            function_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            grouped_free_list: std::sync::Mutex::new(Vec::new()),
            grouped: Arc::new(RwLock::new(Vec::new())),
            x_if_free_list: std::sync::Mutex::new(Vec::new()),
            x_if: Arc::new(RwLock::new(Vec::new())),
            implementation_free_list: std::sync::Mutex::new(Vec::new()),
            implementation: Arc::new(RwLock::new(Vec::new())),
            import_free_list: std::sync::Mutex::new(Vec::new()),
            import: Arc::new(RwLock::new(Vec::new())),
            index_free_list: std::sync::Mutex::new(Vec::new()),
            index: Arc::new(RwLock::new(Vec::new())),
            integer_literal_free_list: std::sync::Mutex::new(Vec::new()),
            integer_literal: Arc::new(RwLock::new(Vec::new())),
            item_free_list: std::sync::Mutex::new(Vec::new()),
            item: Arc::new(RwLock::new(Vec::new())),
            lambda_free_list: std::sync::Mutex::new(Vec::new()),
            lambda: Arc::new(RwLock::new(Vec::new())),
            lambda_parameter_free_list: std::sync::Mutex::new(Vec::new()),
            lambda_parameter: Arc::new(RwLock::new(Vec::new())),
            let_statement_free_list: std::sync::Mutex::new(Vec::new()),
            let_statement: Arc::new(RwLock::new(Vec::new())),
            list_free_list: std::sync::Mutex::new(Vec::new()),
            list: Arc::new(RwLock::new(Vec::new())),
            list_element_free_list: std::sync::Mutex::new(Vec::new()),
            list_element: Arc::new(RwLock::new(Vec::new())),
            list_expression_free_list: std::sync::Mutex::new(Vec::new()),
            list_expression: Arc::new(RwLock::new(Vec::new())),
            literal_free_list: std::sync::Mutex::new(Vec::new()),
            literal: Arc::new(RwLock::new(Vec::new())),
            local_variable_free_list: std::sync::Mutex::new(Vec::new()),
            local_variable: Arc::new(RwLock::new(Vec::new())),
            x_macro_free_list: std::sync::Mutex::new(Vec::new()),
            x_macro: Arc::new(RwLock::new(Vec::new())),
            method_call_free_list: std::sync::Mutex::new(Vec::new()),
            method_call: Arc::new(RwLock::new(Vec::new())),
            z_object_store_free_list: std::sync::Mutex::new(Vec::new()),
            z_object_store: Arc::new(RwLock::new(Vec::new())),
            operator_free_list: std::sync::Mutex::new(Vec::new()),
            operator: Arc::new(RwLock::new(Vec::new())),
            woog_option_free_list: std::sync::Mutex::new(Vec::new()),
            woog_option: Arc::new(RwLock::new(Vec::new())),
            parameter_free_list: std::sync::Mutex::new(Vec::new()),
            parameter: Arc::new(RwLock::new(Vec::new())),
            print_free_list: std::sync::Mutex::new(Vec::new()),
            print: Arc::new(RwLock::new(Vec::new())),
            range_expression_free_list: std::sync::Mutex::new(Vec::new()),
            range_expression: Arc::new(RwLock::new(Vec::new())),
            reference_free_list: std::sync::Mutex::new(Vec::new()),
            reference: Arc::new(RwLock::new(Vec::new())),
            result_statement_free_list: std::sync::Mutex::new(Vec::new()),
            result_statement: Arc::new(RwLock::new(Vec::new())),
            x_return_free_list: std::sync::Mutex::new(Vec::new()),
            x_return: Arc::new(RwLock::new(Vec::new())),
            z_some_free_list: std::sync::Mutex::new(Vec::new()),
            z_some: Arc::new(RwLock::new(Vec::new())),
            span_free_list: std::sync::Mutex::new(Vec::new()),
            span: Arc::new(RwLock::new(Vec::new())),
            statement_free_list: std::sync::Mutex::new(Vec::new()),
            statement: Arc::new(RwLock::new(Vec::new())),
            static_method_call_free_list: std::sync::Mutex::new(Vec::new()),
            static_method_call: Arc::new(RwLock::new(Vec::new())),
            string_literal_free_list: std::sync::Mutex::new(Vec::new()),
            string_literal: Arc::new(RwLock::new(Vec::new())),
            woog_struct_free_list: std::sync::Mutex::new(Vec::new()),
            woog_struct: Arc::new(RwLock::new(Vec::new())),
            woog_struct_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            struct_expression_free_list: std::sync::Mutex::new(Vec::new()),
            struct_expression: Arc::new(RwLock::new(Vec::new())),
            type_cast_free_list: std::sync::Mutex::new(Vec::new()),
            type_cast: Arc::new(RwLock::new(Vec::new())),
            unary_free_list: std::sync::Mutex::new(Vec::new()),
            unary: Arc::new(RwLock::new(Vec::new())),
            x_value_free_list: std::sync::Mutex::new(Vec::new()),
            x_value: Arc::new(RwLock::new(Vec::new())),
            value_type_free_list: std::sync::Mutex::new(Vec::new()),
            value_type: Arc::new(RwLock::new(Vec::new())),
            variable_free_list: std::sync::Mutex::new(Vec::new()),
            variable: Arc::new(RwLock::new(Vec::new())),
            variable_expression_free_list: std::sync::Mutex::new(Vec::new()),
            variable_expression: Arc::new(RwLock::new(Vec::new())),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥
        store.inter_binary(|id| {
            Arc::new(RwLock::new(Binary {
                subtype: super::BinaryEnum::Addition(ADDITION),
                id,
            }))
        });
        store.inter_binary(|id| {
            Arc::new(RwLock::new(Binary {
                subtype: super::BinaryEnum::Assignment(ASSIGNMENT),
                id,
            }))
        });
        store.inter_binary(|id| {
            Arc::new(RwLock::new(Binary {
                subtype: super::BinaryEnum::Division(DIVISION),
                id,
            }))
        });
        store.inter_binary(|id| {
            Arc::new(RwLock::new(Binary {
                subtype: super::BinaryEnum::Multiplication(MULTIPLICATION),
                id,
            }))
        });
        store.inter_binary(|id| {
            Arc::new(RwLock::new(Binary {
                subtype: super::BinaryEnum::Subtraction(SUBTRACTION),
                id,
            }))
        });
        store.inter_boolean_literal(|id| {
            Arc::new(RwLock::new(BooleanLiteral {
                subtype: super::BooleanLiteralEnum::FalseLiteral(FALSE_LITERAL),
                id,
            }))
        });
        store.inter_boolean_literal(|id| {
            Arc::new(RwLock::new(BooleanLiteral {
                subtype: super::BooleanLiteralEnum::TrueLiteral(TRUE_LITERAL),
                id,
            }))
        });
        store.inter_boolean_operator(|id| {
            Arc::new(RwLock::new(BooleanOperator {
                subtype: super::BooleanOperatorEnum::And(AND),
                id,
            }))
        });
        store.inter_boolean_operator(|id| {
            Arc::new(RwLock::new(BooleanOperator {
                subtype: super::BooleanOperatorEnum::Or(OR),
                id,
            }))
        });
        store.inter_comparison(|id| {
            Arc::new(RwLock::new(Comparison {
                subtype: super::ComparisonEnum::Equal(EQUAL),
                id,
            }))
        });
        store.inter_comparison(|id| {
            Arc::new(RwLock::new(Comparison {
                subtype: super::ComparisonEnum::GreaterThan(GREATER_THAN),
                id,
            }))
        });
        store.inter_comparison(|id| {
            Arc::new(RwLock::new(Comparison {
                subtype: super::ComparisonEnum::GreaterThanOrEqual(GREATER_THAN_OR_EQUAL),
                id,
            }))
        });
        store.inter_comparison(|id| {
            Arc::new(RwLock::new(Comparison {
                subtype: super::ComparisonEnum::LessThan(LESS_THAN),
                id,
            }))
        });
        store.inter_comparison(|id| {
            Arc::new(RwLock::new(Comparison {
                subtype: super::ComparisonEnum::LessThanOrEqual(LESS_THAN_OR_EQUAL),
                id,
            }))
        });
        store.inter_comparison(|id| {
            Arc::new(RwLock::new(Comparison {
                subtype: super::ComparisonEnum::NotEqual(NOT_EQUAL),
                id,
            }))
        });
        store.inter_error(|id| {
            Arc::new(RwLock::new(Error {
                subtype: super::ErrorEnum::UnknownVariable(UNKNOWN_VARIABLE),
                id,
            }))
        });
        store.inter_expression(|id| {
            Arc::new(RwLock::new(Expression {
                subtype: super::ExpressionEnum::Debugger(DEBUGGER),
                id,
            }))
        });
        store.inter_expression(|id| {
            Arc::new(RwLock::new(Expression {
                subtype: super::ExpressionEnum::ZNone(Z_NONE),
                id,
            }))
        });
        store.inter_unary(|id| {
            Arc::new(RwLock::new(Unary {
                subtype: super::UnaryEnum::Negation(NEGATION),
                id,
            }))
        });
        store.inter_unary(|id| {
            Arc::new(RwLock::new(Unary {
                subtype: super::UnaryEnum::Not(NOT),
                id,
            }))
        });
        store.inter_value_type(|id| {
            Arc::new(RwLock::new(ValueType {
                subtype: super::ValueTypeEnum::Char(CHAR),
                id,
            }))
        });
        store.inter_value_type(|id| {
            Arc::new(RwLock::new(ValueType {
                subtype: super::ValueTypeEnum::Empty(EMPTY),
                id,
            }))
        });
        store.inter_value_type(|id| {
            Arc::new(RwLock::new(ValueType {
                subtype: super::ValueTypeEnum::Range(RANGE),
                id,
            }))
        });
        store.inter_value_type(|id| {
            Arc::new(RwLock::new(ValueType {
                subtype: super::ValueTypeEnum::Unknown(UNKNOWN),
                id,
            }))
        });

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_rwlock_vec-object-store-methods"}}}
    /// Inter (insert) [`Argument`] into the store.
    ///
    pub fn inter_argument<F>(&mut self, argument: F) -> Arc<RwLock<Argument>>
    where
        F: Fn(usize) -> Arc<RwLock<Argument>>,
    {
        if let Some(_index) = self.argument_free_list.lock().unwrap().pop() {
            let argument = argument(_index);
            self.argument.write().unwrap()[_index] = Some(argument.clone());
            argument
        } else {
            let _index = self.argument.read().unwrap().len();
            let argument = argument(_index);
            self.argument.write().unwrap().push(Some(argument.clone()));
            argument
        }
    }

    /// Exhume (get) [`Argument`] from the store.
    ///
    pub fn exhume_argument(&self, id: &usize) -> Option<Arc<RwLock<Argument>>> {
        match self.argument.read().unwrap().get(*id) {
            Some(argument) => argument.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Argument`] from the store.
    ///
    pub fn exorcise_argument(&mut self, id: &usize) -> Option<Arc<RwLock<Argument>>> {
        let result = self.argument.write().unwrap()[*id].take();
        self.argument_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Argument>`.
    ///
    pub fn iter_argument(&self) -> impl Iterator<Item = Arc<RwLock<Argument>>> + '_ {
        let len = self.argument.read().unwrap().len();
        (0..len).map(move |i| {
            self.argument.read().unwrap()[i]
                .as_ref()
                .map(|argument| argument.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Binary`] into the store.
    ///
    pub fn inter_binary<F>(&mut self, binary: F) -> Arc<RwLock<Binary>>
    where
        F: Fn(usize) -> Arc<RwLock<Binary>>,
    {
        if let Some(_index) = self.binary_free_list.lock().unwrap().pop() {
            let binary = binary(_index);
            self.binary.write().unwrap()[_index] = Some(binary.clone());
            binary
        } else {
            let _index = self.binary.read().unwrap().len();
            let binary = binary(_index);
            self.binary.write().unwrap().push(Some(binary.clone()));
            binary
        }
    }

    /// Exhume (get) [`Binary`] from the store.
    ///
    pub fn exhume_binary(&self, id: &usize) -> Option<Arc<RwLock<Binary>>> {
        match self.binary.read().unwrap().get(*id) {
            Some(binary) => binary.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Binary`] from the store.
    ///
    pub fn exorcise_binary(&mut self, id: &usize) -> Option<Arc<RwLock<Binary>>> {
        let result = self.binary.write().unwrap()[*id].take();
        self.binary_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Binary>`.
    ///
    pub fn iter_binary(&self) -> impl Iterator<Item = Arc<RwLock<Binary>>> + '_ {
        let len = self.binary.read().unwrap().len();
        (0..len).map(move |i| {
            self.binary.read().unwrap()[i]
                .as_ref()
                .map(|binary| binary.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Block`] into the store.
    ///
    pub fn inter_block<F>(&mut self, block: F) -> Arc<RwLock<Block>>
    where
        F: Fn(usize) -> Arc<RwLock<Block>>,
    {
        if let Some(_index) = self.block_free_list.lock().unwrap().pop() {
            let block = block(_index);
            self.block.write().unwrap()[_index] = Some(block.clone());
            block
        } else {
            let _index = self.block.read().unwrap().len();
            let block = block(_index);
            self.block.write().unwrap().push(Some(block.clone()));
            block
        }
    }

    /// Exhume (get) [`Block`] from the store.
    ///
    pub fn exhume_block(&self, id: &usize) -> Option<Arc<RwLock<Block>>> {
        match self.block.read().unwrap().get(*id) {
            Some(block) => block.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Block`] from the store.
    ///
    pub fn exorcise_block(&mut self, id: &usize) -> Option<Arc<RwLock<Block>>> {
        let result = self.block.write().unwrap()[*id].take();
        self.block_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Block>`.
    ///
    pub fn iter_block(&self) -> impl Iterator<Item = Arc<RwLock<Block>>> + '_ {
        let len = self.block.read().unwrap().len();
        (0..len).map(move |i| {
            self.block.read().unwrap()[i]
                .as_ref()
                .map(|block| block.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`BooleanLiteral`] into the store.
    ///
    pub fn inter_boolean_literal<F>(&mut self, boolean_literal: F) -> Arc<RwLock<BooleanLiteral>>
    where
        F: Fn(usize) -> Arc<RwLock<BooleanLiteral>>,
    {
        if let Some(_index) = self.boolean_literal_free_list.lock().unwrap().pop() {
            let boolean_literal = boolean_literal(_index);
            self.boolean_literal.write().unwrap()[_index] = Some(boolean_literal.clone());
            boolean_literal
        } else {
            let _index = self.boolean_literal.read().unwrap().len();
            let boolean_literal = boolean_literal(_index);
            self.boolean_literal
                .write()
                .unwrap()
                .push(Some(boolean_literal.clone()));
            boolean_literal
        }
    }

    /// Exhume (get) [`BooleanLiteral`] from the store.
    ///
    pub fn exhume_boolean_literal(&self, id: &usize) -> Option<Arc<RwLock<BooleanLiteral>>> {
        match self.boolean_literal.read().unwrap().get(*id) {
            Some(boolean_literal) => boolean_literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`BooleanLiteral`] from the store.
    ///
    pub fn exorcise_boolean_literal(&mut self, id: &usize) -> Option<Arc<RwLock<BooleanLiteral>>> {
        let result = self.boolean_literal.write().unwrap()[*id].take();
        self.boolean_literal_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanLiteral>`.
    ///
    pub fn iter_boolean_literal(&self) -> impl Iterator<Item = Arc<RwLock<BooleanLiteral>>> + '_ {
        let len = self.boolean_literal.read().unwrap().len();
        (0..len).map(move |i| {
            self.boolean_literal.read().unwrap()[i]
                .as_ref()
                .map(|boolean_literal| boolean_literal.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`BooleanOperator`] into the store.
    ///
    pub fn inter_boolean_operator<F>(&mut self, boolean_operator: F) -> Arc<RwLock<BooleanOperator>>
    where
        F: Fn(usize) -> Arc<RwLock<BooleanOperator>>,
    {
        if let Some(_index) = self.boolean_operator_free_list.lock().unwrap().pop() {
            let boolean_operator = boolean_operator(_index);
            self.boolean_operator.write().unwrap()[_index] = Some(boolean_operator.clone());
            boolean_operator
        } else {
            let _index = self.boolean_operator.read().unwrap().len();
            let boolean_operator = boolean_operator(_index);
            self.boolean_operator
                .write()
                .unwrap()
                .push(Some(boolean_operator.clone()));
            boolean_operator
        }
    }

    /// Exhume (get) [`BooleanOperator`] from the store.
    ///
    pub fn exhume_boolean_operator(&self, id: &usize) -> Option<Arc<RwLock<BooleanOperator>>> {
        match self.boolean_operator.read().unwrap().get(*id) {
            Some(boolean_operator) => boolean_operator.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`BooleanOperator`] from the store.
    ///
    pub fn exorcise_boolean_operator(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<BooleanOperator>>> {
        let result = self.boolean_operator.write().unwrap()[*id].take();
        self.boolean_operator_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanOperator>`.
    ///
    pub fn iter_boolean_operator(&self) -> impl Iterator<Item = Arc<RwLock<BooleanOperator>>> + '_ {
        let len = self.boolean_operator.read().unwrap().len();
        (0..len).map(move |i| {
            self.boolean_operator.read().unwrap()[i]
                .as_ref()
                .map(|boolean_operator| boolean_operator.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Call`] into the store.
    ///
    pub fn inter_call<F>(&mut self, call: F) -> Arc<RwLock<Call>>
    where
        F: Fn(usize) -> Arc<RwLock<Call>>,
    {
        if let Some(_index) = self.call_free_list.lock().unwrap().pop() {
            let call = call(_index);
            self.call.write().unwrap()[_index] = Some(call.clone());
            call
        } else {
            let _index = self.call.read().unwrap().len();
            let call = call(_index);
            self.call.write().unwrap().push(Some(call.clone()));
            call
        }
    }

    /// Exhume (get) [`Call`] from the store.
    ///
    pub fn exhume_call(&self, id: &usize) -> Option<Arc<RwLock<Call>>> {
        match self.call.read().unwrap().get(*id) {
            Some(call) => call.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Call`] from the store.
    ///
    pub fn exorcise_call(&mut self, id: &usize) -> Option<Arc<RwLock<Call>>> {
        let result = self.call.write().unwrap()[*id].take();
        self.call_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Call>`.
    ///
    pub fn iter_call(&self) -> impl Iterator<Item = Arc<RwLock<Call>>> + '_ {
        let len = self.call.read().unwrap().len();
        (0..len).map(move |i| {
            self.call.read().unwrap()[i]
                .as_ref()
                .map(|call| call.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Comparison`] into the store.
    ///
    pub fn inter_comparison<F>(&mut self, comparison: F) -> Arc<RwLock<Comparison>>
    where
        F: Fn(usize) -> Arc<RwLock<Comparison>>,
    {
        if let Some(_index) = self.comparison_free_list.lock().unwrap().pop() {
            let comparison = comparison(_index);
            self.comparison.write().unwrap()[_index] = Some(comparison.clone());
            comparison
        } else {
            let _index = self.comparison.read().unwrap().len();
            let comparison = comparison(_index);
            self.comparison
                .write()
                .unwrap()
                .push(Some(comparison.clone()));
            comparison
        }
    }

    /// Exhume (get) [`Comparison`] from the store.
    ///
    pub fn exhume_comparison(&self, id: &usize) -> Option<Arc<RwLock<Comparison>>> {
        match self.comparison.read().unwrap().get(*id) {
            Some(comparison) => comparison.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Comparison`] from the store.
    ///
    pub fn exorcise_comparison(&mut self, id: &usize) -> Option<Arc<RwLock<Comparison>>> {
        let result = self.comparison.write().unwrap()[*id].take();
        self.comparison_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Comparison>`.
    ///
    pub fn iter_comparison(&self) -> impl Iterator<Item = Arc<RwLock<Comparison>>> + '_ {
        let len = self.comparison.read().unwrap().len();
        (0..len).map(move |i| {
            self.comparison.read().unwrap()[i]
                .as_ref()
                .map(|comparison| comparison.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`DwarfSourceFile`] into the store.
    ///
    pub fn inter_dwarf_source_file<F>(
        &mut self,
        dwarf_source_file: F,
    ) -> Arc<RwLock<DwarfSourceFile>>
    where
        F: Fn(usize) -> Arc<RwLock<DwarfSourceFile>>,
    {
        if let Some(_index) = self.dwarf_source_file_free_list.lock().unwrap().pop() {
            let dwarf_source_file = dwarf_source_file(_index);
            self.dwarf_source_file.write().unwrap()[_index] = Some(dwarf_source_file.clone());
            dwarf_source_file
        } else {
            let _index = self.dwarf_source_file.read().unwrap().len();
            let dwarf_source_file = dwarf_source_file(_index);
            self.dwarf_source_file
                .write()
                .unwrap()
                .push(Some(dwarf_source_file.clone()));
            dwarf_source_file
        }
    }

    /// Exhume (get) [`DwarfSourceFile`] from the store.
    ///
    pub fn exhume_dwarf_source_file(&self, id: &usize) -> Option<Arc<RwLock<DwarfSourceFile>>> {
        match self.dwarf_source_file.read().unwrap().get(*id) {
            Some(dwarf_source_file) => dwarf_source_file.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`DwarfSourceFile`] from the store.
    ///
    pub fn exorcise_dwarf_source_file(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<DwarfSourceFile>>> {
        let result = self.dwarf_source_file.write().unwrap()[*id].take();
        self.dwarf_source_file_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, DwarfSourceFile>`.
    ///
    pub fn iter_dwarf_source_file(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<DwarfSourceFile>>> + '_ {
        let len = self.dwarf_source_file.read().unwrap().len();
        (0..len).map(move |i| {
            self.dwarf_source_file.read().unwrap()[i]
                .as_ref()
                .map(|dwarf_source_file| dwarf_source_file.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Error`] into the store.
    ///
    pub fn inter_error<F>(&mut self, error: F) -> Arc<RwLock<Error>>
    where
        F: Fn(usize) -> Arc<RwLock<Error>>,
    {
        if let Some(_index) = self.error_free_list.lock().unwrap().pop() {
            let error = error(_index);
            self.error.write().unwrap()[_index] = Some(error.clone());
            error
        } else {
            let _index = self.error.read().unwrap().len();
            let error = error(_index);
            self.error.write().unwrap().push(Some(error.clone()));
            error
        }
    }

    /// Exhume (get) [`Error`] from the store.
    ///
    pub fn exhume_error(&self, id: &usize) -> Option<Arc<RwLock<Error>>> {
        match self.error.read().unwrap().get(*id) {
            Some(error) => error.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Error`] from the store.
    ///
    pub fn exorcise_error(&mut self, id: &usize) -> Option<Arc<RwLock<Error>>> {
        let result = self.error.write().unwrap()[*id].take();
        self.error_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Error>`.
    ///
    pub fn iter_error(&self) -> impl Iterator<Item = Arc<RwLock<Error>>> + '_ {
        let len = self.error.read().unwrap().len();
        (0..len).map(move |i| {
            self.error.read().unwrap()[i]
                .as_ref()
                .map(|error| error.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`ErrorExpression`] into the store.
    ///
    pub fn inter_error_expression<F>(&mut self, error_expression: F) -> Arc<RwLock<ErrorExpression>>
    where
        F: Fn(usize) -> Arc<RwLock<ErrorExpression>>,
    {
        if let Some(_index) = self.error_expression_free_list.lock().unwrap().pop() {
            let error_expression = error_expression(_index);
            self.error_expression.write().unwrap()[_index] = Some(error_expression.clone());
            error_expression
        } else {
            let _index = self.error_expression.read().unwrap().len();
            let error_expression = error_expression(_index);
            self.error_expression
                .write()
                .unwrap()
                .push(Some(error_expression.clone()));
            error_expression
        }
    }

    /// Exhume (get) [`ErrorExpression`] from the store.
    ///
    pub fn exhume_error_expression(&self, id: &usize) -> Option<Arc<RwLock<ErrorExpression>>> {
        match self.error_expression.read().unwrap().get(*id) {
            Some(error_expression) => error_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ErrorExpression`] from the store.
    ///
    pub fn exorcise_error_expression(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<ErrorExpression>>> {
        let result = self.error_expression.write().unwrap()[*id].take();
        self.error_expression_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ErrorExpression>`.
    ///
    pub fn iter_error_expression(&self) -> impl Iterator<Item = Arc<RwLock<ErrorExpression>>> + '_ {
        let len = self.error_expression.read().unwrap().len();
        (0..len).map(move |i| {
            self.error_expression.read().unwrap()[i]
                .as_ref()
                .map(|error_expression| error_expression.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Expression`] into the store.
    ///
    pub fn inter_expression<F>(&mut self, expression: F) -> Arc<RwLock<Expression>>
    where
        F: Fn(usize) -> Arc<RwLock<Expression>>,
    {
        if let Some(_index) = self.expression_free_list.lock().unwrap().pop() {
            let expression = expression(_index);
            self.expression.write().unwrap()[_index] = Some(expression.clone());
            expression
        } else {
            let _index = self.expression.read().unwrap().len();
            let expression = expression(_index);
            self.expression
                .write()
                .unwrap()
                .push(Some(expression.clone()));
            expression
        }
    }

    /// Exhume (get) [`Expression`] from the store.
    ///
    pub fn exhume_expression(&self, id: &usize) -> Option<Arc<RwLock<Expression>>> {
        match self.expression.read().unwrap().get(*id) {
            Some(expression) => expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Expression`] from the store.
    ///
    pub fn exorcise_expression(&mut self, id: &usize) -> Option<Arc<RwLock<Expression>>> {
        let result = self.expression.write().unwrap()[*id].take();
        self.expression_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Expression>`.
    ///
    pub fn iter_expression(&self) -> impl Iterator<Item = Arc<RwLock<Expression>>> + '_ {
        let len = self.expression.read().unwrap().len();
        (0..len).map(move |i| {
            self.expression.read().unwrap()[i]
                .as_ref()
                .map(|expression| expression.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`ExpressionStatement`] into the store.
    ///
    pub fn inter_expression_statement<F>(
        &mut self,
        expression_statement: F,
    ) -> Arc<RwLock<ExpressionStatement>>
    where
        F: Fn(usize) -> Arc<RwLock<ExpressionStatement>>,
    {
        if let Some(_index) = self.expression_statement_free_list.lock().unwrap().pop() {
            let expression_statement = expression_statement(_index);
            self.expression_statement.write().unwrap()[_index] = Some(expression_statement.clone());
            expression_statement
        } else {
            let _index = self.expression_statement.read().unwrap().len();
            let expression_statement = expression_statement(_index);
            self.expression_statement
                .write()
                .unwrap()
                .push(Some(expression_statement.clone()));
            expression_statement
        }
    }

    /// Exhume (get) [`ExpressionStatement`] from the store.
    ///
    pub fn exhume_expression_statement(
        &self,
        id: &usize,
    ) -> Option<Arc<RwLock<ExpressionStatement>>> {
        match self.expression_statement.read().unwrap().get(*id) {
            Some(expression_statement) => expression_statement.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ExpressionStatement`] from the store.
    ///
    pub fn exorcise_expression_statement(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<ExpressionStatement>>> {
        let result = self.expression_statement.write().unwrap()[*id].take();
        self.expression_statement_free_list
            .lock()
            .unwrap()
            .push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ExpressionStatement>`.
    ///
    pub fn iter_expression_statement(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<ExpressionStatement>>> + '_ {
        let len = self.expression_statement.read().unwrap().len();
        (0..len).map(move |i| {
            self.expression_statement.read().unwrap()[i]
                .as_ref()
                .map(|expression_statement| expression_statement.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Field`] into the store.
    ///
    pub fn inter_field<F>(&mut self, field: F) -> Arc<RwLock<Field>>
    where
        F: Fn(usize) -> Arc<RwLock<Field>>,
    {
        let field = if let Some(_index) = self.field_free_list.lock().unwrap().pop() {
            let field = field(_index);
            self.field.write().unwrap()[_index] = Some(field.clone());
            field
        } else {
            let _index = self.field.read().unwrap().len();
            let field = field(_index);
            self.field.write().unwrap().push(Some(field.clone()));
            field
        };
        self.field_id_by_name.write().unwrap().insert(
            field.read().unwrap().name.to_upper_camel_case(),
            field.read().unwrap().id,
        );
        field
    }

    /// Exhume (get) [`Field`] from the store.
    ///
    pub fn exhume_field(&self, id: &usize) -> Option<Arc<RwLock<Field>>> {
        match self.field.read().unwrap().get(*id) {
            Some(field) => field.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Field`] from the store.
    ///
    pub fn exorcise_field(&mut self, id: &usize) -> Option<Arc<RwLock<Field>>> {
        let result = self.field.write().unwrap()[*id].take();
        self.field_free_list.lock().unwrap().push(*id);
        result
    }

    /// Exorcise [`Field`] id from the store by name.
    ///
    pub fn exhume_field_id_by_name(&self, name: &str) -> Option<usize> {
        self.field_id_by_name
            .read()
            .unwrap()
            .get(name)
            .map(|field| *field)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Field>`.
    ///
    pub fn iter_field(&self) -> impl Iterator<Item = Arc<RwLock<Field>>> + '_ {
        let len = self.field.read().unwrap().len();
        (0..len).map(move |i| {
            self.field.read().unwrap()[i]
                .as_ref()
                .map(|field| field.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`FieldAccess`] into the store.
    ///
    pub fn inter_field_access<F>(&mut self, field_access: F) -> Arc<RwLock<FieldAccess>>
    where
        F: Fn(usize) -> Arc<RwLock<FieldAccess>>,
    {
        if let Some(_index) = self.field_access_free_list.lock().unwrap().pop() {
            let field_access = field_access(_index);
            self.field_access.write().unwrap()[_index] = Some(field_access.clone());
            field_access
        } else {
            let _index = self.field_access.read().unwrap().len();
            let field_access = field_access(_index);
            self.field_access
                .write()
                .unwrap()
                .push(Some(field_access.clone()));
            field_access
        }
    }

    /// Exhume (get) [`FieldAccess`] from the store.
    ///
    pub fn exhume_field_access(&self, id: &usize) -> Option<Arc<RwLock<FieldAccess>>> {
        match self.field_access.read().unwrap().get(*id) {
            Some(field_access) => field_access.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FieldAccess`] from the store.
    ///
    pub fn exorcise_field_access(&mut self, id: &usize) -> Option<Arc<RwLock<FieldAccess>>> {
        let result = self.field_access.write().unwrap()[*id].take();
        self.field_access_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccess>`.
    ///
    pub fn iter_field_access(&self) -> impl Iterator<Item = Arc<RwLock<FieldAccess>>> + '_ {
        let len = self.field_access.read().unwrap().len();
        (0..len).map(move |i| {
            self.field_access.read().unwrap()[i]
                .as_ref()
                .map(|field_access| field_access.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`FieldAccessTarget`] into the store.
    ///
    pub fn inter_field_access_target<F>(
        &mut self,
        field_access_target: F,
    ) -> Arc<RwLock<FieldAccessTarget>>
    where
        F: Fn(usize) -> Arc<RwLock<FieldAccessTarget>>,
    {
        if let Some(_index) = self.field_access_target_free_list.lock().unwrap().pop() {
            let field_access_target = field_access_target(_index);
            self.field_access_target.write().unwrap()[_index] = Some(field_access_target.clone());
            field_access_target
        } else {
            let _index = self.field_access_target.read().unwrap().len();
            let field_access_target = field_access_target(_index);
            self.field_access_target
                .write()
                .unwrap()
                .push(Some(field_access_target.clone()));
            field_access_target
        }
    }

    /// Exhume (get) [`FieldAccessTarget`] from the store.
    ///
    pub fn exhume_field_access_target(&self, id: &usize) -> Option<Arc<RwLock<FieldAccessTarget>>> {
        match self.field_access_target.read().unwrap().get(*id) {
            Some(field_access_target) => field_access_target.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FieldAccessTarget`] from the store.
    ///
    pub fn exorcise_field_access_target(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<FieldAccessTarget>>> {
        let result = self.field_access_target.write().unwrap()[*id].take();
        self.field_access_target_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccessTarget>`.
    ///
    pub fn iter_field_access_target(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<FieldAccessTarget>>> + '_ {
        let len = self.field_access_target.read().unwrap().len();
        (0..len).map(move |i| {
            self.field_access_target.read().unwrap()[i]
                .as_ref()
                .map(|field_access_target| field_access_target.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`FieldExpression`] into the store.
    ///
    pub fn inter_field_expression<F>(&mut self, field_expression: F) -> Arc<RwLock<FieldExpression>>
    where
        F: Fn(usize) -> Arc<RwLock<FieldExpression>>,
    {
        if let Some(_index) = self.field_expression_free_list.lock().unwrap().pop() {
            let field_expression = field_expression(_index);
            self.field_expression.write().unwrap()[_index] = Some(field_expression.clone());
            field_expression
        } else {
            let _index = self.field_expression.read().unwrap().len();
            let field_expression = field_expression(_index);
            self.field_expression
                .write()
                .unwrap()
                .push(Some(field_expression.clone()));
            field_expression
        }
    }

    /// Exhume (get) [`FieldExpression`] from the store.
    ///
    pub fn exhume_field_expression(&self, id: &usize) -> Option<Arc<RwLock<FieldExpression>>> {
        match self.field_expression.read().unwrap().get(*id) {
            Some(field_expression) => field_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FieldExpression`] from the store.
    ///
    pub fn exorcise_field_expression(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<FieldExpression>>> {
        let result = self.field_expression.write().unwrap()[*id].take();
        self.field_expression_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldExpression>`.
    ///
    pub fn iter_field_expression(&self) -> impl Iterator<Item = Arc<RwLock<FieldExpression>>> + '_ {
        let len = self.field_expression.read().unwrap().len();
        (0..len).map(move |i| {
            self.field_expression.read().unwrap()[i]
                .as_ref()
                .map(|field_expression| field_expression.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`FloatLiteral`] into the store.
    ///
    pub fn inter_float_literal<F>(&mut self, float_literal: F) -> Arc<RwLock<FloatLiteral>>
    where
        F: Fn(usize) -> Arc<RwLock<FloatLiteral>>,
    {
        if let Some(_index) = self.float_literal_free_list.lock().unwrap().pop() {
            let float_literal = float_literal(_index);
            self.float_literal.write().unwrap()[_index] = Some(float_literal.clone());
            float_literal
        } else {
            let _index = self.float_literal.read().unwrap().len();
            let float_literal = float_literal(_index);
            self.float_literal
                .write()
                .unwrap()
                .push(Some(float_literal.clone()));
            float_literal
        }
    }

    /// Exhume (get) [`FloatLiteral`] from the store.
    ///
    pub fn exhume_float_literal(&self, id: &usize) -> Option<Arc<RwLock<FloatLiteral>>> {
        match self.float_literal.read().unwrap().get(*id) {
            Some(float_literal) => float_literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FloatLiteral`] from the store.
    ///
    pub fn exorcise_float_literal(&mut self, id: &usize) -> Option<Arc<RwLock<FloatLiteral>>> {
        let result = self.float_literal.write().unwrap()[*id].take();
        self.float_literal_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FloatLiteral>`.
    ///
    pub fn iter_float_literal(&self) -> impl Iterator<Item = Arc<RwLock<FloatLiteral>>> + '_ {
        let len = self.float_literal.read().unwrap().len();
        (0..len).map(move |i| {
            self.float_literal.read().unwrap()[i]
                .as_ref()
                .map(|float_literal| float_literal.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`ForLoop`] into the store.
    ///
    pub fn inter_for_loop<F>(&mut self, for_loop: F) -> Arc<RwLock<ForLoop>>
    where
        F: Fn(usize) -> Arc<RwLock<ForLoop>>,
    {
        if let Some(_index) = self.for_loop_free_list.lock().unwrap().pop() {
            let for_loop = for_loop(_index);
            self.for_loop.write().unwrap()[_index] = Some(for_loop.clone());
            for_loop
        } else {
            let _index = self.for_loop.read().unwrap().len();
            let for_loop = for_loop(_index);
            self.for_loop.write().unwrap().push(Some(for_loop.clone()));
            for_loop
        }
    }

    /// Exhume (get) [`ForLoop`] from the store.
    ///
    pub fn exhume_for_loop(&self, id: &usize) -> Option<Arc<RwLock<ForLoop>>> {
        match self.for_loop.read().unwrap().get(*id) {
            Some(for_loop) => for_loop.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ForLoop`] from the store.
    ///
    pub fn exorcise_for_loop(&mut self, id: &usize) -> Option<Arc<RwLock<ForLoop>>> {
        let result = self.for_loop.write().unwrap()[*id].take();
        self.for_loop_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ForLoop>`.
    ///
    pub fn iter_for_loop(&self) -> impl Iterator<Item = Arc<RwLock<ForLoop>>> + '_ {
        let len = self.for_loop.read().unwrap().len();
        (0..len).map(move |i| {
            self.for_loop.read().unwrap()[i]
                .as_ref()
                .map(|for_loop| for_loop.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Function`] into the store.
    ///
    pub fn inter_function<F>(&mut self, function: F) -> Arc<RwLock<Function>>
    where
        F: Fn(usize) -> Arc<RwLock<Function>>,
    {
        let function = if let Some(_index) = self.function_free_list.lock().unwrap().pop() {
            let function = function(_index);
            self.function.write().unwrap()[_index] = Some(function.clone());
            function
        } else {
            let _index = self.function.read().unwrap().len();
            let function = function(_index);
            self.function.write().unwrap().push(Some(function.clone()));
            function
        };
        self.function_id_by_name.write().unwrap().insert(
            function.read().unwrap().name.to_upper_camel_case(),
            function.read().unwrap().id,
        );
        function
    }

    /// Exhume (get) [`Function`] from the store.
    ///
    pub fn exhume_function(&self, id: &usize) -> Option<Arc<RwLock<Function>>> {
        match self.function.read().unwrap().get(*id) {
            Some(function) => function.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Function`] from the store.
    ///
    pub fn exorcise_function(&mut self, id: &usize) -> Option<Arc<RwLock<Function>>> {
        let result = self.function.write().unwrap()[*id].take();
        self.function_free_list.lock().unwrap().push(*id);
        result
    }

    /// Exorcise [`Function`] id from the store by name.
    ///
    pub fn exhume_function_id_by_name(&self, name: &str) -> Option<usize> {
        self.function_id_by_name
            .read()
            .unwrap()
            .get(name)
            .map(|function| *function)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Function>`.
    ///
    pub fn iter_function(&self) -> impl Iterator<Item = Arc<RwLock<Function>>> + '_ {
        let len = self.function.read().unwrap().len();
        (0..len).map(move |i| {
            self.function.read().unwrap()[i]
                .as_ref()
                .map(|function| function.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Grouped`] into the store.
    ///
    pub fn inter_grouped<F>(&mut self, grouped: F) -> Arc<RwLock<Grouped>>
    where
        F: Fn(usize) -> Arc<RwLock<Grouped>>,
    {
        if let Some(_index) = self.grouped_free_list.lock().unwrap().pop() {
            let grouped = grouped(_index);
            self.grouped.write().unwrap()[_index] = Some(grouped.clone());
            grouped
        } else {
            let _index = self.grouped.read().unwrap().len();
            let grouped = grouped(_index);
            self.grouped.write().unwrap().push(Some(grouped.clone()));
            grouped
        }
    }

    /// Exhume (get) [`Grouped`] from the store.
    ///
    pub fn exhume_grouped(&self, id: &usize) -> Option<Arc<RwLock<Grouped>>> {
        match self.grouped.read().unwrap().get(*id) {
            Some(grouped) => grouped.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Grouped`] from the store.
    ///
    pub fn exorcise_grouped(&mut self, id: &usize) -> Option<Arc<RwLock<Grouped>>> {
        let result = self.grouped.write().unwrap()[*id].take();
        self.grouped_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Grouped>`.
    ///
    pub fn iter_grouped(&self) -> impl Iterator<Item = Arc<RwLock<Grouped>>> + '_ {
        let len = self.grouped.read().unwrap().len();
        (0..len).map(move |i| {
            self.grouped.read().unwrap()[i]
                .as_ref()
                .map(|grouped| grouped.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`XIf`] into the store.
    ///
    pub fn inter_x_if<F>(&mut self, x_if: F) -> Arc<RwLock<XIf>>
    where
        F: Fn(usize) -> Arc<RwLock<XIf>>,
    {
        if let Some(_index) = self.x_if_free_list.lock().unwrap().pop() {
            let x_if = x_if(_index);
            self.x_if.write().unwrap()[_index] = Some(x_if.clone());
            x_if
        } else {
            let _index = self.x_if.read().unwrap().len();
            let x_if = x_if(_index);
            self.x_if.write().unwrap().push(Some(x_if.clone()));
            x_if
        }
    }

    /// Exhume (get) [`XIf`] from the store.
    ///
    pub fn exhume_x_if(&self, id: &usize) -> Option<Arc<RwLock<XIf>>> {
        match self.x_if.read().unwrap().get(*id) {
            Some(x_if) => x_if.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XIf`] from the store.
    ///
    pub fn exorcise_x_if(&mut self, id: &usize) -> Option<Arc<RwLock<XIf>>> {
        let result = self.x_if.write().unwrap()[*id].take();
        self.x_if_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XIf>`.
    ///
    pub fn iter_x_if(&self) -> impl Iterator<Item = Arc<RwLock<XIf>>> + '_ {
        let len = self.x_if.read().unwrap().len();
        (0..len).map(move |i| {
            self.x_if.read().unwrap()[i]
                .as_ref()
                .map(|x_if| x_if.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Implementation`] into the store.
    ///
    pub fn inter_implementation<F>(&mut self, implementation: F) -> Arc<RwLock<Implementation>>
    where
        F: Fn(usize) -> Arc<RwLock<Implementation>>,
    {
        if let Some(_index) = self.implementation_free_list.lock().unwrap().pop() {
            let implementation = implementation(_index);
            self.implementation.write().unwrap()[_index] = Some(implementation.clone());
            implementation
        } else {
            let _index = self.implementation.read().unwrap().len();
            let implementation = implementation(_index);
            self.implementation
                .write()
                .unwrap()
                .push(Some(implementation.clone()));
            implementation
        }
    }

    /// Exhume (get) [`Implementation`] from the store.
    ///
    pub fn exhume_implementation(&self, id: &usize) -> Option<Arc<RwLock<Implementation>>> {
        match self.implementation.read().unwrap().get(*id) {
            Some(implementation) => implementation.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Implementation`] from the store.
    ///
    pub fn exorcise_implementation(&mut self, id: &usize) -> Option<Arc<RwLock<Implementation>>> {
        let result = self.implementation.write().unwrap()[*id].take();
        self.implementation_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Implementation>`.
    ///
    pub fn iter_implementation(&self) -> impl Iterator<Item = Arc<RwLock<Implementation>>> + '_ {
        let len = self.implementation.read().unwrap().len();
        (0..len).map(move |i| {
            self.implementation.read().unwrap()[i]
                .as_ref()
                .map(|implementation| implementation.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Import`] into the store.
    ///
    pub fn inter_import<F>(&mut self, import: F) -> Arc<RwLock<Import>>
    where
        F: Fn(usize) -> Arc<RwLock<Import>>,
    {
        if let Some(_index) = self.import_free_list.lock().unwrap().pop() {
            let import = import(_index);
            self.import.write().unwrap()[_index] = Some(import.clone());
            import
        } else {
            let _index = self.import.read().unwrap().len();
            let import = import(_index);
            self.import.write().unwrap().push(Some(import.clone()));
            import
        }
    }

    /// Exhume (get) [`Import`] from the store.
    ///
    pub fn exhume_import(&self, id: &usize) -> Option<Arc<RwLock<Import>>> {
        match self.import.read().unwrap().get(*id) {
            Some(import) => import.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Import`] from the store.
    ///
    pub fn exorcise_import(&mut self, id: &usize) -> Option<Arc<RwLock<Import>>> {
        let result = self.import.write().unwrap()[*id].take();
        self.import_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Import>`.
    ///
    pub fn iter_import(&self) -> impl Iterator<Item = Arc<RwLock<Import>>> + '_ {
        let len = self.import.read().unwrap().len();
        (0..len).map(move |i| {
            self.import.read().unwrap()[i]
                .as_ref()
                .map(|import| import.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Index`] into the store.
    ///
    pub fn inter_index<F>(&mut self, index: F) -> Arc<RwLock<Index>>
    where
        F: Fn(usize) -> Arc<RwLock<Index>>,
    {
        if let Some(_index) = self.index_free_list.lock().unwrap().pop() {
            let index = index(_index);
            self.index.write().unwrap()[_index] = Some(index.clone());
            index
        } else {
            let _index = self.index.read().unwrap().len();
            let index = index(_index);
            self.index.write().unwrap().push(Some(index.clone()));
            index
        }
    }

    /// Exhume (get) [`Index`] from the store.
    ///
    pub fn exhume_index(&self, id: &usize) -> Option<Arc<RwLock<Index>>> {
        match self.index.read().unwrap().get(*id) {
            Some(index) => index.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Index`] from the store.
    ///
    pub fn exorcise_index(&mut self, id: &usize) -> Option<Arc<RwLock<Index>>> {
        let result = self.index.write().unwrap()[*id].take();
        self.index_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Index>`.
    ///
    pub fn iter_index(&self) -> impl Iterator<Item = Arc<RwLock<Index>>> + '_ {
        let len = self.index.read().unwrap().len();
        (0..len).map(move |i| {
            self.index.read().unwrap()[i]
                .as_ref()
                .map(|index| index.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`IntegerLiteral`] into the store.
    ///
    pub fn inter_integer_literal<F>(&mut self, integer_literal: F) -> Arc<RwLock<IntegerLiteral>>
    where
        F: Fn(usize) -> Arc<RwLock<IntegerLiteral>>,
    {
        if let Some(_index) = self.integer_literal_free_list.lock().unwrap().pop() {
            let integer_literal = integer_literal(_index);
            self.integer_literal.write().unwrap()[_index] = Some(integer_literal.clone());
            integer_literal
        } else {
            let _index = self.integer_literal.read().unwrap().len();
            let integer_literal = integer_literal(_index);
            self.integer_literal
                .write()
                .unwrap()
                .push(Some(integer_literal.clone()));
            integer_literal
        }
    }

    /// Exhume (get) [`IntegerLiteral`] from the store.
    ///
    pub fn exhume_integer_literal(&self, id: &usize) -> Option<Arc<RwLock<IntegerLiteral>>> {
        match self.integer_literal.read().unwrap().get(*id) {
            Some(integer_literal) => integer_literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`IntegerLiteral`] from the store.
    ///
    pub fn exorcise_integer_literal(&mut self, id: &usize) -> Option<Arc<RwLock<IntegerLiteral>>> {
        let result = self.integer_literal.write().unwrap()[*id].take();
        self.integer_literal_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, IntegerLiteral>`.
    ///
    pub fn iter_integer_literal(&self) -> impl Iterator<Item = Arc<RwLock<IntegerLiteral>>> + '_ {
        let len = self.integer_literal.read().unwrap().len();
        (0..len).map(move |i| {
            self.integer_literal.read().unwrap()[i]
                .as_ref()
                .map(|integer_literal| integer_literal.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Item`] into the store.
    ///
    pub fn inter_item<F>(&mut self, item: F) -> Arc<RwLock<Item>>
    where
        F: Fn(usize) -> Arc<RwLock<Item>>,
    {
        if let Some(_index) = self.item_free_list.lock().unwrap().pop() {
            let item = item(_index);
            self.item.write().unwrap()[_index] = Some(item.clone());
            item
        } else {
            let _index = self.item.read().unwrap().len();
            let item = item(_index);
            self.item.write().unwrap().push(Some(item.clone()));
            item
        }
    }

    /// Exhume (get) [`Item`] from the store.
    ///
    pub fn exhume_item(&self, id: &usize) -> Option<Arc<RwLock<Item>>> {
        match self.item.read().unwrap().get(*id) {
            Some(item) => item.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Item`] from the store.
    ///
    pub fn exorcise_item(&mut self, id: &usize) -> Option<Arc<RwLock<Item>>> {
        let result = self.item.write().unwrap()[*id].take();
        self.item_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Item>`.
    ///
    pub fn iter_item(&self) -> impl Iterator<Item = Arc<RwLock<Item>>> + '_ {
        let len = self.item.read().unwrap().len();
        (0..len).map(move |i| {
            self.item.read().unwrap()[i]
                .as_ref()
                .map(|item| item.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Lambda`] into the store.
    ///
    pub fn inter_lambda<F>(&mut self, lambda: F) -> Arc<RwLock<Lambda>>
    where
        F: Fn(usize) -> Arc<RwLock<Lambda>>,
    {
        if let Some(_index) = self.lambda_free_list.lock().unwrap().pop() {
            let lambda = lambda(_index);
            self.lambda.write().unwrap()[_index] = Some(lambda.clone());
            lambda
        } else {
            let _index = self.lambda.read().unwrap().len();
            let lambda = lambda(_index);
            self.lambda.write().unwrap().push(Some(lambda.clone()));
            lambda
        }
    }

    /// Exhume (get) [`Lambda`] from the store.
    ///
    pub fn exhume_lambda(&self, id: &usize) -> Option<Arc<RwLock<Lambda>>> {
        match self.lambda.read().unwrap().get(*id) {
            Some(lambda) => lambda.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Lambda`] from the store.
    ///
    pub fn exorcise_lambda(&mut self, id: &usize) -> Option<Arc<RwLock<Lambda>>> {
        let result = self.lambda.write().unwrap()[*id].take();
        self.lambda_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Lambda>`.
    ///
    pub fn iter_lambda(&self) -> impl Iterator<Item = Arc<RwLock<Lambda>>> + '_ {
        let len = self.lambda.read().unwrap().len();
        (0..len).map(move |i| {
            self.lambda.read().unwrap()[i]
                .as_ref()
                .map(|lambda| lambda.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`LambdaParameter`] into the store.
    ///
    pub fn inter_lambda_parameter<F>(&mut self, lambda_parameter: F) -> Arc<RwLock<LambdaParameter>>
    where
        F: Fn(usize) -> Arc<RwLock<LambdaParameter>>,
    {
        if let Some(_index) = self.lambda_parameter_free_list.lock().unwrap().pop() {
            let lambda_parameter = lambda_parameter(_index);
            self.lambda_parameter.write().unwrap()[_index] = Some(lambda_parameter.clone());
            lambda_parameter
        } else {
            let _index = self.lambda_parameter.read().unwrap().len();
            let lambda_parameter = lambda_parameter(_index);
            self.lambda_parameter
                .write()
                .unwrap()
                .push(Some(lambda_parameter.clone()));
            lambda_parameter
        }
    }

    /// Exhume (get) [`LambdaParameter`] from the store.
    ///
    pub fn exhume_lambda_parameter(&self, id: &usize) -> Option<Arc<RwLock<LambdaParameter>>> {
        match self.lambda_parameter.read().unwrap().get(*id) {
            Some(lambda_parameter) => lambda_parameter.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`LambdaParameter`] from the store.
    ///
    pub fn exorcise_lambda_parameter(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<LambdaParameter>>> {
        let result = self.lambda_parameter.write().unwrap()[*id].take();
        self.lambda_parameter_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LambdaParameter>`.
    ///
    pub fn iter_lambda_parameter(&self) -> impl Iterator<Item = Arc<RwLock<LambdaParameter>>> + '_ {
        let len = self.lambda_parameter.read().unwrap().len();
        (0..len).map(move |i| {
            self.lambda_parameter.read().unwrap()[i]
                .as_ref()
                .map(|lambda_parameter| lambda_parameter.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`LetStatement`] into the store.
    ///
    pub fn inter_let_statement<F>(&mut self, let_statement: F) -> Arc<RwLock<LetStatement>>
    where
        F: Fn(usize) -> Arc<RwLock<LetStatement>>,
    {
        if let Some(_index) = self.let_statement_free_list.lock().unwrap().pop() {
            let let_statement = let_statement(_index);
            self.let_statement.write().unwrap()[_index] = Some(let_statement.clone());
            let_statement
        } else {
            let _index = self.let_statement.read().unwrap().len();
            let let_statement = let_statement(_index);
            self.let_statement
                .write()
                .unwrap()
                .push(Some(let_statement.clone()));
            let_statement
        }
    }

    /// Exhume (get) [`LetStatement`] from the store.
    ///
    pub fn exhume_let_statement(&self, id: &usize) -> Option<Arc<RwLock<LetStatement>>> {
        match self.let_statement.read().unwrap().get(*id) {
            Some(let_statement) => let_statement.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`LetStatement`] from the store.
    ///
    pub fn exorcise_let_statement(&mut self, id: &usize) -> Option<Arc<RwLock<LetStatement>>> {
        let result = self.let_statement.write().unwrap()[*id].take();
        self.let_statement_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LetStatement>`.
    ///
    pub fn iter_let_statement(&self) -> impl Iterator<Item = Arc<RwLock<LetStatement>>> + '_ {
        let len = self.let_statement.read().unwrap().len();
        (0..len).map(move |i| {
            self.let_statement.read().unwrap()[i]
                .as_ref()
                .map(|let_statement| let_statement.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`List`] into the store.
    ///
    pub fn inter_list<F>(&mut self, list: F) -> Arc<RwLock<List>>
    where
        F: Fn(usize) -> Arc<RwLock<List>>,
    {
        if let Some(_index) = self.list_free_list.lock().unwrap().pop() {
            let list = list(_index);
            self.list.write().unwrap()[_index] = Some(list.clone());
            list
        } else {
            let _index = self.list.read().unwrap().len();
            let list = list(_index);
            self.list.write().unwrap().push(Some(list.clone()));
            list
        }
    }

    /// Exhume (get) [`List`] from the store.
    ///
    pub fn exhume_list(&self, id: &usize) -> Option<Arc<RwLock<List>>> {
        match self.list.read().unwrap().get(*id) {
            Some(list) => list.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`List`] from the store.
    ///
    pub fn exorcise_list(&mut self, id: &usize) -> Option<Arc<RwLock<List>>> {
        let result = self.list.write().unwrap()[*id].take();
        self.list_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, List>`.
    ///
    pub fn iter_list(&self) -> impl Iterator<Item = Arc<RwLock<List>>> + '_ {
        let len = self.list.read().unwrap().len();
        (0..len).map(move |i| {
            self.list.read().unwrap()[i]
                .as_ref()
                .map(|list| list.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`ListElement`] into the store.
    ///
    pub fn inter_list_element<F>(&mut self, list_element: F) -> Arc<RwLock<ListElement>>
    where
        F: Fn(usize) -> Arc<RwLock<ListElement>>,
    {
        if let Some(_index) = self.list_element_free_list.lock().unwrap().pop() {
            let list_element = list_element(_index);
            self.list_element.write().unwrap()[_index] = Some(list_element.clone());
            list_element
        } else {
            let _index = self.list_element.read().unwrap().len();
            let list_element = list_element(_index);
            self.list_element
                .write()
                .unwrap()
                .push(Some(list_element.clone()));
            list_element
        }
    }

    /// Exhume (get) [`ListElement`] from the store.
    ///
    pub fn exhume_list_element(&self, id: &usize) -> Option<Arc<RwLock<ListElement>>> {
        match self.list_element.read().unwrap().get(*id) {
            Some(list_element) => list_element.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ListElement`] from the store.
    ///
    pub fn exorcise_list_element(&mut self, id: &usize) -> Option<Arc<RwLock<ListElement>>> {
        let result = self.list_element.write().unwrap()[*id].take();
        self.list_element_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListElement>`.
    ///
    pub fn iter_list_element(&self) -> impl Iterator<Item = Arc<RwLock<ListElement>>> + '_ {
        let len = self.list_element.read().unwrap().len();
        (0..len).map(move |i| {
            self.list_element.read().unwrap()[i]
                .as_ref()
                .map(|list_element| list_element.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`ListExpression`] into the store.
    ///
    pub fn inter_list_expression<F>(&mut self, list_expression: F) -> Arc<RwLock<ListExpression>>
    where
        F: Fn(usize) -> Arc<RwLock<ListExpression>>,
    {
        if let Some(_index) = self.list_expression_free_list.lock().unwrap().pop() {
            let list_expression = list_expression(_index);
            self.list_expression.write().unwrap()[_index] = Some(list_expression.clone());
            list_expression
        } else {
            let _index = self.list_expression.read().unwrap().len();
            let list_expression = list_expression(_index);
            self.list_expression
                .write()
                .unwrap()
                .push(Some(list_expression.clone()));
            list_expression
        }
    }

    /// Exhume (get) [`ListExpression`] from the store.
    ///
    pub fn exhume_list_expression(&self, id: &usize) -> Option<Arc<RwLock<ListExpression>>> {
        match self.list_expression.read().unwrap().get(*id) {
            Some(list_expression) => list_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ListExpression`] from the store.
    ///
    pub fn exorcise_list_expression(&mut self, id: &usize) -> Option<Arc<RwLock<ListExpression>>> {
        let result = self.list_expression.write().unwrap()[*id].take();
        self.list_expression_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListExpression>`.
    ///
    pub fn iter_list_expression(&self) -> impl Iterator<Item = Arc<RwLock<ListExpression>>> + '_ {
        let len = self.list_expression.read().unwrap().len();
        (0..len).map(move |i| {
            self.list_expression.read().unwrap()[i]
                .as_ref()
                .map(|list_expression| list_expression.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Literal`] into the store.
    ///
    pub fn inter_literal<F>(&mut self, literal: F) -> Arc<RwLock<Literal>>
    where
        F: Fn(usize) -> Arc<RwLock<Literal>>,
    {
        if let Some(_index) = self.literal_free_list.lock().unwrap().pop() {
            let literal = literal(_index);
            self.literal.write().unwrap()[_index] = Some(literal.clone());
            literal
        } else {
            let _index = self.literal.read().unwrap().len();
            let literal = literal(_index);
            self.literal.write().unwrap().push(Some(literal.clone()));
            literal
        }
    }

    /// Exhume (get) [`Literal`] from the store.
    ///
    pub fn exhume_literal(&self, id: &usize) -> Option<Arc<RwLock<Literal>>> {
        match self.literal.read().unwrap().get(*id) {
            Some(literal) => literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Literal`] from the store.
    ///
    pub fn exorcise_literal(&mut self, id: &usize) -> Option<Arc<RwLock<Literal>>> {
        let result = self.literal.write().unwrap()[*id].take();
        self.literal_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Literal>`.
    ///
    pub fn iter_literal(&self) -> impl Iterator<Item = Arc<RwLock<Literal>>> + '_ {
        let len = self.literal.read().unwrap().len();
        (0..len).map(move |i| {
            self.literal.read().unwrap()[i]
                .as_ref()
                .map(|literal| literal.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`LocalVariable`] into the store.
    ///
    pub fn inter_local_variable<F>(&mut self, local_variable: F) -> Arc<RwLock<LocalVariable>>
    where
        F: Fn(usize) -> Arc<RwLock<LocalVariable>>,
    {
        if let Some(_index) = self.local_variable_free_list.lock().unwrap().pop() {
            let local_variable = local_variable(_index);
            self.local_variable.write().unwrap()[_index] = Some(local_variable.clone());
            local_variable
        } else {
            let _index = self.local_variable.read().unwrap().len();
            let local_variable = local_variable(_index);
            self.local_variable
                .write()
                .unwrap()
                .push(Some(local_variable.clone()));
            local_variable
        }
    }

    /// Exhume (get) [`LocalVariable`] from the store.
    ///
    pub fn exhume_local_variable(&self, id: &usize) -> Option<Arc<RwLock<LocalVariable>>> {
        match self.local_variable.read().unwrap().get(*id) {
            Some(local_variable) => local_variable.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`LocalVariable`] from the store.
    ///
    pub fn exorcise_local_variable(&mut self, id: &usize) -> Option<Arc<RwLock<LocalVariable>>> {
        let result = self.local_variable.write().unwrap()[*id].take();
        self.local_variable_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LocalVariable>`.
    ///
    pub fn iter_local_variable(&self) -> impl Iterator<Item = Arc<RwLock<LocalVariable>>> + '_ {
        let len = self.local_variable.read().unwrap().len();
        (0..len).map(move |i| {
            self.local_variable.read().unwrap()[i]
                .as_ref()
                .map(|local_variable| local_variable.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`XMacro`] into the store.
    ///
    pub fn inter_x_macro<F>(&mut self, x_macro: F) -> Arc<RwLock<XMacro>>
    where
        F: Fn(usize) -> Arc<RwLock<XMacro>>,
    {
        if let Some(_index) = self.x_macro_free_list.lock().unwrap().pop() {
            let x_macro = x_macro(_index);
            self.x_macro.write().unwrap()[_index] = Some(x_macro.clone());
            x_macro
        } else {
            let _index = self.x_macro.read().unwrap().len();
            let x_macro = x_macro(_index);
            self.x_macro.write().unwrap().push(Some(x_macro.clone()));
            x_macro
        }
    }

    /// Exhume (get) [`XMacro`] from the store.
    ///
    pub fn exhume_x_macro(&self, id: &usize) -> Option<Arc<RwLock<XMacro>>> {
        match self.x_macro.read().unwrap().get(*id) {
            Some(x_macro) => x_macro.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XMacro`] from the store.
    ///
    pub fn exorcise_x_macro(&mut self, id: &usize) -> Option<Arc<RwLock<XMacro>>> {
        let result = self.x_macro.write().unwrap()[*id].take();
        self.x_macro_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XMacro>`.
    ///
    pub fn iter_x_macro(&self) -> impl Iterator<Item = Arc<RwLock<XMacro>>> + '_ {
        let len = self.x_macro.read().unwrap().len();
        (0..len).map(move |i| {
            self.x_macro.read().unwrap()[i]
                .as_ref()
                .map(|x_macro| x_macro.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`MethodCall`] into the store.
    ///
    pub fn inter_method_call<F>(&mut self, method_call: F) -> Arc<RwLock<MethodCall>>
    where
        F: Fn(usize) -> Arc<RwLock<MethodCall>>,
    {
        if let Some(_index) = self.method_call_free_list.lock().unwrap().pop() {
            let method_call = method_call(_index);
            self.method_call.write().unwrap()[_index] = Some(method_call.clone());
            method_call
        } else {
            let _index = self.method_call.read().unwrap().len();
            let method_call = method_call(_index);
            self.method_call
                .write()
                .unwrap()
                .push(Some(method_call.clone()));
            method_call
        }
    }

    /// Exhume (get) [`MethodCall`] from the store.
    ///
    pub fn exhume_method_call(&self, id: &usize) -> Option<Arc<RwLock<MethodCall>>> {
        match self.method_call.read().unwrap().get(*id) {
            Some(method_call) => method_call.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`MethodCall`] from the store.
    ///
    pub fn exorcise_method_call(&mut self, id: &usize) -> Option<Arc<RwLock<MethodCall>>> {
        let result = self.method_call.write().unwrap()[*id].take();
        self.method_call_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, MethodCall>`.
    ///
    pub fn iter_method_call(&self) -> impl Iterator<Item = Arc<RwLock<MethodCall>>> + '_ {
        let len = self.method_call.read().unwrap().len();
        (0..len).map(move |i| {
            self.method_call.read().unwrap()[i]
                .as_ref()
                .map(|method_call| method_call.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`ZObjectStore`] into the store.
    ///
    pub fn inter_z_object_store<F>(&mut self, z_object_store: F) -> Arc<RwLock<ZObjectStore>>
    where
        F: Fn(usize) -> Arc<RwLock<ZObjectStore>>,
    {
        if let Some(_index) = self.z_object_store_free_list.lock().unwrap().pop() {
            let z_object_store = z_object_store(_index);
            self.z_object_store.write().unwrap()[_index] = Some(z_object_store.clone());
            z_object_store
        } else {
            let _index = self.z_object_store.read().unwrap().len();
            let z_object_store = z_object_store(_index);
            self.z_object_store
                .write()
                .unwrap()
                .push(Some(z_object_store.clone()));
            z_object_store
        }
    }

    /// Exhume (get) [`ZObjectStore`] from the store.
    ///
    pub fn exhume_z_object_store(&self, id: &usize) -> Option<Arc<RwLock<ZObjectStore>>> {
        match self.z_object_store.read().unwrap().get(*id) {
            Some(z_object_store) => z_object_store.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ZObjectStore`] from the store.
    ///
    pub fn exorcise_z_object_store(&mut self, id: &usize) -> Option<Arc<RwLock<ZObjectStore>>> {
        let result = self.z_object_store.write().unwrap()[*id].take();
        self.z_object_store_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ZObjectStore>`.
    ///
    pub fn iter_z_object_store(&self) -> impl Iterator<Item = Arc<RwLock<ZObjectStore>>> + '_ {
        let len = self.z_object_store.read().unwrap().len();
        (0..len).map(move |i| {
            self.z_object_store.read().unwrap()[i]
                .as_ref()
                .map(|z_object_store| z_object_store.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Operator`] into the store.
    ///
    pub fn inter_operator<F>(&mut self, operator: F) -> Arc<RwLock<Operator>>
    where
        F: Fn(usize) -> Arc<RwLock<Operator>>,
    {
        if let Some(_index) = self.operator_free_list.lock().unwrap().pop() {
            let operator = operator(_index);
            self.operator.write().unwrap()[_index] = Some(operator.clone());
            operator
        } else {
            let _index = self.operator.read().unwrap().len();
            let operator = operator(_index);
            self.operator.write().unwrap().push(Some(operator.clone()));
            operator
        }
    }

    /// Exhume (get) [`Operator`] from the store.
    ///
    pub fn exhume_operator(&self, id: &usize) -> Option<Arc<RwLock<Operator>>> {
        match self.operator.read().unwrap().get(*id) {
            Some(operator) => operator.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Operator`] from the store.
    ///
    pub fn exorcise_operator(&mut self, id: &usize) -> Option<Arc<RwLock<Operator>>> {
        let result = self.operator.write().unwrap()[*id].take();
        self.operator_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Operator>`.
    ///
    pub fn iter_operator(&self) -> impl Iterator<Item = Arc<RwLock<Operator>>> + '_ {
        let len = self.operator.read().unwrap().len();
        (0..len).map(move |i| {
            self.operator.read().unwrap()[i]
                .as_ref()
                .map(|operator| operator.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`WoogOption`] into the store.
    ///
    pub fn inter_woog_option<F>(&mut self, woog_option: F) -> Arc<RwLock<WoogOption>>
    where
        F: Fn(usize) -> Arc<RwLock<WoogOption>>,
    {
        if let Some(_index) = self.woog_option_free_list.lock().unwrap().pop() {
            let woog_option = woog_option(_index);
            self.woog_option.write().unwrap()[_index] = Some(woog_option.clone());
            woog_option
        } else {
            let _index = self.woog_option.read().unwrap().len();
            let woog_option = woog_option(_index);
            self.woog_option
                .write()
                .unwrap()
                .push(Some(woog_option.clone()));
            woog_option
        }
    }

    /// Exhume (get) [`WoogOption`] from the store.
    ///
    pub fn exhume_woog_option(&self, id: &usize) -> Option<Arc<RwLock<WoogOption>>> {
        match self.woog_option.read().unwrap().get(*id) {
            Some(woog_option) => woog_option.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`WoogOption`] from the store.
    ///
    pub fn exorcise_woog_option(&mut self, id: &usize) -> Option<Arc<RwLock<WoogOption>>> {
        let result = self.woog_option.write().unwrap()[*id].take();
        self.woog_option_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, WoogOption>`.
    ///
    pub fn iter_woog_option(&self) -> impl Iterator<Item = Arc<RwLock<WoogOption>>> + '_ {
        let len = self.woog_option.read().unwrap().len();
        (0..len).map(move |i| {
            self.woog_option.read().unwrap()[i]
                .as_ref()
                .map(|woog_option| woog_option.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Parameter`] into the store.
    ///
    pub fn inter_parameter<F>(&mut self, parameter: F) -> Arc<RwLock<Parameter>>
    where
        F: Fn(usize) -> Arc<RwLock<Parameter>>,
    {
        if let Some(_index) = self.parameter_free_list.lock().unwrap().pop() {
            let parameter = parameter(_index);
            self.parameter.write().unwrap()[_index] = Some(parameter.clone());
            parameter
        } else {
            let _index = self.parameter.read().unwrap().len();
            let parameter = parameter(_index);
            self.parameter
                .write()
                .unwrap()
                .push(Some(parameter.clone()));
            parameter
        }
    }

    /// Exhume (get) [`Parameter`] from the store.
    ///
    pub fn exhume_parameter(&self, id: &usize) -> Option<Arc<RwLock<Parameter>>> {
        match self.parameter.read().unwrap().get(*id) {
            Some(parameter) => parameter.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Parameter`] from the store.
    ///
    pub fn exorcise_parameter(&mut self, id: &usize) -> Option<Arc<RwLock<Parameter>>> {
        let result = self.parameter.write().unwrap()[*id].take();
        self.parameter_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Parameter>`.
    ///
    pub fn iter_parameter(&self) -> impl Iterator<Item = Arc<RwLock<Parameter>>> + '_ {
        let len = self.parameter.read().unwrap().len();
        (0..len).map(move |i| {
            self.parameter.read().unwrap()[i]
                .as_ref()
                .map(|parameter| parameter.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Print`] into the store.
    ///
    pub fn inter_print<F>(&mut self, print: F) -> Arc<RwLock<Print>>
    where
        F: Fn(usize) -> Arc<RwLock<Print>>,
    {
        if let Some(_index) = self.print_free_list.lock().unwrap().pop() {
            let print = print(_index);
            self.print.write().unwrap()[_index] = Some(print.clone());
            print
        } else {
            let _index = self.print.read().unwrap().len();
            let print = print(_index);
            self.print.write().unwrap().push(Some(print.clone()));
            print
        }
    }

    /// Exhume (get) [`Print`] from the store.
    ///
    pub fn exhume_print(&self, id: &usize) -> Option<Arc<RwLock<Print>>> {
        match self.print.read().unwrap().get(*id) {
            Some(print) => print.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Print`] from the store.
    ///
    pub fn exorcise_print(&mut self, id: &usize) -> Option<Arc<RwLock<Print>>> {
        let result = self.print.write().unwrap()[*id].take();
        self.print_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Print>`.
    ///
    pub fn iter_print(&self) -> impl Iterator<Item = Arc<RwLock<Print>>> + '_ {
        let len = self.print.read().unwrap().len();
        (0..len).map(move |i| {
            self.print.read().unwrap()[i]
                .as_ref()
                .map(|print| print.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`RangeExpression`] into the store.
    ///
    pub fn inter_range_expression<F>(&mut self, range_expression: F) -> Arc<RwLock<RangeExpression>>
    where
        F: Fn(usize) -> Arc<RwLock<RangeExpression>>,
    {
        if let Some(_index) = self.range_expression_free_list.lock().unwrap().pop() {
            let range_expression = range_expression(_index);
            self.range_expression.write().unwrap()[_index] = Some(range_expression.clone());
            range_expression
        } else {
            let _index = self.range_expression.read().unwrap().len();
            let range_expression = range_expression(_index);
            self.range_expression
                .write()
                .unwrap()
                .push(Some(range_expression.clone()));
            range_expression
        }
    }

    /// Exhume (get) [`RangeExpression`] from the store.
    ///
    pub fn exhume_range_expression(&self, id: &usize) -> Option<Arc<RwLock<RangeExpression>>> {
        match self.range_expression.read().unwrap().get(*id) {
            Some(range_expression) => range_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`RangeExpression`] from the store.
    ///
    pub fn exorcise_range_expression(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<RangeExpression>>> {
        let result = self.range_expression.write().unwrap()[*id].take();
        self.range_expression_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RangeExpression>`.
    ///
    pub fn iter_range_expression(&self) -> impl Iterator<Item = Arc<RwLock<RangeExpression>>> + '_ {
        let len = self.range_expression.read().unwrap().len();
        (0..len).map(move |i| {
            self.range_expression.read().unwrap()[i]
                .as_ref()
                .map(|range_expression| range_expression.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Reference`] into the store.
    ///
    pub fn inter_reference<F>(&mut self, reference: F) -> Arc<RwLock<Reference>>
    where
        F: Fn(usize) -> Arc<RwLock<Reference>>,
    {
        if let Some(_index) = self.reference_free_list.lock().unwrap().pop() {
            let reference = reference(_index);
            self.reference.write().unwrap()[_index] = Some(reference.clone());
            reference
        } else {
            let _index = self.reference.read().unwrap().len();
            let reference = reference(_index);
            self.reference
                .write()
                .unwrap()
                .push(Some(reference.clone()));
            reference
        }
    }

    /// Exhume (get) [`Reference`] from the store.
    ///
    pub fn exhume_reference(&self, id: &usize) -> Option<Arc<RwLock<Reference>>> {
        match self.reference.read().unwrap().get(*id) {
            Some(reference) => reference.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Reference`] from the store.
    ///
    pub fn exorcise_reference(&mut self, id: &usize) -> Option<Arc<RwLock<Reference>>> {
        let result = self.reference.write().unwrap()[*id].take();
        self.reference_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Reference>`.
    ///
    pub fn iter_reference(&self) -> impl Iterator<Item = Arc<RwLock<Reference>>> + '_ {
        let len = self.reference.read().unwrap().len();
        (0..len).map(move |i| {
            self.reference.read().unwrap()[i]
                .as_ref()
                .map(|reference| reference.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`ResultStatement`] into the store.
    ///
    pub fn inter_result_statement<F>(&mut self, result_statement: F) -> Arc<RwLock<ResultStatement>>
    where
        F: Fn(usize) -> Arc<RwLock<ResultStatement>>,
    {
        if let Some(_index) = self.result_statement_free_list.lock().unwrap().pop() {
            let result_statement = result_statement(_index);
            self.result_statement.write().unwrap()[_index] = Some(result_statement.clone());
            result_statement
        } else {
            let _index = self.result_statement.read().unwrap().len();
            let result_statement = result_statement(_index);
            self.result_statement
                .write()
                .unwrap()
                .push(Some(result_statement.clone()));
            result_statement
        }
    }

    /// Exhume (get) [`ResultStatement`] from the store.
    ///
    pub fn exhume_result_statement(&self, id: &usize) -> Option<Arc<RwLock<ResultStatement>>> {
        match self.result_statement.read().unwrap().get(*id) {
            Some(result_statement) => result_statement.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ResultStatement`] from the store.
    ///
    pub fn exorcise_result_statement(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<ResultStatement>>> {
        let result = self.result_statement.write().unwrap()[*id].take();
        self.result_statement_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ResultStatement>`.
    ///
    pub fn iter_result_statement(&self) -> impl Iterator<Item = Arc<RwLock<ResultStatement>>> + '_ {
        let len = self.result_statement.read().unwrap().len();
        (0..len).map(move |i| {
            self.result_statement.read().unwrap()[i]
                .as_ref()
                .map(|result_statement| result_statement.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`XReturn`] into the store.
    ///
    pub fn inter_x_return<F>(&mut self, x_return: F) -> Arc<RwLock<XReturn>>
    where
        F: Fn(usize) -> Arc<RwLock<XReturn>>,
    {
        if let Some(_index) = self.x_return_free_list.lock().unwrap().pop() {
            let x_return = x_return(_index);
            self.x_return.write().unwrap()[_index] = Some(x_return.clone());
            x_return
        } else {
            let _index = self.x_return.read().unwrap().len();
            let x_return = x_return(_index);
            self.x_return.write().unwrap().push(Some(x_return.clone()));
            x_return
        }
    }

    /// Exhume (get) [`XReturn`] from the store.
    ///
    pub fn exhume_x_return(&self, id: &usize) -> Option<Arc<RwLock<XReturn>>> {
        match self.x_return.read().unwrap().get(*id) {
            Some(x_return) => x_return.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XReturn`] from the store.
    ///
    pub fn exorcise_x_return(&mut self, id: &usize) -> Option<Arc<RwLock<XReturn>>> {
        let result = self.x_return.write().unwrap()[*id].take();
        self.x_return_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XReturn>`.
    ///
    pub fn iter_x_return(&self) -> impl Iterator<Item = Arc<RwLock<XReturn>>> + '_ {
        let len = self.x_return.read().unwrap().len();
        (0..len).map(move |i| {
            self.x_return.read().unwrap()[i]
                .as_ref()
                .map(|x_return| x_return.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`ZSome`] into the store.
    ///
    pub fn inter_z_some<F>(&mut self, z_some: F) -> Arc<RwLock<ZSome>>
    where
        F: Fn(usize) -> Arc<RwLock<ZSome>>,
    {
        if let Some(_index) = self.z_some_free_list.lock().unwrap().pop() {
            let z_some = z_some(_index);
            self.z_some.write().unwrap()[_index] = Some(z_some.clone());
            z_some
        } else {
            let _index = self.z_some.read().unwrap().len();
            let z_some = z_some(_index);
            self.z_some.write().unwrap().push(Some(z_some.clone()));
            z_some
        }
    }

    /// Exhume (get) [`ZSome`] from the store.
    ///
    pub fn exhume_z_some(&self, id: &usize) -> Option<Arc<RwLock<ZSome>>> {
        match self.z_some.read().unwrap().get(*id) {
            Some(z_some) => z_some.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ZSome`] from the store.
    ///
    pub fn exorcise_z_some(&mut self, id: &usize) -> Option<Arc<RwLock<ZSome>>> {
        let result = self.z_some.write().unwrap()[*id].take();
        self.z_some_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ZSome>`.
    ///
    pub fn iter_z_some(&self) -> impl Iterator<Item = Arc<RwLock<ZSome>>> + '_ {
        let len = self.z_some.read().unwrap().len();
        (0..len).map(move |i| {
            self.z_some.read().unwrap()[i]
                .as_ref()
                .map(|z_some| z_some.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Span`] into the store.
    ///
    pub fn inter_span<F>(&mut self, span: F) -> Arc<RwLock<Span>>
    where
        F: Fn(usize) -> Arc<RwLock<Span>>,
    {
        if let Some(_index) = self.span_free_list.lock().unwrap().pop() {
            let span = span(_index);
            self.span.write().unwrap()[_index] = Some(span.clone());
            span
        } else {
            let _index = self.span.read().unwrap().len();
            let span = span(_index);
            self.span.write().unwrap().push(Some(span.clone()));
            span
        }
    }

    /// Exhume (get) [`Span`] from the store.
    ///
    pub fn exhume_span(&self, id: &usize) -> Option<Arc<RwLock<Span>>> {
        match self.span.read().unwrap().get(*id) {
            Some(span) => span.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Span`] from the store.
    ///
    pub fn exorcise_span(&mut self, id: &usize) -> Option<Arc<RwLock<Span>>> {
        let result = self.span.write().unwrap()[*id].take();
        self.span_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Span>`.
    ///
    pub fn iter_span(&self) -> impl Iterator<Item = Arc<RwLock<Span>>> + '_ {
        let len = self.span.read().unwrap().len();
        (0..len).map(move |i| {
            self.span.read().unwrap()[i]
                .as_ref()
                .map(|span| span.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Statement`] into the store.
    ///
    pub fn inter_statement<F>(&mut self, statement: F) -> Arc<RwLock<Statement>>
    where
        F: Fn(usize) -> Arc<RwLock<Statement>>,
    {
        if let Some(_index) = self.statement_free_list.lock().unwrap().pop() {
            let statement = statement(_index);
            self.statement.write().unwrap()[_index] = Some(statement.clone());
            statement
        } else {
            let _index = self.statement.read().unwrap().len();
            let statement = statement(_index);
            self.statement
                .write()
                .unwrap()
                .push(Some(statement.clone()));
            statement
        }
    }

    /// Exhume (get) [`Statement`] from the store.
    ///
    pub fn exhume_statement(&self, id: &usize) -> Option<Arc<RwLock<Statement>>> {
        match self.statement.read().unwrap().get(*id) {
            Some(statement) => statement.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Statement`] from the store.
    ///
    pub fn exorcise_statement(&mut self, id: &usize) -> Option<Arc<RwLock<Statement>>> {
        let result = self.statement.write().unwrap()[*id].take();
        self.statement_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Statement>`.
    ///
    pub fn iter_statement(&self) -> impl Iterator<Item = Arc<RwLock<Statement>>> + '_ {
        let len = self.statement.read().unwrap().len();
        (0..len).map(move |i| {
            self.statement.read().unwrap()[i]
                .as_ref()
                .map(|statement| statement.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`StaticMethodCall`] into the store.
    ///
    pub fn inter_static_method_call<F>(
        &mut self,
        static_method_call: F,
    ) -> Arc<RwLock<StaticMethodCall>>
    where
        F: Fn(usize) -> Arc<RwLock<StaticMethodCall>>,
    {
        if let Some(_index) = self.static_method_call_free_list.lock().unwrap().pop() {
            let static_method_call = static_method_call(_index);
            self.static_method_call.write().unwrap()[_index] = Some(static_method_call.clone());
            static_method_call
        } else {
            let _index = self.static_method_call.read().unwrap().len();
            let static_method_call = static_method_call(_index);
            self.static_method_call
                .write()
                .unwrap()
                .push(Some(static_method_call.clone()));
            static_method_call
        }
    }

    /// Exhume (get) [`StaticMethodCall`] from the store.
    ///
    pub fn exhume_static_method_call(&self, id: &usize) -> Option<Arc<RwLock<StaticMethodCall>>> {
        match self.static_method_call.read().unwrap().get(*id) {
            Some(static_method_call) => static_method_call.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`StaticMethodCall`] from the store.
    ///
    pub fn exorcise_static_method_call(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<StaticMethodCall>>> {
        let result = self.static_method_call.write().unwrap()[*id].take();
        self.static_method_call_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StaticMethodCall>`.
    ///
    pub fn iter_static_method_call(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<StaticMethodCall>>> + '_ {
        let len = self.static_method_call.read().unwrap().len();
        (0..len).map(move |i| {
            self.static_method_call.read().unwrap()[i]
                .as_ref()
                .map(|static_method_call| static_method_call.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`StringLiteral`] into the store.
    ///
    pub fn inter_string_literal<F>(&mut self, string_literal: F) -> Arc<RwLock<StringLiteral>>
    where
        F: Fn(usize) -> Arc<RwLock<StringLiteral>>,
    {
        if let Some(_index) = self.string_literal_free_list.lock().unwrap().pop() {
            let string_literal = string_literal(_index);
            self.string_literal.write().unwrap()[_index] = Some(string_literal.clone());
            string_literal
        } else {
            let _index = self.string_literal.read().unwrap().len();
            let string_literal = string_literal(_index);
            self.string_literal
                .write()
                .unwrap()
                .push(Some(string_literal.clone()));
            string_literal
        }
    }

    /// Exhume (get) [`StringLiteral`] from the store.
    ///
    pub fn exhume_string_literal(&self, id: &usize) -> Option<Arc<RwLock<StringLiteral>>> {
        match self.string_literal.read().unwrap().get(*id) {
            Some(string_literal) => string_literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`StringLiteral`] from the store.
    ///
    pub fn exorcise_string_literal(&mut self, id: &usize) -> Option<Arc<RwLock<StringLiteral>>> {
        let result = self.string_literal.write().unwrap()[*id].take();
        self.string_literal_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StringLiteral>`.
    ///
    pub fn iter_string_literal(&self) -> impl Iterator<Item = Arc<RwLock<StringLiteral>>> + '_ {
        let len = self.string_literal.read().unwrap().len();
        (0..len).map(move |i| {
            self.string_literal.read().unwrap()[i]
                .as_ref()
                .map(|string_literal| string_literal.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`WoogStruct`] into the store.
    ///
    pub fn inter_woog_struct<F>(&mut self, woog_struct: F) -> Arc<RwLock<WoogStruct>>
    where
        F: Fn(usize) -> Arc<RwLock<WoogStruct>>,
    {
        let woog_struct = if let Some(_index) = self.woog_struct_free_list.lock().unwrap().pop() {
            let woog_struct = woog_struct(_index);
            self.woog_struct.write().unwrap()[_index] = Some(woog_struct.clone());
            woog_struct
        } else {
            let _index = self.woog_struct.read().unwrap().len();
            let woog_struct = woog_struct(_index);
            self.woog_struct
                .write()
                .unwrap()
                .push(Some(woog_struct.clone()));
            woog_struct
        };
        self.woog_struct_id_by_name.write().unwrap().insert(
            woog_struct.read().unwrap().name.to_upper_camel_case(),
            woog_struct.read().unwrap().id,
        );
        woog_struct
    }

    /// Exhume (get) [`WoogStruct`] from the store.
    ///
    pub fn exhume_woog_struct(&self, id: &usize) -> Option<Arc<RwLock<WoogStruct>>> {
        match self.woog_struct.read().unwrap().get(*id) {
            Some(woog_struct) => woog_struct.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`WoogStruct`] from the store.
    ///
    pub fn exorcise_woog_struct(&mut self, id: &usize) -> Option<Arc<RwLock<WoogStruct>>> {
        let result = self.woog_struct.write().unwrap()[*id].take();
        self.woog_struct_free_list.lock().unwrap().push(*id);
        result
    }

    /// Exorcise [`WoogStruct`] id from the store by name.
    ///
    pub fn exhume_woog_struct_id_by_name(&self, name: &str) -> Option<usize> {
        self.woog_struct_id_by_name
            .read()
            .unwrap()
            .get(name)
            .map(|woog_struct| *woog_struct)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, WoogStruct>`.
    ///
    pub fn iter_woog_struct(&self) -> impl Iterator<Item = Arc<RwLock<WoogStruct>>> + '_ {
        let len = self.woog_struct.read().unwrap().len();
        (0..len).map(move |i| {
            self.woog_struct.read().unwrap()[i]
                .as_ref()
                .map(|woog_struct| woog_struct.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`StructExpression`] into the store.
    ///
    pub fn inter_struct_expression<F>(
        &mut self,
        struct_expression: F,
    ) -> Arc<RwLock<StructExpression>>
    where
        F: Fn(usize) -> Arc<RwLock<StructExpression>>,
    {
        if let Some(_index) = self.struct_expression_free_list.lock().unwrap().pop() {
            let struct_expression = struct_expression(_index);
            self.struct_expression.write().unwrap()[_index] = Some(struct_expression.clone());
            struct_expression
        } else {
            let _index = self.struct_expression.read().unwrap().len();
            let struct_expression = struct_expression(_index);
            self.struct_expression
                .write()
                .unwrap()
                .push(Some(struct_expression.clone()));
            struct_expression
        }
    }

    /// Exhume (get) [`StructExpression`] from the store.
    ///
    pub fn exhume_struct_expression(&self, id: &usize) -> Option<Arc<RwLock<StructExpression>>> {
        match self.struct_expression.read().unwrap().get(*id) {
            Some(struct_expression) => struct_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`StructExpression`] from the store.
    ///
    pub fn exorcise_struct_expression(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<StructExpression>>> {
        let result = self.struct_expression.write().unwrap()[*id].take();
        self.struct_expression_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructExpression>`.
    ///
    pub fn iter_struct_expression(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<StructExpression>>> + '_ {
        let len = self.struct_expression.read().unwrap().len();
        (0..len).map(move |i| {
            self.struct_expression.read().unwrap()[i]
                .as_ref()
                .map(|struct_expression| struct_expression.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`TypeCast`] into the store.
    ///
    pub fn inter_type_cast<F>(&mut self, type_cast: F) -> Arc<RwLock<TypeCast>>
    where
        F: Fn(usize) -> Arc<RwLock<TypeCast>>,
    {
        if let Some(_index) = self.type_cast_free_list.lock().unwrap().pop() {
            let type_cast = type_cast(_index);
            self.type_cast.write().unwrap()[_index] = Some(type_cast.clone());
            type_cast
        } else {
            let _index = self.type_cast.read().unwrap().len();
            let type_cast = type_cast(_index);
            self.type_cast
                .write()
                .unwrap()
                .push(Some(type_cast.clone()));
            type_cast
        }
    }

    /// Exhume (get) [`TypeCast`] from the store.
    ///
    pub fn exhume_type_cast(&self, id: &usize) -> Option<Arc<RwLock<TypeCast>>> {
        match self.type_cast.read().unwrap().get(*id) {
            Some(type_cast) => type_cast.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`TypeCast`] from the store.
    ///
    pub fn exorcise_type_cast(&mut self, id: &usize) -> Option<Arc<RwLock<TypeCast>>> {
        let result = self.type_cast.write().unwrap()[*id].take();
        self.type_cast_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, TypeCast>`.
    ///
    pub fn iter_type_cast(&self) -> impl Iterator<Item = Arc<RwLock<TypeCast>>> + '_ {
        let len = self.type_cast.read().unwrap().len();
        (0..len).map(move |i| {
            self.type_cast.read().unwrap()[i]
                .as_ref()
                .map(|type_cast| type_cast.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Unary`] into the store.
    ///
    pub fn inter_unary<F>(&mut self, unary: F) -> Arc<RwLock<Unary>>
    where
        F: Fn(usize) -> Arc<RwLock<Unary>>,
    {
        if let Some(_index) = self.unary_free_list.lock().unwrap().pop() {
            let unary = unary(_index);
            self.unary.write().unwrap()[_index] = Some(unary.clone());
            unary
        } else {
            let _index = self.unary.read().unwrap().len();
            let unary = unary(_index);
            self.unary.write().unwrap().push(Some(unary.clone()));
            unary
        }
    }

    /// Exhume (get) [`Unary`] from the store.
    ///
    pub fn exhume_unary(&self, id: &usize) -> Option<Arc<RwLock<Unary>>> {
        match self.unary.read().unwrap().get(*id) {
            Some(unary) => unary.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Unary`] from the store.
    ///
    pub fn exorcise_unary(&mut self, id: &usize) -> Option<Arc<RwLock<Unary>>> {
        let result = self.unary.write().unwrap()[*id].take();
        self.unary_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Unary>`.
    ///
    pub fn iter_unary(&self) -> impl Iterator<Item = Arc<RwLock<Unary>>> + '_ {
        let len = self.unary.read().unwrap().len();
        (0..len).map(move |i| {
            self.unary.read().unwrap()[i]
                .as_ref()
                .map(|unary| unary.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`XValue`] into the store.
    ///
    pub fn inter_x_value<F>(&mut self, x_value: F) -> Arc<RwLock<XValue>>
    where
        F: Fn(usize) -> Arc<RwLock<XValue>>,
    {
        if let Some(_index) = self.x_value_free_list.lock().unwrap().pop() {
            let x_value = x_value(_index);
            self.x_value.write().unwrap()[_index] = Some(x_value.clone());
            x_value
        } else {
            let _index = self.x_value.read().unwrap().len();
            let x_value = x_value(_index);
            self.x_value.write().unwrap().push(Some(x_value.clone()));
            x_value
        }
    }

    /// Exhume (get) [`XValue`] from the store.
    ///
    pub fn exhume_x_value(&self, id: &usize) -> Option<Arc<RwLock<XValue>>> {
        match self.x_value.read().unwrap().get(*id) {
            Some(x_value) => x_value.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XValue`] from the store.
    ///
    pub fn exorcise_x_value(&mut self, id: &usize) -> Option<Arc<RwLock<XValue>>> {
        let result = self.x_value.write().unwrap()[*id].take();
        self.x_value_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XValue>`.
    ///
    pub fn iter_x_value(&self) -> impl Iterator<Item = Arc<RwLock<XValue>>> + '_ {
        let len = self.x_value.read().unwrap().len();
        (0..len).map(move |i| {
            self.x_value.read().unwrap()[i]
                .as_ref()
                .map(|x_value| x_value.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`ValueType`] into the store.
    ///
    pub fn inter_value_type<F>(&mut self, value_type: F) -> Arc<RwLock<ValueType>>
    where
        F: Fn(usize) -> Arc<RwLock<ValueType>>,
    {
        if let Some(_index) = self.value_type_free_list.lock().unwrap().pop() {
            let value_type = value_type(_index);
            self.value_type.write().unwrap()[_index] = Some(value_type.clone());
            value_type
        } else {
            let _index = self.value_type.read().unwrap().len();
            let value_type = value_type(_index);
            self.value_type
                .write()
                .unwrap()
                .push(Some(value_type.clone()));
            value_type
        }
    }

    /// Exhume (get) [`ValueType`] from the store.
    ///
    pub fn exhume_value_type(&self, id: &usize) -> Option<Arc<RwLock<ValueType>>> {
        match self.value_type.read().unwrap().get(*id) {
            Some(value_type) => value_type.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ValueType`] from the store.
    ///
    pub fn exorcise_value_type(&mut self, id: &usize) -> Option<Arc<RwLock<ValueType>>> {
        let result = self.value_type.write().unwrap()[*id].take();
        self.value_type_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ValueType>`.
    ///
    pub fn iter_value_type(&self) -> impl Iterator<Item = Arc<RwLock<ValueType>>> + '_ {
        let len = self.value_type.read().unwrap().len();
        (0..len).map(move |i| {
            self.value_type.read().unwrap()[i]
                .as_ref()
                .map(|value_type| value_type.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Variable`] into the store.
    ///
    pub fn inter_variable<F>(&mut self, variable: F) -> Arc<RwLock<Variable>>
    where
        F: Fn(usize) -> Arc<RwLock<Variable>>,
    {
        if let Some(_index) = self.variable_free_list.lock().unwrap().pop() {
            let variable = variable(_index);
            self.variable.write().unwrap()[_index] = Some(variable.clone());
            variable
        } else {
            let _index = self.variable.read().unwrap().len();
            let variable = variable(_index);
            self.variable.write().unwrap().push(Some(variable.clone()));
            variable
        }
    }

    /// Exhume (get) [`Variable`] from the store.
    ///
    pub fn exhume_variable(&self, id: &usize) -> Option<Arc<RwLock<Variable>>> {
        match self.variable.read().unwrap().get(*id) {
            Some(variable) => variable.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Variable`] from the store.
    ///
    pub fn exorcise_variable(&mut self, id: &usize) -> Option<Arc<RwLock<Variable>>> {
        let result = self.variable.write().unwrap()[*id].take();
        self.variable_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Variable>`.
    ///
    pub fn iter_variable(&self) -> impl Iterator<Item = Arc<RwLock<Variable>>> + '_ {
        let len = self.variable.read().unwrap().len();
        (0..len).map(move |i| {
            self.variable.read().unwrap()[i]
                .as_ref()
                .map(|variable| variable.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`VariableExpression`] into the store.
    ///
    pub fn inter_variable_expression<F>(
        &mut self,
        variable_expression: F,
    ) -> Arc<RwLock<VariableExpression>>
    where
        F: Fn(usize) -> Arc<RwLock<VariableExpression>>,
    {
        if let Some(_index) = self.variable_expression_free_list.lock().unwrap().pop() {
            let variable_expression = variable_expression(_index);
            self.variable_expression.write().unwrap()[_index] = Some(variable_expression.clone());
            variable_expression
        } else {
            let _index = self.variable_expression.read().unwrap().len();
            let variable_expression = variable_expression(_index);
            self.variable_expression
                .write()
                .unwrap()
                .push(Some(variable_expression.clone()));
            variable_expression
        }
    }

    /// Exhume (get) [`VariableExpression`] from the store.
    ///
    pub fn exhume_variable_expression(
        &self,
        id: &usize,
    ) -> Option<Arc<RwLock<VariableExpression>>> {
        match self.variable_expression.read().unwrap().get(*id) {
            Some(variable_expression) => variable_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`VariableExpression`] from the store.
    ///
    pub fn exorcise_variable_expression(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<VariableExpression>>> {
        let result = self.variable_expression.write().unwrap()[*id].take();
        self.variable_expression_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, VariableExpression>`.
    ///
    pub fn iter_variable_expression(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<VariableExpression>>> + '_ {
        let len = self.variable_expression.read().unwrap().len();
        (0..len).map(move |i| {
            self.variable_expression.read().unwrap()[i]
                .as_ref()
                .map(|variable_expression| variable_expression.clone())
                .unwrap()
        })
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_rwlock_vec-object-store-persistence"}}}
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
            for argument in &*self.argument.read().unwrap() {
                if let Some(argument) = argument {
                    let path = path.join(format!("{}.json", argument.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &argument)?;
                }
            }
        }

        // Persist Binary.
        {
            let path = path.join("binary");
            fs::create_dir_all(&path)?;
            for binary in &*self.binary.read().unwrap() {
                if let Some(binary) = binary {
                    let path = path.join(format!("{}.json", binary.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &binary)?;
                }
            }
        }

        // Persist Block.
        {
            let path = path.join("block");
            fs::create_dir_all(&path)?;
            for block in &*self.block.read().unwrap() {
                if let Some(block) = block {
                    let path = path.join(format!("{}.json", block.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &block)?;
                }
            }
        }

        // Persist Boolean Literal.
        {
            let path = path.join("boolean_literal");
            fs::create_dir_all(&path)?;
            for boolean_literal in &*self.boolean_literal.read().unwrap() {
                if let Some(boolean_literal) = boolean_literal {
                    let path = path.join(format!("{}.json", boolean_literal.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &boolean_literal)?;
                }
            }
        }

        // Persist Boolean Operator.
        {
            let path = path.join("boolean_operator");
            fs::create_dir_all(&path)?;
            for boolean_operator in &*self.boolean_operator.read().unwrap() {
                if let Some(boolean_operator) = boolean_operator {
                    let path = path.join(format!("{}.json", boolean_operator.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &boolean_operator)?;
                }
            }
        }

        // Persist Call.
        {
            let path = path.join("call");
            fs::create_dir_all(&path)?;
            for call in &*self.call.read().unwrap() {
                if let Some(call) = call {
                    let path = path.join(format!("{}.json", call.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &call)?;
                }
            }
        }

        // Persist Comparison.
        {
            let path = path.join("comparison");
            fs::create_dir_all(&path)?;
            for comparison in &*self.comparison.read().unwrap() {
                if let Some(comparison) = comparison {
                    let path = path.join(format!("{}.json", comparison.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &comparison)?;
                }
            }
        }

        // Persist Dwarf Source File.
        {
            let path = path.join("dwarf_source_file");
            fs::create_dir_all(&path)?;
            for dwarf_source_file in &*self.dwarf_source_file.read().unwrap() {
                if let Some(dwarf_source_file) = dwarf_source_file {
                    let path = path.join(format!("{}.json", dwarf_source_file.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &dwarf_source_file)?;
                }
            }
        }

        // Persist Error.
        {
            let path = path.join("error");
            fs::create_dir_all(&path)?;
            for error in &*self.error.read().unwrap() {
                if let Some(error) = error {
                    let path = path.join(format!("{}.json", error.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &error)?;
                }
            }
        }

        // Persist Error Expression.
        {
            let path = path.join("error_expression");
            fs::create_dir_all(&path)?;
            for error_expression in &*self.error_expression.read().unwrap() {
                if let Some(error_expression) = error_expression {
                    let path = path.join(format!("{}.json", error_expression.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &error_expression)?;
                }
            }
        }

        // Persist Expression.
        {
            let path = path.join("expression");
            fs::create_dir_all(&path)?;
            for expression in &*self.expression.read().unwrap() {
                if let Some(expression) = expression {
                    let path = path.join(format!("{}.json", expression.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &expression)?;
                }
            }
        }

        // Persist Expression Statement.
        {
            let path = path.join("expression_statement");
            fs::create_dir_all(&path)?;
            for expression_statement in &*self.expression_statement.read().unwrap() {
                if let Some(expression_statement) = expression_statement {
                    let path =
                        path.join(format!("{}.json", expression_statement.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &expression_statement)?;
                }
            }
        }

        // Persist Field.
        {
            let path = path.join("field");
            fs::create_dir_all(&path)?;
            for field in &*self.field.read().unwrap() {
                if let Some(field) = field {
                    let path = path.join(format!("{}.json", field.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &field)?;
                }
            }
        }

        // Persist Field Access.
        {
            let path = path.join("field_access");
            fs::create_dir_all(&path)?;
            for field_access in &*self.field_access.read().unwrap() {
                if let Some(field_access) = field_access {
                    let path = path.join(format!("{}.json", field_access.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &field_access)?;
                }
            }
        }

        // Persist Field Access Target.
        {
            let path = path.join("field_access_target");
            fs::create_dir_all(&path)?;
            for field_access_target in &*self.field_access_target.read().unwrap() {
                if let Some(field_access_target) = field_access_target {
                    let path =
                        path.join(format!("{}.json", field_access_target.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &field_access_target)?;
                }
            }
        }

        // Persist Field Expression.
        {
            let path = path.join("field_expression");
            fs::create_dir_all(&path)?;
            for field_expression in &*self.field_expression.read().unwrap() {
                if let Some(field_expression) = field_expression {
                    let path = path.join(format!("{}.json", field_expression.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &field_expression)?;
                }
            }
        }

        // Persist Float Literal.
        {
            let path = path.join("float_literal");
            fs::create_dir_all(&path)?;
            for float_literal in &*self.float_literal.read().unwrap() {
                if let Some(float_literal) = float_literal {
                    let path = path.join(format!("{}.json", float_literal.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &float_literal)?;
                }
            }
        }

        // Persist For Loop.
        {
            let path = path.join("for_loop");
            fs::create_dir_all(&path)?;
            for for_loop in &*self.for_loop.read().unwrap() {
                if let Some(for_loop) = for_loop {
                    let path = path.join(format!("{}.json", for_loop.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &for_loop)?;
                }
            }
        }

        // Persist Function.
        {
            let path = path.join("function");
            fs::create_dir_all(&path)?;
            for function in &*self.function.read().unwrap() {
                if let Some(function) = function {
                    let path = path.join(format!("{}.json", function.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &function)?;
                }
            }
        }

        // Persist Grouped.
        {
            let path = path.join("grouped");
            fs::create_dir_all(&path)?;
            for grouped in &*self.grouped.read().unwrap() {
                if let Some(grouped) = grouped {
                    let path = path.join(format!("{}.json", grouped.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &grouped)?;
                }
            }
        }

        // Persist If.
        {
            let path = path.join("x_if");
            fs::create_dir_all(&path)?;
            for x_if in &*self.x_if.read().unwrap() {
                if let Some(x_if) = x_if {
                    let path = path.join(format!("{}.json", x_if.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &x_if)?;
                }
            }
        }

        // Persist Implementation.
        {
            let path = path.join("implementation");
            fs::create_dir_all(&path)?;
            for implementation in &*self.implementation.read().unwrap() {
                if let Some(implementation) = implementation {
                    let path = path.join(format!("{}.json", implementation.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &implementation)?;
                }
            }
        }

        // Persist Import.
        {
            let path = path.join("import");
            fs::create_dir_all(&path)?;
            for import in &*self.import.read().unwrap() {
                if let Some(import) = import {
                    let path = path.join(format!("{}.json", import.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &import)?;
                }
            }
        }

        // Persist Index.
        {
            let path = path.join("index");
            fs::create_dir_all(&path)?;
            for index in &*self.index.read().unwrap() {
                if let Some(index) = index {
                    let path = path.join(format!("{}.json", index.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &index)?;
                }
            }
        }

        // Persist Integer Literal.
        {
            let path = path.join("integer_literal");
            fs::create_dir_all(&path)?;
            for integer_literal in &*self.integer_literal.read().unwrap() {
                if let Some(integer_literal) = integer_literal {
                    let path = path.join(format!("{}.json", integer_literal.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &integer_literal)?;
                }
            }
        }

        // Persist Item.
        {
            let path = path.join("item");
            fs::create_dir_all(&path)?;
            for item in &*self.item.read().unwrap() {
                if let Some(item) = item {
                    let path = path.join(format!("{}.json", item.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &item)?;
                }
            }
        }

        // Persist Lambda.
        {
            let path = path.join("lambda");
            fs::create_dir_all(&path)?;
            for lambda in &*self.lambda.read().unwrap() {
                if let Some(lambda) = lambda {
                    let path = path.join(format!("{}.json", lambda.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &lambda)?;
                }
            }
        }

        // Persist Lambda Parameter.
        {
            let path = path.join("lambda_parameter");
            fs::create_dir_all(&path)?;
            for lambda_parameter in &*self.lambda_parameter.read().unwrap() {
                if let Some(lambda_parameter) = lambda_parameter {
                    let path = path.join(format!("{}.json", lambda_parameter.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &lambda_parameter)?;
                }
            }
        }

        // Persist Let Statement.
        {
            let path = path.join("let_statement");
            fs::create_dir_all(&path)?;
            for let_statement in &*self.let_statement.read().unwrap() {
                if let Some(let_statement) = let_statement {
                    let path = path.join(format!("{}.json", let_statement.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &let_statement)?;
                }
            }
        }

        // Persist List.
        {
            let path = path.join("list");
            fs::create_dir_all(&path)?;
            for list in &*self.list.read().unwrap() {
                if let Some(list) = list {
                    let path = path.join(format!("{}.json", list.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &list)?;
                }
            }
        }

        // Persist List Element.
        {
            let path = path.join("list_element");
            fs::create_dir_all(&path)?;
            for list_element in &*self.list_element.read().unwrap() {
                if let Some(list_element) = list_element {
                    let path = path.join(format!("{}.json", list_element.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &list_element)?;
                }
            }
        }

        // Persist List Expression.
        {
            let path = path.join("list_expression");
            fs::create_dir_all(&path)?;
            for list_expression in &*self.list_expression.read().unwrap() {
                if let Some(list_expression) = list_expression {
                    let path = path.join(format!("{}.json", list_expression.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &list_expression)?;
                }
            }
        }

        // Persist Literal.
        {
            let path = path.join("literal");
            fs::create_dir_all(&path)?;
            for literal in &*self.literal.read().unwrap() {
                if let Some(literal) = literal {
                    let path = path.join(format!("{}.json", literal.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &literal)?;
                }
            }
        }

        // Persist Local Variable.
        {
            let path = path.join("local_variable");
            fs::create_dir_all(&path)?;
            for local_variable in &*self.local_variable.read().unwrap() {
                if let Some(local_variable) = local_variable {
                    let path = path.join(format!("{}.json", local_variable.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &local_variable)?;
                }
            }
        }

        // Persist Macro.
        {
            let path = path.join("x_macro");
            fs::create_dir_all(&path)?;
            for x_macro in &*self.x_macro.read().unwrap() {
                if let Some(x_macro) = x_macro {
                    let path = path.join(format!("{}.json", x_macro.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &x_macro)?;
                }
            }
        }

        // Persist Method Call.
        {
            let path = path.join("method_call");
            fs::create_dir_all(&path)?;
            for method_call in &*self.method_call.read().unwrap() {
                if let Some(method_call) = method_call {
                    let path = path.join(format!("{}.json", method_call.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &method_call)?;
                }
            }
        }

        // Persist Object Store.
        {
            let path = path.join("z_object_store");
            fs::create_dir_all(&path)?;
            for z_object_store in &*self.z_object_store.read().unwrap() {
                if let Some(z_object_store) = z_object_store {
                    let path = path.join(format!("{}.json", z_object_store.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &z_object_store)?;
                }
            }
        }

        // Persist Operator.
        {
            let path = path.join("operator");
            fs::create_dir_all(&path)?;
            for operator in &*self.operator.read().unwrap() {
                if let Some(operator) = operator {
                    let path = path.join(format!("{}.json", operator.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &operator)?;
                }
            }
        }

        // Persist Option.
        {
            let path = path.join("woog_option");
            fs::create_dir_all(&path)?;
            for woog_option in &*self.woog_option.read().unwrap() {
                if let Some(woog_option) = woog_option {
                    let path = path.join(format!("{}.json", woog_option.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &woog_option)?;
                }
            }
        }

        // Persist Parameter.
        {
            let path = path.join("parameter");
            fs::create_dir_all(&path)?;
            for parameter in &*self.parameter.read().unwrap() {
                if let Some(parameter) = parameter {
                    let path = path.join(format!("{}.json", parameter.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &parameter)?;
                }
            }
        }

        // Persist Print.
        {
            let path = path.join("print");
            fs::create_dir_all(&path)?;
            for print in &*self.print.read().unwrap() {
                if let Some(print) = print {
                    let path = path.join(format!("{}.json", print.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &print)?;
                }
            }
        }

        // Persist Range Expression.
        {
            let path = path.join("range_expression");
            fs::create_dir_all(&path)?;
            for range_expression in &*self.range_expression.read().unwrap() {
                if let Some(range_expression) = range_expression {
                    let path = path.join(format!("{}.json", range_expression.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &range_expression)?;
                }
            }
        }

        // Persist Reference.
        {
            let path = path.join("reference");
            fs::create_dir_all(&path)?;
            for reference in &*self.reference.read().unwrap() {
                if let Some(reference) = reference {
                    let path = path.join(format!("{}.json", reference.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &reference)?;
                }
            }
        }

        // Persist Result Statement.
        {
            let path = path.join("result_statement");
            fs::create_dir_all(&path)?;
            for result_statement in &*self.result_statement.read().unwrap() {
                if let Some(result_statement) = result_statement {
                    let path = path.join(format!("{}.json", result_statement.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &result_statement)?;
                }
            }
        }

        // Persist Return.
        {
            let path = path.join("x_return");
            fs::create_dir_all(&path)?;
            for x_return in &*self.x_return.read().unwrap() {
                if let Some(x_return) = x_return {
                    let path = path.join(format!("{}.json", x_return.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &x_return)?;
                }
            }
        }

        // Persist Some.
        {
            let path = path.join("z_some");
            fs::create_dir_all(&path)?;
            for z_some in &*self.z_some.read().unwrap() {
                if let Some(z_some) = z_some {
                    let path = path.join(format!("{}.json", z_some.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &z_some)?;
                }
            }
        }

        // Persist Span.
        {
            let path = path.join("span");
            fs::create_dir_all(&path)?;
            for span in &*self.span.read().unwrap() {
                if let Some(span) = span {
                    let path = path.join(format!("{}.json", span.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &span)?;
                }
            }
        }

        // Persist Statement.
        {
            let path = path.join("statement");
            fs::create_dir_all(&path)?;
            for statement in &*self.statement.read().unwrap() {
                if let Some(statement) = statement {
                    let path = path.join(format!("{}.json", statement.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &statement)?;
                }
            }
        }

        // Persist Static Method Call.
        {
            let path = path.join("static_method_call");
            fs::create_dir_all(&path)?;
            for static_method_call in &*self.static_method_call.read().unwrap() {
                if let Some(static_method_call) = static_method_call {
                    let path = path.join(format!("{}.json", static_method_call.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &static_method_call)?;
                }
            }
        }

        // Persist String Literal.
        {
            let path = path.join("string_literal");
            fs::create_dir_all(&path)?;
            for string_literal in &*self.string_literal.read().unwrap() {
                if let Some(string_literal) = string_literal {
                    let path = path.join(format!("{}.json", string_literal.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &string_literal)?;
                }
            }
        }

        // Persist Struct.
        {
            let path = path.join("woog_struct");
            fs::create_dir_all(&path)?;
            for woog_struct in &*self.woog_struct.read().unwrap() {
                if let Some(woog_struct) = woog_struct {
                    let path = path.join(format!("{}.json", woog_struct.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &woog_struct)?;
                }
            }
        }

        // Persist Struct Expression.
        {
            let path = path.join("struct_expression");
            fs::create_dir_all(&path)?;
            for struct_expression in &*self.struct_expression.read().unwrap() {
                if let Some(struct_expression) = struct_expression {
                    let path = path.join(format!("{}.json", struct_expression.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &struct_expression)?;
                }
            }
        }

        // Persist Type Cast.
        {
            let path = path.join("type_cast");
            fs::create_dir_all(&path)?;
            for type_cast in &*self.type_cast.read().unwrap() {
                if let Some(type_cast) = type_cast {
                    let path = path.join(format!("{}.json", type_cast.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &type_cast)?;
                }
            }
        }

        // Persist Unary.
        {
            let path = path.join("unary");
            fs::create_dir_all(&path)?;
            for unary in &*self.unary.read().unwrap() {
                if let Some(unary) = unary {
                    let path = path.join(format!("{}.json", unary.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &unary)?;
                }
            }
        }

        // Persist Value.
        {
            let path = path.join("x_value");
            fs::create_dir_all(&path)?;
            for x_value in &*self.x_value.read().unwrap() {
                if let Some(x_value) = x_value {
                    let path = path.join(format!("{}.json", x_value.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &x_value)?;
                }
            }
        }

        // Persist Value Type.
        {
            let path = path.join("value_type");
            fs::create_dir_all(&path)?;
            for value_type in &*self.value_type.read().unwrap() {
                if let Some(value_type) = value_type {
                    let path = path.join(format!("{}.json", value_type.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &value_type)?;
                }
            }
        }

        // Persist Variable.
        {
            let path = path.join("variable");
            fs::create_dir_all(&path)?;
            for variable in &*self.variable.read().unwrap() {
                if let Some(variable) = variable {
                    let path = path.join(format!("{}.json", variable.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &variable)?;
                }
            }
        }

        // Persist Variable Expression.
        {
            let path = path.join("variable_expression");
            fs::create_dir_all(&path)?;
            for variable_expression in &*self.variable_expression.read().unwrap() {
                if let Some(variable_expression) = variable_expression {
                    let path =
                        path.join(format!("{}.json", variable_expression.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &variable_expression)?;
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
                let argument: Arc<RwLock<Argument>> = serde_json::from_reader(reader)?;
                store
                    .argument
                    .write()
                    .unwrap()
                    .insert(argument.read().unwrap().id, Some(argument.clone()));
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
                let binary: Arc<RwLock<Binary>> = serde_json::from_reader(reader)?;
                store
                    .binary
                    .write()
                    .unwrap()
                    .insert(binary.read().unwrap().id, Some(binary.clone()));
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
                let block: Arc<RwLock<Block>> = serde_json::from_reader(reader)?;
                store
                    .block
                    .write()
                    .unwrap()
                    .insert(block.read().unwrap().id, Some(block.clone()));
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
                let boolean_literal: Arc<RwLock<BooleanLiteral>> = serde_json::from_reader(reader)?;
                store.boolean_literal.write().unwrap().insert(
                    boolean_literal.read().unwrap().id,
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
                    serde_json::from_reader(reader)?;
                store.boolean_operator.write().unwrap().insert(
                    boolean_operator.read().unwrap().id,
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
                let call: Arc<RwLock<Call>> = serde_json::from_reader(reader)?;
                store
                    .call
                    .write()
                    .unwrap()
                    .insert(call.read().unwrap().id, Some(call.clone()));
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
                let comparison: Arc<RwLock<Comparison>> = serde_json::from_reader(reader)?;
                store
                    .comparison
                    .write()
                    .unwrap()
                    .insert(comparison.read().unwrap().id, Some(comparison.clone()));
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
                    serde_json::from_reader(reader)?;
                store.dwarf_source_file.write().unwrap().insert(
                    dwarf_source_file.read().unwrap().id,
                    Some(dwarf_source_file.clone()),
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
                let error: Arc<RwLock<Error>> = serde_json::from_reader(reader)?;
                store
                    .error
                    .write()
                    .unwrap()
                    .insert(error.read().unwrap().id, Some(error.clone()));
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
                    serde_json::from_reader(reader)?;
                store.error_expression.write().unwrap().insert(
                    error_expression.read().unwrap().id,
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
                let expression: Arc<RwLock<Expression>> = serde_json::from_reader(reader)?;
                store
                    .expression
                    .write()
                    .unwrap()
                    .insert(expression.read().unwrap().id, Some(expression.clone()));
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
                    serde_json::from_reader(reader)?;
                store.expression_statement.write().unwrap().insert(
                    expression_statement.read().unwrap().id,
                    Some(expression_statement.clone()),
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
                let field: Arc<RwLock<Field>> = serde_json::from_reader(reader)?;
                store.field_id_by_name.write().unwrap().insert(
                    field.read().unwrap().name.to_upper_camel_case(),
                    field.read().unwrap().id,
                );
                store
                    .field
                    .write()
                    .unwrap()
                    .insert(field.read().unwrap().id, Some(field.clone()));
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
                let field_access: Arc<RwLock<FieldAccess>> = serde_json::from_reader(reader)?;
                store
                    .field_access
                    .write()
                    .unwrap()
                    .insert(field_access.read().unwrap().id, Some(field_access.clone()));
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
                    serde_json::from_reader(reader)?;
                store.field_access_target.write().unwrap().insert(
                    field_access_target.read().unwrap().id,
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
                    serde_json::from_reader(reader)?;
                store.field_expression.write().unwrap().insert(
                    field_expression.read().unwrap().id,
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
                let float_literal: Arc<RwLock<FloatLiteral>> = serde_json::from_reader(reader)?;
                store.float_literal.write().unwrap().insert(
                    float_literal.read().unwrap().id,
                    Some(float_literal.clone()),
                );
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
                let for_loop: Arc<RwLock<ForLoop>> = serde_json::from_reader(reader)?;
                store
                    .for_loop
                    .write()
                    .unwrap()
                    .insert(for_loop.read().unwrap().id, Some(for_loop.clone()));
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
                let function: Arc<RwLock<Function>> = serde_json::from_reader(reader)?;
                store.function_id_by_name.write().unwrap().insert(
                    function.read().unwrap().name.to_upper_camel_case(),
                    function.read().unwrap().id,
                );
                store
                    .function
                    .write()
                    .unwrap()
                    .insert(function.read().unwrap().id, Some(function.clone()));
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
                let grouped: Arc<RwLock<Grouped>> = serde_json::from_reader(reader)?;
                store
                    .grouped
                    .write()
                    .unwrap()
                    .insert(grouped.read().unwrap().id, Some(grouped.clone()));
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
                let x_if: Arc<RwLock<XIf>> = serde_json::from_reader(reader)?;
                store
                    .x_if
                    .write()
                    .unwrap()
                    .insert(x_if.read().unwrap().id, Some(x_if.clone()));
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
                let implementation: Arc<RwLock<Implementation>> = serde_json::from_reader(reader)?;
                store.implementation.write().unwrap().insert(
                    implementation.read().unwrap().id,
                    Some(implementation.clone()),
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
                let import: Arc<RwLock<Import>> = serde_json::from_reader(reader)?;
                store
                    .import
                    .write()
                    .unwrap()
                    .insert(import.read().unwrap().id, Some(import.clone()));
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
                let index: Arc<RwLock<Index>> = serde_json::from_reader(reader)?;
                store
                    .index
                    .write()
                    .unwrap()
                    .insert(index.read().unwrap().id, Some(index.clone()));
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
                let integer_literal: Arc<RwLock<IntegerLiteral>> = serde_json::from_reader(reader)?;
                store.integer_literal.write().unwrap().insert(
                    integer_literal.read().unwrap().id,
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
                let item: Arc<RwLock<Item>> = serde_json::from_reader(reader)?;
                store
                    .item
                    .write()
                    .unwrap()
                    .insert(item.read().unwrap().id, Some(item.clone()));
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
                let lambda: Arc<RwLock<Lambda>> = serde_json::from_reader(reader)?;
                store
                    .lambda
                    .write()
                    .unwrap()
                    .insert(lambda.read().unwrap().id, Some(lambda.clone()));
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
                    serde_json::from_reader(reader)?;
                store.lambda_parameter.write().unwrap().insert(
                    lambda_parameter.read().unwrap().id,
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
                let let_statement: Arc<RwLock<LetStatement>> = serde_json::from_reader(reader)?;
                store.let_statement.write().unwrap().insert(
                    let_statement.read().unwrap().id,
                    Some(let_statement.clone()),
                );
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
                let list: Arc<RwLock<List>> = serde_json::from_reader(reader)?;
                store
                    .list
                    .write()
                    .unwrap()
                    .insert(list.read().unwrap().id, Some(list.clone()));
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
                let list_element: Arc<RwLock<ListElement>> = serde_json::from_reader(reader)?;
                store
                    .list_element
                    .write()
                    .unwrap()
                    .insert(list_element.read().unwrap().id, Some(list_element.clone()));
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
                let list_expression: Arc<RwLock<ListExpression>> = serde_json::from_reader(reader)?;
                store.list_expression.write().unwrap().insert(
                    list_expression.read().unwrap().id,
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
                let literal: Arc<RwLock<Literal>> = serde_json::from_reader(reader)?;
                store
                    .literal
                    .write()
                    .unwrap()
                    .insert(literal.read().unwrap().id, Some(literal.clone()));
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
                let local_variable: Arc<RwLock<LocalVariable>> = serde_json::from_reader(reader)?;
                store.local_variable.write().unwrap().insert(
                    local_variable.read().unwrap().id,
                    Some(local_variable.clone()),
                );
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
                let x_macro: Arc<RwLock<XMacro>> = serde_json::from_reader(reader)?;
                store
                    .x_macro
                    .write()
                    .unwrap()
                    .insert(x_macro.read().unwrap().id, Some(x_macro.clone()));
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
                let method_call: Arc<RwLock<MethodCall>> = serde_json::from_reader(reader)?;
                store
                    .method_call
                    .write()
                    .unwrap()
                    .insert(method_call.read().unwrap().id, Some(method_call.clone()));
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
                let z_object_store: Arc<RwLock<ZObjectStore>> = serde_json::from_reader(reader)?;
                store.z_object_store.write().unwrap().insert(
                    z_object_store.read().unwrap().id,
                    Some(z_object_store.clone()),
                );
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
                let operator: Arc<RwLock<Operator>> = serde_json::from_reader(reader)?;
                store
                    .operator
                    .write()
                    .unwrap()
                    .insert(operator.read().unwrap().id, Some(operator.clone()));
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
                let woog_option: Arc<RwLock<WoogOption>> = serde_json::from_reader(reader)?;
                store
                    .woog_option
                    .write()
                    .unwrap()
                    .insert(woog_option.read().unwrap().id, Some(woog_option.clone()));
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
                let parameter: Arc<RwLock<Parameter>> = serde_json::from_reader(reader)?;
                store
                    .parameter
                    .write()
                    .unwrap()
                    .insert(parameter.read().unwrap().id, Some(parameter.clone()));
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
                let print: Arc<RwLock<Print>> = serde_json::from_reader(reader)?;
                store
                    .print
                    .write()
                    .unwrap()
                    .insert(print.read().unwrap().id, Some(print.clone()));
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
                    serde_json::from_reader(reader)?;
                store.range_expression.write().unwrap().insert(
                    range_expression.read().unwrap().id,
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
                let reference: Arc<RwLock<Reference>> = serde_json::from_reader(reader)?;
                store
                    .reference
                    .write()
                    .unwrap()
                    .insert(reference.read().unwrap().id, Some(reference.clone()));
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
                    serde_json::from_reader(reader)?;
                store.result_statement.write().unwrap().insert(
                    result_statement.read().unwrap().id,
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
                let x_return: Arc<RwLock<XReturn>> = serde_json::from_reader(reader)?;
                store
                    .x_return
                    .write()
                    .unwrap()
                    .insert(x_return.read().unwrap().id, Some(x_return.clone()));
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
                let z_some: Arc<RwLock<ZSome>> = serde_json::from_reader(reader)?;
                store
                    .z_some
                    .write()
                    .unwrap()
                    .insert(z_some.read().unwrap().id, Some(z_some.clone()));
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
                let span: Arc<RwLock<Span>> = serde_json::from_reader(reader)?;
                store
                    .span
                    .write()
                    .unwrap()
                    .insert(span.read().unwrap().id, Some(span.clone()));
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
                let statement: Arc<RwLock<Statement>> = serde_json::from_reader(reader)?;
                store
                    .statement
                    .write()
                    .unwrap()
                    .insert(statement.read().unwrap().id, Some(statement.clone()));
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
                    serde_json::from_reader(reader)?;
                store.static_method_call.write().unwrap().insert(
                    static_method_call.read().unwrap().id,
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
                let string_literal: Arc<RwLock<StringLiteral>> = serde_json::from_reader(reader)?;
                store.string_literal.write().unwrap().insert(
                    string_literal.read().unwrap().id,
                    Some(string_literal.clone()),
                );
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
                let woog_struct: Arc<RwLock<WoogStruct>> = serde_json::from_reader(reader)?;
                store.woog_struct_id_by_name.write().unwrap().insert(
                    woog_struct.read().unwrap().name.to_upper_camel_case(),
                    woog_struct.read().unwrap().id,
                );
                store
                    .woog_struct
                    .write()
                    .unwrap()
                    .insert(woog_struct.read().unwrap().id, Some(woog_struct.clone()));
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
                    serde_json::from_reader(reader)?;
                store.struct_expression.write().unwrap().insert(
                    struct_expression.read().unwrap().id,
                    Some(struct_expression.clone()),
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
                let type_cast: Arc<RwLock<TypeCast>> = serde_json::from_reader(reader)?;
                store
                    .type_cast
                    .write()
                    .unwrap()
                    .insert(type_cast.read().unwrap().id, Some(type_cast.clone()));
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
                let unary: Arc<RwLock<Unary>> = serde_json::from_reader(reader)?;
                store
                    .unary
                    .write()
                    .unwrap()
                    .insert(unary.read().unwrap().id, Some(unary.clone()));
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
                let x_value: Arc<RwLock<XValue>> = serde_json::from_reader(reader)?;
                store
                    .x_value
                    .write()
                    .unwrap()
                    .insert(x_value.read().unwrap().id, Some(x_value.clone()));
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
                let value_type: Arc<RwLock<ValueType>> = serde_json::from_reader(reader)?;
                store
                    .value_type
                    .write()
                    .unwrap()
                    .insert(value_type.read().unwrap().id, Some(value_type.clone()));
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
                let variable: Arc<RwLock<Variable>> = serde_json::from_reader(reader)?;
                store
                    .variable
                    .write()
                    .unwrap()
                    .insert(variable.read().unwrap().id, Some(variable.clone()));
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
                    serde_json::from_reader(reader)?;
                store.variable_expression.write().unwrap().insert(
                    variable_expression.read().unwrap().id,
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
