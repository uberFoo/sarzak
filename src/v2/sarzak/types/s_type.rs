// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"s_type-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"s_type-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

// Subtype imports
use crate::v2::sarzak::types::boolean::BOOLEAN;
use crate::v2::sarzak::types::external::External;
use crate::v2::sarzak::types::float::FLOAT;
use crate::v2::sarzak::types::integer::INTEGER;
use crate::v2::sarzak::types::object::Object;
use crate::v2::sarzak::types::string::STRING;
use crate::v2::sarzak::types::uuid::UUID;

use crate::v2::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"s_type-enum-documentation"}}}
/// The type of a value
///
/// There are several values available: [Integer], [Boolean], [Float], [String], and [UUID]
///.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"s_type-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum SType {
    Boolean(Uuid),
    External(Uuid),
    Float(Uuid),
    Integer(Uuid),
    Object(Uuid),
    String(Uuid),
    Uuid(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"s_type-implementation"}}}
impl SType {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"s_type-new-impl"}}}
    /// Create a new instance of SType::Boolean
    pub fn new_boolean() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Boolean(BOOLEAN)
    }

    /// Create a new instance of SType::External
    pub fn new_external(external: &External, store: &mut SarzakStore) -> Self {
        let new = Self::External(external.id);
        store.inter_s_type(new.clone());
        new
    }

    /// Create a new instance of SType::Float
    pub fn new_float() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Float(FLOAT)
    }

    /// Create a new instance of SType::Integer
    pub fn new_integer() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Integer(INTEGER)
    }

    /// Create a new instance of SType::Object
    pub fn new_object(object: &Object, store: &mut SarzakStore) -> Self {
        let new = Self::Object(object.id);
        store.inter_s_type(new.clone());
        new
    }

    /// Create a new instance of SType::String
    pub fn new_string() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::String(STRING)
    }

    /// Create a new instance of SType::Uuid
    pub fn new_uuid() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Uuid(UUID)
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"s_type-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            SType::Boolean(id) => *id,
            SType::External(id) => *id,
            SType::Float(id) => *id,
            SType::Integer(id) => *id,
            SType::Object(id) => *id,
            SType::String(id) => *id,
            SType::Uuid(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
