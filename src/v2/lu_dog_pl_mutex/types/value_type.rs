// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"value_type-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-use-statements"}}}
use crate::v2::lu_dog_pl_mutex::store::ObjectStore as LuDogPlMutexStore;
use crate::v2::lu_dog_pl_mutex::types::empty::EMPTY;
use crate::v2::lu_dog_pl_mutex::types::error::Error;
use crate::v2::lu_dog_pl_mutex::types::field::Field;
use crate::v2::lu_dog_pl_mutex::types::function::Function;
use crate::v2::lu_dog_pl_mutex::types::import::Import;
use crate::v2::lu_dog_pl_mutex::types::list::List;
use crate::v2::lu_dog_pl_mutex::types::range::RANGE;
use crate::v2::lu_dog_pl_mutex::types::reference::Reference;
use crate::v2::lu_dog_pl_mutex::types::span::Span;
use crate::v2::lu_dog_pl_mutex::types::type_cast::TypeCast;
use crate::v2::lu_dog_pl_mutex::types::unknown::UNKNOWN;
use crate::v2::lu_dog_pl_mutex::types::woog_option::WoogOption;
use crate::v2::lu_dog_pl_mutex::types::woog_struct::WoogStruct;
use crate::v2::lu_dog_pl_mutex::types::x_value::XValue;
use crate::v2::lu_dog_pl_mutex::types::z_object_store::ZObjectStore;
use crate::v2::sarzak::types::ty::Ty;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracy_client::span;
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
    pub fn new_empty(store: &LuDogPlMutexStore) -> Arc<Mutex<Self>> {
        // This is already in the store.
        store.exhume_value_type(&EMPTY).unwrap()
    }

    /// Create a new instance of ValueType::Error
    pub fn new_error(error: &Arc<Mutex<Error>>, store: &mut LuDogPlMutexStore) -> Arc<Mutex<Self>> {
        let id = error.lock().id();
        if let Some(error) = store.exhume_value_type(&id) {
            error
        } else {
            let new = Arc::new(Mutex::new(Self::Error(id)));
            store.inter_value_type(new.clone());
            new
        }
    }

    /// Create a new instance of ValueType::Function
    pub fn new_function(
        function: &Arc<Mutex<Function>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Self>> {
        let id = function.lock().id;
        if let Some(function) = store.exhume_value_type(&id) {
            function
        } else {
            let new = Arc::new(Mutex::new(Self::Function(id)));
            store.inter_value_type(new.clone());
            new
        }
    }

    /// Create a new instance of ValueType::Import
    pub fn new_import(
        import: &Arc<Mutex<Import>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Self>> {
        let id = import.lock().id;
        if let Some(import) = store.exhume_value_type(&id) {
            import
        } else {
            let new = Arc::new(Mutex::new(Self::Import(id)));
            store.inter_value_type(new.clone());
            new
        }
    }

    /// Create a new instance of ValueType::List
    pub fn new_list(list: &Arc<Mutex<List>>, store: &mut LuDogPlMutexStore) -> Arc<Mutex<Self>> {
        let id = list.lock().id;
        if let Some(list) = store.exhume_value_type(&id) {
            list
        } else {
            let new = Arc::new(Mutex::new(Self::List(id)));
            store.inter_value_type(new.clone());
            new
        }
    }

    /// Create a new instance of ValueType::ZObjectStore
    pub fn new_z_object_store(
        z_object_store: &Arc<Mutex<ZObjectStore>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Self>> {
        let id = z_object_store.lock().id;
        if let Some(z_object_store) = store.exhume_value_type(&id) {
            z_object_store
        } else {
            let new = Arc::new(Mutex::new(Self::ZObjectStore(id)));
            store.inter_value_type(new.clone());
            new
        }
    }

    /// Create a new instance of ValueType::WoogOption
    pub fn new_woog_option(
        woog_option: &Arc<Mutex<WoogOption>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Self>> {
        let id = woog_option.lock().id;
        if let Some(woog_option) = store.exhume_value_type(&id) {
            woog_option
        } else {
            let new = Arc::new(Mutex::new(Self::WoogOption(id)));
            store.inter_value_type(new.clone());
            new
        }
    }

    /// Create a new instance of ValueType::Range
    pub fn new_range(store: &LuDogPlMutexStore) -> Arc<Mutex<Self>> {
        // This is already in the store.
        store.exhume_value_type(&RANGE).unwrap()
    }

    /// Create a new instance of ValueType::Reference
    pub fn new_reference(
        reference: &Arc<Mutex<Reference>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Self>> {
        let id = reference.lock().id;
        if let Some(reference) = store.exhume_value_type(&id) {
            reference
        } else {
            let new = Arc::new(Mutex::new(Self::Reference(id)));
            store.inter_value_type(new.clone());
            new
        }
    }

    /// Create a new instance of ValueType::WoogStruct
    pub fn new_woog_struct(
        woog_struct: &Arc<Mutex<WoogStruct>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Self>> {
        let id = woog_struct.lock().id;
        if let Some(woog_struct) = store.exhume_value_type(&id) {
            woog_struct
        } else {
            let new = Arc::new(Mutex::new(Self::WoogStruct(id)));
            store.inter_value_type(new.clone());
            new
        }
    }

    /// Create a new instance of ValueType::Ty
    pub fn new_ty(ty: &Arc<Mutex<Ty>>, store: &mut LuDogPlMutexStore) -> Arc<Mutex<Self>> {
        let id = ty.lock().id();
        if let Some(ty) = store.exhume_value_type(&id) {
            ty
        } else {
            let new = Arc::new(Mutex::new(Self::Ty(id)));
            store.inter_value_type(new.clone());
            new
        }
    }

    /// Create a new instance of ValueType::Unknown
    pub fn new_unknown(store: &LuDogPlMutexStore) -> Arc<Mutex<Self>> {
        // This is already in the store.
        store.exhume_value_type(&UNKNOWN).unwrap()
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
    pub fn r5_field<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<Field>>> {
        span!("r5_field");
        store
            .iter_field()
            .filter(|field| field.lock().ty == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-function"}}}
    /// Navigate to [`Function`] across R10(1-M)
    pub fn r10_function<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<Function>>> {
        span!("r10_function");
        store
            .iter_function()
            .filter(|function| function.lock().return_type == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-list"}}}
    /// Navigate to [`List`] across R36(1-M)
    pub fn r36_list<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<List>>> {
        span!("r36_list");
        store
            .iter_list()
            .filter(|list| list.lock().ty == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-woog_option"}}}
    /// Navigate to [`WoogOption`] across R2(1-M)
    pub fn r2_woog_option<'a>(
        &'a self,
        store: &'a LuDogPlMutexStore,
    ) -> Vec<Arc<Mutex<WoogOption>>> {
        span!("r2_woog_option");
        store
            .iter_woog_option()
            .filter(|woog_option| woog_option.lock().ty == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-reference"}}}
    /// Navigate to [`Reference`] across R35(1-M)
    pub fn r35_reference<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<Reference>>> {
        span!("r35_reference");
        store
            .iter_reference()
            .filter(|reference| reference.lock().ty == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_Mc-to-span"}}}
    /// Navigate to [`Span`] across R62(1-Mc)
    pub fn r62_span<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<Span>>> {
        span!("r62_span");
        store
            .iter_span()
            .filter(|span| span.lock().ty == Some(self.id()))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-type_cast"}}}
    /// Navigate to [`TypeCast`] across R69(1-M)
    pub fn r69_type_cast<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<TypeCast>>> {
        span!("r69_type_cast");
        store
            .iter_type_cast()
            .filter(|type_cast| type_cast.lock().ty == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-x_value"}}}
    /// Navigate to [`XValue`] across R24(1-M)
    pub fn r24_x_value<'a>(&'a self, store: &'a LuDogPlMutexStore) -> Vec<Arc<Mutex<XValue>>> {
        span!("r24_x_value");
        store
            .iter_x_value()
            .filter(|x_value| x_value.lock().ty == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
