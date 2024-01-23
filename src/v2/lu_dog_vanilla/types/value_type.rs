// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"value_type-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog_vanilla::types::char::CHAR;
use crate::v2::lu_dog_vanilla::types::empty::EMPTY;
use crate::v2::lu_dog_vanilla::types::enum_generic::EnumGeneric;
use crate::v2::lu_dog_vanilla::types::enumeration::Enumeration;
use crate::v2::lu_dog_vanilla::types::field::Field;
use crate::v2::lu_dog_vanilla::types::function::Function;
use crate::v2::lu_dog_vanilla::types::import::Import;
use crate::v2::lu_dog_vanilla::types::lambda::Lambda;
use crate::v2::lu_dog_vanilla::types::lambda_parameter::LambdaParameter;
use crate::v2::lu_dog_vanilla::types::list::List;
use crate::v2::lu_dog_vanilla::types::parameter::Parameter;
use crate::v2::lu_dog_vanilla::types::range::RANGE;
use crate::v2::lu_dog_vanilla::types::span::Span;
use crate::v2::lu_dog_vanilla::types::struct_generic::StructGeneric;
use crate::v2::lu_dog_vanilla::types::task::TASK;
use crate::v2::lu_dog_vanilla::types::tuple_field::TupleField;
use crate::v2::lu_dog_vanilla::types::type_cast::TypeCast;
use crate::v2::lu_dog_vanilla::types::unknown::UNKNOWN;
use crate::v2::lu_dog_vanilla::types::woog_struct::WoogStruct;
use crate::v2::lu_dog_vanilla::types::x_future::XFuture;
use crate::v2::lu_dog_vanilla::types::x_plugin::XPlugin;
use crate::v2::lu_dog_vanilla::types::x_value::XValue;
use crate::v2::lu_dog_vanilla::types::z_object_store::ZObjectStore;
use crate::v2::sarzak::types::ty::Ty;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
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
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ValueType {
    pub subtype: ValueTypeEnum,
    pub bogus: bool,
    pub id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ValueTypeEnum {
    Char(Uuid),
    Empty(Uuid),
    EnumGeneric(Uuid),
    Enumeration(Uuid),
    Function(Uuid),
    XFuture(Uuid),
    Import(Uuid),
    Lambda(Uuid),
    List(Uuid),
    ZObjectStore(Uuid),
    XPlugin(Uuid),
    Range(Uuid),
    WoogStruct(Uuid),
    StructGeneric(Uuid),
    Task(Uuid),
    Ty(Uuid),
    Unknown(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-implementation"}}}
impl ValueType {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_char"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_char(bogus: bool, store: &mut LuDogVanillaStore) -> ValueType {
        let id = Uuid::new_v4();
        let new = ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::Char(CHAR),
            id,
        };
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_empty"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_empty(bogus: bool, store: &mut LuDogVanillaStore) -> ValueType {
        let id = Uuid::new_v4();
        let new = ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::Empty(EMPTY),
            id,
        };
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_enum_generic"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_enum_generic(
        bogus: bool,
        subtype: &EnumGeneric,
        store: &mut LuDogVanillaStore,
    ) -> ValueType {
        let id = Uuid::new_v4();
        let new = ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::EnumGeneric(subtype.id),
            id,
        };
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_enumeration"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_enumeration(
        bogus: bool,
        subtype: &Enumeration,
        store: &mut LuDogVanillaStore,
    ) -> ValueType {
        let id = Uuid::new_v4();
        let new = ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::Enumeration(subtype.id),
            id,
        };
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_function"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_function(
        bogus: bool,
        subtype: &Function,
        store: &mut LuDogVanillaStore,
    ) -> ValueType {
        let id = Uuid::new_v4();
        let new = ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::Function(subtype.id),
            id,
        };
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_x_future"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_x_future(
        bogus: bool,
        subtype: &XFuture,
        store: &mut LuDogVanillaStore,
    ) -> ValueType {
        let id = Uuid::new_v4();
        let new = ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::XFuture(subtype.id),
            id,
        };
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_import"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_import(bogus: bool, subtype: &Import, store: &mut LuDogVanillaStore) -> ValueType {
        let id = Uuid::new_v4();
        let new = ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::Import(subtype.id),
            id,
        };
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_lambda"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_lambda(bogus: bool, subtype: &Lambda, store: &mut LuDogVanillaStore) -> ValueType {
        let id = Uuid::new_v4();
        let new = ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::Lambda(subtype.id),
            id,
        };
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_list"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_list(bogus: bool, subtype: &List, store: &mut LuDogVanillaStore) -> ValueType {
        let id = Uuid::new_v4();
        let new = ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::List(subtype.id),
            id,
        };
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_z_object_store"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_z_object_store(
        bogus: bool,
        subtype: &ZObjectStore,
        store: &mut LuDogVanillaStore,
    ) -> ValueType {
        let id = Uuid::new_v4();
        let new = ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::ZObjectStore(subtype.id),
            id,
        };
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_x_plugin"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_x_plugin(
        bogus: bool,
        subtype: &XPlugin,
        store: &mut LuDogVanillaStore,
    ) -> ValueType {
        let id = Uuid::new_v4();
        let new = ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::XPlugin(subtype.id),
            id,
        };
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_range"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_range(bogus: bool, store: &mut LuDogVanillaStore) -> ValueType {
        let id = Uuid::new_v4();
        let new = ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::Range(RANGE),
            id,
        };
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_woog_struct"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_woog_struct(
        bogus: bool,
        subtype: &WoogStruct,
        store: &mut LuDogVanillaStore,
    ) -> ValueType {
        let id = Uuid::new_v4();
        let new = ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::WoogStruct(subtype.id),
            id,
        };
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_struct_generic"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_struct_generic(
        bogus: bool,
        subtype: &StructGeneric,
        store: &mut LuDogVanillaStore,
    ) -> ValueType {
        let id = Uuid::new_v4();
        let new = ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::StructGeneric(subtype.id),
            id,
        };
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_task"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_task(bogus: bool, store: &mut LuDogVanillaStore) -> ValueType {
        let id = Uuid::new_v4();
        let new = ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::Task(TASK),
            id,
        };
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_ty"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_ty(bogus: bool, subtype: &Ty, store: &mut LuDogVanillaStore) -> ValueType {
        let id = Uuid::new_v4();
        let new = ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::Ty(subtype.id()),
            id,
        };
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_unknown"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub fn new_unknown(bogus: bool, store: &mut LuDogVanillaStore) -> ValueType {
        let id = Uuid::new_v4();
        let new = ValueType {
            bogus: bogus,
            subtype: ValueTypeEnum::Unknown(UNKNOWN),
            id,
        };
        store.inter_value_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-field"}}}
    /// Navigate to [`Field`] across R5(1-M)
    pub fn r5_field<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Field> {
        store
            .iter_field()
            .filter(|field| field.ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-function"}}}
    /// Navigate to [`Function`] across R10(1-M)
    pub fn r10_function<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Function> {
        store
            .iter_function()
            .filter(|function| function.return_type == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-x_future"}}}
    /// Navigate to [`XFuture`] across R2(1-M)
    pub fn r2_x_future<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&XFuture> {
        store
            .iter_x_future()
            .filter(|x_future| x_future.x_value == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-lambda"}}}
    /// Navigate to [`Lambda`] across R74(1-M)
    pub fn r74_lambda<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Lambda> {
        store
            .iter_lambda()
            .filter(|lambda| lambda.return_type == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_Mc-to-lambda_parameter"}}}
    /// Navigate to [`LambdaParameter`] across R77(1-Mc)
    pub fn r77_lambda_parameter<'a>(
        &'a self,
        store: &'a LuDogVanillaStore,
    ) -> Vec<&LambdaParameter> {
        store
            .iter_lambda_parameter()
            .filter(|lambda_parameter| lambda_parameter.ty == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-list"}}}
    /// Navigate to [`List`] across R36(1-M)
    pub fn r36_list<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&List> {
        store
            .iter_list()
            .filter(|list| list.ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-parameter"}}}
    /// Navigate to [`Parameter`] across R79(1-M)
    pub fn r79_parameter<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Parameter> {
        store
            .iter_parameter()
            .filter(|parameter| parameter.ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_Mc-to-span"}}}
    /// Navigate to [`Span`] across R62(1-Mc)
    pub fn r62_span<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Span> {
        store
            .iter_span()
            .filter(|span| span.ty == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-tuple_field"}}}
    /// Navigate to [`TupleField`] across R86(1-M)
    pub fn r86_tuple_field<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&TupleField> {
        store
            .iter_tuple_field()
            .filter(|tuple_field| tuple_field.ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-type_cast"}}}
    /// Navigate to [`TypeCast`] across R69(1-M)
    pub fn r69_type_cast<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&TypeCast> {
        store
            .iter_type_cast()
            .filter(|type_cast| type_cast.ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-x_value"}}}
    /// Navigate to [`XValue`] across R24(1-M)
    pub fn r24_x_value<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&XValue> {
        store
            .iter_x_value()
            .filter(|x_value| x_value.ty == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
