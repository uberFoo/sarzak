// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"statement-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::block::Block;
use crate::v2::lu_dog_async::types::expression_statement::ExpressionStatement;
use crate::v2::lu_dog_async::types::item_statement::ITEM_STATEMENT;
use crate::v2::lu_dog_async::types::let_statement::LetStatement;
use crate::v2::lu_dog_async::types::result_statement::ResultStatement;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-hybrid-documentation"}}}
/// A Statement
///
/// A statement is followed by a semi-colon (`;`), and in general yields no value.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Statement {
    pub subtype: StatementEnum,
    pub id: Uuid,
    /// R18: [`Statement`] 'is contianed in a' [`Block`]
    pub block: Uuid,
    /// R17: [`Statement`] 'follows' [`Statement`]
    pub next: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum StatementEnum {
    ExpressionStatement(Uuid),
    ItemStatement(Uuid),
    LetStatement(Uuid),
    ResultStatement(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-implementation"}}}
impl Statement {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-new_expression_statement"}}}
    /// Inter a new Statement in the store, and return it's `id`.
    pub async fn new_expression_statement(
        block: &Arc<RwLock<Block>>,
        next: Option<&Arc<RwLock<Statement>>>,
        subtype: &Arc<RwLock<ExpressionStatement>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Statement>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Statement {
            block: block.read().await.id,
            next: futures::future::OptionFuture::from(
                next.map(|statement| async { statement.read().await.id }),
            )
            .await,
            subtype: StatementEnum::ExpressionStatement(subtype.read().await.id),
            id,
        }));
        store.inter_statement(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-new_item_statement"}}}
    /// Inter a new Statement in the store, and return it's `id`.
    pub async fn new_item_statement(
        block: &Arc<RwLock<Block>>,
        next: Option<&Arc<RwLock<Statement>>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Statement>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Statement {
            block: block.read().await.id,
            next: futures::future::OptionFuture::from(
                next.map(|statement| async { statement.read().await.id }),
            )
            .await,
            subtype: StatementEnum::ItemStatement(ITEM_STATEMENT),
            id,
        }));
        store.inter_statement(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-new_let_statement"}}}
    /// Inter a new Statement in the store, and return it's `id`.
    pub async fn new_let_statement(
        block: &Arc<RwLock<Block>>,
        next: Option<&Arc<RwLock<Statement>>>,
        subtype: &Arc<RwLock<LetStatement>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Statement>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Statement {
            block: block.read().await.id,
            next: futures::future::OptionFuture::from(
                next.map(|statement| async { statement.read().await.id }),
            )
            .await,
            subtype: StatementEnum::LetStatement(subtype.read().await.id),
            id,
        }));
        store.inter_statement(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-new_result_statement"}}}
    /// Inter a new Statement in the store, and return it's `id`.
    pub async fn new_result_statement(
        block: &Arc<RwLock<Block>>,
        next: Option<&Arc<RwLock<Statement>>>,
        subtype: &Arc<RwLock<ResultStatement>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Statement>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Statement {
            block: block.read().await.id,
            next: futures::future::OptionFuture::from(
                next.map(|statement| async { statement.read().await.id }),
            )
            .await,
            subtype: StatementEnum::ResultStatement(subtype.read().await.id),
            id,
        }));
        store.inter_statement(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-nav-forward-to-block"}}}
    /// Navigate to [`Block`] across R18(1-*)
    pub async fn r18_block<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Block>>> {
        span!("r18_block");
        vec![store.exhume_block(&self.block).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`Statement`] across R17(1-*c)
    pub async fn r17_statement<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Statement>>> {
        span!("r17_statement");
        match self.next {
            Some(ref next) => vec![store.exhume_statement(next).await.unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-nav-backward-one-bi-cond-to-block"}}}
    /// Navigate to [`Block`] across R71(1c-1c)
    pub async fn r71c_block<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Block>>> {
        span!("r71_block");
        let mut result = Vec::new();
        for block in store.iter_block().await {
            if block.read().await.statement == Some(self.id) {
                result.push(block.clone());
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-nav-backward-one-bi-cond-to-statement"}}}
    /// Navigate to [`Statement`] across R17(1c-1c)
    pub async fn r17c_statement<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Statement>>> {
        span!("r17_statement");
        let mut result = Vec::new();
        for statement in store.iter_statement().await {
            if statement.read().await.next == Some(self.id) {
                result.push(statement.clone());
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
