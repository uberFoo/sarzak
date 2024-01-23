// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-use-statements"}}}
use parking_lot::Mutex;
use std::sync::Arc;
use uuid::Uuid;

use crate::v2::lu_dog_pl_mutex::types::a_wait::AWait;
use crate::v2::lu_dog_pl_mutex::types::argument::Argument;
use crate::v2::lu_dog_pl_mutex::types::block::Block;
use crate::v2::lu_dog_pl_mutex::types::call::Call;
use crate::v2::lu_dog_pl_mutex::types::empty_expression::EMPTY_EXPRESSION;
use crate::v2::lu_dog_pl_mutex::types::expression_statement::ExpressionStatement;
use crate::v2::lu_dog_pl_mutex::types::field_access::FieldAccess;
use crate::v2::lu_dog_pl_mutex::types::field_expression::FieldExpression;
use crate::v2::lu_dog_pl_mutex::types::for_loop::ForLoop;
use crate::v2::lu_dog_pl_mutex::types::grouped::Grouped;
use crate::v2::lu_dog_pl_mutex::types::index::Index;
use crate::v2::lu_dog_pl_mutex::types::lambda::Lambda;
use crate::v2::lu_dog_pl_mutex::types::let_statement::LetStatement;
use crate::v2::lu_dog_pl_mutex::types::list_element::ListElement;
use crate::v2::lu_dog_pl_mutex::types::list_expression::ListExpression;
use crate::v2::lu_dog_pl_mutex::types::literal::Literal;
use crate::v2::lu_dog_pl_mutex::types::operator::Operator;
use crate::v2::lu_dog_pl_mutex::types::pattern::Pattern;
use crate::v2::lu_dog_pl_mutex::types::range_expression::RangeExpression;
use crate::v2::lu_dog_pl_mutex::types::result_statement::ResultStatement;
use crate::v2::lu_dog_pl_mutex::types::struct_expression::StructExpression;
use crate::v2::lu_dog_pl_mutex::types::type_cast::TypeCast;
use crate::v2::lu_dog_pl_mutex::types::variable_expression::VariableExpression;
use crate::v2::lu_dog_pl_mutex::types::x_debugger::X_DEBUGGER;
use crate::v2::lu_dog_pl_mutex::types::x_if::XIf;
use crate::v2::lu_dog_pl_mutex::types::x_match::XMatch;
use crate::v2::lu_dog_pl_mutex::types::x_path::XPath;
use crate::v2::lu_dog_pl_mutex::types::x_print::XPrint;
use crate::v2::lu_dog_pl_mutex::types::x_return::XReturn;
use crate::v2::lu_dog_pl_mutex::types::x_value::XValue;
use crate::v2::lu_dog_pl_mutex::types::x_value::XValueEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_pl_mutex::store::ObjectStore as LuDogPlMutexStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-hybrid-documentation"}}}
/// An Expression
///
/// Expressions are calculations that render values.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Expression {
    pub subtype: ExpressionEnum,
    pub bogus: bool,
    pub id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ExpressionEnum {
    AWait(Uuid),
    Block(Uuid),
    Call(Uuid),
    XDebugger(Uuid),
    EmptyExpression(Uuid),
    FieldAccess(Uuid),
    FieldExpression(Uuid),
    ForLoop(Uuid),
    Grouped(Uuid),
    XIf(Uuid),
    Index(Uuid),
    Lambda(Uuid),
    ListElement(Uuid),
    ListExpression(Uuid),
    Literal(Uuid),
    XMatch(Uuid),
    Operator(Uuid),
    XPath(Uuid),
    XPrint(Uuid),
    RangeExpression(Uuid),
    XReturn(Uuid),
    StructExpression(Uuid),
    TypeCast(Uuid),
    VariableExpression(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-implementation"}}}
impl Expression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_a_wait"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_a_wait(
        bogus: bool,
        subtype: &Arc<Mutex<AWait>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Expression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Expression {
            bogus: bogus,
            subtype: ExpressionEnum::AWait(subtype.lock().id), // b
            id,
        }));
        store.inter_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_block"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_block(
        bogus: bool,
        subtype: &Arc<Mutex<Block>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Expression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Expression {
            bogus: bogus,
            subtype: ExpressionEnum::Block(subtype.lock().id), // b
            id,
        }));
        store.inter_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_call"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_call(
        bogus: bool,
        subtype: &Arc<Mutex<Call>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Expression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Expression {
            bogus: bogus,
            subtype: ExpressionEnum::Call(subtype.lock().id), // b
            id,
        }));
        store.inter_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_x_debugger"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_x_debugger(bogus: bool, store: &mut LuDogPlMutexStore) -> Arc<Mutex<Expression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Expression {
            bogus: bogus,
            subtype: ExpressionEnum::XDebugger(X_DEBUGGER),
            id,
        }));
        store.inter_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_empty_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_empty_expression(
        bogus: bool,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Expression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Expression {
            bogus: bogus,
            subtype: ExpressionEnum::EmptyExpression(EMPTY_EXPRESSION),
            id,
        }));
        store.inter_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_field_access"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_field_access(
        bogus: bool,
        subtype: &Arc<Mutex<FieldAccess>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Expression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Expression {
            bogus: bogus,
            subtype: ExpressionEnum::FieldAccess(subtype.lock().id), // b
            id,
        }));
        store.inter_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_field_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_field_expression(
        bogus: bool,
        subtype: &Arc<Mutex<FieldExpression>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Expression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Expression {
            bogus: bogus,
            subtype: ExpressionEnum::FieldExpression(subtype.lock().id), // b
            id,
        }));
        store.inter_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_for_loop"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_for_loop(
        bogus: bool,
        subtype: &Arc<Mutex<ForLoop>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Expression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Expression {
            bogus: bogus,
            subtype: ExpressionEnum::ForLoop(subtype.lock().id), // b
            id,
        }));
        store.inter_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_grouped"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_grouped(
        bogus: bool,
        subtype: &Arc<Mutex<Grouped>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Expression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Expression {
            bogus: bogus,
            subtype: ExpressionEnum::Grouped(subtype.lock().id), // b
            id,
        }));
        store.inter_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_x_if"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_x_if(
        bogus: bool,
        subtype: &Arc<Mutex<XIf>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Expression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Expression {
            bogus: bogus,
            subtype: ExpressionEnum::XIf(subtype.lock().id), // b
            id,
        }));
        store.inter_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_index"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_index(
        bogus: bool,
        subtype: &Arc<Mutex<Index>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Expression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Expression {
            bogus: bogus,
            subtype: ExpressionEnum::Index(subtype.lock().id), // b
            id,
        }));
        store.inter_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_lambda"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_lambda(
        bogus: bool,
        subtype: &Arc<Mutex<Lambda>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Expression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Expression {
            bogus: bogus,
            subtype: ExpressionEnum::Lambda(subtype.lock().id), // b
            id,
        }));
        store.inter_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_list_element"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_list_element(
        bogus: bool,
        subtype: &Arc<Mutex<ListElement>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Expression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Expression {
            bogus: bogus,
            subtype: ExpressionEnum::ListElement(subtype.lock().id), // b
            id,
        }));
        store.inter_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_list_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_list_expression(
        bogus: bool,
        subtype: &Arc<Mutex<ListExpression>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Expression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Expression {
            bogus: bogus,
            subtype: ExpressionEnum::ListExpression(subtype.lock().id), // b
            id,
        }));
        store.inter_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_literal"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_literal(
        bogus: bool,
        subtype: &Arc<Mutex<Literal>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Expression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Expression {
            bogus: bogus,
            subtype: ExpressionEnum::Literal(subtype.lock().id), // b
            id,
        }));
        store.inter_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_x_match"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_x_match(
        bogus: bool,
        subtype: &Arc<Mutex<XMatch>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Expression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Expression {
            bogus: bogus,
            subtype: ExpressionEnum::XMatch(subtype.lock().id), // b
            id,
        }));
        store.inter_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_operator"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_operator(
        bogus: bool,
        subtype: &Arc<Mutex<Operator>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Expression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Expression {
            bogus: bogus,
            subtype: ExpressionEnum::Operator(subtype.lock().id), // b
            id,
        }));
        store.inter_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_x_path"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_x_path(
        bogus: bool,
        subtype: &Arc<Mutex<XPath>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Expression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Expression {
            bogus: bogus,
            subtype: ExpressionEnum::XPath(subtype.lock().id), // b
            id,
        }));
        store.inter_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_x_print"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_x_print(
        bogus: bool,
        subtype: &Arc<Mutex<XPrint>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Expression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Expression {
            bogus: bogus,
            subtype: ExpressionEnum::XPrint(subtype.lock().id), // b
            id,
        }));
        store.inter_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_range_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_range_expression(
        bogus: bool,
        subtype: &Arc<Mutex<RangeExpression>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Expression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Expression {
            bogus: bogus,
            subtype: ExpressionEnum::RangeExpression(subtype.lock().id), // b
            id,
        }));
        store.inter_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_x_return"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_x_return(
        bogus: bool,
        subtype: &Arc<Mutex<XReturn>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Expression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Expression {
            bogus: bogus,
            subtype: ExpressionEnum::XReturn(subtype.lock().id), // b
            id,
        }));
        store.inter_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_struct_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_struct_expression(
        bogus: bool,
        subtype: &Arc<Mutex<StructExpression>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Expression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Expression {
            bogus: bogus,
            subtype: ExpressionEnum::StructExpression(subtype.lock().id), // b
            id,
        }));
        store.inter_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_type_cast"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_type_cast(
        bogus: bool,
        subtype: &Arc<Mutex<TypeCast>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Expression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Expression {
            bogus: bogus,
            subtype: ExpressionEnum::TypeCast(subtype.lock().id), // b
            id,
        }));
        store.inter_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_variable_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub fn new_variable_expression(
        bogus: bool,
        subtype: &Arc<Mutex<VariableExpression>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Expression>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Expression {
            bogus: bogus,
            subtype: ExpressionEnum::VariableExpression(subtype.lock().id), // b
            id,
        }));
        store.inter_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-argument"}}}
    /// Navigate to [`Argument`] across R37(1-M)
    pub fn r37_argument<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<Argument>>> {
        store
            .iter_argument()
            .filter(|argument| argument.lock().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-cond-to-a_wait"}}}
    /// Navigate to [`AWait`] across R98(1-1c)
    pub fn r98c_a_wait<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<AWait>>> {
        let a_wait = store
            .iter_a_wait()
            .find(|a_wait| a_wait.lock().x_future == self.id);
        match a_wait {
            Some(ref a_wait) => vec![a_wait.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-call"}}}
    /// Navigate to [`Call`] across R29(1-Mc)
    pub fn r29_call<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<Call>>> {
        store
            .iter_call()
            .filter(|call| call.lock().expression == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-expression_statement"}}}
    /// Navigate to [`ExpressionStatement`] across R31(1-M)
    pub fn r31_expression_statement<'a>(
        &'a self,
        store: &'a LuDogPlMutexStore,
    ) -> Vec<Arc<Mutex<ExpressionStatement>>> {
        store
            .iter_expression_statement()
            .filter(|expression_statement| expression_statement.lock().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-field_access"}}}
    /// Navigate to [`FieldAccess`] across R27(1-M)
    pub fn r27_field_access<'a>(
        &'a self,
        store: &'a LuDogPlMutexStore,
    ) -> Vec<Arc<Mutex<FieldAccess>>> {
        store
            .iter_field_access()
            .filter(|field_access| field_access.lock().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-field_expression"}}}
    /// Navigate to [`FieldExpression`] across R38(1-M)
    pub fn r38_field_expression<'a>(
        &'a self,
        store: &'a LuDogPlMutexStore,
    ) -> Vec<Arc<Mutex<FieldExpression>>> {
        store
            .iter_field_expression()
            .filter(|field_expression| field_expression.lock().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-for_loop"}}}
    /// Navigate to [`ForLoop`] across R43(1-M)
    pub fn r43_for_loop<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<ForLoop>>> {
        store
            .iter_for_loop()
            .filter(|for_loop| for_loop.lock().block == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-for_loop"}}}
    /// Navigate to [`ForLoop`] across R42(1-M)
    pub fn r42_for_loop<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<ForLoop>>> {
        store
            .iter_for_loop()
            .filter(|for_loop| for_loop.lock().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-grouped"}}}
    /// Navigate to [`Grouped`] across R61(1-M)
    pub fn r61_grouped<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<Grouped>>> {
        store
            .iter_grouped()
            .filter(|grouped| grouped.lock().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-x_if"}}}
    /// Navigate to [`XIf`] across R44(1-M)
    pub fn r44_x_if<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<XIf>>> {
        store
            .iter_x_if()
            .filter(|x_if| x_if.lock().test == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-x_if"}}}
    /// Navigate to [`XIf`] across R52(1-Mc)
    pub fn r52_x_if<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<XIf>>> {
        store
            .iter_x_if()
            .filter(|x_if| x_if.lock().false_block == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-index"}}}
    /// Navigate to [`Index`] across R56(1-M)
    pub fn r56_index<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<Index>>> {
        store
            .iter_index()
            .filter(|index| index.lock().index == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-index"}}}
    /// Navigate to [`Index`] across R57(1-M)
    pub fn r57_index<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<Index>>> {
        store
            .iter_index()
            .filter(|index| index.lock().target == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-cond-to-let_statement"}}}
    /// Navigate to [`LetStatement`] across R20(1-1c)
    pub fn r20c_let_statement<'a>(
        &'a self,
        store: &'a LuDogPlMutexStore,
    ) -> Vec<Arc<Mutex<LetStatement>>> {
        let let_statement = store
            .iter_let_statement()
            .find(|let_statement| let_statement.lock().expression == self.id);
        match let_statement {
            Some(ref let_statement) => vec![let_statement.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-list_element"}}}
    /// Navigate to [`ListElement`] across R55(1-M)
    pub fn r55_list_element<'a>(
        &'a self,
        store: &'a LuDogPlMutexStore,
    ) -> Vec<Arc<Mutex<ListElement>>> {
        store
            .iter_list_element()
            .filter(|list_element| list_element.lock().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-x_match"}}}
    /// Navigate to [`XMatch`] across R91(1-M)
    pub fn r91_x_match<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<XMatch>>> {
        store
            .iter_x_match()
            .filter(|x_match| x_match.lock().scrutinee == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-operator"}}}
    /// Navigate to [`Operator`] across R51(1-Mc)
    pub fn r51_operator<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<Operator>>> {
        store
            .iter_operator()
            .filter(|operator| operator.lock().rhs == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-operator"}}}
    /// Navigate to [`Operator`] across R50(1-M)
    pub fn r50_operator<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<Operator>>> {
        store
            .iter_operator()
            .filter(|operator| operator.lock().lhs == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-pattern"}}}
    /// Navigate to [`Pattern`] across R92(1-M)
    pub fn r92_pattern<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<Pattern>>> {
        store
            .iter_pattern()
            .filter(|pattern| pattern.lock().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-x_print"}}}
    /// Navigate to [`XPrint`] across R32(1-M)
    pub fn r32_x_print<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<XPrint>>> {
        store
            .iter_x_print()
            .filter(|x_print| x_print.lock().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-range_expression"}}}
    /// Navigate to [`RangeExpression`] across R58(1-Mc)
    pub fn r58_range_expression<'a>(
        &'a self,
        store: &'a LuDogPlMutexStore,
    ) -> Vec<Arc<Mutex<RangeExpression>>> {
        store
            .iter_range_expression()
            .filter(|range_expression| range_expression.lock().lhs == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-range_expression"}}}
    /// Navigate to [`RangeExpression`] across R59(1-Mc)
    pub fn r59_range_expression<'a>(
        &'a self,
        store: &'a LuDogPlMutexStore,
    ) -> Vec<Arc<Mutex<RangeExpression>>> {
        store
            .iter_range_expression()
            .filter(|range_expression| range_expression.lock().rhs == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-result_statement"}}}
    /// Navigate to [`ResultStatement`] across R41(1-M)
    pub fn r41_result_statement<'a>(
        &'a self,
        store: &'a LuDogPlMutexStore,
    ) -> Vec<Arc<Mutex<ResultStatement>>> {
        store
            .iter_result_statement()
            .filter(|result_statement| result_statement.lock().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-x_return"}}}
    /// Navigate to [`XReturn`] across R45(1-M)
    pub fn r45_x_return<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<XReturn>>> {
        store
            .iter_x_return()
            .filter(|x_return| x_return.lock().expression == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-type_cast"}}}
    /// Navigate to [`TypeCast`] across R68(1-M)
    pub fn r68_type_cast<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<TypeCast>>> {
        store
            .iter_type_cast()
            .filter(|type_cast| type_cast.lock().lhs == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-assoc-many-to-pattern"}}}
    /// Navigate to [`Pattern`] across R87(1-M)
    pub fn r87_pattern<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<Pattern>>> {
        store
            .iter_pattern()
            .filter(|pattern| pattern.lock().match_expr == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-impl-nav-subtype-to-supertype-x_value"}}}
    // Navigate to [`XValue`] across R11(isa)
    pub fn r11_x_value<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<XValue>>> {
        vec![store
            .iter_x_value()
            .find(|x_value| {
                if let XValueEnum::Expression(id) = x_value.lock().subtype {
                    id == self.id
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
