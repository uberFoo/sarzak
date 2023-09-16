// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"enumeration-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::enum_field::EnumField;
use crate::v2::lu_dog_async::types::implementation_block::ImplementationBlock;
use crate::v2::lu_dog_async::types::item::Item;
use crate::v2::lu_dog_async::types::item::ItemEnum;
use crate::v2::lu_dog_async::types::value_type::ValueType;
use crate::v2::lu_dog_async::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-struct-documentation"}}}
/// An Enumeration
///
/// An enumeration is an algebraic type that takes on one if it’s fielsd, another type. as
///  it’s value.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Enumeration {
    pub id: usize,
    pub name: String,
    /// R84: [`Enumeration`] 'may have an' [`ImplementationBlock`]
    pub implementation: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-implementation"}}}
impl Enumeration {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-struct-impl-new"}}}
    /// Inter a new 'Enumeration' in the store, and return it's `id`.
    pub async fn new(
        name: String,
        implementation: Option<&Arc<RwLock<ImplementationBlock>>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Enumeration>> {
        let implementation_block = match implementation {
            Some(implementation_block) => Some(implementation_block.read().await.id),
            None => None,
        };
        store
            .inter_enumeration(|id| {
                Arc::new(RwLock::new(Enumeration {
                    id,
                    name: name.to_owned(),
                    implementation: implementation_block,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-struct-impl-nav-forward-cond-to-implementation"}}}
    /// Navigate to [`ImplementationBlock`] across R84(1-*c)
    pub async fn r84_implementation_block<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<ImplementationBlock>>> + '_ {
        span!("r84_implementation_block");
        match self.implementation {
            Some(ref implementation) => stream::iter(
                vec![store
                    .exhume_implementation_block(implementation)
                    .await
                    .unwrap()]
                .into_iter(),
            ),
            None => stream::iter(vec![].into_iter()),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-struct-impl-nav-backward-1_M-to-enum_field"}}}
    /// Navigate to [`EnumField`] across R88(1-M)
    pub async fn r88_enum_field<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<EnumField>>> + '_ {
        span!("r88_enum_field");
        store
            .iter_enum_field()
            .await
            .filter_map(|enum_field| async {
                if enum_field.read().await.woog_enum == self.id {
                    Some(enum_field)
                } else {
                    None
                }
            })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R6(isa)
    pub async fn r6_item<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Item>>> {
        span!("r6_item");
        store
            .iter_item()
            .await
            .filter_map(|item| async move {
                if let ItemEnum::Enumeration(id) = item.read().await.subtype {
                    Some(item.clone())
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-impl-nav-subtype-to-supertype-value_type"}}}
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
                if let ValueTypeEnum::Enumeration(id) = value_type.read().await.subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enumeration-implementation"}}}
impl PartialEq for Enumeration {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.implementation == other.implementation
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
