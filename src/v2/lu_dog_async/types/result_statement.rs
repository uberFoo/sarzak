// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"result_statement-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"result_statement-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::expression::Expression;
use crate::v2::lu_dog_async::types::statement::Statement;
use crate::v2::lu_dog_async::types::statement::StatementEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
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
    pub async fn new(
        expression: &Arc<RwLock<Expression>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ResultStatement>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(ResultStatement {
            id,
            expression: expression.read().await.id(),
        }));
        store.inter_result_statement(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"result_statement-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R41(1-*)
    pub async fn r41_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r41_expression");
        vec![store.exhume_expression(&self.expression).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"result_statement-impl-nav-subtype-to-supertype-statement"}}}
    // Navigate to [`Statement`] across R16(isa)
    pub async fn r16_statement<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Statement>>> {
        span!("r16_statement");
        let mut result = Vec::new();
        for statement in store.iter_statement().await {
            if let StatementEnum::ResultStatement(id) = statement.read().await.subtype {
                result.push(statement.clone());
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
