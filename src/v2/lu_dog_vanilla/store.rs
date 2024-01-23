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
//! * [`FuncGeneric`]
//! * [`Function`]
//! * [`FunctionCall`]
//! * [`XFuture`]
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_vanilla-object-store-definition"}}}
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
};

use heck::ToUpperCamelCase;
use rustc_hash::FxHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v2::lu_dog_vanilla::types::{
    AWait, Argument, Binary, Block, Body, BooleanLiteral, BooleanOperator, Call, Comparison,
    DataStructure, DwarfSourceFile, EnumField, EnumGeneric, Enumeration, Expression,
    ExpressionStatement, ExternalImplementation, Field, FieldAccess, FieldAccessTarget,
    FieldExpression, FloatLiteral, ForLoop, FuncGeneric, Function, FunctionCall, Grouped,
    ImplementationBlock, Import, Index, IntegerLiteral, Item, Lambda, LambdaParameter,
    LetStatement, List, ListElement, ListExpression, Literal, LocalVariable, MethodCall,
    NamedFieldExpression, ObjectWrapper, Operator, Parameter, PathElement, Pattern,
    RangeExpression, ResultStatement, Span, Statement, StaticMethodCall, StringLiteral,
    StructExpression, StructField, StructGeneric, TupleField, TypeCast, Unary, Unit,
    UnnamedFieldExpression, ValueType, Variable, VariableExpression, WoogStruct, XFuture, XIf,
    XMacro, XMatch, XPath, XPlugin, XPrint, XReturn, XValue, ZObjectStore,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    argument: HashMap<Uuid, Argument>,
    a_wait: HashMap<Uuid, AWait>,
    binary: HashMap<Uuid, Binary>,
    block: HashMap<Uuid, Block>,
    body: HashMap<Uuid, Body>,
    boolean_literal: HashMap<Uuid, BooleanLiteral>,
    boolean_operator: HashMap<Uuid, BooleanOperator>,
    call: HashMap<Uuid, Call>,
    comparison: HashMap<Uuid, Comparison>,
    data_structure: HashMap<Uuid, DataStructure>,
    dwarf_source_file: HashMap<Uuid, DwarfSourceFile>,
    enum_field: HashMap<Uuid, EnumField>,
    enum_generic: HashMap<Uuid, EnumGeneric>,
    enumeration: HashMap<Uuid, Enumeration>,
    enumeration_id_by_name: HashMap<String, Uuid>,
    expression: HashMap<Uuid, Expression>,
    expression_statement: HashMap<Uuid, ExpressionStatement>,
    external_implementation: HashMap<Uuid, ExternalImplementation>,
    field: HashMap<Uuid, Field>,
    field_id_by_name: HashMap<String, Uuid>,
    field_access: HashMap<Uuid, FieldAccess>,
    field_access_target: HashMap<Uuid, FieldAccessTarget>,
    field_expression: HashMap<Uuid, FieldExpression>,
    float_literal: HashMap<Uuid, FloatLiteral>,
    for_loop: HashMap<Uuid, ForLoop>,
    func_generic: HashMap<Uuid, FuncGeneric>,
    function: HashMap<Uuid, Function>,
    function_id_by_name: HashMap<String, Uuid>,
    function_call: HashMap<Uuid, FunctionCall>,
    x_future: HashMap<Uuid, XFuture>,
    grouped: HashMap<Uuid, Grouped>,
    x_if: HashMap<Uuid, XIf>,
    implementation_block: HashMap<Uuid, ImplementationBlock>,
    import: HashMap<Uuid, Import>,
    index: HashMap<Uuid, Index>,
    integer_literal: HashMap<Uuid, IntegerLiteral>,
    item: HashMap<Uuid, Item>,
    lambda: HashMap<Uuid, Lambda>,
    lambda_parameter: HashMap<Uuid, LambdaParameter>,
    let_statement: HashMap<Uuid, LetStatement>,
    list: HashMap<Uuid, List>,
    list_element: HashMap<Uuid, ListElement>,
    list_expression: HashMap<Uuid, ListExpression>,
    literal: HashMap<Uuid, Literal>,
    local_variable: HashMap<Uuid, LocalVariable>,
    x_macro: HashMap<Uuid, XMacro>,
    x_match: HashMap<Uuid, XMatch>,
    method_call: HashMap<Uuid, MethodCall>,
    named_field_expression: HashMap<Uuid, NamedFieldExpression>,
    z_object_store: HashMap<Uuid, ZObjectStore>,
    z_object_store_id_by_name: HashMap<String, Uuid>,
    object_wrapper: HashMap<Uuid, ObjectWrapper>,
    operator: HashMap<Uuid, Operator>,
    parameter: HashMap<Uuid, Parameter>,
    x_path: HashMap<Uuid, XPath>,
    path_element: HashMap<Uuid, PathElement>,
    pattern: HashMap<Uuid, Pattern>,
    x_plugin: HashMap<Uuid, XPlugin>,
    x_plugin_id_by_name: HashMap<String, Uuid>,
    x_print: HashMap<Uuid, XPrint>,
    range_expression: HashMap<Uuid, RangeExpression>,
    result_statement: HashMap<Uuid, ResultStatement>,
    x_return: HashMap<Uuid, XReturn>,
    span: HashMap<Uuid, Span>,
    statement: HashMap<Uuid, Statement>,
    static_method_call: HashMap<Uuid, StaticMethodCall>,
    string_literal: HashMap<Uuid, StringLiteral>,
    woog_struct: HashMap<Uuid, WoogStruct>,
    woog_struct_id_by_name: HashMap<String, Uuid>,
    struct_expression: HashMap<Uuid, StructExpression>,
    struct_field: HashMap<Uuid, StructField>,
    struct_generic: HashMap<Uuid, StructGeneric>,
    tuple_field: HashMap<Uuid, TupleField>,
    type_cast: HashMap<Uuid, TypeCast>,
    unary: HashMap<Uuid, Unary>,
    unit: HashMap<Uuid, Unit>,
    unnamed_field_expression: HashMap<Uuid, UnnamedFieldExpression>,
    x_value: HashMap<Uuid, XValue>,
    value_type: HashMap<Uuid, ValueType>,
    variable: HashMap<Uuid, Variable>,
    variable_expression: HashMap<Uuid, VariableExpression>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let store = Self {
            argument: HashMap::default(),
            a_wait: HashMap::default(),
            binary: HashMap::default(),
            block: HashMap::default(),
            body: HashMap::default(),
            boolean_literal: HashMap::default(),
            boolean_operator: HashMap::default(),
            call: HashMap::default(),
            comparison: HashMap::default(),
            data_structure: HashMap::default(),
            dwarf_source_file: HashMap::default(),
            enum_field: HashMap::default(),
            enum_generic: HashMap::default(),
            enumeration: HashMap::default(),
            enumeration_id_by_name: HashMap::default(),
            expression: HashMap::default(),
            expression_statement: HashMap::default(),
            external_implementation: HashMap::default(),
            field: HashMap::default(),
            field_id_by_name: HashMap::default(),
            field_access: HashMap::default(),
            field_access_target: HashMap::default(),
            field_expression: HashMap::default(),
            float_literal: HashMap::default(),
            for_loop: HashMap::default(),
            func_generic: HashMap::default(),
            function: HashMap::default(),
            function_id_by_name: HashMap::default(),
            function_call: HashMap::default(),
            x_future: HashMap::default(),
            grouped: HashMap::default(),
            x_if: HashMap::default(),
            implementation_block: HashMap::default(),
            import: HashMap::default(),
            index: HashMap::default(),
            integer_literal: HashMap::default(),
            item: HashMap::default(),
            lambda: HashMap::default(),
            lambda_parameter: HashMap::default(),
            let_statement: HashMap::default(),
            list: HashMap::default(),
            list_element: HashMap::default(),
            list_expression: HashMap::default(),
            literal: HashMap::default(),
            local_variable: HashMap::default(),
            x_macro: HashMap::default(),
            x_match: HashMap::default(),
            method_call: HashMap::default(),
            named_field_expression: HashMap::default(),
            z_object_store: HashMap::default(),
            z_object_store_id_by_name: HashMap::default(),
            object_wrapper: HashMap::default(),
            operator: HashMap::default(),
            parameter: HashMap::default(),
            x_path: HashMap::default(),
            path_element: HashMap::default(),
            pattern: HashMap::default(),
            x_plugin: HashMap::default(),
            x_plugin_id_by_name: HashMap::default(),
            x_print: HashMap::default(),
            range_expression: HashMap::default(),
            result_statement: HashMap::default(),
            x_return: HashMap::default(),
            span: HashMap::default(),
            statement: HashMap::default(),
            static_method_call: HashMap::default(),
            string_literal: HashMap::default(),
            woog_struct: HashMap::default(),
            woog_struct_id_by_name: HashMap::default(),
            struct_expression: HashMap::default(),
            struct_field: HashMap::default(),
            struct_generic: HashMap::default(),
            tuple_field: HashMap::default(),
            type_cast: HashMap::default(),
            unary: HashMap::default(),
            unit: HashMap::default(),
            unnamed_field_expression: HashMap::default(),
            x_value: HashMap::default(),
            value_type: HashMap::default(),
            variable: HashMap::default(),
            variable_expression: HashMap::default(),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_vanilla-object-store-methods"}}}
    /// Inter (insert) [`Argument`] into the store.
    ///
    pub fn inter_argument(&mut self, argument: Argument) {
        self.argument.insert(argument.id, argument);
    }

    /// Exhume (get) [`Argument`] from the store.
    ///
    pub fn exhume_argument(&self, id: &Uuid) -> Option<&Argument> {
        self.argument.get(id)
    }

    /// Exorcise (remove) [`Argument`] from the store.
    ///
    pub fn exorcise_argument(&mut self, id: &Uuid) -> Option<Argument> {
        self.argument.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Argument>`.
    ///
    pub fn iter_argument(&self) -> impl Iterator<Item = &Argument> {
        self.argument.values()
    }

    /// Inter (insert) [`AWait`] into the store.
    ///
    pub fn inter_a_wait(&mut self, a_wait: AWait) {
        self.a_wait.insert(a_wait.id, a_wait);
    }

    /// Exhume (get) [`AWait`] from the store.
    ///
    pub fn exhume_a_wait(&self, id: &Uuid) -> Option<&AWait> {
        self.a_wait.get(id)
    }

    /// Exorcise (remove) [`AWait`] from the store.
    ///
    pub fn exorcise_a_wait(&mut self, id: &Uuid) -> Option<AWait> {
        self.a_wait.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, AWait>`.
    ///
    pub fn iter_a_wait(&self) -> impl Iterator<Item = &AWait> {
        self.a_wait.values()
    }

    /// Inter (insert) [`Binary`] into the store.
    ///
    pub fn inter_binary(&mut self, binary: Binary) {
        self.binary.insert(binary.id, binary);
    }

    /// Exhume (get) [`Binary`] from the store.
    ///
    pub fn exhume_binary(&self, id: &Uuid) -> Option<&Binary> {
        self.binary.get(id)
    }

    /// Exorcise (remove) [`Binary`] from the store.
    ///
    pub fn exorcise_binary(&mut self, id: &Uuid) -> Option<Binary> {
        self.binary.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Binary>`.
    ///
    pub fn iter_binary(&self) -> impl Iterator<Item = &Binary> {
        self.binary.values()
    }

    /// Inter (insert) [`Block`] into the store.
    ///
    pub fn inter_block(&mut self, block: Block) {
        self.block.insert(block.id, block);
    }

    /// Exhume (get) [`Block`] from the store.
    ///
    pub fn exhume_block(&self, id: &Uuid) -> Option<&Block> {
        self.block.get(id)
    }

    /// Exorcise (remove) [`Block`] from the store.
    ///
    pub fn exorcise_block(&mut self, id: &Uuid) -> Option<Block> {
        self.block.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Block>`.
    ///
    pub fn iter_block(&self) -> impl Iterator<Item = &Block> {
        self.block.values()
    }

    /// Inter (insert) [`Body`] into the store.
    ///
    pub fn inter_body(&mut self, body: Body) {
        self.body.insert(body.id, body);
    }

    /// Exhume (get) [`Body`] from the store.
    ///
    pub fn exhume_body(&self, id: &Uuid) -> Option<&Body> {
        self.body.get(id)
    }

    /// Exorcise (remove) [`Body`] from the store.
    ///
    pub fn exorcise_body(&mut self, id: &Uuid) -> Option<Body> {
        self.body.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Body>`.
    ///
    pub fn iter_body(&self) -> impl Iterator<Item = &Body> {
        self.body.values()
    }

    /// Inter (insert) [`BooleanLiteral`] into the store.
    ///
    pub fn inter_boolean_literal(&mut self, boolean_literal: BooleanLiteral) {
        self.boolean_literal
            .insert(boolean_literal.id, boolean_literal);
    }

    /// Exhume (get) [`BooleanLiteral`] from the store.
    ///
    pub fn exhume_boolean_literal(&self, id: &Uuid) -> Option<&BooleanLiteral> {
        self.boolean_literal.get(id)
    }

    /// Exorcise (remove) [`BooleanLiteral`] from the store.
    ///
    pub fn exorcise_boolean_literal(&mut self, id: &Uuid) -> Option<BooleanLiteral> {
        self.boolean_literal.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanLiteral>`.
    ///
    pub fn iter_boolean_literal(&self) -> impl Iterator<Item = &BooleanLiteral> {
        self.boolean_literal.values()
    }

    /// Inter (insert) [`BooleanOperator`] into the store.
    ///
    pub fn inter_boolean_operator(&mut self, boolean_operator: BooleanOperator) {
        self.boolean_operator
            .insert(boolean_operator.id, boolean_operator);
    }

    /// Exhume (get) [`BooleanOperator`] from the store.
    ///
    pub fn exhume_boolean_operator(&self, id: &Uuid) -> Option<&BooleanOperator> {
        self.boolean_operator.get(id)
    }

    /// Exorcise (remove) [`BooleanOperator`] from the store.
    ///
    pub fn exorcise_boolean_operator(&mut self, id: &Uuid) -> Option<BooleanOperator> {
        self.boolean_operator.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanOperator>`.
    ///
    pub fn iter_boolean_operator(&self) -> impl Iterator<Item = &BooleanOperator> {
        self.boolean_operator.values()
    }

    /// Inter (insert) [`Call`] into the store.
    ///
    pub fn inter_call(&mut self, call: Call) {
        self.call.insert(call.id, call);
    }

    /// Exhume (get) [`Call`] from the store.
    ///
    pub fn exhume_call(&self, id: &Uuid) -> Option<&Call> {
        self.call.get(id)
    }

    /// Exorcise (remove) [`Call`] from the store.
    ///
    pub fn exorcise_call(&mut self, id: &Uuid) -> Option<Call> {
        self.call.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Call>`.
    ///
    pub fn iter_call(&self) -> impl Iterator<Item = &Call> {
        self.call.values()
    }

    /// Inter (insert) [`Comparison`] into the store.
    ///
    pub fn inter_comparison(&mut self, comparison: Comparison) {
        self.comparison.insert(comparison.id, comparison);
    }

    /// Exhume (get) [`Comparison`] from the store.
    ///
    pub fn exhume_comparison(&self, id: &Uuid) -> Option<&Comparison> {
        self.comparison.get(id)
    }

    /// Exorcise (remove) [`Comparison`] from the store.
    ///
    pub fn exorcise_comparison(&mut self, id: &Uuid) -> Option<Comparison> {
        self.comparison.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Comparison>`.
    ///
    pub fn iter_comparison(&self) -> impl Iterator<Item = &Comparison> {
        self.comparison.values()
    }

    /// Inter (insert) [`DataStructure`] into the store.
    ///
    pub fn inter_data_structure(&mut self, data_structure: DataStructure) {
        self.data_structure
            .insert(data_structure.id, data_structure);
    }

    /// Exhume (get) [`DataStructure`] from the store.
    ///
    pub fn exhume_data_structure(&self, id: &Uuid) -> Option<&DataStructure> {
        self.data_structure.get(id)
    }

    /// Exorcise (remove) [`DataStructure`] from the store.
    ///
    pub fn exorcise_data_structure(&mut self, id: &Uuid) -> Option<DataStructure> {
        self.data_structure.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, DataStructure>`.
    ///
    pub fn iter_data_structure(&self) -> impl Iterator<Item = &DataStructure> {
        self.data_structure.values()
    }

    /// Inter (insert) [`DwarfSourceFile`] into the store.
    ///
    pub fn inter_dwarf_source_file(&mut self, dwarf_source_file: DwarfSourceFile) {
        self.dwarf_source_file
            .insert(dwarf_source_file.id, dwarf_source_file);
    }

    /// Exhume (get) [`DwarfSourceFile`] from the store.
    ///
    pub fn exhume_dwarf_source_file(&self, id: &Uuid) -> Option<&DwarfSourceFile> {
        self.dwarf_source_file.get(id)
    }

    /// Exorcise (remove) [`DwarfSourceFile`] from the store.
    ///
    pub fn exorcise_dwarf_source_file(&mut self, id: &Uuid) -> Option<DwarfSourceFile> {
        self.dwarf_source_file.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, DwarfSourceFile>`.
    ///
    pub fn iter_dwarf_source_file(&self) -> impl Iterator<Item = &DwarfSourceFile> {
        self.dwarf_source_file.values()
    }

    /// Inter (insert) [`EnumField`] into the store.
    ///
    pub fn inter_enum_field(&mut self, enum_field: EnumField) {
        self.enum_field.insert(enum_field.id, enum_field);
    }

    /// Exhume (get) [`EnumField`] from the store.
    ///
    pub fn exhume_enum_field(&self, id: &Uuid) -> Option<&EnumField> {
        self.enum_field.get(id)
    }

    /// Exorcise (remove) [`EnumField`] from the store.
    ///
    pub fn exorcise_enum_field(&mut self, id: &Uuid) -> Option<EnumField> {
        self.enum_field.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, EnumField>`.
    ///
    pub fn iter_enum_field(&self) -> impl Iterator<Item = &EnumField> {
        self.enum_field.values()
    }

    /// Inter (insert) [`EnumGeneric`] into the store.
    ///
    pub fn inter_enum_generic(&mut self, enum_generic: EnumGeneric) {
        self.enum_generic.insert(enum_generic.id, enum_generic);
    }

    /// Exhume (get) [`EnumGeneric`] from the store.
    ///
    pub fn exhume_enum_generic(&self, id: &Uuid) -> Option<&EnumGeneric> {
        self.enum_generic.get(id)
    }

    /// Exorcise (remove) [`EnumGeneric`] from the store.
    ///
    pub fn exorcise_enum_generic(&mut self, id: &Uuid) -> Option<EnumGeneric> {
        self.enum_generic.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, EnumGeneric>`.
    ///
    pub fn iter_enum_generic(&self) -> impl Iterator<Item = &EnumGeneric> {
        self.enum_generic.values()
    }

    /// Inter (insert) [`Enumeration`] into the store.
    ///
    pub fn inter_enumeration(&mut self, enumeration: Enumeration) {
        self.enumeration_id_by_name
            .insert(enumeration.name.to_upper_camel_case(), enumeration.id);
        self.enumeration.insert(enumeration.id, enumeration);
    }

    /// Exhume (get) [`Enumeration`] from the store.
    ///
    pub fn exhume_enumeration(&self, id: &Uuid) -> Option<&Enumeration> {
        self.enumeration.get(id)
    }

    /// Exorcise (remove) [`Enumeration`] from the store.
    ///
    pub fn exorcise_enumeration(&mut self, id: &Uuid) -> Option<Enumeration> {
        self.enumeration.remove(id)
    }

    /// Exhume [`Enumeration`] id from the store by name.
    ///
    pub fn exhume_enumeration_id_by_name(&self, name: &str) -> Option<&Uuid> {
        self.enumeration_id_by_name.get(name)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Enumeration>`.
    ///
    pub fn iter_enumeration(&self) -> impl Iterator<Item = &Enumeration> {
        self.enumeration.values()
    }

    /// Inter (insert) [`Expression`] into the store.
    ///
    pub fn inter_expression(&mut self, expression: Expression) {
        self.expression.insert(expression.id, expression);
    }

    /// Exhume (get) [`Expression`] from the store.
    ///
    pub fn exhume_expression(&self, id: &Uuid) -> Option<&Expression> {
        self.expression.get(id)
    }

    /// Exorcise (remove) [`Expression`] from the store.
    ///
    pub fn exorcise_expression(&mut self, id: &Uuid) -> Option<Expression> {
        self.expression.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Expression>`.
    ///
    pub fn iter_expression(&self) -> impl Iterator<Item = &Expression> {
        self.expression.values()
    }

    /// Inter (insert) [`ExpressionStatement`] into the store.
    ///
    pub fn inter_expression_statement(&mut self, expression_statement: ExpressionStatement) {
        self.expression_statement
            .insert(expression_statement.id, expression_statement);
    }

    /// Exhume (get) [`ExpressionStatement`] from the store.
    ///
    pub fn exhume_expression_statement(&self, id: &Uuid) -> Option<&ExpressionStatement> {
        self.expression_statement.get(id)
    }

    /// Exorcise (remove) [`ExpressionStatement`] from the store.
    ///
    pub fn exorcise_expression_statement(&mut self, id: &Uuid) -> Option<ExpressionStatement> {
        self.expression_statement.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ExpressionStatement>`.
    ///
    pub fn iter_expression_statement(&self) -> impl Iterator<Item = &ExpressionStatement> {
        self.expression_statement.values()
    }

    /// Inter (insert) [`ExternalImplementation`] into the store.
    ///
    pub fn inter_external_implementation(
        &mut self,
        external_implementation: ExternalImplementation,
    ) {
        self.external_implementation
            .insert(external_implementation.id, external_implementation);
    }

    /// Exhume (get) [`ExternalImplementation`] from the store.
    ///
    pub fn exhume_external_implementation(&self, id: &Uuid) -> Option<&ExternalImplementation> {
        self.external_implementation.get(id)
    }

    /// Exorcise (remove) [`ExternalImplementation`] from the store.
    ///
    pub fn exorcise_external_implementation(
        &mut self,
        id: &Uuid,
    ) -> Option<ExternalImplementation> {
        self.external_implementation.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ExternalImplementation>`.
    ///
    pub fn iter_external_implementation(&self) -> impl Iterator<Item = &ExternalImplementation> {
        self.external_implementation.values()
    }

    /// Inter (insert) [`Field`] into the store.
    ///
    pub fn inter_field(&mut self, field: Field) {
        self.field_id_by_name
            .insert(field.name.to_upper_camel_case(), field.id);
        self.field.insert(field.id, field);
    }

    /// Exhume (get) [`Field`] from the store.
    ///
    pub fn exhume_field(&self, id: &Uuid) -> Option<&Field> {
        self.field.get(id)
    }

    /// Exorcise (remove) [`Field`] from the store.
    ///
    pub fn exorcise_field(&mut self, id: &Uuid) -> Option<Field> {
        self.field.remove(id)
    }

    /// Exhume [`Field`] id from the store by name.
    ///
    pub fn exhume_field_id_by_name(&self, name: &str) -> Option<&Uuid> {
        self.field_id_by_name.get(name)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Field>`.
    ///
    pub fn iter_field(&self) -> impl Iterator<Item = &Field> {
        self.field.values()
    }

    /// Inter (insert) [`FieldAccess`] into the store.
    ///
    pub fn inter_field_access(&mut self, field_access: FieldAccess) {
        self.field_access.insert(field_access.id, field_access);
    }

    /// Exhume (get) [`FieldAccess`] from the store.
    ///
    pub fn exhume_field_access(&self, id: &Uuid) -> Option<&FieldAccess> {
        self.field_access.get(id)
    }

    /// Exorcise (remove) [`FieldAccess`] from the store.
    ///
    pub fn exorcise_field_access(&mut self, id: &Uuid) -> Option<FieldAccess> {
        self.field_access.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccess>`.
    ///
    pub fn iter_field_access(&self) -> impl Iterator<Item = &FieldAccess> {
        self.field_access.values()
    }

    /// Inter (insert) [`FieldAccessTarget`] into the store.
    ///
    pub fn inter_field_access_target(&mut self, field_access_target: FieldAccessTarget) {
        self.field_access_target
            .insert(field_access_target.id, field_access_target);
    }

    /// Exhume (get) [`FieldAccessTarget`] from the store.
    ///
    pub fn exhume_field_access_target(&self, id: &Uuid) -> Option<&FieldAccessTarget> {
        self.field_access_target.get(id)
    }

    /// Exorcise (remove) [`FieldAccessTarget`] from the store.
    ///
    pub fn exorcise_field_access_target(&mut self, id: &Uuid) -> Option<FieldAccessTarget> {
        self.field_access_target.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccessTarget>`.
    ///
    pub fn iter_field_access_target(&self) -> impl Iterator<Item = &FieldAccessTarget> {
        self.field_access_target.values()
    }

    /// Inter (insert) [`FieldExpression`] into the store.
    ///
    pub fn inter_field_expression(&mut self, field_expression: FieldExpression) {
        self.field_expression
            .insert(field_expression.id, field_expression);
    }

    /// Exhume (get) [`FieldExpression`] from the store.
    ///
    pub fn exhume_field_expression(&self, id: &Uuid) -> Option<&FieldExpression> {
        self.field_expression.get(id)
    }

    /// Exorcise (remove) [`FieldExpression`] from the store.
    ///
    pub fn exorcise_field_expression(&mut self, id: &Uuid) -> Option<FieldExpression> {
        self.field_expression.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldExpression>`.
    ///
    pub fn iter_field_expression(&self) -> impl Iterator<Item = &FieldExpression> {
        self.field_expression.values()
    }

    /// Inter (insert) [`FloatLiteral`] into the store.
    ///
    pub fn inter_float_literal(&mut self, float_literal: FloatLiteral) {
        self.float_literal.insert(float_literal.id, float_literal);
    }

    /// Exhume (get) [`FloatLiteral`] from the store.
    ///
    pub fn exhume_float_literal(&self, id: &Uuid) -> Option<&FloatLiteral> {
        self.float_literal.get(id)
    }

    /// Exorcise (remove) [`FloatLiteral`] from the store.
    ///
    pub fn exorcise_float_literal(&mut self, id: &Uuid) -> Option<FloatLiteral> {
        self.float_literal.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FloatLiteral>`.
    ///
    pub fn iter_float_literal(&self) -> impl Iterator<Item = &FloatLiteral> {
        self.float_literal.values()
    }

    /// Inter (insert) [`ForLoop`] into the store.
    ///
    pub fn inter_for_loop(&mut self, for_loop: ForLoop) {
        self.for_loop.insert(for_loop.id, for_loop);
    }

    /// Exhume (get) [`ForLoop`] from the store.
    ///
    pub fn exhume_for_loop(&self, id: &Uuid) -> Option<&ForLoop> {
        self.for_loop.get(id)
    }

    /// Exorcise (remove) [`ForLoop`] from the store.
    ///
    pub fn exorcise_for_loop(&mut self, id: &Uuid) -> Option<ForLoop> {
        self.for_loop.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ForLoop>`.
    ///
    pub fn iter_for_loop(&self) -> impl Iterator<Item = &ForLoop> {
        self.for_loop.values()
    }

    /// Inter (insert) [`FuncGeneric`] into the store.
    ///
    pub fn inter_func_generic(&mut self, func_generic: FuncGeneric) {
        self.func_generic.insert(func_generic.id, func_generic);
    }

    /// Exhume (get) [`FuncGeneric`] from the store.
    ///
    pub fn exhume_func_generic(&self, id: &Uuid) -> Option<&FuncGeneric> {
        self.func_generic.get(id)
    }

    /// Exorcise (remove) [`FuncGeneric`] from the store.
    ///
    pub fn exorcise_func_generic(&mut self, id: &Uuid) -> Option<FuncGeneric> {
        self.func_generic.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FuncGeneric>`.
    ///
    pub fn iter_func_generic(&self) -> impl Iterator<Item = &FuncGeneric> {
        self.func_generic.values()
    }

    /// Inter (insert) [`Function`] into the store.
    ///
    pub fn inter_function(&mut self, function: Function) {
        self.function_id_by_name
            .insert(function.name.to_upper_camel_case(), function.id);
        self.function.insert(function.id, function);
    }

    /// Exhume (get) [`Function`] from the store.
    ///
    pub fn exhume_function(&self, id: &Uuid) -> Option<&Function> {
        self.function.get(id)
    }

    /// Exorcise (remove) [`Function`] from the store.
    ///
    pub fn exorcise_function(&mut self, id: &Uuid) -> Option<Function> {
        self.function.remove(id)
    }

    /// Exhume [`Function`] id from the store by name.
    ///
    pub fn exhume_function_id_by_name(&self, name: &str) -> Option<&Uuid> {
        self.function_id_by_name.get(name)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Function>`.
    ///
    pub fn iter_function(&self) -> impl Iterator<Item = &Function> {
        self.function.values()
    }

    /// Inter (insert) [`FunctionCall`] into the store.
    ///
    pub fn inter_function_call(&mut self, function_call: FunctionCall) {
        self.function_call.insert(function_call.id, function_call);
    }

    /// Exhume (get) [`FunctionCall`] from the store.
    ///
    pub fn exhume_function_call(&self, id: &Uuid) -> Option<&FunctionCall> {
        self.function_call.get(id)
    }

    /// Exorcise (remove) [`FunctionCall`] from the store.
    ///
    pub fn exorcise_function_call(&mut self, id: &Uuid) -> Option<FunctionCall> {
        self.function_call.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FunctionCall>`.
    ///
    pub fn iter_function_call(&self) -> impl Iterator<Item = &FunctionCall> {
        self.function_call.values()
    }

    /// Inter (insert) [`XFuture`] into the store.
    ///
    pub fn inter_x_future(&mut self, x_future: XFuture) {
        self.x_future.insert(x_future.id, x_future);
    }

    /// Exhume (get) [`XFuture`] from the store.
    ///
    pub fn exhume_x_future(&self, id: &Uuid) -> Option<&XFuture> {
        self.x_future.get(id)
    }

    /// Exorcise (remove) [`XFuture`] from the store.
    ///
    pub fn exorcise_x_future(&mut self, id: &Uuid) -> Option<XFuture> {
        self.x_future.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XFuture>`.
    ///
    pub fn iter_x_future(&self) -> impl Iterator<Item = &XFuture> {
        self.x_future.values()
    }

    /// Inter (insert) [`Grouped`] into the store.
    ///
    pub fn inter_grouped(&mut self, grouped: Grouped) {
        self.grouped.insert(grouped.id, grouped);
    }

    /// Exhume (get) [`Grouped`] from the store.
    ///
    pub fn exhume_grouped(&self, id: &Uuid) -> Option<&Grouped> {
        self.grouped.get(id)
    }

    /// Exorcise (remove) [`Grouped`] from the store.
    ///
    pub fn exorcise_grouped(&mut self, id: &Uuid) -> Option<Grouped> {
        self.grouped.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Grouped>`.
    ///
    pub fn iter_grouped(&self) -> impl Iterator<Item = &Grouped> {
        self.grouped.values()
    }

    /// Inter (insert) [`XIf`] into the store.
    ///
    pub fn inter_x_if(&mut self, x_if: XIf) {
        self.x_if.insert(x_if.id, x_if);
    }

    /// Exhume (get) [`XIf`] from the store.
    ///
    pub fn exhume_x_if(&self, id: &Uuid) -> Option<&XIf> {
        self.x_if.get(id)
    }

    /// Exorcise (remove) [`XIf`] from the store.
    ///
    pub fn exorcise_x_if(&mut self, id: &Uuid) -> Option<XIf> {
        self.x_if.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XIf>`.
    ///
    pub fn iter_x_if(&self) -> impl Iterator<Item = &XIf> {
        self.x_if.values()
    }

    /// Inter (insert) [`ImplementationBlock`] into the store.
    ///
    pub fn inter_implementation_block(&mut self, implementation_block: ImplementationBlock) {
        self.implementation_block
            .insert(implementation_block.id, implementation_block);
    }

    /// Exhume (get) [`ImplementationBlock`] from the store.
    ///
    pub fn exhume_implementation_block(&self, id: &Uuid) -> Option<&ImplementationBlock> {
        self.implementation_block.get(id)
    }

    /// Exorcise (remove) [`ImplementationBlock`] from the store.
    ///
    pub fn exorcise_implementation_block(&mut self, id: &Uuid) -> Option<ImplementationBlock> {
        self.implementation_block.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ImplementationBlock>`.
    ///
    pub fn iter_implementation_block(&self) -> impl Iterator<Item = &ImplementationBlock> {
        self.implementation_block.values()
    }

    /// Inter (insert) [`Import`] into the store.
    ///
    pub fn inter_import(&mut self, import: Import) {
        self.import.insert(import.id, import);
    }

    /// Exhume (get) [`Import`] from the store.
    ///
    pub fn exhume_import(&self, id: &Uuid) -> Option<&Import> {
        self.import.get(id)
    }

    /// Exorcise (remove) [`Import`] from the store.
    ///
    pub fn exorcise_import(&mut self, id: &Uuid) -> Option<Import> {
        self.import.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Import>`.
    ///
    pub fn iter_import(&self) -> impl Iterator<Item = &Import> {
        self.import.values()
    }

    /// Inter (insert) [`Index`] into the store.
    ///
    pub fn inter_index(&mut self, index: Index) {
        self.index.insert(index.id, index);
    }

    /// Exhume (get) [`Index`] from the store.
    ///
    pub fn exhume_index(&self, id: &Uuid) -> Option<&Index> {
        self.index.get(id)
    }

    /// Exorcise (remove) [`Index`] from the store.
    ///
    pub fn exorcise_index(&mut self, id: &Uuid) -> Option<Index> {
        self.index.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Index>`.
    ///
    pub fn iter_index(&self) -> impl Iterator<Item = &Index> {
        self.index.values()
    }

    /// Inter (insert) [`IntegerLiteral`] into the store.
    ///
    pub fn inter_integer_literal(&mut self, integer_literal: IntegerLiteral) {
        self.integer_literal
            .insert(integer_literal.id, integer_literal);
    }

    /// Exhume (get) [`IntegerLiteral`] from the store.
    ///
    pub fn exhume_integer_literal(&self, id: &Uuid) -> Option<&IntegerLiteral> {
        self.integer_literal.get(id)
    }

    /// Exorcise (remove) [`IntegerLiteral`] from the store.
    ///
    pub fn exorcise_integer_literal(&mut self, id: &Uuid) -> Option<IntegerLiteral> {
        self.integer_literal.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, IntegerLiteral>`.
    ///
    pub fn iter_integer_literal(&self) -> impl Iterator<Item = &IntegerLiteral> {
        self.integer_literal.values()
    }

    /// Inter (insert) [`Item`] into the store.
    ///
    pub fn inter_item(&mut self, item: Item) {
        self.item.insert(item.id, item);
    }

    /// Exhume (get) [`Item`] from the store.
    ///
    pub fn exhume_item(&self, id: &Uuid) -> Option<&Item> {
        self.item.get(id)
    }

    /// Exorcise (remove) [`Item`] from the store.
    ///
    pub fn exorcise_item(&mut self, id: &Uuid) -> Option<Item> {
        self.item.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Item>`.
    ///
    pub fn iter_item(&self) -> impl Iterator<Item = &Item> {
        self.item.values()
    }

    /// Inter (insert) [`Lambda`] into the store.
    ///
    pub fn inter_lambda(&mut self, lambda: Lambda) {
        self.lambda.insert(lambda.id, lambda);
    }

    /// Exhume (get) [`Lambda`] from the store.
    ///
    pub fn exhume_lambda(&self, id: &Uuid) -> Option<&Lambda> {
        self.lambda.get(id)
    }

    /// Exorcise (remove) [`Lambda`] from the store.
    ///
    pub fn exorcise_lambda(&mut self, id: &Uuid) -> Option<Lambda> {
        self.lambda.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Lambda>`.
    ///
    pub fn iter_lambda(&self) -> impl Iterator<Item = &Lambda> {
        self.lambda.values()
    }

    /// Inter (insert) [`LambdaParameter`] into the store.
    ///
    pub fn inter_lambda_parameter(&mut self, lambda_parameter: LambdaParameter) {
        self.lambda_parameter
            .insert(lambda_parameter.id, lambda_parameter);
    }

    /// Exhume (get) [`LambdaParameter`] from the store.
    ///
    pub fn exhume_lambda_parameter(&self, id: &Uuid) -> Option<&LambdaParameter> {
        self.lambda_parameter.get(id)
    }

    /// Exorcise (remove) [`LambdaParameter`] from the store.
    ///
    pub fn exorcise_lambda_parameter(&mut self, id: &Uuid) -> Option<LambdaParameter> {
        self.lambda_parameter.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LambdaParameter>`.
    ///
    pub fn iter_lambda_parameter(&self) -> impl Iterator<Item = &LambdaParameter> {
        self.lambda_parameter.values()
    }

    /// Inter (insert) [`LetStatement`] into the store.
    ///
    pub fn inter_let_statement(&mut self, let_statement: LetStatement) {
        self.let_statement.insert(let_statement.id, let_statement);
    }

    /// Exhume (get) [`LetStatement`] from the store.
    ///
    pub fn exhume_let_statement(&self, id: &Uuid) -> Option<&LetStatement> {
        self.let_statement.get(id)
    }

    /// Exorcise (remove) [`LetStatement`] from the store.
    ///
    pub fn exorcise_let_statement(&mut self, id: &Uuid) -> Option<LetStatement> {
        self.let_statement.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LetStatement>`.
    ///
    pub fn iter_let_statement(&self) -> impl Iterator<Item = &LetStatement> {
        self.let_statement.values()
    }

    /// Inter (insert) [`List`] into the store.
    ///
    pub fn inter_list(&mut self, list: List) {
        self.list.insert(list.id, list);
    }

    /// Exhume (get) [`List`] from the store.
    ///
    pub fn exhume_list(&self, id: &Uuid) -> Option<&List> {
        self.list.get(id)
    }

    /// Exorcise (remove) [`List`] from the store.
    ///
    pub fn exorcise_list(&mut self, id: &Uuid) -> Option<List> {
        self.list.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, List>`.
    ///
    pub fn iter_list(&self) -> impl Iterator<Item = &List> {
        self.list.values()
    }

    /// Inter (insert) [`ListElement`] into the store.
    ///
    pub fn inter_list_element(&mut self, list_element: ListElement) {
        self.list_element.insert(list_element.id, list_element);
    }

    /// Exhume (get) [`ListElement`] from the store.
    ///
    pub fn exhume_list_element(&self, id: &Uuid) -> Option<&ListElement> {
        self.list_element.get(id)
    }

    /// Exorcise (remove) [`ListElement`] from the store.
    ///
    pub fn exorcise_list_element(&mut self, id: &Uuid) -> Option<ListElement> {
        self.list_element.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListElement>`.
    ///
    pub fn iter_list_element(&self) -> impl Iterator<Item = &ListElement> {
        self.list_element.values()
    }

    /// Inter (insert) [`ListExpression`] into the store.
    ///
    pub fn inter_list_expression(&mut self, list_expression: ListExpression) {
        self.list_expression
            .insert(list_expression.id, list_expression);
    }

    /// Exhume (get) [`ListExpression`] from the store.
    ///
    pub fn exhume_list_expression(&self, id: &Uuid) -> Option<&ListExpression> {
        self.list_expression.get(id)
    }

    /// Exorcise (remove) [`ListExpression`] from the store.
    ///
    pub fn exorcise_list_expression(&mut self, id: &Uuid) -> Option<ListExpression> {
        self.list_expression.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListExpression>`.
    ///
    pub fn iter_list_expression(&self) -> impl Iterator<Item = &ListExpression> {
        self.list_expression.values()
    }

    /// Inter (insert) [`Literal`] into the store.
    ///
    pub fn inter_literal(&mut self, literal: Literal) {
        self.literal.insert(literal.id, literal);
    }

    /// Exhume (get) [`Literal`] from the store.
    ///
    pub fn exhume_literal(&self, id: &Uuid) -> Option<&Literal> {
        self.literal.get(id)
    }

    /// Exorcise (remove) [`Literal`] from the store.
    ///
    pub fn exorcise_literal(&mut self, id: &Uuid) -> Option<Literal> {
        self.literal.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Literal>`.
    ///
    pub fn iter_literal(&self) -> impl Iterator<Item = &Literal> {
        self.literal.values()
    }

    /// Inter (insert) [`LocalVariable`] into the store.
    ///
    pub fn inter_local_variable(&mut self, local_variable: LocalVariable) {
        self.local_variable
            .insert(local_variable.id, local_variable);
    }

    /// Exhume (get) [`LocalVariable`] from the store.
    ///
    pub fn exhume_local_variable(&self, id: &Uuid) -> Option<&LocalVariable> {
        self.local_variable.get(id)
    }

    /// Exorcise (remove) [`LocalVariable`] from the store.
    ///
    pub fn exorcise_local_variable(&mut self, id: &Uuid) -> Option<LocalVariable> {
        self.local_variable.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LocalVariable>`.
    ///
    pub fn iter_local_variable(&self) -> impl Iterator<Item = &LocalVariable> {
        self.local_variable.values()
    }

    /// Inter (insert) [`XMacro`] into the store.
    ///
    pub fn inter_x_macro(&mut self, x_macro: XMacro) {
        self.x_macro.insert(x_macro.id, x_macro);
    }

    /// Exhume (get) [`XMacro`] from the store.
    ///
    pub fn exhume_x_macro(&self, id: &Uuid) -> Option<&XMacro> {
        self.x_macro.get(id)
    }

    /// Exorcise (remove) [`XMacro`] from the store.
    ///
    pub fn exorcise_x_macro(&mut self, id: &Uuid) -> Option<XMacro> {
        self.x_macro.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XMacro>`.
    ///
    pub fn iter_x_macro(&self) -> impl Iterator<Item = &XMacro> {
        self.x_macro.values()
    }

    /// Inter (insert) [`XMatch`] into the store.
    ///
    pub fn inter_x_match(&mut self, x_match: XMatch) {
        self.x_match.insert(x_match.id, x_match);
    }

    /// Exhume (get) [`XMatch`] from the store.
    ///
    pub fn exhume_x_match(&self, id: &Uuid) -> Option<&XMatch> {
        self.x_match.get(id)
    }

    /// Exorcise (remove) [`XMatch`] from the store.
    ///
    pub fn exorcise_x_match(&mut self, id: &Uuid) -> Option<XMatch> {
        self.x_match.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XMatch>`.
    ///
    pub fn iter_x_match(&self) -> impl Iterator<Item = &XMatch> {
        self.x_match.values()
    }

    /// Inter (insert) [`MethodCall`] into the store.
    ///
    pub fn inter_method_call(&mut self, method_call: MethodCall) {
        self.method_call.insert(method_call.id, method_call);
    }

    /// Exhume (get) [`MethodCall`] from the store.
    ///
    pub fn exhume_method_call(&self, id: &Uuid) -> Option<&MethodCall> {
        self.method_call.get(id)
    }

    /// Exorcise (remove) [`MethodCall`] from the store.
    ///
    pub fn exorcise_method_call(&mut self, id: &Uuid) -> Option<MethodCall> {
        self.method_call.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, MethodCall>`.
    ///
    pub fn iter_method_call(&self) -> impl Iterator<Item = &MethodCall> {
        self.method_call.values()
    }

    /// Inter (insert) [`NamedFieldExpression`] into the store.
    ///
    pub fn inter_named_field_expression(&mut self, named_field_expression: NamedFieldExpression) {
        self.named_field_expression
            .insert(named_field_expression.id, named_field_expression);
    }

    /// Exhume (get) [`NamedFieldExpression`] from the store.
    ///
    pub fn exhume_named_field_expression(&self, id: &Uuid) -> Option<&NamedFieldExpression> {
        self.named_field_expression.get(id)
    }

    /// Exorcise (remove) [`NamedFieldExpression`] from the store.
    ///
    pub fn exorcise_named_field_expression(&mut self, id: &Uuid) -> Option<NamedFieldExpression> {
        self.named_field_expression.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, NamedFieldExpression>`.
    ///
    pub fn iter_named_field_expression(&self) -> impl Iterator<Item = &NamedFieldExpression> {
        self.named_field_expression.values()
    }

    /// Inter (insert) [`ZObjectStore`] into the store.
    ///
    pub fn inter_z_object_store(&mut self, z_object_store: ZObjectStore) {
        self.z_object_store_id_by_name
            .insert(z_object_store.name.to_upper_camel_case(), z_object_store.id);
        self.z_object_store
            .insert(z_object_store.id, z_object_store);
    }

    /// Exhume (get) [`ZObjectStore`] from the store.
    ///
    pub fn exhume_z_object_store(&self, id: &Uuid) -> Option<&ZObjectStore> {
        self.z_object_store.get(id)
    }

    /// Exorcise (remove) [`ZObjectStore`] from the store.
    ///
    pub fn exorcise_z_object_store(&mut self, id: &Uuid) -> Option<ZObjectStore> {
        self.z_object_store.remove(id)
    }

    /// Exhume [`ZObjectStore`] id from the store by name.
    ///
    pub fn exhume_z_object_store_id_by_name(&self, name: &str) -> Option<&Uuid> {
        self.z_object_store_id_by_name.get(name)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ZObjectStore>`.
    ///
    pub fn iter_z_object_store(&self) -> impl Iterator<Item = &ZObjectStore> {
        self.z_object_store.values()
    }

    /// Inter (insert) [`ObjectWrapper`] into the store.
    ///
    pub fn inter_object_wrapper(&mut self, object_wrapper: ObjectWrapper) {
        self.object_wrapper
            .insert(object_wrapper.id, object_wrapper);
    }

    /// Exhume (get) [`ObjectWrapper`] from the store.
    ///
    pub fn exhume_object_wrapper(&self, id: &Uuid) -> Option<&ObjectWrapper> {
        self.object_wrapper.get(id)
    }

    /// Exorcise (remove) [`ObjectWrapper`] from the store.
    ///
    pub fn exorcise_object_wrapper(&mut self, id: &Uuid) -> Option<ObjectWrapper> {
        self.object_wrapper.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ObjectWrapper>`.
    ///
    pub fn iter_object_wrapper(&self) -> impl Iterator<Item = &ObjectWrapper> {
        self.object_wrapper.values()
    }

    /// Inter (insert) [`Operator`] into the store.
    ///
    pub fn inter_operator(&mut self, operator: Operator) {
        self.operator.insert(operator.id, operator);
    }

    /// Exhume (get) [`Operator`] from the store.
    ///
    pub fn exhume_operator(&self, id: &Uuid) -> Option<&Operator> {
        self.operator.get(id)
    }

    /// Exorcise (remove) [`Operator`] from the store.
    ///
    pub fn exorcise_operator(&mut self, id: &Uuid) -> Option<Operator> {
        self.operator.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Operator>`.
    ///
    pub fn iter_operator(&self) -> impl Iterator<Item = &Operator> {
        self.operator.values()
    }

    /// Inter (insert) [`Parameter`] into the store.
    ///
    pub fn inter_parameter(&mut self, parameter: Parameter) {
        self.parameter.insert(parameter.id, parameter);
    }

    /// Exhume (get) [`Parameter`] from the store.
    ///
    pub fn exhume_parameter(&self, id: &Uuid) -> Option<&Parameter> {
        self.parameter.get(id)
    }

    /// Exorcise (remove) [`Parameter`] from the store.
    ///
    pub fn exorcise_parameter(&mut self, id: &Uuid) -> Option<Parameter> {
        self.parameter.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Parameter>`.
    ///
    pub fn iter_parameter(&self) -> impl Iterator<Item = &Parameter> {
        self.parameter.values()
    }

    /// Inter (insert) [`XPath`] into the store.
    ///
    pub fn inter_x_path(&mut self, x_path: XPath) {
        self.x_path.insert(x_path.id, x_path);
    }

    /// Exhume (get) [`XPath`] from the store.
    ///
    pub fn exhume_x_path(&self, id: &Uuid) -> Option<&XPath> {
        self.x_path.get(id)
    }

    /// Exorcise (remove) [`XPath`] from the store.
    ///
    pub fn exorcise_x_path(&mut self, id: &Uuid) -> Option<XPath> {
        self.x_path.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XPath>`.
    ///
    pub fn iter_x_path(&self) -> impl Iterator<Item = &XPath> {
        self.x_path.values()
    }

    /// Inter (insert) [`PathElement`] into the store.
    ///
    pub fn inter_path_element(&mut self, path_element: PathElement) {
        self.path_element.insert(path_element.id, path_element);
    }

    /// Exhume (get) [`PathElement`] from the store.
    ///
    pub fn exhume_path_element(&self, id: &Uuid) -> Option<&PathElement> {
        self.path_element.get(id)
    }

    /// Exorcise (remove) [`PathElement`] from the store.
    ///
    pub fn exorcise_path_element(&mut self, id: &Uuid) -> Option<PathElement> {
        self.path_element.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, PathElement>`.
    ///
    pub fn iter_path_element(&self) -> impl Iterator<Item = &PathElement> {
        self.path_element.values()
    }

    /// Inter (insert) [`Pattern`] into the store.
    ///
    pub fn inter_pattern(&mut self, pattern: Pattern) {
        self.pattern.insert(pattern.id, pattern);
    }

    /// Exhume (get) [`Pattern`] from the store.
    ///
    pub fn exhume_pattern(&self, id: &Uuid) -> Option<&Pattern> {
        self.pattern.get(id)
    }

    /// Exorcise (remove) [`Pattern`] from the store.
    ///
    pub fn exorcise_pattern(&mut self, id: &Uuid) -> Option<Pattern> {
        self.pattern.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Pattern>`.
    ///
    pub fn iter_pattern(&self) -> impl Iterator<Item = &Pattern> {
        self.pattern.values()
    }

    /// Inter (insert) [`XPlugin`] into the store.
    ///
    pub fn inter_x_plugin(&mut self, x_plugin: XPlugin) {
        self.x_plugin_id_by_name
            .insert(x_plugin.name.to_upper_camel_case(), x_plugin.id);
        self.x_plugin.insert(x_plugin.id, x_plugin);
    }

    /// Exhume (get) [`XPlugin`] from the store.
    ///
    pub fn exhume_x_plugin(&self, id: &Uuid) -> Option<&XPlugin> {
        self.x_plugin.get(id)
    }

    /// Exorcise (remove) [`XPlugin`] from the store.
    ///
    pub fn exorcise_x_plugin(&mut self, id: &Uuid) -> Option<XPlugin> {
        self.x_plugin.remove(id)
    }

    /// Exhume [`XPlugin`] id from the store by name.
    ///
    pub fn exhume_x_plugin_id_by_name(&self, name: &str) -> Option<&Uuid> {
        self.x_plugin_id_by_name.get(name)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XPlugin>`.
    ///
    pub fn iter_x_plugin(&self) -> impl Iterator<Item = &XPlugin> {
        self.x_plugin.values()
    }

    /// Inter (insert) [`XPrint`] into the store.
    ///
    pub fn inter_x_print(&mut self, x_print: XPrint) {
        self.x_print.insert(x_print.id, x_print);
    }

    /// Exhume (get) [`XPrint`] from the store.
    ///
    pub fn exhume_x_print(&self, id: &Uuid) -> Option<&XPrint> {
        self.x_print.get(id)
    }

    /// Exorcise (remove) [`XPrint`] from the store.
    ///
    pub fn exorcise_x_print(&mut self, id: &Uuid) -> Option<XPrint> {
        self.x_print.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XPrint>`.
    ///
    pub fn iter_x_print(&self) -> impl Iterator<Item = &XPrint> {
        self.x_print.values()
    }

    /// Inter (insert) [`RangeExpression`] into the store.
    ///
    pub fn inter_range_expression(&mut self, range_expression: RangeExpression) {
        self.range_expression
            .insert(range_expression.id, range_expression);
    }

    /// Exhume (get) [`RangeExpression`] from the store.
    ///
    pub fn exhume_range_expression(&self, id: &Uuid) -> Option<&RangeExpression> {
        self.range_expression.get(id)
    }

    /// Exorcise (remove) [`RangeExpression`] from the store.
    ///
    pub fn exorcise_range_expression(&mut self, id: &Uuid) -> Option<RangeExpression> {
        self.range_expression.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RangeExpression>`.
    ///
    pub fn iter_range_expression(&self) -> impl Iterator<Item = &RangeExpression> {
        self.range_expression.values()
    }

    /// Inter (insert) [`ResultStatement`] into the store.
    ///
    pub fn inter_result_statement(&mut self, result_statement: ResultStatement) {
        self.result_statement
            .insert(result_statement.id, result_statement);
    }

    /// Exhume (get) [`ResultStatement`] from the store.
    ///
    pub fn exhume_result_statement(&self, id: &Uuid) -> Option<&ResultStatement> {
        self.result_statement.get(id)
    }

    /// Exorcise (remove) [`ResultStatement`] from the store.
    ///
    pub fn exorcise_result_statement(&mut self, id: &Uuid) -> Option<ResultStatement> {
        self.result_statement.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ResultStatement>`.
    ///
    pub fn iter_result_statement(&self) -> impl Iterator<Item = &ResultStatement> {
        self.result_statement.values()
    }

    /// Inter (insert) [`XReturn`] into the store.
    ///
    pub fn inter_x_return(&mut self, x_return: XReturn) {
        self.x_return.insert(x_return.id, x_return);
    }

    /// Exhume (get) [`XReturn`] from the store.
    ///
    pub fn exhume_x_return(&self, id: &Uuid) -> Option<&XReturn> {
        self.x_return.get(id)
    }

    /// Exorcise (remove) [`XReturn`] from the store.
    ///
    pub fn exorcise_x_return(&mut self, id: &Uuid) -> Option<XReturn> {
        self.x_return.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XReturn>`.
    ///
    pub fn iter_x_return(&self) -> impl Iterator<Item = &XReturn> {
        self.x_return.values()
    }

    /// Inter (insert) [`Span`] into the store.
    ///
    pub fn inter_span(&mut self, span: Span) {
        self.span.insert(span.id, span);
    }

    /// Exhume (get) [`Span`] from the store.
    ///
    pub fn exhume_span(&self, id: &Uuid) -> Option<&Span> {
        self.span.get(id)
    }

    /// Exorcise (remove) [`Span`] from the store.
    ///
    pub fn exorcise_span(&mut self, id: &Uuid) -> Option<Span> {
        self.span.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Span>`.
    ///
    pub fn iter_span(&self) -> impl Iterator<Item = &Span> {
        self.span.values()
    }

    /// Inter (insert) [`Statement`] into the store.
    ///
    pub fn inter_statement(&mut self, statement: Statement) {
        self.statement.insert(statement.id, statement);
    }

    /// Exhume (get) [`Statement`] from the store.
    ///
    pub fn exhume_statement(&self, id: &Uuid) -> Option<&Statement> {
        self.statement.get(id)
    }

    /// Exorcise (remove) [`Statement`] from the store.
    ///
    pub fn exorcise_statement(&mut self, id: &Uuid) -> Option<Statement> {
        self.statement.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Statement>`.
    ///
    pub fn iter_statement(&self) -> impl Iterator<Item = &Statement> {
        self.statement.values()
    }

    /// Inter (insert) [`StaticMethodCall`] into the store.
    ///
    pub fn inter_static_method_call(&mut self, static_method_call: StaticMethodCall) {
        self.static_method_call
            .insert(static_method_call.id, static_method_call);
    }

    /// Exhume (get) [`StaticMethodCall`] from the store.
    ///
    pub fn exhume_static_method_call(&self, id: &Uuid) -> Option<&StaticMethodCall> {
        self.static_method_call.get(id)
    }

    /// Exorcise (remove) [`StaticMethodCall`] from the store.
    ///
    pub fn exorcise_static_method_call(&mut self, id: &Uuid) -> Option<StaticMethodCall> {
        self.static_method_call.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StaticMethodCall>`.
    ///
    pub fn iter_static_method_call(&self) -> impl Iterator<Item = &StaticMethodCall> {
        self.static_method_call.values()
    }

    /// Inter (insert) [`StringLiteral`] into the store.
    ///
    pub fn inter_string_literal(&mut self, string_literal: StringLiteral) {
        self.string_literal
            .insert(string_literal.id, string_literal);
    }

    /// Exhume (get) [`StringLiteral`] from the store.
    ///
    pub fn exhume_string_literal(&self, id: &Uuid) -> Option<&StringLiteral> {
        self.string_literal.get(id)
    }

    /// Exorcise (remove) [`StringLiteral`] from the store.
    ///
    pub fn exorcise_string_literal(&mut self, id: &Uuid) -> Option<StringLiteral> {
        self.string_literal.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StringLiteral>`.
    ///
    pub fn iter_string_literal(&self) -> impl Iterator<Item = &StringLiteral> {
        self.string_literal.values()
    }

    /// Inter (insert) [`WoogStruct`] into the store.
    ///
    pub fn inter_woog_struct(&mut self, woog_struct: WoogStruct) {
        self.woog_struct_id_by_name
            .insert(woog_struct.name.to_upper_camel_case(), woog_struct.id);
        self.woog_struct.insert(woog_struct.id, woog_struct);
    }

    /// Exhume (get) [`WoogStruct`] from the store.
    ///
    pub fn exhume_woog_struct(&self, id: &Uuid) -> Option<&WoogStruct> {
        self.woog_struct.get(id)
    }

    /// Exorcise (remove) [`WoogStruct`] from the store.
    ///
    pub fn exorcise_woog_struct(&mut self, id: &Uuid) -> Option<WoogStruct> {
        self.woog_struct.remove(id)
    }

    /// Exhume [`WoogStruct`] id from the store by name.
    ///
    pub fn exhume_woog_struct_id_by_name(&self, name: &str) -> Option<&Uuid> {
        self.woog_struct_id_by_name.get(name)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, WoogStruct>`.
    ///
    pub fn iter_woog_struct(&self) -> impl Iterator<Item = &WoogStruct> {
        self.woog_struct.values()
    }

    /// Inter (insert) [`StructExpression`] into the store.
    ///
    pub fn inter_struct_expression(&mut self, struct_expression: StructExpression) {
        self.struct_expression
            .insert(struct_expression.id, struct_expression);
    }

    /// Exhume (get) [`StructExpression`] from the store.
    ///
    pub fn exhume_struct_expression(&self, id: &Uuid) -> Option<&StructExpression> {
        self.struct_expression.get(id)
    }

    /// Exorcise (remove) [`StructExpression`] from the store.
    ///
    pub fn exorcise_struct_expression(&mut self, id: &Uuid) -> Option<StructExpression> {
        self.struct_expression.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructExpression>`.
    ///
    pub fn iter_struct_expression(&self) -> impl Iterator<Item = &StructExpression> {
        self.struct_expression.values()
    }

    /// Inter (insert) [`StructField`] into the store.
    ///
    pub fn inter_struct_field(&mut self, struct_field: StructField) {
        self.struct_field.insert(struct_field.id, struct_field);
    }

    /// Exhume (get) [`StructField`] from the store.
    ///
    pub fn exhume_struct_field(&self, id: &Uuid) -> Option<&StructField> {
        self.struct_field.get(id)
    }

    /// Exorcise (remove) [`StructField`] from the store.
    ///
    pub fn exorcise_struct_field(&mut self, id: &Uuid) -> Option<StructField> {
        self.struct_field.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructField>`.
    ///
    pub fn iter_struct_field(&self) -> impl Iterator<Item = &StructField> {
        self.struct_field.values()
    }

    /// Inter (insert) [`StructGeneric`] into the store.
    ///
    pub fn inter_struct_generic(&mut self, struct_generic: StructGeneric) {
        self.struct_generic
            .insert(struct_generic.id, struct_generic);
    }

    /// Exhume (get) [`StructGeneric`] from the store.
    ///
    pub fn exhume_struct_generic(&self, id: &Uuid) -> Option<&StructGeneric> {
        self.struct_generic.get(id)
    }

    /// Exorcise (remove) [`StructGeneric`] from the store.
    ///
    pub fn exorcise_struct_generic(&mut self, id: &Uuid) -> Option<StructGeneric> {
        self.struct_generic.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructGeneric>`.
    ///
    pub fn iter_struct_generic(&self) -> impl Iterator<Item = &StructGeneric> {
        self.struct_generic.values()
    }

    /// Inter (insert) [`TupleField`] into the store.
    ///
    pub fn inter_tuple_field(&mut self, tuple_field: TupleField) {
        self.tuple_field.insert(tuple_field.id, tuple_field);
    }

    /// Exhume (get) [`TupleField`] from the store.
    ///
    pub fn exhume_tuple_field(&self, id: &Uuid) -> Option<&TupleField> {
        self.tuple_field.get(id)
    }

    /// Exorcise (remove) [`TupleField`] from the store.
    ///
    pub fn exorcise_tuple_field(&mut self, id: &Uuid) -> Option<TupleField> {
        self.tuple_field.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, TupleField>`.
    ///
    pub fn iter_tuple_field(&self) -> impl Iterator<Item = &TupleField> {
        self.tuple_field.values()
    }

    /// Inter (insert) [`TypeCast`] into the store.
    ///
    pub fn inter_type_cast(&mut self, type_cast: TypeCast) {
        self.type_cast.insert(type_cast.id, type_cast);
    }

    /// Exhume (get) [`TypeCast`] from the store.
    ///
    pub fn exhume_type_cast(&self, id: &Uuid) -> Option<&TypeCast> {
        self.type_cast.get(id)
    }

    /// Exorcise (remove) [`TypeCast`] from the store.
    ///
    pub fn exorcise_type_cast(&mut self, id: &Uuid) -> Option<TypeCast> {
        self.type_cast.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, TypeCast>`.
    ///
    pub fn iter_type_cast(&self) -> impl Iterator<Item = &TypeCast> {
        self.type_cast.values()
    }

    /// Inter (insert) [`Unary`] into the store.
    ///
    pub fn inter_unary(&mut self, unary: Unary) {
        self.unary.insert(unary.id, unary);
    }

    /// Exhume (get) [`Unary`] from the store.
    ///
    pub fn exhume_unary(&self, id: &Uuid) -> Option<&Unary> {
        self.unary.get(id)
    }

    /// Exorcise (remove) [`Unary`] from the store.
    ///
    pub fn exorcise_unary(&mut self, id: &Uuid) -> Option<Unary> {
        self.unary.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Unary>`.
    ///
    pub fn iter_unary(&self) -> impl Iterator<Item = &Unary> {
        self.unary.values()
    }

    /// Inter (insert) [`Unit`] into the store.
    ///
    pub fn inter_unit(&mut self, unit: Unit) {
        self.unit.insert(unit.id, unit);
    }

    /// Exhume (get) [`Unit`] from the store.
    ///
    pub fn exhume_unit(&self, id: &Uuid) -> Option<&Unit> {
        self.unit.get(id)
    }

    /// Exorcise (remove) [`Unit`] from the store.
    ///
    pub fn exorcise_unit(&mut self, id: &Uuid) -> Option<Unit> {
        self.unit.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Unit>`.
    ///
    pub fn iter_unit(&self) -> impl Iterator<Item = &Unit> {
        self.unit.values()
    }

    /// Inter (insert) [`UnnamedFieldExpression`] into the store.
    ///
    pub fn inter_unnamed_field_expression(
        &mut self,
        unnamed_field_expression: UnnamedFieldExpression,
    ) {
        self.unnamed_field_expression
            .insert(unnamed_field_expression.id, unnamed_field_expression);
    }

    /// Exhume (get) [`UnnamedFieldExpression`] from the store.
    ///
    pub fn exhume_unnamed_field_expression(&self, id: &Uuid) -> Option<&UnnamedFieldExpression> {
        self.unnamed_field_expression.get(id)
    }

    /// Exorcise (remove) [`UnnamedFieldExpression`] from the store.
    ///
    pub fn exorcise_unnamed_field_expression(
        &mut self,
        id: &Uuid,
    ) -> Option<UnnamedFieldExpression> {
        self.unnamed_field_expression.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, UnnamedFieldExpression>`.
    ///
    pub fn iter_unnamed_field_expression(&self) -> impl Iterator<Item = &UnnamedFieldExpression> {
        self.unnamed_field_expression.values()
    }

    /// Inter (insert) [`XValue`] into the store.
    ///
    pub fn inter_x_value(&mut self, x_value: XValue) {
        self.x_value.insert(x_value.id, x_value);
    }

    /// Exhume (get) [`XValue`] from the store.
    ///
    pub fn exhume_x_value(&self, id: &Uuid) -> Option<&XValue> {
        self.x_value.get(id)
    }

    /// Exorcise (remove) [`XValue`] from the store.
    ///
    pub fn exorcise_x_value(&mut self, id: &Uuid) -> Option<XValue> {
        self.x_value.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XValue>`.
    ///
    pub fn iter_x_value(&self) -> impl Iterator<Item = &XValue> {
        self.x_value.values()
    }

    /// Inter (insert) [`ValueType`] into the store.
    ///
    pub fn inter_value_type(&mut self, value_type: ValueType) {
        self.value_type.insert(value_type.id, value_type);
    }

    /// Exhume (get) [`ValueType`] from the store.
    ///
    pub fn exhume_value_type(&self, id: &Uuid) -> Option<&ValueType> {
        self.value_type.get(id)
    }

    /// Exorcise (remove) [`ValueType`] from the store.
    ///
    pub fn exorcise_value_type(&mut self, id: &Uuid) -> Option<ValueType> {
        self.value_type.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ValueType>`.
    ///
    pub fn iter_value_type(&self) -> impl Iterator<Item = &ValueType> {
        self.value_type.values()
    }

    /// Inter (insert) [`Variable`] into the store.
    ///
    pub fn inter_variable(&mut self, variable: Variable) {
        self.variable.insert(variable.id, variable);
    }

    /// Exhume (get) [`Variable`] from the store.
    ///
    pub fn exhume_variable(&self, id: &Uuid) -> Option<&Variable> {
        self.variable.get(id)
    }

    /// Exorcise (remove) [`Variable`] from the store.
    ///
    pub fn exorcise_variable(&mut self, id: &Uuid) -> Option<Variable> {
        self.variable.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Variable>`.
    ///
    pub fn iter_variable(&self) -> impl Iterator<Item = &Variable> {
        self.variable.values()
    }

    /// Inter (insert) [`VariableExpression`] into the store.
    ///
    pub fn inter_variable_expression(&mut self, variable_expression: VariableExpression) {
        self.variable_expression
            .insert(variable_expression.id, variable_expression);
    }

    /// Exhume (get) [`VariableExpression`] from the store.
    ///
    pub fn exhume_variable_expression(&self, id: &Uuid) -> Option<&VariableExpression> {
        self.variable_expression.get(id)
    }

    /// Exorcise (remove) [`VariableExpression`] from the store.
    ///
    pub fn exorcise_variable_expression(&mut self, id: &Uuid) -> Option<VariableExpression> {
        self.variable_expression.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, VariableExpression>`.
    ///
    pub fn iter_variable_expression(&self) -> impl Iterator<Item = &VariableExpression> {
        self.variable_expression.values()
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
            for argument in self.argument.values() {
                let path = path.join(format!("{}.json", argument.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &argument)?;
            }
        }

        // Persist Await.
        {
            let path = path.join("a_wait");
            fs::create_dir_all(&path)?;
            for a_wait in self.a_wait.values() {
                let path = path.join(format!("{}.json", a_wait.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &a_wait)?;
            }
        }

        // Persist Binary.
        {
            let path = path.join("binary");
            fs::create_dir_all(&path)?;
            for binary in self.binary.values() {
                let path = path.join(format!("{}.json", binary.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &binary)?;
            }
        }

        // Persist Block.
        {
            let path = path.join("block");
            fs::create_dir_all(&path)?;
            for block in self.block.values() {
                let path = path.join(format!("{}.json", block.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &block)?;
            }
        }

        // Persist Body.
        {
            let path = path.join("body");
            fs::create_dir_all(&path)?;
            for body in self.body.values() {
                let path = path.join(format!("{}.json", body.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &body)?;
            }
        }

        // Persist Boolean Literal.
        {
            let path = path.join("boolean_literal");
            fs::create_dir_all(&path)?;
            for boolean_literal in self.boolean_literal.values() {
                let path = path.join(format!("{}.json", boolean_literal.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &boolean_literal)?;
            }
        }

        // Persist Boolean Operator.
        {
            let path = path.join("boolean_operator");
            fs::create_dir_all(&path)?;
            for boolean_operator in self.boolean_operator.values() {
                let path = path.join(format!("{}.json", boolean_operator.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &boolean_operator)?;
            }
        }

        // Persist Call.
        {
            let path = path.join("call");
            fs::create_dir_all(&path)?;
            for call in self.call.values() {
                let path = path.join(format!("{}.json", call.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &call)?;
            }
        }

        // Persist Comparison.
        {
            let path = path.join("comparison");
            fs::create_dir_all(&path)?;
            for comparison in self.comparison.values() {
                let path = path.join(format!("{}.json", comparison.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &comparison)?;
            }
        }

        // Persist Data Structure.
        {
            let path = path.join("data_structure");
            fs::create_dir_all(&path)?;
            for data_structure in self.data_structure.values() {
                let path = path.join(format!("{}.json", data_structure.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &data_structure)?;
            }
        }

        // Persist Dwarf Source File.
        {
            let path = path.join("dwarf_source_file");
            fs::create_dir_all(&path)?;
            for dwarf_source_file in self.dwarf_source_file.values() {
                let path = path.join(format!("{}.json", dwarf_source_file.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &dwarf_source_file)?;
            }
        }

        // Persist Enum Field.
        {
            let path = path.join("enum_field");
            fs::create_dir_all(&path)?;
            for enum_field in self.enum_field.values() {
                let path = path.join(format!("{}.json", enum_field.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &enum_field)?;
            }
        }

        // Persist Enum Generic.
        {
            let path = path.join("enum_generic");
            fs::create_dir_all(&path)?;
            for enum_generic in self.enum_generic.values() {
                let path = path.join(format!("{}.json", enum_generic.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &enum_generic)?;
            }
        }

        // Persist Enumeration.
        {
            let path = path.join("enumeration");
            fs::create_dir_all(&path)?;
            for enumeration in self.enumeration.values() {
                let path = path.join(format!("{}.json", enumeration.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &enumeration)?;
            }
        }

        // Persist Expression.
        {
            let path = path.join("expression");
            fs::create_dir_all(&path)?;
            for expression in self.expression.values() {
                let path = path.join(format!("{}.json", expression.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &expression)?;
            }
        }

        // Persist Expression Statement.
        {
            let path = path.join("expression_statement");
            fs::create_dir_all(&path)?;
            for expression_statement in self.expression_statement.values() {
                let path = path.join(format!("{}.json", expression_statement.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &expression_statement)?;
            }
        }

        // Persist External Implementation.
        {
            let path = path.join("external_implementation");
            fs::create_dir_all(&path)?;
            for external_implementation in self.external_implementation.values() {
                let path = path.join(format!("{}.json", external_implementation.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &external_implementation)?;
            }
        }

        // Persist Field.
        {
            let path = path.join("field");
            fs::create_dir_all(&path)?;
            for field in self.field.values() {
                let path = path.join(format!("{}.json", field.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &field)?;
            }
        }

        // Persist Field Access.
        {
            let path = path.join("field_access");
            fs::create_dir_all(&path)?;
            for field_access in self.field_access.values() {
                let path = path.join(format!("{}.json", field_access.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &field_access)?;
            }
        }

        // Persist Field Access Target.
        {
            let path = path.join("field_access_target");
            fs::create_dir_all(&path)?;
            for field_access_target in self.field_access_target.values() {
                let path = path.join(format!("{}.json", field_access_target.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &field_access_target)?;
            }
        }

        // Persist Field Expression.
        {
            let path = path.join("field_expression");
            fs::create_dir_all(&path)?;
            for field_expression in self.field_expression.values() {
                let path = path.join(format!("{}.json", field_expression.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &field_expression)?;
            }
        }

        // Persist Float Literal.
        {
            let path = path.join("float_literal");
            fs::create_dir_all(&path)?;
            for float_literal in self.float_literal.values() {
                let path = path.join(format!("{}.json", float_literal.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &float_literal)?;
            }
        }

        // Persist For Loop.
        {
            let path = path.join("for_loop");
            fs::create_dir_all(&path)?;
            for for_loop in self.for_loop.values() {
                let path = path.join(format!("{}.json", for_loop.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &for_loop)?;
            }
        }

        // Persist Func Generic.
        {
            let path = path.join("func_generic");
            fs::create_dir_all(&path)?;
            for func_generic in self.func_generic.values() {
                let path = path.join(format!("{}.json", func_generic.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &func_generic)?;
            }
        }

        // Persist Function.
        {
            let path = path.join("function");
            fs::create_dir_all(&path)?;
            for function in self.function.values() {
                let path = path.join(format!("{}.json", function.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &function)?;
            }
        }

        // Persist Function Call.
        {
            let path = path.join("function_call");
            fs::create_dir_all(&path)?;
            for function_call in self.function_call.values() {
                let path = path.join(format!("{}.json", function_call.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &function_call)?;
            }
        }

        // Persist Future.
        {
            let path = path.join("x_future");
            fs::create_dir_all(&path)?;
            for x_future in self.x_future.values() {
                let path = path.join(format!("{}.json", x_future.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_future)?;
            }
        }

        // Persist Grouped.
        {
            let path = path.join("grouped");
            fs::create_dir_all(&path)?;
            for grouped in self.grouped.values() {
                let path = path.join(format!("{}.json", grouped.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &grouped)?;
            }
        }

        // Persist If.
        {
            let path = path.join("x_if");
            fs::create_dir_all(&path)?;
            for x_if in self.x_if.values() {
                let path = path.join(format!("{}.json", x_if.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_if)?;
            }
        }

        // Persist Implementation Block.
        {
            let path = path.join("implementation_block");
            fs::create_dir_all(&path)?;
            for implementation_block in self.implementation_block.values() {
                let path = path.join(format!("{}.json", implementation_block.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &implementation_block)?;
            }
        }

        // Persist Import.
        {
            let path = path.join("import");
            fs::create_dir_all(&path)?;
            for import in self.import.values() {
                let path = path.join(format!("{}.json", import.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &import)?;
            }
        }

        // Persist Index.
        {
            let path = path.join("index");
            fs::create_dir_all(&path)?;
            for index in self.index.values() {
                let path = path.join(format!("{}.json", index.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &index)?;
            }
        }

        // Persist Integer Literal.
        {
            let path = path.join("integer_literal");
            fs::create_dir_all(&path)?;
            for integer_literal in self.integer_literal.values() {
                let path = path.join(format!("{}.json", integer_literal.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &integer_literal)?;
            }
        }

        // Persist Item.
        {
            let path = path.join("item");
            fs::create_dir_all(&path)?;
            for item in self.item.values() {
                let path = path.join(format!("{}.json", item.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &item)?;
            }
        }

        // Persist Lambda.
        {
            let path = path.join("lambda");
            fs::create_dir_all(&path)?;
            for lambda in self.lambda.values() {
                let path = path.join(format!("{}.json", lambda.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &lambda)?;
            }
        }

        // Persist Lambda Parameter.
        {
            let path = path.join("lambda_parameter");
            fs::create_dir_all(&path)?;
            for lambda_parameter in self.lambda_parameter.values() {
                let path = path.join(format!("{}.json", lambda_parameter.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &lambda_parameter)?;
            }
        }

        // Persist Let Statement.
        {
            let path = path.join("let_statement");
            fs::create_dir_all(&path)?;
            for let_statement in self.let_statement.values() {
                let path = path.join(format!("{}.json", let_statement.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &let_statement)?;
            }
        }

        // Persist List.
        {
            let path = path.join("list");
            fs::create_dir_all(&path)?;
            for list in self.list.values() {
                let path = path.join(format!("{}.json", list.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &list)?;
            }
        }

        // Persist List Element.
        {
            let path = path.join("list_element");
            fs::create_dir_all(&path)?;
            for list_element in self.list_element.values() {
                let path = path.join(format!("{}.json", list_element.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &list_element)?;
            }
        }

        // Persist List Expression.
        {
            let path = path.join("list_expression");
            fs::create_dir_all(&path)?;
            for list_expression in self.list_expression.values() {
                let path = path.join(format!("{}.json", list_expression.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &list_expression)?;
            }
        }

        // Persist Literal.
        {
            let path = path.join("literal");
            fs::create_dir_all(&path)?;
            for literal in self.literal.values() {
                let path = path.join(format!("{}.json", literal.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &literal)?;
            }
        }

        // Persist Local Variable.
        {
            let path = path.join("local_variable");
            fs::create_dir_all(&path)?;
            for local_variable in self.local_variable.values() {
                let path = path.join(format!("{}.json", local_variable.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &local_variable)?;
            }
        }

        // Persist Macro.
        {
            let path = path.join("x_macro");
            fs::create_dir_all(&path)?;
            for x_macro in self.x_macro.values() {
                let path = path.join(format!("{}.json", x_macro.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_macro)?;
            }
        }

        // Persist Match.
        {
            let path = path.join("x_match");
            fs::create_dir_all(&path)?;
            for x_match in self.x_match.values() {
                let path = path.join(format!("{}.json", x_match.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_match)?;
            }
        }

        // Persist Method Call.
        {
            let path = path.join("method_call");
            fs::create_dir_all(&path)?;
            for method_call in self.method_call.values() {
                let path = path.join(format!("{}.json", method_call.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &method_call)?;
            }
        }

        // Persist Named Field Expression.
        {
            let path = path.join("named_field_expression");
            fs::create_dir_all(&path)?;
            for named_field_expression in self.named_field_expression.values() {
                let path = path.join(format!("{}.json", named_field_expression.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &named_field_expression)?;
            }
        }

        // Persist Object Store.
        {
            let path = path.join("z_object_store");
            fs::create_dir_all(&path)?;
            for z_object_store in self.z_object_store.values() {
                let path = path.join(format!("{}.json", z_object_store.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &z_object_store)?;
            }
        }

        // Persist Object Wrapper.
        {
            let path = path.join("object_wrapper");
            fs::create_dir_all(&path)?;
            for object_wrapper in self.object_wrapper.values() {
                let path = path.join(format!("{}.json", object_wrapper.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &object_wrapper)?;
            }
        }

        // Persist Operator.
        {
            let path = path.join("operator");
            fs::create_dir_all(&path)?;
            for operator in self.operator.values() {
                let path = path.join(format!("{}.json", operator.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &operator)?;
            }
        }

        // Persist Parameter.
        {
            let path = path.join("parameter");
            fs::create_dir_all(&path)?;
            for parameter in self.parameter.values() {
                let path = path.join(format!("{}.json", parameter.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &parameter)?;
            }
        }

        // Persist Path.
        {
            let path = path.join("x_path");
            fs::create_dir_all(&path)?;
            for x_path in self.x_path.values() {
                let path = path.join(format!("{}.json", x_path.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_path)?;
            }
        }

        // Persist Path Element.
        {
            let path = path.join("path_element");
            fs::create_dir_all(&path)?;
            for path_element in self.path_element.values() {
                let path = path.join(format!("{}.json", path_element.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &path_element)?;
            }
        }

        // Persist Pattern.
        {
            let path = path.join("pattern");
            fs::create_dir_all(&path)?;
            for pattern in self.pattern.values() {
                let path = path.join(format!("{}.json", pattern.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &pattern)?;
            }
        }

        // Persist Plugin.
        {
            let path = path.join("x_plugin");
            fs::create_dir_all(&path)?;
            for x_plugin in self.x_plugin.values() {
                let path = path.join(format!("{}.json", x_plugin.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_plugin)?;
            }
        }

        // Persist Print.
        {
            let path = path.join("x_print");
            fs::create_dir_all(&path)?;
            for x_print in self.x_print.values() {
                let path = path.join(format!("{}.json", x_print.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_print)?;
            }
        }

        // Persist Range Expression.
        {
            let path = path.join("range_expression");
            fs::create_dir_all(&path)?;
            for range_expression in self.range_expression.values() {
                let path = path.join(format!("{}.json", range_expression.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &range_expression)?;
            }
        }

        // Persist Result Statement.
        {
            let path = path.join("result_statement");
            fs::create_dir_all(&path)?;
            for result_statement in self.result_statement.values() {
                let path = path.join(format!("{}.json", result_statement.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &result_statement)?;
            }
        }

        // Persist Return.
        {
            let path = path.join("x_return");
            fs::create_dir_all(&path)?;
            for x_return in self.x_return.values() {
                let path = path.join(format!("{}.json", x_return.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_return)?;
            }
        }

        // Persist Span.
        {
            let path = path.join("span");
            fs::create_dir_all(&path)?;
            for span in self.span.values() {
                let path = path.join(format!("{}.json", span.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &span)?;
            }
        }

        // Persist Statement.
        {
            let path = path.join("statement");
            fs::create_dir_all(&path)?;
            for statement in self.statement.values() {
                let path = path.join(format!("{}.json", statement.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &statement)?;
            }
        }

        // Persist Static Method Call.
        {
            let path = path.join("static_method_call");
            fs::create_dir_all(&path)?;
            for static_method_call in self.static_method_call.values() {
                let path = path.join(format!("{}.json", static_method_call.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &static_method_call)?;
            }
        }

        // Persist String Literal.
        {
            let path = path.join("string_literal");
            fs::create_dir_all(&path)?;
            for string_literal in self.string_literal.values() {
                let path = path.join(format!("{}.json", string_literal.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &string_literal)?;
            }
        }

        // Persist Struct.
        {
            let path = path.join("woog_struct");
            fs::create_dir_all(&path)?;
            for woog_struct in self.woog_struct.values() {
                let path = path.join(format!("{}.json", woog_struct.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &woog_struct)?;
            }
        }

        // Persist Struct Expression.
        {
            let path = path.join("struct_expression");
            fs::create_dir_all(&path)?;
            for struct_expression in self.struct_expression.values() {
                let path = path.join(format!("{}.json", struct_expression.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &struct_expression)?;
            }
        }

        // Persist Struct Field.
        {
            let path = path.join("struct_field");
            fs::create_dir_all(&path)?;
            for struct_field in self.struct_field.values() {
                let path = path.join(format!("{}.json", struct_field.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &struct_field)?;
            }
        }

        // Persist Struct Generic.
        {
            let path = path.join("struct_generic");
            fs::create_dir_all(&path)?;
            for struct_generic in self.struct_generic.values() {
                let path = path.join(format!("{}.json", struct_generic.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &struct_generic)?;
            }
        }

        // Persist Tuple Field.
        {
            let path = path.join("tuple_field");
            fs::create_dir_all(&path)?;
            for tuple_field in self.tuple_field.values() {
                let path = path.join(format!("{}.json", tuple_field.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &tuple_field)?;
            }
        }

        // Persist Type Cast.
        {
            let path = path.join("type_cast");
            fs::create_dir_all(&path)?;
            for type_cast in self.type_cast.values() {
                let path = path.join(format!("{}.json", type_cast.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &type_cast)?;
            }
        }

        // Persist Unary.
        {
            let path = path.join("unary");
            fs::create_dir_all(&path)?;
            for unary in self.unary.values() {
                let path = path.join(format!("{}.json", unary.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &unary)?;
            }
        }

        // Persist Unit.
        {
            let path = path.join("unit");
            fs::create_dir_all(&path)?;
            for unit in self.unit.values() {
                let path = path.join(format!("{}.json", unit.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &unit)?;
            }
        }

        // Persist Unnamed Field Expression.
        {
            let path = path.join("unnamed_field_expression");
            fs::create_dir_all(&path)?;
            for unnamed_field_expression in self.unnamed_field_expression.values() {
                let path = path.join(format!("{}.json", unnamed_field_expression.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &unnamed_field_expression)?;
            }
        }

        // Persist Value.
        {
            let path = path.join("x_value");
            fs::create_dir_all(&path)?;
            for x_value in self.x_value.values() {
                let path = path.join(format!("{}.json", x_value.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &x_value)?;
            }
        }

        // Persist Value Type.
        {
            let path = path.join("value_type");
            fs::create_dir_all(&path)?;
            for value_type in self.value_type.values() {
                let path = path.join(format!("{}.json", value_type.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &value_type)?;
            }
        }

        // Persist Variable.
        {
            let path = path.join("variable");
            fs::create_dir_all(&path)?;
            for variable in self.variable.values() {
                let path = path.join(format!("{}.json", variable.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &variable)?;
            }
        }

        // Persist Variable Expression.
        {
            let path = path.join("variable_expression");
            fs::create_dir_all(&path)?;
            for variable_expression in self.variable_expression.values() {
                let path = path.join(format!("{}.json", variable_expression.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &variable_expression)?;
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
                let argument: Argument = serde_json::from_reader(reader)?;
                store.argument.insert(argument.id, argument);
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
                let a_wait: AWait = serde_json::from_reader(reader)?;
                store.a_wait.insert(a_wait.id, a_wait);
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
                let binary: Binary = serde_json::from_reader(reader)?;
                store.binary.insert(binary.id, binary);
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
                let block: Block = serde_json::from_reader(reader)?;
                store.block.insert(block.id, block);
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
                let body: Body = serde_json::from_reader(reader)?;
                store.body.insert(body.id, body);
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
                let boolean_literal: BooleanLiteral = serde_json::from_reader(reader)?;
                store
                    .boolean_literal
                    .insert(boolean_literal.id, boolean_literal);
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
                let boolean_operator: BooleanOperator = serde_json::from_reader(reader)?;
                store
                    .boolean_operator
                    .insert(boolean_operator.id, boolean_operator);
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
                let call: Call = serde_json::from_reader(reader)?;
                store.call.insert(call.id, call);
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
                let comparison: Comparison = serde_json::from_reader(reader)?;
                store.comparison.insert(comparison.id, comparison);
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
                let data_structure: DataStructure = serde_json::from_reader(reader)?;
                store
                    .data_structure
                    .insert(data_structure.id, data_structure);
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
                let dwarf_source_file: DwarfSourceFile = serde_json::from_reader(reader)?;
                store
                    .dwarf_source_file
                    .insert(dwarf_source_file.id, dwarf_source_file);
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
                let enum_field: EnumField = serde_json::from_reader(reader)?;
                store.enum_field.insert(enum_field.id, enum_field);
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
                let enum_generic: EnumGeneric = serde_json::from_reader(reader)?;
                store.enum_generic.insert(enum_generic.id, enum_generic);
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
                let enumeration: Enumeration = serde_json::from_reader(reader)?;
                store
                    .enumeration_id_by_name
                    .insert(enumeration.name.to_upper_camel_case(), enumeration.id);
                store.enumeration.insert(enumeration.id, enumeration);
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
                let expression: Expression = serde_json::from_reader(reader)?;
                store.expression.insert(expression.id, expression);
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
                let expression_statement: ExpressionStatement = serde_json::from_reader(reader)?;
                store
                    .expression_statement
                    .insert(expression_statement.id, expression_statement);
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
                let external_implementation: ExternalImplementation =
                    serde_json::from_reader(reader)?;
                store
                    .external_implementation
                    .insert(external_implementation.id, external_implementation);
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
                let field: Field = serde_json::from_reader(reader)?;
                store
                    .field_id_by_name
                    .insert(field.name.to_upper_camel_case(), field.id);
                store.field.insert(field.id, field);
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
                let field_access: FieldAccess = serde_json::from_reader(reader)?;
                store.field_access.insert(field_access.id, field_access);
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
                let field_access_target: FieldAccessTarget = serde_json::from_reader(reader)?;
                store
                    .field_access_target
                    .insert(field_access_target.id, field_access_target);
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
                let field_expression: FieldExpression = serde_json::from_reader(reader)?;
                store
                    .field_expression
                    .insert(field_expression.id, field_expression);
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
                let float_literal: FloatLiteral = serde_json::from_reader(reader)?;
                store.float_literal.insert(float_literal.id, float_literal);
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
                let for_loop: ForLoop = serde_json::from_reader(reader)?;
                store.for_loop.insert(for_loop.id, for_loop);
            }
        }

        // Load Func Generic.
        {
            let path = path.join("func_generic");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let func_generic: FuncGeneric = serde_json::from_reader(reader)?;
                store.func_generic.insert(func_generic.id, func_generic);
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
                let function: Function = serde_json::from_reader(reader)?;
                store
                    .function_id_by_name
                    .insert(function.name.to_upper_camel_case(), function.id);
                store.function.insert(function.id, function);
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
                let function_call: FunctionCall = serde_json::from_reader(reader)?;
                store.function_call.insert(function_call.id, function_call);
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
                let x_future: XFuture = serde_json::from_reader(reader)?;
                store.x_future.insert(x_future.id, x_future);
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
                let grouped: Grouped = serde_json::from_reader(reader)?;
                store.grouped.insert(grouped.id, grouped);
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
                let x_if: XIf = serde_json::from_reader(reader)?;
                store.x_if.insert(x_if.id, x_if);
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
                let implementation_block: ImplementationBlock = serde_json::from_reader(reader)?;
                store
                    .implementation_block
                    .insert(implementation_block.id, implementation_block);
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
                let import: Import = serde_json::from_reader(reader)?;
                store.import.insert(import.id, import);
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
                let index: Index = serde_json::from_reader(reader)?;
                store.index.insert(index.id, index);
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
                let integer_literal: IntegerLiteral = serde_json::from_reader(reader)?;
                store
                    .integer_literal
                    .insert(integer_literal.id, integer_literal);
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
                let item: Item = serde_json::from_reader(reader)?;
                store.item.insert(item.id, item);
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
                let lambda: Lambda = serde_json::from_reader(reader)?;
                store.lambda.insert(lambda.id, lambda);
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
                let lambda_parameter: LambdaParameter = serde_json::from_reader(reader)?;
                store
                    .lambda_parameter
                    .insert(lambda_parameter.id, lambda_parameter);
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
                let let_statement: LetStatement = serde_json::from_reader(reader)?;
                store.let_statement.insert(let_statement.id, let_statement);
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
                let list: List = serde_json::from_reader(reader)?;
                store.list.insert(list.id, list);
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
                let list_element: ListElement = serde_json::from_reader(reader)?;
                store.list_element.insert(list_element.id, list_element);
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
                let list_expression: ListExpression = serde_json::from_reader(reader)?;
                store
                    .list_expression
                    .insert(list_expression.id, list_expression);
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
                let literal: Literal = serde_json::from_reader(reader)?;
                store.literal.insert(literal.id, literal);
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
                let local_variable: LocalVariable = serde_json::from_reader(reader)?;
                store
                    .local_variable
                    .insert(local_variable.id, local_variable);
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
                let x_macro: XMacro = serde_json::from_reader(reader)?;
                store.x_macro.insert(x_macro.id, x_macro);
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
                let x_match: XMatch = serde_json::from_reader(reader)?;
                store.x_match.insert(x_match.id, x_match);
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
                let method_call: MethodCall = serde_json::from_reader(reader)?;
                store.method_call.insert(method_call.id, method_call);
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
                let named_field_expression: NamedFieldExpression = serde_json::from_reader(reader)?;
                store
                    .named_field_expression
                    .insert(named_field_expression.id, named_field_expression);
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
                let z_object_store: ZObjectStore = serde_json::from_reader(reader)?;
                store
                    .z_object_store_id_by_name
                    .insert(z_object_store.name.to_upper_camel_case(), z_object_store.id);
                store
                    .z_object_store
                    .insert(z_object_store.id, z_object_store);
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
                let object_wrapper: ObjectWrapper = serde_json::from_reader(reader)?;
                store
                    .object_wrapper
                    .insert(object_wrapper.id, object_wrapper);
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
                let operator: Operator = serde_json::from_reader(reader)?;
                store.operator.insert(operator.id, operator);
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
                let parameter: Parameter = serde_json::from_reader(reader)?;
                store.parameter.insert(parameter.id, parameter);
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
                let x_path: XPath = serde_json::from_reader(reader)?;
                store.x_path.insert(x_path.id, x_path);
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
                let path_element: PathElement = serde_json::from_reader(reader)?;
                store.path_element.insert(path_element.id, path_element);
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
                let pattern: Pattern = serde_json::from_reader(reader)?;
                store.pattern.insert(pattern.id, pattern);
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
                let x_plugin: XPlugin = serde_json::from_reader(reader)?;
                store
                    .x_plugin_id_by_name
                    .insert(x_plugin.name.to_upper_camel_case(), x_plugin.id);
                store.x_plugin.insert(x_plugin.id, x_plugin);
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
                let x_print: XPrint = serde_json::from_reader(reader)?;
                store.x_print.insert(x_print.id, x_print);
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
                let range_expression: RangeExpression = serde_json::from_reader(reader)?;
                store
                    .range_expression
                    .insert(range_expression.id, range_expression);
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
                let result_statement: ResultStatement = serde_json::from_reader(reader)?;
                store
                    .result_statement
                    .insert(result_statement.id, result_statement);
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
                let x_return: XReturn = serde_json::from_reader(reader)?;
                store.x_return.insert(x_return.id, x_return);
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
                let span: Span = serde_json::from_reader(reader)?;
                store.span.insert(span.id, span);
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
                let statement: Statement = serde_json::from_reader(reader)?;
                store.statement.insert(statement.id, statement);
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
                let static_method_call: StaticMethodCall = serde_json::from_reader(reader)?;
                store
                    .static_method_call
                    .insert(static_method_call.id, static_method_call);
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
                let string_literal: StringLiteral = serde_json::from_reader(reader)?;
                store
                    .string_literal
                    .insert(string_literal.id, string_literal);
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
                let woog_struct: WoogStruct = serde_json::from_reader(reader)?;
                store
                    .woog_struct_id_by_name
                    .insert(woog_struct.name.to_upper_camel_case(), woog_struct.id);
                store.woog_struct.insert(woog_struct.id, woog_struct);
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
                let struct_expression: StructExpression = serde_json::from_reader(reader)?;
                store
                    .struct_expression
                    .insert(struct_expression.id, struct_expression);
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
                let struct_field: StructField = serde_json::from_reader(reader)?;
                store.struct_field.insert(struct_field.id, struct_field);
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
                let struct_generic: StructGeneric = serde_json::from_reader(reader)?;
                store
                    .struct_generic
                    .insert(struct_generic.id, struct_generic);
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
                let tuple_field: TupleField = serde_json::from_reader(reader)?;
                store.tuple_field.insert(tuple_field.id, tuple_field);
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
                let type_cast: TypeCast = serde_json::from_reader(reader)?;
                store.type_cast.insert(type_cast.id, type_cast);
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
                let unary: Unary = serde_json::from_reader(reader)?;
                store.unary.insert(unary.id, unary);
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
                let unit: Unit = serde_json::from_reader(reader)?;
                store.unit.insert(unit.id, unit);
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
                let unnamed_field_expression: UnnamedFieldExpression =
                    serde_json::from_reader(reader)?;
                store
                    .unnamed_field_expression
                    .insert(unnamed_field_expression.id, unnamed_field_expression);
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
                let x_value: XValue = serde_json::from_reader(reader)?;
                store.x_value.insert(x_value.id, x_value);
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
                let value_type: ValueType = serde_json::from_reader(reader)?;
                store.value_type.insert(value_type.id, value_type);
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
                let variable: Variable = serde_json::from_reader(reader)?;
                store.variable.insert(variable.id, variable);
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
                let variable_expression: VariableExpression = serde_json::from_reader(reader)?;
                store
                    .variable_expression
                    .insert(variable_expression.id, variable_expression);
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
