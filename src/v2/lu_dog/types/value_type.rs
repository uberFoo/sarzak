// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"value_type-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-use-statements"}}}
use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
use crate::v2::lu_dog::types::empty::EMPTY;
use crate::v2::lu_dog::types::error::Error;
use crate::v2::lu_dog::types::field::Field;
use crate::v2::lu_dog::types::function::Function;
use crate::v2::lu_dog::types::import::Import;
use crate::v2::lu_dog::types::list::List;
use crate::v2::lu_dog::types::range::RANGE;
use crate::v2::lu_dog::types::reference::Reference;
use crate::v2::lu_dog::types::span::Span;
use crate::v2::lu_dog::types::type_cast::TypeCast;
use crate::v2::lu_dog::types::unknown::UNKNOWN;
use crate::v2::lu_dog::types::woog_option::WoogOption;
use crate::v2::lu_dog::types::woog_struct::WoogStruct;
use crate::v2::lu_dog::types::x_value::XValue;
use crate::v2::lu_dog::types::z_object_store::ZObjectStore;
use crate::v2::sarzak::types::ty::Ty;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
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
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
    pub fn new_empty(store: &LuDogStore) -> Rc<RefCell<Self>> {
        // This is already in the store.
        store.exhume_value_type(&EMPTY).unwrap()
    }

    /// Create a new instance of ValueType::Error
    pub fn new_error(error: &Rc<RefCell<Error>>, store: &mut LuDogStore) -> Rc<RefCell<Self>> {
        let id = error.borrow().id();
        if let Some(error) = store.exhume_value_type(&id) {
            error
        } else {
            let new = Rc::new(RefCell::new(Self::Error(id)));
            store.inter_value_type(new.clone());
            new
        }
    }

    /// Create a new instance of ValueType::Function
    pub fn new_function(
        function: &Rc<RefCell<Function>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Self>> {
        let id = function.borrow().id;
        if let Some(function) = store.exhume_value_type(&id) {
            function
        } else {
            let new = Rc::new(RefCell::new(Self::Function(id)));
            store.inter_value_type(new.clone());
            new
        }
    }

    /// Create a new instance of ValueType::Import
    pub fn new_import(import: &Rc<RefCell<Import>>, store: &mut LuDogStore) -> Rc<RefCell<Self>> {
        let id = import.borrow().id;
        if let Some(import) = store.exhume_value_type(&id) {
            import
        } else {
            let new = Rc::new(RefCell::new(Self::Import(id)));
            store.inter_value_type(new.clone());
            new
        }
    }

    /// Create a new instance of ValueType::List
    pub fn new_list(list: &Rc<RefCell<List>>, store: &mut LuDogStore) -> Rc<RefCell<Self>> {
        let id = list.borrow().id;
        if let Some(list) = store.exhume_value_type(&id) {
            list
        } else {
            let new = Rc::new(RefCell::new(Self::List(id)));
            store.inter_value_type(new.clone());
            new
        }
    }

    /// Create a new instance of ValueType::ZObjectStore
    pub fn new_z_object_store(
        z_object_store: &Rc<RefCell<ZObjectStore>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Self>> {
        let id = z_object_store.borrow().id;
        if let Some(z_object_store) = store.exhume_value_type(&id) {
            z_object_store
        } else {
            let new = Rc::new(RefCell::new(Self::ZObjectStore(id)));
            store.inter_value_type(new.clone());
            new
        }
    }

    /// Create a new instance of ValueType::WoogOption
    pub fn new_woog_option(
        woog_option: &Rc<RefCell<WoogOption>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Self>> {
        let id = woog_option.borrow().id;
        if let Some(woog_option) = store.exhume_value_type(&id) {
            woog_option
        } else {
            let new = Rc::new(RefCell::new(Self::WoogOption(id)));
            store.inter_value_type(new.clone());
            new
        }
    }

    /// Create a new instance of ValueType::Range
    pub fn new_range(store: &LuDogStore) -> Rc<RefCell<Self>> {
        // This is already in the store.
        store.exhume_value_type(&RANGE).unwrap()
    }

    /// Create a new instance of ValueType::Reference
    pub fn new_reference(
        reference: &Rc<RefCell<Reference>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Self>> {
        let id = reference.borrow().id;
        if let Some(reference) = store.exhume_value_type(&id) {
            reference
        } else {
            let new = Rc::new(RefCell::new(Self::Reference(id)));
            store.inter_value_type(new.clone());
            new
        }
    }

    /// Create a new instance of ValueType::WoogStruct
    pub fn new_woog_struct(
        woog_struct: &Rc<RefCell<WoogStruct>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Self>> {
        let id = woog_struct.borrow().id;
        if let Some(woog_struct) = store.exhume_value_type(&id) {
            woog_struct
        } else {
            let new = Rc::new(RefCell::new(Self::WoogStruct(id)));
            store.inter_value_type(new.clone());
            new
        }
    }

    /// Create a new instance of ValueType::Ty
    pub fn new_ty(ty: &Rc<RefCell<Ty>>, store: &mut LuDogStore) -> Rc<RefCell<Self>> {
        let id = ty.borrow().id();
        if let Some(ty) = store.exhume_value_type(&id) {
            ty
        } else {
            let new = Rc::new(RefCell::new(Self::Ty(id)));
            store.inter_value_type(new.clone());
            new
        }
    }

    /// Create a new instance of ValueType::Unknown
    pub fn new_unknown(store: &LuDogStore) -> Rc<RefCell<Self>> {
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
    pub fn r5_field<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Field>>> {
        span!("r5_field");
        store
            .iter_field()
            .filter(|field| field.borrow().ty == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-function"}}}
    /// Navigate to [`Function`] across R10(1-M)
    pub fn r10_function<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Function>>> {
        span!("r10_function");
        store
            .iter_function()
            .filter(|function| function.borrow().return_type == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-list"}}}
    /// Navigate to [`List`] across R36(1-M)
    pub fn r36_list<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<List>>> {
        span!("r36_list");
        store
            .iter_list()
            .filter(|list| list.borrow().ty == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-woog_option"}}}
    /// Navigate to [`WoogOption`] across R2(1-M)
    pub fn r2_woog_option<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<WoogOption>>> {
        span!("r2_woog_option");
        store
            .iter_woog_option()
            .filter(|woog_option| woog_option.borrow().ty == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-reference"}}}
    /// Navigate to [`Reference`] across R35(1-M)
    pub fn r35_reference<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Reference>>> {
        span!("r35_reference");
        store
            .iter_reference()
            .filter(|reference| reference.borrow().ty == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_Mc-to-span"}}}
    /// Navigate to [`Span`] across R62(1-Mc)
    pub fn r62_span<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Span>>> {
        span!("r62_span");
        store
            .iter_span()
            .filter_map(|span| {
                if span.borrow().ty == Some(self.id()) {
                    Some(span)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-type_cast"}}}
    /// Navigate to [`TypeCast`] across R69(1-M)
    pub fn r69_type_cast<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<TypeCast>>> {
        span!("r69_type_cast");
        store
            .iter_type_cast()
            .filter(|type_cast| type_cast.borrow().ty == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-x_value"}}}
    /// Navigate to [`XValue`] across R24(1-M)
    pub fn r24_x_value<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<XValue>>> {
        span!("r24_x_value");
        store
            .iter_x_value()
            .filter(|x_value| x_value.borrow().ty == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
