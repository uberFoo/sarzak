// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"item-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-use-statements"}}}
use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
use crate::v2::lu_dog::types::function::Function;
use crate::v2::lu_dog::types::implementation::Implementation;
use crate::v2::lu_dog::types::model_type::ModelType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Item {
    Function(Uuid),
    Implementation(Uuid),
    ModelType(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-implementation"}}}
impl Item {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-new-impl"}}}
    /// Create a new instance of Item::Function
    pub fn new_function(function: &Function, store: &mut LuDogStore) -> Self {
        let new = Self::Function(function.id);
        store.inter_item(new.clone());
        new
    }

    pub fn new_function_(function: &Function) -> Self {
        let new = Self::Function(function.id);
        new
    }

    /// Create a new instance of Item::Implementation
    pub fn new_implementation(implementation: &Implementation, store: &mut LuDogStore) -> Self {
        let new = Self::Implementation(implementation.id);
        store.inter_item(new.clone());
        new
    }

    pub fn new_implementation_(implementation: &Implementation) -> Self {
        let new = Self::Implementation(implementation.id);
        new
    }

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

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Item::Function(id) => *id,
            Item::Implementation(id) => *id,
            Item::ModelType(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
