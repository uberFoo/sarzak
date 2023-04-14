// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-use-statements"}}}
use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
use crate::v2::lu_dog::types::argument::Argument;
use crate::v2::lu_dog::types::block::Block;
use crate::v2::lu_dog::types::error::Error;
use crate::v2::lu_dog::types::field_access::FieldAccess;
use crate::v2::lu_dog::types::function_call::FunctionCall;
use crate::v2::lu_dog::types::let_statement::LetStatement;
use crate::v2::lu_dog::types::literal::Literal;
use crate::v2::lu_dog::types::print::PRINT;
use crate::v2::lu_dog::types::struct_expression::StructExpression;
use crate::v2::lu_dog::types::value::Value;
use crate::v2::lu_dog::types::variable_expression::VARIABLE_EXPRESSION;
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
    Error(Uuid),
    FieldAccess(Uuid),
    FunctionCall(Uuid),
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

    /// Create a new instance of Expression::Error
    pub fn new_error(error: &Error, store: &mut LuDogStore) -> Self {
        let new = Self::Error(error.id);
        store.inter_expression(new.clone());
        new
    }

    pub fn new_error_(error: &Error) -> Self {
        let new = Self::Error(error.id);
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

    /// Create a new instance of Expression::FunctionCall
    pub fn new_function_call(function_call: &FunctionCall, store: &mut LuDogStore) -> Self {
        let new = Self::FunctionCall(function_call.id);
        store.inter_expression(new.clone());
        new
    }

    pub fn new_function_call_(function_call: &FunctionCall) -> Self {
        let new = Self::FunctionCall(function_call.id);
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
    pub fn new_print() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Print(PRINT)
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
    pub fn new_variable_expression() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::VariableExpression(VARIABLE_EXPRESSION)
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Expression::Argument(id) => *id,
            Expression::Block(id) => *id,
            Expression::Error(id) => *id,
            Expression::FieldAccess(id) => *id,
            Expression::FunctionCall(id) => *id,
            Expression::Literal(id) => *id,
            Expression::Print(id) => *id,
            Expression::StructExpression(id) => *id,
            Expression::VariableExpression(id) => *id,
        }
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
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-function_call"}}}
    /// Navigate to [`FunctionCall`] across R29(1-Mc)
    pub fn r29_function_call<'a>(&'a self, store: &'a LuDogStore) -> Vec<&FunctionCall> {
        store
            .iter_function_call()
            .filter_map(|function_call| {
                if function_call.expression == Some(self.id()) {
                    Some(function_call)
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
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-impl-nav-subtype-to-supertype-value"}}}
    // Navigate to [`Value`] across R11(isa)
    pub fn r11_value<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Value> {
        vec![store.exhume_value(&self.id()).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
