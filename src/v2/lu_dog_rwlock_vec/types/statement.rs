// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"statement-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock_vec::types::block::Block;
use crate::v2::lu_dog_rwlock_vec::types::expression_statement::ExpressionStatement;
use crate::v2::lu_dog_rwlock_vec::types::item_statement::ITEM_STATEMENT;
use crate::v2::lu_dog_rwlock_vec::types::let_statement::LetStatement;
use crate::v2::lu_dog_rwlock_vec::types::result_statement::ResultStatement;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock_vec::store::ObjectStore as LuDogRwlockVecStore;
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
    pub fn new_expression_statement(
        index: i64,
        block: &Arc<RwLock<Block>>,
        next: Option<&Arc<RwLock<Statement>>>,
        subtype: &Arc<RwLock<ExpressionStatement>>,
        store: &mut LuDogRwlockVecStore,
    ) -> Arc<RwLock<Statement>> {
        store.inter_statement(|id| {
            Arc::new(RwLock::new(Statement {
                index: index,
                block: block.read().unwrap().id,
                next: next.map(|statement| statement.read().unwrap().id),
                subtype: StatementEnum::ExpressionStatement(subtype.read().unwrap().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-new_item_statement"}}}
    /// Inter a new Statement in the store, and return it's `id`.
    pub fn new_item_statement(
        index: i64,
        block: &Arc<RwLock<Block>>,
        next: Option<&Arc<RwLock<Statement>>>,
        store: &mut LuDogRwlockVecStore,
    ) -> Arc<RwLock<Statement>> {
        store.inter_statement(|id| {
            Arc::new(RwLock::new(Statement {
                index: index,
                block: block.read().unwrap().id,
                next: next.map(|statement| statement.read().unwrap().id),
                subtype: StatementEnum::ItemStatement(ITEM_STATEMENT),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-new_let_statement"}}}
    /// Inter a new Statement in the store, and return it's `id`.
    pub fn new_let_statement(
        index: i64,
        block: &Arc<RwLock<Block>>,
        next: Option<&Arc<RwLock<Statement>>>,
        subtype: &Arc<RwLock<LetStatement>>,
        store: &mut LuDogRwlockVecStore,
    ) -> Arc<RwLock<Statement>> {
        store.inter_statement(|id| {
            Arc::new(RwLock::new(Statement {
                index: index,
                block: block.read().unwrap().id,
                next: next.map(|statement| statement.read().unwrap().id),
                subtype: StatementEnum::LetStatement(subtype.read().unwrap().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-new_result_statement"}}}
    /// Inter a new Statement in the store, and return it's `id`.
    pub fn new_result_statement(
        index: i64,
        block: &Arc<RwLock<Block>>,
        next: Option<&Arc<RwLock<Statement>>>,
        subtype: &Arc<RwLock<ResultStatement>>,
        store: &mut LuDogRwlockVecStore,
    ) -> Arc<RwLock<Statement>> {
        store.inter_statement(|id| {
            Arc::new(RwLock::new(Statement {
                index: index,
                block: block.read().unwrap().id,
                next: next.map(|statement| statement.read().unwrap().id),
                subtype: StatementEnum::ResultStatement(subtype.read().unwrap().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-nav-forward-to-block"}}}
    /// Navigate to [`Block`] across R18(1-*)
    pub fn r18_block<'a>(&'a self, store: &'a LuDogRwlockVecStore) -> Vec<Arc<RwLock<Block>>> {
        vec![store.exhume_block(&self.block).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`Statement`] across R17(1-*c)
    pub fn r17_statement<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<Statement>>> {
        match self.next {
            Some(ref next) => vec![store.exhume_statement(&next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-nav-backward-one-bi-cond-to-block"}}}
    /// Navigate to [`Block`] across R71(1c-1c)
    pub fn r71c_block<'a>(&'a self, store: &'a LuDogRwlockVecStore) -> Vec<Arc<RwLock<Block>>> {
        let block = store
            .iter_block()
            .find(|block| block.read().unwrap().statement == Some(self.id));
        match block {
            Some(ref block) => vec![block.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-nav-backward-one-bi-cond-to-statement"}}}
    /// Navigate to [`Statement`] across R17(1c-1c)
    pub fn r17c_statement<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<Statement>>> {
        let statement = store
            .iter_statement()
            .find(|statement| statement.read().unwrap().next == Some(self.id));
        match statement {
            Some(ref statement) => vec![statement.clone()],
            None => Vec::new(),
        }
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
