// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"item-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-use-statements"}}}
use crate::v2::woog::store::ObjectStore as WoogStore;
use crate::v2::woog::types::constant::Constant;
use crate::v2::woog::types::enumeration::Enumeration;
use crate::v2::woog::types::function::Function;
use crate::v2::woog::types::implementation::IMPLEMENTATION;
use crate::v2::woog::types::statement::Statement;
use crate::v2::woog::types::statement::StatementEnum;
use crate::v2::woog::types::structure::Structure;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-enum-documentation"}}}
/// An Item
///
/// This [entails a lot](https://doc.rust-lang.org/reference/items.html) of  syntax that I'm
///  just rolling up into one for now. We'll see for how long I can manage to get away with this
/// . 😎
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-enum-definition"}}}
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Item {
    Constant(Uuid),
    Enumeration(Uuid),
    Function(Uuid),
    Implementation(Uuid),
    Structure(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-implementation"}}}
impl Item {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-new-impl"}}}
    /// Create a new instance of Item::Constant
    pub fn new_constant(
        constant: &Arc<RwLock<Constant>>,
        store: &mut WoogStore,
    ) -> Arc<RwLock<Self>> {
        let id = constant.read().unwrap().id;
        if let Some(constant) = store.exhume_item(&id) {
            constant
        } else {
            let new = Arc::new(RwLock::new(Self::Constant(id)));
            store.inter_item(new.clone());
            new
        }
    } // wtf?

    /// Create a new instance of Item::Enumeration
    pub fn new_enumeration(
        enumeration: &Arc<RwLock<Enumeration>>,
        store: &mut WoogStore,
    ) -> Arc<RwLock<Self>> {
        let id = enumeration.read().unwrap().id;
        if let Some(enumeration) = store.exhume_item(&id) {
            enumeration
        } else {
            let new = Arc::new(RwLock::new(Self::Enumeration(id)));
            store.inter_item(new.clone());
            new
        }
    } // wtf?

    /// Create a new instance of Item::Function
    pub fn new_function(
        function: &Arc<RwLock<Function>>,
        store: &mut WoogStore,
    ) -> Arc<RwLock<Self>> {
        let id = function.read().unwrap().id;
        if let Some(function) = store.exhume_item(&id) {
            function
        } else {
            let new = Arc::new(RwLock::new(Self::Function(id)));
            store.inter_item(new.clone());
            new
        }
    } // wtf?

    /// Create a new instance of Item::Implementation
    pub fn new_implementation(store: &WoogStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_item(&IMPLEMENTATION).unwrap()
    }

    /// Create a new instance of Item::Structure
    pub fn new_structure(
        structure: &Arc<RwLock<Structure>>,
        store: &mut WoogStore,
    ) -> Arc<RwLock<Self>> {
        let id = structure.read().unwrap().id;
        if let Some(structure) = store.exhume_item(&id) {
            structure
        } else {
            let new = Arc::new(RwLock::new(Self::Structure(id)));
            store.inter_item(new.clone());
            new
        }
    } // wtf?

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Self::Constant(id) => *id,
            Self::Enumeration(id) => *id,
            Self::Function(id) => *id,
            Self::Implementation(id) => *id,
            Self::Structure(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-impl-nav-subtype-to-supertype-statement"}}}
    // Navigate to [`Statement`] across R11(isa)
    pub fn r11_statement<'a>(&'a self, store: &'a WoogStore) -> Vec<Arc<RwLock<Statement>>> {
        span!("r11_statement");
        vec![store
            .iter_statement()
            .find(|statement| {
                if let StatementEnum::Item(id) = statement.read().unwrap().subtype {
                    id == self.id()
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
