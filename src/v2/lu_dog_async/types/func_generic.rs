// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"func_generic-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use uuid::Uuid;

use crate::v2::lu_dog_async::types::function::Function;
use crate::v2::lu_dog_async::types::value_type::ValueType;
use crate::v2::lu_dog_async::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-documentation"}}}
/// These are generics associated with a function.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FuncGeneric {
    pub id: usize,
    pub name: String,
    /// R107: [`FuncGeneric`] '' [`Function`]
    pub func: Option<usize>,
    /// R3: [`FuncGeneric`] '' [`FuncGeneric`]
    pub next: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-implementation"}}}
impl FuncGeneric {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-impl-new"}}}
    /// Inter a new 'Func Generic' in the store, and return it's `id`.
    pub async fn new(
        name: String,
        func: Option<&Arc<RwLock<Function>>>,
        next: Option<&Arc<RwLock<FuncGeneric>>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<FuncGeneric>> {
        let func_generic = match next {
            Some(func_generic) => Some(func_generic.read().await.id),
            None => None,
        };
        let function = match func {
            Some(function) => Some(function.read().await.id),
            None => None,
        };
        store
            .inter_func_generic(|id| {
                Arc::new(RwLock::new(FuncGeneric {
                    id,
                    name: name.to_owned(),
                    func: function,
                    next: func_generic,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-impl-nav-forward-to-func"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-impl-nav-forward-cond-to-func"}}}
    /// Navigate to [`Function`] across R107(1-*c)
    pub async fn r107_function<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Function>>> + '_ {
        match self.func {
            Some(ref func) => {
                stream::iter(vec![store.exhume_function(func).await.unwrap()].into_iter())
            }
            None => stream::iter(vec![].into_iter()),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`FuncGeneric`] across R3(1-*c)
    pub async fn r3_func_generic<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<FuncGeneric>>> + '_ {
        match self.next {
            Some(ref next) => {
                stream::iter(vec![store.exhume_func_generic(next).await.unwrap()].into_iter())
            }
            None => stream::iter(vec![].into_iter()),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-impl-nav-backward-one-bi-cond-to-func_generic"}}}
    /// Navigate to [`FuncGeneric`] across R3(1c-1c)
    pub async fn r3c_func_generic<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<FuncGeneric>>> + '_ {
        store
            .iter_func_generic()
            .await
            .filter_map(move |func_generic| async move {
                if func_generic.read().await.next == Some(self.id) {
                    Some(func_generic.clone())
                } else {
                    None
                }
            })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-struct-impl-nav-backward-one-to-function"}}}
    /// Navigate to [`Function`] across R99(1-1)
    pub async fn r99_function<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Function>>> + '_ {
        store.iter_function().await.filter_map(|function| async {
            if function.read().await.first_generic == Some(self.id) {
                Some(function)
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub async fn r1_value_type<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<ValueType>>> {
        store
            .iter_value_type()
            .await
            .filter_map(|value_type| async move {
                if let ValueTypeEnum::FuncGeneric(id) = value_type.read().await.subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"func_generic-implementation"}}}
impl PartialEq for FuncGeneric {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.func == other.func && self.next == other.next
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
