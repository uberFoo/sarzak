// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"value_type-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::char::CHAR;
use crate::v2::lu_dog_vec::types::empty::EMPTY;
use crate::v2::lu_dog_vec::types::enumeration::Enumeration;
use crate::v2::lu_dog_vec::types::field::Field;
use crate::v2::lu_dog_vec::types::function::Function;
use crate::v2::lu_dog_vec::types::generic::Generic;
use crate::v2::lu_dog_vec::types::import::Import;
use crate::v2::lu_dog_vec::types::lambda::Lambda;
use crate::v2::lu_dog_vec::types::lambda_parameter::LambdaParameter;
use crate::v2::lu_dog_vec::types::list::List;
use crate::v2::lu_dog_vec::types::parameter::Parameter;
use crate::v2::lu_dog_vec::types::plugin::Plugin;
use crate::v2::lu_dog_vec::types::range::RANGE;
use crate::v2::lu_dog_vec::types::span::Span;
use crate::v2::lu_dog_vec::types::task::TASK;
use crate::v2::lu_dog_vec::types::tuple_field::TupleField;
use crate::v2::lu_dog_vec::types::type_cast::TypeCast;
use crate::v2::lu_dog_vec::types::unknown::UNKNOWN;
use crate::v2::lu_dog_vec::types::woog_struct::WoogStruct;
use crate::v2::lu_dog_vec::types::x_future::XFuture;
use crate::v2::lu_dog_vec::types::x_value::XValue;
use crate::v2::lu_dog_vec::types::z_object_store::ZObjectStore;
use crate::v2::sarzak::types::ty::Ty;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-hybrid-documentation"}}}
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValueType {
    pub subtype: ValueTypeEnum,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ValueTypeEnum {
    Char(Uuid),
    Empty(Uuid),
    Enumeration(usize),
    Function(usize),
    XFuture(usize),
    Generic(usize),
    Import(usize),
    Lambda(usize),
    List(usize),
    ZObjectStore(usize),
    Plugin(usize),
    Range(Uuid),
    WoogStruct(usize),
    Task(Uuid),
    Ty(Uuid),
    Unknown(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-implementation"}}}
impl ValueType {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_char"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_char(store: &mut LuDogVecStore) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::Char(CHAR),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_empty"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_empty(store: &mut LuDogVecStore) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::Empty(EMPTY),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_enumeration"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_enumeration(
        subtype: &Rc<RefCell<Enumeration>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::Enumeration(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_error"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_x_error"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_function"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_function(
        subtype: &Rc<RefCell<Function>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::Function(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_future"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_x_future"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_x_future(
        subtype: &Rc<RefCell<XFuture>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::XFuture(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_generic"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_generic(
        subtype: &Rc<RefCell<Generic>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::Generic(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_import"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_import(
        subtype: &Rc<RefCell<Import>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::Import(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_lambda"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_lambda(
        subtype: &Rc<RefCell<Lambda>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::Lambda(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_list"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_list(
        subtype: &Rc<RefCell<List>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::List(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_z_object_store"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_z_object_store(
        subtype: &Rc<RefCell<ZObjectStore>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::ZObjectStore(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_woog_option"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_range"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_plugin"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_plugin(
        subtype: &Rc<RefCell<Plugin>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::Plugin(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_reference"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_range"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_range(store: &mut LuDogVecStore) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::Range(RANGE),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_woog_struct"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_woog_struct(
        subtype: &Rc<RefCell<WoogStruct>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::WoogStruct(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_task"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_task(store: &mut LuDogVecStore) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::Task(TASK),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_ty"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_ty(
        subtype: &std::sync::Arc<std::sync::RwLock<Ty>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::Ty(subtype.read().unwrap().id()),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_unknown"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_unknown(store: &mut LuDogVecStore) -> Rc<RefCell<ValueType>> {
        store.inter_value_type(|id| {
            Rc::new(RefCell::new(ValueType {
                subtype: ValueTypeEnum::Unknown(UNKNOWN),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-field"}}}
    /// Navigate to [`Field`] across R5(1-M)
    pub fn r5_field<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Field>>> {
        store
            .iter_field()
            .filter(|field| field.borrow().ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-function"}}}
    /// Navigate to [`Function`] across R10(1-M)
    pub fn r10_function<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Function>>> {
        store
            .iter_function()
            .filter(|function| function.borrow().return_type == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-future"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-x_future"}}}
    /// Navigate to [`XFuture`] across R2(1-M)
    pub fn r2_x_future<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<XFuture>>> {
        store
            .iter_x_future()
            .filter(|x_future| x_future.borrow().x_value == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-generic"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_Mc-to-generic"}}}
    /// Navigate to [`Generic`] across R99(1-Mc)
    pub fn r99_generic<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Generic>>> {
        store
            .iter_generic()
            .filter(|generic| generic.borrow().ty == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-lambda"}}}
    /// Navigate to [`Lambda`] across R74(1-M)
    pub fn r74_lambda<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Lambda>>> {
        store
            .iter_lambda()
            .filter(|lambda| lambda.borrow().return_type == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_Mc-to-lambda_parameter"}}}
    /// Navigate to [`LambdaParameter`] across R77(1-Mc)
    pub fn r77_lambda_parameter<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<LambdaParameter>>> {
        store
            .iter_lambda_parameter()
            .filter(|lambda_parameter| lambda_parameter.borrow().ty == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-list"}}}
    /// Navigate to [`List`] across R36(1-M)
    pub fn r36_list<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<List>>> {
        store
            .iter_list()
            .filter(|list| list.borrow().ty == self.id)
            // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
            // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-woog_option"}}}
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-parameter"}}}
    /// Navigate to [`Parameter`] across R79(1-M)
    pub fn r79_parameter<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Parameter>>> {
        store
            .iter_parameter()
            .filter(|parameter| parameter.borrow().ty == self.id)
            // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
            // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-reference"}}}
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_Mc-to-span"}}}
    /// Navigate to [`Span`] across R62(1-Mc)
    pub fn r62_span<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Span>>> {
        store
            .iter_span()
            .filter(|span| span.borrow().ty == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-tuple_field"}}}
    /// Navigate to [`TupleField`] across R86(1-M)
    pub fn r86_tuple_field<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<TupleField>>> {
        store
            .iter_tuple_field()
            .filter(|tuple_field| tuple_field.borrow().ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-type_cast"}}}
    /// Navigate to [`TypeCast`] across R69(1-M)
    pub fn r69_type_cast<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<TypeCast>>> {
        store
            .iter_type_cast()
            .filter(|type_cast| type_cast.borrow().ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-x_value"}}}
    /// Navigate to [`XValue`] across R24(1-M)
    pub fn r24_x_value<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<XValue>>> {
        store
            .iter_x_value()
            .filter(|x_value| x_value.borrow().ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-implementation"}}}
impl PartialEq for ValueType {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
