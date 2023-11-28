// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"path_element-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"path_element-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec_tracy::types::x_path::XPath;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec_tracy::store::ObjectStore as LuDogVecTracyStore;
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
    pub fn new(
        name: String,
        next: Option<&Rc<RefCell<PathElement>>>,
        x_path: &Rc<RefCell<XPath>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<PathElement>> {
        store.inter_path_element(|id| {
            Rc::new(RefCell::new(PathElement {
                id,
                name: name.to_owned(),
                next: next.map(|path_element| path_element.borrow().id),
                x_path: x_path.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"path_element-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`PathElement`] across R89(1-*c)
    pub fn r89_path_element<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<PathElement>>> {
        span!("r89_path_element");
        match self.next {
            Some(ref next) => vec![store.exhume_path_element(&next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"path_element-struct-impl-nav-forward-to-x_path"}}}
    /// Navigate to [`XPath`] across R90(1-*)
    pub fn r90_x_path<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<XPath>>> {
        span!("r90_x_path");
        vec![store.exhume_x_path(&self.x_path).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"path_element-struct-impl-nav-backward-one-to-x_path"}}}
    /// Navigate to [`XPath`] across R97(1-1)
    pub fn r97_x_path<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<XPath>>> {
        span!("r97_x_path");
        vec![store
            .iter_x_path()
            .find(|x_path| x_path.borrow().first == Some(self.id))
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"path_element-struct-impl-nav-backward-one-bi-cond-to-path_element"}}}
    /// Navigate to [`PathElement`] across R89(1c-1c)
    pub fn r89c_path_element<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<PathElement>>> {
        span!("r89_path_element");
        let path_element = store
            .iter_path_element()
            .find(|path_element| path_element.borrow().next == Some(self.id));
        match path_element {
            Some(ref path_element) => vec![path_element.clone()],
            None => Vec::new(),
        }
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
