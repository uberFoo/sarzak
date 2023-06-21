// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"value_type-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-use-statements"}}}
use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
use crate::v2::lu_dog_vanilla::types::empty::EMPTY;
use crate::v2::lu_dog_vanilla::types::error::Error;
use crate::v2::lu_dog_vanilla::types::field::Field;
use crate::v2::lu_dog_vanilla::types::function::Function;
use crate::v2::lu_dog_vanilla::types::import::Import;
use crate::v2::lu_dog_vanilla::types::list::List;
use crate::v2::lu_dog_vanilla::types::range::RANGE;
use crate::v2::lu_dog_vanilla::types::reference::Reference;
use crate::v2::lu_dog_vanilla::types::span::Span;
use crate::v2::lu_dog_vanilla::types::type_cast::TypeCast;
use crate::v2::lu_dog_vanilla::types::unknown::UNKNOWN;
use crate::v2::lu_dog_vanilla::types::woog_option::WoogOption;
use crate::v2::lu_dog_vanilla::types::woog_struct::WoogStruct;
use crate::v2::lu_dog_vanilla::types::x_value::XValue;
use crate::v2::lu_dog_vanilla::types::z_object_store::ZObjectStore;
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
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ValueType {
    Empty(Uuid),
    Error(Uuid),
    Function(Uuid),
    Import(Uuid),
    List(Uuid),
    ZObjectStore(Uuid),
    WoogOption(Uuid),
    Range(Uuid),
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
    pub fn new_error(error: &Error, store: &mut LuDogVanillaStore) -> Self {
        let new = Self::Error(error.id());
        store.inter_value_type(new.clone());
        new
    }

    /// Create a new instance of ValueType::Function
    pub fn new_function(function: &Function, store: &mut LuDogVanillaStore) -> Self {
        let new = Self::Function(function.id);
        store.inter_value_type(new.clone());
        new
    }

    /// Create a new instance of ValueType::Import
    pub fn new_import(import: &Import, store: &mut LuDogVanillaStore) -> Self {
        let new = Self::Import(import.id);
        store.inter_value_type(new.clone());
        new
    }

    /// Create a new instance of ValueType::List
    pub fn new_list(list: &List, store: &mut LuDogVanillaStore) -> Self {
        let new = Self::List(list.id);
        store.inter_value_type(new.clone());
        new
    }

    /// Create a new instance of ValueType::ZObjectStore
    pub fn new_z_object_store(
        z_object_store: &ZObjectStore,
        store: &mut LuDogVanillaStore,
    ) -> Self {
        let new = Self::ZObjectStore(z_object_store.id);
        store.inter_value_type(new.clone());
        new
    }

    /// Create a new instance of ValueType::WoogOption
    pub fn new_woog_option(woog_option: &WoogOption, store: &mut LuDogVanillaStore) -> Self {
        let new = Self::WoogOption(woog_option.id);
        store.inter_value_type(new.clone());
        new
    }

    /// Create a new instance of ValueType::Range
    pub fn new_range() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Range(RANGE)
    }

    /// Create a new instance of ValueType::Reference
    pub fn new_reference(reference: &Reference, store: &mut LuDogVanillaStore) -> Self {
        let new = Self::Reference(reference.id);
        store.inter_value_type(new.clone());
        new
    }

    /// Create a new instance of ValueType::WoogStruct
    pub fn new_woog_struct(woog_struct: &WoogStruct, store: &mut LuDogVanillaStore) -> Self {
        let new = Self::WoogStruct(woog_struct.id);
        store.inter_value_type(new.clone());
        new
    }

    /// Create a new instance of ValueType::Ty
    pub fn new_ty(ty: &Ty, store: &mut LuDogVanillaStore) -> Self {
        let new = Self::Ty(ty.id());
        store.inter_value_type(new.clone());
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
            ValueType::Import(id) => *id,
            ValueType::List(id) => *id,
            ValueType::ZObjectStore(id) => *id,
            ValueType::WoogOption(id) => *id,
            ValueType::Range(id) => *id,
            ValueType::Reference(id) => *id,
            ValueType::WoogStruct(id) => *id,
            ValueType::Ty(id) => *id,
            ValueType::Unknown(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-field"}}}
    /// Navigate to [`Field`] across R5(1-M)
    pub fn r5_field<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Field> {
        store
            .iter_field()
            .filter(|field| field.ty == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-function"}}}
    /// Navigate to [`Function`] across R10(1-M)
    pub fn r10_function<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Function> {
        store
            .iter_function()
            .filter(|function| function.return_type == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-list"}}}
    /// Navigate to [`List`] across R36(1-M)
    pub fn r36_list<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&List> {
        store
            .iter_list()
            .filter(|list| list.ty == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-woog_option"}}}
    /// Navigate to [`WoogOption`] across R2(1-M)
    pub fn r2_woog_option<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&WoogOption> {
        store
            .iter_woog_option()
            .filter(|woog_option| woog_option.ty == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-reference"}}}
    /// Navigate to [`Reference`] across R35(1-M)
    pub fn r35_reference<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Reference> {
        store
            .iter_reference()
            .filter(|reference| reference.ty == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_Mc-to-span"}}}
    /// Navigate to [`Span`] across R62(1-Mc)
    pub fn r62_span<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Span> {
        store
            .iter_span()
            .filter(|span| span.ty == Some(self.id()))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-type_cast"}}}
    /// Navigate to [`TypeCast`] across R69(1-M)
    pub fn r69_type_cast<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&TypeCast> {
        store
            .iter_type_cast()
            .filter(|type_cast| type_cast.ty == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-x_value"}}}
    /// Navigate to [`XValue`] across R24(1-M)
    pub fn r24_x_value<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&XValue> {
        store
            .iter_x_value()
            .filter(|x_value| x_value.ty == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
