// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"item-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-use-statements"}}}
use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
use crate::v2::lu_dog::types::function::Function;
use crate::v2::lu_dog::types::implementation::Implementation;
use crate::v2::lu_dog::types::import::Import;
use crate::v2::lu_dog::types::woog_struct::WoogStruct;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Item {
    Function(Uuid),
    Implementation(Uuid),
    Import(Uuid),
    WoogStruct(Uuid),
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

    /// Create a new instance of Item::Import
    pub fn new_import(import: &Import, store: &mut LuDogStore) -> Self {
        let new = Self::Import(import.id);
        store.inter_item(new.clone());
        new
    }

    pub fn new_import_(import: &Import) -> Self {
        let new = Self::Import(import.id);
        new
    }

    /// Create a new instance of Item::WoogStruct
    pub fn new_woog_struct(woog_struct: &WoogStruct, store: &mut LuDogStore) -> Self {
        let new = Self::WoogStruct(woog_struct.id);
        store.inter_item(new.clone());
        new
    }

    pub fn new_woog_struct_(woog_struct: &WoogStruct) -> Self {
        let new = Self::WoogStruct(woog_struct.id);
        new
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Item::Function(id) => *id,
            Item::Implementation(id) => *id,
            Item::Import(id) => *id,
            Item::WoogStruct(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
