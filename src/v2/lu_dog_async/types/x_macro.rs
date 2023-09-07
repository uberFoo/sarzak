// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"x_macro-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_macro-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::item::Item;
use crate::v2::lu_dog_async::types::item::ItemEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_macro-struct-documentation"}}}
/// This is a macro
///
/// It sort of interesting that the way that we create a new macro is with a macro. So it’s
///  nice and recursive like that. The macro in question, will be none other than `macro_rules
/// !`!.
///
/// OK, so that means something like this:
///
/// ```no-test
/// macro_rules! `ident` {
///     ($[ ( | `ident`]<,)*>[,$ident]*) => {
///     }
/// }
/// ```
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_macro-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XMacro {
    pub id: usize,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_macro-implementation"}}}
impl XMacro {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_macro-struct-impl-new"}}}
    /// Inter a new 'Macro' in the store, and return it's `id`.
    pub async fn new(name: String, store: &mut LuDogAsyncStore) -> Arc<RwLock<XMacro>> {
        store
            .inter_x_macro(|id| {
                Arc::new(RwLock::new(XMacro {
                    id,
                    name: name.to_owned(),
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_macro-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R6(isa)
    pub async fn r6_item<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Item>>> {
        span!("r6_item");
        store
            .iter_item()
            .await
            .filter_map(|item| async move {
                if let ItemEnum::XMacro(id) = item.read().await.subtype {
                    Some(item.clone())
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_macro-implementation"}}}
impl PartialEq for XMacro {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
