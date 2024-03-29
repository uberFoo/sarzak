// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"grace_type-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grace_type-use-statements"}}}
use crate::v2::sarzak::types::ty::Ty;
use crate::v2::woog_single::store::ObjectStore as WoogSingleStore;
use crate::v2::woog_single::types::field::Field;
use crate::v2::woog_single::types::reference::Reference;
use crate::v2::woog_single::types::time_stamp::TimeStamp;
use crate::v2::woog_single::types::usize::USIZE;
use crate::v2::woog_single::types::woog_option::WoogOption;
use crate::v2::woog_single::types::x_value::XValue;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grace_type-enum-documentation"}}}
/// Grace Model Compiler Type
///
/// The model compiler domain contains at least one type that doesn't make sense within the
///  modeling domain. That type is an object reference. References, in my mind, have no place
///  in a modeling domain.
///
/// So that's what this is about.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grace_type-enum-definition"}}}
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum GraceType {
    WoogOption(Uuid),
    Reference(Uuid),
    TimeStamp(Uuid),
    Ty(Uuid),
    Usize(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grace_type-implementation"}}}
impl GraceType {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grace_type-new-impl"}}}
    /// Create a new instance of GraceType::WoogOption
    pub fn new_woog_option(woog_option: &WoogOption, store: &mut WoogSingleStore) -> Self {
        let new = Self::WoogOption(woog_option.id);
        store.inter_grace_type(new.clone());
        new
    } // wtf?

    /// Create a new instance of GraceType::Reference
    pub fn new_reference(reference: &Reference, store: &mut WoogSingleStore) -> Self {
        let new = Self::Reference(reference.id);
        store.inter_grace_type(new.clone());
        new
    } // wtf?

    /// Create a new instance of GraceType::TimeStamp
    pub fn new_time_stamp(time_stamp: &TimeStamp, store: &mut WoogSingleStore) -> Self {
        let new = Self::TimeStamp(time_stamp.id);
        store.inter_grace_type(new.clone());
        new
    } // wtf?

    /// Create a new instance of GraceType::Ty
    pub fn new_ty(ty: &Ty, store: &mut WoogSingleStore) -> Self {
        let new = Self::Ty(ty.id());
        store.inter_grace_type(new.clone());
        new
    } // wtf?

    /// Create a new instance of GraceType::Usize
    pub fn new_usize() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Usize(USIZE)
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grace_type-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Self::WoogOption(id) => *id,
            Self::Reference(id) => *id,
            Self::TimeStamp(id) => *id,
            Self::Ty(id) => *id,
            Self::Usize(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grace_type-struct-impl-nav-backward-1_M-to-field"}}}
    /// Navigate to [`Field`] across R29(1-M)
    pub fn r29_field<'a>(&'a self, store: &'a WoogSingleStore) -> Vec<&Field> {
        store
            .iter_field()
            .filter(|field| field.ty == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grace_type-struct-impl-nav-backward-1_M-to-woog_option"}}}
    /// Navigate to [`WoogOption`] across R20(1-M)
    pub fn r20_woog_option<'a>(&'a self, store: &'a WoogSingleStore) -> Vec<&WoogOption> {
        store
            .iter_woog_option()
            .filter(|woog_option| woog_option.ty == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grace_type-struct-impl-nav-backward-1_M-to-x_value"}}}
    /// Navigate to [`XValue`] across R3(1-M)
    pub fn r3_x_value<'a>(&'a self, store: &'a WoogSingleStore) -> Vec<&XValue> {
        store
            .iter_x_value()
            .filter(|x_value| x_value.ty == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
