// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"statement-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-use-statements"}}}
use uuid::Uuid;

use crate::v2::woog::types::block::Block;
use crate::v2::woog::types::expression_statement::EXPRESSION_STATEMENT;
use crate::v2::woog::types::item::ITEM;
use crate::v2::woog::types::x_let::XLet;
use crate::v2::woog::types::x_macro::X_MACRO;
use serde::{Deserialize, Serialize};

use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-hybrid-documentation"}}}
/// A Statement
///
/// There aren't all that many types of [statement in rust.](https://doc.rust-lang.org/reference
////statements.html).
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-hybrid-enum-definition"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Statement {
    pub subtype: StatementEnum,
    pub id: Uuid,
    pub value: String,
    /// R12: [`Statement`] 'belongs to a' [`Block`]
    pub block: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum StatementEnum {
    ExpressionStatement(Uuid),
    Item(Uuid),
    XLet(Uuid),
    XMacro(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-implementation"}}}
impl Statement {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-new"}}}
    /// Inter a new Statement in the store, and return it's `id`.
    pub fn new_expression_statement(
        value: String,
        block: &Block,
        store: &mut WoogStore,
    ) -> Statement {
        let id = EXPRESSION_STATEMENT;
        let new = Statement {
            value: value,
            block: block.id,
            subtype: StatementEnum::ExpressionStatement(EXPRESSION_STATEMENT),
            id,
        };
        store.inter_statement(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-new"}}}
    /// Inter a new Statement in the store, and return it's `id`.
    pub fn new_item(value: String, block: &Block, store: &mut WoogStore) -> Statement {
        let id = ITEM;
        let new = Statement {
            value: value,
            block: block.id,
            subtype: StatementEnum::Item(ITEM),
            id,
        };
        store.inter_statement(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-new"}}}
    /// Inter a new Statement in the store, and return it's `id`.
    pub fn new_x_let(
        value: String,
        block: &Block,
        subtype: &XLet,
        store: &mut WoogStore,
    ) -> Statement {
        let id = subtype.id;
        let new = Statement {
            value: value,
            block: block.id,
            subtype: StatementEnum::XLet(subtype.id),
            id,
        };
        store.inter_statement(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-new"}}}
    /// Inter a new Statement in the store, and return it's `id`.
    pub fn new_x_macro(value: String, block: &Block, store: &mut WoogStore) -> Statement {
        let id = X_MACRO;
        let new = Statement {
            value: value,
            block: block.id,
            subtype: StatementEnum::XMacro(X_MACRO),
            id,
        };
        store.inter_statement(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-nav-forward-to-block"}}}
    /// Navigate to [`Block`] across R12(1-*)
    pub fn r12_block<'a>(&'a self, store: &'a WoogStore) -> Vec<&Block> {
        vec![store.exhume_block(&self.block).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
