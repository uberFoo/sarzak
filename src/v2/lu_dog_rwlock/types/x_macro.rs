// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"x_macro-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_macro-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::item::Item;
use crate::v2::lu_dog_rwlock::types::item::ItemEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
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
/// ```
/// macro_rules! `ident` {
///     ($[ ( | `ident`]<,)*>[,$ident]*) => {
///     }
/// }
/// ```
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_macro-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct XMacro {
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_macro-implementation"}}}
impl XMacro {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_macro-struct-impl-new"}}}
    /// Inter a new 'Macro' in the store, and return it's `id`.
    pub fn new(name: String, store: &mut LuDogRwlockStore) -> Arc<RwLock<XMacro>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(XMacro { id, name }));
        store.inter_x_macro(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_macro-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R6(isa)
    pub fn r6_item<'a>(&'a self, store: &'a LuDogRwlockStore) -> Vec<Arc<RwLock<Item>>> {
        span!("r6_item");
        vec![store
            .iter_item()
            .find(|item| {
                if let ItemEnum::XMacro(id) = item.read().unwrap().subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
