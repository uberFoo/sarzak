// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"hybrid_enum-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"hybrid_enum-use-statements"}}}
use uuid::Uuid;

use crate::v2::sarzak::types::object::Object;
use crate::v2::woog::types::enumeration::Enumeration;
use crate::v2::woog::types::reference_type::ReferenceType;
use serde::{Deserialize, Serialize};

use crate::v2::sarzak::store::ObjectStore as SarzakStore;
use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"hybrid_enum-struct-documentation"}}}
/// Hybrid Enumeration
///
/// This is an auxiliary data structure that is used when an object is a supertype with attributes
///.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"hybrid_enum-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct HybridEnum {
    pub id: Uuid,
    pub name: String,
    /// R43: [`HybridEnum`] 'supports' [`Object`]
    pub object: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"hybrid_enum-implementation"}}}
impl HybridEnum {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"hybrid_enum-struct-impl-new"}}}
    /// Inter a new 'Hybrid Enum' in the store, and return it's `id`.
    pub fn new(name: String, object: &Object, store: &mut WoogStore) -> HybridEnum {
        let id = Uuid::new_v4();
        let new = HybridEnum {
            id: id,
            name: name,
            object: object.id,
        };
        store.inter_hybrid_enum(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"hybrid_enum-struct-impl-new_"}}}
    /// Inter a new 'Hybrid Enum' in the store, and return it's `id`.
    pub fn new_(name: String, object: &Object) -> HybridEnum {
        let id = Uuid::new_v4();
        let new = HybridEnum {
            id: id,
            name: name,
            object: object.id,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"hybrid_enum-struct-impl-nav-forward-to-object"}}}
    /// Navigate to [`Object`] across R43(1-*)
    pub fn r43_object<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Object> {
        vec![store.exhume_object(&self.object).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"hybrid_enum-impl-nav-subtype-to-supertype-enumeration"}}}
    // Navigate to [`Enumeration`] across R46(isa)
    pub fn r46_enumeration<'a>(&'a self, store: &'a WoogStore) -> Vec<&Enumeration> {
        vec![store.exhume_enumeration(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"hybrid_enum-impl-nav-subtype-to-supertype-reference_type"}}}
    // Navigate to [`ReferenceType`] across R44(isa)
    pub fn r44_reference_type<'a>(&'a self, store: &'a WoogStore) -> Vec<&ReferenceType> {
        vec![store.exhume_reference_type(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
