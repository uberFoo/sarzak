// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"block-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::body::Body;
use crate::v2::lu_dog_async::types::body::BodyEnum;
use crate::v2::lu_dog_async::types::expression::Expression;
use crate::v2::lu_dog_async::types::expression::ExpressionEnum;
use crate::v2::lu_dog_async::types::for_loop::ForLoop;
use crate::v2::lu_dog_async::types::lambda::Lambda;
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Block {
    pub bug: Uuid,
    pub id: usize,
    /// R93: [`Block`] '' [`Block`]
    pub next: Option<usize>,
    /// R71: [`Block`] '' [`Statement`]
    pub statement: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-implementation"}}}
impl Block {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-new"}}}
    /// Inter a new 'Block' in the store, and return it's `id`.
    pub async fn new(
        bug: Uuid,
        next: Option<&Arc<RwLock<Block>>>,
        statement: Option<&Arc<RwLock<Statement>>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Block>> {
        let block = match next {
            Some(block) => Some(block.read().await.id),
            None => None,
        };
        let statement = match statement {
            Some(statement) => Some(statement.read().await.id),
            None => None,
        };
        store
            .inter_block(|id| {
                Arc::new(RwLock::new(Block {
                    bug,
                    id,
                    next: block,
                    statement,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`Block`] across R93(1-*c)
    pub async fn r93_block<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Block>>> {
        span!("r93_block");
        match self.next {
            Some(ref next) => vec![store.exhume_block(next).await.unwrap()],
            None => Vec::new(),
        }
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
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-one-bi-cond-to-block"}}}
    /// Navigate to [`Block`] across R93(1c-1c)
    pub async fn r93c_block<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Block>>> {
        span!("r93_block");
        store
            .iter_block()
            .await
            .filter_map(|block| async move {
                if block.read().await.next == Some(self.id) {
                    Some(block.clone())
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-for_loop"}}}
    /// Navigate to [`ForLoop`] across R43(1-M)
    pub async fn r43_for_loop<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<ForLoop>>> {
        span!("r43_for_loop");
        store
            .iter_for_loop()
            .await
            .filter_map(|for_loop| async {
                if for_loop.read().await.block == self.id {
                    Some(for_loop)
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-x_if"}}}
    /// Navigate to [`XIf`] across R46(1-M)
    pub async fn r46_x_if<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<XIf>>> {
        span!("r46_x_if");
        store
            .iter_x_if()
            .await
            .filter_map(|x_if| async {
                if x_if.read().await.true_block == self.id {
                    Some(x_if)
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_Mc-to-x_if"}}}
    /// Navigate to [`XIf`] across R52(1-Mc)
    pub async fn r52_x_if<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<XIf>>> {
        span!("r52_x_if");
        store
            .iter_x_if()
            .await
            .filter_map(|x_if| async move {
                if x_if.read().await.false_block == Some(self.id) {
                    Some(x_if.clone())
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-one-bi-cond-to-lambda"}}}
    /// Navigate to [`Lambda`] across R73(1c-1c)
    pub async fn r73c_lambda<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Lambda>>> {
        span!("r73_lambda");
        store
            .iter_lambda()
            .await
            .filter_map(|lambda| async move {
                if lambda.read().await.block == Some(self.id) {
                    Some(lambda.clone())
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-statement"}}}
    /// Navigate to [`Statement`] across R18(1-M)
    pub async fn r18_statement<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Statement>>> {
        span!("r18_statement");
        store
            .iter_statement()
            .await
            .filter_map(|statement| async {
                if statement.read().await.block == self.id {
                    Some(statement)
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-struct-impl-nav-backward-1_M-to-x_value"}}}
    /// Navigate to [`XValue`] across R33(1-M)
    pub async fn r33_x_value<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<XValue>>> {
        span!("r33_x_value");
        store
            .iter_x_value()
            .await
            .filter_map(|x_value| async {
                if x_value.read().await.block == self.id {
                    Some(x_value)
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-impl-nav-subtype-to-supertype-body"}}}
    // Navigate to [`Body`] across R80(isa)
    pub async fn r80_body<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Body>>> {
        span!("r80_body");
        store
            .iter_body()
            .await
            .filter_map(|body| async move {
                if let BodyEnum::Block(id) = body.read().await.subtype {
                    Some(body.clone())
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub async fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r15_expression");
        store
            .iter_expression()
            .await
            .filter_map(|expression| async move {
                if let ExpressionEnum::Block(id) = expression.read().await.subtype {
                    Some(expression.clone())
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"block-implementation"}}}
impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.bug == other.bug && self.next == other.next && self.statement == other.statement
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
