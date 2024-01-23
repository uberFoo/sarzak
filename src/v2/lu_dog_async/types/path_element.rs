// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"path_element-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"path_element-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use uuid::Uuid;

use crate::v2::lu_dog_async::types::x_path::XPath;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"path_element-struct-documentation"}}}
/// 🖕🤣
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"path_element-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PathElement {
    pub id: usize,
    pub name: String,
    /// R89: [`PathElement`] 'next' [`PathElement`]
    pub next: Option<usize>,
    /// R90: [`PathElement`] 'comprises' [`XPath`]
    pub x_path: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"path_element-implementation"}}}
impl PathElement {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"path_element-struct-impl-new"}}}
    /// Inter a new 'Path Element' in the store, and return it's `id`.
    pub async fn new(
        name: String,
        next: Option<&Arc<RwLock<PathElement>>>,
        x_path: &Arc<RwLock<XPath>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<PathElement>> {
        let path_element = match next {
            Some(path_element) => Some(path_element.read().await.id),
            None => None,
        };
        let x_path = x_path.read().await.id;
        store
            .inter_path_element(|id| {
                Arc::new(RwLock::new(PathElement {
                    id,
                    name: name.to_owned(),
                    next: path_element,
                    x_path,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"path_element-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`PathElement`] across R89(1-*c)
    pub async fn r89_path_element<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<PathElement>>> + '_ {
        match self.next {
            Some(ref next) => {
                stream::iter(vec![store.exhume_path_element(next).await.unwrap()].into_iter())
            }
            None => stream::iter(vec![].into_iter()),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"path_element-struct-impl-nav-forward-to-x_path"}}}
    /// Navigate to [`XPath`] across R90(1-*)
    pub async fn r90_x_path<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<XPath>>> + '_ {
        stream::iter(vec![store.exhume_x_path(&self.x_path).await.unwrap()].into_iter())
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"path_element-struct-impl-nav-backward-one-to-x_path"}}}
    /// Navigate to [`XPath`] across R97(1-1)
    pub async fn r97_x_path<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<XPath>>> + '_ {
        store.iter_x_path().await.filter_map(|x_path| async {
            if x_path.read().await.first == Some(self.id) {
                Some(x_path)
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"path_element-struct-impl-nav-backward-one-bi-cond-to-path_element"}}}
    /// Navigate to [`PathElement`] across R89(1c-1c)
    pub async fn r89c_path_element<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<PathElement>>> + '_ {
        store
            .iter_path_element()
            .await
            .filter_map(move |path_element| async move {
                if path_element.read().await.next == Some(self.id) {
                    Some(path_element.clone())
                } else {
                    None
                }
            })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"path_element-implementation"}}}
impl PartialEq for PathElement {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.next == other.next && self.x_path == other.x_path
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
