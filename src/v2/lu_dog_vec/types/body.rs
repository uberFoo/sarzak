// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"body-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"body-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::block::Block;
use crate::v2::lu_dog_vec::types::external_implementation::ExternalImplementation;
use crate::v2::lu_dog_vec::types::function::Function;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"body-hybrid-documentation"}}}
/// The function body. Generally contains statements, but may point to some other implementation
/// .
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"body-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Body {
    pub subtype: BodyEnum,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"body-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum BodyEnum {
    Block(usize),
    ExternalImplementation(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"body-implementation"}}}
impl Body {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"body-struct-impl-new_block"}}}
    /// Inter a new Body in the store, and return it's `id`.
    pub fn new_block(subtype: &Rc<RefCell<Block>>, store: &mut LuDogVecStore) -> Rc<RefCell<Body>> {
        store.inter_body(|id| {
            Rc::new(RefCell::new(Body {
                subtype: BodyEnum::Block(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"body-struct-impl-new_external_implementation"}}}
    /// Inter a new Body in the store, and return it's `id`.
    pub fn new_external_implementation(
        subtype: &Rc<RefCell<ExternalImplementation>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Body>> {
        store.inter_body(|id| {
            Rc::new(RefCell::new(Body {
                subtype: BodyEnum::ExternalImplementation(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"body-struct-impl-nav-backward-one-bi-cond-to-function"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"body-struct-impl-nav-backward-cond-to-function"}}}
    /// Navigate to [`Function`] across R19(1-1c)
    pub fn r19c_function<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Function>>> {
        span!("r19_function");
        let function = store
            .iter_function()
            .find(|function| function.borrow().body == self.id);
        match function {
            Some(ref function) => vec![function.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"body-implementation"}}}
impl PartialEq for Body {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
