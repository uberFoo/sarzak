// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"item-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-use-statements"}}}
use crate::v2::woog::types::constant::Constant;
use crate::v2::woog::types::enumeration::Enumeration;
use crate::v2::woog::types::function::Function;
use crate::v2::woog::types::implementation::IMPLEMENTATION;
use crate::v2::woog::types::statement::Statement;
use crate::v2::woog::types::structure::Structure;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-const-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-enum-documentation"}}}
/// An Item
///
/// This [entails a lot](https://doc.rust-lang.org/reference/items.html) of  syntax that I'm
/// just rolling up into one for now. We'll see for how long I can manage to get away with this
///. 😎
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-const-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
    pub fn new_constant(constant: &Constant, store: &mut WoogStore) -> Self {
        let new = Self::Constant(constant.id);
        store.inter_item(new.clone());
        new
    }

    /// Create a new instance of Item::Enumeration
    pub fn new_enumeration(enumeration: &Enumeration, store: &mut WoogStore) -> Self {
        let new = Self::Enumeration(enumeration.id);
        store.inter_item(new.clone());
        new
    }

    /// Create a new instance of Item::Function
    pub fn new_function(function: &Function, store: &mut WoogStore) -> Self {
        let new = Self::Function(function.id);
        store.inter_item(new.clone());
        new
    }

    /// Create a new instance of Item::Implementation
    pub fn new_implementation() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Implementation(IMPLEMENTATION)
    }

    /// Create a new instance of Item::Structure
    pub fn new_structure(structure: &Structure, store: &mut WoogStore) -> Self {
        let new = Self::Structure(structure.id);
        store.inter_item(new.clone());
        new
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Item::Constant(id) => *id,
            Item::Enumeration(id) => *id,
            Item::Function(id) => *id,
            Item::Implementation(id) => *id,
            Item::Structure(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-impl-nav-subtype-to-supertype-statement"}}}
    // Navigate to [`Statement`] across R11(isa)
    pub fn r11_statement<'a>(&'a self, store: &'a WoogStore) -> Vec<&Statement> {
        vec![store.exhume_statement(&self.id()).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
