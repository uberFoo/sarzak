// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"field_access-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::expression::Expression;
use crate::v2::lu_dog_async::types::expression::ExpressionEnum;
use crate::v2::lu_dog_async::types::field_access_target::FieldAccessTarget;
use crate::v2::lu_dog_async::types::woog_struct::WoogStruct;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-struct-documentation"}}}
/// A Struct Field Access
///
/// Think dotted notation.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FieldAccess {
    pub id: usize,
    /// R27: [`FieldAccess`] 'contains an' [`Expression`]
    pub expression: usize,
    /// R65: [`FieldAccess`] '' [`FieldAccessTarget`]
    pub field: usize,
    /// R66: [`FieldAccess`] '' [`WoogStruct`]
    pub woog_struct: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-implementation"}}}
impl FieldAccess {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-struct-impl-new"}}}
    /// Inter a new 'Field Access' in the store, and return it's `id`.
    pub async fn new(
        expression: &Arc<RwLock<Expression>>,
        field: &Arc<RwLock<FieldAccessTarget>>,
        woog_struct: &Arc<RwLock<WoogStruct>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<FieldAccess>> {
        let expression = expression.read().await.id;
        let woog_struct = woog_struct.read().await.id;
        let field = field.read().await.id;
        store
            .inter_field_access(|id| {
                Arc::new(RwLock::new(FieldAccess {
                    id,
                    expression,
                    field,
                    woog_struct,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R27(1-*)
    pub async fn r27_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r27_expression");
        vec![store.exhume_expression(&self.expression).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-struct-impl-nav-forward-to-field"}}}
    /// Navigate to [`FieldAccessTarget`] across R65(1-*)
    pub async fn r65_field_access_target<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<FieldAccessTarget>>> {
        span!("r65_field_access_target");
        vec![store.exhume_field_access_target(&self.field).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-struct-impl-nav-forward-to-woog_struct"}}}
    /// Navigate to [`WoogStruct`] across R66(1-*)
    pub async fn r66_woog_struct<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<WoogStruct>>> {
        span!("r66_woog_struct");
        vec![store.exhume_woog_struct(&self.woog_struct).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-impl-nav-subtype-to-supertype-expression"}}}
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
                if let ExpressionEnum::FieldAccess(id) = expression.read().await.subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-implementation"}}}
impl PartialEq for FieldAccess {
    fn eq(&self, other: &Self) -> bool {
        self.expression == other.expression
            && self.field == other.field
            && self.woog_struct == other.woog_struct
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
