// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"value_type-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::char::CHAR;
use crate::v2::lu_dog_async::types::empty::EMPTY;
use crate::v2::lu_dog_async::types::enumeration::Enumeration;
use crate::v2::lu_dog_async::types::field::Field;
use crate::v2::lu_dog_async::types::function::Function;
use crate::v2::lu_dog_async::types::generic::Generic;
use crate::v2::lu_dog_async::types::import::Import;
use crate::v2::lu_dog_async::types::lambda::Lambda;
use crate::v2::lu_dog_async::types::lambda_parameter::LambdaParameter;
use crate::v2::lu_dog_async::types::list::List;
use crate::v2::lu_dog_async::types::parameter::Parameter;
use crate::v2::lu_dog_async::types::range::RANGE;
use crate::v2::lu_dog_async::types::reference::Reference;
use crate::v2::lu_dog_async::types::span::Span;
use crate::v2::lu_dog_async::types::tuple_field::TupleField;
use crate::v2::lu_dog_async::types::type_cast::TypeCast;
use crate::v2::lu_dog_async::types::unknown::UNKNOWN;
use crate::v2::lu_dog_async::types::woog_option::WoogOption;
use crate::v2::lu_dog_async::types::woog_struct::WoogStruct;
use crate::v2::lu_dog_async::types::x_error::XError;
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
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ValueTypeEnum {
    Char(Uuid),
    Empty(Uuid),
    Enumeration(usize),
    XError(usize),
    Function(usize),
    Generic(usize),
    Import(usize),
    Lambda(usize),
    List(usize),
    ZObjectStore(usize),
    WoogOption(usize),
    Range(Uuid),
    Reference(usize),
    WoogStruct(usize),
    Ty(Uuid),
    Unknown(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-implementation"}}}
impl ValueType {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_char"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_char(store: &mut LuDogAsyncStore) -> Arc<RwLock<ValueType>> {
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    subtype: ValueTypeEnum::Char(CHAR),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_empty"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_empty(store: &mut LuDogAsyncStore) -> Arc<RwLock<ValueType>> {
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    subtype: ValueTypeEnum::Empty(EMPTY),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_enumeration"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_enumeration(
        subtype: &Arc<RwLock<Enumeration>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ValueType>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    subtype: ValueTypeEnum::Enumeration(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_x_error"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_x_error(
        subtype: &Arc<RwLock<XError>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ValueType>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    subtype: ValueTypeEnum::XError(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_function"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_function(
        subtype: &Arc<RwLock<Function>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ValueType>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    subtype: ValueTypeEnum::Function(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_generic"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_generic(
        subtype: &Arc<RwLock<Generic>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ValueType>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    subtype: ValueTypeEnum::Generic(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_import"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_import(
        subtype: &Arc<RwLock<Import>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ValueType>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
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
        subtype: &Arc<RwLock<Lambda>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ValueType>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
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
        subtype: &Arc<RwLock<List>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ValueType>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
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
        subtype: &Arc<RwLock<ZObjectStore>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ValueType>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    subtype: ValueTypeEnum::ZObjectStore(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_woog_option"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_woog_option(
        subtype: &Arc<RwLock<WoogOption>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ValueType>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    subtype: ValueTypeEnum::WoogOption(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_range"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_range(store: &mut LuDogAsyncStore) -> Arc<RwLock<ValueType>> {
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    subtype: ValueTypeEnum::Range(RANGE),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_reference"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_reference(
        subtype: &Arc<RwLock<Reference>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ValueType>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    subtype: ValueTypeEnum::Reference(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_woog_struct"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_woog_struct(
        subtype: &Arc<RwLock<WoogStruct>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<ValueType>> {
        let s_id = subtype.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    subtype: ValueTypeEnum::WoogStruct(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_ty"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_ty(subtype: &Ty, store: &mut LuDogAsyncStore) -> Arc<RwLock<ValueType>> {
        let s_id = subtype.id();
        let subtype = subtype.id();
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    subtype: ValueTypeEnum::Ty(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-new_unknown"}}}
    /// Inter a new ValueType in the store, and return it's `id`.
    pub async fn new_unknown(store: &mut LuDogAsyncStore) -> Arc<RwLock<ValueType>> {
        store
            .inter_value_type(|id| {
                Arc::new(RwLock::new(ValueType {
                    subtype: ValueTypeEnum::Unknown(UNKNOWN),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-field"}}}
    /// Navigate to [`Field`] across R5(1-M)
    pub async fn r5_field<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Field>>> {
        span!("r5_field");
        store
            .iter_field()
            .await
            .filter_map(|field| async {
                if field.read().await.ty == self.id {
                    Some(field)
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-function"}}}
    /// Navigate to [`Function`] across R10(1-M)
    pub async fn r10_function<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Function>>> {
        span!("r10_function");
        store
            .iter_function()
            .await
            .filter_map(|function| async {
                if function.read().await.return_type == self.id {
                    Some(function)
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-lambda"}}}
    /// Navigate to [`Lambda`] across R74(1-M)
    pub async fn r74_lambda<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Lambda>>> {
        span!("r74_lambda");
        store
            .iter_lambda()
            .await
            .filter_map(|lambda| async {
                if lambda.read().await.return_type == self.id {
                    Some(lambda)
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_Mc-to-lambda_parameter"}}}
    /// Navigate to [`LambdaParameter`] across R77(1-Mc)
    pub async fn r77_lambda_parameter<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<LambdaParameter>>> {
        span!("r77_lambda_parameter");
        store
            .iter_lambda_parameter()
            .await
            .filter_map(|lambda_parameter| async move {
                if lambda_parameter.read().await.ty == Some(self.id) {
                    Some(lambda_parameter.clone())
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-list"}}}
    /// Navigate to [`List`] across R36(1-M)
    pub async fn r36_list<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<List>>> {
        span!("r36_list");
        store
            .iter_list()
            .await
            .filter_map(|list| async {
                if list.read().await.ty == self.id {
                    Some(list)
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-woog_option"}}}
    /// Navigate to [`WoogOption`] across R2(1-M)
    pub async fn r2_woog_option<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<WoogOption>>> {
        span!("r2_woog_option");
        store
            .iter_woog_option()
            .await
            .filter_map(|woog_option| async {
                if woog_option.read().await.ty == self.id {
                    Some(woog_option)
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-parameter"}}}
    /// Navigate to [`Parameter`] across R79(1-M)
    pub async fn r79_parameter<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Parameter>>> {
        span!("r79_parameter");
        store
            .iter_parameter()
            .await
            .filter_map(|parameter| async {
                if parameter.read().await.ty == self.id {
                    Some(parameter)
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-reference"}}}
    /// Navigate to [`Reference`] across R35(1-M)
    pub async fn r35_reference<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Reference>>> {
        span!("r35_reference");
        store
            .iter_reference()
            .await
            .filter_map(|reference| async {
                if reference.read().await.ty == self.id {
                    Some(reference)
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_Mc-to-span"}}}
    /// Navigate to [`Span`] across R62(1-Mc)
    pub async fn r62_span<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Span>>> {
        span!("r62_span");
        store
            .iter_span()
            .await
            .filter_map(|span| async move {
                if span.read().await.ty == Some(self.id) {
                    Some(span.clone())
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-tuple_field"}}}
    /// Navigate to [`TupleField`] across R86(1-M)
    pub async fn r86_tuple_field<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<TupleField>>> {
        span!("r86_tuple_field");
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
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-type_cast"}}}
    /// Navigate to [`TypeCast`] across R69(1-M)
    pub async fn r69_type_cast<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<TypeCast>>> {
        span!("r69_type_cast");
        store
            .iter_type_cast()
            .await
            .filter_map(|type_cast| async {
                if type_cast.read().await.ty == self.id {
                    Some(type_cast)
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-x_value"}}}
    /// Navigate to [`XValue`] across R24(1-M)
    pub async fn r24_x_value<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<XValue>>> {
        span!("r24_x_value");
        store
            .iter_x_value()
            .await
            .filter_map(|x_value| async {
                if x_value.read().await.ty == self.id {
                    Some(x_value)
                } else {
                    None
                }
            })
            .collect()
            .await
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
