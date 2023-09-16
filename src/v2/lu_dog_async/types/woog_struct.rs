// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"woog_struct-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::field::Field;
use crate::v2::lu_dog_async::types::field_access::FieldAccess;
use crate::v2::lu_dog_async::types::implementation_block::ImplementationBlock;
use crate::v2::lu_dog_async::types::item::Item;
use crate::v2::lu_dog_async::types::item::ItemEnum;
use crate::v2::lu_dog_async::types::struct_expression::StructExpression;
use crate::v2::lu_dog_async::types::value_type::ValueType;
use crate::v2::lu_dog_async::types::value_type::ValueTypeEnum;
use crate::v2::sarzak::types::object::Object;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
use crate::v2::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-documentation"}}}
/// A Type from the Model
///
/// This is really just an alias for `[Object]`.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WoogStruct {
    pub id: usize,
    pub name: String,
    /// R4: [`WoogStruct`] 'mirrors an' [`Object`]
    pub object: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-implementation"}}}
impl WoogStruct {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-impl-new"}}}
    /// Inter a new 'Struct' in the store, and return it's `id`.
    pub async fn new(
        name: String,
        object: Option<&Object>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<WoogStruct>> {
        let object = match object {
            Some(object) => Some(object.id),
            None => None,
        };
        store
            .inter_woog_struct(|id| {
                Arc::new(RwLock::new(WoogStruct {
                    id,
                    name: name.to_owned(),
                    object,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-impl-nav-forward-cond-to-object"}}}
    /// Navigate to [`Object`] across R4(1-*c)
    pub fn r4_object<'a>(
        &'a self,
        store: &'a SarzakStore,
    ) -> Vec<std::sync::Arc<std::sync::RwLock<Object>>> {
        span!("r4_object");
        match self.object {
            Some(ref object) => vec![store.exhume_object(object).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-impl-nav-backward-1_M-to-field"}}}
    /// Navigate to [`Field`] across R7(1-M)
    pub async fn r7_field<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Field>>> + '_ {
        span!("r7_field");
        store.iter_field().await.filter_map(|field| async {
            if field.read().await.x_model == self.id {
                Some(field)
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-impl-nav-backward-1_M-to-field_access"}}}
    /// Navigate to [`FieldAccess`] across R66(1-M)
    pub async fn r66_field_access<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<FieldAccess>>> + '_ {
        span!("r66_field_access");
        store
            .iter_field_access()
            .await
            .filter_map(|field_access| async {
                if field_access.read().await.woog_struct == self.id {
                    Some(field_access)
                } else {
                    None
                }
            })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-impl-nav-backward-one-bi-cond-to-implementation_block"}}}
    /// Navigate to [`ImplementationBlock`] across R8(1c-1c)
    pub async fn r8c_implementation_block<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<ImplementationBlock>>> + '_ {
        span!("r8_implementation_block");
        store
            .iter_implementation_block()
            .await
            .filter_map(move |implementation_block| async move {
                if implementation_block.read().await.model_type == Some(self.id) {
                    Some(implementation_block.clone())
                } else {
                    None
                }
            })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-impl-nav-backward-1_M-to-struct_expression"}}}
    /// Navigate to [`StructExpression`] across R39(1-M)
    pub async fn r39_struct_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<StructExpression>>> + '_ {
        span!("r39_struct_expression");
        store
            .iter_struct_expression()
            .await
            .filter_map(|struct_expression| async {
                if struct_expression.read().await.woog_struct == self.id {
                    Some(struct_expression)
                } else {
                    None
                }
            })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R6(isa)
    pub async fn r6_item<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Item>>> {
        span!("r6_item");
        store
            .iter_item()
            .await
            .filter_map(|item| async move {
                if let ItemEnum::WoogStruct(id) = item.read().await.subtype {
                    Some(item.clone())
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub async fn r1_value_type<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<ValueType>>> {
        span!("r1_value_type");
        store
            .iter_value_type()
            .await
            .filter_map(|value_type| async move {
                if let ValueTypeEnum::WoogStruct(id) = value_type.read().await.subtype {
                    Some(value_type.clone())
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-implementation"}}}
impl PartialEq for WoogStruct {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.object == other.object
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
