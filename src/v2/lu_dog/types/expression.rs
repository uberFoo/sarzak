// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-use-statements"}}}
use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
use crate::v2::lu_dog::types::argument::Argument;
use crate::v2::lu_dog::types::block::Block;
use crate::v2::lu_dog::types::call::Call;
use crate::v2::lu_dog::types::error_expression::ErrorExpression;
use crate::v2::lu_dog::types::expression_statement::ExpressionStatement;
use crate::v2::lu_dog::types::field_access::FieldAccess;
use crate::v2::lu_dog::types::let_statement::LetStatement;
use crate::v2::lu_dog::types::literal::Literal;
use crate::v2::lu_dog::types::print::Print;
use crate::v2::lu_dog::types::struct_expression::StructExpression;
use crate::v2::lu_dog::types::value::Value;
use crate::v2::lu_dog::types::variable_expression::VariableExpression;
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
    Argument(Uuid),
    Block(Uuid),
    Call(Uuid),
    ErrorExpression(Uuid),
    FieldAccess(Uuid),
    Literal(Uuid),
    Print(Uuid),
    StructExpression(Uuid),
    VariableExpression(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-implementation"}}}
impl Expression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-new-impl"}}}
    /// Create a new instance of Expression::Argument
    pub fn new_argument(argument: &Argument, store: &mut LuDogStore) -> Self {
        let new = Self::Argument(argument.id);
        store.inter_expression(new.clone());
        new
    }

    pub fn new_argument_(argument: &Argument) -> Self {
        let new = Self::Argument(argument.id);
        new
    }

    /// Create a new instance of Expression::Block
    pub fn new_block(block: &Block, store: &mut LuDogStore) -> Self {
        let new = Self::Block(block.id);
        store.inter_expression(new.clone());
        new
    }

    pub fn new_block_(block: &Block) -> Self {
        let new = Self::Block(block.id);
        new
    }

    /// Create a new instance of Expression::Call
    pub fn new_call(call: &Call, store: &mut LuDogStore) -> Self {
        let new = Self::Call(call.id);
        store.inter_expression(new.clone());
        new
    }

    pub fn new_call_(call: &Call) -> Self {
        let new = Self::Call(call.id);
        new
    }

    /// Create a new instance of Expression::ErrorExpression
    pub fn new_error_expression(
        error_expression: &ErrorExpression,
        store: &mut LuDogStore,
    ) -> Self {
        let new = Self::ErrorExpression(error_expression.id);
        store.inter_expression(new.clone());
        new
    }

    pub fn new_error_expression_(error_expression: &ErrorExpression) -> Self {
        let new = Self::ErrorExpression(error_expression.id);
        new
    }

    /// Create a new instance of Expression::FieldAccess
    pub fn new_field_access(field_access: &FieldAccess, store: &mut LuDogStore) -> Self {
        let new = Self::FieldAccess(field_access.id);
        store.inter_expression(new.clone());
        new
    }

    pub fn new_field_access_(field_access: &FieldAccess) -> Self {
        let new = Self::FieldAccess(field_access.id);
        new
    }

    /// Create a new instance of Expression::Literal
    pub fn new_literal(literal: &Literal, store: &mut LuDogStore) -> Self {
        let new = Self::Literal(literal.id());
        store.inter_expression(new.clone());
        new
    }

    pub fn new_literal_(literal: &Literal) -> Self {
        let new = Self::Literal(literal.id());
        new
    }

    /// Create a new instance of Expression::Print
    pub fn new_print(print: &Print, store: &mut LuDogStore) -> Self {
        let new = Self::Print(print.id);
        store.inter_expression(new.clone());
        new
    }

    pub fn new_print_(print: &Print) -> Self {
        let new = Self::Print(print.id);
        new
    }

    /// Create a new instance of Expression::StructExpression
    pub fn new_struct_expression(
        struct_expression: &StructExpression,
        store: &mut LuDogStore,
    ) -> Self {
        let new = Self::StructExpression(struct_expression.id);
        store.inter_expression(new.clone());
        new
    }

    pub fn new_struct_expression_(struct_expression: &StructExpression) -> Self {
        let new = Self::StructExpression(struct_expression.id);
        new
    }

    /// Create a new instance of Expression::VariableExpression
    pub fn new_variable_expression(
        variable_expression: &VariableExpression,
        store: &mut LuDogStore,
    ) -> Self {
        let new = Self::VariableExpression(variable_expression.id);
        store.inter_expression(new.clone());
        new
    }

    pub fn new_variable_expression_(variable_expression: &VariableExpression) -> Self {
        let new = Self::VariableExpression(variable_expression.id);
        new
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Expression::Argument(id) => *id,
            Expression::Block(id) => *id,
            Expression::Call(id) => *id,
            Expression::ErrorExpression(id) => *id,
            Expression::FieldAccess(id) => *id,
            Expression::Literal(id) => *id,
            Expression::Print(id) => *id,
            Expression::StructExpression(id) => *id,
            Expression::VariableExpression(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-call"}}}
    /// Navigate to [`Call`] across R29(1-Mc)
    pub fn r29_call<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Call> {
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
        store: &'a LuDogStore,
    ) -> Vec<&ExpressionStatement> {
        store
            .iter_expression_statement()
            .filter_map(|expression_statement| {
                if expression_statement.expression == self.id() {
                    Some(expression_statement)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-field_access"}}}
    /// Navigate to [`FieldAccess`] across R27(1-M)
    pub fn r27_field_access<'a>(&'a self, store: &'a LuDogStore) -> Vec<&FieldAccess> {
        store
            .iter_field_access()
            .filter_map(|field_access| {
                if field_access.expression == self.id() {
                    Some(field_access)
                // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
                // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-function_call"}}}
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-cond-to-let_statement"}}}
    /// Navigate to [`LetStatement`] across R20(1-1c)
    pub fn r20c_let_statement<'a>(&'a self, store: &'a LuDogStore) -> Vec<&LetStatement> {
        let let_statement = store
            .iter_let_statement()
            .find(|let_statement| let_statement.expression == self.id());
        match let_statement {
            Some(ref let_statement) => vec![let_statement],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-print"}}}
    /// Navigate to [`Print`] across R32(1-M)
    pub fn r32_print<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Print> {
        store
            .iter_print()
            .filter_map(|print| {
                if print.expression == self.id() {
                    Some(print)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-impl-nav-subtype-to-supertype-value"}}}
    // Navigate to [`Value`] across R11(isa)
    pub fn r11_value<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Value> {
        vec![store.exhume_value(&self.id()).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
