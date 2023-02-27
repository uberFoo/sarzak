// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"statement-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-use-statements"}}}
use uuid::Uuid;

use crate::v2::woog::UUID_NS;

use serde::{Deserialize, Serialize};

// Subtype imports
use crate::v2::woog::types::expression_statement::EXPRESSION_STATEMENT;
use crate::v2::woog::types::item::ITEM;
use crate::v2::woog::types::woog_let::WoogLet;
use crate::v2::woog::types::woog_macro::WOOG_MACRO;

// Referrer imports
use crate::v2::woog::types::block::Block;

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
    WoogLet(Uuid),
    WoogMacro(Uuid),
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
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{}:{:?}:{}", value, block, EXPRESSION_STATEMENT).as_bytes(),
        );
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
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{}:{:?}:{}", value, block, ITEM).as_bytes(),
        );
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
    pub fn new_woog_let(
        value: String,
        block: &Block,
        subtype: &WoogLet,
        store: &mut WoogStore,
    ) -> Statement {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{}:{:?}:{:?}", value, block, subtype).as_bytes(),
        );
        let new = Statement {
            value: value,
            block: block.id,
            subtype: StatementEnum::WoogLet(subtype.id),
            id,
        };
        store.inter_statement(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"statement-struct-impl-new"}}}
    /// Inter a new Statement in the store, and return it's `id`.
    pub fn new_woog_macro(value: String, block: &Block, store: &mut WoogStore) -> Statement {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{}:{:?}:{}", value, block, WOOG_MACRO).as_bytes(),
        );
        let new = Statement {
            value: value,
            block: block.id,
            subtype: StatementEnum::WoogMacro(WOOG_MACRO),
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
