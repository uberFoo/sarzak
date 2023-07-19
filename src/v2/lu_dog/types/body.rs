// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"body-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"body-use-statements"}}}
use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
use crate::v2::lu_dog::types::block::Block;
use crate::v2::lu_dog::types::external_implementation::ExternalImplementation;
use crate::v2::lu_dog::types::function::Function;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"body-enum-documentation"}}}
/// The function body. Generally contains statements, but may point to some other implementation
/// .
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"body-enum-definition"}}}
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Body {
    Block(Uuid),
    ExternalImplementation(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"body-implementation"}}}
impl Body {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"body-new-impl"}}}
    /// Create a new instance of Body::Block
    pub fn new_block(block: &Rc<RefCell<Block>>, store: &mut LuDogStore) -> Rc<RefCell<Self>> {
        let id = block.borrow().id;
        if let Some(block) = store.exhume_body(id) {
            block
        } else {
            store.inter_body(|id| Rc::new(RefCell::new(Self::Block(id))))
        }
    }

    /// Create a new instance of Body::ExternalImplementation
    pub fn new_external_implementation(
        external_implementation: &Rc<RefCell<ExternalImplementation>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Self>> {
        let id = external_implementation.borrow().id;
        if let Some(external_implementation) = store.exhume_body(id) {
            external_implementation
        } else {
            store.inter_body(|id| Rc::new(RefCell::new(Self::ExternalImplementation(id))))
        }
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"body-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Self::Block(id) => *id,
            Self::ExternalImplementation(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"body-struct-impl-nav-backward-cond-to-function"}}}
    /// Navigate to [`Function`] across R19(1-1c)
    pub fn r19c_function<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Function>>> {
        span!("r19_function");
        let function = store
            .iter_function()
            .find(|function| function.borrow().body == self.id());
        match function {
            Some(ref function) => vec![function.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
