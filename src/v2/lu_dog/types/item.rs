// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"item-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-use-statements"}}}
use std::sync::{Arc, RwLock};

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
    pub fn new_function(
        function: Arc<RwLock<Function>>,
        store: &mut LuDogStore,
    ) -> Arc<RwLock<Self>> {
        let new = Arc::new(RwLock::new(Self::Function(function.read().unwrap().id)));
        store.inter_item(new.clone());
        new
    }

    /// Create a new instance of Item::Implementation
    pub fn new_implementation(
        implementation: Arc<RwLock<Implementation>>,
        store: &mut LuDogStore,
    ) -> Arc<RwLock<Self>> {
        let new = Arc::new(RwLock::new(Self::Implementation(
            implementation.read().unwrap().id,
        )));
        store.inter_item(new.clone());
        new
    }

    /// Create a new instance of Item::Import
    pub fn new_import(import: Arc<RwLock<Import>>, store: &mut LuDogStore) -> Arc<RwLock<Self>> {
        let new = Arc::new(RwLock::new(Self::Import(import.read().unwrap().id)));
        store.inter_item(new.clone());
        new
    }

    /// Create a new instance of Item::WoogStruct
    pub fn new_woog_struct(
        woog_struct: Arc<RwLock<WoogStruct>>,
        store: &mut LuDogStore,
    ) -> Arc<RwLock<Self>> {
        let new = Arc::new(RwLock::new(Self::WoogStruct(
            woog_struct.read().unwrap().id,
        )));
        store.inter_item(new.clone());
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
