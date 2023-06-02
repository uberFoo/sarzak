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
//! * [`Negation`]
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
//! * [`XValue`]
//! * [`ValueType`]
//! * [`Variable`]
//! * [`VariableExpression`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog-object-store-definition"}}}
use parking_lot::Mutex;
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

use crate::v2::lu_dog::types::{
    Argument, Binary, Block, BooleanLiteral, Call, Comparison, DwarfSourceFile, Error,
    ErrorExpression, Expression, ExpressionStatement, Field, FieldAccess, FieldAccessTarget,
    FieldExpression, FloatLiteral, ForLoop, Function, Grouped, Implementation, Import, Index,
    IntegerLiteral, Item, LetStatement, List, ListElement, ListExpression, Literal, LocalVariable,
    MethodCall, Negation, Operator, Parameter, Print, RangeExpression, Reference, ResultStatement,
    Span, Statement, StaticMethodCall, StringLiteral, StructExpression, TypeCast, ValueType,
    Variable, VariableExpression, WoogOption, WoogStruct, XIf, XReturn, XValue, ZObjectStore,
    ZSome, ADDITION, ASSIGNMENT, DEBUGGER, DIVISION, EMPTY, EQUAL, FALSE_LITERAL, GREATER_THAN,
    GREATER_THAN_OR_EQUAL, LESS_THAN_OR_EQUAL, MULTIPLICATION, RANGE, SUBTRACTION, TRUE_LITERAL,
    UNKNOWN, UNKNOWN_VARIABLE, Z_NONE,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    argument: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<Argument>>, SystemTime)>>>,
    binary: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<Binary>>, SystemTime)>>>,
    block: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<Block>>, SystemTime)>>>,
    boolean_literal: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<BooleanLiteral>>, SystemTime)>>>,
    call: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<Call>>, SystemTime)>>>,
    comparison: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<Comparison>>, SystemTime)>>>,
    dwarf_source_file: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<DwarfSourceFile>>, SystemTime)>>>,
    error: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<Error>>, SystemTime)>>>,
    error_expression: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<ErrorExpression>>, SystemTime)>>>,
    expression: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<Expression>>, SystemTime)>>>,
    expression_statement: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<ExpressionStatement>>, SystemTime)>>>,
    field: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<Field>>, SystemTime)>>>,
    field_id_by_name: Arc<Mutex<HashMap<String, (Uuid, SystemTime)>>>,
    field_access: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<FieldAccess>>, SystemTime)>>>,
    field_access_target: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<FieldAccessTarget>>, SystemTime)>>>,
    field_expression: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<FieldExpression>>, SystemTime)>>>,
    float_literal: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<FloatLiteral>>, SystemTime)>>>,
    for_loop: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<ForLoop>>, SystemTime)>>>,
    function: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<Function>>, SystemTime)>>>,
    function_id_by_name: Arc<Mutex<HashMap<String, (Uuid, SystemTime)>>>,
    grouped: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<Grouped>>, SystemTime)>>>,
    x_if: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<XIf>>, SystemTime)>>>,
    implementation: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<Implementation>>, SystemTime)>>>,
    import: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<Import>>, SystemTime)>>>,
    index: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<Index>>, SystemTime)>>>,
    integer_literal: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<IntegerLiteral>>, SystemTime)>>>,
    item: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<Item>>, SystemTime)>>>,
    let_statement: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<LetStatement>>, SystemTime)>>>,
    list: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<List>>, SystemTime)>>>,
    list_element: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<ListElement>>, SystemTime)>>>,
    list_expression: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<ListExpression>>, SystemTime)>>>,
    literal: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<Literal>>, SystemTime)>>>,
    local_variable: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<LocalVariable>>, SystemTime)>>>,
    method_call: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<MethodCall>>, SystemTime)>>>,
    negation: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<Negation>>, SystemTime)>>>,
    z_object_store: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<ZObjectStore>>, SystemTime)>>>,
    operator: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<Operator>>, SystemTime)>>>,
    woog_option: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<WoogOption>>, SystemTime)>>>,
    parameter: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<Parameter>>, SystemTime)>>>,
    print: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<Print>>, SystemTime)>>>,
    range_expression: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<RangeExpression>>, SystemTime)>>>,
    reference: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<Reference>>, SystemTime)>>>,
    result_statement: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<ResultStatement>>, SystemTime)>>>,
    x_return: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<XReturn>>, SystemTime)>>>,
    z_some: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<ZSome>>, SystemTime)>>>,
    span: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<Span>>, SystemTime)>>>,
    statement: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<Statement>>, SystemTime)>>>,
    static_method_call: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<StaticMethodCall>>, SystemTime)>>>,
    string_literal: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<StringLiteral>>, SystemTime)>>>,
    woog_struct: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<WoogStruct>>, SystemTime)>>>,
    woog_struct_id_by_name: Arc<Mutex<HashMap<String, (Uuid, SystemTime)>>>,
    struct_expression: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<StructExpression>>, SystemTime)>>>,
    type_cast: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<TypeCast>>, SystemTime)>>>,
    x_value: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<XValue>>, SystemTime)>>>,
    value_type: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<ValueType>>, SystemTime)>>>,
    variable: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<Variable>>, SystemTime)>>>,
    variable_expression: Arc<Mutex<HashMap<Uuid, (Arc<Mutex<VariableExpression>>, SystemTime)>>>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            argument: Arc::new(Mutex::new(HashMap::default())),
            binary: Arc::new(Mutex::new(HashMap::default())),
            block: Arc::new(Mutex::new(HashMap::default())),
            boolean_literal: Arc::new(Mutex::new(HashMap::default())),
            call: Arc::new(Mutex::new(HashMap::default())),
            comparison: Arc::new(Mutex::new(HashMap::default())),
            dwarf_source_file: Arc::new(Mutex::new(HashMap::default())),
            error: Arc::new(Mutex::new(HashMap::default())),
            error_expression: Arc::new(Mutex::new(HashMap::default())),
            expression: Arc::new(Mutex::new(HashMap::default())),
            expression_statement: Arc::new(Mutex::new(HashMap::default())),
            field: Arc::new(Mutex::new(HashMap::default())),
            field_id_by_name: Arc::new(Mutex::new(HashMap::default())),
            field_access: Arc::new(Mutex::new(HashMap::default())),
            field_access_target: Arc::new(Mutex::new(HashMap::default())),
            field_expression: Arc::new(Mutex::new(HashMap::default())),
            float_literal: Arc::new(Mutex::new(HashMap::default())),
            for_loop: Arc::new(Mutex::new(HashMap::default())),
            function: Arc::new(Mutex::new(HashMap::default())),
            function_id_by_name: Arc::new(Mutex::new(HashMap::default())),
            grouped: Arc::new(Mutex::new(HashMap::default())),
            x_if: Arc::new(Mutex::new(HashMap::default())),
            implementation: Arc::new(Mutex::new(HashMap::default())),
            import: Arc::new(Mutex::new(HashMap::default())),
            index: Arc::new(Mutex::new(HashMap::default())),
            integer_literal: Arc::new(Mutex::new(HashMap::default())),
            item: Arc::new(Mutex::new(HashMap::default())),
            let_statement: Arc::new(Mutex::new(HashMap::default())),
            list: Arc::new(Mutex::new(HashMap::default())),
            list_element: Arc::new(Mutex::new(HashMap::default())),
            list_expression: Arc::new(Mutex::new(HashMap::default())),
            literal: Arc::new(Mutex::new(HashMap::default())),
            local_variable: Arc::new(Mutex::new(HashMap::default())),
            method_call: Arc::new(Mutex::new(HashMap::default())),
            negation: Arc::new(Mutex::new(HashMap::default())),
            z_object_store: Arc::new(Mutex::new(HashMap::default())),
            operator: Arc::new(Mutex::new(HashMap::default())),
            woog_option: Arc::new(Mutex::new(HashMap::default())),
            parameter: Arc::new(Mutex::new(HashMap::default())),
            print: Arc::new(Mutex::new(HashMap::default())),
            range_expression: Arc::new(Mutex::new(HashMap::default())),
            reference: Arc::new(Mutex::new(HashMap::default())),
            result_statement: Arc::new(Mutex::new(HashMap::default())),
            x_return: Arc::new(Mutex::new(HashMap::default())),
            z_some: Arc::new(Mutex::new(HashMap::default())),
            span: Arc::new(Mutex::new(HashMap::default())),
            statement: Arc::new(Mutex::new(HashMap::default())),
            static_method_call: Arc::new(Mutex::new(HashMap::default())),
            string_literal: Arc::new(Mutex::new(HashMap::default())),
            woog_struct: Arc::new(Mutex::new(HashMap::default())),
            woog_struct_id_by_name: Arc::new(Mutex::new(HashMap::default())),
            struct_expression: Arc::new(Mutex::new(HashMap::default())),
            type_cast: Arc::new(Mutex::new(HashMap::default())),
            x_value: Arc::new(Mutex::new(HashMap::default())),
            value_type: Arc::new(Mutex::new(HashMap::default())),
            variable: Arc::new(Mutex::new(HashMap::default())),
            variable_expression: Arc::new(Mutex::new(HashMap::default())),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥
        store.inter_binary(Arc::new(Mutex::new(Binary::Addition(ADDITION))));
        store.inter_binary(Arc::new(Mutex::new(Binary::Assignment(ASSIGNMENT))));
        store.inter_binary(Arc::new(Mutex::new(Binary::Division(DIVISION))));
        store.inter_binary(Arc::new(Mutex::new(Binary::Multiplication(MULTIPLICATION))));
        store.inter_binary(Arc::new(Mutex::new(Binary::Subtraction(SUBTRACTION))));
        store.inter_boolean_literal(Arc::new(Mutex::new(BooleanLiteral::FalseLiteral(
            FALSE_LITERAL,
        ))));
        store.inter_boolean_literal(Arc::new(Mutex::new(BooleanLiteral::TrueLiteral(
            TRUE_LITERAL,
        ))));
        store.inter_comparison(Arc::new(Mutex::new(Comparison::Equal(EQUAL))));
        store.inter_comparison(Arc::new(Mutex::new(Comparison::GreaterThan(GREATER_THAN))));
        store.inter_comparison(Arc::new(Mutex::new(Comparison::GreaterThanOrEqual(
            GREATER_THAN_OR_EQUAL,
        ))));
        store.inter_comparison(Arc::new(Mutex::new(Comparison::LessThanOrEqual(
            LESS_THAN_OR_EQUAL,
        ))));
        store.inter_error(Arc::new(Mutex::new(Error::UnknownVariable(
            UNKNOWN_VARIABLE,
        ))));
        store.inter_expression(Arc::new(Mutex::new(Expression::Debugger(DEBUGGER))));
        store.inter_expression(Arc::new(Mutex::new(Expression::Literal(
            Literal::BooleanLiteral(BooleanLiteral::FalseLiteral(FALSE_LITERAL).id()).id(),
        ))));
        store.inter_expression(Arc::new(Mutex::new(Expression::Literal(
            Literal::BooleanLiteral(BooleanLiteral::TrueLiteral(TRUE_LITERAL).id()).id(),
        ))));
        store.inter_expression(Arc::new(Mutex::new(Expression::ZNone(Z_NONE))));
        store.inter_literal(Arc::new(Mutex::new(Literal::BooleanLiteral(
            BooleanLiteral::FalseLiteral(FALSE_LITERAL).id(),
        ))));
        store.inter_literal(Arc::new(Mutex::new(Literal::BooleanLiteral(
            BooleanLiteral::TrueLiteral(TRUE_LITERAL).id(),
        ))));
        store.inter_value_type(Arc::new(Mutex::new(ValueType::Empty(EMPTY))));
        store.inter_value_type(Arc::new(Mutex::new(ValueType::Error(
            Error::UnknownVariable(UNKNOWN_VARIABLE).id(),
        ))));
        store.inter_value_type(Arc::new(Mutex::new(ValueType::Range(RANGE))));
        store.inter_value_type(Arc::new(Mutex::new(ValueType::Unknown(UNKNOWN))));

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog-object-store-methods"}}}
    /// Inter (insert) [`Argument`] into the store.
    ///
    pub fn inter_argument(&mut self, argument: Arc<Mutex<Argument>>) {
        let read = argument.lock();
        self.argument
            .lock()
            .insert(read.id, (argument.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Argument`] from the store.
    ///
    pub fn exhume_argument(&self, id: &Uuid) -> Option<Arc<Mutex<Argument>>> {
        self.argument
            .lock()
            .get(id)
            .map(|argument| argument.0.clone())
    }

    /// Exorcise (remove) [`Argument`] from the store.
    ///
    pub fn exorcise_argument(&mut self, id: &Uuid) -> Option<Arc<Mutex<Argument>>> {
        self.argument
            .lock()
            .remove(id)
            .map(|argument| argument.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Argument>`.
    ///
    pub fn iter_argument(&self) -> impl Iterator<Item = Arc<Mutex<Argument>>> + '_ {
        let values: Vec<Arc<Mutex<Argument>>> = self
            .argument
            .lock()
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
            .lock()
            .get(&argument.id)
            .map(|argument| argument.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Binary`] into the store.
    ///
    pub fn inter_binary(&mut self, binary: Arc<Mutex<Binary>>) {
        let read = binary.lock();
        self.binary
            .lock()
            .insert(read.id(), (binary.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Binary`] from the store.
    ///
    pub fn exhume_binary(&self, id: &Uuid) -> Option<Arc<Mutex<Binary>>> {
        self.binary.lock().get(id).map(|binary| binary.0.clone())
    }

    /// Exorcise (remove) [`Binary`] from the store.
    ///
    pub fn exorcise_binary(&mut self, id: &Uuid) -> Option<Arc<Mutex<Binary>>> {
        self.binary.lock().remove(id).map(|binary| binary.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Binary>`.
    ///
    pub fn iter_binary(&self) -> impl Iterator<Item = Arc<Mutex<Binary>>> + '_ {
        let values: Vec<Arc<Mutex<Binary>>> = self
            .binary
            .lock()
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
            .lock()
            .get(&binary.id())
            .map(|binary| binary.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Block`] into the store.
    ///
    pub fn inter_block(&mut self, block: Arc<Mutex<Block>>) {
        let read = block.lock();
        self.block
            .lock()
            .insert(read.id, (block.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Block`] from the store.
    ///
    pub fn exhume_block(&self, id: &Uuid) -> Option<Arc<Mutex<Block>>> {
        self.block.lock().get(id).map(|block| block.0.clone())
    }

    /// Exorcise (remove) [`Block`] from the store.
    ///
    pub fn exorcise_block(&mut self, id: &Uuid) -> Option<Arc<Mutex<Block>>> {
        self.block.lock().remove(id).map(|block| block.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Block>`.
    ///
    pub fn iter_block(&self) -> impl Iterator<Item = Arc<Mutex<Block>>> + '_ {
        let values: Vec<Arc<Mutex<Block>>> = self
            .block
            .lock()
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
            .lock()
            .get(&block.id)
            .map(|block| block.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`BooleanLiteral`] into the store.
    ///
    pub fn inter_boolean_literal(&mut self, boolean_literal: Arc<Mutex<BooleanLiteral>>) {
        let read = boolean_literal.lock();
        self.boolean_literal
            .lock()
            .insert(read.id(), (boolean_literal.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`BooleanLiteral`] from the store.
    ///
    pub fn exhume_boolean_literal(&self, id: &Uuid) -> Option<Arc<Mutex<BooleanLiteral>>> {
        self.boolean_literal
            .lock()
            .get(id)
            .map(|boolean_literal| boolean_literal.0.clone())
    }

    /// Exorcise (remove) [`BooleanLiteral`] from the store.
    ///
    pub fn exorcise_boolean_literal(&mut self, id: &Uuid) -> Option<Arc<Mutex<BooleanLiteral>>> {
        self.boolean_literal
            .lock()
            .remove(id)
            .map(|boolean_literal| boolean_literal.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanLiteral>`.
    ///
    pub fn iter_boolean_literal(&self) -> impl Iterator<Item = Arc<Mutex<BooleanLiteral>>> + '_ {
        let values: Vec<Arc<Mutex<BooleanLiteral>>> = self
            .boolean_literal
            .lock()
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
            .lock()
            .get(&boolean_literal.id())
            .map(|boolean_literal| boolean_literal.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Call`] into the store.
    ///
    pub fn inter_call(&mut self, call: Arc<Mutex<Call>>) {
        let read = call.lock();
        self.call
            .lock()
            .insert(read.id, (call.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Call`] from the store.
    ///
    pub fn exhume_call(&self, id: &Uuid) -> Option<Arc<Mutex<Call>>> {
        self.call.lock().get(id).map(|call| call.0.clone())
    }

    /// Exorcise (remove) [`Call`] from the store.
    ///
    pub fn exorcise_call(&mut self, id: &Uuid) -> Option<Arc<Mutex<Call>>> {
        self.call.lock().remove(id).map(|call| call.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Call>`.
    ///
    pub fn iter_call(&self) -> impl Iterator<Item = Arc<Mutex<Call>>> + '_ {
        let values: Vec<Arc<Mutex<Call>>> = self
            .call
            .lock()
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
            .lock()
            .get(&call.id)
            .map(|call| call.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Comparison`] into the store.
    ///
    pub fn inter_comparison(&mut self, comparison: Arc<Mutex<Comparison>>) {
        let read = comparison.lock();
        self.comparison
            .lock()
            .insert(read.id(), (comparison.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Comparison`] from the store.
    ///
    pub fn exhume_comparison(&self, id: &Uuid) -> Option<Arc<Mutex<Comparison>>> {
        self.comparison
            .lock()
            .get(id)
            .map(|comparison| comparison.0.clone())
    }

    /// Exorcise (remove) [`Comparison`] from the store.
    ///
    pub fn exorcise_comparison(&mut self, id: &Uuid) -> Option<Arc<Mutex<Comparison>>> {
        self.comparison
            .lock()
            .remove(id)
            .map(|comparison| comparison.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Comparison>`.
    ///
    pub fn iter_comparison(&self) -> impl Iterator<Item = Arc<Mutex<Comparison>>> + '_ {
        let values: Vec<Arc<Mutex<Comparison>>> = self
            .comparison
            .lock()
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
            .lock()
            .get(&comparison.id())
            .map(|comparison| comparison.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`DwarfSourceFile`] into the store.
    ///
    pub fn inter_dwarf_source_file(&mut self, dwarf_source_file: Arc<Mutex<DwarfSourceFile>>) {
        let read = dwarf_source_file.lock();
        self.dwarf_source_file
            .lock()
            .insert(read.id, (dwarf_source_file.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`DwarfSourceFile`] from the store.
    ///
    pub fn exhume_dwarf_source_file(&self, id: &Uuid) -> Option<Arc<Mutex<DwarfSourceFile>>> {
        self.dwarf_source_file
            .lock()
            .get(id)
            .map(|dwarf_source_file| dwarf_source_file.0.clone())
    }

    /// Exorcise (remove) [`DwarfSourceFile`] from the store.
    ///
    pub fn exorcise_dwarf_source_file(&mut self, id: &Uuid) -> Option<Arc<Mutex<DwarfSourceFile>>> {
        self.dwarf_source_file
            .lock()
            .remove(id)
            .map(|dwarf_source_file| dwarf_source_file.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, DwarfSourceFile>`.
    ///
    pub fn iter_dwarf_source_file(&self) -> impl Iterator<Item = Arc<Mutex<DwarfSourceFile>>> + '_ {
        let values: Vec<Arc<Mutex<DwarfSourceFile>>> = self
            .dwarf_source_file
            .lock()
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
            .lock()
            .get(&dwarf_source_file.id)
            .map(|dwarf_source_file| dwarf_source_file.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Error`] into the store.
    ///
    pub fn inter_error(&mut self, error: Arc<Mutex<Error>>) {
        let read = error.lock();
        self.error
            .lock()
            .insert(read.id(), (error.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Error`] from the store.
    ///
    pub fn exhume_error(&self, id: &Uuid) -> Option<Arc<Mutex<Error>>> {
        self.error.lock().get(id).map(|error| error.0.clone())
    }

    /// Exorcise (remove) [`Error`] from the store.
    ///
    pub fn exorcise_error(&mut self, id: &Uuid) -> Option<Arc<Mutex<Error>>> {
        self.error.lock().remove(id).map(|error| error.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Error>`.
    ///
    pub fn iter_error(&self) -> impl Iterator<Item = Arc<Mutex<Error>>> + '_ {
        let values: Vec<Arc<Mutex<Error>>> = self
            .error
            .lock()
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
            .lock()
            .get(&error.id())
            .map(|error| error.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ErrorExpression`] into the store.
    ///
    pub fn inter_error_expression(&mut self, error_expression: Arc<Mutex<ErrorExpression>>) {
        let read = error_expression.lock();
        self.error_expression
            .lock()
            .insert(read.id, (error_expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ErrorExpression`] from the store.
    ///
    pub fn exhume_error_expression(&self, id: &Uuid) -> Option<Arc<Mutex<ErrorExpression>>> {
        self.error_expression
            .lock()
            .get(id)
            .map(|error_expression| error_expression.0.clone())
    }

    /// Exorcise (remove) [`ErrorExpression`] from the store.
    ///
    pub fn exorcise_error_expression(&mut self, id: &Uuid) -> Option<Arc<Mutex<ErrorExpression>>> {
        self.error_expression
            .lock()
            .remove(id)
            .map(|error_expression| error_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ErrorExpression>`.
    ///
    pub fn iter_error_expression(&self) -> impl Iterator<Item = Arc<Mutex<ErrorExpression>>> + '_ {
        let values: Vec<Arc<Mutex<ErrorExpression>>> = self
            .error_expression
            .lock()
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
            .lock()
            .get(&error_expression.id)
            .map(|error_expression| error_expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Expression`] into the store.
    ///
    pub fn inter_expression(&mut self, expression: Arc<Mutex<Expression>>) {
        let read = expression.lock();
        self.expression
            .lock()
            .insert(read.id(), (expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Expression`] from the store.
    ///
    pub fn exhume_expression(&self, id: &Uuid) -> Option<Arc<Mutex<Expression>>> {
        self.expression
            .lock()
            .get(id)
            .map(|expression| expression.0.clone())
    }

    /// Exorcise (remove) [`Expression`] from the store.
    ///
    pub fn exorcise_expression(&mut self, id: &Uuid) -> Option<Arc<Mutex<Expression>>> {
        self.expression
            .lock()
            .remove(id)
            .map(|expression| expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Expression>`.
    ///
    pub fn iter_expression(&self) -> impl Iterator<Item = Arc<Mutex<Expression>>> + '_ {
        let values: Vec<Arc<Mutex<Expression>>> = self
            .expression
            .lock()
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
            .lock()
            .get(&expression.id())
            .map(|expression| expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ExpressionStatement`] into the store.
    ///
    pub fn inter_expression_statement(
        &mut self,
        expression_statement: Arc<Mutex<ExpressionStatement>>,
    ) {
        let read = expression_statement.lock();
        self.expression_statement
            .lock()
            .insert(read.id, (expression_statement.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ExpressionStatement`] from the store.
    ///
    pub fn exhume_expression_statement(
        &self,
        id: &Uuid,
    ) -> Option<Arc<Mutex<ExpressionStatement>>> {
        self.expression_statement
            .lock()
            .get(id)
            .map(|expression_statement| expression_statement.0.clone())
    }

    /// Exorcise (remove) [`ExpressionStatement`] from the store.
    ///
    pub fn exorcise_expression_statement(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<Mutex<ExpressionStatement>>> {
        self.expression_statement
            .lock()
            .remove(id)
            .map(|expression_statement| expression_statement.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ExpressionStatement>`.
    ///
    pub fn iter_expression_statement(
        &self,
    ) -> impl Iterator<Item = Arc<Mutex<ExpressionStatement>>> + '_ {
        let values: Vec<Arc<Mutex<ExpressionStatement>>> = self
            .expression_statement
            .lock()
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
            .lock()
            .get(&expression_statement.id)
            .map(|expression_statement| expression_statement.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Field`] into the store.
    ///
    pub fn inter_field(&mut self, field: Arc<Mutex<Field>>) {
        let read = field.lock();
        let value = (field.clone(), SystemTime::now());
        self.field_id_by_name
            .lock()
            .insert(read.name.to_upper_camel_case(), (read.id, value.1));
        self.field.lock().insert(read.id, value);
    }

    /// Exhume (get) [`Field`] from the store.
    ///
    pub fn exhume_field(&self, id: &Uuid) -> Option<Arc<Mutex<Field>>> {
        self.field.lock().get(id).map(|field| field.0.clone())
    }

    /// Exorcise (remove) [`Field`] from the store.
    ///
    pub fn exorcise_field(&mut self, id: &Uuid) -> Option<Arc<Mutex<Field>>> {
        self.field.lock().remove(id).map(|field| field.0.clone())
    }

    /// Exhume [`Field`] id from the store by name.
    ///
    pub fn exhume_field_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.field_id_by_name.lock().get(name).map(|field| field.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Field>`.
    ///
    pub fn iter_field(&self) -> impl Iterator<Item = Arc<Mutex<Field>>> + '_ {
        let values: Vec<Arc<Mutex<Field>>> = self
            .field
            .lock()
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
            .lock()
            .get(&field.id)
            .map(|field| field.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`FieldAccess`] into the store.
    ///
    pub fn inter_field_access(&mut self, field_access: Arc<Mutex<FieldAccess>>) {
        let read = field_access.lock();
        self.field_access
            .lock()
            .insert(read.id, (field_access.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`FieldAccess`] from the store.
    ///
    pub fn exhume_field_access(&self, id: &Uuid) -> Option<Arc<Mutex<FieldAccess>>> {
        self.field_access
            .lock()
            .get(id)
            .map(|field_access| field_access.0.clone())
    }

    /// Exorcise (remove) [`FieldAccess`] from the store.
    ///
    pub fn exorcise_field_access(&mut self, id: &Uuid) -> Option<Arc<Mutex<FieldAccess>>> {
        self.field_access
            .lock()
            .remove(id)
            .map(|field_access| field_access.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccess>`.
    ///
    pub fn iter_field_access(&self) -> impl Iterator<Item = Arc<Mutex<FieldAccess>>> + '_ {
        let values: Vec<Arc<Mutex<FieldAccess>>> = self
            .field_access
            .lock()
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
            .lock()
            .get(&field_access.id)
            .map(|field_access| field_access.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`FieldAccessTarget`] into the store.
    ///
    pub fn inter_field_access_target(
        &mut self,
        field_access_target: Arc<Mutex<FieldAccessTarget>>,
    ) {
        let read = field_access_target.lock();
        self.field_access_target
            .lock()
            .insert(read.id(), (field_access_target.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`FieldAccessTarget`] from the store.
    ///
    pub fn exhume_field_access_target(&self, id: &Uuid) -> Option<Arc<Mutex<FieldAccessTarget>>> {
        self.field_access_target
            .lock()
            .get(id)
            .map(|field_access_target| field_access_target.0.clone())
    }

    /// Exorcise (remove) [`FieldAccessTarget`] from the store.
    ///
    pub fn exorcise_field_access_target(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<Mutex<FieldAccessTarget>>> {
        self.field_access_target
            .lock()
            .remove(id)
            .map(|field_access_target| field_access_target.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccessTarget>`.
    ///
    pub fn iter_field_access_target(
        &self,
    ) -> impl Iterator<Item = Arc<Mutex<FieldAccessTarget>>> + '_ {
        let values: Vec<Arc<Mutex<FieldAccessTarget>>> = self
            .field_access_target
            .lock()
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
            .lock()
            .get(&field_access_target.id())
            .map(|field_access_target| field_access_target.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`FieldExpression`] into the store.
    ///
    pub fn inter_field_expression(&mut self, field_expression: Arc<Mutex<FieldExpression>>) {
        let read = field_expression.lock();
        self.field_expression
            .lock()
            .insert(read.id, (field_expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`FieldExpression`] from the store.
    ///
    pub fn exhume_field_expression(&self, id: &Uuid) -> Option<Arc<Mutex<FieldExpression>>> {
        self.field_expression
            .lock()
            .get(id)
            .map(|field_expression| field_expression.0.clone())
    }

    /// Exorcise (remove) [`FieldExpression`] from the store.
    ///
    pub fn exorcise_field_expression(&mut self, id: &Uuid) -> Option<Arc<Mutex<FieldExpression>>> {
        self.field_expression
            .lock()
            .remove(id)
            .map(|field_expression| field_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldExpression>`.
    ///
    pub fn iter_field_expression(&self) -> impl Iterator<Item = Arc<Mutex<FieldExpression>>> + '_ {
        let values: Vec<Arc<Mutex<FieldExpression>>> = self
            .field_expression
            .lock()
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
            .lock()
            .get(&field_expression.id)
            .map(|field_expression| field_expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`FloatLiteral`] into the store.
    ///
    pub fn inter_float_literal(&mut self, float_literal: Arc<Mutex<FloatLiteral>>) {
        let read = float_literal.lock();
        self.float_literal
            .lock()
            .insert(read.id, (float_literal.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`FloatLiteral`] from the store.
    ///
    pub fn exhume_float_literal(&self, id: &Uuid) -> Option<Arc<Mutex<FloatLiteral>>> {
        self.float_literal
            .lock()
            .get(id)
            .map(|float_literal| float_literal.0.clone())
    }

    /// Exorcise (remove) [`FloatLiteral`] from the store.
    ///
    pub fn exorcise_float_literal(&mut self, id: &Uuid) -> Option<Arc<Mutex<FloatLiteral>>> {
        self.float_literal
            .lock()
            .remove(id)
            .map(|float_literal| float_literal.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FloatLiteral>`.
    ///
    pub fn iter_float_literal(&self) -> impl Iterator<Item = Arc<Mutex<FloatLiteral>>> + '_ {
        let values: Vec<Arc<Mutex<FloatLiteral>>> = self
            .float_literal
            .lock()
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
            .lock()
            .get(&float_literal.id)
            .map(|float_literal| float_literal.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ForLoop`] into the store.
    ///
    pub fn inter_for_loop(&mut self, for_loop: Arc<Mutex<ForLoop>>) {
        let read = for_loop.lock();
        self.for_loop
            .lock()
            .insert(read.id, (for_loop.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ForLoop`] from the store.
    ///
    pub fn exhume_for_loop(&self, id: &Uuid) -> Option<Arc<Mutex<ForLoop>>> {
        self.for_loop
            .lock()
            .get(id)
            .map(|for_loop| for_loop.0.clone())
    }

    /// Exorcise (remove) [`ForLoop`] from the store.
    ///
    pub fn exorcise_for_loop(&mut self, id: &Uuid) -> Option<Arc<Mutex<ForLoop>>> {
        self.for_loop
            .lock()
            .remove(id)
            .map(|for_loop| for_loop.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ForLoop>`.
    ///
    pub fn iter_for_loop(&self) -> impl Iterator<Item = Arc<Mutex<ForLoop>>> + '_ {
        let values: Vec<Arc<Mutex<ForLoop>>> = self
            .for_loop
            .lock()
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
            .lock()
            .get(&for_loop.id)
            .map(|for_loop| for_loop.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Function`] into the store.
    ///
    pub fn inter_function(&mut self, function: Arc<Mutex<Function>>) {
        let read = function.lock();
        let value = (function.clone(), SystemTime::now());
        self.function_id_by_name
            .lock()
            .insert(read.name.to_upper_camel_case(), (read.id, value.1));
        self.function.lock().insert(read.id, value);
    }

    /// Exhume (get) [`Function`] from the store.
    ///
    pub fn exhume_function(&self, id: &Uuid) -> Option<Arc<Mutex<Function>>> {
        self.function
            .lock()
            .get(id)
            .map(|function| function.0.clone())
    }

    /// Exorcise (remove) [`Function`] from the store.
    ///
    pub fn exorcise_function(&mut self, id: &Uuid) -> Option<Arc<Mutex<Function>>> {
        self.function
            .lock()
            .remove(id)
            .map(|function| function.0.clone())
    }

    /// Exhume [`Function`] id from the store by name.
    ///
    pub fn exhume_function_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.function_id_by_name
            .lock()
            .get(name)
            .map(|function| function.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Function>`.
    ///
    pub fn iter_function(&self) -> impl Iterator<Item = Arc<Mutex<Function>>> + '_ {
        let values: Vec<Arc<Mutex<Function>>> = self
            .function
            .lock()
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
            .lock()
            .get(&function.id)
            .map(|function| function.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Grouped`] into the store.
    ///
    pub fn inter_grouped(&mut self, grouped: Arc<Mutex<Grouped>>) {
        let read = grouped.lock();
        self.grouped
            .lock()
            .insert(read.id, (grouped.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Grouped`] from the store.
    ///
    pub fn exhume_grouped(&self, id: &Uuid) -> Option<Arc<Mutex<Grouped>>> {
        self.grouped.lock().get(id).map(|grouped| grouped.0.clone())
    }

    /// Exorcise (remove) [`Grouped`] from the store.
    ///
    pub fn exorcise_grouped(&mut self, id: &Uuid) -> Option<Arc<Mutex<Grouped>>> {
        self.grouped
            .lock()
            .remove(id)
            .map(|grouped| grouped.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Grouped>`.
    ///
    pub fn iter_grouped(&self) -> impl Iterator<Item = Arc<Mutex<Grouped>>> + '_ {
        let values: Vec<Arc<Mutex<Grouped>>> = self
            .grouped
            .lock()
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
            .lock()
            .get(&grouped.id)
            .map(|grouped| grouped.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`XIf`] into the store.
    ///
    pub fn inter_x_if(&mut self, x_if: Arc<Mutex<XIf>>) {
        let read = x_if.lock();
        self.x_if
            .lock()
            .insert(read.id, (x_if.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`XIf`] from the store.
    ///
    pub fn exhume_x_if(&self, id: &Uuid) -> Option<Arc<Mutex<XIf>>> {
        self.x_if.lock().get(id).map(|x_if| x_if.0.clone())
    }

    /// Exorcise (remove) [`XIf`] from the store.
    ///
    pub fn exorcise_x_if(&mut self, id: &Uuid) -> Option<Arc<Mutex<XIf>>> {
        self.x_if.lock().remove(id).map(|x_if| x_if.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XIf>`.
    ///
    pub fn iter_x_if(&self) -> impl Iterator<Item = Arc<Mutex<XIf>>> + '_ {
        let values: Vec<Arc<Mutex<XIf>>> = self
            .x_if
            .lock()
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
            .lock()
            .get(&x_if.id)
            .map(|x_if| x_if.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Implementation`] into the store.
    ///
    pub fn inter_implementation(&mut self, implementation: Arc<Mutex<Implementation>>) {
        let read = implementation.lock();
        self.implementation
            .lock()
            .insert(read.id, (implementation.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Implementation`] from the store.
    ///
    pub fn exhume_implementation(&self, id: &Uuid) -> Option<Arc<Mutex<Implementation>>> {
        self.implementation
            .lock()
            .get(id)
            .map(|implementation| implementation.0.clone())
    }

    /// Exorcise (remove) [`Implementation`] from the store.
    ///
    pub fn exorcise_implementation(&mut self, id: &Uuid) -> Option<Arc<Mutex<Implementation>>> {
        self.implementation
            .lock()
            .remove(id)
            .map(|implementation| implementation.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Implementation>`.
    ///
    pub fn iter_implementation(&self) -> impl Iterator<Item = Arc<Mutex<Implementation>>> + '_ {
        let values: Vec<Arc<Mutex<Implementation>>> = self
            .implementation
            .lock()
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
            .lock()
            .get(&implementation.id)
            .map(|implementation| implementation.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Import`] into the store.
    ///
    pub fn inter_import(&mut self, import: Arc<Mutex<Import>>) {
        let read = import.lock();
        self.import
            .lock()
            .insert(read.id, (import.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Import`] from the store.
    ///
    pub fn exhume_import(&self, id: &Uuid) -> Option<Arc<Mutex<Import>>> {
        self.import.lock().get(id).map(|import| import.0.clone())
    }

    /// Exorcise (remove) [`Import`] from the store.
    ///
    pub fn exorcise_import(&mut self, id: &Uuid) -> Option<Arc<Mutex<Import>>> {
        self.import.lock().remove(id).map(|import| import.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Import>`.
    ///
    pub fn iter_import(&self) -> impl Iterator<Item = Arc<Mutex<Import>>> + '_ {
        let values: Vec<Arc<Mutex<Import>>> = self
            .import
            .lock()
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
            .lock()
            .get(&import.id)
            .map(|import| import.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Index`] into the store.
    ///
    pub fn inter_index(&mut self, index: Arc<Mutex<Index>>) {
        let read = index.lock();
        self.index
            .lock()
            .insert(read.id, (index.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Index`] from the store.
    ///
    pub fn exhume_index(&self, id: &Uuid) -> Option<Arc<Mutex<Index>>> {
        self.index.lock().get(id).map(|index| index.0.clone())
    }

    /// Exorcise (remove) [`Index`] from the store.
    ///
    pub fn exorcise_index(&mut self, id: &Uuid) -> Option<Arc<Mutex<Index>>> {
        self.index.lock().remove(id).map(|index| index.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Index>`.
    ///
    pub fn iter_index(&self) -> impl Iterator<Item = Arc<Mutex<Index>>> + '_ {
        let values: Vec<Arc<Mutex<Index>>> = self
            .index
            .lock()
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
            .lock()
            .get(&index.id)
            .map(|index| index.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`IntegerLiteral`] into the store.
    ///
    pub fn inter_integer_literal(&mut self, integer_literal: Arc<Mutex<IntegerLiteral>>) {
        let read = integer_literal.lock();
        self.integer_literal
            .lock()
            .insert(read.id, (integer_literal.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`IntegerLiteral`] from the store.
    ///
    pub fn exhume_integer_literal(&self, id: &Uuid) -> Option<Arc<Mutex<IntegerLiteral>>> {
        self.integer_literal
            .lock()
            .get(id)
            .map(|integer_literal| integer_literal.0.clone())
    }

    /// Exorcise (remove) [`IntegerLiteral`] from the store.
    ///
    pub fn exorcise_integer_literal(&mut self, id: &Uuid) -> Option<Arc<Mutex<IntegerLiteral>>> {
        self.integer_literal
            .lock()
            .remove(id)
            .map(|integer_literal| integer_literal.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, IntegerLiteral>`.
    ///
    pub fn iter_integer_literal(&self) -> impl Iterator<Item = Arc<Mutex<IntegerLiteral>>> + '_ {
        let values: Vec<Arc<Mutex<IntegerLiteral>>> = self
            .integer_literal
            .lock()
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
            .lock()
            .get(&integer_literal.id)
            .map(|integer_literal| integer_literal.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Item`] into the store.
    ///
    pub fn inter_item(&mut self, item: Arc<Mutex<Item>>) {
        let read = item.lock();
        self.item
            .lock()
            .insert(read.id, (item.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Item`] from the store.
    ///
    pub fn exhume_item(&self, id: &Uuid) -> Option<Arc<Mutex<Item>>> {
        self.item.lock().get(id).map(|item| item.0.clone())
    }

    /// Exorcise (remove) [`Item`] from the store.
    ///
    pub fn exorcise_item(&mut self, id: &Uuid) -> Option<Arc<Mutex<Item>>> {
        self.item.lock().remove(id).map(|item| item.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Item>`.
    ///
    pub fn iter_item(&self) -> impl Iterator<Item = Arc<Mutex<Item>>> + '_ {
        let values: Vec<Arc<Mutex<Item>>> = self
            .item
            .lock()
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
            .lock()
            .get(&item.id)
            .map(|item| item.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`LetStatement`] into the store.
    ///
    pub fn inter_let_statement(&mut self, let_statement: Arc<Mutex<LetStatement>>) {
        let read = let_statement.lock();
        self.let_statement
            .lock()
            .insert(read.id, (let_statement.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`LetStatement`] from the store.
    ///
    pub fn exhume_let_statement(&self, id: &Uuid) -> Option<Arc<Mutex<LetStatement>>> {
        self.let_statement
            .lock()
            .get(id)
            .map(|let_statement| let_statement.0.clone())
    }

    /// Exorcise (remove) [`LetStatement`] from the store.
    ///
    pub fn exorcise_let_statement(&mut self, id: &Uuid) -> Option<Arc<Mutex<LetStatement>>> {
        self.let_statement
            .lock()
            .remove(id)
            .map(|let_statement| let_statement.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LetStatement>`.
    ///
    pub fn iter_let_statement(&self) -> impl Iterator<Item = Arc<Mutex<LetStatement>>> + '_ {
        let values: Vec<Arc<Mutex<LetStatement>>> = self
            .let_statement
            .lock()
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
            .lock()
            .get(&let_statement.id)
            .map(|let_statement| let_statement.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`List`] into the store.
    ///
    pub fn inter_list(&mut self, list: Arc<Mutex<List>>) {
        let read = list.lock();
        self.list
            .lock()
            .insert(read.id, (list.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`List`] from the store.
    ///
    pub fn exhume_list(&self, id: &Uuid) -> Option<Arc<Mutex<List>>> {
        self.list.lock().get(id).map(|list| list.0.clone())
    }

    /// Exorcise (remove) [`List`] from the store.
    ///
    pub fn exorcise_list(&mut self, id: &Uuid) -> Option<Arc<Mutex<List>>> {
        self.list.lock().remove(id).map(|list| list.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, List>`.
    ///
    pub fn iter_list(&self) -> impl Iterator<Item = Arc<Mutex<List>>> + '_ {
        let values: Vec<Arc<Mutex<List>>> = self
            .list
            .lock()
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
            .lock()
            .get(&list.id)
            .map(|list| list.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ListElement`] into the store.
    ///
    pub fn inter_list_element(&mut self, list_element: Arc<Mutex<ListElement>>) {
        let read = list_element.lock();
        self.list_element
            .lock()
            .insert(read.id, (list_element.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ListElement`] from the store.
    ///
    pub fn exhume_list_element(&self, id: &Uuid) -> Option<Arc<Mutex<ListElement>>> {
        self.list_element
            .lock()
            .get(id)
            .map(|list_element| list_element.0.clone())
    }

    /// Exorcise (remove) [`ListElement`] from the store.
    ///
    pub fn exorcise_list_element(&mut self, id: &Uuid) -> Option<Arc<Mutex<ListElement>>> {
        self.list_element
            .lock()
            .remove(id)
            .map(|list_element| list_element.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListElement>`.
    ///
    pub fn iter_list_element(&self) -> impl Iterator<Item = Arc<Mutex<ListElement>>> + '_ {
        let values: Vec<Arc<Mutex<ListElement>>> = self
            .list_element
            .lock()
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
            .lock()
            .get(&list_element.id)
            .map(|list_element| list_element.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ListExpression`] into the store.
    ///
    pub fn inter_list_expression(&mut self, list_expression: Arc<Mutex<ListExpression>>) {
        let read = list_expression.lock();
        self.list_expression
            .lock()
            .insert(read.id, (list_expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ListExpression`] from the store.
    ///
    pub fn exhume_list_expression(&self, id: &Uuid) -> Option<Arc<Mutex<ListExpression>>> {
        self.list_expression
            .lock()
            .get(id)
            .map(|list_expression| list_expression.0.clone())
    }

    /// Exorcise (remove) [`ListExpression`] from the store.
    ///
    pub fn exorcise_list_expression(&mut self, id: &Uuid) -> Option<Arc<Mutex<ListExpression>>> {
        self.list_expression
            .lock()
            .remove(id)
            .map(|list_expression| list_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListExpression>`.
    ///
    pub fn iter_list_expression(&self) -> impl Iterator<Item = Arc<Mutex<ListExpression>>> + '_ {
        let values: Vec<Arc<Mutex<ListExpression>>> = self
            .list_expression
            .lock()
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
            .lock()
            .get(&list_expression.id)
            .map(|list_expression| list_expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Literal`] into the store.
    ///
    pub fn inter_literal(&mut self, literal: Arc<Mutex<Literal>>) {
        let read = literal.lock();
        self.literal
            .lock()
            .insert(read.id(), (literal.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Literal`] from the store.
    ///
    pub fn exhume_literal(&self, id: &Uuid) -> Option<Arc<Mutex<Literal>>> {
        self.literal.lock().get(id).map(|literal| literal.0.clone())
    }

    /// Exorcise (remove) [`Literal`] from the store.
    ///
    pub fn exorcise_literal(&mut self, id: &Uuid) -> Option<Arc<Mutex<Literal>>> {
        self.literal
            .lock()
            .remove(id)
            .map(|literal| literal.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Literal>`.
    ///
    pub fn iter_literal(&self) -> impl Iterator<Item = Arc<Mutex<Literal>>> + '_ {
        let values: Vec<Arc<Mutex<Literal>>> = self
            .literal
            .lock()
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
            .lock()
            .get(&literal.id())
            .map(|literal| literal.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`LocalVariable`] into the store.
    ///
    pub fn inter_local_variable(&mut self, local_variable: Arc<Mutex<LocalVariable>>) {
        let read = local_variable.lock();
        self.local_variable
            .lock()
            .insert(read.id, (local_variable.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`LocalVariable`] from the store.
    ///
    pub fn exhume_local_variable(&self, id: &Uuid) -> Option<Arc<Mutex<LocalVariable>>> {
        self.local_variable
            .lock()
            .get(id)
            .map(|local_variable| local_variable.0.clone())
    }

    /// Exorcise (remove) [`LocalVariable`] from the store.
    ///
    pub fn exorcise_local_variable(&mut self, id: &Uuid) -> Option<Arc<Mutex<LocalVariable>>> {
        self.local_variable
            .lock()
            .remove(id)
            .map(|local_variable| local_variable.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LocalVariable>`.
    ///
    pub fn iter_local_variable(&self) -> impl Iterator<Item = Arc<Mutex<LocalVariable>>> + '_ {
        let values: Vec<Arc<Mutex<LocalVariable>>> = self
            .local_variable
            .lock()
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
            .lock()
            .get(&local_variable.id)
            .map(|local_variable| local_variable.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`MethodCall`] into the store.
    ///
    pub fn inter_method_call(&mut self, method_call: Arc<Mutex<MethodCall>>) {
        let read = method_call.lock();
        self.method_call
            .lock()
            .insert(read.id, (method_call.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`MethodCall`] from the store.
    ///
    pub fn exhume_method_call(&self, id: &Uuid) -> Option<Arc<Mutex<MethodCall>>> {
        self.method_call
            .lock()
            .get(id)
            .map(|method_call| method_call.0.clone())
    }

    /// Exorcise (remove) [`MethodCall`] from the store.
    ///
    pub fn exorcise_method_call(&mut self, id: &Uuid) -> Option<Arc<Mutex<MethodCall>>> {
        self.method_call
            .lock()
            .remove(id)
            .map(|method_call| method_call.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, MethodCall>`.
    ///
    pub fn iter_method_call(&self) -> impl Iterator<Item = Arc<Mutex<MethodCall>>> + '_ {
        let values: Vec<Arc<Mutex<MethodCall>>> = self
            .method_call
            .lock()
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
            .lock()
            .get(&method_call.id)
            .map(|method_call| method_call.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Negation`] into the store.
    ///
    pub fn inter_negation(&mut self, negation: Arc<Mutex<Negation>>) {
        let read = negation.lock();
        self.negation
            .lock()
            .insert(read.id, (negation.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Negation`] from the store.
    ///
    pub fn exhume_negation(&self, id: &Uuid) -> Option<Arc<Mutex<Negation>>> {
        self.negation
            .lock()
            .get(id)
            .map(|negation| negation.0.clone())
    }

    /// Exorcise (remove) [`Negation`] from the store.
    ///
    pub fn exorcise_negation(&mut self, id: &Uuid) -> Option<Arc<Mutex<Negation>>> {
        self.negation
            .lock()
            .remove(id)
            .map(|negation| negation.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Negation>`.
    ///
    pub fn iter_negation(&self) -> impl Iterator<Item = Arc<Mutex<Negation>>> + '_ {
        let values: Vec<Arc<Mutex<Negation>>> = self
            .negation
            .lock()
            .values()
            .map(|negation| negation.0.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Get the timestamp for Negation.
    ///
    pub fn negation_timestamp(&self, negation: &Negation) -> SystemTime {
        self.negation
            .lock()
            .get(&negation.id)
            .map(|negation| negation.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ZObjectStore`] into the store.
    ///
    pub fn inter_z_object_store(&mut self, z_object_store: Arc<Mutex<ZObjectStore>>) {
        let read = z_object_store.lock();
        self.z_object_store
            .lock()
            .insert(read.id, (z_object_store.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ZObjectStore`] from the store.
    ///
    pub fn exhume_z_object_store(&self, id: &Uuid) -> Option<Arc<Mutex<ZObjectStore>>> {
        self.z_object_store
            .lock()
            .get(id)
            .map(|z_object_store| z_object_store.0.clone())
    }

    /// Exorcise (remove) [`ZObjectStore`] from the store.
    ///
    pub fn exorcise_z_object_store(&mut self, id: &Uuid) -> Option<Arc<Mutex<ZObjectStore>>> {
        self.z_object_store
            .lock()
            .remove(id)
            .map(|z_object_store| z_object_store.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ZObjectStore>`.
    ///
    pub fn iter_z_object_store(&self) -> impl Iterator<Item = Arc<Mutex<ZObjectStore>>> + '_ {
        let values: Vec<Arc<Mutex<ZObjectStore>>> = self
            .z_object_store
            .lock()
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
            .lock()
            .get(&z_object_store.id)
            .map(|z_object_store| z_object_store.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Operator`] into the store.
    ///
    pub fn inter_operator(&mut self, operator: Arc<Mutex<Operator>>) {
        let read = operator.lock();
        self.operator
            .lock()
            .insert(read.id, (operator.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Operator`] from the store.
    ///
    pub fn exhume_operator(&self, id: &Uuid) -> Option<Arc<Mutex<Operator>>> {
        self.operator
            .lock()
            .get(id)
            .map(|operator| operator.0.clone())
    }

    /// Exorcise (remove) [`Operator`] from the store.
    ///
    pub fn exorcise_operator(&mut self, id: &Uuid) -> Option<Arc<Mutex<Operator>>> {
        self.operator
            .lock()
            .remove(id)
            .map(|operator| operator.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Operator>`.
    ///
    pub fn iter_operator(&self) -> impl Iterator<Item = Arc<Mutex<Operator>>> + '_ {
        let values: Vec<Arc<Mutex<Operator>>> = self
            .operator
            .lock()
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
            .lock()
            .get(&operator.id)
            .map(|operator| operator.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`WoogOption`] into the store.
    ///
    pub fn inter_woog_option(&mut self, woog_option: Arc<Mutex<WoogOption>>) {
        let read = woog_option.lock();
        self.woog_option
            .lock()
            .insert(read.id, (woog_option.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`WoogOption`] from the store.
    ///
    pub fn exhume_woog_option(&self, id: &Uuid) -> Option<Arc<Mutex<WoogOption>>> {
        self.woog_option
            .lock()
            .get(id)
            .map(|woog_option| woog_option.0.clone())
    }

    /// Exorcise (remove) [`WoogOption`] from the store.
    ///
    pub fn exorcise_woog_option(&mut self, id: &Uuid) -> Option<Arc<Mutex<WoogOption>>> {
        self.woog_option
            .lock()
            .remove(id)
            .map(|woog_option| woog_option.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, WoogOption>`.
    ///
    pub fn iter_woog_option(&self) -> impl Iterator<Item = Arc<Mutex<WoogOption>>> + '_ {
        let values: Vec<Arc<Mutex<WoogOption>>> = self
            .woog_option
            .lock()
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
            .lock()
            .get(&woog_option.id)
            .map(|woog_option| woog_option.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Parameter`] into the store.
    ///
    pub fn inter_parameter(&mut self, parameter: Arc<Mutex<Parameter>>) {
        let read = parameter.lock();
        self.parameter
            .lock()
            .insert(read.id, (parameter.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Parameter`] from the store.
    ///
    pub fn exhume_parameter(&self, id: &Uuid) -> Option<Arc<Mutex<Parameter>>> {
        self.parameter
            .lock()
            .get(id)
            .map(|parameter| parameter.0.clone())
    }

    /// Exorcise (remove) [`Parameter`] from the store.
    ///
    pub fn exorcise_parameter(&mut self, id: &Uuid) -> Option<Arc<Mutex<Parameter>>> {
        self.parameter
            .lock()
            .remove(id)
            .map(|parameter| parameter.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Parameter>`.
    ///
    pub fn iter_parameter(&self) -> impl Iterator<Item = Arc<Mutex<Parameter>>> + '_ {
        let values: Vec<Arc<Mutex<Parameter>>> = self
            .parameter
            .lock()
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
            .lock()
            .get(&parameter.id)
            .map(|parameter| parameter.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Print`] into the store.
    ///
    pub fn inter_print(&mut self, print: Arc<Mutex<Print>>) {
        let read = print.lock();
        self.print
            .lock()
            .insert(read.id, (print.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Print`] from the store.
    ///
    pub fn exhume_print(&self, id: &Uuid) -> Option<Arc<Mutex<Print>>> {
        self.print.lock().get(id).map(|print| print.0.clone())
    }

    /// Exorcise (remove) [`Print`] from the store.
    ///
    pub fn exorcise_print(&mut self, id: &Uuid) -> Option<Arc<Mutex<Print>>> {
        self.print.lock().remove(id).map(|print| print.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Print>`.
    ///
    pub fn iter_print(&self) -> impl Iterator<Item = Arc<Mutex<Print>>> + '_ {
        let values: Vec<Arc<Mutex<Print>>> = self
            .print
            .lock()
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
            .lock()
            .get(&print.id)
            .map(|print| print.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`RangeExpression`] into the store.
    ///
    pub fn inter_range_expression(&mut self, range_expression: Arc<Mutex<RangeExpression>>) {
        let read = range_expression.lock();
        self.range_expression
            .lock()
            .insert(read.id, (range_expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`RangeExpression`] from the store.
    ///
    pub fn exhume_range_expression(&self, id: &Uuid) -> Option<Arc<Mutex<RangeExpression>>> {
        self.range_expression
            .lock()
            .get(id)
            .map(|range_expression| range_expression.0.clone())
    }

    /// Exorcise (remove) [`RangeExpression`] from the store.
    ///
    pub fn exorcise_range_expression(&mut self, id: &Uuid) -> Option<Arc<Mutex<RangeExpression>>> {
        self.range_expression
            .lock()
            .remove(id)
            .map(|range_expression| range_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RangeExpression>`.
    ///
    pub fn iter_range_expression(&self) -> impl Iterator<Item = Arc<Mutex<RangeExpression>>> + '_ {
        let values: Vec<Arc<Mutex<RangeExpression>>> = self
            .range_expression
            .lock()
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
            .lock()
            .get(&range_expression.id)
            .map(|range_expression| range_expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Reference`] into the store.
    ///
    pub fn inter_reference(&mut self, reference: Arc<Mutex<Reference>>) {
        let read = reference.lock();
        self.reference
            .lock()
            .insert(read.id, (reference.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Reference`] from the store.
    ///
    pub fn exhume_reference(&self, id: &Uuid) -> Option<Arc<Mutex<Reference>>> {
        self.reference
            .lock()
            .get(id)
            .map(|reference| reference.0.clone())
    }

    /// Exorcise (remove) [`Reference`] from the store.
    ///
    pub fn exorcise_reference(&mut self, id: &Uuid) -> Option<Arc<Mutex<Reference>>> {
        self.reference
            .lock()
            .remove(id)
            .map(|reference| reference.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Reference>`.
    ///
    pub fn iter_reference(&self) -> impl Iterator<Item = Arc<Mutex<Reference>>> + '_ {
        let values: Vec<Arc<Mutex<Reference>>> = self
            .reference
            .lock()
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
            .lock()
            .get(&reference.id)
            .map(|reference| reference.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ResultStatement`] into the store.
    ///
    pub fn inter_result_statement(&mut self, result_statement: Arc<Mutex<ResultStatement>>) {
        let read = result_statement.lock();
        self.result_statement
            .lock()
            .insert(read.id, (result_statement.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ResultStatement`] from the store.
    ///
    pub fn exhume_result_statement(&self, id: &Uuid) -> Option<Arc<Mutex<ResultStatement>>> {
        self.result_statement
            .lock()
            .get(id)
            .map(|result_statement| result_statement.0.clone())
    }

    /// Exorcise (remove) [`ResultStatement`] from the store.
    ///
    pub fn exorcise_result_statement(&mut self, id: &Uuid) -> Option<Arc<Mutex<ResultStatement>>> {
        self.result_statement
            .lock()
            .remove(id)
            .map(|result_statement| result_statement.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ResultStatement>`.
    ///
    pub fn iter_result_statement(&self) -> impl Iterator<Item = Arc<Mutex<ResultStatement>>> + '_ {
        let values: Vec<Arc<Mutex<ResultStatement>>> = self
            .result_statement
            .lock()
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
            .lock()
            .get(&result_statement.id)
            .map(|result_statement| result_statement.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`XReturn`] into the store.
    ///
    pub fn inter_x_return(&mut self, x_return: Arc<Mutex<XReturn>>) {
        let read = x_return.lock();
        self.x_return
            .lock()
            .insert(read.id, (x_return.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`XReturn`] from the store.
    ///
    pub fn exhume_x_return(&self, id: &Uuid) -> Option<Arc<Mutex<XReturn>>> {
        self.x_return
            .lock()
            .get(id)
            .map(|x_return| x_return.0.clone())
    }

    /// Exorcise (remove) [`XReturn`] from the store.
    ///
    pub fn exorcise_x_return(&mut self, id: &Uuid) -> Option<Arc<Mutex<XReturn>>> {
        self.x_return
            .lock()
            .remove(id)
            .map(|x_return| x_return.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XReturn>`.
    ///
    pub fn iter_x_return(&self) -> impl Iterator<Item = Arc<Mutex<XReturn>>> + '_ {
        let values: Vec<Arc<Mutex<XReturn>>> = self
            .x_return
            .lock()
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
            .lock()
            .get(&x_return.id)
            .map(|x_return| x_return.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ZSome`] into the store.
    ///
    pub fn inter_z_some(&mut self, z_some: Arc<Mutex<ZSome>>) {
        let read = z_some.lock();
        self.z_some
            .lock()
            .insert(read.id, (z_some.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ZSome`] from the store.
    ///
    pub fn exhume_z_some(&self, id: &Uuid) -> Option<Arc<Mutex<ZSome>>> {
        self.z_some.lock().get(id).map(|z_some| z_some.0.clone())
    }

    /// Exorcise (remove) [`ZSome`] from the store.
    ///
    pub fn exorcise_z_some(&mut self, id: &Uuid) -> Option<Arc<Mutex<ZSome>>> {
        self.z_some.lock().remove(id).map(|z_some| z_some.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ZSome>`.
    ///
    pub fn iter_z_some(&self) -> impl Iterator<Item = Arc<Mutex<ZSome>>> + '_ {
        let values: Vec<Arc<Mutex<ZSome>>> = self
            .z_some
            .lock()
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
            .lock()
            .get(&z_some.id)
            .map(|z_some| z_some.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Span`] into the store.
    ///
    pub fn inter_span(&mut self, span: Arc<Mutex<Span>>) {
        let read = span.lock();
        self.span
            .lock()
            .insert(read.id, (span.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Span`] from the store.
    ///
    pub fn exhume_span(&self, id: &Uuid) -> Option<Arc<Mutex<Span>>> {
        self.span.lock().get(id).map(|span| span.0.clone())
    }

    /// Exorcise (remove) [`Span`] from the store.
    ///
    pub fn exorcise_span(&mut self, id: &Uuid) -> Option<Arc<Mutex<Span>>> {
        self.span.lock().remove(id).map(|span| span.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Span>`.
    ///
    pub fn iter_span(&self) -> impl Iterator<Item = Arc<Mutex<Span>>> + '_ {
        let values: Vec<Arc<Mutex<Span>>> = self
            .span
            .lock()
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
            .lock()
            .get(&span.id)
            .map(|span| span.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Statement`] into the store.
    ///
    pub fn inter_statement(&mut self, statement: Arc<Mutex<Statement>>) {
        let read = statement.lock();
        self.statement
            .lock()
            .insert(read.id, (statement.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Statement`] from the store.
    ///
    pub fn exhume_statement(&self, id: &Uuid) -> Option<Arc<Mutex<Statement>>> {
        self.statement
            .lock()
            .get(id)
            .map(|statement| statement.0.clone())
    }

    /// Exorcise (remove) [`Statement`] from the store.
    ///
    pub fn exorcise_statement(&mut self, id: &Uuid) -> Option<Arc<Mutex<Statement>>> {
        self.statement
            .lock()
            .remove(id)
            .map(|statement| statement.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Statement>`.
    ///
    pub fn iter_statement(&self) -> impl Iterator<Item = Arc<Mutex<Statement>>> + '_ {
        let values: Vec<Arc<Mutex<Statement>>> = self
            .statement
            .lock()
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
            .lock()
            .get(&statement.id)
            .map(|statement| statement.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`StaticMethodCall`] into the store.
    ///
    pub fn inter_static_method_call(&mut self, static_method_call: Arc<Mutex<StaticMethodCall>>) {
        let read = static_method_call.lock();
        self.static_method_call
            .lock()
            .insert(read.id, (static_method_call.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`StaticMethodCall`] from the store.
    ///
    pub fn exhume_static_method_call(&self, id: &Uuid) -> Option<Arc<Mutex<StaticMethodCall>>> {
        self.static_method_call
            .lock()
            .get(id)
            .map(|static_method_call| static_method_call.0.clone())
    }

    /// Exorcise (remove) [`StaticMethodCall`] from the store.
    ///
    pub fn exorcise_static_method_call(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<Mutex<StaticMethodCall>>> {
        self.static_method_call
            .lock()
            .remove(id)
            .map(|static_method_call| static_method_call.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StaticMethodCall>`.
    ///
    pub fn iter_static_method_call(
        &self,
    ) -> impl Iterator<Item = Arc<Mutex<StaticMethodCall>>> + '_ {
        let values: Vec<Arc<Mutex<StaticMethodCall>>> = self
            .static_method_call
            .lock()
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
            .lock()
            .get(&static_method_call.id)
            .map(|static_method_call| static_method_call.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`StringLiteral`] into the store.
    ///
    pub fn inter_string_literal(&mut self, string_literal: Arc<Mutex<StringLiteral>>) {
        let read = string_literal.lock();
        self.string_literal
            .lock()
            .insert(read.id, (string_literal.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`StringLiteral`] from the store.
    ///
    pub fn exhume_string_literal(&self, id: &Uuid) -> Option<Arc<Mutex<StringLiteral>>> {
        self.string_literal
            .lock()
            .get(id)
            .map(|string_literal| string_literal.0.clone())
    }

    /// Exorcise (remove) [`StringLiteral`] from the store.
    ///
    pub fn exorcise_string_literal(&mut self, id: &Uuid) -> Option<Arc<Mutex<StringLiteral>>> {
        self.string_literal
            .lock()
            .remove(id)
            .map(|string_literal| string_literal.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StringLiteral>`.
    ///
    pub fn iter_string_literal(&self) -> impl Iterator<Item = Arc<Mutex<StringLiteral>>> + '_ {
        let values: Vec<Arc<Mutex<StringLiteral>>> = self
            .string_literal
            .lock()
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
            .lock()
            .get(&string_literal.id)
            .map(|string_literal| string_literal.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`WoogStruct`] into the store.
    ///
    pub fn inter_woog_struct(&mut self, woog_struct: Arc<Mutex<WoogStruct>>) {
        let read = woog_struct.lock();
        let value = (woog_struct.clone(), SystemTime::now());
        self.woog_struct_id_by_name
            .lock()
            .insert(read.name.to_upper_camel_case(), (read.id, value.1));
        self.woog_struct.lock().insert(read.id, value);
    }

    /// Exhume (get) [`WoogStruct`] from the store.
    ///
    pub fn exhume_woog_struct(&self, id: &Uuid) -> Option<Arc<Mutex<WoogStruct>>> {
        self.woog_struct
            .lock()
            .get(id)
            .map(|woog_struct| woog_struct.0.clone())
    }

    /// Exorcise (remove) [`WoogStruct`] from the store.
    ///
    pub fn exorcise_woog_struct(&mut self, id: &Uuid) -> Option<Arc<Mutex<WoogStruct>>> {
        self.woog_struct
            .lock()
            .remove(id)
            .map(|woog_struct| woog_struct.0.clone())
    }

    /// Exhume [`WoogStruct`] id from the store by name.
    ///
    pub fn exhume_woog_struct_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.woog_struct_id_by_name
            .lock()
            .get(name)
            .map(|woog_struct| woog_struct.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, WoogStruct>`.
    ///
    pub fn iter_woog_struct(&self) -> impl Iterator<Item = Arc<Mutex<WoogStruct>>> + '_ {
        let values: Vec<Arc<Mutex<WoogStruct>>> = self
            .woog_struct
            .lock()
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
            .lock()
            .get(&woog_struct.id)
            .map(|woog_struct| woog_struct.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`StructExpression`] into the store.
    ///
    pub fn inter_struct_expression(&mut self, struct_expression: Arc<Mutex<StructExpression>>) {
        let read = struct_expression.lock();
        self.struct_expression
            .lock()
            .insert(read.id, (struct_expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`StructExpression`] from the store.
    ///
    pub fn exhume_struct_expression(&self, id: &Uuid) -> Option<Arc<Mutex<StructExpression>>> {
        self.struct_expression
            .lock()
            .get(id)
            .map(|struct_expression| struct_expression.0.clone())
    }

    /// Exorcise (remove) [`StructExpression`] from the store.
    ///
    pub fn exorcise_struct_expression(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<Mutex<StructExpression>>> {
        self.struct_expression
            .lock()
            .remove(id)
            .map(|struct_expression| struct_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructExpression>`.
    ///
    pub fn iter_struct_expression(
        &self,
    ) -> impl Iterator<Item = Arc<Mutex<StructExpression>>> + '_ {
        let values: Vec<Arc<Mutex<StructExpression>>> = self
            .struct_expression
            .lock()
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
            .lock()
            .get(&struct_expression.id)
            .map(|struct_expression| struct_expression.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`TypeCast`] into the store.
    ///
    pub fn inter_type_cast(&mut self, type_cast: Arc<Mutex<TypeCast>>) {
        let read = type_cast.lock();
        self.type_cast
            .lock()
            .insert(read.id, (type_cast.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`TypeCast`] from the store.
    ///
    pub fn exhume_type_cast(&self, id: &Uuid) -> Option<Arc<Mutex<TypeCast>>> {
        self.type_cast
            .lock()
            .get(id)
            .map(|type_cast| type_cast.0.clone())
    }

    /// Exorcise (remove) [`TypeCast`] from the store.
    ///
    pub fn exorcise_type_cast(&mut self, id: &Uuid) -> Option<Arc<Mutex<TypeCast>>> {
        self.type_cast
            .lock()
            .remove(id)
            .map(|type_cast| type_cast.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, TypeCast>`.
    ///
    pub fn iter_type_cast(&self) -> impl Iterator<Item = Arc<Mutex<TypeCast>>> + '_ {
        let values: Vec<Arc<Mutex<TypeCast>>> = self
            .type_cast
            .lock()
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
            .lock()
            .get(&type_cast.id)
            .map(|type_cast| type_cast.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`XValue`] into the store.
    ///
    pub fn inter_x_value(&mut self, x_value: Arc<Mutex<XValue>>) {
        let read = x_value.lock();
        self.x_value
            .lock()
            .insert(read.id, (x_value.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`XValue`] from the store.
    ///
    pub fn exhume_x_value(&self, id: &Uuid) -> Option<Arc<Mutex<XValue>>> {
        self.x_value.lock().get(id).map(|x_value| x_value.0.clone())
    }

    /// Exorcise (remove) [`XValue`] from the store.
    ///
    pub fn exorcise_x_value(&mut self, id: &Uuid) -> Option<Arc<Mutex<XValue>>> {
        self.x_value
            .lock()
            .remove(id)
            .map(|x_value| x_value.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XValue>`.
    ///
    pub fn iter_x_value(&self) -> impl Iterator<Item = Arc<Mutex<XValue>>> + '_ {
        let values: Vec<Arc<Mutex<XValue>>> = self
            .x_value
            .lock()
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
            .lock()
            .get(&x_value.id)
            .map(|x_value| x_value.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`ValueType`] into the store.
    ///
    pub fn inter_value_type(&mut self, value_type: Arc<Mutex<ValueType>>) {
        let read = value_type.lock();
        self.value_type
            .lock()
            .insert(read.id(), (value_type.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`ValueType`] from the store.
    ///
    pub fn exhume_value_type(&self, id: &Uuid) -> Option<Arc<Mutex<ValueType>>> {
        self.value_type
            .lock()
            .get(id)
            .map(|value_type| value_type.0.clone())
    }

    /// Exorcise (remove) [`ValueType`] from the store.
    ///
    pub fn exorcise_value_type(&mut self, id: &Uuid) -> Option<Arc<Mutex<ValueType>>> {
        self.value_type
            .lock()
            .remove(id)
            .map(|value_type| value_type.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ValueType>`.
    ///
    pub fn iter_value_type(&self) -> impl Iterator<Item = Arc<Mutex<ValueType>>> + '_ {
        let values: Vec<Arc<Mutex<ValueType>>> = self
            .value_type
            .lock()
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
            .lock()
            .get(&value_type.id())
            .map(|value_type| value_type.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Variable`] into the store.
    ///
    pub fn inter_variable(&mut self, variable: Arc<Mutex<Variable>>) {
        let read = variable.lock();
        self.variable
            .lock()
            .insert(read.id, (variable.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`Variable`] from the store.
    ///
    pub fn exhume_variable(&self, id: &Uuid) -> Option<Arc<Mutex<Variable>>> {
        self.variable
            .lock()
            .get(id)
            .map(|variable| variable.0.clone())
    }

    /// Exorcise (remove) [`Variable`] from the store.
    ///
    pub fn exorcise_variable(&mut self, id: &Uuid) -> Option<Arc<Mutex<Variable>>> {
        self.variable
            .lock()
            .remove(id)
            .map(|variable| variable.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Variable>`.
    ///
    pub fn iter_variable(&self) -> impl Iterator<Item = Arc<Mutex<Variable>>> + '_ {
        let values: Vec<Arc<Mutex<Variable>>> = self
            .variable
            .lock()
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
            .lock()
            .get(&variable.id)
            .map(|variable| variable.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`VariableExpression`] into the store.
    ///
    pub fn inter_variable_expression(
        &mut self,
        variable_expression: Arc<Mutex<VariableExpression>>,
    ) {
        let read = variable_expression.lock();
        self.variable_expression
            .lock()
            .insert(read.id, (variable_expression.clone(), SystemTime::now()));
    }

    /// Exhume (get) [`VariableExpression`] from the store.
    ///
    pub fn exhume_variable_expression(&self, id: &Uuid) -> Option<Arc<Mutex<VariableExpression>>> {
        self.variable_expression
            .lock()
            .get(id)
            .map(|variable_expression| variable_expression.0.clone())
    }

    /// Exorcise (remove) [`VariableExpression`] from the store.
    ///
    pub fn exorcise_variable_expression(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<Mutex<VariableExpression>>> {
        self.variable_expression
            .lock()
            .remove(id)
            .map(|variable_expression| variable_expression.0.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, VariableExpression>`.
    ///
    pub fn iter_variable_expression(
        &self,
    ) -> impl Iterator<Item = Arc<Mutex<VariableExpression>>> + '_ {
        let values: Vec<Arc<Mutex<VariableExpression>>> = self
            .variable_expression
            .lock()
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
            .lock()
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
            for argument_tuple in self.argument.lock().values() {
                let path = path.join(format!("{}.json", argument_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<Argument>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != argument_tuple.0.lock().to_owned() {
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
                    if !self.argument.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Binary.
        {
            let path = path.join("binary");
            fs::create_dir_all(&path)?;
            for binary_tuple in self.binary.lock().values() {
                let path = path.join(format!("{}.json", binary_tuple.0.lock().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<Binary>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != binary_tuple.0.lock().to_owned() {
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
                    if !self.binary.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Block.
        {
            let path = path.join("block");
            fs::create_dir_all(&path)?;
            for block_tuple in self.block.lock().values() {
                let path = path.join(format!("{}.json", block_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<Block>>, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != block_tuple.0.lock().to_owned() {
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
                    if !self.block.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Boolean Literal.
        {
            let path = path.join("boolean_literal");
            fs::create_dir_all(&path)?;
            for boolean_literal_tuple in self.boolean_literal.lock().values() {
                let path = path.join(format!("{}.json", boolean_literal_tuple.0.lock().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<BooleanLiteral>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != boolean_literal_tuple.0.lock().to_owned() {
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
                    if !self.boolean_literal.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Call.
        {
            let path = path.join("call");
            fs::create_dir_all(&path)?;
            for call_tuple in self.call.lock().values() {
                let path = path.join(format!("{}.json", call_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<Call>>, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != call_tuple.0.lock().to_owned() {
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
                    if !self.call.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Comparison.
        {
            let path = path.join("comparison");
            fs::create_dir_all(&path)?;
            for comparison_tuple in self.comparison.lock().values() {
                let path = path.join(format!("{}.json", comparison_tuple.0.lock().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<Comparison>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != comparison_tuple.0.lock().to_owned() {
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
                    if !self.comparison.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Dwarf Source File.
        {
            let path = path.join("dwarf_source_file");
            fs::create_dir_all(&path)?;
            for dwarf_source_file_tuple in self.dwarf_source_file.lock().values() {
                let path = path.join(format!("{}.json", dwarf_source_file_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<DwarfSourceFile>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != dwarf_source_file_tuple.0.lock().to_owned() {
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
                    if !self.dwarf_source_file.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Error.
        {
            let path = path.join("error");
            fs::create_dir_all(&path)?;
            for error_tuple in self.error.lock().values() {
                let path = path.join(format!("{}.json", error_tuple.0.lock().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<Error>>, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != error_tuple.0.lock().to_owned() {
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
                    if !self.error.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Error Expression.
        {
            let path = path.join("error_expression");
            fs::create_dir_all(&path)?;
            for error_expression_tuple in self.error_expression.lock().values() {
                let path = path.join(format!("{}.json", error_expression_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<ErrorExpression>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != error_expression_tuple.0.lock().to_owned() {
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
                    if !self.error_expression.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Expression.
        {
            let path = path.join("expression");
            fs::create_dir_all(&path)?;
            for expression_tuple in self.expression.lock().values() {
                let path = path.join(format!("{}.json", expression_tuple.0.lock().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<Expression>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != expression_tuple.0.lock().to_owned() {
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
                    if !self.expression.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Expression Statement.
        {
            let path = path.join("expression_statement");
            fs::create_dir_all(&path)?;
            for expression_statement_tuple in self.expression_statement.lock().values() {
                let path = path.join(format!("{}.json", expression_statement_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<ExpressionStatement>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != expression_statement_tuple.0.lock().to_owned()
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
                    if !self.expression_statement.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Field.
        {
            let path = path.join("field");
            fs::create_dir_all(&path)?;
            for field_tuple in self.field.lock().values() {
                let path = path.join(format!("{}.json", field_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<Field>>, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != field_tuple.0.lock().to_owned() {
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
                    if !self.field.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Field Access.
        {
            let path = path.join("field_access");
            fs::create_dir_all(&path)?;
            for field_access_tuple in self.field_access.lock().values() {
                let path = path.join(format!("{}.json", field_access_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<FieldAccess>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != field_access_tuple.0.lock().to_owned() {
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
                    if !self.field_access.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Field Access Target.
        {
            let path = path.join("field_access_target");
            fs::create_dir_all(&path)?;
            for field_access_target_tuple in self.field_access_target.lock().values() {
                let path = path.join(format!("{}.json", field_access_target_tuple.0.lock().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<FieldAccessTarget>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != field_access_target_tuple.0.lock().to_owned()
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
                    if !self.field_access_target.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Field Expression.
        {
            let path = path.join("field_expression");
            fs::create_dir_all(&path)?;
            for field_expression_tuple in self.field_expression.lock().values() {
                let path = path.join(format!("{}.json", field_expression_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<FieldExpression>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != field_expression_tuple.0.lock().to_owned() {
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
                    if !self.field_expression.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Float Literal.
        {
            let path = path.join("float_literal");
            fs::create_dir_all(&path)?;
            for float_literal_tuple in self.float_literal.lock().values() {
                let path = path.join(format!("{}.json", float_literal_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<FloatLiteral>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != float_literal_tuple.0.lock().to_owned() {
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
                    if !self.float_literal.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist For Loop.
        {
            let path = path.join("for_loop");
            fs::create_dir_all(&path)?;
            for for_loop_tuple in self.for_loop.lock().values() {
                let path = path.join(format!("{}.json", for_loop_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<ForLoop>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != for_loop_tuple.0.lock().to_owned() {
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
                    if !self.for_loop.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Function.
        {
            let path = path.join("function");
            fs::create_dir_all(&path)?;
            for function_tuple in self.function.lock().values() {
                let path = path.join(format!("{}.json", function_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<Function>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != function_tuple.0.lock().to_owned() {
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
                    if !self.function.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Grouped.
        {
            let path = path.join("grouped");
            fs::create_dir_all(&path)?;
            for grouped_tuple in self.grouped.lock().values() {
                let path = path.join(format!("{}.json", grouped_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<Grouped>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != grouped_tuple.0.lock().to_owned() {
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
                    if !self.grouped.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist If.
        {
            let path = path.join("x_if");
            fs::create_dir_all(&path)?;
            for x_if_tuple in self.x_if.lock().values() {
                let path = path.join(format!("{}.json", x_if_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<XIf>>, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != x_if_tuple.0.lock().to_owned() {
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
                    if !self.x_if.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Implementation.
        {
            let path = path.join("implementation");
            fs::create_dir_all(&path)?;
            for implementation_tuple in self.implementation.lock().values() {
                let path = path.join(format!("{}.json", implementation_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<Implementation>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != implementation_tuple.0.lock().to_owned() {
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
                    if !self.implementation.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Import.
        {
            let path = path.join("import");
            fs::create_dir_all(&path)?;
            for import_tuple in self.import.lock().values() {
                let path = path.join(format!("{}.json", import_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<Import>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != import_tuple.0.lock().to_owned() {
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
                    if !self.import.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Index.
        {
            let path = path.join("index");
            fs::create_dir_all(&path)?;
            for index_tuple in self.index.lock().values() {
                let path = path.join(format!("{}.json", index_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<Index>>, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != index_tuple.0.lock().to_owned() {
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
                    if !self.index.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Integer Literal.
        {
            let path = path.join("integer_literal");
            fs::create_dir_all(&path)?;
            for integer_literal_tuple in self.integer_literal.lock().values() {
                let path = path.join(format!("{}.json", integer_literal_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<IntegerLiteral>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != integer_literal_tuple.0.lock().to_owned() {
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
                    if !self.integer_literal.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Item.
        {
            let path = path.join("item");
            fs::create_dir_all(&path)?;
            for item_tuple in self.item.lock().values() {
                let path = path.join(format!("{}.json", item_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<Item>>, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != item_tuple.0.lock().to_owned() {
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
                    if !self.item.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Let Statement.
        {
            let path = path.join("let_statement");
            fs::create_dir_all(&path)?;
            for let_statement_tuple in self.let_statement.lock().values() {
                let path = path.join(format!("{}.json", let_statement_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<LetStatement>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != let_statement_tuple.0.lock().to_owned() {
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
                    if !self.let_statement.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist List.
        {
            let path = path.join("list");
            fs::create_dir_all(&path)?;
            for list_tuple in self.list.lock().values() {
                let path = path.join(format!("{}.json", list_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<List>>, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != list_tuple.0.lock().to_owned() {
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
                    if !self.list.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist List Element.
        {
            let path = path.join("list_element");
            fs::create_dir_all(&path)?;
            for list_element_tuple in self.list_element.lock().values() {
                let path = path.join(format!("{}.json", list_element_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<ListElement>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != list_element_tuple.0.lock().to_owned() {
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
                    if !self.list_element.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist List Expression.
        {
            let path = path.join("list_expression");
            fs::create_dir_all(&path)?;
            for list_expression_tuple in self.list_expression.lock().values() {
                let path = path.join(format!("{}.json", list_expression_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<ListExpression>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != list_expression_tuple.0.lock().to_owned() {
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
                    if !self.list_expression.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Literal.
        {
            let path = path.join("literal");
            fs::create_dir_all(&path)?;
            for literal_tuple in self.literal.lock().values() {
                let path = path.join(format!("{}.json", literal_tuple.0.lock().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<Literal>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != literal_tuple.0.lock().to_owned() {
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
                    if !self.literal.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Local Variable.
        {
            let path = path.join("local_variable");
            fs::create_dir_all(&path)?;
            for local_variable_tuple in self.local_variable.lock().values() {
                let path = path.join(format!("{}.json", local_variable_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<LocalVariable>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != local_variable_tuple.0.lock().to_owned() {
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
                    if !self.local_variable.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Method Call.
        {
            let path = path.join("method_call");
            fs::create_dir_all(&path)?;
            for method_call_tuple in self.method_call.lock().values() {
                let path = path.join(format!("{}.json", method_call_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<MethodCall>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != method_call_tuple.0.lock().to_owned() {
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
                    if !self.method_call.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Negation.
        {
            let path = path.join("negation");
            fs::create_dir_all(&path)?;
            for negation_tuple in self.negation.lock().values() {
                let path = path.join(format!("{}.json", negation_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<Negation>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != negation_tuple.0.lock().to_owned() {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &negation_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &negation_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.negation.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Object Store.
        {
            let path = path.join("z_object_store");
            fs::create_dir_all(&path)?;
            for z_object_store_tuple in self.z_object_store.lock().values() {
                let path = path.join(format!("{}.json", z_object_store_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<ZObjectStore>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != z_object_store_tuple.0.lock().to_owned() {
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
                    if !self.z_object_store.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Operator.
        {
            let path = path.join("operator");
            fs::create_dir_all(&path)?;
            for operator_tuple in self.operator.lock().values() {
                let path = path.join(format!("{}.json", operator_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<Operator>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != operator_tuple.0.lock().to_owned() {
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
                    if !self.operator.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Option.
        {
            let path = path.join("woog_option");
            fs::create_dir_all(&path)?;
            for woog_option_tuple in self.woog_option.lock().values() {
                let path = path.join(format!("{}.json", woog_option_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<WoogOption>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != woog_option_tuple.0.lock().to_owned() {
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
                    if !self.woog_option.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Parameter.
        {
            let path = path.join("parameter");
            fs::create_dir_all(&path)?;
            for parameter_tuple in self.parameter.lock().values() {
                let path = path.join(format!("{}.json", parameter_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<Parameter>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != parameter_tuple.0.lock().to_owned() {
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
                    if !self.parameter.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Print.
        {
            let path = path.join("print");
            fs::create_dir_all(&path)?;
            for print_tuple in self.print.lock().values() {
                let path = path.join(format!("{}.json", print_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<Print>>, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != print_tuple.0.lock().to_owned() {
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
                    if !self.print.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Range Expression.
        {
            let path = path.join("range_expression");
            fs::create_dir_all(&path)?;
            for range_expression_tuple in self.range_expression.lock().values() {
                let path = path.join(format!("{}.json", range_expression_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<RangeExpression>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != range_expression_tuple.0.lock().to_owned() {
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
                    if !self.range_expression.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Reference.
        {
            let path = path.join("reference");
            fs::create_dir_all(&path)?;
            for reference_tuple in self.reference.lock().values() {
                let path = path.join(format!("{}.json", reference_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<Reference>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != reference_tuple.0.lock().to_owned() {
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
                    if !self.reference.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Result Statement.
        {
            let path = path.join("result_statement");
            fs::create_dir_all(&path)?;
            for result_statement_tuple in self.result_statement.lock().values() {
                let path = path.join(format!("{}.json", result_statement_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<ResultStatement>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != result_statement_tuple.0.lock().to_owned() {
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
                    if !self.result_statement.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Return.
        {
            let path = path.join("x_return");
            fs::create_dir_all(&path)?;
            for x_return_tuple in self.x_return.lock().values() {
                let path = path.join(format!("{}.json", x_return_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<XReturn>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != x_return_tuple.0.lock().to_owned() {
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
                    if !self.x_return.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Some.
        {
            let path = path.join("z_some");
            fs::create_dir_all(&path)?;
            for z_some_tuple in self.z_some.lock().values() {
                let path = path.join(format!("{}.json", z_some_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<ZSome>>, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != z_some_tuple.0.lock().to_owned() {
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
                    if !self.z_some.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Span.
        {
            let path = path.join("span");
            fs::create_dir_all(&path)?;
            for span_tuple in self.span.lock().values() {
                let path = path.join(format!("{}.json", span_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<Span>>, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != span_tuple.0.lock().to_owned() {
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
                    if !self.span.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Statement.
        {
            let path = path.join("statement");
            fs::create_dir_all(&path)?;
            for statement_tuple in self.statement.lock().values() {
                let path = path.join(format!("{}.json", statement_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<Statement>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != statement_tuple.0.lock().to_owned() {
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
                    if !self.statement.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Static Method Call.
        {
            let path = path.join("static_method_call");
            fs::create_dir_all(&path)?;
            for static_method_call_tuple in self.static_method_call.lock().values() {
                let path = path.join(format!("{}.json", static_method_call_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<StaticMethodCall>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != static_method_call_tuple.0.lock().to_owned() {
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
                    if !self.static_method_call.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist String Literal.
        {
            let path = path.join("string_literal");
            fs::create_dir_all(&path)?;
            for string_literal_tuple in self.string_literal.lock().values() {
                let path = path.join(format!("{}.json", string_literal_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<StringLiteral>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != string_literal_tuple.0.lock().to_owned() {
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
                    if !self.string_literal.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Struct.
        {
            let path = path.join("woog_struct");
            fs::create_dir_all(&path)?;
            for woog_struct_tuple in self.woog_struct.lock().values() {
                let path = path.join(format!("{}.json", woog_struct_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<WoogStruct>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != woog_struct_tuple.0.lock().to_owned() {
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
                    if !self.woog_struct.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Struct Expression.
        {
            let path = path.join("struct_expression");
            fs::create_dir_all(&path)?;
            for struct_expression_tuple in self.struct_expression.lock().values() {
                let path = path.join(format!("{}.json", struct_expression_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<StructExpression>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != struct_expression_tuple.0.lock().to_owned() {
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
                    if !self.struct_expression.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Type Cast.
        {
            let path = path.join("type_cast");
            fs::create_dir_all(&path)?;
            for type_cast_tuple in self.type_cast.lock().values() {
                let path = path.join(format!("{}.json", type_cast_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<TypeCast>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != type_cast_tuple.0.lock().to_owned() {
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
                    if !self.type_cast.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Value.
        {
            let path = path.join("x_value");
            fs::create_dir_all(&path)?;
            for x_value_tuple in self.x_value.lock().values() {
                let path = path.join(format!("{}.json", x_value_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<XValue>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != x_value_tuple.0.lock().to_owned() {
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
                    if !self.x_value.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Value Type.
        {
            let path = path.join("value_type");
            fs::create_dir_all(&path)?;
            for value_type_tuple in self.value_type.lock().values() {
                let path = path.join(format!("{}.json", value_type_tuple.0.lock().id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<ValueType>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != value_type_tuple.0.lock().to_owned() {
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
                    if !self.value_type.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Variable.
        {
            let path = path.join("variable");
            fs::create_dir_all(&path)?;
            for variable_tuple in self.variable.lock().values() {
                let path = path.join(format!("{}.json", variable_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<Variable>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != variable_tuple.0.lock().to_owned() {
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
                    if !self.variable.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Variable Expression.
        {
            let path = path.join("variable_expression");
            fs::create_dir_all(&path)?;
            for variable_expression_tuple in self.variable_expression.lock().values() {
                let path = path.join(format!("{}.json", variable_expression_tuple.0.lock().id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Arc<Mutex<VariableExpression>>, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0.lock().to_owned() != variable_expression_tuple.0.lock().to_owned()
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
                    if !self.variable_expression.lock().contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Load the store.
    ///
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
                let argument: (Arc<Mutex<Argument>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .argument
                    .lock()
                    .insert(argument.0.lock().id, argument.clone());
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
                let binary: (Arc<Mutex<Binary>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .binary
                    .lock()
                    .insert(binary.0.lock().id(), binary.clone());
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
                let block: (Arc<Mutex<Block>>, SystemTime) = serde_json::from_reader(reader)?;
                store.block.lock().insert(block.0.lock().id, block.clone());
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
                let boolean_literal: (Arc<Mutex<BooleanLiteral>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .boolean_literal
                    .lock()
                    .insert(boolean_literal.0.lock().id(), boolean_literal.clone());
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
                let call: (Arc<Mutex<Call>>, SystemTime) = serde_json::from_reader(reader)?;
                store.call.lock().insert(call.0.lock().id, call.clone());
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
                let comparison: (Arc<Mutex<Comparison>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .comparison
                    .lock()
                    .insert(comparison.0.lock().id(), comparison.clone());
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
                let dwarf_source_file: (Arc<Mutex<DwarfSourceFile>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .dwarf_source_file
                    .lock()
                    .insert(dwarf_source_file.0.lock().id, dwarf_source_file.clone());
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
                let error: (Arc<Mutex<Error>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .error
                    .lock()
                    .insert(error.0.lock().id(), error.clone());
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
                let error_expression: (Arc<Mutex<ErrorExpression>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .error_expression
                    .lock()
                    .insert(error_expression.0.lock().id, error_expression.clone());
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
                let expression: (Arc<Mutex<Expression>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .expression
                    .lock()
                    .insert(expression.0.lock().id(), expression.clone());
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
                let expression_statement: (Arc<Mutex<ExpressionStatement>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.expression_statement.lock().insert(
                    expression_statement.0.lock().id,
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
                let field: (Arc<Mutex<Field>>, SystemTime) = serde_json::from_reader(reader)?;
                store.field_id_by_name.lock().insert(
                    field.0.lock().name.to_upper_camel_case(),
                    (field.0.lock().id, field.1),
                );
                store.field.lock().insert(field.0.lock().id, field.clone());
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
                let field_access: (Arc<Mutex<FieldAccess>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .field_access
                    .lock()
                    .insert(field_access.0.lock().id, field_access.clone());
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
                let field_access_target: (Arc<Mutex<FieldAccessTarget>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.field_access_target.lock().insert(
                    field_access_target.0.lock().id(),
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
                let field_expression: (Arc<Mutex<FieldExpression>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .field_expression
                    .lock()
                    .insert(field_expression.0.lock().id, field_expression.clone());
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
                let float_literal: (Arc<Mutex<FloatLiteral>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .float_literal
                    .lock()
                    .insert(float_literal.0.lock().id, float_literal.clone());
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
                let for_loop: (Arc<Mutex<ForLoop>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .for_loop
                    .lock()
                    .insert(for_loop.0.lock().id, for_loop.clone());
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
                let function: (Arc<Mutex<Function>>, SystemTime) = serde_json::from_reader(reader)?;
                store.function_id_by_name.lock().insert(
                    function.0.lock().name.to_upper_camel_case(),
                    (function.0.lock().id, function.1),
                );
                store
                    .function
                    .lock()
                    .insert(function.0.lock().id, function.clone());
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
                let grouped: (Arc<Mutex<Grouped>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .grouped
                    .lock()
                    .insert(grouped.0.lock().id, grouped.clone());
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
                let x_if: (Arc<Mutex<XIf>>, SystemTime) = serde_json::from_reader(reader)?;
                store.x_if.lock().insert(x_if.0.lock().id, x_if.clone());
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
                let implementation: (Arc<Mutex<Implementation>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .implementation
                    .lock()
                    .insert(implementation.0.lock().id, implementation.clone());
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
                let import: (Arc<Mutex<Import>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .import
                    .lock()
                    .insert(import.0.lock().id, import.clone());
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
                let index: (Arc<Mutex<Index>>, SystemTime) = serde_json::from_reader(reader)?;
                store.index.lock().insert(index.0.lock().id, index.clone());
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
                let integer_literal: (Arc<Mutex<IntegerLiteral>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .integer_literal
                    .lock()
                    .insert(integer_literal.0.lock().id, integer_literal.clone());
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
                let item: (Arc<Mutex<Item>>, SystemTime) = serde_json::from_reader(reader)?;
                store.item.lock().insert(item.0.lock().id, item.clone());
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
                let let_statement: (Arc<Mutex<LetStatement>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .let_statement
                    .lock()
                    .insert(let_statement.0.lock().id, let_statement.clone());
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
                let list: (Arc<Mutex<List>>, SystemTime) = serde_json::from_reader(reader)?;
                store.list.lock().insert(list.0.lock().id, list.clone());
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
                let list_element: (Arc<Mutex<ListElement>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .list_element
                    .lock()
                    .insert(list_element.0.lock().id, list_element.clone());
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
                let list_expression: (Arc<Mutex<ListExpression>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .list_expression
                    .lock()
                    .insert(list_expression.0.lock().id, list_expression.clone());
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
                let literal: (Arc<Mutex<Literal>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .literal
                    .lock()
                    .insert(literal.0.lock().id(), literal.clone());
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
                let local_variable: (Arc<Mutex<LocalVariable>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .local_variable
                    .lock()
                    .insert(local_variable.0.lock().id, local_variable.clone());
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
                let method_call: (Arc<Mutex<MethodCall>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .method_call
                    .lock()
                    .insert(method_call.0.lock().id, method_call.clone());
            }
        }

        // Load Negation.
        {
            let path = path.join("negation");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let negation: (Arc<Mutex<Negation>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .negation
                    .lock()
                    .insert(negation.0.lock().id, negation.clone());
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
                let z_object_store: (Arc<Mutex<ZObjectStore>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .z_object_store
                    .lock()
                    .insert(z_object_store.0.lock().id, z_object_store.clone());
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
                let operator: (Arc<Mutex<Operator>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .operator
                    .lock()
                    .insert(operator.0.lock().id, operator.clone());
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
                let woog_option: (Arc<Mutex<WoogOption>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .woog_option
                    .lock()
                    .insert(woog_option.0.lock().id, woog_option.clone());
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
                let parameter: (Arc<Mutex<Parameter>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .parameter
                    .lock()
                    .insert(parameter.0.lock().id, parameter.clone());
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
                let print: (Arc<Mutex<Print>>, SystemTime) = serde_json::from_reader(reader)?;
                store.print.lock().insert(print.0.lock().id, print.clone());
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
                let range_expression: (Arc<Mutex<RangeExpression>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .range_expression
                    .lock()
                    .insert(range_expression.0.lock().id, range_expression.clone());
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
                let reference: (Arc<Mutex<Reference>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .reference
                    .lock()
                    .insert(reference.0.lock().id, reference.clone());
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
                let result_statement: (Arc<Mutex<ResultStatement>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .result_statement
                    .lock()
                    .insert(result_statement.0.lock().id, result_statement.clone());
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
                let x_return: (Arc<Mutex<XReturn>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .x_return
                    .lock()
                    .insert(x_return.0.lock().id, x_return.clone());
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
                let z_some: (Arc<Mutex<ZSome>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .z_some
                    .lock()
                    .insert(z_some.0.lock().id, z_some.clone());
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
                let span: (Arc<Mutex<Span>>, SystemTime) = serde_json::from_reader(reader)?;
                store.span.lock().insert(span.0.lock().id, span.clone());
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
                let statement: (Arc<Mutex<Statement>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .statement
                    .lock()
                    .insert(statement.0.lock().id, statement.clone());
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
                let static_method_call: (Arc<Mutex<StaticMethodCall>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .static_method_call
                    .lock()
                    .insert(static_method_call.0.lock().id, static_method_call.clone());
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
                let string_literal: (Arc<Mutex<StringLiteral>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .string_literal
                    .lock()
                    .insert(string_literal.0.lock().id, string_literal.clone());
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
                let woog_struct: (Arc<Mutex<WoogStruct>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store.woog_struct_id_by_name.lock().insert(
                    woog_struct.0.lock().name.to_upper_camel_case(),
                    (woog_struct.0.lock().id, woog_struct.1),
                );
                store
                    .woog_struct
                    .lock()
                    .insert(woog_struct.0.lock().id, woog_struct.clone());
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
                let struct_expression: (Arc<Mutex<StructExpression>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .struct_expression
                    .lock()
                    .insert(struct_expression.0.lock().id, struct_expression.clone());
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
                let type_cast: (Arc<Mutex<TypeCast>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .type_cast
                    .lock()
                    .insert(type_cast.0.lock().id, type_cast.clone());
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
                let x_value: (Arc<Mutex<XValue>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .x_value
                    .lock()
                    .insert(x_value.0.lock().id, x_value.clone());
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
                let value_type: (Arc<Mutex<ValueType>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .value_type
                    .lock()
                    .insert(value_type.0.lock().id(), value_type.clone());
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
                let variable: (Arc<Mutex<Variable>>, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .variable
                    .lock()
                    .insert(variable.0.lock().id, variable.clone());
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
                let variable_expression: (Arc<Mutex<VariableExpression>>, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .variable_expression
                    .lock()
                    .insert(variable_expression.0.lock().id, variable_expression.clone());
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
