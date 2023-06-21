// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"statement-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog_vanilla::types::block::Block;
use crate::v2::lu_dog_vanilla::types::expression_statement::ExpressionStatement;
use crate::v2::lu_dog_vanilla::types::item_statement::ITEM_STATEMENT;
use crate::v2::lu_dog_vanilla::types::let_statement::LetStatement;
use crate::v2::lu_dog_vanilla::types::result_statement::ResultStatement;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
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
        store: &mut LuDogVanillaStore,
    ) -> Statement {
        let id = Uuid::new_v4();
        let new = Statement {
            block: block.id,
            next: next.map(|statement| statement.id),
            subtype: StatementEnum::ExpressionStatement(subtype.id),
            id,
        };
        store.inter_statement(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-new_item_statement"}}}
    /// Inter a new Statement in the store, and return it's `id`.
    pub fn new_item_statement(
        block: &Block,
        next: Option<&Statement>,
        store: &mut LuDogVanillaStore,
    ) -> Statement {
        let id = Uuid::new_v4();
        let new = Statement {
            block: block.id,
            next: next.map(|statement| statement.id),
            subtype: StatementEnum::ItemStatement(ITEM_STATEMENT),
            id,
        };
        store.inter_statement(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-new_let_statement"}}}
    /// Inter a new Statement in the store, and return it's `id`.
    pub fn new_let_statement(
        block: &Block,
        next: Option<&Statement>,
        subtype: &LetStatement,
        store: &mut LuDogVanillaStore,
    ) -> Statement {
        let id = Uuid::new_v4();
        let new = Statement {
            block: block.id,
            next: next.map(|statement| statement.id),
            subtype: StatementEnum::LetStatement(subtype.id),
            id,
        };
        store.inter_statement(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-new_result_statement"}}}
    /// Inter a new Statement in the store, and return it's `id`.
    pub fn new_result_statement(
        block: &Block,
        next: Option<&Statement>,
        subtype: &ResultStatement,
        store: &mut LuDogVanillaStore,
    ) -> Statement {
        let id = Uuid::new_v4();
        let new = Statement {
            block: block.id,
            next: next.map(|statement| statement.id),
            subtype: StatementEnum::ResultStatement(subtype.id),
            id,
        };
        store.inter_statement(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-nav-forward-to-block"}}}
    /// Navigate to [`Block`] across R18(1-*)
    pub fn r18_block<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Block> {
        vec![store.exhume_block(&self.block).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`Statement`] across R17(1-*c)
    pub fn r17_statement<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Statement> {
        match self.next {
            Some(ref next) => vec![store.exhume_statement(next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-nav-backward-one-bi-cond-to-block"}}}
    /// Navigate to [`Block`] across R71(1c-1c)
    pub fn r71c_block<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Block> {
        let block = store
            .iter_block()
            .find(|block| block.statement == Some(self.id));
        match block {
            Some(ref block) => vec![block],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-nav-backward-one-bi-cond-to-statement"}}}
    /// Navigate to [`Statement`] across R17(1c-1c)
    pub fn r17c_statement<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Statement> {
        let statement = store
            .iter_statement()
            .find(|statement| statement.next == Some(self.id));
        match statement {
            Some(ref statement) => vec![statement],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
