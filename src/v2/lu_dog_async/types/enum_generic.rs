// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"enum_generic-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use uuid::Uuid;

use crate::v2::lu_dog_async::types::enumeration::Enumeration;
use crate::v2::lu_dog_async::types::value_type::ValueType;
use crate::v2::lu_dog_async::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EnumGeneric {
    pub id: usize,
    pub name: String,
    /// R104: [`EnumGeneric`] 'parameterizes' [`Enumeration`]
    pub woog_enum: usize,
    /// R106: [`EnumGeneric`] 'next' [`EnumGeneric`]
    pub next: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-implementation"}}}
impl EnumGeneric {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-new"}}}
    /// Inter a new 'Enum Generic' in the store, and return it's `id`.
    pub async fn new(
        name: String,
        woog_enum: &Arc<RwLock<Enumeration>>,
        next: Option<&Arc<RwLock<EnumGeneric>>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<EnumGeneric>> {
        let enum_generic = match next {
            Some(enum_generic) => Some(enum_generic.read().await.id),
            None => None,
        };
        let woog_enum = woog_enum.read().await.id;
        store
            .inter_enum_generic(|id| {
                Arc::new(RwLock::new(EnumGeneric {
                    id,
                    name: name.to_owned(),
                    woog_enum,
                    next: enum_generic,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-nav-forward-to-woog_enum"}}}
    /// Navigate to [`Enumeration`] across R104(1-*)
    pub async fn r104_enumeration<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Enumeration>>> + '_ {
        stream::iter(vec![store.exhume_enumeration(&self.woog_enum).await.unwrap()].into_iter())
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`EnumGeneric`] across R106(1-*c)
    pub async fn r106_enum_generic<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<EnumGeneric>>> + '_ {
        match self.next {
            Some(ref next) => {
                stream::iter(vec![store.exhume_enum_generic(next).await.unwrap()].into_iter())
            }
            None => stream::iter(vec![].into_iter()),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-nav-backward-one-bi-cond-to-enum_generic"}}}
    /// Navigate to [`EnumGeneric`] across R106(1c-1c)
    pub async fn r106c_enum_generic<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<EnumGeneric>>> + '_ {
        store
            .iter_enum_generic()
            .await
            .filter_map(move |enum_generic| async move {
                if enum_generic.read().await.next == Some(self.id) {
                    Some(enum_generic.clone())
                } else {
                    None
                }
            })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-nav-backward-one-to-enumeration"}}}
    /// Navigate to [`Enumeration`] across R105(1-1)
    pub async fn r105_enumeration<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Enumeration>>> + '_ {
        store
            .iter_enumeration()
            .await
            .filter_map(|enumeration| async {
                if enumeration.read().await.first_generic == Some(self.id) {
                    Some(enumeration)
                } else {
                    None
                }
            })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub async fn r1_value_type<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<ValueType>>> {
        store
            .iter_value_type()
            .await
            .filter_map(|value_type| async move {
                if let ValueTypeEnum::EnumGeneric(id) = value_type.read().await.subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-implementation"}}}
impl PartialEq for EnumGeneric {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.woog_enum == other.woog_enum && self.next == other.next
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
