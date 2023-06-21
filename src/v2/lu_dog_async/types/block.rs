// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"block-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::expression::Expression;
use crate::v2::lu_dog_async::types::for_loop::ForLoop;
use crate::v2::lu_dog_async::types::function::Function;
use crate::v2::lu_dog_async::types::statement::Statement;
use crate::v2::lu_dog_async::types::x_if::XIf;
use crate::v2::lu_dog_async::types::x_value::XValue;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-documentation"}}}
/// A Block Expression
///
/// A block expression is an expression that consists of an ordered set of statements, living
///  between an opening `{`, and a closing `}`.
///
///  Given that it's an expression it has a Type. The type is the value of the last expression
///  in the block, if it's not closed by a `;`. If the last statement is termintat thusly, then
///  the value is `[Empty]`, or `()`.
///
/// The `bug` attribute is there to force the compiler to generate code. Apparently there's
///  some bug in grace that's causing this to be generated as a const. I don't want to get into
///  it, and this is the most expedient solution.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Block {
    pub bug: Uuid,
    pub id: Uuid,
    /// R71: [`Block`] '' [`Statement`]
    pub statement: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-implementation"}}}
impl Block {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-new"}}}
    /// Inter a new 'Block' in the store, and return it's `id`.
    pub async fn new(
        bug: Uuid,
        statement: Option<&Arc<RwLock<Statement>>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Block>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Block {
            bug,
            id,
            statement: futures::future::OptionFuture::from(
                statement.map(|statement| async { statement.read().await.id }),
            )
            .await,
        }));
        store.inter_block(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-forward-cond-to-statement"}}}
    /// Navigate to [`Statement`] across R71(1-*c)
    pub async fn r71_statement<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Statement>>> {
        span!("r71_statement");
        match self.statement {
            Some(ref statement) => vec![store.exhume_statement(statement).await.unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-for_loop"}}}
    /// Navigate to [`ForLoop`] across R43(1-M)
    pub async fn r43_for_loop<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<ForLoop>>> {
        span!("r43_for_loop");
        let mut result = Vec::new();
        for for_loop in store.iter_for_loop().await {
            if for_loop.read().await.block == self.id {
                result.push(for_loop)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-cond-to-function"}}}
    /// Navigate to [`Function`] across R19(1-1c)
    pub async fn r19c_function<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Function>>> {
        span!("r19_function");
        let mut result = Vec::new();
        for function in store.iter_function().await {
            if function.read().await.block == self.id {
                result.push(function)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_Mc-to-x_if"}}}
    /// Navigate to [`XIf`] across R52(1-Mc)
    pub async fn r52_x_if<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<XIf>>> {
        use futures::stream::{self, StreamExt};
        span!("r52_x_if");
        stream::iter(store.iter_x_if().await.collect::<Vec<Arc<RwLock<XIf>>>>()).filter(
            |x_if: Arc<RwLock<XIf>>| async move {
                x_if.read().await.false_block == Some(self.id).collect().await
            },
        )
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-x_if"}}}
    /// Navigate to [`XIf`] across R46(1-M)
    pub async fn r46_x_if<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<XIf>>> {
        span!("r46_x_if");
        let mut result = Vec::new();
        for x_if in store.iter_x_if().await {
            if x_if.read().await.true_block == self.id {
                result.push(x_if)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-statement"}}}
    /// Navigate to [`Statement`] across R18(1-M)
    pub async fn r18_statement<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Statement>>> {
        span!("r18_statement");
        let mut result = Vec::new();
        for statement in store.iter_statement().await {
            if statement.read().await.block == self.id {
                result.push(statement)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-x_value"}}}
    /// Navigate to [`XValue`] across R33(1-M)
    pub async fn r33_x_value<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<XValue>>> {
        span!("r33_x_value");
        let mut result = Vec::new();
        for x_value in store.iter_x_value().await {
            if x_value.read().await.block == self.id {
                result.push(x_value)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub async fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r15_expression");
        vec![store.exhume_expression(&self.id).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
