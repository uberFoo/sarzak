// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"woog_option-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-use-statements"}}}
use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
use crate::v2::lu_dog::types::none::NONE;
use crate::v2::lu_dog::types::some::Some;
use crate::v2::lu_dog::types::value_type::ValueType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-enum-documentation"}}}
/// An Optional Type
///
/// This type is either `None` or `Some(`[Type]`)`.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum WoogOption {
    None(Uuid),
    Some(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-implementation"}}}
impl WoogOption {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-new-impl"}}}
    /// Create a new instance of WoogOption::None
    pub fn new_none() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::None(NONE)
    }

    /// Create a new instance of WoogOption::Some
    pub fn new_some(some: &Some, store: &mut LuDogStore) -> Self {
        let new = Self::Some(some.id);
        store.inter_woog_option(new.clone());
        new
    }

    pub fn new_some_(some: &Some) -> Self {
        let new = Self::Some(some.id);
        new
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            WoogOption::None(id) => *id,
            WoogOption::Some(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_option-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogStore) -> Vec<&ValueType> {
        vec![store.exhume_value_type(&self.id()).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
