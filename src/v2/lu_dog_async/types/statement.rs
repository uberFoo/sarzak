// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"statement-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Statement {
    pub subtype: StatementEnum,
    pub id: usize,
    pub index: i64,
    /// R18: [`Statement`] 'is contianed in a' [`Block`]
    pub block: usize,
    /// R17: [`Statement`] 'follows' [`Statement`]
    pub next: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum StatementEnum {
    ExpressionStatement(usize),
    ItemStatement(Uuid),
    LetStatement(usize),
    ResultStatement(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-implementation"}}}
impl Statement {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-new_expression_statement"}}}
    /// Inter a new Statement in the store, and return it's `id`.
    pub async fn new_expression_statement(
        index: i64,
        block: &Arc<RwLock<Block>>,
        next: Option<&Arc<RwLock<Statement>>>,
        subtype: &Arc<RwLock<ExpressionStatement>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Statement>> {
        let s_id = subtype.read().await.id;
        let block = block.read().await.id;
        let next = match next {
            Some(statement) => Some(statement.read().await.id),
            None => None,
        };
        let subtype = subtype.read().await.id;
        store
            .inter_statement(|id| {
                Arc::new(RwLock::new(Statement {
                    index: index,
                    block, // (b)
                    next,  // (a)
                    subtype: StatementEnum::ExpressionStatement(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-new_item_statement"}}}
    /// Inter a new Statement in the store, and return it's `id`.
    pub async fn new_item_statement(
        index: i64,
        block: &Arc<RwLock<Block>>,
        next: Option<&Arc<RwLock<Statement>>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Statement>> {
        let block = block.read().await.id;
        let next = match next {
            Some(statement) => Some(statement.read().await.id),
            None => None,
        };
        store
            .inter_statement(|id| {
                Arc::new(RwLock::new(Statement {
                    index: index,
                    block, // (b)
                    next,  // (a)
                    subtype: StatementEnum::ItemStatement(ITEM_STATEMENT),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-new_let_statement"}}}
    /// Inter a new Statement in the store, and return it's `id`.
    pub async fn new_let_statement(
        index: i64,
        block: &Arc<RwLock<Block>>,
        next: Option<&Arc<RwLock<Statement>>>,
        subtype: &Arc<RwLock<LetStatement>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Statement>> {
        let s_id = subtype.read().await.id;
        let block = block.read().await.id;
        let next = match next {
            Some(statement) => Some(statement.read().await.id),
            None => None,
        };
        let subtype = subtype.read().await.id;
        store
            .inter_statement(|id| {
                Arc::new(RwLock::new(Statement {
                    index: index,
                    block, // (b)
                    next,  // (a)
                    subtype: StatementEnum::LetStatement(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-new_result_statement"}}}
    /// Inter a new Statement in the store, and return it's `id`.
    pub async fn new_result_statement(
        index: i64,
        block: &Arc<RwLock<Block>>,
        next: Option<&Arc<RwLock<Statement>>>,
        subtype: &Arc<RwLock<ResultStatement>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Statement>> {
        let s_id = subtype.read().await.id;
        let block = block.read().await.id;
        let next = match next {
            Some(statement) => Some(statement.read().await.id),
            None => None,
        };
        let subtype = subtype.read().await.id;
        store
            .inter_statement(|id| {
                Arc::new(RwLock::new(Statement {
                    index: index,
                    block, // (b)
                    next,  // (a)
                    subtype: StatementEnum::ResultStatement(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-nav-forward-to-block"}}}
    /// Navigate to [`Block`] across R18(1-*)
    pub async fn r18_block<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Block>>> + '_ {
        stream::iter(vec![store.exhume_block(&self.block).await.unwrap()].into_iter())
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`Statement`] across R17(1-*c)
    pub async fn r17_statement<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Statement>>> + '_ {
        match self.next {
            Some(ref next) => {
                stream::iter(vec![store.exhume_statement(next).await.unwrap()].into_iter())
            }
            None => stream::iter(vec![].into_iter()),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-nav-backward-one-bi-cond-to-block"}}}
    /// Navigate to [`Block`] across R71(1c-1c)
    pub async fn r71c_block<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Block>>> + '_ {
        store
            .iter_block()
            .await
            .filter_map(move |block| async move {
                if block.read().await.statement == Some(self.id) {
                    Some(block.clone())
                } else {
                    None
                }
            })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-nav-backward-one-bi-cond-to-statement"}}}
    /// Navigate to [`Statement`] across R17(1c-1c)
    pub async fn r17c_statement<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Statement>>> + '_ {
        store
            .iter_statement()
            .await
            .filter_map(move |statement| async move {
                if statement.read().await.next == Some(self.id) {
                    Some(statement.clone())
                } else {
                    None
                }
            })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-implementation"}}}
impl PartialEq for Statement {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype
            && self.index == other.index
            && self.block == other.block
            && self.next == other.next
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
