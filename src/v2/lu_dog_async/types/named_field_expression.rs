// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"named_field_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"named_field_expression-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use uuid::Uuid;

use crate::v2::lu_dog_async::types::field_expression::FieldExpression;
use crate::v2::lu_dog_async::types::field_expression::FieldExpressionEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"named_field_expression-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NamedFieldExpression {
    pub id: usize,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"named_field_expression-implementation"}}}
impl NamedFieldExpression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"named_field_expression-struct-impl-new"}}}
    /// Inter a new 'Named Field Expression' in the store, and return it's `id`.
    pub async fn new(
        name: String,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<NamedFieldExpression>> {
        store
            .inter_named_field_expression(|id| {
                Arc::new(RwLock::new(NamedFieldExpression {
                    id,
                    name: name.to_owned(),
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"named_field_expression-impl-nav-subtype-to-supertype-field_expression"}}}
    // Navigate to [`FieldExpression`] across R94(isa)
    pub async fn r94_field_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<FieldExpression>>> {
        store
            .iter_field_expression()
            .await
            .filter_map(|field_expression| async move {
                if let FieldExpressionEnum::NamedFieldExpression(id) =
                    field_expression.read().await.subtype
                {
                    Some(field_expression.clone())
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"named_field_expression-implementation"}}}
impl PartialEq for NamedFieldExpression {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
