// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"grace_type-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grace_type-use-statements"}}}
use crate::v2::sarzak::types::ty::Ty;
use crate::v2::woog::store::ObjectStore as WoogStore;
use crate::v2::woog::types::field::Field;
use crate::v2::woog::types::reference::Reference;
use crate::v2::woog::types::time_stamp::TimeStamp;
use crate::v2::woog::types::usize::USIZE;
use crate::v2::woog::types::woog_option::WoogOption;
use crate::v2::woog::types::x_value::XValue;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::RwLock;
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
    pub fn new_woog_option(
        woog_option: &Arc<RwLock<WoogOption>>,
        store: &mut WoogStore,
    ) -> Arc<RwLock<Self>> {
        let id = woog_option.read().unwrap().id;
        if let Some(woog_option) = store.exhume_grace_type(&id) {
            woog_option
        } else {
            let new = Arc::new(RwLock::new(Self::WoogOption(id)));
            store.inter_grace_type(new.clone());
            new
        }
    } // wtf?

    /// Create a new instance of GraceType::Reference
    pub fn new_reference(
        reference: &Arc<RwLock<Reference>>,
        store: &mut WoogStore,
    ) -> Arc<RwLock<Self>> {
        let id = reference.read().unwrap().id;
        if let Some(reference) = store.exhume_grace_type(&id) {
            reference
        } else {
            let new = Arc::new(RwLock::new(Self::Reference(id)));
            store.inter_grace_type(new.clone());
            new
        }
    } // wtf?

    /// Create a new instance of GraceType::TimeStamp
    pub fn new_time_stamp(
        time_stamp: &Arc<RwLock<TimeStamp>>,
        store: &mut WoogStore,
    ) -> Arc<RwLock<Self>> {
        let id = time_stamp.read().unwrap().id;
        if let Some(time_stamp) = store.exhume_grace_type(&id) {
            time_stamp
        } else {
            let new = Arc::new(RwLock::new(Self::TimeStamp(id)));
            store.inter_grace_type(new.clone());
            new
        }
    } // wtf?

    /// Create a new instance of GraceType::Ty
    pub fn new_ty(ty: &Arc<RwLock<Ty>>, store: &mut WoogStore) -> Arc<RwLock<Self>> {
        let id = ty.read().unwrap().id();
        if let Some(ty) = store.exhume_grace_type(&id) {
            ty
        } else {
            let new = Arc::new(RwLock::new(Self::Ty(id)));
            store.inter_grace_type(new.clone());
            new
        }
    } // wtf?

    /// Create a new instance of GraceType::Usize
    pub fn new_usize(store: &WoogStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_grace_type(&USIZE).unwrap()
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
    pub fn r29_field<'a>(&'a self, store: &'a WoogStore) -> Vec<Arc<RwLock<Field>>> {
        store
            .iter_field()
            .filter(|field| field.read().unwrap().ty == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grace_type-struct-impl-nav-backward-1_M-to-woog_option"}}}
    /// Navigate to [`WoogOption`] across R20(1-M)
    pub fn r20_woog_option<'a>(&'a self, store: &'a WoogStore) -> Vec<Arc<RwLock<WoogOption>>> {
        store
            .iter_woog_option()
            .filter(|woog_option| woog_option.read().unwrap().ty == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grace_type-struct-impl-nav-backward-1_M-to-value"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"grace_type-struct-impl-nav-backward-1_M-to-x_value"}}}
    /// Navigate to [`XValue`] across R3(1-M)
    pub fn r3_x_value<'a>(&'a self, store: &'a WoogStore) -> Vec<Arc<RwLock<XValue>>> {
        store
            .iter_x_value()
            .filter(|x_value| x_value.read().unwrap().ty == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
