// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-use-statements"}}}
use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
use crate::v2::lu_dog_vanilla::types::argument::Argument;
use crate::v2::lu_dog_vanilla::types::block::Block;
use crate::v2::lu_dog_vanilla::types::call::Call;
use crate::v2::lu_dog_vanilla::types::debugger::DEBUGGER;
use crate::v2::lu_dog_vanilla::types::error_expression::ErrorExpression;
use crate::v2::lu_dog_vanilla::types::expression_statement::ExpressionStatement;
use crate::v2::lu_dog_vanilla::types::field_access::FieldAccess;
use crate::v2::lu_dog_vanilla::types::field_expression::FieldExpression;
use crate::v2::lu_dog_vanilla::types::for_loop::ForLoop;
use crate::v2::lu_dog_vanilla::types::grouped::Grouped;
use crate::v2::lu_dog_vanilla::types::index::Index;
use crate::v2::lu_dog_vanilla::types::let_statement::LetStatement;
use crate::v2::lu_dog_vanilla::types::list_element::ListElement;
use crate::v2::lu_dog_vanilla::types::list_expression::ListExpression;
use crate::v2::lu_dog_vanilla::types::literal::Literal;
use crate::v2::lu_dog_vanilla::types::operator::Operator;
use crate::v2::lu_dog_vanilla::types::print::Print;
use crate::v2::lu_dog_vanilla::types::range_expression::RangeExpression;
use crate::v2::lu_dog_vanilla::types::result_statement::ResultStatement;
use crate::v2::lu_dog_vanilla::types::struct_expression::StructExpression;
use crate::v2::lu_dog_vanilla::types::type_cast::TypeCast;
use crate::v2::lu_dog_vanilla::types::variable_expression::VariableExpression;
use crate::v2::lu_dog_vanilla::types::x_if::XIf;
use crate::v2::lu_dog_vanilla::types::x_return::XReturn;
use crate::v2::lu_dog_vanilla::types::x_value::XValue;
use crate::v2::lu_dog_vanilla::types::x_value::XValueEnum;
use crate::v2::lu_dog_vanilla::types::z_none::Z_NONE;
use crate::v2::lu_dog_vanilla::types::z_some::ZSome;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-enum-documentation"}}}
/// An Expression
///
/// Expressions are calculations that render values.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Expression {
    Block(Uuid),
    Call(Uuid),
    Debugger(Uuid),
    ErrorExpression(Uuid),
    FieldAccess(Uuid),
    FieldExpression(Uuid),
    ForLoop(Uuid),
    Grouped(Uuid),
    XIf(Uuid),
    Index(Uuid),
    ListElement(Uuid),
    ListExpression(Uuid),
    Literal(Uuid),
    ZNone(Uuid),
    Operator(Uuid),
    Print(Uuid),
    RangeExpression(Uuid),
    XReturn(Uuid),
    ZSome(Uuid),
    StructExpression(Uuid),
    TypeCast(Uuid),
    VariableExpression(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-implementation"}}}
impl Expression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-new-impl"}}}
    /// Create a new instance of Expression::Block
    pub fn new_block(block: &Block, store: &mut LuDogVanillaStore) -> Self {
        let new = Self::Block(block.id);
        store.inter_expression(new.clone());
        new
    }

    /// Create a new instance of Expression::Call
    pub fn new_call(call: &Call, store: &mut LuDogVanillaStore) -> Self {
        let new = Self::Call(call.id);
        store.inter_expression(new.clone());
        new
    }

    /// Create a new instance of Expression::Debugger
    pub fn new_debugger() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Debugger(DEBUGGER)
    }

    /// Create a new instance of Expression::ErrorExpression
    pub fn new_error_expression(
        error_expression: &ErrorExpression,
        store: &mut LuDogVanillaStore,
    ) -> Self {
        let new = Self::ErrorExpression(error_expression.id);
        store.inter_expression(new.clone());
        new
    }

    /// Create a new instance of Expression::FieldAccess
    pub fn new_field_access(field_access: &FieldAccess, store: &mut LuDogVanillaStore) -> Self {
        let new = Self::FieldAccess(field_access.id);
        store.inter_expression(new.clone());
        new
    }

    /// Create a new instance of Expression::FieldExpression
    pub fn new_field_expression(
        field_expression: &FieldExpression,
        store: &mut LuDogVanillaStore,
    ) -> Self {
        let new = Self::FieldExpression(field_expression.id);
        store.inter_expression(new.clone());
        new
    }

    /// Create a new instance of Expression::ForLoop
    pub fn new_for_loop(for_loop: &ForLoop, store: &mut LuDogVanillaStore) -> Self {
        let new = Self::ForLoop(for_loop.id);
        store.inter_expression(new.clone());
        new
    }

    /// Create a new instance of Expression::Grouped
    pub fn new_grouped(grouped: &Grouped, store: &mut LuDogVanillaStore) -> Self {
        let new = Self::Grouped(grouped.id);
        store.inter_expression(new.clone());
        new
    }

    /// Create a new instance of Expression::XIf
    pub fn new_x_if(x_if: &XIf, store: &mut LuDogVanillaStore) -> Self {
        let new = Self::XIf(x_if.id);
        store.inter_expression(new.clone());
        new
    }

    /// Create a new instance of Expression::Index
    pub fn new_index(index: &Index, store: &mut LuDogVanillaStore) -> Self {
        let new = Self::Index(index.id);
        store.inter_expression(new.clone());
        new
    }

    /// Create a new instance of Expression::ListElement
    pub fn new_list_element(list_element: &ListElement, store: &mut LuDogVanillaStore) -> Self {
        let new = Self::ListElement(list_element.id);
        store.inter_expression(new.clone());
        new
    }

    /// Create a new instance of Expression::ListExpression
    pub fn new_list_expression(
        list_expression: &ListExpression,
        store: &mut LuDogVanillaStore,
    ) -> Self {
        let new = Self::ListExpression(list_expression.id);
        store.inter_expression(new.clone());
        new
    }

    /// Create a new instance of Expression::Literal
    pub fn new_literal(literal: &Literal, store: &mut LuDogVanillaStore) -> Self {
        let new = Self::Literal(literal.id());
        store.inter_expression(new.clone());
        new
    }

    /// Create a new instance of Expression::ZNone
    pub fn new_z_none() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::ZNone(Z_NONE)
    }

    /// Create a new instance of Expression::Operator
    pub fn new_operator(operator: &Operator, store: &mut LuDogVanillaStore) -> Self {
        let new = Self::Operator(operator.id);
        store.inter_expression(new.clone());
        new
    }

    /// Create a new instance of Expression::Print
    pub fn new_print(print: &Print, store: &mut LuDogVanillaStore) -> Self {
        let new = Self::Print(print.id);
        store.inter_expression(new.clone());
        new
    }

    /// Create a new instance of Expression::RangeExpression
    pub fn new_range_expression(
        range_expression: &RangeExpression,
        store: &mut LuDogVanillaStore,
    ) -> Self {
        let new = Self::RangeExpression(range_expression.id);
        store.inter_expression(new.clone());
        new
    }

    /// Create a new instance of Expression::XReturn
    pub fn new_x_return(x_return: &XReturn, store: &mut LuDogVanillaStore) -> Self {
        let new = Self::XReturn(x_return.id);
        store.inter_expression(new.clone());
        new
    }

    /// Create a new instance of Expression::ZSome
    pub fn new_z_some(z_some: &ZSome, store: &mut LuDogVanillaStore) -> Self {
        let new = Self::ZSome(z_some.id);
        store.inter_expression(new.clone());
        new
    }

    /// Create a new instance of Expression::StructExpression
    pub fn new_struct_expression(
        struct_expression: &StructExpression,
        store: &mut LuDogVanillaStore,
    ) -> Self {
        let new = Self::StructExpression(struct_expression.id);
        store.inter_expression(new.clone());
        new
    }

    /// Create a new instance of Expression::TypeCast
    pub fn new_type_cast(type_cast: &TypeCast, store: &mut LuDogVanillaStore) -> Self {
        let new = Self::TypeCast(type_cast.id);
        store.inter_expression(new.clone());
        new
    }

    /// Create a new instance of Expression::VariableExpression
    pub fn new_variable_expression(
        variable_expression: &VariableExpression,
        store: &mut LuDogVanillaStore,
    ) -> Self {
        let new = Self::VariableExpression(variable_expression.id);
        store.inter_expression(new.clone());
        new
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Expression::Block(id) => *id,
            Expression::Call(id) => *id,
            Expression::Debugger(id) => *id,
            Expression::ErrorExpression(id) => *id,
            Expression::FieldAccess(id) => *id,
            Expression::FieldExpression(id) => *id,
            Expression::ForLoop(id) => *id,
            Expression::Grouped(id) => *id,
            Expression::XIf(id) => *id,
            Expression::Index(id) => *id,
            Expression::ListElement(id) => *id,
            Expression::ListExpression(id) => *id,
            Expression::Literal(id) => *id,
            Expression::ZNone(id) => *id,
            Expression::Operator(id) => *id,
            Expression::Print(id) => *id,
            Expression::RangeExpression(id) => *id,
            Expression::XReturn(id) => *id,
            Expression::ZSome(id) => *id,
            Expression::StructExpression(id) => *id,
            Expression::TypeCast(id) => *id,
            Expression::VariableExpression(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-argument"}}}
    /// Navigate to [`Argument`] across R37(1-M)
    pub fn r37_argument<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Argument> {
        store
            .iter_argument()
            .filter(|argument| argument.expression == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-call"}}}
    /// Navigate to [`Call`] across R29(1-Mc)
    pub fn r29_call<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Call> {
        store
            .iter_call()
            .filter_map(|call| {
                if call.expression == Some(self.id()) {
                    Some(call)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-expression_statement"}}}
    /// Navigate to [`ExpressionStatement`] across R31(1-M)
    pub fn r31_expression_statement<'a>(
        &'a self,
        store: &'a LuDogVanillaStore,
    ) -> Vec<&ExpressionStatement> {
        store
            .iter_expression_statement()
            .filter(|expression_statement| expression_statement.expression == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-field_access"}}}
    /// Navigate to [`FieldAccess`] across R27(1-M)
    pub fn r27_field_access<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&FieldAccess> {
        store
            .iter_field_access()
            .filter(|field_access| field_access.expression == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-field_expression"}}}
    /// Navigate to [`FieldExpression`] across R38(1-M)
    pub fn r38_field_expression<'a>(
        &'a self,
        store: &'a LuDogVanillaStore,
    ) -> Vec<&FieldExpression> {
        store
            .iter_field_expression()
            .filter(|field_expression| field_expression.expression == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-for_loop"}}}
    /// Navigate to [`ForLoop`] across R42(1-M)
    pub fn r42_for_loop<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&ForLoop> {
        store
            .iter_for_loop()
            .filter(|for_loop| for_loop.expression == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-grouped"}}}
    /// Navigate to [`Grouped`] across R61(1-M)
    pub fn r61_grouped<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Grouped> {
        store
            .iter_grouped()
            .filter(|grouped| grouped.expression == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-x_if"}}}
    /// Navigate to [`XIf`] across R44(1-M)
    pub fn r44_x_if<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&XIf> {
        store
            .iter_x_if()
            .filter(|x_if| x_if.test == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-index"}}}
    /// Navigate to [`Index`] across R56(1-M)
    pub fn r56_index<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Index> {
        store
            .iter_index()
            .filter(|index| index.index == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-index"}}}
    /// Navigate to [`Index`] across R57(1-M)
    pub fn r57_index<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Index> {
        store
            .iter_index()
            .filter(|index| index.target == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-cond-to-let_statement"}}}
    /// Navigate to [`LetStatement`] across R20(1-1c)
    pub fn r20c_let_statement<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&LetStatement> {
        let let_statement = store
            .iter_let_statement()
            .find(|let_statement| let_statement.expression == self.id());
        match let_statement {
            Some(let_statement) => vec![let_statement],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-list_element"}}}
    /// Navigate to [`ListElement`] across R55(1-M)
    pub fn r55_list_element<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&ListElement> {
        store
            .iter_list_element()
            .filter(|list_element| list_element.expression == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-operator"}}}
    /// Navigate to [`Operator`] across R50(1-M)
    pub fn r50_operator<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Operator> {
        store
            .iter_operator()
            .filter(|operator| operator.lhs == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-operator"}}}
    /// Navigate to [`Operator`] across R51(1-Mc)
    pub fn r51_operator<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Operator> {
        store
            .iter_operator()
            .filter_map(|operator| {
                if operator.rhs == Some(self.id()) {
                    Some(operator)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-print"}}}
    /// Navigate to [`Print`] across R32(1-M)
    pub fn r32_print<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Print> {
        store
            .iter_print()
            .filter(|print| print.expression == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-range_expression"}}}
    /// Navigate to [`RangeExpression`] across R59(1-Mc)
    pub fn r59_range_expression<'a>(
        &'a self,
        store: &'a LuDogVanillaStore,
    ) -> Vec<&RangeExpression> {
        store
            .iter_range_expression()
            .filter_map(|range_expression| {
                if range_expression.rhs == Some(self.id()) {
                    Some(range_expression)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-range_expression"}}}
    /// Navigate to [`RangeExpression`] across R58(1-Mc)
    pub fn r58_range_expression<'a>(
        &'a self,
        store: &'a LuDogVanillaStore,
    ) -> Vec<&RangeExpression> {
        store
            .iter_range_expression()
            .filter_map(|range_expression| {
                if range_expression.lhs == Some(self.id()) {
                    Some(range_expression)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-result_statement"}}}
    /// Navigate to [`ResultStatement`] across R41(1-M)
    pub fn r41_result_statement<'a>(
        &'a self,
        store: &'a LuDogVanillaStore,
    ) -> Vec<&ResultStatement> {
        store
            .iter_result_statement()
            .filter(|result_statement| result_statement.expression == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-x_return"}}}
    /// Navigate to [`XReturn`] across R45(1-M)
    pub fn r45_x_return<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&XReturn> {
        store
            .iter_x_return()
            .filter(|x_return| x_return.expression == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-type_cast"}}}
    /// Navigate to [`TypeCast`] across R68(1-M)
    pub fn r68_type_cast<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&TypeCast> {
        store
            .iter_type_cast()
            .filter(|type_cast| type_cast.lhs == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-impl-nav-subtype-to-supertype-x_value"}}}
    // Navigate to [`XValue`] across R11(isa)
    pub fn r11_x_value<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&XValue> {
        vec![store
            .iter_x_value()
            .find(|x_value| {
                if let XValueEnum::Expression(id) = x_value.subtype {
                    id == self.id()
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
