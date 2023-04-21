// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"value_type-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-use-statements"}}}
use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
use crate::v2::lu_dog::types::empty::EMPTY;
use crate::v2::lu_dog::types::error::Error;
use crate::v2::lu_dog::types::field::Field;
use crate::v2::lu_dog::types::function::Function;
use crate::v2::lu_dog::types::list::List;
use crate::v2::lu_dog::types::reference::Reference;
use crate::v2::lu_dog::types::unknown::UNKNOWN;
use crate::v2::lu_dog::types::value::Value;
use crate::v2::lu_dog::types::woog_option::WoogOption;
use crate::v2::lu_dog::types::woog_struct::WoogStruct;
use crate::v2::lu_dog::types::z_object_store::ZObjectStore;
use crate::v2::sarzak::types::ty::Ty;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-enum-documentation"}}}
/// Value Type
///
/// This is the main type abstraction used in Lu Dog. We mostly rely on what is available in
///  Sarzak, with two additions: ...
///
/// Two? I know that I need an Option<>. I'm not so sure about a & though. Everything from the
///  store is going to be by UUID, so all of my references are really "pointers" underneath.
///  I want them to be typed in the code though.
///
/// So how will the code work? We could store the type next to the pointer: (type, uuid). Huh
/// . This is the eventual output domain. How does that affect my thinking?
///
/// This should end up looking like woog, but simpler. Woog was for generating rust. I want
///  to generate dwarf. Dwarf needs to be typed? If so, when are they resolved to uuid's eventually
/// ?
///
/// Option for now. We'll see later...
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ValueType {
    Empty(Uuid),
    Error(Uuid),
    Function(Uuid),
    List(Uuid),
    ZObjectStore(Uuid),
    WoogOption(Uuid),
    Reference(Uuid),
    WoogStruct(Uuid),
    Ty(Uuid),
    Unknown(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-implementation"}}}
impl ValueType {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-new-impl"}}}
    /// Create a new instance of ValueType::Empty
    pub fn new_empty() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Empty(EMPTY)
    }

    /// Create a new instance of ValueType::Error
    pub fn new_error(error: &Error, store: &mut LuDogStore) -> Self {
        let new = Self::Error(error.id());
        store.inter_value_type(new.clone());
        new
    }

    pub fn new_error_(error: &Error) -> Self {
        let new = Self::Error(error.id());
        new
    }

    /// Create a new instance of ValueType::Function
    pub fn new_function(function: &Function, store: &mut LuDogStore) -> Self {
        let new = Self::Function(function.id);
        store.inter_value_type(new.clone());
        new
    }

    pub fn new_function_(function: &Function) -> Self {
        let new = Self::Function(function.id);
        new
    }

    /// Create a new instance of ValueType::List
    pub fn new_list(list: &List, store: &mut LuDogStore) -> Self {
        let new = Self::List(list.id);
        store.inter_value_type(new.clone());
        new
    }

    pub fn new_list_(list: &List) -> Self {
        let new = Self::List(list.id);
        new
    }

    /// Create a new instance of ValueType::ZObjectStore
    pub fn new_z_object_store(z_object_store: &ZObjectStore, store: &mut LuDogStore) -> Self {
        let new = Self::ZObjectStore(z_object_store.id);
        store.inter_value_type(new.clone());
        new
    }

    pub fn new_z_object_store_(z_object_store: &ZObjectStore) -> Self {
        let new = Self::ZObjectStore(z_object_store.id);
        new
    }

    /// Create a new instance of ValueType::WoogOption
    pub fn new_woog_option(woog_option: &WoogOption, store: &mut LuDogStore) -> Self {
        let new = Self::WoogOption(woog_option.id);
        store.inter_value_type(new.clone());
        new
    }

    pub fn new_woog_option_(woog_option: &WoogOption) -> Self {
        let new = Self::WoogOption(woog_option.id);
        new
    }

    /// Create a new instance of ValueType::Reference
    pub fn new_reference(reference: &Reference, store: &mut LuDogStore) -> Self {
        let new = Self::Reference(reference.id);
        store.inter_value_type(new.clone());
        new
    }

    pub fn new_reference_(reference: &Reference) -> Self {
        let new = Self::Reference(reference.id);
        new
    }

    /// Create a new instance of ValueType::WoogStruct
    pub fn new_woog_struct(woog_struct: &WoogStruct, store: &mut LuDogStore) -> Self {
        let new = Self::WoogStruct(woog_struct.id);
        store.inter_value_type(new.clone());
        new
    }

    pub fn new_woog_struct_(woog_struct: &WoogStruct) -> Self {
        let new = Self::WoogStruct(woog_struct.id);
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

    /// Create a new instance of ValueType::Unknown
    pub fn new_unknown() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Unknown(UNKNOWN)
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            ValueType::Empty(id) => *id,
            ValueType::Error(id) => *id,
            ValueType::Function(id) => *id,
            ValueType::List(id) => *id,
            ValueType::ZObjectStore(id) => *id,
            ValueType::WoogOption(id) => *id,
            ValueType::Reference(id) => *id,
            ValueType::WoogStruct(id) => *id,
            ValueType::Ty(id) => *id,
            ValueType::Unknown(id) => *id,
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
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-function"}}}
    /// Navigate to [`Function`] across R10(1-M)
    pub fn r10_function<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Function> {
        store
            .iter_function()
            .filter_map(|function| {
                if function.return_type == self.id() {
                    Some(function)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-some"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-list"}}}
    /// Navigate to [`List`] across R36(1-M)
    pub fn r36_list<'a>(&'a self, store: &'a LuDogStore) -> Vec<&List> {
        store
            .iter_list()
            .filter_map(|list| {
                if list.ty == self.id() {
                    Some(list)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-woog_option"}}}
    /// Navigate to [`WoogOption`] across R2(1-M)
    pub fn r2_woog_option<'a>(&'a self, store: &'a LuDogStore) -> Vec<&WoogOption> {
        store
            .iter_woog_option()
            .filter_map(|woog_option| {
                if woog_option.ty == self.id() {
                    Some(woog_option)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-reference"}}}
    /// Navigate to [`Reference`] across R35(1-M)
    pub fn r35_reference<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Reference> {
        store
            .iter_reference()
            .filter_map(|reference| {
                if reference.ty == self.id() {
                    Some(reference)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-value"}}}
    /// Navigate to [`Value`] across R24(1-M)
    pub fn r24_value<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Value> {
        store
            .iter_value()
            .filter_map(|value| {
                if value.ty == self.id() {
                    Some(value)
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
