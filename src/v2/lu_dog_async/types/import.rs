// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"import-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"import-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::item::Item;
use crate::v2::lu_dog_async::types::item::ItemEnum;
use crate::v2::lu_dog_async::types::value_type::ValueType;
use crate::v2::lu_dog_async::types::value_type::ValueTypeEnum;
use crate::v2::sarzak::types::object::Object;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
use crate::v2::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"import-struct-documentation"}}}
/// An Import of a foreign ObjectStore
///
/// This indicates to the downstream model compiler that it needs to emit code to load the imported
///  ObjectStore.
///
/// I've got this has_alias boolean here because I don't have `Option<String>`. I never needed
///  it until now, because you get an option with a 1c relationship. Not proud of this, but it's
///  the best alternative. Makes me wonder about adding an `Option` type to the primitives though
/// .
///
/// I suppose if there were a way to signify a null string. Or I could check if it's length
///  is 0. I think adding the bool is cleaner.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"import-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Import {
    pub alias: String,
    pub has_alias: bool,
    pub id: usize,
    pub name: String,
    pub path: String,
    /// R40: [`Import`] '' [`Object`]
    pub object: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"import-implementation"}}}
impl Import {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"import-struct-impl-new"}}}
    /// Inter a new 'Import' in the store, and return it's `id`.
    pub async fn new(
        alias: String,
        has_alias: bool,
        name: String,
        path: String,
        object: Option<&Object>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Import>> {
        let object = match object {
            Some(object) => Some(object.id),
            None => None,
        };
        store
            .inter_import(|id| {
                Arc::new(RwLock::new(Import {
                    alias: alias.to_owned(),
                    has_alias,
                    id,
                    name: name.to_owned(),
                    path: path.to_owned(),
                    object,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"import-struct-impl-nav-forward-cond-to-object"}}}
    /// Navigate to [`Object`] across R40(1-*c)
    pub fn r40_object<'a>(
        &'a self,
        store: &'a SarzakStore,
    ) -> Vec<std::sync::Arc<std::sync::RwLock<Object>>> {
        span!("r40_object");
        match self.object {
            Some(ref object) => vec![store.exhume_object(object).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"import-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R6(isa)
    pub async fn r6_item<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Item>>> {
        span!("r6_item");
        store
            .iter_item()
            .await
            .filter_map(|item| async move {
                if let ItemEnum::Import(id) = item.read().await.subtype {
                    Some(item.clone())
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"import-impl-nav-subtype-to-supertype-value_type"}}}
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
                if let ValueTypeEnum::Import(id) = value_type.read().await.subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"import-implementation"}}}
impl PartialEq for Import {
    fn eq(&self, other: &Self) -> bool {
        self.alias == other.alias
            && self.has_alias == other.has_alias
            && self.name == other.name
            && self.path == other.path
            && self.object == other.object
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
