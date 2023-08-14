//! v2::lu_dog_ndrwlock_vec Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_ndrwlock_vec-object-store-file"}}}
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
//! * [`Error`]
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
//! * [`MethodCall`]
//! * [`ZObjectStore`]
//! * [`ObjectWrapper`]
//! * [`Operator`]
//! * [`WoogOption`]
//! * [`Parameter`]
//! * [`Plain`]
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
//! * [`StructField`]
//! * [`TupleField`]
//! * [`TypeCast`]
//! * [`Unary`]
//! * [`XValue`]
//! * [`ValueType`]
//! * [`Variable`]
//! * [`VariableExpression`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_ndrwlock_vec-object-store-definition"}}}
use no_deadlocks::RwLock;
use std::sync::Arc;

use heck::ToUpperCamelCase;
use rustc_hash::FxHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v2::lu_dog_ndrwlock_vec::types::{
    Argument, Binary, Block, Body, BooleanLiteral, BooleanOperator, Call, Comparison,
    DwarfSourceFile, EnumField, Enumeration, Error, ErrorExpression, Expression,
    ExpressionStatement, ExternalImplementation, Field, FieldAccess, FieldAccessTarget,
    FieldExpression, FloatLiteral, ForLoop, Function, Grouped, ImplementationBlock, Import, Index,
    IntegerLiteral, Item, Lambda, LambdaParameter, LetStatement, List, ListElement, ListExpression,
    Literal, LocalVariable, MethodCall, ObjectWrapper, Operator, Parameter, Plain, Print,
    RangeExpression, Reference, ResultStatement, Span, Statement, StaticMethodCall, StringLiteral,
    StructExpression, StructField, TupleField, TypeCast, Unary, ValueType, Variable,
    VariableExpression, WoogOption, WoogStruct, XIf, XMacro, XReturn, XValue, ZObjectStore, ZSome,
    ADDITION, AND, ASSIGNMENT, CHAR, DEBUGGER, DIVISION, EMPTY, EQUAL, FALSE_LITERAL, FROM, FULL,
    FUNCTION_CALL, GREATER_THAN, GREATER_THAN_OR_EQUAL, INCLUSIVE, ITEM_STATEMENT, LESS_THAN,
    LESS_THAN_OR_EQUAL, MACRO_CALL, MULTIPLICATION, NEGATION, NOT, NOT_EQUAL, OR, RANGE,
    SUBTRACTION, TO, TO_INCLUSIVE, TRUE_LITERAL, UNKNOWN, UNKNOWN_VARIABLE, Z_NONE,
};

#[derive(Debug)]
pub struct ObjectStore {
    argument_free_list: std::sync::Mutex<Vec<usize>>,
    argument: Arc<RwLock<Vec<Option<Arc<RwLock<Argument>>>>>>,
    binary_free_list: std::sync::Mutex<Vec<usize>>,
    binary: Arc<RwLock<Vec<Option<Arc<RwLock<Binary>>>>>>,
    block_free_list: std::sync::Mutex<Vec<usize>>,
    block: Arc<RwLock<Vec<Option<Arc<RwLock<Block>>>>>>,
    body_free_list: std::sync::Mutex<Vec<usize>>,
    body: Arc<RwLock<Vec<Option<Arc<RwLock<Body>>>>>>,
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
    enum_field_free_list: std::sync::Mutex<Vec<usize>>,
    enum_field: Arc<RwLock<Vec<Option<Arc<RwLock<EnumField>>>>>>,
    enumeration_free_list: std::sync::Mutex<Vec<usize>>,
    enumeration: Arc<RwLock<Vec<Option<Arc<RwLock<Enumeration>>>>>>,
    enumeration_id_by_name: Arc<RwLock<HashMap<String, usize>>>,
    error_free_list: std::sync::Mutex<Vec<usize>>,
    error: Arc<RwLock<Vec<Option<Arc<RwLock<Error>>>>>>,
    error_expression_free_list: std::sync::Mutex<Vec<usize>>,
    error_expression: Arc<RwLock<Vec<Option<Arc<RwLock<ErrorExpression>>>>>>,
    expression_free_list: std::sync::Mutex<Vec<usize>>,
    expression: Arc<RwLock<Vec<Option<Arc<RwLock<Expression>>>>>>,
    expression_statement_free_list: std::sync::Mutex<Vec<usize>>,
    expression_statement: Arc<RwLock<Vec<Option<Arc<RwLock<ExpressionStatement>>>>>>,
    external_implementation_free_list: std::sync::Mutex<Vec<usize>>,
    external_implementation: Arc<RwLock<Vec<Option<Arc<RwLock<ExternalImplementation>>>>>>,
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
    implementation_block_free_list: std::sync::Mutex<Vec<usize>>,
    implementation_block: Arc<RwLock<Vec<Option<Arc<RwLock<ImplementationBlock>>>>>>,
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
    z_object_store_id_by_name: Arc<RwLock<HashMap<String, usize>>>,
    object_wrapper_free_list: std::sync::Mutex<Vec<usize>>,
    object_wrapper: Arc<RwLock<Vec<Option<Arc<RwLock<ObjectWrapper>>>>>>,
    operator_free_list: std::sync::Mutex<Vec<usize>>,
    operator: Arc<RwLock<Vec<Option<Arc<RwLock<Operator>>>>>>,
    woog_option_free_list: std::sync::Mutex<Vec<usize>>,
    woog_option: Arc<RwLock<Vec<Option<Arc<RwLock<WoogOption>>>>>>,
    parameter_free_list: std::sync::Mutex<Vec<usize>>,
    parameter: Arc<RwLock<Vec<Option<Arc<RwLock<Parameter>>>>>>,
    plain_free_list: std::sync::Mutex<Vec<usize>>,
    plain: Arc<RwLock<Vec<Option<Arc<RwLock<Plain>>>>>>,
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
    struct_field_free_list: std::sync::Mutex<Vec<usize>>,
    struct_field: Arc<RwLock<Vec<Option<Arc<RwLock<StructField>>>>>>,
    tuple_field_free_list: std::sync::Mutex<Vec<usize>>,
    tuple_field: Arc<RwLock<Vec<Option<Arc<RwLock<TupleField>>>>>>,
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
            body_free_list: std::sync::Mutex::new(Vec::new()),
            body: Arc::new(RwLock::new(Vec::new())),
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
            enum_field_free_list: std::sync::Mutex::new(Vec::new()),
            enum_field: Arc::new(RwLock::new(Vec::new())),
            enumeration_free_list: std::sync::Mutex::new(Vec::new()),
            enumeration: Arc::new(RwLock::new(Vec::new())),
            enumeration_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            error_free_list: std::sync::Mutex::new(Vec::new()),
            error: Arc::new(RwLock::new(Vec::new())),
            error_expression_free_list: std::sync::Mutex::new(Vec::new()),
            error_expression: Arc::new(RwLock::new(Vec::new())),
            expression_free_list: std::sync::Mutex::new(Vec::new()),
            expression: Arc::new(RwLock::new(Vec::new())),
            expression_statement_free_list: std::sync::Mutex::new(Vec::new()),
            expression_statement: Arc::new(RwLock::new(Vec::new())),
            external_implementation_free_list: std::sync::Mutex::new(Vec::new()),
            external_implementation: Arc::new(RwLock::new(Vec::new())),
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
            implementation_block_free_list: std::sync::Mutex::new(Vec::new()),
            implementation_block: Arc::new(RwLock::new(Vec::new())),
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
            z_object_store_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            object_wrapper_free_list: std::sync::Mutex::new(Vec::new()),
            object_wrapper: Arc::new(RwLock::new(Vec::new())),
            operator_free_list: std::sync::Mutex::new(Vec::new()),
            operator: Arc::new(RwLock::new(Vec::new())),
            woog_option_free_list: std::sync::Mutex::new(Vec::new()),
            woog_option: Arc::new(RwLock::new(Vec::new())),
            parameter_free_list: std::sync::Mutex::new(Vec::new()),
            parameter: Arc::new(RwLock::new(Vec::new())),
            plain_free_list: std::sync::Mutex::new(Vec::new()),
            plain: Arc::new(RwLock::new(Vec::new())),
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
            struct_field_free_list: std::sync::Mutex::new(Vec::new()),
            struct_field: Arc::new(RwLock::new(Vec::new())),
            tuple_field_free_list: std::sync::Mutex::new(Vec::new()),
            tuple_field: Arc::new(RwLock::new(Vec::new())),
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

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_ndrwlock_vec-object-store-methods"}}}
    /// Inter (insert) [`Argument`] into the store.
    ///
    pub fn inter_argument<F>(&mut self, argument: F) -> Arc<RwLock<Argument>>
    where
        F: Fn(usize) -> Arc<RwLock<Argument>>,
    {
        let _index = if let Some(_index) = self.argument_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.argument.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.argument.write().unwrap().push(None);
            _index
        };

        let argument = argument(_index);

        let found = if let Some(argument) = self.argument.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *argument.read().unwrap()
            } else {
                false
            }
        }) {
            argument.clone()
        } else {
            None
        };

        if let Some(argument) = found {
            log::debug!(target: "store", "found duplicate {argument:?}.");
            self.argument_free_list.lock().unwrap().push(_index);
            argument.clone()
        } else {
            log::debug!(target: "store", "interring {argument:?}.");
            self.argument.write().unwrap()[_index] = Some(argument.clone());
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
        (0..len)
            .filter(|i| self.argument.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.binary_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.binary.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.binary.write().unwrap().push(None);
            _index
        };

        let binary = binary(_index);

        let found = if let Some(binary) = self.binary.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *binary.read().unwrap()
            } else {
                false
            }
        }) {
            binary.clone()
        } else {
            None
        };

        if let Some(binary) = found {
            log::debug!(target: "store", "found duplicate {binary:?}.");
            self.binary_free_list.lock().unwrap().push(_index);
            binary.clone()
        } else {
            log::debug!(target: "store", "interring {binary:?}.");
            self.binary.write().unwrap()[_index] = Some(binary.clone());
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
        (0..len)
            .filter(|i| self.binary.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.block_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.block.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.block.write().unwrap().push(None);
            _index
        };

        let block = block(_index);

        let found = if let Some(block) = self.block.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *block.read().unwrap()
            } else {
                false
            }
        }) {
            block.clone()
        } else {
            None
        };

        if let Some(block) = found {
            log::debug!(target: "store", "found duplicate {block:?}.");
            self.block_free_list.lock().unwrap().push(_index);
            block.clone()
        } else {
            log::debug!(target: "store", "interring {block:?}.");
            self.block.write().unwrap()[_index] = Some(block.clone());
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
        (0..len)
            .filter(|i| self.block.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.block.read().unwrap()[i]
                    .as_ref()
                    .map(|block| block.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Body`] into the store.
    ///
    pub fn inter_body<F>(&mut self, body: F) -> Arc<RwLock<Body>>
    where
        F: Fn(usize) -> Arc<RwLock<Body>>,
    {
        let _index = if let Some(_index) = self.body_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.body.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.body.write().unwrap().push(None);
            _index
        };

        let body = body(_index);

        let found = if let Some(body) = self.body.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *body.read().unwrap()
            } else {
                false
            }
        }) {
            body.clone()
        } else {
            None
        };

        if let Some(body) = found {
            log::debug!(target: "store", "found duplicate {body:?}.");
            self.body_free_list.lock().unwrap().push(_index);
            body.clone()
        } else {
            log::debug!(target: "store", "interring {body:?}.");
            self.body.write().unwrap()[_index] = Some(body.clone());
            body
        }
    }

    /// Exhume (get) [`Body`] from the store.
    ///
    pub fn exhume_body(&self, id: &usize) -> Option<Arc<RwLock<Body>>> {
        match self.body.read().unwrap().get(*id) {
            Some(body) => body.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Body`] from the store.
    ///
    pub fn exorcise_body(&mut self, id: &usize) -> Option<Arc<RwLock<Body>>> {
        let result = self.body.write().unwrap()[*id].take();
        self.body_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Body>`.
    ///
    pub fn iter_body(&self) -> impl Iterator<Item = Arc<RwLock<Body>>> + '_ {
        let len = self.body.read().unwrap().len();
        (0..len)
            .filter(|i| self.body.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.body.read().unwrap()[i]
                    .as_ref()
                    .map(|body| body.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`BooleanLiteral`] into the store.
    ///
    pub fn inter_boolean_literal<F>(&mut self, boolean_literal: F) -> Arc<RwLock<BooleanLiteral>>
    where
        F: Fn(usize) -> Arc<RwLock<BooleanLiteral>>,
    {
        let _index = if let Some(_index) = self.boolean_literal_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.boolean_literal.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.boolean_literal.write().unwrap().push(None);
            _index
        };

        let boolean_literal = boolean_literal(_index);

        let found = if let Some(boolean_literal) =
            self.boolean_literal.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *boolean_literal.read().unwrap()
                } else {
                    false
                }
            }) {
            boolean_literal.clone()
        } else {
            None
        };

        if let Some(boolean_literal) = found {
            log::debug!(target: "store", "found duplicate {boolean_literal:?}.");
            self.boolean_literal_free_list.lock().unwrap().push(_index);
            boolean_literal.clone()
        } else {
            log::debug!(target: "store", "interring {boolean_literal:?}.");
            self.boolean_literal.write().unwrap()[_index] = Some(boolean_literal.clone());
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
        (0..len)
            .filter(|i| self.boolean_literal.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.boolean_operator_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.boolean_operator.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.boolean_operator.write().unwrap().push(None);
            _index
        };

        let boolean_operator = boolean_operator(_index);

        let found = if let Some(boolean_operator) =
            self.boolean_operator.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *boolean_operator.read().unwrap()
                } else {
                    false
                }
            }) {
            boolean_operator.clone()
        } else {
            None
        };

        if let Some(boolean_operator) = found {
            log::debug!(target: "store", "found duplicate {boolean_operator:?}.");
            self.boolean_operator_free_list.lock().unwrap().push(_index);
            boolean_operator.clone()
        } else {
            log::debug!(target: "store", "interring {boolean_operator:?}.");
            self.boolean_operator.write().unwrap()[_index] = Some(boolean_operator.clone());
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
        (0..len)
            .filter(|i| self.boolean_operator.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.call_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.call.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.call.write().unwrap().push(None);
            _index
        };

        let call = call(_index);

        let found = if let Some(call) = self.call.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *call.read().unwrap()
            } else {
                false
            }
        }) {
            call.clone()
        } else {
            None
        };

        if let Some(call) = found {
            log::debug!(target: "store", "found duplicate {call:?}.");
            self.call_free_list.lock().unwrap().push(_index);
            call.clone()
        } else {
            log::debug!(target: "store", "interring {call:?}.");
            self.call.write().unwrap()[_index] = Some(call.clone());
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
        (0..len)
            .filter(|i| self.call.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.comparison_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.comparison.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.comparison.write().unwrap().push(None);
            _index
        };

        let comparison = comparison(_index);

        let found = if let Some(comparison) =
            self.comparison.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *comparison.read().unwrap()
                } else {
                    false
                }
            }) {
            comparison.clone()
        } else {
            None
        };

        if let Some(comparison) = found {
            log::debug!(target: "store", "found duplicate {comparison:?}.");
            self.comparison_free_list.lock().unwrap().push(_index);
            comparison.clone()
        } else {
            log::debug!(target: "store", "interring {comparison:?}.");
            self.comparison.write().unwrap()[_index] = Some(comparison.clone());
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
        (0..len)
            .filter(|i| self.comparison.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.dwarf_source_file_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.dwarf_source_file.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.dwarf_source_file.write().unwrap().push(None);
            _index
        };

        let dwarf_source_file = dwarf_source_file(_index);

        let found = if let Some(dwarf_source_file) = self
            .dwarf_source_file
            .read()
            .unwrap()
            .iter()
            .find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *dwarf_source_file.read().unwrap()
                } else {
                    false
                }
            }) {
            dwarf_source_file.clone()
        } else {
            None
        };

        if let Some(dwarf_source_file) = found {
            log::debug!(target: "store", "found duplicate {dwarf_source_file:?}.");
            self.dwarf_source_file_free_list
                .lock()
                .unwrap()
                .push(_index);
            dwarf_source_file.clone()
        } else {
            log::debug!(target: "store", "interring {dwarf_source_file:?}.");
            self.dwarf_source_file.write().unwrap()[_index] = Some(dwarf_source_file.clone());
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
        (0..len)
            .filter(|i| self.dwarf_source_file.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.dwarf_source_file.read().unwrap()[i]
                    .as_ref()
                    .map(|dwarf_source_file| dwarf_source_file.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`EnumField`] into the store.
    ///
    pub fn inter_enum_field<F>(&mut self, enum_field: F) -> Arc<RwLock<EnumField>>
    where
        F: Fn(usize) -> Arc<RwLock<EnumField>>,
    {
        let _index = if let Some(_index) = self.enum_field_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.enum_field.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.enum_field.write().unwrap().push(None);
            _index
        };

        let enum_field = enum_field(_index);

        let found = if let Some(enum_field) =
            self.enum_field.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *enum_field.read().unwrap()
                } else {
                    false
                }
            }) {
            enum_field.clone()
        } else {
            None
        };

        if let Some(enum_field) = found {
            log::debug!(target: "store", "found duplicate {enum_field:?}.");
            self.enum_field_free_list.lock().unwrap().push(_index);
            enum_field.clone()
        } else {
            log::debug!(target: "store", "interring {enum_field:?}.");
            self.enum_field.write().unwrap()[_index] = Some(enum_field.clone());
            enum_field
        }
    }

    /// Exhume (get) [`EnumField`] from the store.
    ///
    pub fn exhume_enum_field(&self, id: &usize) -> Option<Arc<RwLock<EnumField>>> {
        match self.enum_field.read().unwrap().get(*id) {
            Some(enum_field) => enum_field.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`EnumField`] from the store.
    ///
    pub fn exorcise_enum_field(&mut self, id: &usize) -> Option<Arc<RwLock<EnumField>>> {
        let result = self.enum_field.write().unwrap()[*id].take();
        self.enum_field_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, EnumField>`.
    ///
    pub fn iter_enum_field(&self) -> impl Iterator<Item = Arc<RwLock<EnumField>>> + '_ {
        let len = self.enum_field.read().unwrap().len();
        (0..len)
            .filter(|i| self.enum_field.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.enum_field.read().unwrap()[i]
                    .as_ref()
                    .map(|enum_field| enum_field.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Enumeration`] into the store.
    ///
    pub fn inter_enumeration<F>(&mut self, enumeration: F) -> Arc<RwLock<Enumeration>>
    where
        F: Fn(usize) -> Arc<RwLock<Enumeration>>,
    {
        let _index = if let Some(_index) = self.enumeration_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.enumeration.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.enumeration.write().unwrap().push(None);
            _index
        };

        let enumeration = enumeration(_index);

        let found = if let Some(enumeration) =
            self.enumeration.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *enumeration.read().unwrap()
                } else {
                    false
                }
            }) {
            enumeration.clone()
        } else {
            None
        };

        let enumeration = if let Some(enumeration) = found {
            log::debug!(target: "store", "found duplicate {enumeration:?}.");
            self.enumeration_free_list.lock().unwrap().push(_index);
            enumeration.clone()
        } else {
            log::debug!(target: "store", "interring {enumeration:?}.");
            self.enumeration.write().unwrap()[_index] = Some(enumeration.clone());
            enumeration
        };
        self.enumeration_id_by_name.write().unwrap().insert(
            enumeration.read().unwrap().name.to_owned(),
            enumeration.read().unwrap().id,
        );
        enumeration
    }

    /// Exhume (get) [`Enumeration`] from the store.
    ///
    pub fn exhume_enumeration(&self, id: &usize) -> Option<Arc<RwLock<Enumeration>>> {
        match self.enumeration.read().unwrap().get(*id) {
            Some(enumeration) => enumeration.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Enumeration`] from the store.
    ///
    pub fn exorcise_enumeration(&mut self, id: &usize) -> Option<Arc<RwLock<Enumeration>>> {
        let result = self.enumeration.write().unwrap()[*id].take();
        self.enumeration_free_list.lock().unwrap().push(*id);
        result
    }

    /// Exorcise [`Enumeration`] id from the store by name.
    ///
    pub fn exhume_enumeration_id_by_name(&self, name: &str) -> Option<usize> {
        self.enumeration_id_by_name
            .read()
            .unwrap()
            .get(name)
            .map(|enumeration| *enumeration)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Enumeration>`.
    ///
    pub fn iter_enumeration(&self) -> impl Iterator<Item = Arc<RwLock<Enumeration>>> + '_ {
        let len = self.enumeration.read().unwrap().len();
        (0..len)
            .filter(|i| self.enumeration.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.enumeration.read().unwrap()[i]
                    .as_ref()
                    .map(|enumeration| enumeration.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Error`] into the store.
    ///
    pub fn inter_error<F>(&mut self, error: F) -> Arc<RwLock<Error>>
    where
        F: Fn(usize) -> Arc<RwLock<Error>>,
    {
        let _index = if let Some(_index) = self.error_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.error.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.error.write().unwrap().push(None);
            _index
        };

        let error = error(_index);

        let found = if let Some(error) = self.error.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *error.read().unwrap()
            } else {
                false
            }
        }) {
            error.clone()
        } else {
            None
        };

        if let Some(error) = found {
            log::debug!(target: "store", "found duplicate {error:?}.");
            self.error_free_list.lock().unwrap().push(_index);
            error.clone()
        } else {
            log::debug!(target: "store", "interring {error:?}.");
            self.error.write().unwrap()[_index] = Some(error.clone());
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
        (0..len)
            .filter(|i| self.error.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.error_expression_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.error_expression.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.error_expression.write().unwrap().push(None);
            _index
        };

        let error_expression = error_expression(_index);

        let found = if let Some(error_expression) =
            self.error_expression.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *error_expression.read().unwrap()
                } else {
                    false
                }
            }) {
            error_expression.clone()
        } else {
            None
        };

        if let Some(error_expression) = found {
            log::debug!(target: "store", "found duplicate {error_expression:?}.");
            self.error_expression_free_list.lock().unwrap().push(_index);
            error_expression.clone()
        } else {
            log::debug!(target: "store", "interring {error_expression:?}.");
            self.error_expression.write().unwrap()[_index] = Some(error_expression.clone());
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
        (0..len)
            .filter(|i| self.error_expression.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.expression_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.expression.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.expression.write().unwrap().push(None);
            _index
        };

        let expression = expression(_index);

        let found = if let Some(expression) =
            self.expression.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *expression.read().unwrap()
                } else {
                    false
                }
            }) {
            expression.clone()
        } else {
            None
        };

        if let Some(expression) = found {
            log::debug!(target: "store", "found duplicate {expression:?}.");
            self.expression_free_list.lock().unwrap().push(_index);
            expression.clone()
        } else {
            log::debug!(target: "store", "interring {expression:?}.");
            self.expression.write().unwrap()[_index] = Some(expression.clone());
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
        (0..len)
            .filter(|i| self.expression.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.expression_statement_free_list.lock().unwrap().pop()
        {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.expression_statement.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.expression_statement.write().unwrap().push(None);
            _index
        };

        let expression_statement = expression_statement(_index);

        let found = if let Some(expression_statement) = self
            .expression_statement
            .read()
            .unwrap()
            .iter()
            .find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *expression_statement.read().unwrap()
                } else {
                    false
                }
            }) {
            expression_statement.clone()
        } else {
            None
        };

        if let Some(expression_statement) = found {
            log::debug!(target: "store", "found duplicate {expression_statement:?}.");
            self.expression_statement_free_list
                .lock()
                .unwrap()
                .push(_index);
            expression_statement.clone()
        } else {
            log::debug!(target: "store", "interring {expression_statement:?}.");
            self.expression_statement.write().unwrap()[_index] = Some(expression_statement.clone());
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
        (0..len)
            .filter(|i| self.expression_statement.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.expression_statement.read().unwrap()[i]
                    .as_ref()
                    .map(|expression_statement| expression_statement.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`ExternalImplementation`] into the store.
    ///
    pub fn inter_external_implementation<F>(
        &mut self,
        external_implementation: F,
    ) -> Arc<RwLock<ExternalImplementation>>
    where
        F: Fn(usize) -> Arc<RwLock<ExternalImplementation>>,
    {
        let _index =
            if let Some(_index) = self.external_implementation_free_list.lock().unwrap().pop() {
                log::trace!(target: "store", "recycling block {_index}.");
                _index
            } else {
                let _index = self.external_implementation.read().unwrap().len();
                log::trace!(target: "store", "allocating block {_index}.");
                self.external_implementation.write().unwrap().push(None);
                _index
            };

        let external_implementation = external_implementation(_index);

        let found = if let Some(external_implementation) = self
            .external_implementation
            .read()
            .unwrap()
            .iter()
            .find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *external_implementation.read().unwrap()
                } else {
                    false
                }
            }) {
            external_implementation.clone()
        } else {
            None
        };

        if let Some(external_implementation) = found {
            log::debug!(target: "store", "found duplicate {external_implementation:?}.");
            self.external_implementation_free_list
                .lock()
                .unwrap()
                .push(_index);
            external_implementation.clone()
        } else {
            log::debug!(target: "store", "interring {external_implementation:?}.");
            self.external_implementation.write().unwrap()[_index] =
                Some(external_implementation.clone());
            external_implementation
        }
    }

    /// Exhume (get) [`ExternalImplementation`] from the store.
    ///
    pub fn exhume_external_implementation(
        &self,
        id: &usize,
    ) -> Option<Arc<RwLock<ExternalImplementation>>> {
        match self.external_implementation.read().unwrap().get(*id) {
            Some(external_implementation) => external_implementation.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ExternalImplementation`] from the store.
    ///
    pub fn exorcise_external_implementation(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<ExternalImplementation>>> {
        let result = self.external_implementation.write().unwrap()[*id].take();
        self.external_implementation_free_list
            .lock()
            .unwrap()
            .push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ExternalImplementation>`.
    ///
    pub fn iter_external_implementation(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<ExternalImplementation>>> + '_ {
        let len = self.external_implementation.read().unwrap().len();
        (0..len)
            .filter(|i| self.external_implementation.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.external_implementation.read().unwrap()[i]
                    .as_ref()
                    .map(|external_implementation| external_implementation.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Field`] into the store.
    ///
    pub fn inter_field<F>(&mut self, field: F) -> Arc<RwLock<Field>>
    where
        F: Fn(usize) -> Arc<RwLock<Field>>,
    {
        let _index = if let Some(_index) = self.field_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.field.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.field.write().unwrap().push(None);
            _index
        };

        let field = field(_index);

        let found = if let Some(field) = self.field.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *field.read().unwrap()
            } else {
                false
            }
        }) {
            field.clone()
        } else {
            None
        };

        let field = if let Some(field) = found {
            log::debug!(target: "store", "found duplicate {field:?}.");
            self.field_free_list.lock().unwrap().push(_index);
            field.clone()
        } else {
            log::debug!(target: "store", "interring {field:?}.");
            self.field.write().unwrap()[_index] = Some(field.clone());
            field
        };
        self.field_id_by_name.write().unwrap().insert(
            field.read().unwrap().name.to_owned(),
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
        (0..len)
            .filter(|i| self.field.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.field_access_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.field_access.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.field_access.write().unwrap().push(None);
            _index
        };

        let field_access = field_access(_index);

        let found = if let Some(field_access) =
            self.field_access.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *field_access.read().unwrap()
                } else {
                    false
                }
            }) {
            field_access.clone()
        } else {
            None
        };

        if let Some(field_access) = found {
            log::debug!(target: "store", "found duplicate {field_access:?}.");
            self.field_access_free_list.lock().unwrap().push(_index);
            field_access.clone()
        } else {
            log::debug!(target: "store", "interring {field_access:?}.");
            self.field_access.write().unwrap()[_index] = Some(field_access.clone());
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
        (0..len)
            .filter(|i| self.field_access.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.field_access_target_free_list.lock().unwrap().pop()
        {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.field_access_target.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.field_access_target.write().unwrap().push(None);
            _index
        };

        let field_access_target = field_access_target(_index);

        let found = if let Some(field_access_target) = self
            .field_access_target
            .read()
            .unwrap()
            .iter()
            .find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *field_access_target.read().unwrap()
                } else {
                    false
                }
            }) {
            field_access_target.clone()
        } else {
            None
        };

        if let Some(field_access_target) = found {
            log::debug!(target: "store", "found duplicate {field_access_target:?}.");
            self.field_access_target_free_list
                .lock()
                .unwrap()
                .push(_index);
            field_access_target.clone()
        } else {
            log::debug!(target: "store", "interring {field_access_target:?}.");
            self.field_access_target.write().unwrap()[_index] = Some(field_access_target.clone());
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
        (0..len)
            .filter(|i| self.field_access_target.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.field_expression_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.field_expression.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.field_expression.write().unwrap().push(None);
            _index
        };

        let field_expression = field_expression(_index);

        let found = if let Some(field_expression) =
            self.field_expression.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *field_expression.read().unwrap()
                } else {
                    false
                }
            }) {
            field_expression.clone()
        } else {
            None
        };

        if let Some(field_expression) = found {
            log::debug!(target: "store", "found duplicate {field_expression:?}.");
            self.field_expression_free_list.lock().unwrap().push(_index);
            field_expression.clone()
        } else {
            log::debug!(target: "store", "interring {field_expression:?}.");
            self.field_expression.write().unwrap()[_index] = Some(field_expression.clone());
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
        (0..len)
            .filter(|i| self.field_expression.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.float_literal_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.float_literal.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.float_literal.write().unwrap().push(None);
            _index
        };

        let float_literal = float_literal(_index);

        let found = if let Some(float_literal) =
            self.float_literal.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *float_literal.read().unwrap()
                } else {
                    false
                }
            }) {
            float_literal.clone()
        } else {
            None
        };

        if let Some(float_literal) = found {
            log::debug!(target: "store", "found duplicate {float_literal:?}.");
            self.float_literal_free_list.lock().unwrap().push(_index);
            float_literal.clone()
        } else {
            log::debug!(target: "store", "interring {float_literal:?}.");
            self.float_literal.write().unwrap()[_index] = Some(float_literal.clone());
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
        (0..len)
            .filter(|i| self.float_literal.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.for_loop_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.for_loop.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.for_loop.write().unwrap().push(None);
            _index
        };

        let for_loop = for_loop(_index);

        let found = if let Some(for_loop) = self.for_loop.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *for_loop.read().unwrap()
            } else {
                false
            }
        }) {
            for_loop.clone()
        } else {
            None
        };

        if let Some(for_loop) = found {
            log::debug!(target: "store", "found duplicate {for_loop:?}.");
            self.for_loop_free_list.lock().unwrap().push(_index);
            for_loop.clone()
        } else {
            log::debug!(target: "store", "interring {for_loop:?}.");
            self.for_loop.write().unwrap()[_index] = Some(for_loop.clone());
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
        (0..len)
            .filter(|i| self.for_loop.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.function_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.function.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.function.write().unwrap().push(None);
            _index
        };

        let function = function(_index);

        let found = if let Some(function) = self.function.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *function.read().unwrap()
            } else {
                false
            }
        }) {
            function.clone()
        } else {
            None
        };

        let function = if let Some(function) = found {
            log::debug!(target: "store", "found duplicate {function:?}.");
            self.function_free_list.lock().unwrap().push(_index);
            function.clone()
        } else {
            log::debug!(target: "store", "interring {function:?}.");
            self.function.write().unwrap()[_index] = Some(function.clone());
            function
        };
        self.function_id_by_name.write().unwrap().insert(
            function.read().unwrap().name.to_owned(),
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
        (0..len)
            .filter(|i| self.function.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.grouped_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.grouped.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.grouped.write().unwrap().push(None);
            _index
        };

        let grouped = grouped(_index);

        let found = if let Some(grouped) = self.grouped.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *grouped.read().unwrap()
            } else {
                false
            }
        }) {
            grouped.clone()
        } else {
            None
        };

        if let Some(grouped) = found {
            log::debug!(target: "store", "found duplicate {grouped:?}.");
            self.grouped_free_list.lock().unwrap().push(_index);
            grouped.clone()
        } else {
            log::debug!(target: "store", "interring {grouped:?}.");
            self.grouped.write().unwrap()[_index] = Some(grouped.clone());
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
        (0..len)
            .filter(|i| self.grouped.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.x_if_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_if.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.x_if.write().unwrap().push(None);
            _index
        };

        let x_if = x_if(_index);

        let found = if let Some(x_if) = self.x_if.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *x_if.read().unwrap()
            } else {
                false
            }
        }) {
            x_if.clone()
        } else {
            None
        };

        if let Some(x_if) = found {
            log::debug!(target: "store", "found duplicate {x_if:?}.");
            self.x_if_free_list.lock().unwrap().push(_index);
            x_if.clone()
        } else {
            log::debug!(target: "store", "interring {x_if:?}.");
            self.x_if.write().unwrap()[_index] = Some(x_if.clone());
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
        (0..len)
            .filter(|i| self.x_if.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.x_if.read().unwrap()[i]
                    .as_ref()
                    .map(|x_if| x_if.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`ImplementationBlock`] into the store.
    ///
    pub fn inter_implementation_block<F>(
        &mut self,
        implementation_block: F,
    ) -> Arc<RwLock<ImplementationBlock>>
    where
        F: Fn(usize) -> Arc<RwLock<ImplementationBlock>>,
    {
        let _index = if let Some(_index) = self.implementation_block_free_list.lock().unwrap().pop()
        {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.implementation_block.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.implementation_block.write().unwrap().push(None);
            _index
        };

        let implementation_block = implementation_block(_index);

        let found = if let Some(implementation_block) = self
            .implementation_block
            .read()
            .unwrap()
            .iter()
            .find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *implementation_block.read().unwrap()
                } else {
                    false
                }
            }) {
            implementation_block.clone()
        } else {
            None
        };

        if let Some(implementation_block) = found {
            log::debug!(target: "store", "found duplicate {implementation_block:?}.");
            self.implementation_block_free_list
                .lock()
                .unwrap()
                .push(_index);
            implementation_block.clone()
        } else {
            log::debug!(target: "store", "interring {implementation_block:?}.");
            self.implementation_block.write().unwrap()[_index] = Some(implementation_block.clone());
            implementation_block
        }
    }

    /// Exhume (get) [`ImplementationBlock`] from the store.
    ///
    pub fn exhume_implementation_block(
        &self,
        id: &usize,
    ) -> Option<Arc<RwLock<ImplementationBlock>>> {
        match self.implementation_block.read().unwrap().get(*id) {
            Some(implementation_block) => implementation_block.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ImplementationBlock`] from the store.
    ///
    pub fn exorcise_implementation_block(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<ImplementationBlock>>> {
        let result = self.implementation_block.write().unwrap()[*id].take();
        self.implementation_block_free_list
            .lock()
            .unwrap()
            .push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ImplementationBlock>`.
    ///
    pub fn iter_implementation_block(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<ImplementationBlock>>> + '_ {
        let len = self.implementation_block.read().unwrap().len();
        (0..len)
            .filter(|i| self.implementation_block.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.implementation_block.read().unwrap()[i]
                    .as_ref()
                    .map(|implementation_block| implementation_block.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Import`] into the store.
    ///
    pub fn inter_import<F>(&mut self, import: F) -> Arc<RwLock<Import>>
    where
        F: Fn(usize) -> Arc<RwLock<Import>>,
    {
        let _index = if let Some(_index) = self.import_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.import.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.import.write().unwrap().push(None);
            _index
        };

        let import = import(_index);

        let found = if let Some(import) = self.import.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *import.read().unwrap()
            } else {
                false
            }
        }) {
            import.clone()
        } else {
            None
        };

        if let Some(import) = found {
            log::debug!(target: "store", "found duplicate {import:?}.");
            self.import_free_list.lock().unwrap().push(_index);
            import.clone()
        } else {
            log::debug!(target: "store", "interring {import:?}.");
            self.import.write().unwrap()[_index] = Some(import.clone());
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
        (0..len)
            .filter(|i| self.import.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.index_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.index.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.index.write().unwrap().push(None);
            _index
        };

        let index = index(_index);

        let found = if let Some(index) = self.index.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *index.read().unwrap()
            } else {
                false
            }
        }) {
            index.clone()
        } else {
            None
        };

        if let Some(index) = found {
            log::debug!(target: "store", "found duplicate {index:?}.");
            self.index_free_list.lock().unwrap().push(_index);
            index.clone()
        } else {
            log::debug!(target: "store", "interring {index:?}.");
            self.index.write().unwrap()[_index] = Some(index.clone());
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
        (0..len)
            .filter(|i| self.index.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.integer_literal_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.integer_literal.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.integer_literal.write().unwrap().push(None);
            _index
        };

        let integer_literal = integer_literal(_index);

        let found = if let Some(integer_literal) =
            self.integer_literal.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *integer_literal.read().unwrap()
                } else {
                    false
                }
            }) {
            integer_literal.clone()
        } else {
            None
        };

        if let Some(integer_literal) = found {
            log::debug!(target: "store", "found duplicate {integer_literal:?}.");
            self.integer_literal_free_list.lock().unwrap().push(_index);
            integer_literal.clone()
        } else {
            log::debug!(target: "store", "interring {integer_literal:?}.");
            self.integer_literal.write().unwrap()[_index] = Some(integer_literal.clone());
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
        (0..len)
            .filter(|i| self.integer_literal.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.item_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.item.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.item.write().unwrap().push(None);
            _index
        };

        let item = item(_index);

        let found = if let Some(item) = self.item.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *item.read().unwrap()
            } else {
                false
            }
        }) {
            item.clone()
        } else {
            None
        };

        if let Some(item) = found {
            log::debug!(target: "store", "found duplicate {item:?}.");
            self.item_free_list.lock().unwrap().push(_index);
            item.clone()
        } else {
            log::debug!(target: "store", "interring {item:?}.");
            self.item.write().unwrap()[_index] = Some(item.clone());
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
        (0..len)
            .filter(|i| self.item.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.lambda_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.lambda.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.lambda.write().unwrap().push(None);
            _index
        };

        let lambda = lambda(_index);

        let found = if let Some(lambda) = self.lambda.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *lambda.read().unwrap()
            } else {
                false
            }
        }) {
            lambda.clone()
        } else {
            None
        };

        if let Some(lambda) = found {
            log::debug!(target: "store", "found duplicate {lambda:?}.");
            self.lambda_free_list.lock().unwrap().push(_index);
            lambda.clone()
        } else {
            log::debug!(target: "store", "interring {lambda:?}.");
            self.lambda.write().unwrap()[_index] = Some(lambda.clone());
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
        (0..len)
            .filter(|i| self.lambda.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.lambda_parameter_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.lambda_parameter.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.lambda_parameter.write().unwrap().push(None);
            _index
        };

        let lambda_parameter = lambda_parameter(_index);

        let found = if let Some(lambda_parameter) =
            self.lambda_parameter.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *lambda_parameter.read().unwrap()
                } else {
                    false
                }
            }) {
            lambda_parameter.clone()
        } else {
            None
        };

        if let Some(lambda_parameter) = found {
            log::debug!(target: "store", "found duplicate {lambda_parameter:?}.");
            self.lambda_parameter_free_list.lock().unwrap().push(_index);
            lambda_parameter.clone()
        } else {
            log::debug!(target: "store", "interring {lambda_parameter:?}.");
            self.lambda_parameter.write().unwrap()[_index] = Some(lambda_parameter.clone());
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
        (0..len)
            .filter(|i| self.lambda_parameter.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.let_statement_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.let_statement.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.let_statement.write().unwrap().push(None);
            _index
        };

        let let_statement = let_statement(_index);

        let found = if let Some(let_statement) =
            self.let_statement.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *let_statement.read().unwrap()
                } else {
                    false
                }
            }) {
            let_statement.clone()
        } else {
            None
        };

        if let Some(let_statement) = found {
            log::debug!(target: "store", "found duplicate {let_statement:?}.");
            self.let_statement_free_list.lock().unwrap().push(_index);
            let_statement.clone()
        } else {
            log::debug!(target: "store", "interring {let_statement:?}.");
            self.let_statement.write().unwrap()[_index] = Some(let_statement.clone());
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
        (0..len)
            .filter(|i| self.let_statement.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.list_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.list.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.list.write().unwrap().push(None);
            _index
        };

        let list = list(_index);

        let found = if let Some(list) = self.list.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *list.read().unwrap()
            } else {
                false
            }
        }) {
            list.clone()
        } else {
            None
        };

        if let Some(list) = found {
            log::debug!(target: "store", "found duplicate {list:?}.");
            self.list_free_list.lock().unwrap().push(_index);
            list.clone()
        } else {
            log::debug!(target: "store", "interring {list:?}.");
            self.list.write().unwrap()[_index] = Some(list.clone());
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
        (0..len)
            .filter(|i| self.list.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.list_element_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.list_element.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.list_element.write().unwrap().push(None);
            _index
        };

        let list_element = list_element(_index);

        let found = if let Some(list_element) =
            self.list_element.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *list_element.read().unwrap()
                } else {
                    false
                }
            }) {
            list_element.clone()
        } else {
            None
        };

        if let Some(list_element) = found {
            log::debug!(target: "store", "found duplicate {list_element:?}.");
            self.list_element_free_list.lock().unwrap().push(_index);
            list_element.clone()
        } else {
            log::debug!(target: "store", "interring {list_element:?}.");
            self.list_element.write().unwrap()[_index] = Some(list_element.clone());
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
        (0..len)
            .filter(|i| self.list_element.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.list_expression_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.list_expression.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.list_expression.write().unwrap().push(None);
            _index
        };

        let list_expression = list_expression(_index);

        let found = if let Some(list_expression) =
            self.list_expression.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *list_expression.read().unwrap()
                } else {
                    false
                }
            }) {
            list_expression.clone()
        } else {
            None
        };

        if let Some(list_expression) = found {
            log::debug!(target: "store", "found duplicate {list_expression:?}.");
            self.list_expression_free_list.lock().unwrap().push(_index);
            list_expression.clone()
        } else {
            log::debug!(target: "store", "interring {list_expression:?}.");
            self.list_expression.write().unwrap()[_index] = Some(list_expression.clone());
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
        (0..len)
            .filter(|i| self.list_expression.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.literal_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.literal.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.literal.write().unwrap().push(None);
            _index
        };

        let literal = literal(_index);

        let found = if let Some(literal) = self.literal.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *literal.read().unwrap()
            } else {
                false
            }
        }) {
            literal.clone()
        } else {
            None
        };

        if let Some(literal) = found {
            log::debug!(target: "store", "found duplicate {literal:?}.");
            self.literal_free_list.lock().unwrap().push(_index);
            literal.clone()
        } else {
            log::debug!(target: "store", "interring {literal:?}.");
            self.literal.write().unwrap()[_index] = Some(literal.clone());
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
        (0..len)
            .filter(|i| self.literal.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.local_variable_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.local_variable.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.local_variable.write().unwrap().push(None);
            _index
        };

        let local_variable = local_variable(_index);

        let found = if let Some(local_variable) =
            self.local_variable.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *local_variable.read().unwrap()
                } else {
                    false
                }
            }) {
            local_variable.clone()
        } else {
            None
        };

        if let Some(local_variable) = found {
            log::debug!(target: "store", "found duplicate {local_variable:?}.");
            self.local_variable_free_list.lock().unwrap().push(_index);
            local_variable.clone()
        } else {
            log::debug!(target: "store", "interring {local_variable:?}.");
            self.local_variable.write().unwrap()[_index] = Some(local_variable.clone());
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
        (0..len)
            .filter(|i| self.local_variable.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.x_macro_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_macro.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.x_macro.write().unwrap().push(None);
            _index
        };

        let x_macro = x_macro(_index);

        let found = if let Some(x_macro) = self.x_macro.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *x_macro.read().unwrap()
            } else {
                false
            }
        }) {
            x_macro.clone()
        } else {
            None
        };

        if let Some(x_macro) = found {
            log::debug!(target: "store", "found duplicate {x_macro:?}.");
            self.x_macro_free_list.lock().unwrap().push(_index);
            x_macro.clone()
        } else {
            log::debug!(target: "store", "interring {x_macro:?}.");
            self.x_macro.write().unwrap()[_index] = Some(x_macro.clone());
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
        (0..len)
            .filter(|i| self.x_macro.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.method_call_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.method_call.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.method_call.write().unwrap().push(None);
            _index
        };

        let method_call = method_call(_index);

        let found = if let Some(method_call) =
            self.method_call.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *method_call.read().unwrap()
                } else {
                    false
                }
            }) {
            method_call.clone()
        } else {
            None
        };

        if let Some(method_call) = found {
            log::debug!(target: "store", "found duplicate {method_call:?}.");
            self.method_call_free_list.lock().unwrap().push(_index);
            method_call.clone()
        } else {
            log::debug!(target: "store", "interring {method_call:?}.");
            self.method_call.write().unwrap()[_index] = Some(method_call.clone());
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
        (0..len)
            .filter(|i| self.method_call.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.z_object_store_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.z_object_store.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.z_object_store.write().unwrap().push(None);
            _index
        };

        let z_object_store = z_object_store(_index);

        let found = if let Some(z_object_store) =
            self.z_object_store.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *z_object_store.read().unwrap()
                } else {
                    false
                }
            }) {
            z_object_store.clone()
        } else {
            None
        };

        let z_object_store = if let Some(z_object_store) = found {
            log::debug!(target: "store", "found duplicate {z_object_store:?}.");
            self.z_object_store_free_list.lock().unwrap().push(_index);
            z_object_store.clone()
        } else {
            log::debug!(target: "store", "interring {z_object_store:?}.");
            self.z_object_store.write().unwrap()[_index] = Some(z_object_store.clone());
            z_object_store
        };
        self.z_object_store_id_by_name.write().unwrap().insert(
            z_object_store.read().unwrap().name.to_owned(),
            z_object_store.read().unwrap().id,
        );
        z_object_store
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

    /// Exorcise [`ZObjectStore`] id from the store by name.
    ///
    pub fn exhume_z_object_store_id_by_name(&self, name: &str) -> Option<usize> {
        self.z_object_store_id_by_name
            .read()
            .unwrap()
            .get(name)
            .map(|z_object_store| *z_object_store)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ZObjectStore>`.
    ///
    pub fn iter_z_object_store(&self) -> impl Iterator<Item = Arc<RwLock<ZObjectStore>>> + '_ {
        let len = self.z_object_store.read().unwrap().len();
        (0..len)
            .filter(|i| self.z_object_store.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.z_object_store.read().unwrap()[i]
                    .as_ref()
                    .map(|z_object_store| z_object_store.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`ObjectWrapper`] into the store.
    ///
    pub fn inter_object_wrapper<F>(&mut self, object_wrapper: F) -> Arc<RwLock<ObjectWrapper>>
    where
        F: Fn(usize) -> Arc<RwLock<ObjectWrapper>>,
    {
        let _index = if let Some(_index) = self.object_wrapper_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.object_wrapper.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.object_wrapper.write().unwrap().push(None);
            _index
        };

        let object_wrapper = object_wrapper(_index);

        let found = if let Some(object_wrapper) =
            self.object_wrapper.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *object_wrapper.read().unwrap()
                } else {
                    false
                }
            }) {
            object_wrapper.clone()
        } else {
            None
        };

        if let Some(object_wrapper) = found {
            log::debug!(target: "store", "found duplicate {object_wrapper:?}.");
            self.object_wrapper_free_list.lock().unwrap().push(_index);
            object_wrapper.clone()
        } else {
            log::debug!(target: "store", "interring {object_wrapper:?}.");
            self.object_wrapper.write().unwrap()[_index] = Some(object_wrapper.clone());
            object_wrapper
        }
    }

    /// Exhume (get) [`ObjectWrapper`] from the store.
    ///
    pub fn exhume_object_wrapper(&self, id: &usize) -> Option<Arc<RwLock<ObjectWrapper>>> {
        match self.object_wrapper.read().unwrap().get(*id) {
            Some(object_wrapper) => object_wrapper.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ObjectWrapper`] from the store.
    ///
    pub fn exorcise_object_wrapper(&mut self, id: &usize) -> Option<Arc<RwLock<ObjectWrapper>>> {
        let result = self.object_wrapper.write().unwrap()[*id].take();
        self.object_wrapper_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ObjectWrapper>`.
    ///
    pub fn iter_object_wrapper(&self) -> impl Iterator<Item = Arc<RwLock<ObjectWrapper>>> + '_ {
        let len = self.object_wrapper.read().unwrap().len();
        (0..len)
            .filter(|i| self.object_wrapper.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.object_wrapper.read().unwrap()[i]
                    .as_ref()
                    .map(|object_wrapper| object_wrapper.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Operator`] into the store.
    ///
    pub fn inter_operator<F>(&mut self, operator: F) -> Arc<RwLock<Operator>>
    where
        F: Fn(usize) -> Arc<RwLock<Operator>>,
    {
        let _index = if let Some(_index) = self.operator_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.operator.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.operator.write().unwrap().push(None);
            _index
        };

        let operator = operator(_index);

        let found = if let Some(operator) = self.operator.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *operator.read().unwrap()
            } else {
                false
            }
        }) {
            operator.clone()
        } else {
            None
        };

        if let Some(operator) = found {
            log::debug!(target: "store", "found duplicate {operator:?}.");
            self.operator_free_list.lock().unwrap().push(_index);
            operator.clone()
        } else {
            log::debug!(target: "store", "interring {operator:?}.");
            self.operator.write().unwrap()[_index] = Some(operator.clone());
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
        (0..len)
            .filter(|i| self.operator.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.woog_option_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.woog_option.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.woog_option.write().unwrap().push(None);
            _index
        };

        let woog_option = woog_option(_index);

        let found = if let Some(woog_option) =
            self.woog_option.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *woog_option.read().unwrap()
                } else {
                    false
                }
            }) {
            woog_option.clone()
        } else {
            None
        };

        if let Some(woog_option) = found {
            log::debug!(target: "store", "found duplicate {woog_option:?}.");
            self.woog_option_free_list.lock().unwrap().push(_index);
            woog_option.clone()
        } else {
            log::debug!(target: "store", "interring {woog_option:?}.");
            self.woog_option.write().unwrap()[_index] = Some(woog_option.clone());
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
        (0..len)
            .filter(|i| self.woog_option.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.parameter_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.parameter.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.parameter.write().unwrap().push(None);
            _index
        };

        let parameter = parameter(_index);

        let found = if let Some(parameter) = self.parameter.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *parameter.read().unwrap()
            } else {
                false
            }
        }) {
            parameter.clone()
        } else {
            None
        };

        if let Some(parameter) = found {
            log::debug!(target: "store", "found duplicate {parameter:?}.");
            self.parameter_free_list.lock().unwrap().push(_index);
            parameter.clone()
        } else {
            log::debug!(target: "store", "interring {parameter:?}.");
            self.parameter.write().unwrap()[_index] = Some(parameter.clone());
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
        (0..len)
            .filter(|i| self.parameter.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.parameter.read().unwrap()[i]
                    .as_ref()
                    .map(|parameter| parameter.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Plain`] into the store.
    ///
    pub fn inter_plain<F>(&mut self, plain: F) -> Arc<RwLock<Plain>>
    where
        F: Fn(usize) -> Arc<RwLock<Plain>>,
    {
        let _index = if let Some(_index) = self.plain_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.plain.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.plain.write().unwrap().push(None);
            _index
        };

        let plain = plain(_index);

        let found = if let Some(plain) = self.plain.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *plain.read().unwrap()
            } else {
                false
            }
        }) {
            plain.clone()
        } else {
            None
        };

        if let Some(plain) = found {
            log::debug!(target: "store", "found duplicate {plain:?}.");
            self.plain_free_list.lock().unwrap().push(_index);
            plain.clone()
        } else {
            log::debug!(target: "store", "interring {plain:?}.");
            self.plain.write().unwrap()[_index] = Some(plain.clone());
            plain
        }
    }

    /// Exhume (get) [`Plain`] from the store.
    ///
    pub fn exhume_plain(&self, id: &usize) -> Option<Arc<RwLock<Plain>>> {
        match self.plain.read().unwrap().get(*id) {
            Some(plain) => plain.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Plain`] from the store.
    ///
    pub fn exorcise_plain(&mut self, id: &usize) -> Option<Arc<RwLock<Plain>>> {
        let result = self.plain.write().unwrap()[*id].take();
        self.plain_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Plain>`.
    ///
    pub fn iter_plain(&self) -> impl Iterator<Item = Arc<RwLock<Plain>>> + '_ {
        let len = self.plain.read().unwrap().len();
        (0..len)
            .filter(|i| self.plain.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.plain.read().unwrap()[i]
                    .as_ref()
                    .map(|plain| plain.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Print`] into the store.
    ///
    pub fn inter_print<F>(&mut self, print: F) -> Arc<RwLock<Print>>
    where
        F: Fn(usize) -> Arc<RwLock<Print>>,
    {
        let _index = if let Some(_index) = self.print_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.print.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.print.write().unwrap().push(None);
            _index
        };

        let print = print(_index);

        let found = if let Some(print) = self.print.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *print.read().unwrap()
            } else {
                false
            }
        }) {
            print.clone()
        } else {
            None
        };

        if let Some(print) = found {
            log::debug!(target: "store", "found duplicate {print:?}.");
            self.print_free_list.lock().unwrap().push(_index);
            print.clone()
        } else {
            log::debug!(target: "store", "interring {print:?}.");
            self.print.write().unwrap()[_index] = Some(print.clone());
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
        (0..len)
            .filter(|i| self.print.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.range_expression_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.range_expression.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.range_expression.write().unwrap().push(None);
            _index
        };

        let range_expression = range_expression(_index);

        let found = if let Some(range_expression) =
            self.range_expression.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *range_expression.read().unwrap()
                } else {
                    false
                }
            }) {
            range_expression.clone()
        } else {
            None
        };

        if let Some(range_expression) = found {
            log::debug!(target: "store", "found duplicate {range_expression:?}.");
            self.range_expression_free_list.lock().unwrap().push(_index);
            range_expression.clone()
        } else {
            log::debug!(target: "store", "interring {range_expression:?}.");
            self.range_expression.write().unwrap()[_index] = Some(range_expression.clone());
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
        (0..len)
            .filter(|i| self.range_expression.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.reference_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.reference.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.reference.write().unwrap().push(None);
            _index
        };

        let reference = reference(_index);

        let found = if let Some(reference) = self.reference.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *reference.read().unwrap()
            } else {
                false
            }
        }) {
            reference.clone()
        } else {
            None
        };

        if let Some(reference) = found {
            log::debug!(target: "store", "found duplicate {reference:?}.");
            self.reference_free_list.lock().unwrap().push(_index);
            reference.clone()
        } else {
            log::debug!(target: "store", "interring {reference:?}.");
            self.reference.write().unwrap()[_index] = Some(reference.clone());
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
        (0..len)
            .filter(|i| self.reference.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.result_statement_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.result_statement.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.result_statement.write().unwrap().push(None);
            _index
        };

        let result_statement = result_statement(_index);

        let found = if let Some(result_statement) =
            self.result_statement.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *result_statement.read().unwrap()
                } else {
                    false
                }
            }) {
            result_statement.clone()
        } else {
            None
        };

        if let Some(result_statement) = found {
            log::debug!(target: "store", "found duplicate {result_statement:?}.");
            self.result_statement_free_list.lock().unwrap().push(_index);
            result_statement.clone()
        } else {
            log::debug!(target: "store", "interring {result_statement:?}.");
            self.result_statement.write().unwrap()[_index] = Some(result_statement.clone());
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
        (0..len)
            .filter(|i| self.result_statement.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.x_return_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_return.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.x_return.write().unwrap().push(None);
            _index
        };

        let x_return = x_return(_index);

        let found = if let Some(x_return) = self.x_return.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *x_return.read().unwrap()
            } else {
                false
            }
        }) {
            x_return.clone()
        } else {
            None
        };

        if let Some(x_return) = found {
            log::debug!(target: "store", "found duplicate {x_return:?}.");
            self.x_return_free_list.lock().unwrap().push(_index);
            x_return.clone()
        } else {
            log::debug!(target: "store", "interring {x_return:?}.");
            self.x_return.write().unwrap()[_index] = Some(x_return.clone());
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
        (0..len)
            .filter(|i| self.x_return.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.z_some_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.z_some.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.z_some.write().unwrap().push(None);
            _index
        };

        let z_some = z_some(_index);

        let found = if let Some(z_some) = self.z_some.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *z_some.read().unwrap()
            } else {
                false
            }
        }) {
            z_some.clone()
        } else {
            None
        };

        if let Some(z_some) = found {
            log::debug!(target: "store", "found duplicate {z_some:?}.");
            self.z_some_free_list.lock().unwrap().push(_index);
            z_some.clone()
        } else {
            log::debug!(target: "store", "interring {z_some:?}.");
            self.z_some.write().unwrap()[_index] = Some(z_some.clone());
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
        (0..len)
            .filter(|i| self.z_some.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.span_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.span.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.span.write().unwrap().push(None);
            _index
        };

        let span = span(_index);

        let found = if let Some(span) = self.span.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *span.read().unwrap()
            } else {
                false
            }
        }) {
            span.clone()
        } else {
            None
        };

        if let Some(span) = found {
            log::debug!(target: "store", "found duplicate {span:?}.");
            self.span_free_list.lock().unwrap().push(_index);
            span.clone()
        } else {
            log::debug!(target: "store", "interring {span:?}.");
            self.span.write().unwrap()[_index] = Some(span.clone());
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
        (0..len)
            .filter(|i| self.span.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.statement_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.statement.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.statement.write().unwrap().push(None);
            _index
        };

        let statement = statement(_index);

        let found = if let Some(statement) = self.statement.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *statement.read().unwrap()
            } else {
                false
            }
        }) {
            statement.clone()
        } else {
            None
        };

        if let Some(statement) = found {
            log::debug!(target: "store", "found duplicate {statement:?}.");
            self.statement_free_list.lock().unwrap().push(_index);
            statement.clone()
        } else {
            log::debug!(target: "store", "interring {statement:?}.");
            self.statement.write().unwrap()[_index] = Some(statement.clone());
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
        (0..len)
            .filter(|i| self.statement.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.static_method_call_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.static_method_call.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.static_method_call.write().unwrap().push(None);
            _index
        };

        let static_method_call = static_method_call(_index);

        let found = if let Some(static_method_call) = self
            .static_method_call
            .read()
            .unwrap()
            .iter()
            .find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *static_method_call.read().unwrap()
                } else {
                    false
                }
            }) {
            static_method_call.clone()
        } else {
            None
        };

        if let Some(static_method_call) = found {
            log::debug!(target: "store", "found duplicate {static_method_call:?}.");
            self.static_method_call_free_list
                .lock()
                .unwrap()
                .push(_index);
            static_method_call.clone()
        } else {
            log::debug!(target: "store", "interring {static_method_call:?}.");
            self.static_method_call.write().unwrap()[_index] = Some(static_method_call.clone());
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
        (0..len)
            .filter(|i| self.static_method_call.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.string_literal_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.string_literal.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.string_literal.write().unwrap().push(None);
            _index
        };

        let string_literal = string_literal(_index);

        let found = if let Some(string_literal) =
            self.string_literal.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *string_literal.read().unwrap()
                } else {
                    false
                }
            }) {
            string_literal.clone()
        } else {
            None
        };

        if let Some(string_literal) = found {
            log::debug!(target: "store", "found duplicate {string_literal:?}.");
            self.string_literal_free_list.lock().unwrap().push(_index);
            string_literal.clone()
        } else {
            log::debug!(target: "store", "interring {string_literal:?}.");
            self.string_literal.write().unwrap()[_index] = Some(string_literal.clone());
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
        (0..len)
            .filter(|i| self.string_literal.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.woog_struct_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.woog_struct.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.woog_struct.write().unwrap().push(None);
            _index
        };

        let woog_struct = woog_struct(_index);

        let found = if let Some(woog_struct) =
            self.woog_struct.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *woog_struct.read().unwrap()
                } else {
                    false
                }
            }) {
            woog_struct.clone()
        } else {
            None
        };

        let woog_struct = if let Some(woog_struct) = found {
            log::debug!(target: "store", "found duplicate {woog_struct:?}.");
            self.woog_struct_free_list.lock().unwrap().push(_index);
            woog_struct.clone()
        } else {
            log::debug!(target: "store", "interring {woog_struct:?}.");
            self.woog_struct.write().unwrap()[_index] = Some(woog_struct.clone());
            woog_struct
        };
        self.woog_struct_id_by_name.write().unwrap().insert(
            woog_struct.read().unwrap().name.to_owned(),
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
        (0..len)
            .filter(|i| self.woog_struct.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.struct_expression_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.struct_expression.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.struct_expression.write().unwrap().push(None);
            _index
        };

        let struct_expression = struct_expression(_index);

        let found = if let Some(struct_expression) = self
            .struct_expression
            .read()
            .unwrap()
            .iter()
            .find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *struct_expression.read().unwrap()
                } else {
                    false
                }
            }) {
            struct_expression.clone()
        } else {
            None
        };

        if let Some(struct_expression) = found {
            log::debug!(target: "store", "found duplicate {struct_expression:?}.");
            self.struct_expression_free_list
                .lock()
                .unwrap()
                .push(_index);
            struct_expression.clone()
        } else {
            log::debug!(target: "store", "interring {struct_expression:?}.");
            self.struct_expression.write().unwrap()[_index] = Some(struct_expression.clone());
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
        (0..len)
            .filter(|i| self.struct_expression.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.struct_expression.read().unwrap()[i]
                    .as_ref()
                    .map(|struct_expression| struct_expression.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`StructField`] into the store.
    ///
    pub fn inter_struct_field<F>(&mut self, struct_field: F) -> Arc<RwLock<StructField>>
    where
        F: Fn(usize) -> Arc<RwLock<StructField>>,
    {
        let _index = if let Some(_index) = self.struct_field_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.struct_field.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.struct_field.write().unwrap().push(None);
            _index
        };

        let struct_field = struct_field(_index);

        let found = if let Some(struct_field) =
            self.struct_field.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *struct_field.read().unwrap()
                } else {
                    false
                }
            }) {
            struct_field.clone()
        } else {
            None
        };

        if let Some(struct_field) = found {
            log::debug!(target: "store", "found duplicate {struct_field:?}.");
            self.struct_field_free_list.lock().unwrap().push(_index);
            struct_field.clone()
        } else {
            log::debug!(target: "store", "interring {struct_field:?}.");
            self.struct_field.write().unwrap()[_index] = Some(struct_field.clone());
            struct_field
        }
    }

    /// Exhume (get) [`StructField`] from the store.
    ///
    pub fn exhume_struct_field(&self, id: &usize) -> Option<Arc<RwLock<StructField>>> {
        match self.struct_field.read().unwrap().get(*id) {
            Some(struct_field) => struct_field.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`StructField`] from the store.
    ///
    pub fn exorcise_struct_field(&mut self, id: &usize) -> Option<Arc<RwLock<StructField>>> {
        let result = self.struct_field.write().unwrap()[*id].take();
        self.struct_field_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructField>`.
    ///
    pub fn iter_struct_field(&self) -> impl Iterator<Item = Arc<RwLock<StructField>>> + '_ {
        let len = self.struct_field.read().unwrap().len();
        (0..len)
            .filter(|i| self.struct_field.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.struct_field.read().unwrap()[i]
                    .as_ref()
                    .map(|struct_field| struct_field.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`TupleField`] into the store.
    ///
    pub fn inter_tuple_field<F>(&mut self, tuple_field: F) -> Arc<RwLock<TupleField>>
    where
        F: Fn(usize) -> Arc<RwLock<TupleField>>,
    {
        let _index = if let Some(_index) = self.tuple_field_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.tuple_field.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.tuple_field.write().unwrap().push(None);
            _index
        };

        let tuple_field = tuple_field(_index);

        let found = if let Some(tuple_field) =
            self.tuple_field.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *tuple_field.read().unwrap()
                } else {
                    false
                }
            }) {
            tuple_field.clone()
        } else {
            None
        };

        if let Some(tuple_field) = found {
            log::debug!(target: "store", "found duplicate {tuple_field:?}.");
            self.tuple_field_free_list.lock().unwrap().push(_index);
            tuple_field.clone()
        } else {
            log::debug!(target: "store", "interring {tuple_field:?}.");
            self.tuple_field.write().unwrap()[_index] = Some(tuple_field.clone());
            tuple_field
        }
    }

    /// Exhume (get) [`TupleField`] from the store.
    ///
    pub fn exhume_tuple_field(&self, id: &usize) -> Option<Arc<RwLock<TupleField>>> {
        match self.tuple_field.read().unwrap().get(*id) {
            Some(tuple_field) => tuple_field.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`TupleField`] from the store.
    ///
    pub fn exorcise_tuple_field(&mut self, id: &usize) -> Option<Arc<RwLock<TupleField>>> {
        let result = self.tuple_field.write().unwrap()[*id].take();
        self.tuple_field_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, TupleField>`.
    ///
    pub fn iter_tuple_field(&self) -> impl Iterator<Item = Arc<RwLock<TupleField>>> + '_ {
        let len = self.tuple_field.read().unwrap().len();
        (0..len)
            .filter(|i| self.tuple_field.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.tuple_field.read().unwrap()[i]
                    .as_ref()
                    .map(|tuple_field| tuple_field.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`TypeCast`] into the store.
    ///
    pub fn inter_type_cast<F>(&mut self, type_cast: F) -> Arc<RwLock<TypeCast>>
    where
        F: Fn(usize) -> Arc<RwLock<TypeCast>>,
    {
        let _index = if let Some(_index) = self.type_cast_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.type_cast.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.type_cast.write().unwrap().push(None);
            _index
        };

        let type_cast = type_cast(_index);

        let found = if let Some(type_cast) = self.type_cast.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *type_cast.read().unwrap()
            } else {
                false
            }
        }) {
            type_cast.clone()
        } else {
            None
        };

        if let Some(type_cast) = found {
            log::debug!(target: "store", "found duplicate {type_cast:?}.");
            self.type_cast_free_list.lock().unwrap().push(_index);
            type_cast.clone()
        } else {
            log::debug!(target: "store", "interring {type_cast:?}.");
            self.type_cast.write().unwrap()[_index] = Some(type_cast.clone());
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
        (0..len)
            .filter(|i| self.type_cast.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.unary_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.unary.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.unary.write().unwrap().push(None);
            _index
        };

        let unary = unary(_index);

        let found = if let Some(unary) = self.unary.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *unary.read().unwrap()
            } else {
                false
            }
        }) {
            unary.clone()
        } else {
            None
        };

        if let Some(unary) = found {
            log::debug!(target: "store", "found duplicate {unary:?}.");
            self.unary_free_list.lock().unwrap().push(_index);
            unary.clone()
        } else {
            log::debug!(target: "store", "interring {unary:?}.");
            self.unary.write().unwrap()[_index] = Some(unary.clone());
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
        (0..len)
            .filter(|i| self.unary.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.x_value_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_value.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.x_value.write().unwrap().push(None);
            _index
        };

        let x_value = x_value(_index);

        let found = if let Some(x_value) = self.x_value.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *x_value.read().unwrap()
            } else {
                false
            }
        }) {
            x_value.clone()
        } else {
            None
        };

        if let Some(x_value) = found {
            log::debug!(target: "store", "found duplicate {x_value:?}.");
            self.x_value_free_list.lock().unwrap().push(_index);
            x_value.clone()
        } else {
            log::debug!(target: "store", "interring {x_value:?}.");
            self.x_value.write().unwrap()[_index] = Some(x_value.clone());
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
        (0..len)
            .filter(|i| self.x_value.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.value_type_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.value_type.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.value_type.write().unwrap().push(None);
            _index
        };

        let value_type = value_type(_index);

        let found = if let Some(value_type) =
            self.value_type.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *value_type.read().unwrap()
                } else {
                    false
                }
            }) {
            value_type.clone()
        } else {
            None
        };

        if let Some(value_type) = found {
            log::debug!(target: "store", "found duplicate {value_type:?}.");
            self.value_type_free_list.lock().unwrap().push(_index);
            value_type.clone()
        } else {
            log::debug!(target: "store", "interring {value_type:?}.");
            self.value_type.write().unwrap()[_index] = Some(value_type.clone());
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
        (0..len)
            .filter(|i| self.value_type.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.variable_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.variable.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.variable.write().unwrap().push(None);
            _index
        };

        let variable = variable(_index);

        let found = if let Some(variable) = self.variable.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *variable.read().unwrap()
            } else {
                false
            }
        }) {
            variable.clone()
        } else {
            None
        };

        if let Some(variable) = found {
            log::debug!(target: "store", "found duplicate {variable:?}.");
            self.variable_free_list.lock().unwrap().push(_index);
            variable.clone()
        } else {
            log::debug!(target: "store", "interring {variable:?}.");
            self.variable.write().unwrap()[_index] = Some(variable.clone());
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
        (0..len)
            .filter(|i| self.variable.read().unwrap()[*i].is_some())
            .map(move |i| {
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
        let _index = if let Some(_index) = self.variable_expression_free_list.lock().unwrap().pop()
        {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.variable_expression.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.variable_expression.write().unwrap().push(None);
            _index
        };

        let variable_expression = variable_expression(_index);

        let found = if let Some(variable_expression) = self
            .variable_expression
            .read()
            .unwrap()
            .iter()
            .find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *variable_expression.read().unwrap()
                } else {
                    false
                }
            }) {
            variable_expression.clone()
        } else {
            None
        };

        if let Some(variable_expression) = found {
            log::debug!(target: "store", "found duplicate {variable_expression:?}.");
            self.variable_expression_free_list
                .lock()
                .unwrap()
                .push(_index);
            variable_expression.clone()
        } else {
            log::debug!(target: "store", "interring {variable_expression:?}.");
            self.variable_expression.write().unwrap()[_index] = Some(variable_expression.clone());
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
        (0..len)
            .filter(|i| self.variable_expression.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.variable_expression.read().unwrap()[i]
                    .as_ref()
                    .map(|variable_expression| variable_expression.clone())
                    .unwrap()
            })
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_ndrwlock_vec-object-store-persistence"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
