//! v2::lu_dog_vec Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_vec-object-store-file"}}}
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_vec-object-store-definition"}}}
use std::cell::RefCell;
use std::rc::Rc;
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
};

use fnv::FnvHashMap as HashMap;
use heck::ToUpperCamelCase;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::{
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
    argument_free_list: Vec<usize>,
    argument: Vec<Option<Rc<RefCell<Argument>>>>,
    binary_free_list: Vec<usize>,
    binary: Vec<Option<Rc<RefCell<Binary>>>>,
    block_free_list: Vec<usize>,
    block: Vec<Option<Rc<RefCell<Block>>>>,
    boolean_literal_free_list: Vec<usize>,
    boolean_literal: Vec<Option<Rc<RefCell<BooleanLiteral>>>>,
    boolean_operator_free_list: Vec<usize>,
    boolean_operator: Vec<Option<Rc<RefCell<BooleanOperator>>>>,
    call_free_list: Vec<usize>,
    call: Vec<Option<Rc<RefCell<Call>>>>,
    comparison_free_list: Vec<usize>,
    comparison: Vec<Option<Rc<RefCell<Comparison>>>>,
    dwarf_source_file_free_list: Vec<usize>,
    dwarf_source_file: Vec<Option<Rc<RefCell<DwarfSourceFile>>>>,
    error_free_list: Vec<usize>,
    error: Vec<Option<Rc<RefCell<Error>>>>,
    error_expression_free_list: Vec<usize>,
    error_expression: Vec<Option<Rc<RefCell<ErrorExpression>>>>,
    expression_free_list: Vec<usize>,
    expression: Vec<Option<Rc<RefCell<Expression>>>>,
    expression_statement_free_list: Vec<usize>,
    expression_statement: Vec<Option<Rc<RefCell<ExpressionStatement>>>>,
    field_free_list: Vec<usize>,
    field: Vec<Option<Rc<RefCell<Field>>>>,
    field_id_by_name: HashMap<String, usize>,
    field_access_free_list: Vec<usize>,
    field_access: Vec<Option<Rc<RefCell<FieldAccess>>>>,
    field_access_target_free_list: Vec<usize>,
    field_access_target: Vec<Option<Rc<RefCell<FieldAccessTarget>>>>,
    field_expression_free_list: Vec<usize>,
    field_expression: Vec<Option<Rc<RefCell<FieldExpression>>>>,
    float_literal_free_list: Vec<usize>,
    float_literal: Vec<Option<Rc<RefCell<FloatLiteral>>>>,
    for_loop_free_list: Vec<usize>,
    for_loop: Vec<Option<Rc<RefCell<ForLoop>>>>,
    function_free_list: Vec<usize>,
    function: Vec<Option<Rc<RefCell<Function>>>>,
    function_id_by_name: HashMap<String, usize>,
    grouped_free_list: Vec<usize>,
    grouped: Vec<Option<Rc<RefCell<Grouped>>>>,
    x_if_free_list: Vec<usize>,
    x_if: Vec<Option<Rc<RefCell<XIf>>>>,
    implementation_free_list: Vec<usize>,
    implementation: Vec<Option<Rc<RefCell<Implementation>>>>,
    import_free_list: Vec<usize>,
    import: Vec<Option<Rc<RefCell<Import>>>>,
    index_free_list: Vec<usize>,
    index: Vec<Option<Rc<RefCell<Index>>>>,
    integer_literal_free_list: Vec<usize>,
    integer_literal: Vec<Option<Rc<RefCell<IntegerLiteral>>>>,
    item_free_list: Vec<usize>,
    item: Vec<Option<Rc<RefCell<Item>>>>,
    lambda_free_list: Vec<usize>,
    lambda: Vec<Option<Rc<RefCell<Lambda>>>>,
    lambda_parameter_free_list: Vec<usize>,
    lambda_parameter: Vec<Option<Rc<RefCell<LambdaParameter>>>>,
    let_statement_free_list: Vec<usize>,
    let_statement: Vec<Option<Rc<RefCell<LetStatement>>>>,
    list_free_list: Vec<usize>,
    list: Vec<Option<Rc<RefCell<List>>>>,
    list_element_free_list: Vec<usize>,
    list_element: Vec<Option<Rc<RefCell<ListElement>>>>,
    list_expression_free_list: Vec<usize>,
    list_expression: Vec<Option<Rc<RefCell<ListExpression>>>>,
    literal_free_list: Vec<usize>,
    literal: Vec<Option<Rc<RefCell<Literal>>>>,
    local_variable_free_list: Vec<usize>,
    local_variable: Vec<Option<Rc<RefCell<LocalVariable>>>>,
    x_macro_free_list: Vec<usize>,
    x_macro: Vec<Option<Rc<RefCell<XMacro>>>>,
    method_call_free_list: Vec<usize>,
    method_call: Vec<Option<Rc<RefCell<MethodCall>>>>,
    z_object_store_free_list: Vec<usize>,
    z_object_store: Vec<Option<Rc<RefCell<ZObjectStore>>>>,
    operator_free_list: Vec<usize>,
    operator: Vec<Option<Rc<RefCell<Operator>>>>,
    woog_option_free_list: Vec<usize>,
    woog_option: Vec<Option<Rc<RefCell<WoogOption>>>>,
    parameter_free_list: Vec<usize>,
    parameter: Vec<Option<Rc<RefCell<Parameter>>>>,
    print_free_list: Vec<usize>,
    print: Vec<Option<Rc<RefCell<Print>>>>,
    range_expression_free_list: Vec<usize>,
    range_expression: Vec<Option<Rc<RefCell<RangeExpression>>>>,
    reference_free_list: Vec<usize>,
    reference: Vec<Option<Rc<RefCell<Reference>>>>,
    result_statement_free_list: Vec<usize>,
    result_statement: Vec<Option<Rc<RefCell<ResultStatement>>>>,
    x_return_free_list: Vec<usize>,
    x_return: Vec<Option<Rc<RefCell<XReturn>>>>,
    z_some_free_list: Vec<usize>,
    z_some: Vec<Option<Rc<RefCell<ZSome>>>>,
    span_free_list: Vec<usize>,
    span: Vec<Option<Rc<RefCell<Span>>>>,
    statement_free_list: Vec<usize>,
    statement: Vec<Option<Rc<RefCell<Statement>>>>,
    static_method_call_free_list: Vec<usize>,
    static_method_call: Vec<Option<Rc<RefCell<StaticMethodCall>>>>,
    string_literal_free_list: Vec<usize>,
    string_literal: Vec<Option<Rc<RefCell<StringLiteral>>>>,
    woog_struct_free_list: Vec<usize>,
    woog_struct: Vec<Option<Rc<RefCell<WoogStruct>>>>,
    woog_struct_id_by_name: HashMap<String, usize>,
    struct_expression_free_list: Vec<usize>,
    struct_expression: Vec<Option<Rc<RefCell<StructExpression>>>>,
    type_cast_free_list: Vec<usize>,
    type_cast: Vec<Option<Rc<RefCell<TypeCast>>>>,
    unary_free_list: Vec<usize>,
    unary: Vec<Option<Rc<RefCell<Unary>>>>,
    x_value_free_list: Vec<usize>,
    x_value: Vec<Option<Rc<RefCell<XValue>>>>,
    value_type_free_list: Vec<usize>,
    value_type: Vec<Option<Rc<RefCell<ValueType>>>>,
    variable_free_list: Vec<usize>,
    variable: Vec<Option<Rc<RefCell<Variable>>>>,
    variable_expression_free_list: Vec<usize>,
    variable_expression: Vec<Option<Rc<RefCell<VariableExpression>>>>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            argument_free_list: Vec::new(),
            argument: Vec::new(),
            binary_free_list: Vec::new(),
            binary: Vec::new(),
            block_free_list: Vec::new(),
            block: Vec::new(),
            boolean_literal_free_list: Vec::new(),
            boolean_literal: Vec::new(),
            boolean_operator_free_list: Vec::new(),
            boolean_operator: Vec::new(),
            call_free_list: Vec::new(),
            call: Vec::new(),
            comparison_free_list: Vec::new(),
            comparison: Vec::new(),
            dwarf_source_file_free_list: Vec::new(),
            dwarf_source_file: Vec::new(),
            error_free_list: Vec::new(),
            error: Vec::new(),
            error_expression_free_list: Vec::new(),
            error_expression: Vec::new(),
            expression_free_list: Vec::new(),
            expression: Vec::new(),
            expression_statement_free_list: Vec::new(),
            expression_statement: Vec::new(),
            field_free_list: Vec::new(),
            field: Vec::new(),
            field_id_by_name: HashMap::default(),
            field_access_free_list: Vec::new(),
            field_access: Vec::new(),
            field_access_target_free_list: Vec::new(),
            field_access_target: Vec::new(),
            field_expression_free_list: Vec::new(),
            field_expression: Vec::new(),
            float_literal_free_list: Vec::new(),
            float_literal: Vec::new(),
            for_loop_free_list: Vec::new(),
            for_loop: Vec::new(),
            function_free_list: Vec::new(),
            function: Vec::new(),
            function_id_by_name: HashMap::default(),
            grouped_free_list: Vec::new(),
            grouped: Vec::new(),
            x_if_free_list: Vec::new(),
            x_if: Vec::new(),
            implementation_free_list: Vec::new(),
            implementation: Vec::new(),
            import_free_list: Vec::new(),
            import: Vec::new(),
            index_free_list: Vec::new(),
            index: Vec::new(),
            integer_literal_free_list: Vec::new(),
            integer_literal: Vec::new(),
            item_free_list: Vec::new(),
            item: Vec::new(),
            lambda_free_list: Vec::new(),
            lambda: Vec::new(),
            lambda_parameter_free_list: Vec::new(),
            lambda_parameter: Vec::new(),
            let_statement_free_list: Vec::new(),
            let_statement: Vec::new(),
            list_free_list: Vec::new(),
            list: Vec::new(),
            list_element_free_list: Vec::new(),
            list_element: Vec::new(),
            list_expression_free_list: Vec::new(),
            list_expression: Vec::new(),
            literal_free_list: Vec::new(),
            literal: Vec::new(),
            local_variable_free_list: Vec::new(),
            local_variable: Vec::new(),
            x_macro_free_list: Vec::new(),
            x_macro: Vec::new(),
            method_call_free_list: Vec::new(),
            method_call: Vec::new(),
            z_object_store_free_list: Vec::new(),
            z_object_store: Vec::new(),
            operator_free_list: Vec::new(),
            operator: Vec::new(),
            woog_option_free_list: Vec::new(),
            woog_option: Vec::new(),
            parameter_free_list: Vec::new(),
            parameter: Vec::new(),
            print_free_list: Vec::new(),
            print: Vec::new(),
            range_expression_free_list: Vec::new(),
            range_expression: Vec::new(),
            reference_free_list: Vec::new(),
            reference: Vec::new(),
            result_statement_free_list: Vec::new(),
            result_statement: Vec::new(),
            x_return_free_list: Vec::new(),
            x_return: Vec::new(),
            z_some_free_list: Vec::new(),
            z_some: Vec::new(),
            span_free_list: Vec::new(),
            span: Vec::new(),
            statement_free_list: Vec::new(),
            statement: Vec::new(),
            static_method_call_free_list: Vec::new(),
            static_method_call: Vec::new(),
            string_literal_free_list: Vec::new(),
            string_literal: Vec::new(),
            woog_struct_free_list: Vec::new(),
            woog_struct: Vec::new(),
            woog_struct_id_by_name: HashMap::default(),
            struct_expression_free_list: Vec::new(),
            struct_expression: Vec::new(),
            type_cast_free_list: Vec::new(),
            type_cast: Vec::new(),
            unary_free_list: Vec::new(),
            unary: Vec::new(),
            x_value_free_list: Vec::new(),
            x_value: Vec::new(),
            value_type_free_list: Vec::new(),
            value_type: Vec::new(),
            variable_free_list: Vec::new(),
            variable: Vec::new(),
            variable_expression_free_list: Vec::new(),
            variable_expression: Vec::new(),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥
        store.inter_binary(|id| {
            Rc::new(RefCell::new(Binary {
                subtype: super::BinaryEnum::Addition(ADDITION),
                id,
            }))
        });
        store.inter_binary(|id| {
            Rc::new(RefCell::new(Binary {
                subtype: super::BinaryEnum::Assignment(ASSIGNMENT),
                id,
            }))
        });
        store.inter_binary(|id| {
            Rc::new(RefCell::new(Binary {
                subtype: super::BinaryEnum::Division(DIVISION),
                id,
            }))
        });
        store.inter_binary(|id| {
            Rc::new(RefCell::new(Binary {
                subtype: super::BinaryEnum::Multiplication(MULTIPLICATION),
                id,
            }))
        });
        store.inter_binary(|id| {
            Rc::new(RefCell::new(Binary {
                subtype: super::BinaryEnum::Subtraction(SUBTRACTION),
                id,
            }))
        });
        store.inter_boolean_literal(|id| {
            Rc::new(RefCell::new(BooleanLiteral {
                subtype: super::BooleanLiteralEnum::FalseLiteral(FALSE_LITERAL),
                id,
            }))
        });
        store.inter_boolean_literal(|id| {
            Rc::new(RefCell::new(BooleanLiteral {
                subtype: super::BooleanLiteralEnum::TrueLiteral(TRUE_LITERAL),
                id,
            }))
        });
        store.inter_boolean_operator(|id| {
            Rc::new(RefCell::new(BooleanOperator {
                subtype: super::BooleanOperatorEnum::And(AND),
                id,
            }))
        });
        store.inter_boolean_operator(|id| {
            Rc::new(RefCell::new(BooleanOperator {
                subtype: super::BooleanOperatorEnum::Or(OR),
                id,
            }))
        });
        store.inter_comparison(|id| {
            Rc::new(RefCell::new(Comparison {
                subtype: super::ComparisonEnum::Equal(EQUAL),
                id,
            }))
        });
        store.inter_comparison(|id| {
            Rc::new(RefCell::new(Comparison {
                subtype: super::ComparisonEnum::GreaterThan(GREATER_THAN),
                id,
            }))
        });
        store.inter_comparison(|id| {
            Rc::new(RefCell::new(Comparison {
                subtype: super::ComparisonEnum::GreaterThanOrEqual(GREATER_THAN_OR_EQUAL),
                id,
            }))
        });
        store.inter_comparison(|id| {
            Rc::new(RefCell::new(Comparison {
                subtype: super::ComparisonEnum::LessThan(LESS_THAN),
                id,
            }))
        });
        store.inter_comparison(|id| {
            Rc::new(RefCell::new(Comparison {
                subtype: super::ComparisonEnum::LessThanOrEqual(LESS_THAN_OR_EQUAL),
                id,
            }))
        });
        store.inter_comparison(|id| {
            Rc::new(RefCell::new(Comparison {
                subtype: super::ComparisonEnum::NotEqual(NOT_EQUAL),
                id,
            }))
        });
        store.inter_error(|id| {
            Rc::new(RefCell::new(Error {
                subtype: super::ErrorEnum::UnknownVariable(UNKNOWN_VARIABLE),
                id,
            }))
        });
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                subtype: super::ExpressionEnum::Debugger(DEBUGGER),
                id,
            }))
        });
        store.inter_expression(|id| {
            Rc::new(RefCell::new(Expression {
                subtype: super::ExpressionEnum::ZNone(Z_NONE),
                id,
            }))
        });
        store.inter_unary(|id| {
            Rc::new(RefCell::new(Unary {
                subtype: super::UnaryEnum::Negation(NEGATION),
                id,
            }))
        });
        store.inter_unary(|id| {
            Rc::new(RefCell::new(Unary {
                subtype: super::UnaryEnum::Not(NOT),
                id,
            }))
        });
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: super::ValueTypeEnum::Char(CHAR),
                id,
            }))
        });
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: super::ValueTypeEnum::Empty(EMPTY),
                id,
            }))
        });
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: super::ValueTypeEnum::Range(RANGE),
                id,
            }))
        });
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: super::ValueTypeEnum::Unknown(UNKNOWN),
                id,
            }))
        });

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_vec-object-store-methods"}}}
    /// Inter (insert) [`Argument`] into the store.
    ///
    pub fn inter_argument<F>(&mut self, argument: F) -> Rc<RefCell<Argument>>
    where
        F: Fn(usize) -> Rc<RefCell<Argument>>,
    {
        if let Some(_index) = self.argument_free_list.pop() {
            let argument = argument(_index);
            self.argument[_index] = Some(argument.clone());
            argument
        } else {
            let _index = self.argument.len();
            let argument = argument(_index);
            self.argument.push(Some(argument.clone()));
            argument
        }
    }

    /// Exhume (get) [`Argument`] from the store.
    ///
    pub fn exhume_argument(&self, id: &usize) -> Option<Rc<RefCell<Argument>>> {
        match self.argument.get(*id) {
            Some(argument) => argument.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Argument`] from the store.
    ///
    pub fn exorcise_argument(&mut self, id: &usize) -> Option<Rc<RefCell<Argument>>> {
        let result = self.argument[*id].take();
        self.argument_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Argument>`.
    ///
    pub fn iter_argument(&self) -> impl Iterator<Item = Rc<RefCell<Argument>>> + '_ {
        let len = self.argument.len();
        (0..len).map(move |i| {
            self.argument[i]
                .as_ref()
                .map(|argument| argument.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Binary`] into the store.
    ///
    pub fn inter_binary<F>(&mut self, binary: F) -> Rc<RefCell<Binary>>
    where
        F: Fn(usize) -> Rc<RefCell<Binary>>,
    {
        if let Some(_index) = self.binary_free_list.pop() {
            let binary = binary(_index);
            self.binary[_index] = Some(binary.clone());
            binary
        } else {
            let _index = self.binary.len();
            let binary = binary(_index);
            self.binary.push(Some(binary.clone()));
            binary
        }
    }

    /// Exhume (get) [`Binary`] from the store.
    ///
    pub fn exhume_binary(&self, id: &usize) -> Option<Rc<RefCell<Binary>>> {
        match self.binary.get(*id) {
            Some(binary) => binary.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Binary`] from the store.
    ///
    pub fn exorcise_binary(&mut self, id: &usize) -> Option<Rc<RefCell<Binary>>> {
        let result = self.binary[*id].take();
        self.binary_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Binary>`.
    ///
    pub fn iter_binary(&self) -> impl Iterator<Item = Rc<RefCell<Binary>>> + '_ {
        let len = self.binary.len();
        (0..len).map(move |i| {
            self.binary[i]
                .as_ref()
                .map(|binary| binary.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Block`] into the store.
    ///
    pub fn inter_block<F>(&mut self, block: F) -> Rc<RefCell<Block>>
    where
        F: Fn(usize) -> Rc<RefCell<Block>>,
    {
        if let Some(_index) = self.block_free_list.pop() {
            let block = block(_index);
            self.block[_index] = Some(block.clone());
            block
        } else {
            let _index = self.block.len();
            let block = block(_index);
            self.block.push(Some(block.clone()));
            block
        }
    }

    /// Exhume (get) [`Block`] from the store.
    ///
    pub fn exhume_block(&self, id: &usize) -> Option<Rc<RefCell<Block>>> {
        match self.block.get(*id) {
            Some(block) => block.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Block`] from the store.
    ///
    pub fn exorcise_block(&mut self, id: &usize) -> Option<Rc<RefCell<Block>>> {
        let result = self.block[*id].take();
        self.block_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Block>`.
    ///
    pub fn iter_block(&self) -> impl Iterator<Item = Rc<RefCell<Block>>> + '_ {
        let len = self.block.len();
        (0..len).map(move |i| self.block[i].as_ref().map(|block| block.clone()).unwrap())
    }

    /// Inter (insert) [`BooleanLiteral`] into the store.
    ///
    pub fn inter_boolean_literal<F>(&mut self, boolean_literal: F) -> Rc<RefCell<BooleanLiteral>>
    where
        F: Fn(usize) -> Rc<RefCell<BooleanLiteral>>,
    {
        if let Some(_index) = self.boolean_literal_free_list.pop() {
            let boolean_literal = boolean_literal(_index);
            self.boolean_literal[_index] = Some(boolean_literal.clone());
            boolean_literal
        } else {
            let _index = self.boolean_literal.len();
            let boolean_literal = boolean_literal(_index);
            self.boolean_literal.push(Some(boolean_literal.clone()));
            boolean_literal
        }
    }

    /// Exhume (get) [`BooleanLiteral`] from the store.
    ///
    pub fn exhume_boolean_literal(&self, id: &usize) -> Option<Rc<RefCell<BooleanLiteral>>> {
        match self.boolean_literal.get(*id) {
            Some(boolean_literal) => boolean_literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`BooleanLiteral`] from the store.
    ///
    pub fn exorcise_boolean_literal(&mut self, id: &usize) -> Option<Rc<RefCell<BooleanLiteral>>> {
        let result = self.boolean_literal[*id].take();
        self.boolean_literal_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanLiteral>`.
    ///
    pub fn iter_boolean_literal(&self) -> impl Iterator<Item = Rc<RefCell<BooleanLiteral>>> + '_ {
        let len = self.boolean_literal.len();
        (0..len).map(move |i| {
            self.boolean_literal[i]
                .as_ref()
                .map(|boolean_literal| boolean_literal.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`BooleanOperator`] into the store.
    ///
    pub fn inter_boolean_operator<F>(&mut self, boolean_operator: F) -> Rc<RefCell<BooleanOperator>>
    where
        F: Fn(usize) -> Rc<RefCell<BooleanOperator>>,
    {
        if let Some(_index) = self.boolean_operator_free_list.pop() {
            let boolean_operator = boolean_operator(_index);
            self.boolean_operator[_index] = Some(boolean_operator.clone());
            boolean_operator
        } else {
            let _index = self.boolean_operator.len();
            let boolean_operator = boolean_operator(_index);
            self.boolean_operator.push(Some(boolean_operator.clone()));
            boolean_operator
        }
    }

    /// Exhume (get) [`BooleanOperator`] from the store.
    ///
    pub fn exhume_boolean_operator(&self, id: &usize) -> Option<Rc<RefCell<BooleanOperator>>> {
        match self.boolean_operator.get(*id) {
            Some(boolean_operator) => boolean_operator.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`BooleanOperator`] from the store.
    ///
    pub fn exorcise_boolean_operator(
        &mut self,
        id: &usize,
    ) -> Option<Rc<RefCell<BooleanOperator>>> {
        let result = self.boolean_operator[*id].take();
        self.boolean_operator_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BooleanOperator>`.
    ///
    pub fn iter_boolean_operator(&self) -> impl Iterator<Item = Rc<RefCell<BooleanOperator>>> + '_ {
        let len = self.boolean_operator.len();
        (0..len).map(move |i| {
            self.boolean_operator[i]
                .as_ref()
                .map(|boolean_operator| boolean_operator.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Call`] into the store.
    ///
    pub fn inter_call<F>(&mut self, call: F) -> Rc<RefCell<Call>>
    where
        F: Fn(usize) -> Rc<RefCell<Call>>,
    {
        if let Some(_index) = self.call_free_list.pop() {
            let call = call(_index);
            self.call[_index] = Some(call.clone());
            call
        } else {
            let _index = self.call.len();
            let call = call(_index);
            self.call.push(Some(call.clone()));
            call
        }
    }

    /// Exhume (get) [`Call`] from the store.
    ///
    pub fn exhume_call(&self, id: &usize) -> Option<Rc<RefCell<Call>>> {
        match self.call.get(*id) {
            Some(call) => call.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Call`] from the store.
    ///
    pub fn exorcise_call(&mut self, id: &usize) -> Option<Rc<RefCell<Call>>> {
        let result = self.call[*id].take();
        self.call_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Call>`.
    ///
    pub fn iter_call(&self) -> impl Iterator<Item = Rc<RefCell<Call>>> + '_ {
        let len = self.call.len();
        (0..len).map(move |i| self.call[i].as_ref().map(|call| call.clone()).unwrap())
    }

    /// Inter (insert) [`Comparison`] into the store.
    ///
    pub fn inter_comparison<F>(&mut self, comparison: F) -> Rc<RefCell<Comparison>>
    where
        F: Fn(usize) -> Rc<RefCell<Comparison>>,
    {
        if let Some(_index) = self.comparison_free_list.pop() {
            let comparison = comparison(_index);
            self.comparison[_index] = Some(comparison.clone());
            comparison
        } else {
            let _index = self.comparison.len();
            let comparison = comparison(_index);
            self.comparison.push(Some(comparison.clone()));
            comparison
        }
    }

    /// Exhume (get) [`Comparison`] from the store.
    ///
    pub fn exhume_comparison(&self, id: &usize) -> Option<Rc<RefCell<Comparison>>> {
        match self.comparison.get(*id) {
            Some(comparison) => comparison.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Comparison`] from the store.
    ///
    pub fn exorcise_comparison(&mut self, id: &usize) -> Option<Rc<RefCell<Comparison>>> {
        let result = self.comparison[*id].take();
        self.comparison_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Comparison>`.
    ///
    pub fn iter_comparison(&self) -> impl Iterator<Item = Rc<RefCell<Comparison>>> + '_ {
        let len = self.comparison.len();
        (0..len).map(move |i| {
            self.comparison[i]
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
    ) -> Rc<RefCell<DwarfSourceFile>>
    where
        F: Fn(usize) -> Rc<RefCell<DwarfSourceFile>>,
    {
        if let Some(_index) = self.dwarf_source_file_free_list.pop() {
            let dwarf_source_file = dwarf_source_file(_index);
            self.dwarf_source_file[_index] = Some(dwarf_source_file.clone());
            dwarf_source_file
        } else {
            let _index = self.dwarf_source_file.len();
            let dwarf_source_file = dwarf_source_file(_index);
            self.dwarf_source_file.push(Some(dwarf_source_file.clone()));
            dwarf_source_file
        }
    }

    /// Exhume (get) [`DwarfSourceFile`] from the store.
    ///
    pub fn exhume_dwarf_source_file(&self, id: &usize) -> Option<Rc<RefCell<DwarfSourceFile>>> {
        match self.dwarf_source_file.get(*id) {
            Some(dwarf_source_file) => dwarf_source_file.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`DwarfSourceFile`] from the store.
    ///
    pub fn exorcise_dwarf_source_file(
        &mut self,
        id: &usize,
    ) -> Option<Rc<RefCell<DwarfSourceFile>>> {
        let result = self.dwarf_source_file[*id].take();
        self.dwarf_source_file_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, DwarfSourceFile>`.
    ///
    pub fn iter_dwarf_source_file(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<DwarfSourceFile>>> + '_ {
        let len = self.dwarf_source_file.len();
        (0..len).map(move |i| {
            self.dwarf_source_file[i]
                .as_ref()
                .map(|dwarf_source_file| dwarf_source_file.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Error`] into the store.
    ///
    pub fn inter_error<F>(&mut self, error: F) -> Rc<RefCell<Error>>
    where
        F: Fn(usize) -> Rc<RefCell<Error>>,
    {
        if let Some(_index) = self.error_free_list.pop() {
            let error = error(_index);
            self.error[_index] = Some(error.clone());
            error
        } else {
            let _index = self.error.len();
            let error = error(_index);
            self.error.push(Some(error.clone()));
            error
        }
    }

    /// Exhume (get) [`Error`] from the store.
    ///
    pub fn exhume_error(&self, id: &usize) -> Option<Rc<RefCell<Error>>> {
        match self.error.get(*id) {
            Some(error) => error.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Error`] from the store.
    ///
    pub fn exorcise_error(&mut self, id: &usize) -> Option<Rc<RefCell<Error>>> {
        let result = self.error[*id].take();
        self.error_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Error>`.
    ///
    pub fn iter_error(&self) -> impl Iterator<Item = Rc<RefCell<Error>>> + '_ {
        let len = self.error.len();
        (0..len).map(move |i| self.error[i].as_ref().map(|error| error.clone()).unwrap())
    }

    /// Inter (insert) [`ErrorExpression`] into the store.
    ///
    pub fn inter_error_expression<F>(&mut self, error_expression: F) -> Rc<RefCell<ErrorExpression>>
    where
        F: Fn(usize) -> Rc<RefCell<ErrorExpression>>,
    {
        if let Some(_index) = self.error_expression_free_list.pop() {
            let error_expression = error_expression(_index);
            self.error_expression[_index] = Some(error_expression.clone());
            error_expression
        } else {
            let _index = self.error_expression.len();
            let error_expression = error_expression(_index);
            self.error_expression.push(Some(error_expression.clone()));
            error_expression
        }
    }

    /// Exhume (get) [`ErrorExpression`] from the store.
    ///
    pub fn exhume_error_expression(&self, id: &usize) -> Option<Rc<RefCell<ErrorExpression>>> {
        match self.error_expression.get(*id) {
            Some(error_expression) => error_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ErrorExpression`] from the store.
    ///
    pub fn exorcise_error_expression(
        &mut self,
        id: &usize,
    ) -> Option<Rc<RefCell<ErrorExpression>>> {
        let result = self.error_expression[*id].take();
        self.error_expression_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ErrorExpression>`.
    ///
    pub fn iter_error_expression(&self) -> impl Iterator<Item = Rc<RefCell<ErrorExpression>>> + '_ {
        let len = self.error_expression.len();
        (0..len).map(move |i| {
            self.error_expression[i]
                .as_ref()
                .map(|error_expression| error_expression.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Expression`] into the store.
    ///
    pub fn inter_expression<F>(&mut self, expression: F) -> Rc<RefCell<Expression>>
    where
        F: Fn(usize) -> Rc<RefCell<Expression>>,
    {
        if let Some(_index) = self.expression_free_list.pop() {
            let expression = expression(_index);
            self.expression[_index] = Some(expression.clone());
            expression
        } else {
            let _index = self.expression.len();
            let expression = expression(_index);
            self.expression.push(Some(expression.clone()));
            expression
        }
    }

    /// Exhume (get) [`Expression`] from the store.
    ///
    pub fn exhume_expression(&self, id: &usize) -> Option<Rc<RefCell<Expression>>> {
        match self.expression.get(*id) {
            Some(expression) => expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Expression`] from the store.
    ///
    pub fn exorcise_expression(&mut self, id: &usize) -> Option<Rc<RefCell<Expression>>> {
        let result = self.expression[*id].take();
        self.expression_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Expression>`.
    ///
    pub fn iter_expression(&self) -> impl Iterator<Item = Rc<RefCell<Expression>>> + '_ {
        let len = self.expression.len();
        (0..len).map(move |i| {
            self.expression[i]
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
    ) -> Rc<RefCell<ExpressionStatement>>
    where
        F: Fn(usize) -> Rc<RefCell<ExpressionStatement>>,
    {
        if let Some(_index) = self.expression_statement_free_list.pop() {
            let expression_statement = expression_statement(_index);
            self.expression_statement[_index] = Some(expression_statement.clone());
            expression_statement
        } else {
            let _index = self.expression_statement.len();
            let expression_statement = expression_statement(_index);
            self.expression_statement
                .push(Some(expression_statement.clone()));
            expression_statement
        }
    }

    /// Exhume (get) [`ExpressionStatement`] from the store.
    ///
    pub fn exhume_expression_statement(
        &self,
        id: &usize,
    ) -> Option<Rc<RefCell<ExpressionStatement>>> {
        match self.expression_statement.get(*id) {
            Some(expression_statement) => expression_statement.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ExpressionStatement`] from the store.
    ///
    pub fn exorcise_expression_statement(
        &mut self,
        id: &usize,
    ) -> Option<Rc<RefCell<ExpressionStatement>>> {
        let result = self.expression_statement[*id].take();
        self.expression_statement_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ExpressionStatement>`.
    ///
    pub fn iter_expression_statement(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<ExpressionStatement>>> + '_ {
        let len = self.expression_statement.len();
        (0..len).map(move |i| {
            self.expression_statement[i]
                .as_ref()
                .map(|expression_statement| expression_statement.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Field`] into the store.
    ///
    pub fn inter_field<F>(&mut self, field: F) -> Rc<RefCell<Field>>
    where
        F: Fn(usize) -> Rc<RefCell<Field>>,
    {
        let field = if let Some(_index) = self.field_free_list.pop() {
            let field = field(_index);
            self.field[_index] = Some(field.clone());
            field
        } else {
            let _index = self.field.len();
            let field = field(_index);
            self.field.push(Some(field.clone()));
            field
        };
        self.field_id_by_name
            .insert(field.borrow().name.to_upper_camel_case(), field.borrow().id);
        field
    }

    /// Exhume (get) [`Field`] from the store.
    ///
    pub fn exhume_field(&self, id: &usize) -> Option<Rc<RefCell<Field>>> {
        match self.field.get(*id) {
            Some(field) => field.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Field`] from the store.
    ///
    pub fn exorcise_field(&mut self, id: &usize) -> Option<Rc<RefCell<Field>>> {
        let result = self.field[*id].take();
        self.field_free_list.push(*id);
        result
    }

    /// Exorcise [`Field`] id from the store by name.
    ///
    pub fn exhume_field_id_by_name(&self, name: &str) -> Option<usize> {
        self.field_id_by_name.get(name).map(|field| *field)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Field>`.
    ///
    pub fn iter_field(&self) -> impl Iterator<Item = Rc<RefCell<Field>>> + '_ {
        let len = self.field.len();
        (0..len).map(move |i| self.field[i].as_ref().map(|field| field.clone()).unwrap())
    }

    /// Inter (insert) [`FieldAccess`] into the store.
    ///
    pub fn inter_field_access<F>(&mut self, field_access: F) -> Rc<RefCell<FieldAccess>>
    where
        F: Fn(usize) -> Rc<RefCell<FieldAccess>>,
    {
        if let Some(_index) = self.field_access_free_list.pop() {
            let field_access = field_access(_index);
            self.field_access[_index] = Some(field_access.clone());
            field_access
        } else {
            let _index = self.field_access.len();
            let field_access = field_access(_index);
            self.field_access.push(Some(field_access.clone()));
            field_access
        }
    }

    /// Exhume (get) [`FieldAccess`] from the store.
    ///
    pub fn exhume_field_access(&self, id: &usize) -> Option<Rc<RefCell<FieldAccess>>> {
        match self.field_access.get(*id) {
            Some(field_access) => field_access.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FieldAccess`] from the store.
    ///
    pub fn exorcise_field_access(&mut self, id: &usize) -> Option<Rc<RefCell<FieldAccess>>> {
        let result = self.field_access[*id].take();
        self.field_access_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccess>`.
    ///
    pub fn iter_field_access(&self) -> impl Iterator<Item = Rc<RefCell<FieldAccess>>> + '_ {
        let len = self.field_access.len();
        (0..len).map(move |i| {
            self.field_access[i]
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
    ) -> Rc<RefCell<FieldAccessTarget>>
    where
        F: Fn(usize) -> Rc<RefCell<FieldAccessTarget>>,
    {
        if let Some(_index) = self.field_access_target_free_list.pop() {
            let field_access_target = field_access_target(_index);
            self.field_access_target[_index] = Some(field_access_target.clone());
            field_access_target
        } else {
            let _index = self.field_access_target.len();
            let field_access_target = field_access_target(_index);
            self.field_access_target
                .push(Some(field_access_target.clone()));
            field_access_target
        }
    }

    /// Exhume (get) [`FieldAccessTarget`] from the store.
    ///
    pub fn exhume_field_access_target(&self, id: &usize) -> Option<Rc<RefCell<FieldAccessTarget>>> {
        match self.field_access_target.get(*id) {
            Some(field_access_target) => field_access_target.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FieldAccessTarget`] from the store.
    ///
    pub fn exorcise_field_access_target(
        &mut self,
        id: &usize,
    ) -> Option<Rc<RefCell<FieldAccessTarget>>> {
        let result = self.field_access_target[*id].take();
        self.field_access_target_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldAccessTarget>`.
    ///
    pub fn iter_field_access_target(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<FieldAccessTarget>>> + '_ {
        let len = self.field_access_target.len();
        (0..len).map(move |i| {
            self.field_access_target[i]
                .as_ref()
                .map(|field_access_target| field_access_target.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`FieldExpression`] into the store.
    ///
    pub fn inter_field_expression<F>(&mut self, field_expression: F) -> Rc<RefCell<FieldExpression>>
    where
        F: Fn(usize) -> Rc<RefCell<FieldExpression>>,
    {
        if let Some(_index) = self.field_expression_free_list.pop() {
            let field_expression = field_expression(_index);
            self.field_expression[_index] = Some(field_expression.clone());
            field_expression
        } else {
            let _index = self.field_expression.len();
            let field_expression = field_expression(_index);
            self.field_expression.push(Some(field_expression.clone()));
            field_expression
        }
    }

    /// Exhume (get) [`FieldExpression`] from the store.
    ///
    pub fn exhume_field_expression(&self, id: &usize) -> Option<Rc<RefCell<FieldExpression>>> {
        match self.field_expression.get(*id) {
            Some(field_expression) => field_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FieldExpression`] from the store.
    ///
    pub fn exorcise_field_expression(
        &mut self,
        id: &usize,
    ) -> Option<Rc<RefCell<FieldExpression>>> {
        let result = self.field_expression[*id].take();
        self.field_expression_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FieldExpression>`.
    ///
    pub fn iter_field_expression(&self) -> impl Iterator<Item = Rc<RefCell<FieldExpression>>> + '_ {
        let len = self.field_expression.len();
        (0..len).map(move |i| {
            self.field_expression[i]
                .as_ref()
                .map(|field_expression| field_expression.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`FloatLiteral`] into the store.
    ///
    pub fn inter_float_literal<F>(&mut self, float_literal: F) -> Rc<RefCell<FloatLiteral>>
    where
        F: Fn(usize) -> Rc<RefCell<FloatLiteral>>,
    {
        if let Some(_index) = self.float_literal_free_list.pop() {
            let float_literal = float_literal(_index);
            self.float_literal[_index] = Some(float_literal.clone());
            float_literal
        } else {
            let _index = self.float_literal.len();
            let float_literal = float_literal(_index);
            self.float_literal.push(Some(float_literal.clone()));
            float_literal
        }
    }

    /// Exhume (get) [`FloatLiteral`] from the store.
    ///
    pub fn exhume_float_literal(&self, id: &usize) -> Option<Rc<RefCell<FloatLiteral>>> {
        match self.float_literal.get(*id) {
            Some(float_literal) => float_literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`FloatLiteral`] from the store.
    ///
    pub fn exorcise_float_literal(&mut self, id: &usize) -> Option<Rc<RefCell<FloatLiteral>>> {
        let result = self.float_literal[*id].take();
        self.float_literal_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, FloatLiteral>`.
    ///
    pub fn iter_float_literal(&self) -> impl Iterator<Item = Rc<RefCell<FloatLiteral>>> + '_ {
        let len = self.float_literal.len();
        (0..len).map(move |i| {
            self.float_literal[i]
                .as_ref()
                .map(|float_literal| float_literal.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`ForLoop`] into the store.
    ///
    pub fn inter_for_loop<F>(&mut self, for_loop: F) -> Rc<RefCell<ForLoop>>
    where
        F: Fn(usize) -> Rc<RefCell<ForLoop>>,
    {
        if let Some(_index) = self.for_loop_free_list.pop() {
            let for_loop = for_loop(_index);
            self.for_loop[_index] = Some(for_loop.clone());
            for_loop
        } else {
            let _index = self.for_loop.len();
            let for_loop = for_loop(_index);
            self.for_loop.push(Some(for_loop.clone()));
            for_loop
        }
    }

    /// Exhume (get) [`ForLoop`] from the store.
    ///
    pub fn exhume_for_loop(&self, id: &usize) -> Option<Rc<RefCell<ForLoop>>> {
        match self.for_loop.get(*id) {
            Some(for_loop) => for_loop.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ForLoop`] from the store.
    ///
    pub fn exorcise_for_loop(&mut self, id: &usize) -> Option<Rc<RefCell<ForLoop>>> {
        let result = self.for_loop[*id].take();
        self.for_loop_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ForLoop>`.
    ///
    pub fn iter_for_loop(&self) -> impl Iterator<Item = Rc<RefCell<ForLoop>>> + '_ {
        let len = self.for_loop.len();
        (0..len).map(move |i| {
            self.for_loop[i]
                .as_ref()
                .map(|for_loop| for_loop.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Function`] into the store.
    ///
    pub fn inter_function<F>(&mut self, function: F) -> Rc<RefCell<Function>>
    where
        F: Fn(usize) -> Rc<RefCell<Function>>,
    {
        let function = if let Some(_index) = self.function_free_list.pop() {
            let function = function(_index);
            self.function[_index] = Some(function.clone());
            function
        } else {
            let _index = self.function.len();
            let function = function(_index);
            self.function.push(Some(function.clone()));
            function
        };
        self.function_id_by_name.insert(
            function.borrow().name.to_upper_camel_case(),
            function.borrow().id,
        );
        function
    }

    /// Exhume (get) [`Function`] from the store.
    ///
    pub fn exhume_function(&self, id: &usize) -> Option<Rc<RefCell<Function>>> {
        match self.function.get(*id) {
            Some(function) => function.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Function`] from the store.
    ///
    pub fn exorcise_function(&mut self, id: &usize) -> Option<Rc<RefCell<Function>>> {
        let result = self.function[*id].take();
        self.function_free_list.push(*id);
        result
    }

    /// Exorcise [`Function`] id from the store by name.
    ///
    pub fn exhume_function_id_by_name(&self, name: &str) -> Option<usize> {
        self.function_id_by_name.get(name).map(|function| *function)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Function>`.
    ///
    pub fn iter_function(&self) -> impl Iterator<Item = Rc<RefCell<Function>>> + '_ {
        let len = self.function.len();
        (0..len).map(move |i| {
            self.function[i]
                .as_ref()
                .map(|function| function.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Grouped`] into the store.
    ///
    pub fn inter_grouped<F>(&mut self, grouped: F) -> Rc<RefCell<Grouped>>
    where
        F: Fn(usize) -> Rc<RefCell<Grouped>>,
    {
        if let Some(_index) = self.grouped_free_list.pop() {
            let grouped = grouped(_index);
            self.grouped[_index] = Some(grouped.clone());
            grouped
        } else {
            let _index = self.grouped.len();
            let grouped = grouped(_index);
            self.grouped.push(Some(grouped.clone()));
            grouped
        }
    }

    /// Exhume (get) [`Grouped`] from the store.
    ///
    pub fn exhume_grouped(&self, id: &usize) -> Option<Rc<RefCell<Grouped>>> {
        match self.grouped.get(*id) {
            Some(grouped) => grouped.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Grouped`] from the store.
    ///
    pub fn exorcise_grouped(&mut self, id: &usize) -> Option<Rc<RefCell<Grouped>>> {
        let result = self.grouped[*id].take();
        self.grouped_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Grouped>`.
    ///
    pub fn iter_grouped(&self) -> impl Iterator<Item = Rc<RefCell<Grouped>>> + '_ {
        let len = self.grouped.len();
        (0..len).map(move |i| {
            self.grouped[i]
                .as_ref()
                .map(|grouped| grouped.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`XIf`] into the store.
    ///
    pub fn inter_x_if<F>(&mut self, x_if: F) -> Rc<RefCell<XIf>>
    where
        F: Fn(usize) -> Rc<RefCell<XIf>>,
    {
        if let Some(_index) = self.x_if_free_list.pop() {
            let x_if = x_if(_index);
            self.x_if[_index] = Some(x_if.clone());
            x_if
        } else {
            let _index = self.x_if.len();
            let x_if = x_if(_index);
            self.x_if.push(Some(x_if.clone()));
            x_if
        }
    }

    /// Exhume (get) [`XIf`] from the store.
    ///
    pub fn exhume_x_if(&self, id: &usize) -> Option<Rc<RefCell<XIf>>> {
        match self.x_if.get(*id) {
            Some(x_if) => x_if.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XIf`] from the store.
    ///
    pub fn exorcise_x_if(&mut self, id: &usize) -> Option<Rc<RefCell<XIf>>> {
        let result = self.x_if[*id].take();
        self.x_if_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XIf>`.
    ///
    pub fn iter_x_if(&self) -> impl Iterator<Item = Rc<RefCell<XIf>>> + '_ {
        let len = self.x_if.len();
        (0..len).map(move |i| self.x_if[i].as_ref().map(|x_if| x_if.clone()).unwrap())
    }

    /// Inter (insert) [`Implementation`] into the store.
    ///
    pub fn inter_implementation<F>(&mut self, implementation: F) -> Rc<RefCell<Implementation>>
    where
        F: Fn(usize) -> Rc<RefCell<Implementation>>,
    {
        if let Some(_index) = self.implementation_free_list.pop() {
            let implementation = implementation(_index);
            self.implementation[_index] = Some(implementation.clone());
            implementation
        } else {
            let _index = self.implementation.len();
            let implementation = implementation(_index);
            self.implementation.push(Some(implementation.clone()));
            implementation
        }
    }

    /// Exhume (get) [`Implementation`] from the store.
    ///
    pub fn exhume_implementation(&self, id: &usize) -> Option<Rc<RefCell<Implementation>>> {
        match self.implementation.get(*id) {
            Some(implementation) => implementation.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Implementation`] from the store.
    ///
    pub fn exorcise_implementation(&mut self, id: &usize) -> Option<Rc<RefCell<Implementation>>> {
        let result = self.implementation[*id].take();
        self.implementation_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Implementation>`.
    ///
    pub fn iter_implementation(&self) -> impl Iterator<Item = Rc<RefCell<Implementation>>> + '_ {
        let len = self.implementation.len();
        (0..len).map(move |i| {
            self.implementation[i]
                .as_ref()
                .map(|implementation| implementation.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Import`] into the store.
    ///
    pub fn inter_import<F>(&mut self, import: F) -> Rc<RefCell<Import>>
    where
        F: Fn(usize) -> Rc<RefCell<Import>>,
    {
        if let Some(_index) = self.import_free_list.pop() {
            let import = import(_index);
            self.import[_index] = Some(import.clone());
            import
        } else {
            let _index = self.import.len();
            let import = import(_index);
            self.import.push(Some(import.clone()));
            import
        }
    }

    /// Exhume (get) [`Import`] from the store.
    ///
    pub fn exhume_import(&self, id: &usize) -> Option<Rc<RefCell<Import>>> {
        match self.import.get(*id) {
            Some(import) => import.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Import`] from the store.
    ///
    pub fn exorcise_import(&mut self, id: &usize) -> Option<Rc<RefCell<Import>>> {
        let result = self.import[*id].take();
        self.import_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Import>`.
    ///
    pub fn iter_import(&self) -> impl Iterator<Item = Rc<RefCell<Import>>> + '_ {
        let len = self.import.len();
        (0..len).map(move |i| {
            self.import[i]
                .as_ref()
                .map(|import| import.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Index`] into the store.
    ///
    pub fn inter_index<F>(&mut self, index: F) -> Rc<RefCell<Index>>
    where
        F: Fn(usize) -> Rc<RefCell<Index>>,
    {
        if let Some(_index) = self.index_free_list.pop() {
            let index = index(_index);
            self.index[_index] = Some(index.clone());
            index
        } else {
            let _index = self.index.len();
            let index = index(_index);
            self.index.push(Some(index.clone()));
            index
        }
    }

    /// Exhume (get) [`Index`] from the store.
    ///
    pub fn exhume_index(&self, id: &usize) -> Option<Rc<RefCell<Index>>> {
        match self.index.get(*id) {
            Some(index) => index.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Index`] from the store.
    ///
    pub fn exorcise_index(&mut self, id: &usize) -> Option<Rc<RefCell<Index>>> {
        let result = self.index[*id].take();
        self.index_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Index>`.
    ///
    pub fn iter_index(&self) -> impl Iterator<Item = Rc<RefCell<Index>>> + '_ {
        let len = self.index.len();
        (0..len).map(move |i| self.index[i].as_ref().map(|index| index.clone()).unwrap())
    }

    /// Inter (insert) [`IntegerLiteral`] into the store.
    ///
    pub fn inter_integer_literal<F>(&mut self, integer_literal: F) -> Rc<RefCell<IntegerLiteral>>
    where
        F: Fn(usize) -> Rc<RefCell<IntegerLiteral>>,
    {
        if let Some(_index) = self.integer_literal_free_list.pop() {
            let integer_literal = integer_literal(_index);
            self.integer_literal[_index] = Some(integer_literal.clone());
            integer_literal
        } else {
            let _index = self.integer_literal.len();
            let integer_literal = integer_literal(_index);
            self.integer_literal.push(Some(integer_literal.clone()));
            integer_literal
        }
    }

    /// Exhume (get) [`IntegerLiteral`] from the store.
    ///
    pub fn exhume_integer_literal(&self, id: &usize) -> Option<Rc<RefCell<IntegerLiteral>>> {
        match self.integer_literal.get(*id) {
            Some(integer_literal) => integer_literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`IntegerLiteral`] from the store.
    ///
    pub fn exorcise_integer_literal(&mut self, id: &usize) -> Option<Rc<RefCell<IntegerLiteral>>> {
        let result = self.integer_literal[*id].take();
        self.integer_literal_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, IntegerLiteral>`.
    ///
    pub fn iter_integer_literal(&self) -> impl Iterator<Item = Rc<RefCell<IntegerLiteral>>> + '_ {
        let len = self.integer_literal.len();
        (0..len).map(move |i| {
            self.integer_literal[i]
                .as_ref()
                .map(|integer_literal| integer_literal.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Item`] into the store.
    ///
    pub fn inter_item<F>(&mut self, item: F) -> Rc<RefCell<Item>>
    where
        F: Fn(usize) -> Rc<RefCell<Item>>,
    {
        if let Some(_index) = self.item_free_list.pop() {
            let item = item(_index);
            self.item[_index] = Some(item.clone());
            item
        } else {
            let _index = self.item.len();
            let item = item(_index);
            self.item.push(Some(item.clone()));
            item
        }
    }

    /// Exhume (get) [`Item`] from the store.
    ///
    pub fn exhume_item(&self, id: &usize) -> Option<Rc<RefCell<Item>>> {
        match self.item.get(*id) {
            Some(item) => item.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Item`] from the store.
    ///
    pub fn exorcise_item(&mut self, id: &usize) -> Option<Rc<RefCell<Item>>> {
        let result = self.item[*id].take();
        self.item_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Item>`.
    ///
    pub fn iter_item(&self) -> impl Iterator<Item = Rc<RefCell<Item>>> + '_ {
        let len = self.item.len();
        (0..len).map(move |i| self.item[i].as_ref().map(|item| item.clone()).unwrap())
    }

    /// Inter (insert) [`Lambda`] into the store.
    ///
    pub fn inter_lambda<F>(&mut self, lambda: F) -> Rc<RefCell<Lambda>>
    where
        F: Fn(usize) -> Rc<RefCell<Lambda>>,
    {
        if let Some(_index) = self.lambda_free_list.pop() {
            let lambda = lambda(_index);
            self.lambda[_index] = Some(lambda.clone());
            lambda
        } else {
            let _index = self.lambda.len();
            let lambda = lambda(_index);
            self.lambda.push(Some(lambda.clone()));
            lambda
        }
    }

    /// Exhume (get) [`Lambda`] from the store.
    ///
    pub fn exhume_lambda(&self, id: &usize) -> Option<Rc<RefCell<Lambda>>> {
        match self.lambda.get(*id) {
            Some(lambda) => lambda.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Lambda`] from the store.
    ///
    pub fn exorcise_lambda(&mut self, id: &usize) -> Option<Rc<RefCell<Lambda>>> {
        let result = self.lambda[*id].take();
        self.lambda_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Lambda>`.
    ///
    pub fn iter_lambda(&self) -> impl Iterator<Item = Rc<RefCell<Lambda>>> + '_ {
        let len = self.lambda.len();
        (0..len).map(move |i| {
            self.lambda[i]
                .as_ref()
                .map(|lambda| lambda.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`LambdaParameter`] into the store.
    ///
    pub fn inter_lambda_parameter<F>(&mut self, lambda_parameter: F) -> Rc<RefCell<LambdaParameter>>
    where
        F: Fn(usize) -> Rc<RefCell<LambdaParameter>>,
    {
        if let Some(_index) = self.lambda_parameter_free_list.pop() {
            let lambda_parameter = lambda_parameter(_index);
            self.lambda_parameter[_index] = Some(lambda_parameter.clone());
            lambda_parameter
        } else {
            let _index = self.lambda_parameter.len();
            let lambda_parameter = lambda_parameter(_index);
            self.lambda_parameter.push(Some(lambda_parameter.clone()));
            lambda_parameter
        }
    }

    /// Exhume (get) [`LambdaParameter`] from the store.
    ///
    pub fn exhume_lambda_parameter(&self, id: &usize) -> Option<Rc<RefCell<LambdaParameter>>> {
        match self.lambda_parameter.get(*id) {
            Some(lambda_parameter) => lambda_parameter.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`LambdaParameter`] from the store.
    ///
    pub fn exorcise_lambda_parameter(
        &mut self,
        id: &usize,
    ) -> Option<Rc<RefCell<LambdaParameter>>> {
        let result = self.lambda_parameter[*id].take();
        self.lambda_parameter_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LambdaParameter>`.
    ///
    pub fn iter_lambda_parameter(&self) -> impl Iterator<Item = Rc<RefCell<LambdaParameter>>> + '_ {
        let len = self.lambda_parameter.len();
        (0..len).map(move |i| {
            self.lambda_parameter[i]
                .as_ref()
                .map(|lambda_parameter| lambda_parameter.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`LetStatement`] into the store.
    ///
    pub fn inter_let_statement<F>(&mut self, let_statement: F) -> Rc<RefCell<LetStatement>>
    where
        F: Fn(usize) -> Rc<RefCell<LetStatement>>,
    {
        if let Some(_index) = self.let_statement_free_list.pop() {
            let let_statement = let_statement(_index);
            self.let_statement[_index] = Some(let_statement.clone());
            let_statement
        } else {
            let _index = self.let_statement.len();
            let let_statement = let_statement(_index);
            self.let_statement.push(Some(let_statement.clone()));
            let_statement
        }
    }

    /// Exhume (get) [`LetStatement`] from the store.
    ///
    pub fn exhume_let_statement(&self, id: &usize) -> Option<Rc<RefCell<LetStatement>>> {
        match self.let_statement.get(*id) {
            Some(let_statement) => let_statement.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`LetStatement`] from the store.
    ///
    pub fn exorcise_let_statement(&mut self, id: &usize) -> Option<Rc<RefCell<LetStatement>>> {
        let result = self.let_statement[*id].take();
        self.let_statement_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LetStatement>`.
    ///
    pub fn iter_let_statement(&self) -> impl Iterator<Item = Rc<RefCell<LetStatement>>> + '_ {
        let len = self.let_statement.len();
        (0..len).map(move |i| {
            self.let_statement[i]
                .as_ref()
                .map(|let_statement| let_statement.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`List`] into the store.
    ///
    pub fn inter_list<F>(&mut self, list: F) -> Rc<RefCell<List>>
    where
        F: Fn(usize) -> Rc<RefCell<List>>,
    {
        if let Some(_index) = self.list_free_list.pop() {
            let list = list(_index);
            self.list[_index] = Some(list.clone());
            list
        } else {
            let _index = self.list.len();
            let list = list(_index);
            self.list.push(Some(list.clone()));
            list
        }
    }

    /// Exhume (get) [`List`] from the store.
    ///
    pub fn exhume_list(&self, id: &usize) -> Option<Rc<RefCell<List>>> {
        match self.list.get(*id) {
            Some(list) => list.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`List`] from the store.
    ///
    pub fn exorcise_list(&mut self, id: &usize) -> Option<Rc<RefCell<List>>> {
        let result = self.list[*id].take();
        self.list_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, List>`.
    ///
    pub fn iter_list(&self) -> impl Iterator<Item = Rc<RefCell<List>>> + '_ {
        let len = self.list.len();
        (0..len).map(move |i| self.list[i].as_ref().map(|list| list.clone()).unwrap())
    }

    /// Inter (insert) [`ListElement`] into the store.
    ///
    pub fn inter_list_element<F>(&mut self, list_element: F) -> Rc<RefCell<ListElement>>
    where
        F: Fn(usize) -> Rc<RefCell<ListElement>>,
    {
        if let Some(_index) = self.list_element_free_list.pop() {
            let list_element = list_element(_index);
            self.list_element[_index] = Some(list_element.clone());
            list_element
        } else {
            let _index = self.list_element.len();
            let list_element = list_element(_index);
            self.list_element.push(Some(list_element.clone()));
            list_element
        }
    }

    /// Exhume (get) [`ListElement`] from the store.
    ///
    pub fn exhume_list_element(&self, id: &usize) -> Option<Rc<RefCell<ListElement>>> {
        match self.list_element.get(*id) {
            Some(list_element) => list_element.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ListElement`] from the store.
    ///
    pub fn exorcise_list_element(&mut self, id: &usize) -> Option<Rc<RefCell<ListElement>>> {
        let result = self.list_element[*id].take();
        self.list_element_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListElement>`.
    ///
    pub fn iter_list_element(&self) -> impl Iterator<Item = Rc<RefCell<ListElement>>> + '_ {
        let len = self.list_element.len();
        (0..len).map(move |i| {
            self.list_element[i]
                .as_ref()
                .map(|list_element| list_element.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`ListExpression`] into the store.
    ///
    pub fn inter_list_expression<F>(&mut self, list_expression: F) -> Rc<RefCell<ListExpression>>
    where
        F: Fn(usize) -> Rc<RefCell<ListExpression>>,
    {
        if let Some(_index) = self.list_expression_free_list.pop() {
            let list_expression = list_expression(_index);
            self.list_expression[_index] = Some(list_expression.clone());
            list_expression
        } else {
            let _index = self.list_expression.len();
            let list_expression = list_expression(_index);
            self.list_expression.push(Some(list_expression.clone()));
            list_expression
        }
    }

    /// Exhume (get) [`ListExpression`] from the store.
    ///
    pub fn exhume_list_expression(&self, id: &usize) -> Option<Rc<RefCell<ListExpression>>> {
        match self.list_expression.get(*id) {
            Some(list_expression) => list_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ListExpression`] from the store.
    ///
    pub fn exorcise_list_expression(&mut self, id: &usize) -> Option<Rc<RefCell<ListExpression>>> {
        let result = self.list_expression[*id].take();
        self.list_expression_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ListExpression>`.
    ///
    pub fn iter_list_expression(&self) -> impl Iterator<Item = Rc<RefCell<ListExpression>>> + '_ {
        let len = self.list_expression.len();
        (0..len).map(move |i| {
            self.list_expression[i]
                .as_ref()
                .map(|list_expression| list_expression.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Literal`] into the store.
    ///
    pub fn inter_literal<F>(&mut self, literal: F) -> Rc<RefCell<Literal>>
    where
        F: Fn(usize) -> Rc<RefCell<Literal>>,
    {
        if let Some(_index) = self.literal_free_list.pop() {
            let literal = literal(_index);
            self.literal[_index] = Some(literal.clone());
            literal
        } else {
            let _index = self.literal.len();
            let literal = literal(_index);
            self.literal.push(Some(literal.clone()));
            literal
        }
    }

    /// Exhume (get) [`Literal`] from the store.
    ///
    pub fn exhume_literal(&self, id: &usize) -> Option<Rc<RefCell<Literal>>> {
        match self.literal.get(*id) {
            Some(literal) => literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Literal`] from the store.
    ///
    pub fn exorcise_literal(&mut self, id: &usize) -> Option<Rc<RefCell<Literal>>> {
        let result = self.literal[*id].take();
        self.literal_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Literal>`.
    ///
    pub fn iter_literal(&self) -> impl Iterator<Item = Rc<RefCell<Literal>>> + '_ {
        let len = self.literal.len();
        (0..len).map(move |i| {
            self.literal[i]
                .as_ref()
                .map(|literal| literal.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`LocalVariable`] into the store.
    ///
    pub fn inter_local_variable<F>(&mut self, local_variable: F) -> Rc<RefCell<LocalVariable>>
    where
        F: Fn(usize) -> Rc<RefCell<LocalVariable>>,
    {
        if let Some(_index) = self.local_variable_free_list.pop() {
            let local_variable = local_variable(_index);
            self.local_variable[_index] = Some(local_variable.clone());
            local_variable
        } else {
            let _index = self.local_variable.len();
            let local_variable = local_variable(_index);
            self.local_variable.push(Some(local_variable.clone()));
            local_variable
        }
    }

    /// Exhume (get) [`LocalVariable`] from the store.
    ///
    pub fn exhume_local_variable(&self, id: &usize) -> Option<Rc<RefCell<LocalVariable>>> {
        match self.local_variable.get(*id) {
            Some(local_variable) => local_variable.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`LocalVariable`] from the store.
    ///
    pub fn exorcise_local_variable(&mut self, id: &usize) -> Option<Rc<RefCell<LocalVariable>>> {
        let result = self.local_variable[*id].take();
        self.local_variable_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, LocalVariable>`.
    ///
    pub fn iter_local_variable(&self) -> impl Iterator<Item = Rc<RefCell<LocalVariable>>> + '_ {
        let len = self.local_variable.len();
        (0..len).map(move |i| {
            self.local_variable[i]
                .as_ref()
                .map(|local_variable| local_variable.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`XMacro`] into the store.
    ///
    pub fn inter_x_macro<F>(&mut self, x_macro: F) -> Rc<RefCell<XMacro>>
    where
        F: Fn(usize) -> Rc<RefCell<XMacro>>,
    {
        if let Some(_index) = self.x_macro_free_list.pop() {
            let x_macro = x_macro(_index);
            self.x_macro[_index] = Some(x_macro.clone());
            x_macro
        } else {
            let _index = self.x_macro.len();
            let x_macro = x_macro(_index);
            self.x_macro.push(Some(x_macro.clone()));
            x_macro
        }
    }

    /// Exhume (get) [`XMacro`] from the store.
    ///
    pub fn exhume_x_macro(&self, id: &usize) -> Option<Rc<RefCell<XMacro>>> {
        match self.x_macro.get(*id) {
            Some(x_macro) => x_macro.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XMacro`] from the store.
    ///
    pub fn exorcise_x_macro(&mut self, id: &usize) -> Option<Rc<RefCell<XMacro>>> {
        let result = self.x_macro[*id].take();
        self.x_macro_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XMacro>`.
    ///
    pub fn iter_x_macro(&self) -> impl Iterator<Item = Rc<RefCell<XMacro>>> + '_ {
        let len = self.x_macro.len();
        (0..len).map(move |i| {
            self.x_macro[i]
                .as_ref()
                .map(|x_macro| x_macro.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`MethodCall`] into the store.
    ///
    pub fn inter_method_call<F>(&mut self, method_call: F) -> Rc<RefCell<MethodCall>>
    where
        F: Fn(usize) -> Rc<RefCell<MethodCall>>,
    {
        if let Some(_index) = self.method_call_free_list.pop() {
            let method_call = method_call(_index);
            self.method_call[_index] = Some(method_call.clone());
            method_call
        } else {
            let _index = self.method_call.len();
            let method_call = method_call(_index);
            self.method_call.push(Some(method_call.clone()));
            method_call
        }
    }

    /// Exhume (get) [`MethodCall`] from the store.
    ///
    pub fn exhume_method_call(&self, id: &usize) -> Option<Rc<RefCell<MethodCall>>> {
        match self.method_call.get(*id) {
            Some(method_call) => method_call.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`MethodCall`] from the store.
    ///
    pub fn exorcise_method_call(&mut self, id: &usize) -> Option<Rc<RefCell<MethodCall>>> {
        let result = self.method_call[*id].take();
        self.method_call_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, MethodCall>`.
    ///
    pub fn iter_method_call(&self) -> impl Iterator<Item = Rc<RefCell<MethodCall>>> + '_ {
        let len = self.method_call.len();
        (0..len).map(move |i| {
            self.method_call[i]
                .as_ref()
                .map(|method_call| method_call.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`ZObjectStore`] into the store.
    ///
    pub fn inter_z_object_store<F>(&mut self, z_object_store: F) -> Rc<RefCell<ZObjectStore>>
    where
        F: Fn(usize) -> Rc<RefCell<ZObjectStore>>,
    {
        if let Some(_index) = self.z_object_store_free_list.pop() {
            let z_object_store = z_object_store(_index);
            self.z_object_store[_index] = Some(z_object_store.clone());
            z_object_store
        } else {
            let _index = self.z_object_store.len();
            let z_object_store = z_object_store(_index);
            self.z_object_store.push(Some(z_object_store.clone()));
            z_object_store
        }
    }

    /// Exhume (get) [`ZObjectStore`] from the store.
    ///
    pub fn exhume_z_object_store(&self, id: &usize) -> Option<Rc<RefCell<ZObjectStore>>> {
        match self.z_object_store.get(*id) {
            Some(z_object_store) => z_object_store.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ZObjectStore`] from the store.
    ///
    pub fn exorcise_z_object_store(&mut self, id: &usize) -> Option<Rc<RefCell<ZObjectStore>>> {
        let result = self.z_object_store[*id].take();
        self.z_object_store_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ZObjectStore>`.
    ///
    pub fn iter_z_object_store(&self) -> impl Iterator<Item = Rc<RefCell<ZObjectStore>>> + '_ {
        let len = self.z_object_store.len();
        (0..len).map(move |i| {
            self.z_object_store[i]
                .as_ref()
                .map(|z_object_store| z_object_store.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Operator`] into the store.
    ///
    pub fn inter_operator<F>(&mut self, operator: F) -> Rc<RefCell<Operator>>
    where
        F: Fn(usize) -> Rc<RefCell<Operator>>,
    {
        if let Some(_index) = self.operator_free_list.pop() {
            let operator = operator(_index);
            self.operator[_index] = Some(operator.clone());
            operator
        } else {
            let _index = self.operator.len();
            let operator = operator(_index);
            self.operator.push(Some(operator.clone()));
            operator
        }
    }

    /// Exhume (get) [`Operator`] from the store.
    ///
    pub fn exhume_operator(&self, id: &usize) -> Option<Rc<RefCell<Operator>>> {
        match self.operator.get(*id) {
            Some(operator) => operator.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Operator`] from the store.
    ///
    pub fn exorcise_operator(&mut self, id: &usize) -> Option<Rc<RefCell<Operator>>> {
        let result = self.operator[*id].take();
        self.operator_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Operator>`.
    ///
    pub fn iter_operator(&self) -> impl Iterator<Item = Rc<RefCell<Operator>>> + '_ {
        let len = self.operator.len();
        (0..len).map(move |i| {
            self.operator[i]
                .as_ref()
                .map(|operator| operator.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`WoogOption`] into the store.
    ///
    pub fn inter_woog_option<F>(&mut self, woog_option: F) -> Rc<RefCell<WoogOption>>
    where
        F: Fn(usize) -> Rc<RefCell<WoogOption>>,
    {
        if let Some(_index) = self.woog_option_free_list.pop() {
            let woog_option = woog_option(_index);
            self.woog_option[_index] = Some(woog_option.clone());
            woog_option
        } else {
            let _index = self.woog_option.len();
            let woog_option = woog_option(_index);
            self.woog_option.push(Some(woog_option.clone()));
            woog_option
        }
    }

    /// Exhume (get) [`WoogOption`] from the store.
    ///
    pub fn exhume_woog_option(&self, id: &usize) -> Option<Rc<RefCell<WoogOption>>> {
        match self.woog_option.get(*id) {
            Some(woog_option) => woog_option.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`WoogOption`] from the store.
    ///
    pub fn exorcise_woog_option(&mut self, id: &usize) -> Option<Rc<RefCell<WoogOption>>> {
        let result = self.woog_option[*id].take();
        self.woog_option_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, WoogOption>`.
    ///
    pub fn iter_woog_option(&self) -> impl Iterator<Item = Rc<RefCell<WoogOption>>> + '_ {
        let len = self.woog_option.len();
        (0..len).map(move |i| {
            self.woog_option[i]
                .as_ref()
                .map(|woog_option| woog_option.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Parameter`] into the store.
    ///
    pub fn inter_parameter<F>(&mut self, parameter: F) -> Rc<RefCell<Parameter>>
    where
        F: Fn(usize) -> Rc<RefCell<Parameter>>,
    {
        if let Some(_index) = self.parameter_free_list.pop() {
            let parameter = parameter(_index);
            self.parameter[_index] = Some(parameter.clone());
            parameter
        } else {
            let _index = self.parameter.len();
            let parameter = parameter(_index);
            self.parameter.push(Some(parameter.clone()));
            parameter
        }
    }

    /// Exhume (get) [`Parameter`] from the store.
    ///
    pub fn exhume_parameter(&self, id: &usize) -> Option<Rc<RefCell<Parameter>>> {
        match self.parameter.get(*id) {
            Some(parameter) => parameter.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Parameter`] from the store.
    ///
    pub fn exorcise_parameter(&mut self, id: &usize) -> Option<Rc<RefCell<Parameter>>> {
        let result = self.parameter[*id].take();
        self.parameter_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Parameter>`.
    ///
    pub fn iter_parameter(&self) -> impl Iterator<Item = Rc<RefCell<Parameter>>> + '_ {
        let len = self.parameter.len();
        (0..len).map(move |i| {
            self.parameter[i]
                .as_ref()
                .map(|parameter| parameter.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Print`] into the store.
    ///
    pub fn inter_print<F>(&mut self, print: F) -> Rc<RefCell<Print>>
    where
        F: Fn(usize) -> Rc<RefCell<Print>>,
    {
        if let Some(_index) = self.print_free_list.pop() {
            let print = print(_index);
            self.print[_index] = Some(print.clone());
            print
        } else {
            let _index = self.print.len();
            let print = print(_index);
            self.print.push(Some(print.clone()));
            print
        }
    }

    /// Exhume (get) [`Print`] from the store.
    ///
    pub fn exhume_print(&self, id: &usize) -> Option<Rc<RefCell<Print>>> {
        match self.print.get(*id) {
            Some(print) => print.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Print`] from the store.
    ///
    pub fn exorcise_print(&mut self, id: &usize) -> Option<Rc<RefCell<Print>>> {
        let result = self.print[*id].take();
        self.print_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Print>`.
    ///
    pub fn iter_print(&self) -> impl Iterator<Item = Rc<RefCell<Print>>> + '_ {
        let len = self.print.len();
        (0..len).map(move |i| self.print[i].as_ref().map(|print| print.clone()).unwrap())
    }

    /// Inter (insert) [`RangeExpression`] into the store.
    ///
    pub fn inter_range_expression<F>(&mut self, range_expression: F) -> Rc<RefCell<RangeExpression>>
    where
        F: Fn(usize) -> Rc<RefCell<RangeExpression>>,
    {
        if let Some(_index) = self.range_expression_free_list.pop() {
            let range_expression = range_expression(_index);
            self.range_expression[_index] = Some(range_expression.clone());
            range_expression
        } else {
            let _index = self.range_expression.len();
            let range_expression = range_expression(_index);
            self.range_expression.push(Some(range_expression.clone()));
            range_expression
        }
    }

    /// Exhume (get) [`RangeExpression`] from the store.
    ///
    pub fn exhume_range_expression(&self, id: &usize) -> Option<Rc<RefCell<RangeExpression>>> {
        match self.range_expression.get(*id) {
            Some(range_expression) => range_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`RangeExpression`] from the store.
    ///
    pub fn exorcise_range_expression(
        &mut self,
        id: &usize,
    ) -> Option<Rc<RefCell<RangeExpression>>> {
        let result = self.range_expression[*id].take();
        self.range_expression_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RangeExpression>`.
    ///
    pub fn iter_range_expression(&self) -> impl Iterator<Item = Rc<RefCell<RangeExpression>>> + '_ {
        let len = self.range_expression.len();
        (0..len).map(move |i| {
            self.range_expression[i]
                .as_ref()
                .map(|range_expression| range_expression.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Reference`] into the store.
    ///
    pub fn inter_reference<F>(&mut self, reference: F) -> Rc<RefCell<Reference>>
    where
        F: Fn(usize) -> Rc<RefCell<Reference>>,
    {
        if let Some(_index) = self.reference_free_list.pop() {
            let reference = reference(_index);
            self.reference[_index] = Some(reference.clone());
            reference
        } else {
            let _index = self.reference.len();
            let reference = reference(_index);
            self.reference.push(Some(reference.clone()));
            reference
        }
    }

    /// Exhume (get) [`Reference`] from the store.
    ///
    pub fn exhume_reference(&self, id: &usize) -> Option<Rc<RefCell<Reference>>> {
        match self.reference.get(*id) {
            Some(reference) => reference.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Reference`] from the store.
    ///
    pub fn exorcise_reference(&mut self, id: &usize) -> Option<Rc<RefCell<Reference>>> {
        let result = self.reference[*id].take();
        self.reference_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Reference>`.
    ///
    pub fn iter_reference(&self) -> impl Iterator<Item = Rc<RefCell<Reference>>> + '_ {
        let len = self.reference.len();
        (0..len).map(move |i| {
            self.reference[i]
                .as_ref()
                .map(|reference| reference.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`ResultStatement`] into the store.
    ///
    pub fn inter_result_statement<F>(&mut self, result_statement: F) -> Rc<RefCell<ResultStatement>>
    where
        F: Fn(usize) -> Rc<RefCell<ResultStatement>>,
    {
        if let Some(_index) = self.result_statement_free_list.pop() {
            let result_statement = result_statement(_index);
            self.result_statement[_index] = Some(result_statement.clone());
            result_statement
        } else {
            let _index = self.result_statement.len();
            let result_statement = result_statement(_index);
            self.result_statement.push(Some(result_statement.clone()));
            result_statement
        }
    }

    /// Exhume (get) [`ResultStatement`] from the store.
    ///
    pub fn exhume_result_statement(&self, id: &usize) -> Option<Rc<RefCell<ResultStatement>>> {
        match self.result_statement.get(*id) {
            Some(result_statement) => result_statement.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ResultStatement`] from the store.
    ///
    pub fn exorcise_result_statement(
        &mut self,
        id: &usize,
    ) -> Option<Rc<RefCell<ResultStatement>>> {
        let result = self.result_statement[*id].take();
        self.result_statement_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ResultStatement>`.
    ///
    pub fn iter_result_statement(&self) -> impl Iterator<Item = Rc<RefCell<ResultStatement>>> + '_ {
        let len = self.result_statement.len();
        (0..len).map(move |i| {
            self.result_statement[i]
                .as_ref()
                .map(|result_statement| result_statement.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`XReturn`] into the store.
    ///
    pub fn inter_x_return<F>(&mut self, x_return: F) -> Rc<RefCell<XReturn>>
    where
        F: Fn(usize) -> Rc<RefCell<XReturn>>,
    {
        if let Some(_index) = self.x_return_free_list.pop() {
            let x_return = x_return(_index);
            self.x_return[_index] = Some(x_return.clone());
            x_return
        } else {
            let _index = self.x_return.len();
            let x_return = x_return(_index);
            self.x_return.push(Some(x_return.clone()));
            x_return
        }
    }

    /// Exhume (get) [`XReturn`] from the store.
    ///
    pub fn exhume_x_return(&self, id: &usize) -> Option<Rc<RefCell<XReturn>>> {
        match self.x_return.get(*id) {
            Some(x_return) => x_return.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XReturn`] from the store.
    ///
    pub fn exorcise_x_return(&mut self, id: &usize) -> Option<Rc<RefCell<XReturn>>> {
        let result = self.x_return[*id].take();
        self.x_return_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XReturn>`.
    ///
    pub fn iter_x_return(&self) -> impl Iterator<Item = Rc<RefCell<XReturn>>> + '_ {
        let len = self.x_return.len();
        (0..len).map(move |i| {
            self.x_return[i]
                .as_ref()
                .map(|x_return| x_return.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`ZSome`] into the store.
    ///
    pub fn inter_z_some<F>(&mut self, z_some: F) -> Rc<RefCell<ZSome>>
    where
        F: Fn(usize) -> Rc<RefCell<ZSome>>,
    {
        if let Some(_index) = self.z_some_free_list.pop() {
            let z_some = z_some(_index);
            self.z_some[_index] = Some(z_some.clone());
            z_some
        } else {
            let _index = self.z_some.len();
            let z_some = z_some(_index);
            self.z_some.push(Some(z_some.clone()));
            z_some
        }
    }

    /// Exhume (get) [`ZSome`] from the store.
    ///
    pub fn exhume_z_some(&self, id: &usize) -> Option<Rc<RefCell<ZSome>>> {
        match self.z_some.get(*id) {
            Some(z_some) => z_some.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ZSome`] from the store.
    ///
    pub fn exorcise_z_some(&mut self, id: &usize) -> Option<Rc<RefCell<ZSome>>> {
        let result = self.z_some[*id].take();
        self.z_some_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ZSome>`.
    ///
    pub fn iter_z_some(&self) -> impl Iterator<Item = Rc<RefCell<ZSome>>> + '_ {
        let len = self.z_some.len();
        (0..len).map(move |i| {
            self.z_some[i]
                .as_ref()
                .map(|z_some| z_some.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Span`] into the store.
    ///
    pub fn inter_span<F>(&mut self, span: F) -> Rc<RefCell<Span>>
    where
        F: Fn(usize) -> Rc<RefCell<Span>>,
    {
        if let Some(_index) = self.span_free_list.pop() {
            let span = span(_index);
            self.span[_index] = Some(span.clone());
            span
        } else {
            let _index = self.span.len();
            let span = span(_index);
            self.span.push(Some(span.clone()));
            span
        }
    }

    /// Exhume (get) [`Span`] from the store.
    ///
    pub fn exhume_span(&self, id: &usize) -> Option<Rc<RefCell<Span>>> {
        match self.span.get(*id) {
            Some(span) => span.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Span`] from the store.
    ///
    pub fn exorcise_span(&mut self, id: &usize) -> Option<Rc<RefCell<Span>>> {
        let result = self.span[*id].take();
        self.span_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Span>`.
    ///
    pub fn iter_span(&self) -> impl Iterator<Item = Rc<RefCell<Span>>> + '_ {
        let len = self.span.len();
        (0..len).map(move |i| self.span[i].as_ref().map(|span| span.clone()).unwrap())
    }

    /// Inter (insert) [`Statement`] into the store.
    ///
    pub fn inter_statement<F>(&mut self, statement: F) -> Rc<RefCell<Statement>>
    where
        F: Fn(usize) -> Rc<RefCell<Statement>>,
    {
        if let Some(_index) = self.statement_free_list.pop() {
            let statement = statement(_index);
            self.statement[_index] = Some(statement.clone());
            statement
        } else {
            let _index = self.statement.len();
            let statement = statement(_index);
            self.statement.push(Some(statement.clone()));
            statement
        }
    }

    /// Exhume (get) [`Statement`] from the store.
    ///
    pub fn exhume_statement(&self, id: &usize) -> Option<Rc<RefCell<Statement>>> {
        match self.statement.get(*id) {
            Some(statement) => statement.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Statement`] from the store.
    ///
    pub fn exorcise_statement(&mut self, id: &usize) -> Option<Rc<RefCell<Statement>>> {
        let result = self.statement[*id].take();
        self.statement_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Statement>`.
    ///
    pub fn iter_statement(&self) -> impl Iterator<Item = Rc<RefCell<Statement>>> + '_ {
        let len = self.statement.len();
        (0..len).map(move |i| {
            self.statement[i]
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
    ) -> Rc<RefCell<StaticMethodCall>>
    where
        F: Fn(usize) -> Rc<RefCell<StaticMethodCall>>,
    {
        if let Some(_index) = self.static_method_call_free_list.pop() {
            let static_method_call = static_method_call(_index);
            self.static_method_call[_index] = Some(static_method_call.clone());
            static_method_call
        } else {
            let _index = self.static_method_call.len();
            let static_method_call = static_method_call(_index);
            self.static_method_call
                .push(Some(static_method_call.clone()));
            static_method_call
        }
    }

    /// Exhume (get) [`StaticMethodCall`] from the store.
    ///
    pub fn exhume_static_method_call(&self, id: &usize) -> Option<Rc<RefCell<StaticMethodCall>>> {
        match self.static_method_call.get(*id) {
            Some(static_method_call) => static_method_call.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`StaticMethodCall`] from the store.
    ///
    pub fn exorcise_static_method_call(
        &mut self,
        id: &usize,
    ) -> Option<Rc<RefCell<StaticMethodCall>>> {
        let result = self.static_method_call[*id].take();
        self.static_method_call_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StaticMethodCall>`.
    ///
    pub fn iter_static_method_call(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<StaticMethodCall>>> + '_ {
        let len = self.static_method_call.len();
        (0..len).map(move |i| {
            self.static_method_call[i]
                .as_ref()
                .map(|static_method_call| static_method_call.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`StringLiteral`] into the store.
    ///
    pub fn inter_string_literal<F>(&mut self, string_literal: F) -> Rc<RefCell<StringLiteral>>
    where
        F: Fn(usize) -> Rc<RefCell<StringLiteral>>,
    {
        if let Some(_index) = self.string_literal_free_list.pop() {
            let string_literal = string_literal(_index);
            self.string_literal[_index] = Some(string_literal.clone());
            string_literal
        } else {
            let _index = self.string_literal.len();
            let string_literal = string_literal(_index);
            self.string_literal.push(Some(string_literal.clone()));
            string_literal
        }
    }

    /// Exhume (get) [`StringLiteral`] from the store.
    ///
    pub fn exhume_string_literal(&self, id: &usize) -> Option<Rc<RefCell<StringLiteral>>> {
        match self.string_literal.get(*id) {
            Some(string_literal) => string_literal.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`StringLiteral`] from the store.
    ///
    pub fn exorcise_string_literal(&mut self, id: &usize) -> Option<Rc<RefCell<StringLiteral>>> {
        let result = self.string_literal[*id].take();
        self.string_literal_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StringLiteral>`.
    ///
    pub fn iter_string_literal(&self) -> impl Iterator<Item = Rc<RefCell<StringLiteral>>> + '_ {
        let len = self.string_literal.len();
        (0..len).map(move |i| {
            self.string_literal[i]
                .as_ref()
                .map(|string_literal| string_literal.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`WoogStruct`] into the store.
    ///
    pub fn inter_woog_struct<F>(&mut self, woog_struct: F) -> Rc<RefCell<WoogStruct>>
    where
        F: Fn(usize) -> Rc<RefCell<WoogStruct>>,
    {
        let woog_struct = if let Some(_index) = self.woog_struct_free_list.pop() {
            let woog_struct = woog_struct(_index);
            self.woog_struct[_index] = Some(woog_struct.clone());
            woog_struct
        } else {
            let _index = self.woog_struct.len();
            let woog_struct = woog_struct(_index);
            self.woog_struct.push(Some(woog_struct.clone()));
            woog_struct
        };
        self.woog_struct_id_by_name.insert(
            woog_struct.borrow().name.to_upper_camel_case(),
            woog_struct.borrow().id,
        );
        woog_struct
    }

    /// Exhume (get) [`WoogStruct`] from the store.
    ///
    pub fn exhume_woog_struct(&self, id: &usize) -> Option<Rc<RefCell<WoogStruct>>> {
        match self.woog_struct.get(*id) {
            Some(woog_struct) => woog_struct.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`WoogStruct`] from the store.
    ///
    pub fn exorcise_woog_struct(&mut self, id: &usize) -> Option<Rc<RefCell<WoogStruct>>> {
        let result = self.woog_struct[*id].take();
        self.woog_struct_free_list.push(*id);
        result
    }

    /// Exorcise [`WoogStruct`] id from the store by name.
    ///
    pub fn exhume_woog_struct_id_by_name(&self, name: &str) -> Option<usize> {
        self.woog_struct_id_by_name
            .get(name)
            .map(|woog_struct| *woog_struct)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, WoogStruct>`.
    ///
    pub fn iter_woog_struct(&self) -> impl Iterator<Item = Rc<RefCell<WoogStruct>>> + '_ {
        let len = self.woog_struct.len();
        (0..len).map(move |i| {
            self.woog_struct[i]
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
    ) -> Rc<RefCell<StructExpression>>
    where
        F: Fn(usize) -> Rc<RefCell<StructExpression>>,
    {
        if let Some(_index) = self.struct_expression_free_list.pop() {
            let struct_expression = struct_expression(_index);
            self.struct_expression[_index] = Some(struct_expression.clone());
            struct_expression
        } else {
            let _index = self.struct_expression.len();
            let struct_expression = struct_expression(_index);
            self.struct_expression.push(Some(struct_expression.clone()));
            struct_expression
        }
    }

    /// Exhume (get) [`StructExpression`] from the store.
    ///
    pub fn exhume_struct_expression(&self, id: &usize) -> Option<Rc<RefCell<StructExpression>>> {
        match self.struct_expression.get(*id) {
            Some(struct_expression) => struct_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`StructExpression`] from the store.
    ///
    pub fn exorcise_struct_expression(
        &mut self,
        id: &usize,
    ) -> Option<Rc<RefCell<StructExpression>>> {
        let result = self.struct_expression[*id].take();
        self.struct_expression_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, StructExpression>`.
    ///
    pub fn iter_struct_expression(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<StructExpression>>> + '_ {
        let len = self.struct_expression.len();
        (0..len).map(move |i| {
            self.struct_expression[i]
                .as_ref()
                .map(|struct_expression| struct_expression.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`TypeCast`] into the store.
    ///
    pub fn inter_type_cast<F>(&mut self, type_cast: F) -> Rc<RefCell<TypeCast>>
    where
        F: Fn(usize) -> Rc<RefCell<TypeCast>>,
    {
        if let Some(_index) = self.type_cast_free_list.pop() {
            let type_cast = type_cast(_index);
            self.type_cast[_index] = Some(type_cast.clone());
            type_cast
        } else {
            let _index = self.type_cast.len();
            let type_cast = type_cast(_index);
            self.type_cast.push(Some(type_cast.clone()));
            type_cast
        }
    }

    /// Exhume (get) [`TypeCast`] from the store.
    ///
    pub fn exhume_type_cast(&self, id: &usize) -> Option<Rc<RefCell<TypeCast>>> {
        match self.type_cast.get(*id) {
            Some(type_cast) => type_cast.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`TypeCast`] from the store.
    ///
    pub fn exorcise_type_cast(&mut self, id: &usize) -> Option<Rc<RefCell<TypeCast>>> {
        let result = self.type_cast[*id].take();
        self.type_cast_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, TypeCast>`.
    ///
    pub fn iter_type_cast(&self) -> impl Iterator<Item = Rc<RefCell<TypeCast>>> + '_ {
        let len = self.type_cast.len();
        (0..len).map(move |i| {
            self.type_cast[i]
                .as_ref()
                .map(|type_cast| type_cast.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Unary`] into the store.
    ///
    pub fn inter_unary<F>(&mut self, unary: F) -> Rc<RefCell<Unary>>
    where
        F: Fn(usize) -> Rc<RefCell<Unary>>,
    {
        if let Some(_index) = self.unary_free_list.pop() {
            let unary = unary(_index);
            self.unary[_index] = Some(unary.clone());
            unary
        } else {
            let _index = self.unary.len();
            let unary = unary(_index);
            self.unary.push(Some(unary.clone()));
            unary
        }
    }

    /// Exhume (get) [`Unary`] from the store.
    ///
    pub fn exhume_unary(&self, id: &usize) -> Option<Rc<RefCell<Unary>>> {
        match self.unary.get(*id) {
            Some(unary) => unary.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Unary`] from the store.
    ///
    pub fn exorcise_unary(&mut self, id: &usize) -> Option<Rc<RefCell<Unary>>> {
        let result = self.unary[*id].take();
        self.unary_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Unary>`.
    ///
    pub fn iter_unary(&self) -> impl Iterator<Item = Rc<RefCell<Unary>>> + '_ {
        let len = self.unary.len();
        (0..len).map(move |i| self.unary[i].as_ref().map(|unary| unary.clone()).unwrap())
    }

    /// Inter (insert) [`XValue`] into the store.
    ///
    pub fn inter_x_value<F>(&mut self, x_value: F) -> Rc<RefCell<XValue>>
    where
        F: Fn(usize) -> Rc<RefCell<XValue>>,
    {
        if let Some(_index) = self.x_value_free_list.pop() {
            let x_value = x_value(_index);
            self.x_value[_index] = Some(x_value.clone());
            x_value
        } else {
            let _index = self.x_value.len();
            let x_value = x_value(_index);
            self.x_value.push(Some(x_value.clone()));
            x_value
        }
    }

    /// Exhume (get) [`XValue`] from the store.
    ///
    pub fn exhume_x_value(&self, id: &usize) -> Option<Rc<RefCell<XValue>>> {
        match self.x_value.get(*id) {
            Some(x_value) => x_value.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`XValue`] from the store.
    ///
    pub fn exorcise_x_value(&mut self, id: &usize) -> Option<Rc<RefCell<XValue>>> {
        let result = self.x_value[*id].take();
        self.x_value_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, XValue>`.
    ///
    pub fn iter_x_value(&self) -> impl Iterator<Item = Rc<RefCell<XValue>>> + '_ {
        let len = self.x_value.len();
        (0..len).map(move |i| {
            self.x_value[i]
                .as_ref()
                .map(|x_value| x_value.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`ValueType`] into the store.
    ///
    pub fn inter_value_type<F>(&mut self, value_type: F) -> Rc<RefCell<ValueType>>
    where
        F: Fn(usize) -> Rc<RefCell<ValueType>>,
    {
        if let Some(_index) = self.value_type_free_list.pop() {
            let value_type = value_type(_index);
            self.value_type[_index] = Some(value_type.clone());
            value_type
        } else {
            let _index = self.value_type.len();
            let value_type = value_type(_index);
            self.value_type.push(Some(value_type.clone()));
            value_type
        }
    }

    /// Exhume (get) [`ValueType`] from the store.
    ///
    pub fn exhume_value_type(&self, id: &usize) -> Option<Rc<RefCell<ValueType>>> {
        match self.value_type.get(*id) {
            Some(value_type) => value_type.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`ValueType`] from the store.
    ///
    pub fn exorcise_value_type(&mut self, id: &usize) -> Option<Rc<RefCell<ValueType>>> {
        let result = self.value_type[*id].take();
        self.value_type_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ValueType>`.
    ///
    pub fn iter_value_type(&self) -> impl Iterator<Item = Rc<RefCell<ValueType>>> + '_ {
        let len = self.value_type.len();
        (0..len).map(move |i| {
            self.value_type[i]
                .as_ref()
                .map(|value_type| value_type.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Variable`] into the store.
    ///
    pub fn inter_variable<F>(&mut self, variable: F) -> Rc<RefCell<Variable>>
    where
        F: Fn(usize) -> Rc<RefCell<Variable>>,
    {
        if let Some(_index) = self.variable_free_list.pop() {
            let variable = variable(_index);
            self.variable[_index] = Some(variable.clone());
            variable
        } else {
            let _index = self.variable.len();
            let variable = variable(_index);
            self.variable.push(Some(variable.clone()));
            variable
        }
    }

    /// Exhume (get) [`Variable`] from the store.
    ///
    pub fn exhume_variable(&self, id: &usize) -> Option<Rc<RefCell<Variable>>> {
        match self.variable.get(*id) {
            Some(variable) => variable.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Variable`] from the store.
    ///
    pub fn exorcise_variable(&mut self, id: &usize) -> Option<Rc<RefCell<Variable>>> {
        let result = self.variable[*id].take();
        self.variable_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Variable>`.
    ///
    pub fn iter_variable(&self) -> impl Iterator<Item = Rc<RefCell<Variable>>> + '_ {
        let len = self.variable.len();
        (0..len).map(move |i| {
            self.variable[i]
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
    ) -> Rc<RefCell<VariableExpression>>
    where
        F: Fn(usize) -> Rc<RefCell<VariableExpression>>,
    {
        if let Some(_index) = self.variable_expression_free_list.pop() {
            let variable_expression = variable_expression(_index);
            self.variable_expression[_index] = Some(variable_expression.clone());
            variable_expression
        } else {
            let _index = self.variable_expression.len();
            let variable_expression = variable_expression(_index);
            self.variable_expression
                .push(Some(variable_expression.clone()));
            variable_expression
        }
    }

    /// Exhume (get) [`VariableExpression`] from the store.
    ///
    pub fn exhume_variable_expression(
        &self,
        id: &usize,
    ) -> Option<Rc<RefCell<VariableExpression>>> {
        match self.variable_expression.get(*id) {
            Some(variable_expression) => variable_expression.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`VariableExpression`] from the store.
    ///
    pub fn exorcise_variable_expression(
        &mut self,
        id: &usize,
    ) -> Option<Rc<RefCell<VariableExpression>>> {
        let result = self.variable_expression[*id].take();
        self.variable_expression_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, VariableExpression>`.
    ///
    pub fn iter_variable_expression(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<VariableExpression>>> + '_ {
        let len = self.variable_expression.len();
        (0..len).map(move |i| {
            self.variable_expression[i]
                .as_ref()
                .map(|variable_expression| variable_expression.clone())
                .unwrap()
        })
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::lu_dog_vec-object-store-persistence"}}}
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
            for argument in &self.argument {
                if let Some(argument) = argument {
                    let path = path.join(format!("{}.json", argument.borrow().id));
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
            for binary in &self.binary {
                if let Some(binary) = binary {
                    let path = path.join(format!("{}.json", binary.borrow().id));
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
            for block in &self.block {
                if let Some(block) = block {
                    let path = path.join(format!("{}.json", block.borrow().id));
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
            for boolean_literal in &self.boolean_literal {
                if let Some(boolean_literal) = boolean_literal {
                    let path = path.join(format!("{}.json", boolean_literal.borrow().id));
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
            for boolean_operator in &self.boolean_operator {
                if let Some(boolean_operator) = boolean_operator {
                    let path = path.join(format!("{}.json", boolean_operator.borrow().id));
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
            for call in &self.call {
                if let Some(call) = call {
                    let path = path.join(format!("{}.json", call.borrow().id));
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
            for comparison in &self.comparison {
                if let Some(comparison) = comparison {
                    let path = path.join(format!("{}.json", comparison.borrow().id));
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
            for dwarf_source_file in &self.dwarf_source_file {
                if let Some(dwarf_source_file) = dwarf_source_file {
                    let path = path.join(format!("{}.json", dwarf_source_file.borrow().id));
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
            for error in &self.error {
                if let Some(error) = error {
                    let path = path.join(format!("{}.json", error.borrow().id));
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
            for error_expression in &self.error_expression {
                if let Some(error_expression) = error_expression {
                    let path = path.join(format!("{}.json", error_expression.borrow().id));
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
            for expression in &self.expression {
                if let Some(expression) = expression {
                    let path = path.join(format!("{}.json", expression.borrow().id));
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
            for expression_statement in &self.expression_statement {
                if let Some(expression_statement) = expression_statement {
                    let path = path.join(format!("{}.json", expression_statement.borrow().id));
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
            for field in &self.field {
                if let Some(field) = field {
                    let path = path.join(format!("{}.json", field.borrow().id));
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
            for field_access in &self.field_access {
                if let Some(field_access) = field_access {
                    let path = path.join(format!("{}.json", field_access.borrow().id));
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
            for field_access_target in &self.field_access_target {
                if let Some(field_access_target) = field_access_target {
                    let path = path.join(format!("{}.json", field_access_target.borrow().id));
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
            for field_expression in &self.field_expression {
                if let Some(field_expression) = field_expression {
                    let path = path.join(format!("{}.json", field_expression.borrow().id));
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
            for float_literal in &self.float_literal {
                if let Some(float_literal) = float_literal {
                    let path = path.join(format!("{}.json", float_literal.borrow().id));
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
            for for_loop in &self.for_loop {
                if let Some(for_loop) = for_loop {
                    let path = path.join(format!("{}.json", for_loop.borrow().id));
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
            for function in &self.function {
                if let Some(function) = function {
                    let path = path.join(format!("{}.json", function.borrow().id));
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
            for grouped in &self.grouped {
                if let Some(grouped) = grouped {
                    let path = path.join(format!("{}.json", grouped.borrow().id));
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
            for x_if in &self.x_if {
                if let Some(x_if) = x_if {
                    let path = path.join(format!("{}.json", x_if.borrow().id));
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
            for implementation in &self.implementation {
                if let Some(implementation) = implementation {
                    let path = path.join(format!("{}.json", implementation.borrow().id));
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
            for import in &self.import {
                if let Some(import) = import {
                    let path = path.join(format!("{}.json", import.borrow().id));
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
            for index in &self.index {
                if let Some(index) = index {
                    let path = path.join(format!("{}.json", index.borrow().id));
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
            for integer_literal in &self.integer_literal {
                if let Some(integer_literal) = integer_literal {
                    let path = path.join(format!("{}.json", integer_literal.borrow().id));
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
            for item in &self.item {
                if let Some(item) = item {
                    let path = path.join(format!("{}.json", item.borrow().id));
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
            for lambda in &self.lambda {
                if let Some(lambda) = lambda {
                    let path = path.join(format!("{}.json", lambda.borrow().id));
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
            for lambda_parameter in &self.lambda_parameter {
                if let Some(lambda_parameter) = lambda_parameter {
                    let path = path.join(format!("{}.json", lambda_parameter.borrow().id));
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
            for let_statement in &self.let_statement {
                if let Some(let_statement) = let_statement {
                    let path = path.join(format!("{}.json", let_statement.borrow().id));
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
            for list in &self.list {
                if let Some(list) = list {
                    let path = path.join(format!("{}.json", list.borrow().id));
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
            for list_element in &self.list_element {
                if let Some(list_element) = list_element {
                    let path = path.join(format!("{}.json", list_element.borrow().id));
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
            for list_expression in &self.list_expression {
                if let Some(list_expression) = list_expression {
                    let path = path.join(format!("{}.json", list_expression.borrow().id));
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
            for literal in &self.literal {
                if let Some(literal) = literal {
                    let path = path.join(format!("{}.json", literal.borrow().id));
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
            for local_variable in &self.local_variable {
                if let Some(local_variable) = local_variable {
                    let path = path.join(format!("{}.json", local_variable.borrow().id));
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
            for x_macro in &self.x_macro {
                if let Some(x_macro) = x_macro {
                    let path = path.join(format!("{}.json", x_macro.borrow().id));
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
            for method_call in &self.method_call {
                if let Some(method_call) = method_call {
                    let path = path.join(format!("{}.json", method_call.borrow().id));
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
            for z_object_store in &self.z_object_store {
                if let Some(z_object_store) = z_object_store {
                    let path = path.join(format!("{}.json", z_object_store.borrow().id));
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
            for operator in &self.operator {
                if let Some(operator) = operator {
                    let path = path.join(format!("{}.json", operator.borrow().id));
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
            for woog_option in &self.woog_option {
                if let Some(woog_option) = woog_option {
                    let path = path.join(format!("{}.json", woog_option.borrow().id));
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
            for parameter in &self.parameter {
                if let Some(parameter) = parameter {
                    let path = path.join(format!("{}.json", parameter.borrow().id));
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
            for print in &self.print {
                if let Some(print) = print {
                    let path = path.join(format!("{}.json", print.borrow().id));
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
            for range_expression in &self.range_expression {
                if let Some(range_expression) = range_expression {
                    let path = path.join(format!("{}.json", range_expression.borrow().id));
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
            for reference in &self.reference {
                if let Some(reference) = reference {
                    let path = path.join(format!("{}.json", reference.borrow().id));
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
            for result_statement in &self.result_statement {
                if let Some(result_statement) = result_statement {
                    let path = path.join(format!("{}.json", result_statement.borrow().id));
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
            for x_return in &self.x_return {
                if let Some(x_return) = x_return {
                    let path = path.join(format!("{}.json", x_return.borrow().id));
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
            for z_some in &self.z_some {
                if let Some(z_some) = z_some {
                    let path = path.join(format!("{}.json", z_some.borrow().id));
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
            for span in &self.span {
                if let Some(span) = span {
                    let path = path.join(format!("{}.json", span.borrow().id));
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
            for statement in &self.statement {
                if let Some(statement) = statement {
                    let path = path.join(format!("{}.json", statement.borrow().id));
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
            for static_method_call in &self.static_method_call {
                if let Some(static_method_call) = static_method_call {
                    let path = path.join(format!("{}.json", static_method_call.borrow().id));
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
            for string_literal in &self.string_literal {
                if let Some(string_literal) = string_literal {
                    let path = path.join(format!("{}.json", string_literal.borrow().id));
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
            for woog_struct in &self.woog_struct {
                if let Some(woog_struct) = woog_struct {
                    let path = path.join(format!("{}.json", woog_struct.borrow().id));
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
            for struct_expression in &self.struct_expression {
                if let Some(struct_expression) = struct_expression {
                    let path = path.join(format!("{}.json", struct_expression.borrow().id));
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
            for type_cast in &self.type_cast {
                if let Some(type_cast) = type_cast {
                    let path = path.join(format!("{}.json", type_cast.borrow().id));
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
            for unary in &self.unary {
                if let Some(unary) = unary {
                    let path = path.join(format!("{}.json", unary.borrow().id));
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
            for x_value in &self.x_value {
                if let Some(x_value) = x_value {
                    let path = path.join(format!("{}.json", x_value.borrow().id));
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
            for value_type in &self.value_type {
                if let Some(value_type) = value_type {
                    let path = path.join(format!("{}.json", value_type.borrow().id));
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
            for variable in &self.variable {
                if let Some(variable) = variable {
                    let path = path.join(format!("{}.json", variable.borrow().id));
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
            for variable_expression in &self.variable_expression {
                if let Some(variable_expression) = variable_expression {
                    let path = path.join(format!("{}.json", variable_expression.borrow().id));
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
                let argument: Rc<RefCell<Argument>> = serde_json::from_reader(reader)?;
                store
                    .argument
                    .insert(argument.borrow().id, Some(argument.clone()));
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
                let binary: Rc<RefCell<Binary>> = serde_json::from_reader(reader)?;
                store
                    .binary
                    .insert(binary.borrow().id, Some(binary.clone()));
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
                let block: Rc<RefCell<Block>> = serde_json::from_reader(reader)?;
                store.block.insert(block.borrow().id, Some(block.clone()));
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
                let boolean_literal: Rc<RefCell<BooleanLiteral>> = serde_json::from_reader(reader)?;
                store
                    .boolean_literal
                    .insert(boolean_literal.borrow().id, Some(boolean_literal.clone()));
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
                let boolean_operator: Rc<RefCell<BooleanOperator>> =
                    serde_json::from_reader(reader)?;
                store
                    .boolean_operator
                    .insert(boolean_operator.borrow().id, Some(boolean_operator.clone()));
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
                let call: Rc<RefCell<Call>> = serde_json::from_reader(reader)?;
                store.call.insert(call.borrow().id, Some(call.clone()));
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
                let comparison: Rc<RefCell<Comparison>> = serde_json::from_reader(reader)?;
                store
                    .comparison
                    .insert(comparison.borrow().id, Some(comparison.clone()));
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
                let dwarf_source_file: Rc<RefCell<DwarfSourceFile>> =
                    serde_json::from_reader(reader)?;
                store.dwarf_source_file.insert(
                    dwarf_source_file.borrow().id,
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
                let error: Rc<RefCell<Error>> = serde_json::from_reader(reader)?;
                store.error.insert(error.borrow().id, Some(error.clone()));
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
                let error_expression: Rc<RefCell<ErrorExpression>> =
                    serde_json::from_reader(reader)?;
                store
                    .error_expression
                    .insert(error_expression.borrow().id, Some(error_expression.clone()));
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
                let expression: Rc<RefCell<Expression>> = serde_json::from_reader(reader)?;
                store
                    .expression
                    .insert(expression.borrow().id, Some(expression.clone()));
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
                let expression_statement: Rc<RefCell<ExpressionStatement>> =
                    serde_json::from_reader(reader)?;
                store.expression_statement.insert(
                    expression_statement.borrow().id,
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
                let field: Rc<RefCell<Field>> = serde_json::from_reader(reader)?;
                store
                    .field_id_by_name
                    .insert(field.borrow().name.to_upper_camel_case(), field.borrow().id);
                store.field.insert(field.borrow().id, Some(field.clone()));
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
                let field_access: Rc<RefCell<FieldAccess>> = serde_json::from_reader(reader)?;
                store
                    .field_access
                    .insert(field_access.borrow().id, Some(field_access.clone()));
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
                let field_access_target: Rc<RefCell<FieldAccessTarget>> =
                    serde_json::from_reader(reader)?;
                store.field_access_target.insert(
                    field_access_target.borrow().id,
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
                let field_expression: Rc<RefCell<FieldExpression>> =
                    serde_json::from_reader(reader)?;
                store
                    .field_expression
                    .insert(field_expression.borrow().id, Some(field_expression.clone()));
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
                let float_literal: Rc<RefCell<FloatLiteral>> = serde_json::from_reader(reader)?;
                store
                    .float_literal
                    .insert(float_literal.borrow().id, Some(float_literal.clone()));
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
                let for_loop: Rc<RefCell<ForLoop>> = serde_json::from_reader(reader)?;
                store
                    .for_loop
                    .insert(for_loop.borrow().id, Some(for_loop.clone()));
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
                let function: Rc<RefCell<Function>> = serde_json::from_reader(reader)?;
                store.function_id_by_name.insert(
                    function.borrow().name.to_upper_camel_case(),
                    function.borrow().id,
                );
                store
                    .function
                    .insert(function.borrow().id, Some(function.clone()));
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
                let grouped: Rc<RefCell<Grouped>> = serde_json::from_reader(reader)?;
                store
                    .grouped
                    .insert(grouped.borrow().id, Some(grouped.clone()));
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
                let x_if: Rc<RefCell<XIf>> = serde_json::from_reader(reader)?;
                store.x_if.insert(x_if.borrow().id, Some(x_if.clone()));
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
                let implementation: Rc<RefCell<Implementation>> = serde_json::from_reader(reader)?;
                store
                    .implementation
                    .insert(implementation.borrow().id, Some(implementation.clone()));
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
                let import: Rc<RefCell<Import>> = serde_json::from_reader(reader)?;
                store
                    .import
                    .insert(import.borrow().id, Some(import.clone()));
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
                let index: Rc<RefCell<Index>> = serde_json::from_reader(reader)?;
                store.index.insert(index.borrow().id, Some(index.clone()));
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
                let integer_literal: Rc<RefCell<IntegerLiteral>> = serde_json::from_reader(reader)?;
                store
                    .integer_literal
                    .insert(integer_literal.borrow().id, Some(integer_literal.clone()));
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
                let item: Rc<RefCell<Item>> = serde_json::from_reader(reader)?;
                store.item.insert(item.borrow().id, Some(item.clone()));
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
                let lambda: Rc<RefCell<Lambda>> = serde_json::from_reader(reader)?;
                store
                    .lambda
                    .insert(lambda.borrow().id, Some(lambda.clone()));
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
                let lambda_parameter: Rc<RefCell<LambdaParameter>> =
                    serde_json::from_reader(reader)?;
                store
                    .lambda_parameter
                    .insert(lambda_parameter.borrow().id, Some(lambda_parameter.clone()));
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
                let let_statement: Rc<RefCell<LetStatement>> = serde_json::from_reader(reader)?;
                store
                    .let_statement
                    .insert(let_statement.borrow().id, Some(let_statement.clone()));
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
                let list: Rc<RefCell<List>> = serde_json::from_reader(reader)?;
                store.list.insert(list.borrow().id, Some(list.clone()));
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
                let list_element: Rc<RefCell<ListElement>> = serde_json::from_reader(reader)?;
                store
                    .list_element
                    .insert(list_element.borrow().id, Some(list_element.clone()));
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
                let list_expression: Rc<RefCell<ListExpression>> = serde_json::from_reader(reader)?;
                store
                    .list_expression
                    .insert(list_expression.borrow().id, Some(list_expression.clone()));
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
                let literal: Rc<RefCell<Literal>> = serde_json::from_reader(reader)?;
                store
                    .literal
                    .insert(literal.borrow().id, Some(literal.clone()));
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
                let local_variable: Rc<RefCell<LocalVariable>> = serde_json::from_reader(reader)?;
                store
                    .local_variable
                    .insert(local_variable.borrow().id, Some(local_variable.clone()));
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
                let x_macro: Rc<RefCell<XMacro>> = serde_json::from_reader(reader)?;
                store
                    .x_macro
                    .insert(x_macro.borrow().id, Some(x_macro.clone()));
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
                let method_call: Rc<RefCell<MethodCall>> = serde_json::from_reader(reader)?;
                store
                    .method_call
                    .insert(method_call.borrow().id, Some(method_call.clone()));
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
                let z_object_store: Rc<RefCell<ZObjectStore>> = serde_json::from_reader(reader)?;
                store
                    .z_object_store
                    .insert(z_object_store.borrow().id, Some(z_object_store.clone()));
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
                let operator: Rc<RefCell<Operator>> = serde_json::from_reader(reader)?;
                store
                    .operator
                    .insert(operator.borrow().id, Some(operator.clone()));
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
                let woog_option: Rc<RefCell<WoogOption>> = serde_json::from_reader(reader)?;
                store
                    .woog_option
                    .insert(woog_option.borrow().id, Some(woog_option.clone()));
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
                let parameter: Rc<RefCell<Parameter>> = serde_json::from_reader(reader)?;
                store
                    .parameter
                    .insert(parameter.borrow().id, Some(parameter.clone()));
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
                let print: Rc<RefCell<Print>> = serde_json::from_reader(reader)?;
                store.print.insert(print.borrow().id, Some(print.clone()));
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
                let range_expression: Rc<RefCell<RangeExpression>> =
                    serde_json::from_reader(reader)?;
                store
                    .range_expression
                    .insert(range_expression.borrow().id, Some(range_expression.clone()));
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
                let reference: Rc<RefCell<Reference>> = serde_json::from_reader(reader)?;
                store
                    .reference
                    .insert(reference.borrow().id, Some(reference.clone()));
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
                let result_statement: Rc<RefCell<ResultStatement>> =
                    serde_json::from_reader(reader)?;
                store
                    .result_statement
                    .insert(result_statement.borrow().id, Some(result_statement.clone()));
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
                let x_return: Rc<RefCell<XReturn>> = serde_json::from_reader(reader)?;
                store
                    .x_return
                    .insert(x_return.borrow().id, Some(x_return.clone()));
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
                let z_some: Rc<RefCell<ZSome>> = serde_json::from_reader(reader)?;
                store
                    .z_some
                    .insert(z_some.borrow().id, Some(z_some.clone()));
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
                let span: Rc<RefCell<Span>> = serde_json::from_reader(reader)?;
                store.span.insert(span.borrow().id, Some(span.clone()));
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
                let statement: Rc<RefCell<Statement>> = serde_json::from_reader(reader)?;
                store
                    .statement
                    .insert(statement.borrow().id, Some(statement.clone()));
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
                let static_method_call: Rc<RefCell<StaticMethodCall>> =
                    serde_json::from_reader(reader)?;
                store.static_method_call.insert(
                    static_method_call.borrow().id,
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
                let string_literal: Rc<RefCell<StringLiteral>> = serde_json::from_reader(reader)?;
                store
                    .string_literal
                    .insert(string_literal.borrow().id, Some(string_literal.clone()));
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
                let woog_struct: Rc<RefCell<WoogStruct>> = serde_json::from_reader(reader)?;
                store.woog_struct_id_by_name.insert(
                    woog_struct.borrow().name.to_upper_camel_case(),
                    woog_struct.borrow().id,
                );
                store
                    .woog_struct
                    .insert(woog_struct.borrow().id, Some(woog_struct.clone()));
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
                let struct_expression: Rc<RefCell<StructExpression>> =
                    serde_json::from_reader(reader)?;
                store.struct_expression.insert(
                    struct_expression.borrow().id,
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
                let type_cast: Rc<RefCell<TypeCast>> = serde_json::from_reader(reader)?;
                store
                    .type_cast
                    .insert(type_cast.borrow().id, Some(type_cast.clone()));
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
                let unary: Rc<RefCell<Unary>> = serde_json::from_reader(reader)?;
                store.unary.insert(unary.borrow().id, Some(unary.clone()));
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
                let x_value: Rc<RefCell<XValue>> = serde_json::from_reader(reader)?;
                store
                    .x_value
                    .insert(x_value.borrow().id, Some(x_value.clone()));
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
                let value_type: Rc<RefCell<ValueType>> = serde_json::from_reader(reader)?;
                store
                    .value_type
                    .insert(value_type.borrow().id, Some(value_type.clone()));
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
                let variable: Rc<RefCell<Variable>> = serde_json::from_reader(reader)?;
                store
                    .variable
                    .insert(variable.borrow().id, Some(variable.clone()));
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
                let variable_expression: Rc<RefCell<VariableExpression>> =
                    serde_json::from_reader(reader)?;
                store.variable_expression.insert(
                    variable_expression.borrow().id,
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
