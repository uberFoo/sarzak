// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"struct_field-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_field-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::enum_field::EnumField;
use crate::v2::lu_dog_async::types::enum_field::EnumFieldEnum;
use crate::v2::lu_dog_async::types::expression::Expression;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_field-struct-documentation"}}}
/// A field that is a structure.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_field-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StructField {
    pub id: usize,
    pub name: String,
    /// R89: [`StructField`] 'is composed with a' [`Expression`]
    pub expression: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_field-implementation"}}}
impl StructField {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_field-struct-impl-new"}}}
    /// Inter a new 'Struct Field' in the store, and return it's `id`.
    pub async fn new(
        name: String,
        expression: Option<&Arc<RwLock<Expression>>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<StructField>> {
        let expression = match expression {
            Some(expression) => Some(expression.read().await.id),
            None => None,
        };
        store
            .inter_struct_field(|id| {
                Arc::new(RwLock::new(StructField {
                    id,
                    name: name.to_owned(),
                    expression,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_field-struct-impl-nav-forward-cond-to-expression"}}}
    /// Navigate to [`Expression`] across R89(1-*c)
    pub async fn r89_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r89_expression");
        match self.expression {
            Some(ref expression) => vec![store.exhume_expression(expression).await.unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_field-impl-nav-subtype-to-supertype-enum_field"}}}
    // Navigate to [`EnumField`] across R85(isa)
    pub async fn r85_enum_field<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<EnumField>>> {
        span!("r85_enum_field");
        store
            .iter_enum_field()
            .await
            .filter_map(|enum_field| async move {
                if let EnumFieldEnum::StructField(id) = enum_field.read().await.subtype {
                    Some(enum_field.clone())
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_field-implementation"}}}
impl PartialEq for StructField {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.expression == other.expression
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
