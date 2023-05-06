// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-use-statements"}}}
use std::sync::{Arc, RwLock};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
use crate::v2::lu_dog::types::argument::Argument;
use crate::v2::lu_dog::types::block::Block;
use crate::v2::lu_dog::types::call::Call;
use crate::v2::lu_dog::types::error_expression::ErrorExpression;
use crate::v2::lu_dog::types::expression_statement::ExpressionStatement;
use crate::v2::lu_dog::types::field_access::FieldAccess;
use crate::v2::lu_dog::types::field_expression::FieldExpression;
use crate::v2::lu_dog::types::for_loop::ForLoop;
use crate::v2::lu_dog::types::let_statement::LetStatement;
use crate::v2::lu_dog::types::literal::Literal;
use crate::v2::lu_dog::types::operator::Operator;
use crate::v2::lu_dog::types::print::Print;
use crate::v2::lu_dog::types::result_statement::ResultStatement;
use crate::v2::lu_dog::types::struct_expression::StructExpression;
use crate::v2::lu_dog::types::value::Value;
use crate::v2::lu_dog::types::value::ValueEnum;
use crate::v2::lu_dog::types::variable_expression::VariableExpression;
use crate::v2::lu_dog::types::x_if::XIf;
use crate::v2::lu_dog::types::x_return::XReturn;
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
    ErrorExpression(Uuid),
    FieldAccess(Uuid),
    ForLoop(Uuid),
    XIf(Uuid),
    Literal(Uuid),
    Operator(Uuid),
    Print(Uuid),
    XReturn(Uuid),
    StructExpression(Uuid),
    VariableExpression(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-implementation"}}}
impl Expression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-new-impl"}}}
    /// Create a new instance of Expression::Block
    pub fn new_block(block: &Arc<RwLock<Block>>, store: &mut LuDogStore) -> Arc<RwLock<Self>> {
        if let Some(block) = store.exhume_expression(&block.read().unwrap().id) {
            block
        } else {
            let new = Arc::new(RwLock::new(Self::Block(block.read().unwrap().id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::Call
    pub fn new_call(call: &Arc<RwLock<Call>>, store: &mut LuDogStore) -> Arc<RwLock<Self>> {
        if let Some(call) = store.exhume_expression(&call.read().unwrap().id) {
            call
        } else {
            let new = Arc::new(RwLock::new(Self::Call(call.read().unwrap().id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::ErrorExpression
    pub fn new_error_expression(
        error_expression: &Arc<RwLock<ErrorExpression>>,
        store: &mut LuDogStore,
    ) -> Arc<RwLock<Self>> {
        if let Some(error_expression) =
            store.exhume_expression(&error_expression.read().unwrap().id)
        {
            error_expression
        } else {
            let new = Arc::new(RwLock::new(Self::ErrorExpression(
                error_expression.read().unwrap().id,
            )));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::FieldAccess
    pub fn new_field_access(
        field_access: &Arc<RwLock<FieldAccess>>,
        store: &mut LuDogStore,
    ) -> Arc<RwLock<Self>> {
        if let Some(field_access) = store.exhume_expression(&field_access.read().unwrap().id) {
            field_access
        } else {
            let new = Arc::new(RwLock::new(Self::FieldAccess(
                field_access.read().unwrap().id,
            )));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::ForLoop
    pub fn new_for_loop(
        for_loop: &Arc<RwLock<ForLoop>>,
        store: &mut LuDogStore,
    ) -> Arc<RwLock<Self>> {
        if let Some(for_loop) = store.exhume_expression(&for_loop.read().unwrap().id) {
            for_loop
        } else {
            let new = Arc::new(RwLock::new(Self::ForLoop(for_loop.read().unwrap().id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::XIf
    pub fn new_x_if(x_if: &Arc<RwLock<XIf>>, store: &mut LuDogStore) -> Arc<RwLock<Self>> {
        if let Some(x_if) = store.exhume_expression(&x_if.read().unwrap().id) {
            x_if
        } else {
            let new = Arc::new(RwLock::new(Self::XIf(x_if.read().unwrap().id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::Literal
    pub fn new_literal(
        literal: &Arc<RwLock<Literal>>,
        store: &mut LuDogStore,
    ) -> Arc<RwLock<Self>> {
        if let Some(literal) = store.exhume_expression(&literal.read().unwrap().id()) {
            literal
        } else {
            let new = Arc::new(RwLock::new(Self::Literal(literal.read().unwrap().id())));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::Operator
    pub fn new_operator(
        operator: &Arc<RwLock<Operator>>,
        store: &mut LuDogStore,
    ) -> Arc<RwLock<Self>> {
        if let Some(operator) = store.exhume_expression(&operator.read().unwrap().id) {
            operator
        } else {
            let new = Arc::new(RwLock::new(Self::Operator(operator.read().unwrap().id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::Print
    pub fn new_print(print: &Arc<RwLock<Print>>, store: &mut LuDogStore) -> Arc<RwLock<Self>> {
        if let Some(print) = store.exhume_expression(&print.read().unwrap().id) {
            print
        } else {
            let new = Arc::new(RwLock::new(Self::Print(print.read().unwrap().id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::XReturn
    pub fn new_x_return(
        x_return: &Arc<RwLock<XReturn>>,
        store: &mut LuDogStore,
    ) -> Arc<RwLock<Self>> {
        if let Some(x_return) = store.exhume_expression(&x_return.read().unwrap().id) {
            x_return
        } else {
            let new = Arc::new(RwLock::new(Self::XReturn(x_return.read().unwrap().id)));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::StructExpression
    pub fn new_struct_expression(
        struct_expression: &Arc<RwLock<StructExpression>>,
        store: &mut LuDogStore,
    ) -> Arc<RwLock<Self>> {
        if let Some(struct_expression) =
            store.exhume_expression(&struct_expression.read().unwrap().id)
        {
            struct_expression
        } else {
            let new = Arc::new(RwLock::new(Self::StructExpression(
                struct_expression.read().unwrap().id,
            )));
            store.inter_expression(new.clone());
            new
        }
    }

    /// Create a new instance of Expression::VariableExpression
    pub fn new_variable_expression(
        variable_expression: &Arc<RwLock<VariableExpression>>,
        store: &mut LuDogStore,
    ) -> Arc<RwLock<Self>> {
        if let Some(variable_expression) =
            store.exhume_expression(&variable_expression.read().unwrap().id)
        {
            variable_expression
        } else {
            let new = Arc::new(RwLock::new(Self::VariableExpression(
                variable_expression.read().unwrap().id,
            )));
            store.inter_expression(new.clone());
            new
        }
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Expression::Block(id) => *id,
            Expression::Call(id) => *id,
            Expression::ErrorExpression(id) => *id,
            Expression::FieldAccess(id) => *id,
            Expression::ForLoop(id) => *id,
            Expression::XIf(id) => *id,
            Expression::Literal(id) => *id,
            Expression::Operator(id) => *id,
            Expression::Print(id) => *id,
            Expression::XReturn(id) => *id,
            Expression::StructExpression(id) => *id,
            Expression::VariableExpression(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-argument"}}}
    /// Navigate to [`Argument`] across R37(1-M)
    pub fn r37_argument<'a>(&'a self, store: &'a LuDogStore) -> Vec<Arc<RwLock<Argument>>> {
        store
            .iter_argument()
            .filter_map(|argument| {
                if argument.read().unwrap().expression == self.id() {
                    Some(argument)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-call"}}}
    /// Navigate to [`Call`] across R29(1-Mc)
    pub fn r29_call<'a>(&'a self, store: &'a LuDogStore) -> Vec<Arc<RwLock<Call>>> {
        store
            .iter_call()
            .filter_map(|call| {
                if call.read().unwrap().expression == Some(self.id()) {
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
    ) -> Vec<Arc<RwLock<ExpressionStatement>>> {
        store
            .iter_expression_statement()
            .filter_map(|expression_statement| {
                if expression_statement.read().unwrap().expression == self.id() {
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
    pub fn r27_field_access<'a>(&'a self, store: &'a LuDogStore) -> Vec<Arc<RwLock<FieldAccess>>> {
        store
            .iter_field_access()
            .filter_map(|field_access| {
                if field_access.read().unwrap().expression == self.id() {
                    Some(field_access)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-field_expression"}}}
    /// Navigate to [`FieldExpression`] across R38(1-M)
    pub fn r38_field_expression<'a>(
        &'a self,
        store: &'a LuDogStore,
    ) -> Vec<Arc<RwLock<FieldExpression>>> {
        store
            .iter_field_expression()
            .filter_map(|field_expression| {
                if field_expression.read().unwrap().expression == self.id() {
                    Some(field_expression)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-for_loop"}}}
    /// Navigate to [`ForLoop`] across R42(1-M)
    pub fn r42_for_loop<'a>(&'a self, store: &'a LuDogStore) -> Vec<Arc<RwLock<ForLoop>>> {
        store
            .iter_for_loop()
            .filter_map(|for_loop| {
                if for_loop.read().unwrap().expression == self.id() {
                    Some(for_loop)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-x_if"}}}
    /// Navigate to [`XIf`] across R44(1-M)
    pub fn r44_x_if<'a>(&'a self, store: &'a LuDogStore) -> Vec<Arc<RwLock<XIf>>> {
        store
            .iter_x_if()
            .filter_map(|x_if| {
                if x_if.read().unwrap().test == self.id() {
                    Some(x_if)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-cond-to-let_statement"}}}
    /// Navigate to [`LetStatement`] across R20(1-1c)
    pub fn r20c_let_statement<'a>(
        &'a self,
        store: &'a LuDogStore,
    ) -> Vec<Arc<RwLock<LetStatement>>> {
        let let_statement = store
            .iter_let_statement()
            .find(|let_statement| let_statement.read().unwrap().expression == self.id());
        match let_statement {
            Some(ref let_statement) => vec![let_statement.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-operator"}}}
    /// Navigate to [`Operator`] across R51(1-Mc)
    pub fn r51_operator<'a>(&'a self, store: &'a LuDogStore) -> Vec<Arc<RwLock<Operator>>> {
        store
            .iter_operator()
            .filter_map(|operator| {
                if operator.read().unwrap().rhs == Some(self.id()) {
                    Some(operator)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-operator"}}}
    /// Navigate to [`Operator`] across R50(1-M)
    pub fn r50_operator<'a>(&'a self, store: &'a LuDogStore) -> Vec<Arc<RwLock<Operator>>> {
        store
            .iter_operator()
            .filter_map(|operator| {
                if operator.read().unwrap().lhs == self.id() {
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
    pub fn r32_print<'a>(&'a self, store: &'a LuDogStore) -> Vec<Arc<RwLock<Print>>> {
        store
            .iter_print()
            .filter_map(|print| {
                if print.read().unwrap().expression == self.id() {
                    Some(print)
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
        store: &'a LuDogStore,
    ) -> Vec<Arc<RwLock<ResultStatement>>> {
        store
            .iter_result_statement()
            .filter_map(|result_statement| {
                if result_statement.read().unwrap().expression == self.id() {
                    Some(result_statement)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-x_return"}}}
    /// Navigate to [`XReturn`] across R45(1-M)
    pub fn r45_x_return<'a>(&'a self, store: &'a LuDogStore) -> Vec<Arc<RwLock<XReturn>>> {
        store
            .iter_x_return()
            .filter_map(|x_return| {
                if x_return.read().unwrap().expression == self.id() {
                    Some(x_return)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-impl-nav-subtype-to-supertype-value"}}}
    // Navigate to [`Value`] across R11(isa)
    pub fn r11_value<'a>(&'a self, store: &'a LuDogStore) -> Vec<Arc<RwLock<Value>>> {
        vec![store
            .iter_value()
            .find(|value| {
                if let ValueEnum::Expression(id) = value.read().unwrap().subtype {
                    id == self.id()
                } else {
                    false
                }
            })
            .unwrap()] // 💥
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
