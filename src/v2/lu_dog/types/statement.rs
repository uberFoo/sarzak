// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"statement-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-use-statements"}}}
use std::{sync::Arc, sync::RwLock};

use uuid::Uuid;

use crate::v2::lu_dog::types::block::Block;
use crate::v2::lu_dog::types::expression_statement::ExpressionStatement;
use crate::v2::lu_dog::types::item_statement::ITEM_STATEMENT;
use crate::v2::lu_dog::types::let_statement::LetStatement;
use crate::v2::lu_dog::types::result_statement::ResultStatement;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
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
    pub fn new_expression_statement(
        block: &Block,
        next: Option<&Statement>,
        subtype: &ExpressionStatement,
        store: &mut LuDogStore,
    ) -> Arc<RwLock<Statement>> {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id;
        let new = Arc::new(RwLock::new(Statement {
            block: block.id,
            next: next.map(|statement| statement.id),
            subtype: StatementEnum::ExpressionStatement(subtype.id),
            id,
        }));
        store.inter_statement(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-new_expression_statement_"}}}
    /// Inter a new Statement in the store, and return it's `id`.
    pub fn new_expression_statement_(
        block: &Block,
        next: Option<&Statement>,
        subtype: &ExpressionStatement,
    ) -> Statement {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id;
        let new = Statement {
            block: block.id,
            next: next.map(|statement| statement.id),
            subtype: StatementEnum::ExpressionStatement(subtype.id),
            id,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-new_item_statement"}}}
    /// Inter a new Statement in the store, and return it's `id`.
    pub fn new_item_statement(
        block: &Block,
        next: Option<&Statement>,
        store: &mut LuDogStore,
    ) -> Arc<RwLock<Statement>> {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = ITEM_STATEMENT;
        let new = Arc::new(RwLock::new(Statement {
            block: block.id,
            next: next.map(|statement| statement.id),
            subtype: StatementEnum::ItemStatement(ITEM_STATEMENT),
            id,
        }));
        store.inter_statement(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-new_item_statement_"}}}
    /// Inter a new Statement in the store, and return it's `id`.
    pub fn new_item_statement_(block: &Block, next: Option<&Statement>) -> Statement {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = ITEM_STATEMENT;
        let new = Statement {
            block: block.id,
            next: next.map(|statement| statement.id),
            subtype: StatementEnum::ItemStatement(ITEM_STATEMENT),
            id,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-new_let_statement"}}}
    /// Inter a new Statement in the store, and return it's `id`.
    pub fn new_let_statement(
        block: &Block,
        next: Option<&Statement>,
        subtype: &LetStatement,
        store: &mut LuDogStore,
    ) -> Arc<RwLock<Statement>> {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id;
        let new = Arc::new(RwLock::new(Statement {
            block: block.id,
            next: next.map(|statement| statement.id),
            subtype: StatementEnum::LetStatement(subtype.id),
            id,
        }));
        store.inter_statement(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-new_let_statement_"}}}
    /// Inter a new Statement in the store, and return it's `id`.
    pub fn new_let_statement_(
        block: &Block,
        next: Option<&Statement>,
        subtype: &LetStatement,
    ) -> Statement {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id;
        let new = Statement {
            block: block.id,
            next: next.map(|statement| statement.id),
            subtype: StatementEnum::LetStatement(subtype.id),
            id,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-new_result_statement"}}}
    /// Inter a new Statement in the store, and return it's `id`.
    pub fn new_result_statement(
        block: &Block,
        next: Option<&Statement>,
        subtype: &ResultStatement,
        store: &mut LuDogStore,
    ) -> Arc<RwLock<Statement>> {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id;
        let new = Arc::new(RwLock::new(Statement {
            block: block.id,
            next: next.map(|statement| statement.id),
            subtype: StatementEnum::ResultStatement(subtype.id),
            id,
        }));
        store.inter_statement(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-new_result_statement_"}}}
    /// Inter a new Statement in the store, and return it's `id`.
    pub fn new_result_statement_(
        block: &Block,
        next: Option<&Statement>,
        subtype: &ResultStatement,
    ) -> Statement {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id;
        let new = Statement {
            block: block.id,
            next: next.map(|statement| statement.id),
            subtype: StatementEnum::ResultStatement(subtype.id),
            id,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-nav-forward-to-block"}}}
    /// Navigate to [`Block`] across R18(1-*)
    pub fn r18_block<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Block> {
        vec![store.exhume_block(&self.block).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`Statement`] across R17(1-*c)
    pub fn r17_statement<'a>(&'a self, store: &'a LuDogStore) -> Vec<Arc<RwLock<Statement>>> {
        match self.next {
            Some(ref next) => vec![store.exhume_statement(next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-nav-backward-one-bi-cond-to-statement"}}}
    /// Navigate to [`Statement`] across R17(1c-1c)
    pub fn r17c_statement<'a>(&'a self, store: &'a LuDogStore) -> Vec<Arc<RwLock<Statement>>> {
        let statement = store.iter_statement().find(|statement| {
            let statement = statement.read().unwrap();
            statement.next == Some(self.id)
        });
        match statement {
            Some(statement) => vec![statement],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
