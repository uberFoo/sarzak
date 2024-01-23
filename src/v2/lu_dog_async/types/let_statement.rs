// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"let_statement-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"let_statement-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use uuid::Uuid;

use crate::v2::lu_dog_async::types::expression::Expression;
use crate::v2::lu_dog_async::types::local_variable::LocalVariable;
use crate::v2::lu_dog_async::types::statement::Statement;
use crate::v2::lu_dog_async::types::statement::StatementEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"let_statement-struct-documentation"}}}
/// A Let Statement
///
/// This statement assigns a value from an expression to a local variable.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"let_statement-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LetStatement {
    pub id: usize,
    /// R20: [`LetStatement`] 'assigns the value of an' [`Expression`]
    pub expression: usize,
    /// R21: [`LetStatement`] 'assigns a value to a' [`LocalVariable`]
    pub variable: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"let_statement-implementation"}}}
impl LetStatement {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"let_statement-struct-impl-new"}}}
    /// Inter a new 'Let Statement' in the store, and return it's `id`.
    pub async fn new(
        expression: &Arc<RwLock<Expression>>,
        variable: &Arc<RwLock<LocalVariable>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<LetStatement>> {
        let expression = expression.read().await.id;
        let variable = variable.read().await.id;
        store
            .inter_let_statement(|id| {
                Arc::new(RwLock::new(LetStatement {
                    id,
                    expression,
                    variable,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"let_statement-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R20(1-*)
    pub async fn r20_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Expression>>> + '_ {
        stream::iter(vec![store.exhume_expression(&self.expression).await.unwrap()].into_iter())
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"let_statement-struct-impl-nav-forward-to-variable"}}}
    /// Navigate to [`LocalVariable`] across R21(1-*)
    pub async fn r21_local_variable<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<LocalVariable>>> + '_ {
        stream::iter(vec![store.exhume_local_variable(&self.variable).await.unwrap()].into_iter())
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"let_statement-impl-nav-subtype-to-supertype-statement"}}}
    // Navigate to [`Statement`] across R16(isa)
    pub async fn r16_statement<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Statement>>> {
        store
            .iter_statement()
            .await
            .filter_map(|statement| async move {
                if let StatementEnum::LetStatement(id) = statement.read().await.subtype {
                    Some(statement.clone())
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"let_statement-implementation"}}}
impl PartialEq for LetStatement {
    fn eq(&self, other: &Self) -> bool {
        self.expression == other.expression && self.variable == other.variable
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
