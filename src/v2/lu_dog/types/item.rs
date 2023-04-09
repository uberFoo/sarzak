// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"item-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-use-statements"}}}
use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
use crate::v2::lu_dog::types::model_type::ModelType;
use crate::v2::lu_dog::types::test_object::TEST_OBJECT;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Item {
    ModelType(Uuid),
    TestObject(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-implementation"}}}
impl Item {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-new-impl"}}}
    /// Create a new instance of Item::ModelType
    pub fn new_model_type(model_type: &ModelType, store: &mut LuDogStore) -> Self {
        let new = Self::ModelType(model_type.id);
        store.inter_item(new.clone());
        new
    }

    pub fn new_model_type_(model_type: &ModelType) -> Self {
        let new = Self::ModelType(model_type.id);
        new
    }

    /// Create a new instance of Item::TestObject
    pub fn new_test_object() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::TestObject(TEST_OBJECT)
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Item::ModelType(id) => *id,
            Item::TestObject(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
