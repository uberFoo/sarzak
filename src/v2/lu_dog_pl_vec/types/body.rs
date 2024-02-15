// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"body-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"body-use-statements"}}}
use parking_lot::RwLock;
use std::sync::Arc;
use uuid::Uuid;

use crate::v2::lu_dog_pl_vec::types::block::Block;
use crate::v2::lu_dog_pl_vec::types::external_implementation::ExternalImplementation;
use crate::v2::lu_dog_pl_vec::types::function::Function;
use crate::v2::lu_dog_pl_vec::types::lambda::Lambda;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_pl_vec::store::ObjectStore as LuDogPlVecStore;
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
    pub a_sink: bool,
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
    pub fn new_block(
        a_sink: bool,
        subtype: &Arc<RwLock<Block>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Body>> {
        store.inter_body(|id| {
            Arc::new(RwLock::new(Body {
                a_sink: a_sink,
                subtype: BodyEnum::Block(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"body-struct-impl-new_external_implementation"}}}
    /// Inter a new Body in the store, and return it's `id`.
    pub fn new_external_implementation(
        a_sink: bool,
        subtype: &Arc<RwLock<ExternalImplementation>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<Body>> {
        store.inter_body(|id| {
            Arc::new(RwLock::new(Body {
                a_sink: a_sink,
                subtype: BodyEnum::ExternalImplementation(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"body-struct-impl-nav-backward-cond-to-function"}}}
    /// Navigate to [`Function`] across R19(1-1c)
    pub fn r19c_function<'a>(&'a self, store: &'a LuDogPlVecStore) -> Vec<Arc<RwLock<Function>>> {
        let function = store
            .iter_function()
            .find(|function| function.read().body == self.id);
        match function {
            Some(ref function) => vec![function.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"body-struct-impl-nav-backward-one-bi-cond-to-lambda"}}}
    /// Navigate to [`Lambda`] across R73(1c-1c)
    pub fn r73c_lambda<'a>(&'a self, store: &'a LuDogPlVecStore) -> Vec<Arc<RwLock<Lambda>>> {
        let lambda = store
            .iter_lambda()
            .find(|lambda| lambda.read().body == Some(self.id));
        match lambda {
            Some(ref lambda) => vec![lambda.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"body-implementation"}}}
impl PartialEq for Body {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype && self.a_sink == other.a_sink
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
