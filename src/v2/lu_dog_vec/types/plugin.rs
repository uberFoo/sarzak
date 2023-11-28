// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"plugin-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"plugin-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::value_type::ValueType;
use crate::v2::lu_dog_vec::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"plugin-const-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"plugin-struct-documentation"}}}
/// An external compilation unit that may be loaded at run time.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"plugin-const-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"plugin-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Plugin {
    pub id: usize,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"plugin-implementation"}}}
impl Plugin {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"plugin-struct-impl-new"}}}
    /// Inter a new 'Plugin' in the store, and return it's `id`.
    pub fn new(name: String, store: &mut LuDogVecStore) -> Rc<RefCell<Plugin>> {
        store.inter_plugin(|id| {
            Rc::new(RefCell::new(Plugin {
                id,
                name: name.to_owned(),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"plugin-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<ValueType>>> {
        vec![store
            .iter_value_type()
            .find(|value_type| {
                if let ValueTypeEnum::Plugin(id) = value_type.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"plugin-implementation"}}}
impl PartialEq for Plugin {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
