// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"reference_type-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference_type-use-statements"}}}
use crate::v2::sarzak::types::external::External;
use crate::v2::sarzak::types::object::Object;
use crate::v2::woog::store::ObjectStore as WoogStore;
use crate::v2::woog::types::enumeration_field::EnumerationField;
use crate::v2::woog::types::hybrid_enum::HybridEnum;
use crate::v2::woog::types::reference::Reference;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference_type-enum-documentation"}}}
/// The Type of the Reference
///
/// Is this really the type of the referent?
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference_type-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ReferenceType {
    EnumerationField(Uuid),
    External(Uuid),
    HybridEnum(Uuid),
    Object(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference_type-implementation"}}}
impl ReferenceType {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference_type-new-impl"}}}
    /// Create a new instance of ReferenceType::EnumerationField
    pub fn new_enumeration_field(
        enumeration_field: &EnumerationField,
        store: &mut WoogStore,
    ) -> Self {
        let new = Self::EnumerationField(enumeration_field.id);
        store.inter_reference_type(new.clone());
        new
    }

    pub fn new_enumeration_field_(enumeration_field: &EnumerationField) -> Self {
        let new = Self::EnumerationField(enumeration_field.id);
        new
    }

    /// Create a new instance of ReferenceType::External
    pub fn new_external(external: &External, store: &mut WoogStore) -> Self {
        let new = Self::External(external.id);
        store.inter_reference_type(new.clone());
        new
    }

    pub fn new_external_(external: &External) -> Self {
        let new = Self::External(external.id);
        new
    }

    /// Create a new instance of ReferenceType::HybridEnum
    pub fn new_hybrid_enum(hybrid_enum: &HybridEnum, store: &mut WoogStore) -> Self {
        let new = Self::HybridEnum(hybrid_enum.id);
        store.inter_reference_type(new.clone());
        new
    }

    pub fn new_hybrid_enum_(hybrid_enum: &HybridEnum) -> Self {
        let new = Self::HybridEnum(hybrid_enum.id);
        new
    }

    /// Create a new instance of ReferenceType::Object
    pub fn new_object(object: &Object, store: &mut WoogStore) -> Self {
        let new = Self::Object(object.id);
        store.inter_reference_type(new.clone());
        new
    }

    pub fn new_object_(object: &Object) -> Self {
        let new = Self::Object(object.id);
        new
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference_type-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            ReferenceType::EnumerationField(id) => *id,
            ReferenceType::External(id) => *id,
            ReferenceType::HybridEnum(id) => *id,
            ReferenceType::Object(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference_type-struct-impl-nav-backward-1_M-to-reference"}}}
    /// Navigate to [`Reference`] across R13(1-M)
    pub fn r13_reference<'a>(&'a self, store: &'a WoogStore) -> Vec<&Reference> {
        store
            .iter_reference()
            .filter_map(|reference| {
                if reference.referent == self.id() {
                    Some(reference)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
