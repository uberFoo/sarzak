// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use uuid::Uuid;

use crate::v2::lu_dog_async::types::a_wait::AWait;
use crate::v2::lu_dog_async::types::argument::Argument;
use crate::v2::lu_dog_async::types::block::Block;
use crate::v2::lu_dog_async::types::call::Call;
use crate::v2::lu_dog_async::types::empty_expression::EMPTY_EXPRESSION;
use crate::v2::lu_dog_async::types::expression_statement::ExpressionStatement;
use crate::v2::lu_dog_async::types::field_access::FieldAccess;
use crate::v2::lu_dog_async::types::field_expression::FieldExpression;
use crate::v2::lu_dog_async::types::for_loop::ForLoop;
use crate::v2::lu_dog_async::types::grouped::Grouped;
use crate::v2::lu_dog_async::types::index::Index;
use crate::v2::lu_dog_async::types::lambda::Lambda;
use crate::v2::lu_dog_async::types::let_statement::LetStatement;
use crate::v2::lu_dog_async::types::list_element::ListElement;
use crate::v2::lu_dog_async::types::list_expression::ListExpression;
use crate::v2::lu_dog_async::types::literal::Literal;
use crate::v2::lu_dog_async::types::operator::Operator;
use crate::v2::lu_dog_async::types::pattern::Pattern;
use crate::v2::lu_dog_async::types::range_expression::RangeExpression;
use crate::v2::lu_dog_async::types::result_statement::ResultStatement;
use crate::v2::lu_dog_async::types::struct_expression::StructExpression;
use crate::v2::lu_dog_async::types::type_cast::TypeCast;
use crate::v2::lu_dog_async::types::variable_expression::VariableExpression;
use crate::v2::lu_dog_async::types::x_debugger::X_DEBUGGER;
use crate::v2::lu_dog_async::types::x_if::XIf;
use crate::v2::lu_dog_async::types::x_match::XMatch;
use crate::v2::lu_dog_async::types::x_path::XPath;
use crate::v2::lu_dog_async::types::x_print::XPrint;
use crate::v2::lu_dog_async::types::x_return::XReturn;
use crate::v2::lu_dog_async::types::x_value::XValue;
use crate::v2::lu_dog_async::types::x_value::XValueEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-hybrid-documentation"}}}
/// An Expression
///
/// Expressions are calculations that render values.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Expression {
    pub subtype: ExpressionEnum,
    pub bogus: bool,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ExpressionEnum {
    AWait(usize),
    Block(usize),
    Call(usize),
    XDebugger(Uuid),
    EmptyExpression(Uuid),
    FieldAccess(usize),
    FieldExpression(usize),
    ForLoop(usize),
    Grouped(usize),
    XIf(usize),
    Index(usize),
    Lambda(usize),
    ListElement(usize),
    ListExpression(usize),
    Literal(usize),
    XMatch(usize),
    Operator(usize),
    XPath(usize),
    XPrint(usize),
    RangeExpression(usize),
    XReturn(usize),
    StructExpression(usize),
    TypeCast(usize),
    VariableExpression(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-implementation"}}}
impl Expression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_block"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_a_wait"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_a_wait(
        bogus: bool,
        subtype: &Arc<RwLock<AWait>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    bogus: bogus,
                    subtype: ExpressionEnum::AWait(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_call"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_block"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_block(
        bogus: bool,
        subtype: &Arc<RwLock<Block>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    bogus: bogus,
                    subtype: ExpressionEnum::Block(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_debugger"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_call"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_call(
        bogus: bool,
        subtype: &Arc<RwLock<Call>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    bogus: bogus,
                    subtype: ExpressionEnum::Call(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_enum_field"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_x_debugger"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_x_debugger(
        bogus: bool,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    bogus: bogus,
                    subtype: ExpressionEnum::XDebugger(X_DEBUGGER),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_error_expression"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_empty_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_empty_expression(
        bogus: bool,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    bogus: bogus,
                    subtype: ExpressionEnum::EmptyExpression(EMPTY_EXPRESSION),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_field_access"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_field_access(
        bogus: bool,
        subtype: &Arc<RwLock<FieldAccess>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    bogus: bogus,
                    subtype: ExpressionEnum::FieldAccess(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_field_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_field_expression(
        bogus: bool,
        subtype: &Arc<RwLock<FieldExpression>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    bogus: bogus,
                    subtype: ExpressionEnum::FieldExpression(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_for_loop"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_for_loop(
        bogus: bool,
        subtype: &Arc<RwLock<ForLoop>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    bogus: bogus,
                    subtype: ExpressionEnum::ForLoop(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_grouped"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_grouped(
        bogus: bool,
        subtype: &Arc<RwLock<Grouped>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    bogus: bogus,
                    subtype: ExpressionEnum::Grouped(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_x_if"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_x_if(
        bogus: bool,
        subtype: &Arc<RwLock<XIf>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    bogus: bogus,
                    subtype: ExpressionEnum::XIf(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_index"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_index(
        bogus: bool,
        subtype: &Arc<RwLock<Index>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    bogus: bogus,
                    subtype: ExpressionEnum::Index(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_lambda"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_lambda(
        bogus: bool,
        subtype: &Arc<RwLock<Lambda>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    bogus: bogus,
                    subtype: ExpressionEnum::Lambda(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_list_element"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_list_element(
        bogus: bool,
        subtype: &Arc<RwLock<ListElement>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    bogus: bogus,
                    subtype: ExpressionEnum::ListElement(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_list_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_list_expression(
        bogus: bool,
        subtype: &Arc<RwLock<ListExpression>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    bogus: bogus,
                    subtype: ExpressionEnum::ListExpression(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_literal"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_literal(
        bogus: bool,
        subtype: &Arc<RwLock<Literal>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    bogus: bogus,
                    subtype: ExpressionEnum::Literal(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_x_match"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_x_match(
        bogus: bool,
        subtype: &Arc<RwLock<XMatch>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    bogus: bogus,
                    subtype: ExpressionEnum::XMatch(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_z_none"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_operator"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_operator(
        bogus: bool,
        subtype: &Arc<RwLock<Operator>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    bogus: bogus,
                    subtype: ExpressionEnum::Operator(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_operator"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_x_path"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_x_path(
        bogus: bool,
        subtype: &Arc<RwLock<XPath>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    bogus: bogus,
                    subtype: ExpressionEnum::XPath(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_x_print"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_x_print(
        bogus: bool,
        subtype: &Arc<RwLock<XPrint>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    bogus: bogus,
                    subtype: ExpressionEnum::XPrint(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_range_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_range_expression(
        bogus: bool,
        subtype: &Arc<RwLock<RangeExpression>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    bogus: bogus,
                    subtype: ExpressionEnum::RangeExpression(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_x_return"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_x_return(
        bogus: bool,
        subtype: &Arc<RwLock<XReturn>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    bogus: bogus,
                    subtype: ExpressionEnum::XReturn(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_z_some"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_struct_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_struct_expression(
        bogus: bool,
        subtype: &Arc<RwLock<StructExpression>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    bogus: bogus,
                    subtype: ExpressionEnum::StructExpression(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_type_cast"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_type_cast(
        bogus: bool,
        subtype: &Arc<RwLock<TypeCast>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    bogus: bogus,
                    subtype: ExpressionEnum::TypeCast(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_variable_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_variable_expression(
        bogus: bool,
        subtype: &Arc<RwLock<VariableExpression>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    bogus: bogus,
                    subtype: ExpressionEnum::VariableExpression(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-argument"}}}
    /// Navigate to [`Argument`] across R37(1-M)
    pub async fn r37_argument<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Argument>>> + '_ {
        store.iter_argument().await.filter_map(|argument| async {
            if argument.read().await.expression == self.id {
                Some(argument)
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-cond-to-a_wait"}}}
    /// Navigate to [`AWait`] across R98(1-1c)
    pub async fn r98c_a_wait<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<AWait>>> {
        store
            .iter_a_wait()
            .await
            .filter_map(|a_wait| async {
                if a_wait.read().await.x_future == self.id {
                    Some(a_wait)
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-call"}}}
    /// Navigate to [`Call`] across R29(1-Mc)
    pub async fn r29_call<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Call>>> + '_ {
        store.iter_call().await.filter_map(move |call| async move {
            if call.read().await.expression == Some(self.id) {
                Some(call.clone())
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-expression_statement"}}}
    /// Navigate to [`ExpressionStatement`] across R31(1-M)
    pub async fn r31_expression_statement<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<ExpressionStatement>>> + '_ {
        store
            .iter_expression_statement()
            .await
            .filter_map(|expression_statement| async {
                if expression_statement.read().await.expression == self.id {
                    Some(expression_statement)
                } else {
                    None
                }
            })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-field_access"}}}
    /// Navigate to [`FieldAccess`] across R27(1-M)
    pub async fn r27_field_access<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<FieldAccess>>> + '_ {
        store
            .iter_field_access()
            .await
            .filter_map(|field_access| async {
                if field_access.read().await.expression == self.id {
                    Some(field_access)
                } else {
                    None
                }
            })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-field_expression"}}}
    /// Navigate to [`FieldExpression`] across R38(1-M)
    pub async fn r38_field_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<FieldExpression>>> + '_ {
        store
            .iter_field_expression()
            .await
            .filter_map(|field_expression| async {
                if field_expression.read().await.expression == self.id {
                    Some(field_expression)
                } else {
                    None
                }
            })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-for_loop"}}}
    /// Navigate to [`ForLoop`] across R43(1-M)
    pub async fn r43_for_loop<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<ForLoop>>> + '_ {
        store.iter_for_loop().await.filter_map(|for_loop| async {
            if for_loop.read().await.block == self.id {
                Some(for_loop)
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-for_loop"}}}
    /// Navigate to [`ForLoop`] across R42(1-M)
    pub async fn r42_for_loop<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<ForLoop>>> + '_ {
        store.iter_for_loop().await.filter_map(|for_loop| async {
            if for_loop.read().await.expression == self.id {
                Some(for_loop)
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-grouped"}}}
    /// Navigate to [`Grouped`] across R61(1-M)
    pub async fn r61_grouped<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Grouped>>> + '_ {
        store.iter_grouped().await.filter_map(|grouped| async {
            if grouped.read().await.expression == self.id {
                Some(grouped)
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-x_if"}}}
    /// Navigate to [`XIf`] across R44(1-M)
    pub async fn r44_x_if<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<XIf>>> + '_ {
        store.iter_x_if().await.filter_map(|x_if| async {
            if x_if.read().await.test == self.id {
                Some(x_if)
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-x_if"}}}
    /// Navigate to [`XIf`] across R52(1-Mc)
    pub async fn r52_x_if<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<XIf>>> + '_ {
        store.iter_x_if().await.filter_map(move |x_if| async move {
            if x_if.read().await.false_block == Some(self.id) {
                Some(x_if.clone())
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-index"}}}
    /// Navigate to [`Index`] across R56(1-M)
    pub async fn r56_index<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Index>>> + '_ {
        store.iter_index().await.filter_map(|index| async {
            if index.read().await.index == self.id {
                Some(index)
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-index"}}}
    /// Navigate to [`Index`] across R57(1-M)
    pub async fn r57_index<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Index>>> + '_ {
        store.iter_index().await.filter_map(|index| async {
            if index.read().await.target == self.id {
                Some(index)
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-cond-to-let_statement"}}}
    /// Navigate to [`LetStatement`] across R20(1-1c)
    pub async fn r20c_let_statement<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<LetStatement>>> {
        store
            .iter_let_statement()
            .await
            .filter_map(|let_statement| async {
                if let_statement.read().await.expression == self.id {
                    Some(let_statement)
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-list_element"}}}
    /// Navigate to [`ListElement`] across R55(1-M)
    pub async fn r55_list_element<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<ListElement>>> + '_ {
        store
            .iter_list_element()
            .await
            .filter_map(|list_element| async {
                if list_element.read().await.expression == self.id {
                    Some(list_element)
                } else {
                    None
                }
            })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-x_match"}}}
    /// Navigate to [`XMatch`] across R91(1-M)
    pub async fn r91_x_match<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<XMatch>>> + '_ {
        store.iter_x_match().await.filter_map(|x_match| async {
            if x_match.read().await.scrutinee == self.id {
                Some(x_match)
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-operator"}}}
    /// Navigate to [`Operator`] across R51(1-Mc)
    pub async fn r51_operator<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Operator>>> + '_ {
        store
            .iter_operator()
            .await
            .filter_map(move |operator| async move {
                if operator.read().await.rhs == Some(self.id) {
                    Some(operator.clone())
                } else {
                    None
                }
            })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-operator"}}}
    /// Navigate to [`Operator`] across R50(1-M)
    pub async fn r50_operator<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Operator>>> + '_ {
        store.iter_operator().await.filter_map(|operator| async {
            if operator.read().await.lhs == self.id {
                Some(operator)
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-pattern"}}}
    /// Navigate to [`Pattern`] across R92(1-M)
    pub async fn r92_pattern<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Pattern>>> + '_ {
        store.iter_pattern().await.filter_map(|pattern| async {
            if pattern.read().await.expression == self.id {
                Some(pattern)
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-x_print"}}}
    /// Navigate to [`XPrint`] across R32(1-M)
    pub async fn r32_x_print<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<XPrint>>> + '_ {
        store.iter_x_print().await.filter_map(|x_print| async {
            if x_print.read().await.expression == self.id {
                Some(x_print)
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-range_expression"}}}
    /// Navigate to [`RangeExpression`] across R58(1-Mc)
    pub async fn r58_range_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<RangeExpression>>> + '_ {
        store
            .iter_range_expression()
            .await
            .filter_map(move |range_expression| async move {
                if range_expression.read().await.lhs == Some(self.id) {
                    Some(range_expression.clone())
                } else {
                    None
                }
            })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-range_expression"}}}
    /// Navigate to [`RangeExpression`] across R59(1-Mc)
    pub async fn r59_range_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<RangeExpression>>> + '_ {
        store
            .iter_range_expression()
            .await
            .filter_map(move |range_expression| async move {
                if range_expression.read().await.rhs == Some(self.id) {
                    Some(range_expression.clone())
                } else {
                    None
                }
            })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-result_statement"}}}
    /// Navigate to [`ResultStatement`] across R41(1-M)
    pub async fn r41_result_statement<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<ResultStatement>>> + '_ {
        store
            .iter_result_statement()
            .await
            .filter_map(|result_statement| async {
                if result_statement.read().await.expression == self.id {
                    Some(result_statement)
                } else {
                    None
                }
            })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-x_return"}}}
    /// Navigate to [`XReturn`] across R45(1-M)
    pub async fn r45_x_return<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<XReturn>>> + '_ {
        store.iter_x_return().await.filter_map(|x_return| async {
            if x_return.read().await.expression == self.id {
                Some(x_return)
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-struct_field"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-tuple_field"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-type_cast"}}}
    /// Navigate to [`TypeCast`] across R68(1-M)
    pub async fn r68_type_cast<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<TypeCast>>> + '_ {
        store.iter_type_cast().await.filter_map(|type_cast| async {
            if type_cast.read().await.lhs == self.id {
                Some(type_cast)
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-assoc-many-to-pattern"}}}
    /// Navigate to [`Pattern`] across R87(1-M)
    pub async fn r87_pattern<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Pattern>>> {
        store
            .iter_pattern()
            .await
            .filter_map(|pattern| async {
                if pattern.read().await.match_expr == self.id {
                    Some(pattern)
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-impl-nav-subtype-to-supertype-x_value"}}}
    // Navigate to [`XValue`] across R11(isa)
    pub async fn r11_x_value<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<XValue>>> {
        store
            .iter_x_value()
            .await
            .filter_map(|x_value| async move {
                if let XValueEnum::Expression(id) = x_value.read().await.subtype {
                    Some(x_value.clone())
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-implementation"}}}
impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype && self.bogus == other.bogus
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
