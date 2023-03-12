// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"grace_type-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grace_type-use-statements"}}}
use crate::v2::sarzak::types::ty::Ty;
use crate::v2::woog::types::reference::Reference;
use crate::v2::woog::types::time_stamp::TimeStamp;
use crate::v2::woog::types::woog_option::WoogOption;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grace_type-enum-documentation"}}}
/// Grace Model Compiler Type
///
/// The model compiler domain contains at least one type that doesn't make sense within the
/// modeling domain. That type is an object reference. References, in my mind, have no place
/// in a modeling domain.
///
/// So that's what this is about.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grace_type-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum GraceType {
    WoogOption(Uuid),
    Reference(Uuid),
    TimeStamp(Uuid),
    Ty(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grace_type-implementation"}}}
impl GraceType {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grace_type-new-impl"}}}
    /// Create a new instance of GraceType::WoogOption
    pub fn new_woog_option(woog_option: &WoogOption, store: &mut WoogStore) -> Self {
        let new = Self::WoogOption(woog_option.id);
        store.inter_grace_type(new.clone());
        new
    }

    /// Create a new instance of GraceType::Reference
    pub fn new_reference(reference: &Reference, store: &mut WoogStore) -> Self {
        let new = Self::Reference(reference.id);
        store.inter_grace_type(new.clone());
        new
    }

    /// Create a new instance of GraceType::TimeStamp
    pub fn new_time_stamp(time_stamp: &TimeStamp, store: &mut WoogStore) -> Self {
        let new = Self::TimeStamp(time_stamp.id);
        store.inter_grace_type(new.clone());
        new
    }

    /// Create a new instance of GraceType::Ty
    pub fn new_ty(ty: &Ty, store: &mut WoogStore) -> Self {
        let new = Self::Ty(ty.id());
        store.inter_grace_type(new.clone());
        new
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grace_type-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            GraceType::WoogOption(id) => *id,
            GraceType::Reference(id) => *id,
            GraceType::TimeStamp(id) => *id,
            GraceType::Ty(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

impl From<Ty> for GraceType {
    fn from(ty: Ty) -> Self {
        GraceType::Ty(ty.id())
    }
}

impl From<&Ty> for GraceType {
    fn from(ty: &Ty) -> Self {
        GraceType::Ty(ty.id())
    }
}

// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
