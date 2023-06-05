// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"result_statement-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"result_statement-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog::types::expression::Expression;
use crate::v2::lu_dog::types::statement::Statement;
use crate::v2::lu_dog::types::statement::StatementEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"result_statement-struct-documentation"}}}
/// An Expression Statement that is not terminated by a semi-colon, and this yields a result
/// . This is only applicable if it's the last statement in a block.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"result_statement-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ResultStatement {
    pub id: Uuid,
    /// R41: [`ResultStatement`] '' [`Expression`]
    pub expression: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"result_statement-implementation"}}}
impl ResultStatement {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"result_statement-struct-impl-new"}}}
    /// Inter a new 'Result Statement' in the store, and return it's `id`.
    pub fn new(
        expression: &Rc<RefCell<Expression>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<ResultStatement>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(ResultStatement {
            id,
            expression: expression.borrow().id(),
        }));
        store.inter_result_statement(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"result_statement-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R41(1-*)
    pub fn r41_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Expression>>> {
        span!("r41_expression");
        vec![store.exhume_expression(&self.expression).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"result_statement-impl-nav-subtype-to-supertype-statement"}}}
    // Navigate to [`Statement`] across R16(isa)
    pub fn r16_statement<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Statement>>> {
        span!("r16_statement");
        vec![store
            .iter_statement()
            .find(|statement| {
                if let StatementEnum::ResultStatement(id) = statement.borrow().subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
