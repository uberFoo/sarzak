// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"value_type-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-use-statements"}}}
use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
use crate::v2::lu_dog::types::field::Field;
use crate::v2::lu_dog::types::function::Function;
use crate::v2::lu_dog::types::some::Some;
use crate::v2::lu_dog::types::woog_option::WoogOption;
use crate::v2::sarzak::types::ty::Ty;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-enum-documentation"}}}
/// Value Type
///
/// This is the main type abstraction used in Lu Dog. We mostly rely on what is available in
/// Sarzak, with two additions: ...
///
/// Two? I know that I need an Option<>. I'm not so sure about a & though. Everything from the
/// store is going to be by UUID, so all of my references are really "pointers" underneath.
/// I want them to be typed in the code though.
///
/// So how will the code work? We could store the type next to the pointer: (type, uuid). Huh
///. This is the eventual output domain. How does that affect my thinking?
///
/// This should end up looking like woog, but simpler. Woog was for generating rust. I want
/// to generate dwarf. Dwarf needs to be typed? If so, when are they resolved to uuid's eventually
///?
///
/// Option for now. We'll see later...
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ValueType {
    WoogOption(Uuid),
    Ty(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-implementation"}}}
impl ValueType {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-new-impl"}}}
    /// Create a new instance of ValueType::WoogOption
    pub fn new_woog_option(woog_option: &WoogOption, store: &mut LuDogStore) -> Self {
        let new = Self::WoogOption(woog_option.id());
        store.inter_value_type(new.clone());
        new
    }

    pub fn new_woog_option_(woog_option: &WoogOption) -> Self {
        let new = Self::WoogOption(woog_option.id());
        new
    }

    /// Create a new instance of ValueType::Ty
    pub fn new_ty(ty: &Ty, store: &mut LuDogStore) -> Self {
        let new = Self::Ty(ty.id());
        store.inter_value_type(new.clone());
        new
    }

    pub fn new_ty_(ty: &Ty) -> Self {
        let new = Self::Ty(ty.id());
        new
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            ValueType::WoogOption(id) => *id,
            ValueType::Ty(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-field"}}}
    /// Navigate to [`Field`] across R5(1-M)
    pub fn r5_field<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Field> {
        store
            .iter_field()
            .filter_map(|field| {
                if field.ty == self.id() {
                    Some(field)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_Mc-to-function"}}}
    /// Navigate to [`Function`] across R10(1-Mc)
    pub fn r10_function<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Function> {
        store
            .iter_function()
            .filter_map(|function| {
                if function.return_type == Some(self.id()) {
                    Some(function)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-some"}}}
    /// Navigate to [`Some`] across R2(1-M)
    pub fn r2_some<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Some> {
        store
            .iter_some()
            .filter_map(|some| {
                if some.inner_type == self.id() {
                    Some(some)
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
