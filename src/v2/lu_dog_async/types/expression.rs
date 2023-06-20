// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-use-statements"}}}
use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
use crate::v2::lu_dog_async::types::argument::Argument;
use crate::v2::lu_dog_async::types::block::Block;
use crate::v2::lu_dog_async::types::call::Call;
use crate::v2::lu_dog_async::types::debugger::DEBUGGER;
use crate::v2::lu_dog_async::types::error_expression::ErrorExpression;
use crate::v2::lu_dog_async::types::expression_statement::ExpressionStatement;
use crate::v2::lu_dog_async::types::field_access::FieldAccess;
use crate::v2::lu_dog_async::types::field_expression::FieldExpression;
use crate::v2::lu_dog_async::types::for_loop::ForLoop;
use crate::v2::lu_dog_async::types::grouped::Grouped;
use crate::v2::lu_dog_async::types::index::Index;
use crate::v2::lu_dog_async::types::let_statement::LetStatement;
use crate::v2::lu_dog_async::types::list_element::ListElement;
use crate::v2::lu_dog_async::types::list_expression::ListExpression;
use crate::v2::lu_dog_async::types::literal::Literal;
use crate::v2::lu_dog_async::types::operator::Operator;
use crate::v2::lu_dog_async::types::print::Print;
use crate::v2::lu_dog_async::types::range_expression::RangeExpression;
use crate::v2::lu_dog_async::types::result_statement::ResultStatement;
use crate::v2::lu_dog_async::types::struct_expression::StructExpression;
use crate::v2::lu_dog_async::types::type_cast::TypeCast;
use crate::v2::lu_dog_async::types::variable_expression::VariableExpression;
use crate::v2::lu_dog_async::types::x_if::XIf;
use crate::v2::lu_dog_async::types::x_return::XReturn;
use crate::v2::lu_dog_async::types::x_value::XValue;
use crate::v2::lu_dog_async::types::x_value::XValueEnum;
use crate::v2::lu_dog_async::types::z_none::Z_NONE;
use crate::v2::lu_dog_async::types::z_some::ZSome;
use async_std::sync::Arc;
use async_std::sync::RwLock;
use serde::{Deserialize, Serialize};
use tracy_client::span;
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
    pub async fn new_block(
        block: &Arc<RwLock<Block>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = block.read().await.id;
        if let Some(block) = store.exhume_expression(&id).await {
            block
        } else {
            let new = Arc::new(RwLock::new(Self::Block(id)));
            store.inter_expression(new.clone()).await;
            new
        }
    }

    /// Create a new instance of Expression::Call
    pub async fn new_call(
        call: &Arc<RwLock<Call>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = call.read().await.id;
        if let Some(call) = store.exhume_expression(&id).await {
            call
        } else {
            let new = Arc::new(RwLock::new(Self::Call(id)));
            store.inter_expression(new.clone()).await;
            new
        }
    }

    /// Create a new instance of Expression::Debugger
    pub async fn new_debugger(store: &LuDogAsyncStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_expression(&DEBUGGER).await.unwrap()
    }

    /// Create a new instance of Expression::ErrorExpression
    pub async fn new_error_expression(
        error_expression: &Arc<RwLock<ErrorExpression>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = error_expression.read().await.id;
        if let Some(error_expression) = store.exhume_expression(&id).await {
            error_expression
        } else {
            let new = Arc::new(RwLock::new(Self::ErrorExpression(id)));
            store.inter_expression(new.clone()).await;
            new
        }
    }

    /// Create a new instance of Expression::FieldAccess
    pub async fn new_field_access(
        field_access: &Arc<RwLock<FieldAccess>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = field_access.read().await.id;
        if let Some(field_access) = store.exhume_expression(&id).await {
            field_access
        } else {
            let new = Arc::new(RwLock::new(Self::FieldAccess(id)));
            store.inter_expression(new.clone()).await;
            new
        }
    }

    /// Create a new instance of Expression::FieldExpression
    pub async fn new_field_expression(
        field_expression: &Arc<RwLock<FieldExpression>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = field_expression.read().await.id;
        if let Some(field_expression) = store.exhume_expression(&id).await {
            field_expression
        } else {
            let new = Arc::new(RwLock::new(Self::FieldExpression(id)));
            store.inter_expression(new.clone()).await;
            new
        }
    }

    /// Create a new instance of Expression::ForLoop
    pub async fn new_for_loop(
        for_loop: &Arc<RwLock<ForLoop>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = for_loop.read().await.id;
        if let Some(for_loop) = store.exhume_expression(&id).await {
            for_loop
        } else {
            let new = Arc::new(RwLock::new(Self::ForLoop(id)));
            store.inter_expression(new.clone()).await;
            new
        }
    }

    /// Create a new instance of Expression::Grouped
    pub async fn new_grouped(
        grouped: &Arc<RwLock<Grouped>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = grouped.read().await.id;
        if let Some(grouped) = store.exhume_expression(&id).await {
            grouped
        } else {
            let new = Arc::new(RwLock::new(Self::Grouped(id)));
            store.inter_expression(new.clone()).await;
            new
        }
    }

    /// Create a new instance of Expression::XIf
    pub async fn new_x_if(
        x_if: &Arc<RwLock<XIf>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = x_if.read().await.id;
        if let Some(x_if) = store.exhume_expression(&id).await {
            x_if
        } else {
            let new = Arc::new(RwLock::new(Self::XIf(id)));
            store.inter_expression(new.clone()).await;
            new
        }
    }

    /// Create a new instance of Expression::Index
    pub async fn new_index(
        index: &Arc<RwLock<Index>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = index.read().await.id;
        if let Some(index) = store.exhume_expression(&id).await {
            index
        } else {
            let new = Arc::new(RwLock::new(Self::Index(id)));
            store.inter_expression(new.clone()).await;
            new
        }
    }

    /// Create a new instance of Expression::ListElement
    pub async fn new_list_element(
        list_element: &Arc<RwLock<ListElement>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = list_element.read().await.id;
        if let Some(list_element) = store.exhume_expression(&id).await {
            list_element
        } else {
            let new = Arc::new(RwLock::new(Self::ListElement(id)));
            store.inter_expression(new.clone()).await;
            new
        }
    }

    /// Create a new instance of Expression::ListExpression
    pub async fn new_list_expression(
        list_expression: &Arc<RwLock<ListExpression>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = list_expression.read().await.id;
        if let Some(list_expression) = store.exhume_expression(&id).await {
            list_expression
        } else {
            let new = Arc::new(RwLock::new(Self::ListExpression(id)));
            store.inter_expression(new.clone()).await;
            new
        }
    }

    /// Create a new instance of Expression::Literal
    pub async fn new_literal(
        literal: &Arc<RwLock<Literal>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = literal.read().await.id();
        if let Some(literal) = store.exhume_expression(&id).await {
            literal
        } else {
            let new = Arc::new(RwLock::new(Self::Literal(id)));
            store.inter_expression(new.clone()).await;
            new
        }
    }

    /// Create a new instance of Expression::ZNone
    pub async fn new_z_none(store: &LuDogAsyncStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_expression(&Z_NONE).await.unwrap()
    }

    /// Create a new instance of Expression::Operator
    pub async fn new_operator(
        operator: &Arc<RwLock<Operator>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = operator.read().await.id;
        if let Some(operator) = store.exhume_expression(&id).await {
            operator
        } else {
            let new = Arc::new(RwLock::new(Self::Operator(id)));
            store.inter_expression(new.clone()).await;
            new
        }
    }

    /// Create a new instance of Expression::Print
    pub async fn new_print(
        print: &Arc<RwLock<Print>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = print.read().await.id;
        if let Some(print) = store.exhume_expression(&id).await {
            print
        } else {
            let new = Arc::new(RwLock::new(Self::Print(id)));
            store.inter_expression(new.clone()).await;
            new
        }
    }

    /// Create a new instance of Expression::RangeExpression
    pub async fn new_range_expression(
        range_expression: &Arc<RwLock<RangeExpression>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = range_expression.read().await.id;
        if let Some(range_expression) = store.exhume_expression(&id).await {
            range_expression
        } else {
            let new = Arc::new(RwLock::new(Self::RangeExpression(id)));
            store.inter_expression(new.clone()).await;
            new
        }
    }

    /// Create a new instance of Expression::XReturn
    pub async fn new_x_return(
        x_return: &Arc<RwLock<XReturn>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = x_return.read().await.id;
        if let Some(x_return) = store.exhume_expression(&id).await {
            x_return
        } else {
            let new = Arc::new(RwLock::new(Self::XReturn(id)));
            store.inter_expression(new.clone()).await;
            new
        }
    }

    /// Create a new instance of Expression::ZSome
    pub async fn new_z_some(
        z_some: &Arc<RwLock<ZSome>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = z_some.read().await.id;
        if let Some(z_some) = store.exhume_expression(&id).await {
            z_some
        } else {
            let new = Arc::new(RwLock::new(Self::ZSome(id)));
            store.inter_expression(new.clone()).await;
            new
        }
    }

    /// Create a new instance of Expression::StructExpression
    pub async fn new_struct_expression(
        struct_expression: &Arc<RwLock<StructExpression>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = struct_expression.read().await.id;
        if let Some(struct_expression) = store.exhume_expression(&id).await {
            struct_expression
        } else {
            let new = Arc::new(RwLock::new(Self::StructExpression(id)));
            store.inter_expression(new.clone()).await;
            new
        }
    }

    /// Create a new instance of Expression::TypeCast
    pub async fn new_type_cast(
        type_cast: &Arc<RwLock<TypeCast>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = type_cast.read().await.id;
        if let Some(type_cast) = store.exhume_expression(&id).await {
            type_cast
        } else {
            let new = Arc::new(RwLock::new(Self::TypeCast(id)));
            store.inter_expression(new.clone()).await;
            new
        }
    }

    /// Create a new instance of Expression::VariableExpression
    pub async fn new_variable_expression(
        variable_expression: &Arc<RwLock<VariableExpression>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = variable_expression.read().await.id;
        if let Some(variable_expression) = store.exhume_expression(&id).await {
            variable_expression
        } else {
            let new = Arc::new(RwLock::new(Self::VariableExpression(id)));
            store.inter_expression(new.clone()).await;
            new
        }
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
    pub async fn r37_argument<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Argument>>> {
        span!("r37_argument");
        let mut result = Vec::new();
        for argument in store.iter_argument().await {
            if argument.read().await.expression == self.id() {
                result.push(argument)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-call"}}}
    /// Navigate to [`Call`] across R29(1-Mc)
    pub async fn r29_call<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Call>>> {
        use futures::stream::{self, StreamExt};
        span!("r29_call");
        stream::iter(store.iter_call().await.collect::<Vec<Arc<RwLock<Call>>>>())
            .filter_map(|call: Arc<RwLock<Call>>| async move {
                if call.read().await.expression == Some(self.id()) {
                    Some(call.clone())
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-expression_statement"}}}
    /// Navigate to [`ExpressionStatement`] across R31(1-M)
    pub async fn r31_expression_statement<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<ExpressionStatement>>> {
        span!("r31_expression_statement");
        let mut result = Vec::new();
        for expression_statement in store.iter_expression_statement().await {
            if expression_statement.read().await.expression == self.id() {
                result.push(expression_statement)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-field_access"}}}
    /// Navigate to [`FieldAccess`] across R27(1-M)
    pub async fn r27_field_access<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<FieldAccess>>> {
        span!("r27_field_access");
        let mut result = Vec::new();
        for field_access in store.iter_field_access().await {
            if field_access.read().await.expression == self.id() {
                result.push(field_access)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-field_expression"}}}
    /// Navigate to [`FieldExpression`] across R38(1-M)
    pub async fn r38_field_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<FieldExpression>>> {
        span!("r38_field_expression");
        let mut result = Vec::new();
        for field_expression in store.iter_field_expression().await {
            if field_expression.read().await.expression == self.id() {
                result.push(field_expression)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-for_loop"}}}
    /// Navigate to [`ForLoop`] across R42(1-M)
    pub async fn r42_for_loop<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<ForLoop>>> {
        span!("r42_for_loop");
        let mut result = Vec::new();
        for for_loop in store.iter_for_loop().await {
            if for_loop.read().await.expression == self.id() {
                result.push(for_loop)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-grouped"}}}
    /// Navigate to [`Grouped`] across R61(1-M)
    pub async fn r61_grouped<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Grouped>>> {
        span!("r61_grouped");
        let mut result = Vec::new();
        for grouped in store.iter_grouped().await {
            if grouped.read().await.expression == self.id() {
                result.push(grouped)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-x_if"}}}
    /// Navigate to [`XIf`] across R44(1-M)
    pub async fn r44_x_if<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<XIf>>> {
        span!("r44_x_if");
        let mut result = Vec::new();
        for x_if in store.iter_x_if().await {
            if x_if.read().await.test == self.id() {
                result.push(x_if)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-index"}}}
    /// Navigate to [`Index`] across R56(1-M)
    pub async fn r56_index<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Index>>> {
        span!("r56_index");
        let mut result = Vec::new();
        for index in store.iter_index().await {
            if index.read().await.index == self.id() {
                result.push(index)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-index"}}}
    /// Navigate to [`Index`] across R57(1-M)
    pub async fn r57_index<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Index>>> {
        span!("r57_index");
        let mut result = Vec::new();
        for index in store.iter_index().await {
            if index.read().await.target == self.id() {
                result.push(index)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-cond-to-let_statement"}}}
    /// Navigate to [`LetStatement`] across R20(1-1c)
    pub async fn r20c_let_statement<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<LetStatement>>> {
        span!("r20_let_statement");
        let mut result = Vec::new();
        for let_statement in store.iter_let_statement().await {
            if let_statement.read().await.expression == self.id() {
                result.push(let_statement)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-list_element"}}}
    /// Navigate to [`ListElement`] across R55(1-M)
    pub async fn r55_list_element<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<ListElement>>> {
        span!("r55_list_element");
        let mut result = Vec::new();
        for list_element in store.iter_list_element().await {
            if list_element.read().await.expression == self.id() {
                result.push(list_element)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-operator"}}}
    /// Navigate to [`Operator`] across R50(1-M)
    pub async fn r50_operator<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Operator>>> {
        span!("r50_operator");
        let mut result = Vec::new();
        for operator in store.iter_operator().await {
            if operator.read().await.lhs == self.id() {
                result.push(operator)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-operator"}}}
    /// Navigate to [`Operator`] across R51(1-Mc)
    pub async fn r51_operator<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Operator>>> {
        use futures::stream::{self, StreamExt};
        span!("r51_operator");
        stream::iter(
            store
                .iter_operator()
                .await
                .collect::<Vec<Arc<RwLock<Operator>>>>(),
        )
        .filter_map(|operator: Arc<RwLock<Operator>>| async move {
            if operator.read().await.rhs == Some(self.id()) {
                Some(operator.clone())
            } else {
                None
            }
        })
        .collect()
        .await
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-operator"}}}
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-operator"}}}
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-operator"}}}
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-operator"}}}
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-operator"}}}
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-operator"}}}
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-operator"}}}
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-operator"}}}
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-operator"}}}
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-operator"}}}
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-print"}}}
    /// Navigate to [`Print`] across R32(1-M)
    pub async fn r32_print<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Print>>> {
        span!("r32_print");
        let mut result = Vec::new();
        for print in store.iter_print().await {
            if print.read().await.expression == self.id() {
                result.push(print)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-range_expression"}}}
    /// Navigate to [`RangeExpression`] across R59(1-Mc)
    pub async fn r59_range_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<RangeExpression>>> {
        use futures::stream::{self, StreamExt};
        span!("r59_range_expression");
        stream::iter(
            store
                .iter_range_expression()
                .await
                .collect::<Vec<Arc<RwLock<RangeExpression>>>>(),
        )
        .filter_map(
            |range_expression: Arc<RwLock<RangeExpression>>| async move {
                if range_expression.read().await.rhs == Some(self.id()) {
                    Some(range_expression.clone())
                } else {
                    None
                }
            },
        )
        .collect()
        .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_Mc-to-range_expression"}}}
    /// Navigate to [`RangeExpression`] across R58(1-Mc)
    pub async fn r58_range_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<RangeExpression>>> {
        use futures::stream::{self, StreamExt};
        span!("r58_range_expression");
        stream::iter(
            store
                .iter_range_expression()
                .await
                .collect::<Vec<Arc<RwLock<RangeExpression>>>>(),
        )
        .filter_map(
            |range_expression: Arc<RwLock<RangeExpression>>| async move {
                if range_expression.read().await.lhs == Some(self.id()) {
                    Some(range_expression.clone())
                } else {
                    None
                }
            },
        )
        .collect()
        .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-result_statement"}}}
    /// Navigate to [`ResultStatement`] across R41(1-M)
    pub async fn r41_result_statement<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<ResultStatement>>> {
        span!("r41_result_statement");
        let mut result = Vec::new();
        for result_statement in store.iter_result_statement().await {
            if result_statement.read().await.expression == self.id() {
                result.push(result_statement)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-x_return"}}}
    /// Navigate to [`XReturn`] across R45(1-M)
    pub async fn r45_x_return<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<XReturn>>> {
        span!("r45_x_return");
        let mut result = Vec::new();
        for x_return in store.iter_x_return().await {
            if x_return.read().await.expression == self.id() {
                result.push(x_return)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-struct-impl-nav-backward-1_M-to-type_cast"}}}
    /// Navigate to [`TypeCast`] across R68(1-M)
    pub async fn r68_type_cast<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<TypeCast>>> {
        span!("r68_type_cast");
        let mut result = Vec::new();
        for type_cast in store.iter_type_cast().await {
            if type_cast.read().await.lhs == self.id() {
                result.push(type_cast)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"expression-impl-nav-subtype-to-supertype-x_value"}}}
    // Navigate to [`XValue`] across R11(isa)
    pub async fn r11_x_value<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<XValue>>> {
        span!("r11_x_value");
        let mut result = Vec::new();
        for x_value in store.iter_x_value().await {
            if let XValueEnum::Expression(id) = x_value.read().await.subtype {
                result.push(x_value.clone());
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
