// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::argument::Argument;
use crate::v2::lu_dog_async::types::block::Block;
use crate::v2::lu_dog_async::types::call::Call;
use crate::v2::lu_dog_async::types::debugger::DEBUGGER;
use crate::v2::lu_dog_async::types::enum_field::EnumField;
use crate::v2::lu_dog_async::types::error_expression::ErrorExpression;
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
use crate::v2::lu_dog_async::types::struct_field::StructField;
use crate::v2::lu_dog_async::types::tuple_field::TupleField;
use crate::v2::lu_dog_async::types::type_cast::TypeCast;
use crate::v2::lu_dog_async::types::variable_expression::VariableExpression;
use crate::v2::lu_dog_async::types::x_if::XIf;
use crate::v2::lu_dog_async::types::x_match::XMatch;
use crate::v2::lu_dog_async::types::x_print::XPrint;
use crate::v2::lu_dog_async::types::x_return::XReturn;
use crate::v2::lu_dog_async::types::x_value::XValue;
use crate::v2::lu_dog_async::types::x_value::XValueEnum;
use crate::v2::lu_dog_async::types::z_none::Z_NONE;
use crate::v2::lu_dog_async::types::z_some::ZSome;
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
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ExpressionEnum {
    Block(usize),
    Call(usize),
    Debugger(Uuid),
    EnumField(usize),
    ErrorExpression(usize),
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
    ZNone(Uuid),
    Operator(usize),
    XPrint(usize),
    RangeExpression(usize),
    XReturn(usize),
    ZSome(usize),
    StructExpression(usize),
    TypeCast(usize),
    VariableExpression(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-implementation"}}}
impl Expression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_block"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_block(
        subtype: &Arc<RwLock<Block>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    subtype: ExpressionEnum::Block(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_call"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_call(
        subtype: &Arc<RwLock<Call>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    subtype: ExpressionEnum::Call(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_debugger"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_debugger(store: &mut LuDogAsyncStore) -> Arc<RwLock<Expression>> {
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    subtype: ExpressionEnum::Debugger(DEBUGGER),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_enum_field"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_enum_field(
        subtype: &Arc<RwLock<EnumField>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    subtype: ExpressionEnum::EnumField(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_error_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_error_expression(
        subtype: &Arc<RwLock<ErrorExpression>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    subtype: ExpressionEnum::ErrorExpression(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_field_access"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_field_access(
        subtype: &Arc<RwLock<FieldAccess>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
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
        subtype: &Arc<RwLock<FieldExpression>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
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
        subtype: &Arc<RwLock<ForLoop>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
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
        subtype: &Arc<RwLock<Grouped>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
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
        subtype: &Arc<RwLock<XIf>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
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
        subtype: &Arc<RwLock<Index>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
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
        subtype: &Arc<RwLock<Lambda>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
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
        subtype: &Arc<RwLock<ListElement>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
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
        subtype: &Arc<RwLock<ListExpression>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
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
        subtype: &Arc<RwLock<Literal>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
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
        subtype: &Arc<RwLock<XMatch>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    subtype: ExpressionEnum::XMatch(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_z_none"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_z_none(store: &mut LuDogAsyncStore) -> Arc<RwLock<Expression>> {
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    subtype: ExpressionEnum::ZNone(Z_NONE),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_operator"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_operator(
        subtype: &Arc<RwLock<Operator>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    subtype: ExpressionEnum::Operator(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_x_print"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_x_print(
        subtype: &Arc<RwLock<XPrint>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
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
        subtype: &Arc<RwLock<RangeExpression>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
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
        subtype: &Arc<RwLock<XReturn>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    subtype: ExpressionEnum::XReturn(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_z_some"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_z_some(
        subtype: &Arc<RwLock<ZSome>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
                    subtype: ExpressionEnum::ZSome(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-new_struct_expression"}}}
    /// Inter a new Expression in the store, and return it's `id`.
    pub async fn new_struct_expression(
        subtype: &Arc<RwLock<StructExpression>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
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
        subtype: &Arc<RwLock<TypeCast>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
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
        subtype: &Arc<RwLock<VariableExpression>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Expression>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_expression(|id| {
                Arc::new(RwLock::new(Expression {
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
        span!("r37_argument");
        store.iter_argument().await.filter_map(|argument| async {
            if argument.read().await.expression == self.id {
                Some(argument)
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-call"}}}
    /// Navigate to [`Call`] across R29(1-Mc)
    pub async fn r29_call<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Call>>> + '_ {
        span!("r29_call");
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
        span!("r31_expression_statement");
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
        span!("r27_field_access");
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
        span!("r38_field_expression");
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
    /// Navigate to [`ForLoop`] across R42(1-M)
    pub async fn r42_for_loop<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<ForLoop>>> + '_ {
        span!("r42_for_loop");
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
        span!("r61_grouped");
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
        span!("r44_x_if");
        store.iter_x_if().await.filter_map(|x_if| async {
            if x_if.read().await.test == self.id {
                Some(x_if)
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
        span!("r56_index");
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
        span!("r57_index");
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
        span!("r55_list_element");
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
        span!("r91_x_match");
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
        span!("r51_operator");
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
        span!("r50_operator");
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
        span!("r92_pattern");
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
        span!("r32_x_print");
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
        span!("r58_range_expression");
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
        span!("r59_range_expression");
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
        span!("r41_result_statement");
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
        span!("r45_x_return");
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
    /// Navigate to [`StructField`] across R89(1-Mc)
    pub async fn r89_struct_field<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<StructField>>> + '_ {
        span!("r89_struct_field");
        store
            .iter_struct_field()
            .await
            .filter_map(move |struct_field| async move {
                if struct_field.read().await.expression == Some(self.id) {
                    Some(struct_field.clone())
                } else {
                    None
                }
            })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-tuple_field"}}}
    /// Navigate to [`TupleField`] across R90(1-Mc)
    pub async fn r90_tuple_field<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<TupleField>>> + '_ {
        span!("r90_tuple_field");
        store
            .iter_tuple_field()
            .await
            .filter_map(move |tuple_field| async move {
                if tuple_field.read().await.expression == Some(self.id) {
                    Some(tuple_field.clone())
                } else {
                    None
                }
            })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-type_cast"}}}
    /// Navigate to [`TypeCast`] across R68(1-M)
    pub async fn r68_type_cast<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<TypeCast>>> + '_ {
        span!("r68_type_cast");
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
        span!("r87_pattern");
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
        span!("r11_x_value");
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
        self.subtype == other.subtype
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
