// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"list_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_expression-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::expression::Expression;
use crate::v2::lu_dog_async::types::expression::ExpressionEnum;
use crate::v2::lu_dog_async::types::list_element::ListElement;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_expression-struct-documentation"}}}
/// A list of expressions
///
/// E.g., `let a = [0, 1, 2, 3];`
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_expression-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListExpression {
    pub id: usize,
    /// R54: [`ListExpression`] 'contains' [`ListElement`]
    pub elements: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_expression-implementation"}}}
impl ListExpression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_expression-struct-impl-new"}}}
    /// Inter a new 'List Expression' in the store, and return it's `id`.
    pub async fn new(
        elements: Option<&Arc<RwLock<ListElement>>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ListExpression>> {
        let list_element = match elements {
            Some(list_element) => Some(list_element.read().await.id),
            None => None,
        };
        store
            .inter_list_expression(|id| {
                Arc::new(RwLock::new(ListExpression {
                    id,
                    elements: list_element,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_expression-struct-impl-nav-forward-cond-to-elements"}}}
    /// Navigate to [`ListElement`] across R54(1-*c)
    pub async fn r54_list_element<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<ListElement>>> + '_ {
        span!("r54_list_element");
        match self.elements {
            Some(ref elements) => {
                stream::iter(vec![store.exhume_list_element(elements).await.unwrap()].into_iter())
            }
            None => stream::iter(vec![].into_iter()),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_expression-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub async fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r15_expression");
        store
            .iter_expression()
            .await
            .filter_map(|expression| async move {
                if let ExpressionEnum::ListExpression(id) = expression.read().await.subtype {
                    Some(expression.clone())
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_expression-implementation"}}}
impl PartialEq for ListExpression {
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
