// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"ty-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-use-statements"}}}
use crate::v2::sarzak::store::ObjectStore as SarzakStore;
use crate::v2::sarzak::types::attribute::Attribute;
use crate::v2::sarzak::types::boolean::BOOLEAN;
use crate::v2::sarzak::types::external::External;
use crate::v2::sarzak::types::float::FLOAT;
use crate::v2::sarzak::types::integer::INTEGER;
use crate::v2::sarzak::types::object::Object;
use crate::v2::sarzak::types::z_string::Z_STRING;
use crate::v2::sarzak::types::z_uuid::Z_UUID;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-enum-documentation"}}}
/// The type of a value
///
/// There are several values available: [Integer], [Boolean], [Float], [String], and [UUID]
/// .
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-enum-definition"}}}
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Ty {
    Boolean(Uuid),
    External(Uuid),
    Float(Uuid),
    Integer(Uuid),
    Object(Uuid),
    ZString(Uuid),
    ZUuid(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-implementation"}}}
impl Ty {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-new-impl"}}}
    /// Create a new instance of Ty::Boolean
    pub fn new_boolean(store: &SarzakStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_ty(&BOOLEAN).unwrap()
    }

    /// Create a new instance of Ty::External
    pub fn new_external(
        external: &Arc<RwLock<External>>,
        store: &mut SarzakStore,
    ) -> Arc<RwLock<Self>> {
        let id = external.read().unwrap().id;
        if let Some(external) = store.exhume_ty(&id) {
            external
        } else {
            let new = Arc::new(RwLock::new(Self::External(id)));
            store.inter_ty(new.clone());
            new
        }
    } // wtf?

    /// Create a new instance of Ty::Float
    pub fn new_float(store: &SarzakStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_ty(&FLOAT).unwrap()
    }

    /// Create a new instance of Ty::Integer
    pub fn new_integer(store: &SarzakStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_ty(&INTEGER).unwrap()
    }

    /// Create a new instance of Ty::Object
    pub fn new_object(object: &Arc<RwLock<Object>>, store: &mut SarzakStore) -> Arc<RwLock<Self>> {
        let id = object.read().unwrap().id;
        if let Some(object) = store.exhume_ty(&id) {
            object
        } else {
            let new = Arc::new(RwLock::new(Self::Object(id)));
            store.inter_ty(new.clone());
            new
        }
    } // wtf?

    /// Create a new instance of Ty::ZString
    pub fn new_z_string(store: &SarzakStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_ty(&Z_STRING).unwrap()
    }

    /// Create a new instance of Ty::ZUuid
    pub fn new_z_uuid(store: &SarzakStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_ty(&Z_UUID).unwrap()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Self::Boolean(id) => *id,
            Self::External(id) => *id,
            Self::Float(id) => *id,
            Self::Integer(id) => *id,
            Self::Object(id) => *id,
            Self::ZString(id) => *id,
            Self::ZUuid(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-struct-impl-nav-backward-one-to-attribute"}}}
    /// Navigate to [`Attribute`] across R2(1-1)
    pub fn r2_attribute<'a>(&'a self, store: &'a SarzakStore) -> Vec<Arc<RwLock<Attribute>>> {
        vec![store
            .iter_attribute()
            .find(|attribute| attribute.read().unwrap().ty == self.id())
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
