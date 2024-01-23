// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"value_type-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use uuid::Uuid;

use crate::v2::lu_dog_async::types::char::CHAR;
use crate::v2::lu_dog_async::types::empty::EMPTY;
use crate::v2::lu_dog_async::types::enum_generic::EnumGeneric;
use crate::v2::lu_dog_async::types::enumeration::Enumeration;
use crate::v2::lu_dog_async::types::field::Field;
use crate::v2::lu_dog_async::types::func_generic::FuncGeneric;
use crate::v2::lu_dog_async::types::function::Function;
use crate::v2::lu_dog_async::types::import::Import;
use crate::v2::lu_dog_async::types::lambda::Lambda;
use crate::v2::lu_dog_async::types::lambda_parameter::LambdaParameter;
use crate::v2::lu_dog_async::types::list::List;
use crate::v2::lu_dog_async::types::parameter::Parameter;
use crate::v2::lu_dog_async::types::range::RANGE;
use crate::v2::lu_dog_async::types::span::Span;
use crate::v2::lu_dog_async::types::struct_generic::StructGeneric;
use crate::v2::lu_dog_async::types::task::TASK;
use crate::v2::lu_dog_async::types::tuple_field::TupleField;
use crate::v2::lu_dog_async::types::type_cast::TypeCast;
use crate::v2::lu_dog_async::types::unknown::UNKNOWN;
use crate::v2::lu_dog_async::types::woog_struct::WoogStruct;
use crate::v2::lu_dog_async::types::x_future::XFuture;
use crate::v2::lu_dog_async::types::x_plugin::XPlugin;
use crate::v2::lu_dog_async::types::x_value::XValue;
use crate::v2::lu_dog_async::types::z_object_store::ZObjectStore;
use crate::v2::sarzak::types::ty::Ty;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
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
    pub bogus: bool,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ValueTypeEnum {
    Char(Uuid),
    Empty(Uuid),
    EnumGeneric(usize),
    Enumeration(usize),
    FuncGeneric(usize),
    Function(usize),
    XFuture(usize),
    Import(usize),
    Lambda(usize),
    List(usize),
    ZObjectStore(usize),
    XPlugin(usize),
    Range(Uuid),
    WoogStruct(usize),
    StructGeneric(usize),
    Task(Uuid),
    Ty(Uuid),
    Unknown(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-implementation"}}}
impl ValueType {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_char"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_char(bogus: bool, store: &mut LuDogAsyncStore) -> Arc<RwLock<ValueType>> {
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    bogus: bogus,
                    subtype: ValueTypeEnum::Char(CHAR),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_empty"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_empty(bogus: bool, store: &mut LuDogAsyncStore) -> Arc<RwLock<ValueType>> {
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    bogus: bogus,
                    subtype: ValueTypeEnum::Empty(EMPTY),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_enumeration"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_enum_generic"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_enum_generic(
        bogus: bool,
        subtype: &Arc<RwLock<EnumGeneric>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ValueType>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    bogus: bogus,
                    subtype: ValueTypeEnum::EnumGeneric(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_x_error"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_enumeration"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_enumeration(
        bogus: bool,
        subtype: &Arc<RwLock<Enumeration>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ValueType>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    bogus: bogus,
                    subtype: ValueTypeEnum::Enumeration(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_func_generic"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_func_generic(
        bogus: bool,
        subtype: &Arc<RwLock<FuncGeneric>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ValueType>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    bogus: bogus,
                    subtype: ValueTypeEnum::FuncGeneric(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_function"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_function(
        bogus: bool,
        subtype: &Arc<RwLock<Function>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ValueType>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    bogus: bogus,
                    subtype: ValueTypeEnum::Function(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_generic"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_x_future"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_x_future(
        bogus: bool,
        subtype: &Arc<RwLock<XFuture>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ValueType>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    bogus: bogus,
                    subtype: ValueTypeEnum::XFuture(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_import"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_import(
        bogus: bool,
        subtype: &Arc<RwLock<Import>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ValueType>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    bogus: bogus,
                    subtype: ValueTypeEnum::Import(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_lambda"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_lambda(
        bogus: bool,
        subtype: &Arc<RwLock<Lambda>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ValueType>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    bogus: bogus,
                    subtype: ValueTypeEnum::Lambda(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_list"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_list(
        bogus: bool,
        subtype: &Arc<RwLock<List>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ValueType>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    bogus: bogus,
                    subtype: ValueTypeEnum::List(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_z_object_store"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_z_object_store(
        bogus: bool,
        subtype: &Arc<RwLock<ZObjectStore>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ValueType>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    bogus: bogus,
                    subtype: ValueTypeEnum::ZObjectStore(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_woog_option"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_x_plugin"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_x_plugin(
        bogus: bool,
        subtype: &Arc<RwLock<XPlugin>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ValueType>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    bogus: bogus,
                    subtype: ValueTypeEnum::XPlugin(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_range"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_range(bogus: bool, store: &mut LuDogAsyncStore) -> Arc<RwLock<ValueType>> {
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    bogus: bogus,
                    subtype: ValueTypeEnum::Range(RANGE),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_reference"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_woog_struct"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_woog_struct(
        bogus: bool,
        subtype: &Arc<RwLock<WoogStruct>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ValueType>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    bogus: bogus,
                    subtype: ValueTypeEnum::WoogStruct(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_woog_struct"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_struct_generic"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_struct_generic(
        bogus: bool,
        subtype: &Arc<RwLock<StructGeneric>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ValueType>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    bogus: bogus,
                    subtype: ValueTypeEnum::StructGeneric(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_task"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_task(bogus: bool, store: &mut LuDogAsyncStore) -> Arc<RwLock<ValueType>> {
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    bogus: bogus,
                    subtype: ValueTypeEnum::Task(TASK),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_ty"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_ty(
        bogus: bool,
        subtype: &std::sync::Arc<std::sync::RwLock<Ty>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ValueType>> {
        let s_id = subtype.read().unwrap().id();
        let subtype = subtype.read().unwrap().id();
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    bogus: bogus,
                    subtype: ValueTypeEnum::Ty(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_unknown"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_unknown(bogus: bool, store: &mut LuDogAsyncStore) -> Arc<RwLock<ValueType>> {
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    bogus: bogus,
                    subtype: ValueTypeEnum::Unknown(UNKNOWN),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-field"}}}
    /// Navigate to [`Field`] across R5(1-M)
    pub async fn r5_field<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Field>>> + '_ {
        store.iter_field().await.filter_map(|field| async {
            if field.read().await.ty == self.id {
                Some(field)
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-function"}}}
    /// Navigate to [`Function`] across R10(1-M)
    pub async fn r10_function<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Function>>> + '_ {
        store.iter_function().await.filter_map(|function| async {
            if function.read().await.return_type == self.id {
                Some(function)
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-x_future"}}}
    /// Navigate to [`XFuture`] across R2(1-M)
    pub async fn r2_x_future<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<XFuture>>> + '_ {
        store.iter_x_future().await.filter_map(|x_future| async {
            if x_future.read().await.x_value == self.id {
                Some(x_future)
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-lambda"}}}
    /// Navigate to [`Lambda`] across R74(1-M)
    pub async fn r74_lambda<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Lambda>>> + '_ {
        store.iter_lambda().await.filter_map(|lambda| async {
            if lambda.read().await.return_type == self.id {
                Some(lambda)
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_Mc-to-lambda_parameter"}}}
    /// Navigate to [`LambdaParameter`] across R77(1-Mc)
    pub async fn r77_lambda_parameter<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<LambdaParameter>>> + '_ {
        store
            .iter_lambda_parameter()
            .await
            .filter_map(move |lambda_parameter| async move {
                if lambda_parameter.read().await.ty == Some(self.id) {
                    Some(lambda_parameter.clone())
                } else {
                    None
                }
            })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-list"}}}
    /// Navigate to [`List`] across R36(1-M)
    pub async fn r36_list<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<List>>> + '_ {
        store.iter_list().await.filter_map(|list| async {
            if list.read().await.ty == self.id {
                Some(list)
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-woog_option"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-parameter"}}}
    /// Navigate to [`Parameter`] across R79(1-M)
    pub async fn r79_parameter<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Parameter>>> + '_ {
        store.iter_parameter().await.filter_map(|parameter| async {
            if parameter.read().await.ty == self.id {
                Some(parameter)
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-reference"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_Mc-to-span"}}}
    /// Navigate to [`Span`] across R62(1-Mc)
    pub async fn r62_span<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Span>>> + '_ {
        store.iter_span().await.filter_map(move |span| async move {
            if span.read().await.ty == Some(self.id) {
                Some(span.clone())
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-tuple_field"}}}
    /// Navigate to [`TupleField`] across R86(1-M)
    pub async fn r86_tuple_field<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<TupleField>>> + '_ {
        store
            .iter_tuple_field()
            .await
            .filter_map(|tuple_field| async {
                if tuple_field.read().await.ty == self.id {
                    Some(tuple_field)
                } else {
                    None
                }
            })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-type_cast"}}}
    /// Navigate to [`TypeCast`] across R69(1-M)
    pub async fn r69_type_cast<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<TypeCast>>> + '_ {
        store.iter_type_cast().await.filter_map(|type_cast| async {
            if type_cast.read().await.ty == self.id {
                Some(type_cast)
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-x_value"}}}
    /// Navigate to [`XValue`] across R24(1-M)
    pub async fn r24_x_value<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<XValue>>> + '_ {
        store.iter_x_value().await.filter_map(|x_value| async {
            if x_value.read().await.ty == self.id {
                Some(x_value)
            } else {
                None
            }
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-implementation"}}}
impl PartialEq for ValueType {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype && self.bogus == other.bogus
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
