// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"list_element-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::expression::Expression;
use crate::v2::lu_dog_async::types::list_expression::ListExpression;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ListElement {
    pub id: Uuid,
    /// R55: [`ListElement`] 'points at an' [`Expression`]
    pub expression: Uuid,
    /// R53: [`ListElement`] 'follows' [`ListElement`]
    pub next: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-implementation"}}}
impl ListElement {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-struct-impl-new"}}}
    /// Inter a new 'List Element' in the store, and return it's `id`.
    pub async fn new(
        expression: &Arc<RwLock<Expression>>,
        next: Option<&Arc<RwLock<ListElement>>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ListElement>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(ListElement {
            id,
            expression: expression.read().await.id(),
            next: futures::future::OptionFuture::from(
                next.map(|list_element| async { list_element.read().await.id }),
            )
            .await,
        }));
        store.inter_list_element(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R55(1-*)
    pub async fn r55_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r55_expression");
        vec![store.exhume_expression(&self.expression).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`ListElement`] across R53(1-*c)
    pub async fn r53_list_element<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<ListElement>>> {
        span!("r53_list_element");
        match self.next {
            Some(ref next) => vec![store.exhume_list_element(next).await.unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-struct-impl-nav-backward-one-bi-cond-to-list_element"}}}
    /// Navigate to [`ListElement`] across R53(1c-1c)
    pub async fn r53c_list_element<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<ListElement>>> {
        span!("r53_list_element");
        let mut result = Vec::new();
        for list_element in store.iter_list_element().await {
            if list_element.read().await.next == Some(self.id) {
                result.push(list_element.clone());
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-struct-impl-nav-backward-one-to-list_expression"}}}
    /// Navigate to [`ListExpression`] across R54(1-1)
    pub async fn r54_list_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<ListExpression>>> {
        span!("r54_list_expression");
        let mut result = Vec::new();
        for list_expression in store.iter_list_expression().await {
            if list_expression.read().await.elements == Some(self.id) {
                result.push(list_expression);
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub async fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r15_expression");
        vec![store.exhume_expression(&self.id).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
