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
//! * [`AWait`]
//! * [`Binary`]
//! * [`Block`]
//! * [`Body`]
//! * [`BooleanLiteral`]
//! * [`BooleanOperator`]
//! * [`Call`]
//! * [`Comparison`]
//! * [`DataStructure`]
//! * [`DwarfSourceFile`]
//! * [`EnumField`]
//! * [`EnumGeneric`]
//! * [`Enumeration`]
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
//! * [`FunctionCall`]
//! * [`XFuture`]
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
//! * [`NamedFieldExpression`]
//! * [`ZObjectStore`]
//! * [`ObjectWrapper`]
//! * [`Operator`]
//! * [`Parameter`]
//! * [`XPath`]
//! * [`PathElement`]
//! * [`Pattern`]
//! * [`XPlugin`]
//! * [`XPrint`]
//! * [`RangeExpression`]
//! * [`ResultStatement`]
//! * [`XReturn`]
//! * [`Span`]
//! * [`Statement`]
//! * [`StaticMethodCall`]
//! * [`StringLiteral`]
//! * [`WoogStruct`]
//! * [`StructExpression`]
//! * [`StructField`]
//! * [`StructGeneric`]
//! * [`TupleField`]
//! * [`TypeCast`]
//! * [`Unary`]
//! * [`Unit`]
//! * [`UnnamedFieldExpression`]
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

use heck::ToUpperCamelCase;
use rustc_hash::FxHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v2::lu_dog_rwlock_vec::types::{
    AWait, Argument, Binary, Block, Body, BooleanLiteral, BooleanOperator, Call, Comparison,
    DataStructure, DwarfSourceFile, EnumField, EnumGeneric, Enumeration, Expression,
    ExpressionStatement, ExternalImplementation, Field, FieldAccess, FieldAccessTarget,
    FieldExpression, FloatLiteral, ForLoop, Function, FunctionCall, Generic, Grouped,
    ImplementationBlock, Import, Index, IntegerLiteral, Item, Lambda, LambdaParameter,
    LetStatement, List, ListElement, ListExpression, Literal, LocalVariable, MethodCall,
    NamedFieldExpression, ObjectWrapper, Operator, Parameter, PathElement, Pattern,
    RangeExpression, ResultStatement, Span, Statement, StaticMethodCall, StringLiteral,
    StructExpression, StructField, StructGeneric, TupleField, TypeCast, Unary, Unit,
    UnnamedFieldExpression, ValueType, Variable, VariableExpression, WoogStruct, XFuture, XIf,
    XMacro, XMatch, XPath, XPlugin, XPrint, XReturn, XValue, ZObjectStore, ADDITION, AND,
    ASSIGNMENT, CHAR, DIVISION, EMPTY, EMPTY_EXPRESSION, EQUAL, FALSE_LITERAL, FROM, FULL,
    GREATER_THAN, GREATER_THAN_OR_EQUAL, INCLUSIVE, ITEM_STATEMENT, LESS_THAN, LESS_THAN_OR_EQUAL,
    MACRO_CALL, MULTIPLICATION, NEGATION, NOT, NOT_EQUAL, OR, RANGE, SUBTRACTION, TASK, TO,
    TO_INCLUSIVE, TRUE_LITERAL, UNKNOWN, X_DEBUGGER,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    argument_free_list: std::sync::Mutex<Vec<usize>>,
    argument: Arc<RwLock<Vec<Option<Arc<RwLock<Argument>>>>>>,
    a_wait_free_list: std::sync::Mutex<Vec<usize>>,
    a_wait: Arc<RwLock<Vec<Option<Arc<RwLock<AWait>>>>>>,
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
    data_structure_free_list: std::sync::Mutex<Vec<usize>>,
    data_structure: Arc<RwLock<Vec<Option<Arc<RwLock<DataStructure>>>>>>,
    dwarf_source_file_free_list: std::sync::Mutex<Vec<usize>>,
    dwarf_source_file: Arc<RwLock<Vec<Option<Arc<RwLock<DwarfSourceFile>>>>>>,
    enum_field_free_list: std::sync::Mutex<Vec<usize>>,
    enum_field: Arc<RwLock<Vec<Option<Arc<RwLock<EnumField>>>>>>,
    enum_generic_free_list: std::sync::Mutex<Vec<usize>>,
    enum_generic: Arc<RwLock<Vec<Option<Arc<RwLock<EnumGeneric>>>>>>,
    enumeration_free_list: std::sync::Mutex<Vec<usize>>,
    enumeration: Arc<RwLock<Vec<Option<Arc<RwLock<Enumeration>>>>>>,
    enumeration_id_by_name: Arc<RwLock<HashMap<String, usize>>>,
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
    function_call_free_list: std::sync::Mutex<Vec<usize>>,
    function_call: Arc<RwLock<Vec<Option<Arc<RwLock<FunctionCall>>>>>>,
    x_future_free_list: std::sync::Mutex<Vec<usize>>,
    x_future: Arc<RwLock<Vec<Option<Arc<RwLock<XFuture>>>>>>,
    generic_free_list: std::sync::Mutex<Vec<usize>>,
    generic: Arc<RwLock<Vec<Option<Arc<RwLock<Generic>>>>>>,
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
    x_match_free_list: std::sync::Mutex<Vec<usize>>,
    x_match: Arc<RwLock<Vec<Option<Arc<RwLock<XMatch>>>>>>,
    method_call_free_list: std::sync::Mutex<Vec<usize>>,
    method_call: Arc<RwLock<Vec<Option<Arc<RwLock<MethodCall>>>>>>,
    named_field_expression_free_list: std::sync::Mutex<Vec<usize>>,
    named_field_expression: Arc<RwLock<Vec<Option<Arc<RwLock<NamedFieldExpression>>>>>>,
    z_object_store_free_list: std::sync::Mutex<Vec<usize>>,
    z_object_store: Arc<RwLock<Vec<Option<Arc<RwLock<ZObjectStore>>>>>>,
    z_object_store_id_by_name: Arc<RwLock<HashMap<String, usize>>>,
    object_wrapper_free_list: std::sync::Mutex<Vec<usize>>,
    object_wrapper: Arc<RwLock<Vec<Option<Arc<RwLock<ObjectWrapper>>>>>>,
    operator_free_list: std::sync::Mutex<Vec<usize>>,
    operator: Arc<RwLock<Vec<Option<Arc<RwLock<Operator>>>>>>,
    parameter_free_list: std::sync::Mutex<Vec<usize>>,
    parameter: Arc<RwLock<Vec<Option<Arc<RwLock<Parameter>>>>>>,
    x_path_free_list: std::sync::Mutex<Vec<usize>>,
    x_path: Arc<RwLock<Vec<Option<Arc<RwLock<XPath>>>>>>,
    path_element_free_list: std::sync::Mutex<Vec<usize>>,
    path_element: Arc<RwLock<Vec<Option<Arc<RwLock<PathElement>>>>>>,
    pattern_free_list: std::sync::Mutex<Vec<usize>>,
    pattern: Arc<RwLock<Vec<Option<Arc<RwLock<Pattern>>>>>>,
    x_plugin_free_list: std::sync::Mutex<Vec<usize>>,
    x_plugin: Arc<RwLock<Vec<Option<Arc<RwLock<XPlugin>>>>>>,
    x_plugin_id_by_name: Arc<RwLock<HashMap<String, usize>>>,
    x_print_free_list: std::sync::Mutex<Vec<usize>>,
    x_print: Arc<RwLock<Vec<Option<Arc<RwLock<XPrint>>>>>>,
    range_expression_free_list: std::sync::Mutex<Vec<usize>>,
    range_expression: Arc<RwLock<Vec<Option<Arc<RwLock<RangeExpression>>>>>>,
    result_statement_free_list: std::sync::Mutex<Vec<usize>>,
    result_statement: Arc<RwLock<Vec<Option<Arc<RwLock<ResultStatement>>>>>>,
    x_return_free_list: std::sync::Mutex<Vec<usize>>,
    x_return: Arc<RwLock<Vec<Option<Arc<RwLock<XReturn>>>>>>,
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
    struct_generic_free_list: std::sync::Mutex<Vec<usize>>,
    struct_generic: Arc<RwLock<Vec<Option<Arc<RwLock<StructGeneric>>>>>>,
    tuple_field_free_list: std::sync::Mutex<Vec<usize>>,
    tuple_field: Arc<RwLock<Vec<Option<Arc<RwLock<TupleField>>>>>>,
    type_cast_free_list: std::sync::Mutex<Vec<usize>>,
    type_cast: Arc<RwLock<Vec<Option<Arc<RwLock<TypeCast>>>>>>,
    unary_free_list: std::sync::Mutex<Vec<usize>>,
    unary: Arc<RwLock<Vec<Option<Arc<RwLock<Unary>>>>>>,
    unit_free_list: std::sync::Mutex<Vec<usize>>,
    unit: Arc<RwLock<Vec<Option<Arc<RwLock<Unit>>>>>>,
    unnamed_field_expression_free_list: std::sync::Mutex<Vec<usize>>,
    unnamed_field_expression: Arc<RwLock<Vec<Option<Arc<RwLock<UnnamedFieldExpression>>>>>>,
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
            a_wait_free_list: std::sync::Mutex::new(Vec::new()),
            a_wait: Arc::new(RwLock::new(Vec::new())),
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
            data_structure_free_list: std::sync::Mutex::new(Vec::new()),
            data_structure: Arc::new(RwLock::new(Vec::new())),
            dwarf_source_file_free_list: std::sync::Mutex::new(Vec::new()),
            dwarf_source_file: Arc::new(RwLock::new(Vec::new())),
            enum_field_free_list: std::sync::Mutex::new(Vec::new()),
            enum_field: Arc::new(RwLock::new(Vec::new())),
            enum_generic_free_list: std::sync::Mutex::new(Vec::new()),
            enum_generic: Arc::new(RwLock::new(Vec::new())),
            enumeration_free_list: std::sync::Mutex::new(Vec::new()),
            enumeration: Arc::new(RwLock::new(Vec::new())),
            enumeration_id_by_name: Arc::new(RwLock::new(HashMap::default())),
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
            function_call_free_list: std::sync::Mutex::new(Vec::new()),
            function_call: Arc::new(RwLock::new(Vec::new())),
            x_future_free_list: std::sync::Mutex::new(Vec::new()),
            x_future: Arc::new(RwLock::new(Vec::new())),
            generic_free_list: std::sync::Mutex::new(Vec::new()),
            generic: Arc::new(RwLock::new(Vec::new())),
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
            x_match_free_list: std::sync::Mutex::new(Vec::new()),
            x_match: Arc::new(RwLock::new(Vec::new())),
            method_call_free_list: std::sync::Mutex::new(Vec::new()),
            method_call: Arc::new(RwLock::new(Vec::new())),
            named_field_expression_free_list: std::sync::Mutex::new(Vec::new()),
            named_field_expression: Arc::new(RwLock::new(Vec::new())),
            z_object_store_free_list: std::sync::Mutex::new(Vec::new()),
            z_object_store: Arc::new(RwLock::new(Vec::new())),
            z_object_store_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            object_wrapper_free_list: std::sync::Mutex::new(Vec::new()),
            object_wrapper: Arc::new(RwLock::new(Vec::new())),
            operator_free_list: std::sync::Mutex::new(Vec::new()),
            operator: Arc::new(RwLock::new(Vec::new())),
            parameter_free_list: std::sync::Mutex::new(Vec::new()),
            parameter: Arc::new(RwLock::new(Vec::new())),
            x_path_free_list: std::sync::Mutex::new(Vec::new()),
            x_path: Arc::new(RwLock::new(Vec::new())),
            path_element_free_list: std::sync::Mutex::new(Vec::new()),
            path_element: Arc::new(RwLock::new(Vec::new())),
            pattern_free_list: std::sync::Mutex::new(Vec::new()),
            pattern: Arc::new(RwLock::new(Vec::new())),
            x_plugin_free_list: std::sync::Mutex::new(Vec::new()),
            x_plugin: Arc::new(RwLock::new(Vec::new())),
            x_plugin_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            x_print_free_list: std::sync::Mutex::new(Vec::new()),
            x_print: Arc::new(RwLock::new(Vec::new())),
            range_expression_free_list: std::sync::Mutex::new(Vec::new()),
            range_expression: Arc::new(RwLock::new(Vec::new())),
            result_statement_free_list: std::sync::Mutex::new(Vec::new()),
            result_statement: Arc::new(RwLock::new(Vec::new())),
            x_return_free_list: std::sync::Mutex::new(Vec::new()),
            x_return: Arc::new(RwLock::new(Vec::new())),
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
            struct_generic_free_list: std::sync::Mutex::new(Vec::new()),
            struct_generic: Arc::new(RwLock::new(Vec::new())),
            tuple_field_free_list: std::sync::Mutex::new(Vec::new()),
            tuple_field: Arc::new(RwLock::new(Vec::new())),
            type_cast_free_list: std::sync::Mutex::new(Vec::new()),
            type_cast: Arc::new(RwLock::new(Vec::new())),
            unary_free_list: std::sync::Mutex::new(Vec::new()),
            unary: Arc::new(RwLock::new(Vec::new())),
            unit_free_list: std::sync::Mutex::new(Vec::new()),
            unit: Arc::new(RwLock::new(Vec::new())),
            unnamed_field_expression_free_list: std::sync::Mutex::new(Vec::new()),
            unnamed_field_expression: Arc::new(RwLock::new(Vec::new())),
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

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_rwlock_vec-object-store-methods"}}}
    /// Inter (insert) [`Argument`] into the store.
    ///
    #[inline]
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
    #[inline]
    pub fn exhume_argument(&self, id: &usize) -> Option<Arc<RwLock<Argument>>> {
        match self.argument.read().unwrap().get(*id) {
            Some(argument) => argument.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Argument`] from the store.
    ///
    #[inline]
    pub fn exorcise_argument(&mut self, id: &usize) -> Option<Arc<RwLock<Argument>>> {
        log::debug!(target: "store", "exorcising argument slot: {id}.");
        let result = self.argument.write().unwrap()[*id].take();
        self.argument_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Argument>`.
    ///
    #[inline]
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

    /// Inter (insert) [`AWait`] into the store.
    ///
    #[inline]
    pub fn inter_a_wait<F>(&mut self, a_wait: F) -> Arc<RwLock<AWait>>
    where
        F: Fn(usize) -> Arc<RwLock<AWait>>,
    {
        let _index = if let Some(_index) = self.a_wait_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.a_wait.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.a_wait.write().unwrap().push(None);
            _index
        };

        let a_wait = a_wait(_index);

        let found = if let Some(a_wait) = self.a_wait.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *a_wait.read().unwrap()
            } else {
                false
            }
        }) {
            a_wait.clone()
        } else {
            None
        };

        if let Some(a_wait) = found {
            log::debug!(target: "store", "found duplicate {a_wait:?}.");
            self.a_wait_free_list.lock().unwrap().push(_index);
            a_wait.clone()
        } else {
            log::debug!(target: "store", "interring {a_wait:?}.");
            self.a_wait.write().unwrap()[_index] = Some(a_wait.clone());
            a_wait
        }
    }

    /// Exhume (get) [`AWait`] from the store.
    ///
    #[inline]
    pub fn exhume_a_wait(&self, id: &usize) -> Option<Arc<RwLock<AWait>>> {
        match self.a_wait.read().unwrap().get(*id) {
            Some(a_wait) => a_wait.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`AWait`] from the store.
    ///
    #[inline]
    pub fn exorcise_a_wait(&mut self, id: &usize) -> Option<Arc<RwLock<AWait>>> {
        log::debug!(target: "store", "exorcising a_wait slot: {id}.");
        let result = self.a_wait.write().unwrap()[*id].take();
        self.a_wait_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, AWait>`.
    ///
    #[inline]
    pub fn iter_a_wait(&self) -> impl Iterator<Item = Arc<RwLock<AWait>>> + '_ {
        let len = self.a_wait.read().unwrap().len();
        (0..len)
            .filter(|i| self.a_wait.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.a_wait.read().unwrap()[i]
                    .as_ref()
                    .map(|a_wait| a_wait.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Binary`] into the store.
    ///
    #[inline]
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
    #[inline]
    pub fn exhume_binary(&self, id: &usize) -> Option<Arc<RwLock<Binary>>> {
        match self.binary.read().unwrap().get(*id) {
            Some(binary) => binary.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Binary`] from the store.
    ///
    #[inline]
    pub fn exorcise_binary(&mut self, id: &usize) -> Option<Arc<RwLock<Binary>>> {
        log::debug!(target: "store", "exorcising binary slot: {id}.");
        let result = self.binary.write().unwrap()[*id].take();
        self.binary_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Binary>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_block(&self, id: &usize) -> Option<Arc<RwLock<Block>>> {
        match self.block.read().unwrap().get(*id) {
            Some(block) => block.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Block`] from the store.
    ///
    #[inline]
    pub fn exorcise_block(&mut self, id: &usize) -> Option<Arc<RwLock<Block>>> {
        log::debug!(target: "store", "exorcising block slot: {id}.");
        let result = self.block.write().unwrap()[*id].take();
        self.block_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Block>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_body(&self, id: &usize) -> Option<Arc<RwLock<Body>>> {
        match self.body.read().unwrap().get(*id) {
            Some(body) => body.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Body`] from the store.
    ///
    #[inline]
    pub fn exorcise_body(&mut self, id: &usize) -> Option<Arc<RwLock<Body>>> {
        log::debug!(target: "store", "exorcising body slot: {id}.");
        let result = self.body.write().unwrap()[*id].take();
        self.body_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Body>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_boolean_literal(&self, id: &usize) -> Option<Arc<RwLock<BooleanLiteral>>> {
        match self.boolean_literal.read().unwrap().get(*id) {
            Some(boolean_literal) => boolean_literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`BooleanLiteral`] from the store.
    ///
    #[inline]
    pub fn exorcise_boolean_literal(&mut self, id: &usize) -> Option<Arc<RwLock<BooleanLiteral>>> {
        log::debug!(target: "store", "exorcising boolean_literal slot: {id}.");
        let result = self.boolean_literal.write().unwrap()[*id].take();
        self.boolean_literal_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanLiteral>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_boolean_operator(&self, id: &usize) -> Option<Arc<RwLock<BooleanOperator>>> {
        match self.boolean_operator.read().unwrap().get(*id) {
            Some(boolean_operator) => boolean_operator.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`BooleanOperator`] from the store.
    ///
    #[inline]
    pub fn exorcise_boolean_operator(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<BooleanOperator>>> {
        log::debug!(target: "store", "exorcising boolean_operator slot: {id}.");
        let result = self.boolean_operator.write().unwrap()[*id].take();
        self.boolean_operator_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanOperator>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_call(&self, id: &usize) -> Option<Arc<RwLock<Call>>> {
        match self.call.read().unwrap().get(*id) {
            Some(call) => call.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Call`] from the store.
    ///
    #[inline]
    pub fn exorcise_call(&mut self, id: &usize) -> Option<Arc<RwLock<Call>>> {
        log::debug!(target: "store", "exorcising call slot: {id}.");
        let result = self.call.write().unwrap()[*id].take();
        self.call_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Call>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_comparison(&self, id: &usize) -> Option<Arc<RwLock<Comparison>>> {
        match self.comparison.read().unwrap().get(*id) {
            Some(comparison) => comparison.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Comparison`] from the store.
    ///
    #[inline]
    pub fn exorcise_comparison(&mut self, id: &usize) -> Option<Arc<RwLock<Comparison>>> {
        log::debug!(target: "store", "exorcising comparison slot: {id}.");
        let result = self.comparison.write().unwrap()[*id].take();
        self.comparison_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Comparison>`.
    ///
    #[inline]
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

    /// Inter (insert) [`DataStructure`] into the store.
    ///
    #[inline]
    pub fn inter_data_structure<F>(&mut self, data_structure: F) -> Arc<RwLock<DataStructure>>
    where
        F: Fn(usize) -> Arc<RwLock<DataStructure>>,
    {
        let _index = if let Some(_index) = self.data_structure_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.data_structure.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.data_structure.write().unwrap().push(None);
            _index
        };

        let data_structure = data_structure(_index);

        let found = if let Some(data_structure) =
            self.data_structure.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *data_structure.read().unwrap()
                } else {
                    false
                }
            }) {
            data_structure.clone()
        } else {
            None
        };

        if let Some(data_structure) = found {
            log::debug!(target: "store", "found duplicate {data_structure:?}.");
            self.data_structure_free_list.lock().unwrap().push(_index);
            data_structure.clone()
        } else {
            log::debug!(target: "store", "interring {data_structure:?}.");
            self.data_structure.write().unwrap()[_index] = Some(data_structure.clone());
            data_structure
        }
    }

    /// Exhume (get) [`DataStructure`] from the store.
    ///
    #[inline]
    pub fn exhume_data_structure(&self, id: &usize) -> Option<Arc<RwLock<DataStructure>>> {
        match self.data_structure.read().unwrap().get(*id) {
            Some(data_structure) => data_structure.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`DataStructure`] from the store.
    ///
    #[inline]
    pub fn exorcise_data_structure(&mut self, id: &usize) -> Option<Arc<RwLock<DataStructure>>> {
        log::debug!(target: "store", "exorcising data_structure slot: {id}.");
        let result = self.data_structure.write().unwrap()[*id].take();
        self.data_structure_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, DataStructure>`.
    ///
    #[inline]
    pub fn iter_data_structure(&self) -> impl Iterator<Item = Arc<RwLock<DataStructure>>> + '_ {
        let len = self.data_structure.read().unwrap().len();
        (0..len)
            .filter(|i| self.data_structure.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.data_structure.read().unwrap()[i]
                    .as_ref()
                    .map(|data_structure| data_structure.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`DwarfSourceFile`] into the store.
    ///
    #[inline]
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
    #[inline]
    pub fn exhume_dwarf_source_file(&self, id: &usize) -> Option<Arc<RwLock<DwarfSourceFile>>> {
        match self.dwarf_source_file.read().unwrap().get(*id) {
            Some(dwarf_source_file) => dwarf_source_file.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`DwarfSourceFile`] from the store.
    ///
    #[inline]
    pub fn exorcise_dwarf_source_file(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<DwarfSourceFile>>> {
        log::debug!(target: "store", "exorcising dwarf_source_file slot: {id}.");
        let result = self.dwarf_source_file.write().unwrap()[*id].take();
        self.dwarf_source_file_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, DwarfSourceFile>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_enum_field(&self, id: &usize) -> Option<Arc<RwLock<EnumField>>> {
        match self.enum_field.read().unwrap().get(*id) {
            Some(enum_field) => enum_field.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`EnumField`] from the store.
    ///
    #[inline]
    pub fn exorcise_enum_field(&mut self, id: &usize) -> Option<Arc<RwLock<EnumField>>> {
        log::debug!(target: "store", "exorcising enum_field slot: {id}.");
        let result = self.enum_field.write().unwrap()[*id].take();
        self.enum_field_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, EnumField>`.
    ///
    #[inline]
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

    /// Inter (insert) [`EnumGeneric`] into the store.
    ///
    #[inline]
    pub fn inter_enum_generic<F>(&mut self, enum_generic: F) -> Arc<RwLock<EnumGeneric>>
    where
        F: Fn(usize) -> Arc<RwLock<EnumGeneric>>,
    {
        let _index = if let Some(_index) = self.enum_generic_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.enum_generic.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.enum_generic.write().unwrap().push(None);
            _index
        };

        let enum_generic = enum_generic(_index);

        let found = if let Some(enum_generic) =
            self.enum_generic.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *enum_generic.read().unwrap()
                } else {
                    false
                }
            }) {
            enum_generic.clone()
        } else {
            None
        };

        if let Some(enum_generic) = found {
            log::debug!(target: "store", "found duplicate {enum_generic:?}.");
            self.enum_generic_free_list.lock().unwrap().push(_index);
            enum_generic.clone()
        } else {
            log::debug!(target: "store", "interring {enum_generic:?}.");
            self.enum_generic.write().unwrap()[_index] = Some(enum_generic.clone());
            enum_generic
        }
    }

    /// Exhume (get) [`EnumGeneric`] from the store.
    ///
    #[inline]
    pub fn exhume_enum_generic(&self, id: &usize) -> Option<Arc<RwLock<EnumGeneric>>> {
        match self.enum_generic.read().unwrap().get(*id) {
            Some(enum_generic) => enum_generic.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`EnumGeneric`] from the store.
    ///
    #[inline]
    pub fn exorcise_enum_generic(&mut self, id: &usize) -> Option<Arc<RwLock<EnumGeneric>>> {
        log::debug!(target: "store", "exorcising enum_generic slot: {id}.");
        let result = self.enum_generic.write().unwrap()[*id].take();
        self.enum_generic_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, EnumGeneric>`.
    ///
    #[inline]
    pub fn iter_enum_generic(&self) -> impl Iterator<Item = Arc<RwLock<EnumGeneric>>> + '_ {
        let len = self.enum_generic.read().unwrap().len();
        (0..len)
            .filter(|i| self.enum_generic.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.enum_generic.read().unwrap()[i]
                    .as_ref()
                    .map(|enum_generic| enum_generic.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Enumeration`] into the store.
    ///
    #[inline]
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
    #[inline]
    pub fn exhume_enumeration(&self, id: &usize) -> Option<Arc<RwLock<Enumeration>>> {
        match self.enumeration.read().unwrap().get(*id) {
            Some(enumeration) => enumeration.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Enumeration`] from the store.
    ///
    #[inline]
    pub fn exorcise_enumeration(&mut self, id: &usize) -> Option<Arc<RwLock<Enumeration>>> {
        log::debug!(target: "store", "exorcising enumeration slot: {id}.");
        let result = self.enumeration.write().unwrap()[*id].take();
        self.enumeration_free_list.lock().unwrap().push(*id);
        result
    }

    /// Exorcise [`Enumeration`] id from the store by name.
    ///
    #[inline]
    pub fn exhume_enumeration_id_by_name(&self, name: &str) -> Option<usize> {
        self.enumeration_id_by_name
            .read()
            .unwrap()
            .get(name)
            .map(|enumeration| *enumeration)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Enumeration>`.
    ///
    #[inline]
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

    /// Inter (insert) [`Expression`] into the store.
    ///
    #[inline]
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
    #[inline]
    pub fn exhume_expression(&self, id: &usize) -> Option<Arc<RwLock<Expression>>> {
        match self.expression.read().unwrap().get(*id) {
            Some(expression) => expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Expression`] from the store.
    ///
    #[inline]
    pub fn exorcise_expression(&mut self, id: &usize) -> Option<Arc<RwLock<Expression>>> {
        log::debug!(target: "store", "exorcising expression slot: {id}.");
        let result = self.expression.write().unwrap()[*id].take();
        self.expression_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Expression>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exorcise_expression_statement(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<ExpressionStatement>>> {
        log::debug!(target: "store", "exorcising expression_statement slot: {id}.");
        let result = self.expression_statement.write().unwrap()[*id].take();
        self.expression_statement_free_list
            .lock()
            .unwrap()
            .push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ExpressionStatement>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exorcise_external_implementation(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<ExternalImplementation>>> {
        log::debug!(target: "store", "exorcising external_implementation slot: {id}.");
        let result = self.external_implementation.write().unwrap()[*id].take();
        self.external_implementation_free_list
            .lock()
            .unwrap()
            .push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ExternalImplementation>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_field(&self, id: &usize) -> Option<Arc<RwLock<Field>>> {
        match self.field.read().unwrap().get(*id) {
            Some(field) => field.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Field`] from the store.
    ///
    #[inline]
    pub fn exorcise_field(&mut self, id: &usize) -> Option<Arc<RwLock<Field>>> {
        log::debug!(target: "store", "exorcising field slot: {id}.");
        let result = self.field.write().unwrap()[*id].take();
        self.field_free_list.lock().unwrap().push(*id);
        result
    }

    /// Exorcise [`Field`] id from the store by name.
    ///
    #[inline]
    pub fn exhume_field_id_by_name(&self, name: &str) -> Option<usize> {
        self.field_id_by_name
            .read()
            .unwrap()
            .get(name)
            .map(|field| *field)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Field>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_field_access(&self, id: &usize) -> Option<Arc<RwLock<FieldAccess>>> {
        match self.field_access.read().unwrap().get(*id) {
            Some(field_access) => field_access.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FieldAccess`] from the store.
    ///
    #[inline]
    pub fn exorcise_field_access(&mut self, id: &usize) -> Option<Arc<RwLock<FieldAccess>>> {
        log::debug!(target: "store", "exorcising field_access slot: {id}.");
        let result = self.field_access.write().unwrap()[*id].take();
        self.field_access_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccess>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_field_access_target(&self, id: &usize) -> Option<Arc<RwLock<FieldAccessTarget>>> {
        match self.field_access_target.read().unwrap().get(*id) {
            Some(field_access_target) => field_access_target.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FieldAccessTarget`] from the store.
    ///
    #[inline]
    pub fn exorcise_field_access_target(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<FieldAccessTarget>>> {
        log::debug!(target: "store", "exorcising field_access_target slot: {id}.");
        let result = self.field_access_target.write().unwrap()[*id].take();
        self.field_access_target_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccessTarget>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_field_expression(&self, id: &usize) -> Option<Arc<RwLock<FieldExpression>>> {
        match self.field_expression.read().unwrap().get(*id) {
            Some(field_expression) => field_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FieldExpression`] from the store.
    ///
    #[inline]
    pub fn exorcise_field_expression(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<FieldExpression>>> {
        log::debug!(target: "store", "exorcising field_expression slot: {id}.");
        let result = self.field_expression.write().unwrap()[*id].take();
        self.field_expression_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldExpression>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_float_literal(&self, id: &usize) -> Option<Arc<RwLock<FloatLiteral>>> {
        match self.float_literal.read().unwrap().get(*id) {
            Some(float_literal) => float_literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FloatLiteral`] from the store.
    ///
    #[inline]
    pub fn exorcise_float_literal(&mut self, id: &usize) -> Option<Arc<RwLock<FloatLiteral>>> {
        log::debug!(target: "store", "exorcising float_literal slot: {id}.");
        let result = self.float_literal.write().unwrap()[*id].take();
        self.float_literal_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FloatLiteral>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_for_loop(&self, id: &usize) -> Option<Arc<RwLock<ForLoop>>> {
        match self.for_loop.read().unwrap().get(*id) {
            Some(for_loop) => for_loop.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ForLoop`] from the store.
    ///
    #[inline]
    pub fn exorcise_for_loop(&mut self, id: &usize) -> Option<Arc<RwLock<ForLoop>>> {
        log::debug!(target: "store", "exorcising for_loop slot: {id}.");
        let result = self.for_loop.write().unwrap()[*id].take();
        self.for_loop_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ForLoop>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_function(&self, id: &usize) -> Option<Arc<RwLock<Function>>> {
        match self.function.read().unwrap().get(*id) {
            Some(function) => function.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Function`] from the store.
    ///
    #[inline]
    pub fn exorcise_function(&mut self, id: &usize) -> Option<Arc<RwLock<Function>>> {
        log::debug!(target: "store", "exorcising function slot: {id}.");
        let result = self.function.write().unwrap()[*id].take();
        self.function_free_list.lock().unwrap().push(*id);
        result
    }

    /// Exorcise [`Function`] id from the store by name.
    ///
    #[inline]
    pub fn exhume_function_id_by_name(&self, name: &str) -> Option<usize> {
        self.function_id_by_name
            .read()
            .unwrap()
            .get(name)
            .map(|function| *function)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Function>`.
    ///
    #[inline]
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

    /// Inter (insert) [`FunctionCall`] into the store.
    ///
    #[inline]
    pub fn inter_function_call<F>(&mut self, function_call: F) -> Arc<RwLock<FunctionCall>>
    where
        F: Fn(usize) -> Arc<RwLock<FunctionCall>>,
    {
        let _index = if let Some(_index) = self.function_call_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.function_call.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.function_call.write().unwrap().push(None);
            _index
        };

        let function_call = function_call(_index);

        let found = if let Some(function_call) =
            self.function_call.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *function_call.read().unwrap()
                } else {
                    false
                }
            }) {
            function_call.clone()
        } else {
            None
        };

        if let Some(function_call) = found {
            log::debug!(target: "store", "found duplicate {function_call:?}.");
            self.function_call_free_list.lock().unwrap().push(_index);
            function_call.clone()
        } else {
            log::debug!(target: "store", "interring {function_call:?}.");
            self.function_call.write().unwrap()[_index] = Some(function_call.clone());
            function_call
        }
    }

    /// Exhume (get) [`FunctionCall`] from the store.
    ///
    #[inline]
    pub fn exhume_function_call(&self, id: &usize) -> Option<Arc<RwLock<FunctionCall>>> {
        match self.function_call.read().unwrap().get(*id) {
            Some(function_call) => function_call.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FunctionCall`] from the store.
    ///
    #[inline]
    pub fn exorcise_function_call(&mut self, id: &usize) -> Option<Arc<RwLock<FunctionCall>>> {
        log::debug!(target: "store", "exorcising function_call slot: {id}.");
        let result = self.function_call.write().unwrap()[*id].take();
        self.function_call_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FunctionCall>`.
    ///
    #[inline]
    pub fn iter_function_call(&self) -> impl Iterator<Item = Arc<RwLock<FunctionCall>>> + '_ {
        let len = self.function_call.read().unwrap().len();
        (0..len)
            .filter(|i| self.function_call.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.function_call.read().unwrap()[i]
                    .as_ref()
                    .map(|function_call| function_call.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`XFuture`] into the store.
    ///
    #[inline]
    pub fn inter_x_future<F>(&mut self, x_future: F) -> Arc<RwLock<XFuture>>
    where
        F: Fn(usize) -> Arc<RwLock<XFuture>>,
    {
        let _index = if let Some(_index) = self.x_future_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_future.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.x_future.write().unwrap().push(None);
            _index
        };

        let x_future = x_future(_index);

        let found = if let Some(x_future) = self.x_future.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *x_future.read().unwrap()
            } else {
                false
            }
        }) {
            x_future.clone()
        } else {
            None
        };

        if let Some(x_future) = found {
            log::debug!(target: "store", "found duplicate {x_future:?}.");
            self.x_future_free_list.lock().unwrap().push(_index);
            x_future.clone()
        } else {
            log::debug!(target: "store", "interring {x_future:?}.");
            self.x_future.write().unwrap()[_index] = Some(x_future.clone());
            x_future
        }
    }

    /// Exhume (get) [`XFuture`] from the store.
    ///
    #[inline]
    pub fn exhume_x_future(&self, id: &usize) -> Option<Arc<RwLock<XFuture>>> {
        match self.x_future.read().unwrap().get(*id) {
            Some(x_future) => x_future.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XFuture`] from the store.
    ///
    #[inline]
    pub fn exorcise_x_future(&mut self, id: &usize) -> Option<Arc<RwLock<XFuture>>> {
        log::debug!(target: "store", "exorcising x_future slot: {id}.");
        let result = self.x_future.write().unwrap()[*id].take();
        self.x_future_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XFuture>`.
    ///
    #[inline]
    pub fn iter_x_future(&self) -> impl Iterator<Item = Arc<RwLock<XFuture>>> + '_ {
        let len = self.x_future.read().unwrap().len();
        (0..len)
            .filter(|i| self.x_future.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.x_future.read().unwrap()[i]
                    .as_ref()
                    .map(|x_future| x_future.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Generic`] into the store.
    ///
    #[inline]
    pub fn inter_generic<F>(&mut self, generic: F) -> Arc<RwLock<Generic>>
    where
        F: Fn(usize) -> Arc<RwLock<Generic>>,
    {
        let _index = if let Some(_index) = self.generic_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.generic.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.generic.write().unwrap().push(None);
            _index
        };

        let generic = generic(_index);

        let found = if let Some(generic) = self.generic.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *generic.read().unwrap()
            } else {
                false
            }
        }) {
            generic.clone()
        } else {
            None
        };

        if let Some(generic) = found {
            log::debug!(target: "store", "found duplicate {generic:?}.");
            self.generic_free_list.lock().unwrap().push(_index);
            generic.clone()
        } else {
            log::debug!(target: "store", "interring {generic:?}.");
            self.generic.write().unwrap()[_index] = Some(generic.clone());
            generic
        }
    }

    /// Exhume (get) [`Generic`] from the store.
    ///
    #[inline]
    pub fn exhume_generic(&self, id: &usize) -> Option<Arc<RwLock<Generic>>> {
        match self.generic.read().unwrap().get(*id) {
            Some(generic) => generic.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Generic`] from the store.
    ///
    #[inline]
    pub fn exorcise_generic(&mut self, id: &usize) -> Option<Arc<RwLock<Generic>>> {
        log::debug!(target: "store", "exorcising generic slot: {id}.");
        let result = self.generic.write().unwrap()[*id].take();
        self.generic_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Generic>`.
    ///
    #[inline]
    pub fn iter_generic(&self) -> impl Iterator<Item = Arc<RwLock<Generic>>> + '_ {
        let len = self.generic.read().unwrap().len();
        (0..len)
            .filter(|i| self.generic.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.generic.read().unwrap()[i]
                    .as_ref()
                    .map(|generic| generic.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Grouped`] into the store.
    ///
    #[inline]
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
    #[inline]
    pub fn exhume_grouped(&self, id: &usize) -> Option<Arc<RwLock<Grouped>>> {
        match self.grouped.read().unwrap().get(*id) {
            Some(grouped) => grouped.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Grouped`] from the store.
    ///
    #[inline]
    pub fn exorcise_grouped(&mut self, id: &usize) -> Option<Arc<RwLock<Grouped>>> {
        log::debug!(target: "store", "exorcising grouped slot: {id}.");
        let result = self.grouped.write().unwrap()[*id].take();
        self.grouped_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Grouped>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_x_if(&self, id: &usize) -> Option<Arc<RwLock<XIf>>> {
        match self.x_if.read().unwrap().get(*id) {
            Some(x_if) => x_if.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XIf`] from the store.
    ///
    #[inline]
    pub fn exorcise_x_if(&mut self, id: &usize) -> Option<Arc<RwLock<XIf>>> {
        log::debug!(target: "store", "exorcising x_if slot: {id}.");
        let result = self.x_if.write().unwrap()[*id].take();
        self.x_if_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XIf>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exorcise_implementation_block(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<ImplementationBlock>>> {
        log::debug!(target: "store", "exorcising implementation_block slot: {id}.");
        let result = self.implementation_block.write().unwrap()[*id].take();
        self.implementation_block_free_list
            .lock()
            .unwrap()
            .push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ImplementationBlock>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_import(&self, id: &usize) -> Option<Arc<RwLock<Import>>> {
        match self.import.read().unwrap().get(*id) {
            Some(import) => import.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Import`] from the store.
    ///
    #[inline]
    pub fn exorcise_import(&mut self, id: &usize) -> Option<Arc<RwLock<Import>>> {
        log::debug!(target: "store", "exorcising import slot: {id}.");
        let result = self.import.write().unwrap()[*id].take();
        self.import_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Import>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_index(&self, id: &usize) -> Option<Arc<RwLock<Index>>> {
        match self.index.read().unwrap().get(*id) {
            Some(index) => index.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Index`] from the store.
    ///
    #[inline]
    pub fn exorcise_index(&mut self, id: &usize) -> Option<Arc<RwLock<Index>>> {
        log::debug!(target: "store", "exorcising index slot: {id}.");
        let result = self.index.write().unwrap()[*id].take();
        self.index_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Index>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_integer_literal(&self, id: &usize) -> Option<Arc<RwLock<IntegerLiteral>>> {
        match self.integer_literal.read().unwrap().get(*id) {
            Some(integer_literal) => integer_literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`IntegerLiteral`] from the store.
    ///
    #[inline]
    pub fn exorcise_integer_literal(&mut self, id: &usize) -> Option<Arc<RwLock<IntegerLiteral>>> {
        log::debug!(target: "store", "exorcising integer_literal slot: {id}.");
        let result = self.integer_literal.write().unwrap()[*id].take();
        self.integer_literal_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, IntegerLiteral>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_item(&self, id: &usize) -> Option<Arc<RwLock<Item>>> {
        match self.item.read().unwrap().get(*id) {
            Some(item) => item.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Item`] from the store.
    ///
    #[inline]
    pub fn exorcise_item(&mut self, id: &usize) -> Option<Arc<RwLock<Item>>> {
        log::debug!(target: "store", "exorcising item slot: {id}.");
        let result = self.item.write().unwrap()[*id].take();
        self.item_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Item>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_lambda(&self, id: &usize) -> Option<Arc<RwLock<Lambda>>> {
        match self.lambda.read().unwrap().get(*id) {
            Some(lambda) => lambda.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Lambda`] from the store.
    ///
    #[inline]
    pub fn exorcise_lambda(&mut self, id: &usize) -> Option<Arc<RwLock<Lambda>>> {
        log::debug!(target: "store", "exorcising lambda slot: {id}.");
        let result = self.lambda.write().unwrap()[*id].take();
        self.lambda_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Lambda>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_lambda_parameter(&self, id: &usize) -> Option<Arc<RwLock<LambdaParameter>>> {
        match self.lambda_parameter.read().unwrap().get(*id) {
            Some(lambda_parameter) => lambda_parameter.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`LambdaParameter`] from the store.
    ///
    #[inline]
    pub fn exorcise_lambda_parameter(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<LambdaParameter>>> {
        log::debug!(target: "store", "exorcising lambda_parameter slot: {id}.");
        let result = self.lambda_parameter.write().unwrap()[*id].take();
        self.lambda_parameter_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LambdaParameter>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_let_statement(&self, id: &usize) -> Option<Arc<RwLock<LetStatement>>> {
        match self.let_statement.read().unwrap().get(*id) {
            Some(let_statement) => let_statement.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`LetStatement`] from the store.
    ///
    #[inline]
    pub fn exorcise_let_statement(&mut self, id: &usize) -> Option<Arc<RwLock<LetStatement>>> {
        log::debug!(target: "store", "exorcising let_statement slot: {id}.");
        let result = self.let_statement.write().unwrap()[*id].take();
        self.let_statement_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LetStatement>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_list(&self, id: &usize) -> Option<Arc<RwLock<List>>> {
        match self.list.read().unwrap().get(*id) {
            Some(list) => list.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`List`] from the store.
    ///
    #[inline]
    pub fn exorcise_list(&mut self, id: &usize) -> Option<Arc<RwLock<List>>> {
        log::debug!(target: "store", "exorcising list slot: {id}.");
        let result = self.list.write().unwrap()[*id].take();
        self.list_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, List>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_list_element(&self, id: &usize) -> Option<Arc<RwLock<ListElement>>> {
        match self.list_element.read().unwrap().get(*id) {
            Some(list_element) => list_element.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ListElement`] from the store.
    ///
    #[inline]
    pub fn exorcise_list_element(&mut self, id: &usize) -> Option<Arc<RwLock<ListElement>>> {
        log::debug!(target: "store", "exorcising list_element slot: {id}.");
        let result = self.list_element.write().unwrap()[*id].take();
        self.list_element_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListElement>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_list_expression(&self, id: &usize) -> Option<Arc<RwLock<ListExpression>>> {
        match self.list_expression.read().unwrap().get(*id) {
            Some(list_expression) => list_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ListExpression`] from the store.
    ///
    #[inline]
    pub fn exorcise_list_expression(&mut self, id: &usize) -> Option<Arc<RwLock<ListExpression>>> {
        log::debug!(target: "store", "exorcising list_expression slot: {id}.");
        let result = self.list_expression.write().unwrap()[*id].take();
        self.list_expression_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListExpression>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_literal(&self, id: &usize) -> Option<Arc<RwLock<Literal>>> {
        match self.literal.read().unwrap().get(*id) {
            Some(literal) => literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Literal`] from the store.
    ///
    #[inline]
    pub fn exorcise_literal(&mut self, id: &usize) -> Option<Arc<RwLock<Literal>>> {
        log::debug!(target: "store", "exorcising literal slot: {id}.");
        let result = self.literal.write().unwrap()[*id].take();
        self.literal_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Literal>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_local_variable(&self, id: &usize) -> Option<Arc<RwLock<LocalVariable>>> {
        match self.local_variable.read().unwrap().get(*id) {
            Some(local_variable) => local_variable.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`LocalVariable`] from the store.
    ///
    #[inline]
    pub fn exorcise_local_variable(&mut self, id: &usize) -> Option<Arc<RwLock<LocalVariable>>> {
        log::debug!(target: "store", "exorcising local_variable slot: {id}.");
        let result = self.local_variable.write().unwrap()[*id].take();
        self.local_variable_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LocalVariable>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_x_macro(&self, id: &usize) -> Option<Arc<RwLock<XMacro>>> {
        match self.x_macro.read().unwrap().get(*id) {
            Some(x_macro) => x_macro.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XMacro`] from the store.
    ///
    #[inline]
    pub fn exorcise_x_macro(&mut self, id: &usize) -> Option<Arc<RwLock<XMacro>>> {
        log::debug!(target: "store", "exorcising x_macro slot: {id}.");
        let result = self.x_macro.write().unwrap()[*id].take();
        self.x_macro_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XMacro>`.
    ///
    #[inline]
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

    /// Inter (insert) [`XMatch`] into the store.
    ///
    #[inline]
    pub fn inter_x_match<F>(&mut self, x_match: F) -> Arc<RwLock<XMatch>>
    where
        F: Fn(usize) -> Arc<RwLock<XMatch>>,
    {
        let _index = if let Some(_index) = self.x_match_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_match.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.x_match.write().unwrap().push(None);
            _index
        };

        let x_match = x_match(_index);

        let found = if let Some(x_match) = self.x_match.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *x_match.read().unwrap()
            } else {
                false
            }
        }) {
            x_match.clone()
        } else {
            None
        };

        if let Some(x_match) = found {
            log::debug!(target: "store", "found duplicate {x_match:?}.");
            self.x_match_free_list.lock().unwrap().push(_index);
            x_match.clone()
        } else {
            log::debug!(target: "store", "interring {x_match:?}.");
            self.x_match.write().unwrap()[_index] = Some(x_match.clone());
            x_match
        }
    }

    /// Exhume (get) [`XMatch`] from the store.
    ///
    #[inline]
    pub fn exhume_x_match(&self, id: &usize) -> Option<Arc<RwLock<XMatch>>> {
        match self.x_match.read().unwrap().get(*id) {
            Some(x_match) => x_match.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XMatch`] from the store.
    ///
    #[inline]
    pub fn exorcise_x_match(&mut self, id: &usize) -> Option<Arc<RwLock<XMatch>>> {
        log::debug!(target: "store", "exorcising x_match slot: {id}.");
        let result = self.x_match.write().unwrap()[*id].take();
        self.x_match_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XMatch>`.
    ///
    #[inline]
    pub fn iter_x_match(&self) -> impl Iterator<Item = Arc<RwLock<XMatch>>> + '_ {
        let len = self.x_match.read().unwrap().len();
        (0..len)
            .filter(|i| self.x_match.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.x_match.read().unwrap()[i]
                    .as_ref()
                    .map(|x_match| x_match.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`MethodCall`] into the store.
    ///
    #[inline]
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
    #[inline]
    pub fn exhume_method_call(&self, id: &usize) -> Option<Arc<RwLock<MethodCall>>> {
        match self.method_call.read().unwrap().get(*id) {
            Some(method_call) => method_call.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`MethodCall`] from the store.
    ///
    #[inline]
    pub fn exorcise_method_call(&mut self, id: &usize) -> Option<Arc<RwLock<MethodCall>>> {
        log::debug!(target: "store", "exorcising method_call slot: {id}.");
        let result = self.method_call.write().unwrap()[*id].take();
        self.method_call_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, MethodCall>`.
    ///
    #[inline]
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

    /// Inter (insert) [`NamedFieldExpression`] into the store.
    ///
    #[inline]
    pub fn inter_named_field_expression<F>(
        &mut self,
        named_field_expression: F,
    ) -> Arc<RwLock<NamedFieldExpression>>
    where
        F: Fn(usize) -> Arc<RwLock<NamedFieldExpression>>,
    {
        let _index =
            if let Some(_index) = self.named_field_expression_free_list.lock().unwrap().pop() {
                log::trace!(target: "store", "recycling block {_index}.");
                _index
            } else {
                let _index = self.named_field_expression.read().unwrap().len();
                log::trace!(target: "store", "allocating block {_index}.");
                self.named_field_expression.write().unwrap().push(None);
                _index
            };

        let named_field_expression = named_field_expression(_index);

        let found = if let Some(named_field_expression) = self
            .named_field_expression
            .read()
            .unwrap()
            .iter()
            .find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *named_field_expression.read().unwrap()
                } else {
                    false
                }
            }) {
            named_field_expression.clone()
        } else {
            None
        };

        if let Some(named_field_expression) = found {
            log::debug!(target: "store", "found duplicate {named_field_expression:?}.");
            self.named_field_expression_free_list
                .lock()
                .unwrap()
                .push(_index);
            named_field_expression.clone()
        } else {
            log::debug!(target: "store", "interring {named_field_expression:?}.");
            self.named_field_expression.write().unwrap()[_index] =
                Some(named_field_expression.clone());
            named_field_expression
        }
    }

    /// Exhume (get) [`NamedFieldExpression`] from the store.
    ///
    #[inline]
    pub fn exhume_named_field_expression(
        &self,
        id: &usize,
    ) -> Option<Arc<RwLock<NamedFieldExpression>>> {
        match self.named_field_expression.read().unwrap().get(*id) {
            Some(named_field_expression) => named_field_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`NamedFieldExpression`] from the store.
    ///
    #[inline]
    pub fn exorcise_named_field_expression(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<NamedFieldExpression>>> {
        log::debug!(target: "store", "exorcising named_field_expression slot: {id}.");
        let result = self.named_field_expression.write().unwrap()[*id].take();
        self.named_field_expression_free_list
            .lock()
            .unwrap()
            .push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, NamedFieldExpression>`.
    ///
    #[inline]
    pub fn iter_named_field_expression(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<NamedFieldExpression>>> + '_ {
        let len = self.named_field_expression.read().unwrap().len();
        (0..len)
            .filter(|i| self.named_field_expression.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.named_field_expression.read().unwrap()[i]
                    .as_ref()
                    .map(|named_field_expression| named_field_expression.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`ZObjectStore`] into the store.
    ///
    #[inline]
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
    #[inline]
    pub fn exhume_z_object_store(&self, id: &usize) -> Option<Arc<RwLock<ZObjectStore>>> {
        match self.z_object_store.read().unwrap().get(*id) {
            Some(z_object_store) => z_object_store.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ZObjectStore`] from the store.
    ///
    #[inline]
    pub fn exorcise_z_object_store(&mut self, id: &usize) -> Option<Arc<RwLock<ZObjectStore>>> {
        log::debug!(target: "store", "exorcising z_object_store slot: {id}.");
        let result = self.z_object_store.write().unwrap()[*id].take();
        self.z_object_store_free_list.lock().unwrap().push(*id);
        result
    }

    /// Exorcise [`ZObjectStore`] id from the store by name.
    ///
    #[inline]
    pub fn exhume_z_object_store_id_by_name(&self, name: &str) -> Option<usize> {
        self.z_object_store_id_by_name
            .read()
            .unwrap()
            .get(name)
            .map(|z_object_store| *z_object_store)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ZObjectStore>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_object_wrapper(&self, id: &usize) -> Option<Arc<RwLock<ObjectWrapper>>> {
        match self.object_wrapper.read().unwrap().get(*id) {
            Some(object_wrapper) => object_wrapper.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ObjectWrapper`] from the store.
    ///
    #[inline]
    pub fn exorcise_object_wrapper(&mut self, id: &usize) -> Option<Arc<RwLock<ObjectWrapper>>> {
        log::debug!(target: "store", "exorcising object_wrapper slot: {id}.");
        let result = self.object_wrapper.write().unwrap()[*id].take();
        self.object_wrapper_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ObjectWrapper>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_operator(&self, id: &usize) -> Option<Arc<RwLock<Operator>>> {
        match self.operator.read().unwrap().get(*id) {
            Some(operator) => operator.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Operator`] from the store.
    ///
    #[inline]
    pub fn exorcise_operator(&mut self, id: &usize) -> Option<Arc<RwLock<Operator>>> {
        log::debug!(target: "store", "exorcising operator slot: {id}.");
        let result = self.operator.write().unwrap()[*id].take();
        self.operator_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Operator>`.
    ///
    #[inline]
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

    /// Inter (insert) [`Parameter`] into the store.
    ///
    #[inline]
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
    #[inline]
    pub fn exhume_parameter(&self, id: &usize) -> Option<Arc<RwLock<Parameter>>> {
        match self.parameter.read().unwrap().get(*id) {
            Some(parameter) => parameter.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Parameter`] from the store.
    ///
    #[inline]
    pub fn exorcise_parameter(&mut self, id: &usize) -> Option<Arc<RwLock<Parameter>>> {
        log::debug!(target: "store", "exorcising parameter slot: {id}.");
        let result = self.parameter.write().unwrap()[*id].take();
        self.parameter_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Parameter>`.
    ///
    #[inline]
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

    /// Inter (insert) [`XPath`] into the store.
    ///
    #[inline]
    pub fn inter_x_path<F>(&mut self, x_path: F) -> Arc<RwLock<XPath>>
    where
        F: Fn(usize) -> Arc<RwLock<XPath>>,
    {
        let _index = if let Some(_index) = self.x_path_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_path.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.x_path.write().unwrap().push(None);
            _index
        };

        let x_path = x_path(_index);

        let found = if let Some(x_path) = self.x_path.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *x_path.read().unwrap()
            } else {
                false
            }
        }) {
            x_path.clone()
        } else {
            None
        };

        if let Some(x_path) = found {
            log::debug!(target: "store", "found duplicate {x_path:?}.");
            self.x_path_free_list.lock().unwrap().push(_index);
            x_path.clone()
        } else {
            log::debug!(target: "store", "interring {x_path:?}.");
            self.x_path.write().unwrap()[_index] = Some(x_path.clone());
            x_path
        }
    }

    /// Exhume (get) [`XPath`] from the store.
    ///
    #[inline]
    pub fn exhume_x_path(&self, id: &usize) -> Option<Arc<RwLock<XPath>>> {
        match self.x_path.read().unwrap().get(*id) {
            Some(x_path) => x_path.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XPath`] from the store.
    ///
    #[inline]
    pub fn exorcise_x_path(&mut self, id: &usize) -> Option<Arc<RwLock<XPath>>> {
        log::debug!(target: "store", "exorcising x_path slot: {id}.");
        let result = self.x_path.write().unwrap()[*id].take();
        self.x_path_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XPath>`.
    ///
    #[inline]
    pub fn iter_x_path(&self) -> impl Iterator<Item = Arc<RwLock<XPath>>> + '_ {
        let len = self.x_path.read().unwrap().len();
        (0..len)
            .filter(|i| self.x_path.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.x_path.read().unwrap()[i]
                    .as_ref()
                    .map(|x_path| x_path.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`PathElement`] into the store.
    ///
    #[inline]
    pub fn inter_path_element<F>(&mut self, path_element: F) -> Arc<RwLock<PathElement>>
    where
        F: Fn(usize) -> Arc<RwLock<PathElement>>,
    {
        let _index = if let Some(_index) = self.path_element_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.path_element.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.path_element.write().unwrap().push(None);
            _index
        };

        let path_element = path_element(_index);

        let found = if let Some(path_element) =
            self.path_element.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *path_element.read().unwrap()
                } else {
                    false
                }
            }) {
            path_element.clone()
        } else {
            None
        };

        if let Some(path_element) = found {
            log::debug!(target: "store", "found duplicate {path_element:?}.");
            self.path_element_free_list.lock().unwrap().push(_index);
            path_element.clone()
        } else {
            log::debug!(target: "store", "interring {path_element:?}.");
            self.path_element.write().unwrap()[_index] = Some(path_element.clone());
            path_element
        }
    }

    /// Exhume (get) [`PathElement`] from the store.
    ///
    #[inline]
    pub fn exhume_path_element(&self, id: &usize) -> Option<Arc<RwLock<PathElement>>> {
        match self.path_element.read().unwrap().get(*id) {
            Some(path_element) => path_element.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`PathElement`] from the store.
    ///
    #[inline]
    pub fn exorcise_path_element(&mut self, id: &usize) -> Option<Arc<RwLock<PathElement>>> {
        log::debug!(target: "store", "exorcising path_element slot: {id}.");
        let result = self.path_element.write().unwrap()[*id].take();
        self.path_element_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, PathElement>`.
    ///
    #[inline]
    pub fn iter_path_element(&self) -> impl Iterator<Item = Arc<RwLock<PathElement>>> + '_ {
        let len = self.path_element.read().unwrap().len();
        (0..len)
            .filter(|i| self.path_element.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.path_element.read().unwrap()[i]
                    .as_ref()
                    .map(|path_element| path_element.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Pattern`] into the store.
    ///
    #[inline]
    pub fn inter_pattern<F>(&mut self, pattern: F) -> Arc<RwLock<Pattern>>
    where
        F: Fn(usize) -> Arc<RwLock<Pattern>>,
    {
        let _index = if let Some(_index) = self.pattern_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.pattern.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.pattern.write().unwrap().push(None);
            _index
        };

        let pattern = pattern(_index);

        let found = if let Some(pattern) = self.pattern.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *pattern.read().unwrap()
            } else {
                false
            }
        }) {
            pattern.clone()
        } else {
            None
        };

        if let Some(pattern) = found {
            log::debug!(target: "store", "found duplicate {pattern:?}.");
            self.pattern_free_list.lock().unwrap().push(_index);
            pattern.clone()
        } else {
            log::debug!(target: "store", "interring {pattern:?}.");
            self.pattern.write().unwrap()[_index] = Some(pattern.clone());
            pattern
        }
    }

    /// Exhume (get) [`Pattern`] from the store.
    ///
    #[inline]
    pub fn exhume_pattern(&self, id: &usize) -> Option<Arc<RwLock<Pattern>>> {
        match self.pattern.read().unwrap().get(*id) {
            Some(pattern) => pattern.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Pattern`] from the store.
    ///
    #[inline]
    pub fn exorcise_pattern(&mut self, id: &usize) -> Option<Arc<RwLock<Pattern>>> {
        log::debug!(target: "store", "exorcising pattern slot: {id}.");
        let result = self.pattern.write().unwrap()[*id].take();
        self.pattern_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Pattern>`.
    ///
    #[inline]
    pub fn iter_pattern(&self) -> impl Iterator<Item = Arc<RwLock<Pattern>>> + '_ {
        let len = self.pattern.read().unwrap().len();
        (0..len)
            .filter(|i| self.pattern.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.pattern.read().unwrap()[i]
                    .as_ref()
                    .map(|pattern| pattern.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`XPlugin`] into the store.
    ///
    #[inline]
    pub fn inter_x_plugin<F>(&mut self, x_plugin: F) -> Arc<RwLock<XPlugin>>
    where
        F: Fn(usize) -> Arc<RwLock<XPlugin>>,
    {
        let _index = if let Some(_index) = self.x_plugin_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_plugin.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.x_plugin.write().unwrap().push(None);
            _index
        };

        let x_plugin = x_plugin(_index);

        let found = if let Some(x_plugin) = self.x_plugin.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *x_plugin.read().unwrap()
            } else {
                false
            }
        }) {
            x_plugin.clone()
        } else {
            None
        };

        let x_plugin = if let Some(x_plugin) = found {
            log::debug!(target: "store", "found duplicate {x_plugin:?}.");
            self.x_plugin_free_list.lock().unwrap().push(_index);
            x_plugin.clone()
        } else {
            log::debug!(target: "store", "interring {x_plugin:?}.");
            self.x_plugin.write().unwrap()[_index] = Some(x_plugin.clone());
            x_plugin
        };
        self.x_plugin_id_by_name.write().unwrap().insert(
            x_plugin.read().unwrap().name.to_owned(),
            x_plugin.read().unwrap().id,
        );
        x_plugin
    }

    /// Exhume (get) [`XPlugin`] from the store.
    ///
    #[inline]
    pub fn exhume_x_plugin(&self, id: &usize) -> Option<Arc<RwLock<XPlugin>>> {
        match self.x_plugin.read().unwrap().get(*id) {
            Some(x_plugin) => x_plugin.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XPlugin`] from the store.
    ///
    #[inline]
    pub fn exorcise_x_plugin(&mut self, id: &usize) -> Option<Arc<RwLock<XPlugin>>> {
        log::debug!(target: "store", "exorcising x_plugin slot: {id}.");
        let result = self.x_plugin.write().unwrap()[*id].take();
        self.x_plugin_free_list.lock().unwrap().push(*id);
        result
    }

    /// Exorcise [`XPlugin`] id from the store by name.
    ///
    #[inline]
    pub fn exhume_x_plugin_id_by_name(&self, name: &str) -> Option<usize> {
        self.x_plugin_id_by_name
            .read()
            .unwrap()
            .get(name)
            .map(|x_plugin| *x_plugin)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XPlugin>`.
    ///
    #[inline]
    pub fn iter_x_plugin(&self) -> impl Iterator<Item = Arc<RwLock<XPlugin>>> + '_ {
        let len = self.x_plugin.read().unwrap().len();
        (0..len)
            .filter(|i| self.x_plugin.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.x_plugin.read().unwrap()[i]
                    .as_ref()
                    .map(|x_plugin| x_plugin.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`XPrint`] into the store.
    ///
    #[inline]
    pub fn inter_x_print<F>(&mut self, x_print: F) -> Arc<RwLock<XPrint>>
    where
        F: Fn(usize) -> Arc<RwLock<XPrint>>,
    {
        let _index = if let Some(_index) = self.x_print_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.x_print.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.x_print.write().unwrap().push(None);
            _index
        };

        let x_print = x_print(_index);

        let found = if let Some(x_print) = self.x_print.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *x_print.read().unwrap()
            } else {
                false
            }
        }) {
            x_print.clone()
        } else {
            None
        };

        if let Some(x_print) = found {
            log::debug!(target: "store", "found duplicate {x_print:?}.");
            self.x_print_free_list.lock().unwrap().push(_index);
            x_print.clone()
        } else {
            log::debug!(target: "store", "interring {x_print:?}.");
            self.x_print.write().unwrap()[_index] = Some(x_print.clone());
            x_print
        }
    }

    /// Exhume (get) [`XPrint`] from the store.
    ///
    #[inline]
    pub fn exhume_x_print(&self, id: &usize) -> Option<Arc<RwLock<XPrint>>> {
        match self.x_print.read().unwrap().get(*id) {
            Some(x_print) => x_print.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XPrint`] from the store.
    ///
    #[inline]
    pub fn exorcise_x_print(&mut self, id: &usize) -> Option<Arc<RwLock<XPrint>>> {
        log::debug!(target: "store", "exorcising x_print slot: {id}.");
        let result = self.x_print.write().unwrap()[*id].take();
        self.x_print_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XPrint>`.
    ///
    #[inline]
    pub fn iter_x_print(&self) -> impl Iterator<Item = Arc<RwLock<XPrint>>> + '_ {
        let len = self.x_print.read().unwrap().len();
        (0..len)
            .filter(|i| self.x_print.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.x_print.read().unwrap()[i]
                    .as_ref()
                    .map(|x_print| x_print.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`RangeExpression`] into the store.
    ///
    #[inline]
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
    #[inline]
    pub fn exhume_range_expression(&self, id: &usize) -> Option<Arc<RwLock<RangeExpression>>> {
        match self.range_expression.read().unwrap().get(*id) {
            Some(range_expression) => range_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`RangeExpression`] from the store.
    ///
    #[inline]
    pub fn exorcise_range_expression(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<RangeExpression>>> {
        log::debug!(target: "store", "exorcising range_expression slot: {id}.");
        let result = self.range_expression.write().unwrap()[*id].take();
        self.range_expression_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RangeExpression>`.
    ///
    #[inline]
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

    /// Inter (insert) [`ResultStatement`] into the store.
    ///
    #[inline]
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
    #[inline]
    pub fn exhume_result_statement(&self, id: &usize) -> Option<Arc<RwLock<ResultStatement>>> {
        match self.result_statement.read().unwrap().get(*id) {
            Some(result_statement) => result_statement.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ResultStatement`] from the store.
    ///
    #[inline]
    pub fn exorcise_result_statement(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<ResultStatement>>> {
        log::debug!(target: "store", "exorcising result_statement slot: {id}.");
        let result = self.result_statement.write().unwrap()[*id].take();
        self.result_statement_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ResultStatement>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_x_return(&self, id: &usize) -> Option<Arc<RwLock<XReturn>>> {
        match self.x_return.read().unwrap().get(*id) {
            Some(x_return) => x_return.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XReturn`] from the store.
    ///
    #[inline]
    pub fn exorcise_x_return(&mut self, id: &usize) -> Option<Arc<RwLock<XReturn>>> {
        log::debug!(target: "store", "exorcising x_return slot: {id}.");
        let result = self.x_return.write().unwrap()[*id].take();
        self.x_return_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XReturn>`.
    ///
    #[inline]
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

    /// Inter (insert) [`Span`] into the store.
    ///
    #[inline]
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
    #[inline]
    pub fn exhume_span(&self, id: &usize) -> Option<Arc<RwLock<Span>>> {
        match self.span.read().unwrap().get(*id) {
            Some(span) => span.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Span`] from the store.
    ///
    #[inline]
    pub fn exorcise_span(&mut self, id: &usize) -> Option<Arc<RwLock<Span>>> {
        log::debug!(target: "store", "exorcising span slot: {id}.");
        let result = self.span.write().unwrap()[*id].take();
        self.span_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Span>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_statement(&self, id: &usize) -> Option<Arc<RwLock<Statement>>> {
        match self.statement.read().unwrap().get(*id) {
            Some(statement) => statement.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Statement`] from the store.
    ///
    #[inline]
    pub fn exorcise_statement(&mut self, id: &usize) -> Option<Arc<RwLock<Statement>>> {
        log::debug!(target: "store", "exorcising statement slot: {id}.");
        let result = self.statement.write().unwrap()[*id].take();
        self.statement_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Statement>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_static_method_call(&self, id: &usize) -> Option<Arc<RwLock<StaticMethodCall>>> {
        match self.static_method_call.read().unwrap().get(*id) {
            Some(static_method_call) => static_method_call.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`StaticMethodCall`] from the store.
    ///
    #[inline]
    pub fn exorcise_static_method_call(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<StaticMethodCall>>> {
        log::debug!(target: "store", "exorcising static_method_call slot: {id}.");
        let result = self.static_method_call.write().unwrap()[*id].take();
        self.static_method_call_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StaticMethodCall>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_string_literal(&self, id: &usize) -> Option<Arc<RwLock<StringLiteral>>> {
        match self.string_literal.read().unwrap().get(*id) {
            Some(string_literal) => string_literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`StringLiteral`] from the store.
    ///
    #[inline]
    pub fn exorcise_string_literal(&mut self, id: &usize) -> Option<Arc<RwLock<StringLiteral>>> {
        log::debug!(target: "store", "exorcising string_literal slot: {id}.");
        let result = self.string_literal.write().unwrap()[*id].take();
        self.string_literal_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StringLiteral>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_woog_struct(&self, id: &usize) -> Option<Arc<RwLock<WoogStruct>>> {
        match self.woog_struct.read().unwrap().get(*id) {
            Some(woog_struct) => woog_struct.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`WoogStruct`] from the store.
    ///
    #[inline]
    pub fn exorcise_woog_struct(&mut self, id: &usize) -> Option<Arc<RwLock<WoogStruct>>> {
        log::debug!(target: "store", "exorcising woog_struct slot: {id}.");
        let result = self.woog_struct.write().unwrap()[*id].take();
        self.woog_struct_free_list.lock().unwrap().push(*id);
        result
    }

    /// Exorcise [`WoogStruct`] id from the store by name.
    ///
    #[inline]
    pub fn exhume_woog_struct_id_by_name(&self, name: &str) -> Option<usize> {
        self.woog_struct_id_by_name
            .read()
            .unwrap()
            .get(name)
            .map(|woog_struct| *woog_struct)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, WoogStruct>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_struct_expression(&self, id: &usize) -> Option<Arc<RwLock<StructExpression>>> {
        match self.struct_expression.read().unwrap().get(*id) {
            Some(struct_expression) => struct_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`StructExpression`] from the store.
    ///
    #[inline]
    pub fn exorcise_struct_expression(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<StructExpression>>> {
        log::debug!(target: "store", "exorcising struct_expression slot: {id}.");
        let result = self.struct_expression.write().unwrap()[*id].take();
        self.struct_expression_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructExpression>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_struct_field(&self, id: &usize) -> Option<Arc<RwLock<StructField>>> {
        match self.struct_field.read().unwrap().get(*id) {
            Some(struct_field) => struct_field.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`StructField`] from the store.
    ///
    #[inline]
    pub fn exorcise_struct_field(&mut self, id: &usize) -> Option<Arc<RwLock<StructField>>> {
        log::debug!(target: "store", "exorcising struct_field slot: {id}.");
        let result = self.struct_field.write().unwrap()[*id].take();
        self.struct_field_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructField>`.
    ///
    #[inline]
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

    /// Inter (insert) [`StructGeneric`] into the store.
    ///
    #[inline]
    pub fn inter_struct_generic<F>(&mut self, struct_generic: F) -> Arc<RwLock<StructGeneric>>
    where
        F: Fn(usize) -> Arc<RwLock<StructGeneric>>,
    {
        let _index = if let Some(_index) = self.struct_generic_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.struct_generic.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.struct_generic.write().unwrap().push(None);
            _index
        };

        let struct_generic = struct_generic(_index);

        let found = if let Some(struct_generic) =
            self.struct_generic.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *struct_generic.read().unwrap()
                } else {
                    false
                }
            }) {
            struct_generic.clone()
        } else {
            None
        };

        if let Some(struct_generic) = found {
            log::debug!(target: "store", "found duplicate {struct_generic:?}.");
            self.struct_generic_free_list.lock().unwrap().push(_index);
            struct_generic.clone()
        } else {
            log::debug!(target: "store", "interring {struct_generic:?}.");
            self.struct_generic.write().unwrap()[_index] = Some(struct_generic.clone());
            struct_generic
        }
    }

    /// Exhume (get) [`StructGeneric`] from the store.
    ///
    #[inline]
    pub fn exhume_struct_generic(&self, id: &usize) -> Option<Arc<RwLock<StructGeneric>>> {
        match self.struct_generic.read().unwrap().get(*id) {
            Some(struct_generic) => struct_generic.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`StructGeneric`] from the store.
    ///
    #[inline]
    pub fn exorcise_struct_generic(&mut self, id: &usize) -> Option<Arc<RwLock<StructGeneric>>> {
        log::debug!(target: "store", "exorcising struct_generic slot: {id}.");
        let result = self.struct_generic.write().unwrap()[*id].take();
        self.struct_generic_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructGeneric>`.
    ///
    #[inline]
    pub fn iter_struct_generic(&self) -> impl Iterator<Item = Arc<RwLock<StructGeneric>>> + '_ {
        let len = self.struct_generic.read().unwrap().len();
        (0..len)
            .filter(|i| self.struct_generic.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.struct_generic.read().unwrap()[i]
                    .as_ref()
                    .map(|struct_generic| struct_generic.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`TupleField`] into the store.
    ///
    #[inline]
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
    #[inline]
    pub fn exhume_tuple_field(&self, id: &usize) -> Option<Arc<RwLock<TupleField>>> {
        match self.tuple_field.read().unwrap().get(*id) {
            Some(tuple_field) => tuple_field.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`TupleField`] from the store.
    ///
    #[inline]
    pub fn exorcise_tuple_field(&mut self, id: &usize) -> Option<Arc<RwLock<TupleField>>> {
        log::debug!(target: "store", "exorcising tuple_field slot: {id}.");
        let result = self.tuple_field.write().unwrap()[*id].take();
        self.tuple_field_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, TupleField>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_type_cast(&self, id: &usize) -> Option<Arc<RwLock<TypeCast>>> {
        match self.type_cast.read().unwrap().get(*id) {
            Some(type_cast) => type_cast.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`TypeCast`] from the store.
    ///
    #[inline]
    pub fn exorcise_type_cast(&mut self, id: &usize) -> Option<Arc<RwLock<TypeCast>>> {
        log::debug!(target: "store", "exorcising type_cast slot: {id}.");
        let result = self.type_cast.write().unwrap()[*id].take();
        self.type_cast_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, TypeCast>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_unary(&self, id: &usize) -> Option<Arc<RwLock<Unary>>> {
        match self.unary.read().unwrap().get(*id) {
            Some(unary) => unary.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Unary`] from the store.
    ///
    #[inline]
    pub fn exorcise_unary(&mut self, id: &usize) -> Option<Arc<RwLock<Unary>>> {
        log::debug!(target: "store", "exorcising unary slot: {id}.");
        let result = self.unary.write().unwrap()[*id].take();
        self.unary_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Unary>`.
    ///
    #[inline]
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

    /// Inter (insert) [`Unit`] into the store.
    ///
    #[inline]
    pub fn inter_unit<F>(&mut self, unit: F) -> Arc<RwLock<Unit>>
    where
        F: Fn(usize) -> Arc<RwLock<Unit>>,
    {
        let _index = if let Some(_index) = self.unit_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.unit.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.unit.write().unwrap().push(None);
            _index
        };

        let unit = unit(_index);

        let found = if let Some(unit) = self.unit.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *unit.read().unwrap()
            } else {
                false
            }
        }) {
            unit.clone()
        } else {
            None
        };

        if let Some(unit) = found {
            log::debug!(target: "store", "found duplicate {unit:?}.");
            self.unit_free_list.lock().unwrap().push(_index);
            unit.clone()
        } else {
            log::debug!(target: "store", "interring {unit:?}.");
            self.unit.write().unwrap()[_index] = Some(unit.clone());
            unit
        }
    }

    /// Exhume (get) [`Unit`] from the store.
    ///
    #[inline]
    pub fn exhume_unit(&self, id: &usize) -> Option<Arc<RwLock<Unit>>> {
        match self.unit.read().unwrap().get(*id) {
            Some(unit) => unit.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Unit`] from the store.
    ///
    #[inline]
    pub fn exorcise_unit(&mut self, id: &usize) -> Option<Arc<RwLock<Unit>>> {
        log::debug!(target: "store", "exorcising unit slot: {id}.");
        let result = self.unit.write().unwrap()[*id].take();
        self.unit_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Unit>`.
    ///
    #[inline]
    pub fn iter_unit(&self) -> impl Iterator<Item = Arc<RwLock<Unit>>> + '_ {
        let len = self.unit.read().unwrap().len();
        (0..len)
            .filter(|i| self.unit.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.unit.read().unwrap()[i]
                    .as_ref()
                    .map(|unit| unit.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`UnnamedFieldExpression`] into the store.
    ///
    #[inline]
    pub fn inter_unnamed_field_expression<F>(
        &mut self,
        unnamed_field_expression: F,
    ) -> Arc<RwLock<UnnamedFieldExpression>>
    where
        F: Fn(usize) -> Arc<RwLock<UnnamedFieldExpression>>,
    {
        let _index = if let Some(_index) = self
            .unnamed_field_expression_free_list
            .lock()
            .unwrap()
            .pop()
        {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.unnamed_field_expression.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.unnamed_field_expression.write().unwrap().push(None);
            _index
        };

        let unnamed_field_expression = unnamed_field_expression(_index);

        let found = if let Some(unnamed_field_expression) = self
            .unnamed_field_expression
            .read()
            .unwrap()
            .iter()
            .find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *unnamed_field_expression.read().unwrap()
                } else {
                    false
                }
            }) {
            unnamed_field_expression.clone()
        } else {
            None
        };

        if let Some(unnamed_field_expression) = found {
            log::debug!(target: "store", "found duplicate {unnamed_field_expression:?}.");
            self.unnamed_field_expression_free_list
                .lock()
                .unwrap()
                .push(_index);
            unnamed_field_expression.clone()
        } else {
            log::debug!(target: "store", "interring {unnamed_field_expression:?}.");
            self.unnamed_field_expression.write().unwrap()[_index] =
                Some(unnamed_field_expression.clone());
            unnamed_field_expression
        }
    }

    /// Exhume (get) [`UnnamedFieldExpression`] from the store.
    ///
    #[inline]
    pub fn exhume_unnamed_field_expression(
        &self,
        id: &usize,
    ) -> Option<Arc<RwLock<UnnamedFieldExpression>>> {
        match self.unnamed_field_expression.read().unwrap().get(*id) {
            Some(unnamed_field_expression) => unnamed_field_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`UnnamedFieldExpression`] from the store.
    ///
    #[inline]
    pub fn exorcise_unnamed_field_expression(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<UnnamedFieldExpression>>> {
        log::debug!(target: "store", "exorcising unnamed_field_expression slot: {id}.");
        let result = self.unnamed_field_expression.write().unwrap()[*id].take();
        self.unnamed_field_expression_free_list
            .lock()
            .unwrap()
            .push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, UnnamedFieldExpression>`.
    ///
    #[inline]
    pub fn iter_unnamed_field_expression(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<UnnamedFieldExpression>>> + '_ {
        let len = self.unnamed_field_expression.read().unwrap().len();
        (0..len)
            .filter(|i| self.unnamed_field_expression.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.unnamed_field_expression.read().unwrap()[i]
                    .as_ref()
                    .map(|unnamed_field_expression| unnamed_field_expression.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`XValue`] into the store.
    ///
    #[inline]
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
    #[inline]
    pub fn exhume_x_value(&self, id: &usize) -> Option<Arc<RwLock<XValue>>> {
        match self.x_value.read().unwrap().get(*id) {
            Some(x_value) => x_value.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XValue`] from the store.
    ///
    #[inline]
    pub fn exorcise_x_value(&mut self, id: &usize) -> Option<Arc<RwLock<XValue>>> {
        log::debug!(target: "store", "exorcising x_value slot: {id}.");
        let result = self.x_value.write().unwrap()[*id].take();
        self.x_value_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XValue>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_value_type(&self, id: &usize) -> Option<Arc<RwLock<ValueType>>> {
        match self.value_type.read().unwrap().get(*id) {
            Some(value_type) => value_type.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ValueType`] from the store.
    ///
    #[inline]
    pub fn exorcise_value_type(&mut self, id: &usize) -> Option<Arc<RwLock<ValueType>>> {
        log::debug!(target: "store", "exorcising value_type slot: {id}.");
        let result = self.value_type.write().unwrap()[*id].take();
        self.value_type_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ValueType>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exhume_variable(&self, id: &usize) -> Option<Arc<RwLock<Variable>>> {
        match self.variable.read().unwrap().get(*id) {
            Some(variable) => variable.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Variable`] from the store.
    ///
    #[inline]
    pub fn exorcise_variable(&mut self, id: &usize) -> Option<Arc<RwLock<Variable>>> {
        log::debug!(target: "store", "exorcising variable slot: {id}.");
        let result = self.variable.write().unwrap()[*id].take();
        self.variable_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Variable>`.
    ///
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn exorcise_variable_expression(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<VariableExpression>>> {
        log::debug!(target: "store", "exorcising variable_expression slot: {id}.");
        let result = self.variable_expression.write().unwrap()[*id].take();
        self.variable_expression_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, VariableExpression>`.
    ///
    #[inline]
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

        // Persist Await.
        {
            let path = path.join("a_wait");
            fs::create_dir_all(&path)?;
            for a_wait in &*self.a_wait.read().unwrap() {
                if let Some(a_wait) = a_wait {
                    let path = path.join(format!("{}.json", a_wait.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &a_wait)?;
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

        // Persist Body.
        {
            let path = path.join("body");
            fs::create_dir_all(&path)?;
            for body in &*self.body.read().unwrap() {
                if let Some(body) = body {
                    let path = path.join(format!("{}.json", body.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &body)?;
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

        // Persist Data Structure.
        {
            let path = path.join("data_structure");
            fs::create_dir_all(&path)?;
            for data_structure in &*self.data_structure.read().unwrap() {
                if let Some(data_structure) = data_structure {
                    let path = path.join(format!("{}.json", data_structure.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &data_structure)?;
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

        // Persist Enum Field.
        {
            let path = path.join("enum_field");
            fs::create_dir_all(&path)?;
            for enum_field in &*self.enum_field.read().unwrap() {
                if let Some(enum_field) = enum_field {
                    let path = path.join(format!("{}.json", enum_field.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &enum_field)?;
                }
            }
        }

        // Persist Enum Generic.
        {
            let path = path.join("enum_generic");
            fs::create_dir_all(&path)?;
            for enum_generic in &*self.enum_generic.read().unwrap() {
                if let Some(enum_generic) = enum_generic {
                    let path = path.join(format!("{}.json", enum_generic.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &enum_generic)?;
                }
            }
        }

        // Persist Enumeration.
        {
            let path = path.join("enumeration");
            fs::create_dir_all(&path)?;
            for enumeration in &*self.enumeration.read().unwrap() {
                if let Some(enumeration) = enumeration {
                    let path = path.join(format!("{}.json", enumeration.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &enumeration)?;
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

        // Persist External Implementation.
        {
            let path = path.join("external_implementation");
            fs::create_dir_all(&path)?;
            for external_implementation in &*self.external_implementation.read().unwrap() {
                if let Some(external_implementation) = external_implementation {
                    let path = path.join(format!(
                        "{}.json",
                        external_implementation.read().unwrap().id
                    ));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &external_implementation)?;
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

        // Persist Function Call.
        {
            let path = path.join("function_call");
            fs::create_dir_all(&path)?;
            for function_call in &*self.function_call.read().unwrap() {
                if let Some(function_call) = function_call {
                    let path = path.join(format!("{}.json", function_call.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &function_call)?;
                }
            }
        }

        // Persist Future.
        {
            let path = path.join("x_future");
            fs::create_dir_all(&path)?;
            for x_future in &*self.x_future.read().unwrap() {
                if let Some(x_future) = x_future {
                    let path = path.join(format!("{}.json", x_future.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &x_future)?;
                }
            }
        }

        // Persist Generic.
        {
            let path = path.join("generic");
            fs::create_dir_all(&path)?;
            for generic in &*self.generic.read().unwrap() {
                if let Some(generic) = generic {
                    let path = path.join(format!("{}.json", generic.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &generic)?;
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

        // Persist Implementation Block.
        {
            let path = path.join("implementation_block");
            fs::create_dir_all(&path)?;
            for implementation_block in &*self.implementation_block.read().unwrap() {
                if let Some(implementation_block) = implementation_block {
                    let path =
                        path.join(format!("{}.json", implementation_block.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &implementation_block)?;
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

        // Persist Match.
        {
            let path = path.join("x_match");
            fs::create_dir_all(&path)?;
            for x_match in &*self.x_match.read().unwrap() {
                if let Some(x_match) = x_match {
                    let path = path.join(format!("{}.json", x_match.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &x_match)?;
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

        // Persist Named Field Expression.
        {
            let path = path.join("named_field_expression");
            fs::create_dir_all(&path)?;
            for named_field_expression in &*self.named_field_expression.read().unwrap() {
                if let Some(named_field_expression) = named_field_expression {
                    let path = path.join(format!(
                        "{}.json",
                        named_field_expression.read().unwrap().id
                    ));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &named_field_expression)?;
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

        // Persist Object Wrapper.
        {
            let path = path.join("object_wrapper");
            fs::create_dir_all(&path)?;
            for object_wrapper in &*self.object_wrapper.read().unwrap() {
                if let Some(object_wrapper) = object_wrapper {
                    let path = path.join(format!("{}.json", object_wrapper.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &object_wrapper)?;
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

        // Persist Path.
        {
            let path = path.join("x_path");
            fs::create_dir_all(&path)?;
            for x_path in &*self.x_path.read().unwrap() {
                if let Some(x_path) = x_path {
                    let path = path.join(format!("{}.json", x_path.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &x_path)?;
                }
            }
        }

        // Persist Path Element.
        {
            let path = path.join("path_element");
            fs::create_dir_all(&path)?;
            for path_element in &*self.path_element.read().unwrap() {
                if let Some(path_element) = path_element {
                    let path = path.join(format!("{}.json", path_element.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &path_element)?;
                }
            }
        }

        // Persist Pattern.
        {
            let path = path.join("pattern");
            fs::create_dir_all(&path)?;
            for pattern in &*self.pattern.read().unwrap() {
                if let Some(pattern) = pattern {
                    let path = path.join(format!("{}.json", pattern.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &pattern)?;
                }
            }
        }

        // Persist Plugin.
        {
            let path = path.join("x_plugin");
            fs::create_dir_all(&path)?;
            for x_plugin in &*self.x_plugin.read().unwrap() {
                if let Some(x_plugin) = x_plugin {
                    let path = path.join(format!("{}.json", x_plugin.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &x_plugin)?;
                }
            }
        }

        // Persist Print.
        {
            let path = path.join("x_print");
            fs::create_dir_all(&path)?;
            for x_print in &*self.x_print.read().unwrap() {
                if let Some(x_print) = x_print {
                    let path = path.join(format!("{}.json", x_print.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &x_print)?;
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

        // Persist Struct Field.
        {
            let path = path.join("struct_field");
            fs::create_dir_all(&path)?;
            for struct_field in &*self.struct_field.read().unwrap() {
                if let Some(struct_field) = struct_field {
                    let path = path.join(format!("{}.json", struct_field.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &struct_field)?;
                }
            }
        }

        // Persist Struct Generic.
        {
            let path = path.join("struct_generic");
            fs::create_dir_all(&path)?;
            for struct_generic in &*self.struct_generic.read().unwrap() {
                if let Some(struct_generic) = struct_generic {
                    let path = path.join(format!("{}.json", struct_generic.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &struct_generic)?;
                }
            }
        }

        // Persist Tuple Field.
        {
            let path = path.join("tuple_field");
            fs::create_dir_all(&path)?;
            for tuple_field in &*self.tuple_field.read().unwrap() {
                if let Some(tuple_field) = tuple_field {
                    let path = path.join(format!("{}.json", tuple_field.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &tuple_field)?;
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

        // Persist Unit.
        {
            let path = path.join("unit");
            fs::create_dir_all(&path)?;
            for unit in &*self.unit.read().unwrap() {
                if let Some(unit) = unit {
                    let path = path.join(format!("{}.json", unit.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &unit)?;
                }
            }
        }

        // Persist Unnamed Field Expression.
        {
            let path = path.join("unnamed_field_expression");
            fs::create_dir_all(&path)?;
            for unnamed_field_expression in &*self.unnamed_field_expression.read().unwrap() {
                if let Some(unnamed_field_expression) = unnamed_field_expression {
                    let path = path.join(format!(
                        "{}.json",
                        unnamed_field_expression.read().unwrap().id
                    ));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &unnamed_field_expression)?;
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

        // Load Await.
        {
            let path = path.join("a_wait");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let a_wait: Arc<RwLock<AWait>> = serde_json::from_reader(reader)?;
                store
                    .a_wait
                    .write()
                    .unwrap()
                    .insert(a_wait.read().unwrap().id, Some(a_wait.clone()));
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

        // Load Body.
        {
            let path = path.join("body");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let body: Arc<RwLock<Body>> = serde_json::from_reader(reader)?;
                store
                    .body
                    .write()
                    .unwrap()
                    .insert(body.read().unwrap().id, Some(body.clone()));
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

        // Load Data Structure.
        {
            let path = path.join("data_structure");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let data_structure: Arc<RwLock<DataStructure>> = serde_json::from_reader(reader)?;
                store.data_structure.write().unwrap().insert(
                    data_structure.read().unwrap().id,
                    Some(data_structure.clone()),
                );
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

        // Load Enum Field.
        {
            let path = path.join("enum_field");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let enum_field: Arc<RwLock<EnumField>> = serde_json::from_reader(reader)?;
                store
                    .enum_field
                    .write()
                    .unwrap()
                    .insert(enum_field.read().unwrap().id, Some(enum_field.clone()));
            }
        }

        // Load Enum Generic.
        {
            let path = path.join("enum_generic");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let enum_generic: Arc<RwLock<EnumGeneric>> = serde_json::from_reader(reader)?;
                store
                    .enum_generic
                    .write()
                    .unwrap()
                    .insert(enum_generic.read().unwrap().id, Some(enum_generic.clone()));
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
                let enumeration: Arc<RwLock<Enumeration>> = serde_json::from_reader(reader)?;
                store.enumeration_id_by_name.write().unwrap().insert(
                    enumeration.read().unwrap().name.to_owned(),
                    enumeration.read().unwrap().id,
                );
                store
                    .enumeration
                    .write()
                    .unwrap()
                    .insert(enumeration.read().unwrap().id, Some(enumeration.clone()));
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
                    serde_json::from_reader(reader)?;
                store.external_implementation.write().unwrap().insert(
                    external_implementation.read().unwrap().id,
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
                let field: Arc<RwLock<Field>> = serde_json::from_reader(reader)?;
                store.field_id_by_name.write().unwrap().insert(
                    field.read().unwrap().name.to_owned(),
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
                    function.read().unwrap().name.to_owned(),
                    function.read().unwrap().id,
                );
                store
                    .function
                    .write()
                    .unwrap()
                    .insert(function.read().unwrap().id, Some(function.clone()));
            }
        }

        // Load Function Call.
        {
            let path = path.join("function_call");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let function_call: Arc<RwLock<FunctionCall>> = serde_json::from_reader(reader)?;
                store.function_call.write().unwrap().insert(
                    function_call.read().unwrap().id,
                    Some(function_call.clone()),
                );
            }
        }

        // Load Future.
        {
            let path = path.join("x_future");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let x_future: Arc<RwLock<XFuture>> = serde_json::from_reader(reader)?;
                store
                    .x_future
                    .write()
                    .unwrap()
                    .insert(x_future.read().unwrap().id, Some(x_future.clone()));
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
                let generic: Arc<RwLock<Generic>> = serde_json::from_reader(reader)?;
                store
                    .generic
                    .write()
                    .unwrap()
                    .insert(generic.read().unwrap().id, Some(generic.clone()));
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
                    serde_json::from_reader(reader)?;
                store.implementation_block.write().unwrap().insert(
                    implementation_block.read().unwrap().id,
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

        // Load Match.
        {
            let path = path.join("x_match");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let x_match: Arc<RwLock<XMatch>> = serde_json::from_reader(reader)?;
                store
                    .x_match
                    .write()
                    .unwrap()
                    .insert(x_match.read().unwrap().id, Some(x_match.clone()));
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

        // Load Named Field Expression.
        {
            let path = path.join("named_field_expression");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let named_field_expression: Arc<RwLock<NamedFieldExpression>> =
                    serde_json::from_reader(reader)?;
                store.named_field_expression.write().unwrap().insert(
                    named_field_expression.read().unwrap().id,
                    Some(named_field_expression.clone()),
                );
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
                store.z_object_store_id_by_name.write().unwrap().insert(
                    z_object_store.read().unwrap().name.to_owned(),
                    z_object_store.read().unwrap().id,
                );
                store.z_object_store.write().unwrap().insert(
                    z_object_store.read().unwrap().id,
                    Some(z_object_store.clone()),
                );
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
                let object_wrapper: Arc<RwLock<ObjectWrapper>> = serde_json::from_reader(reader)?;
                store.object_wrapper.write().unwrap().insert(
                    object_wrapper.read().unwrap().id,
                    Some(object_wrapper.clone()),
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

        // Load Path.
        {
            let path = path.join("x_path");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let x_path: Arc<RwLock<XPath>> = serde_json::from_reader(reader)?;
                store
                    .x_path
                    .write()
                    .unwrap()
                    .insert(x_path.read().unwrap().id, Some(x_path.clone()));
            }
        }

        // Load Path Element.
        {
            let path = path.join("path_element");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let path_element: Arc<RwLock<PathElement>> = serde_json::from_reader(reader)?;
                store
                    .path_element
                    .write()
                    .unwrap()
                    .insert(path_element.read().unwrap().id, Some(path_element.clone()));
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
                let pattern: Arc<RwLock<Pattern>> = serde_json::from_reader(reader)?;
                store
                    .pattern
                    .write()
                    .unwrap()
                    .insert(pattern.read().unwrap().id, Some(pattern.clone()));
            }
        }

        // Load Plugin.
        {
            let path = path.join("x_plugin");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let x_plugin: Arc<RwLock<XPlugin>> = serde_json::from_reader(reader)?;
                store.x_plugin_id_by_name.write().unwrap().insert(
                    x_plugin.read().unwrap().name.to_owned(),
                    x_plugin.read().unwrap().id,
                );
                store
                    .x_plugin
                    .write()
                    .unwrap()
                    .insert(x_plugin.read().unwrap().id, Some(x_plugin.clone()));
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
                let x_print: Arc<RwLock<XPrint>> = serde_json::from_reader(reader)?;
                store
                    .x_print
                    .write()
                    .unwrap()
                    .insert(x_print.read().unwrap().id, Some(x_print.clone()));
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
                    woog_struct.read().unwrap().name.to_owned(),
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

        // Load Struct Field.
        {
            let path = path.join("struct_field");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let struct_field: Arc<RwLock<StructField>> = serde_json::from_reader(reader)?;
                store
                    .struct_field
                    .write()
                    .unwrap()
                    .insert(struct_field.read().unwrap().id, Some(struct_field.clone()));
            }
        }

        // Load Struct Generic.
        {
            let path = path.join("struct_generic");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let struct_generic: Arc<RwLock<StructGeneric>> = serde_json::from_reader(reader)?;
                store.struct_generic.write().unwrap().insert(
                    struct_generic.read().unwrap().id,
                    Some(struct_generic.clone()),
                );
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
                let tuple_field: Arc<RwLock<TupleField>> = serde_json::from_reader(reader)?;
                store
                    .tuple_field
                    .write()
                    .unwrap()
                    .insert(tuple_field.read().unwrap().id, Some(tuple_field.clone()));
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

        // Load Unit.
        {
            let path = path.join("unit");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let unit: Arc<RwLock<Unit>> = serde_json::from_reader(reader)?;
                store
                    .unit
                    .write()
                    .unwrap()
                    .insert(unit.read().unwrap().id, Some(unit.clone()));
            }
        }

        // Load Unnamed Field Expression.
        {
            let path = path.join("unnamed_field_expression");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let unnamed_field_expression: Arc<RwLock<UnnamedFieldExpression>> =
                    serde_json::from_reader(reader)?;
                store.unnamed_field_expression.write().unwrap().insert(
                    unnamed_field_expression.read().unwrap().id,
                    Some(unnamed_field_expression.clone()),
                );
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
