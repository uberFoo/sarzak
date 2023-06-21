// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"value_type-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-use-statements"}}}
use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
use crate::v2::lu_dog_async::types::empty::EMPTY;
use crate::v2::lu_dog_async::types::error::Error;
use crate::v2::lu_dog_async::types::field::Field;
use crate::v2::lu_dog_async::types::function::Function;
use crate::v2::lu_dog_async::types::import::Import;
use crate::v2::lu_dog_async::types::list::List;
use crate::v2::lu_dog_async::types::range::RANGE;
use crate::v2::lu_dog_async::types::reference::Reference;
use crate::v2::lu_dog_async::types::span::Span;
use crate::v2::lu_dog_async::types::type_cast::TypeCast;
use crate::v2::lu_dog_async::types::unknown::UNKNOWN;
use crate::v2::lu_dog_async::types::woog_option::WoogOption;
use crate::v2::lu_dog_async::types::woog_struct::WoogStruct;
use crate::v2::lu_dog_async::types::x_value::XValue;
use crate::v2::lu_dog_async::types::z_object_store::ZObjectStore;
use crate::v2::sarzak::types::ty::Ty;
use async_std::sync::Arc;
use async_std::sync::RwLock;
use serde::{Deserialize, Serialize};
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
    pub async fn new_empty(store: &LuDogAsyncStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_value_type(&EMPTY).await.unwrap()
    }

    /// Create a new instance of ValueType::Error
    pub async fn new_error(
        error: &Arc<RwLock<Error>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = error.read().await.id();
        if let Some(error) = store.exhume_value_type(&id).await {
            error
        } else {
            let new = Arc::new(RwLock::new(Self::Error(id)));
            store.inter_value_type(new.clone()).await;
            new
        }
    }

    /// Create a new instance of ValueType::Function
    pub async fn new_function(
        function: &Arc<RwLock<Function>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = function.read().await.id;
        if let Some(function) = store.exhume_value_type(&id).await {
            function
        } else {
            let new = Arc::new(RwLock::new(Self::Function(id)));
            store.inter_value_type(new.clone()).await;
            new
        }
    }

    /// Create a new instance of ValueType::Import
    pub async fn new_import(
        import: &Arc<RwLock<Import>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = import.read().await.id;
        if let Some(import) = store.exhume_value_type(&id).await {
            import
        } else {
            let new = Arc::new(RwLock::new(Self::Import(id)));
            store.inter_value_type(new.clone()).await;
            new
        }
    }

    /// Create a new instance of ValueType::List
    pub async fn new_list(
        list: &Arc<RwLock<List>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = list.read().await.id;
        if let Some(list) = store.exhume_value_type(&id).await {
            list
        } else {
            let new = Arc::new(RwLock::new(Self::List(id)));
            store.inter_value_type(new.clone()).await;
            new
        }
    }

    /// Create a new instance of ValueType::ZObjectStore
    pub async fn new_z_object_store(
        z_object_store: &Arc<RwLock<ZObjectStore>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = z_object_store.read().await.id;
        if let Some(z_object_store) = store.exhume_value_type(&id).await {
            z_object_store
        } else {
            let new = Arc::new(RwLock::new(Self::ZObjectStore(id)));
            store.inter_value_type(new.clone()).await;
            new
        }
    }

    /// Create a new instance of ValueType::WoogOption
    pub async fn new_woog_option(
        woog_option: &Arc<RwLock<WoogOption>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = woog_option.read().await.id;
        if let Some(woog_option) = store.exhume_value_type(&id).await {
            woog_option
        } else {
            let new = Arc::new(RwLock::new(Self::WoogOption(id)));
            store.inter_value_type(new.clone()).await;
            new
        }
    }

    /// Create a new instance of ValueType::Range
    pub async fn new_range(store: &LuDogAsyncStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_value_type(&RANGE).await.unwrap()
    }

    /// Create a new instance of ValueType::Reference
    pub async fn new_reference(
        reference: &Arc<RwLock<Reference>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = reference.read().await.id;
        if let Some(reference) = store.exhume_value_type(&id).await {
            reference
        } else {
            let new = Arc::new(RwLock::new(Self::Reference(id)));
            store.inter_value_type(new.clone()).await;
            new
        }
    }

    /// Create a new instance of ValueType::WoogStruct
    pub async fn new_woog_struct(
        woog_struct: &Arc<RwLock<WoogStruct>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Self>> {
        let id = woog_struct.read().await.id;
        if let Some(woog_struct) = store.exhume_value_type(&id).await {
            woog_struct
        } else {
            let new = Arc::new(RwLock::new(Self::WoogStruct(id)));
            store.inter_value_type(new.clone()).await;
            new
        }
    }

    /// Create a new instance of ValueType::Ty
    pub async fn new_ty(ty: &Arc<RwLock<Ty>>, store: &mut LuDogAsyncStore) -> Arc<RwLock<Self>> {
        let id = ty.read().await.id();
        if let Some(ty) = store.exhume_value_type(&id).await {
            ty
        } else {
            let new = Arc::new(RwLock::new(Self::Ty(id)));
            store.inter_value_type(new.clone()).await;
            new
        }
    }

    /// Create a new instance of ValueType::Unknown
    pub async fn new_unknown(store: &LuDogAsyncStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_value_type(&UNKNOWN).await.unwrap()
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
    pub async fn r5_field<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Field>>> {
        span!("r5_field");
        let mut result = Vec::new();
        for field in store.iter_field().await {
            if field.read().await.ty == self.id() {
                result.push(field)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-function"}}}
    /// Navigate to [`Function`] across R10(1-M)
    pub async fn r10_function<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Function>>> {
        span!("r10_function");
        let mut result = Vec::new();
        for function in store.iter_function().await {
            if function.read().await.return_type == self.id() {
                result.push(function)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-list"}}}
    /// Navigate to [`List`] across R36(1-M)
    pub async fn r36_list<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<List>>> {
        span!("r36_list");
        let mut result = Vec::new();
        for list in store.iter_list().await {
            if list.read().await.ty == self.id() {
                result.push(list)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-woog_option"}}}
    /// Navigate to [`WoogOption`] across R2(1-M)
    pub async fn r2_woog_option<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<WoogOption>>> {
        span!("r2_woog_option");
        let mut result = Vec::new();
        for woog_option in store.iter_woog_option().await {
            if woog_option.read().await.ty == self.id() {
                result.push(woog_option)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-reference"}}}
    /// Navigate to [`Reference`] across R35(1-M)
    pub async fn r35_reference<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Reference>>> {
        span!("r35_reference");
        let mut result = Vec::new();
        for reference in store.iter_reference().await {
            if reference.read().await.ty == self.id() {
                result.push(reference)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_Mc-to-span"}}}
    /// Navigate to [`Span`] across R62(1-Mc)
    pub async fn r62_span<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Span>>> {
        use futures::stream::{self, StreamExt};
        span!("r62_span");
        stream::iter(store.iter_span().await.collect::<Vec<Arc<RwLock<Span>>>>()).filter(
            |span: Arc<RwLock<Span>>| async move {
                span.read().await.ty == Some(self.id()).collect().await
            },
        )
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-type_cast"}}}
    /// Navigate to [`TypeCast`] across R69(1-M)
    pub async fn r69_type_cast<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<TypeCast>>> {
        span!("r69_type_cast");
        let mut result = Vec::new();
        for type_cast in store.iter_type_cast().await {
            if type_cast.read().await.ty == self.id() {
                result.push(type_cast)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value_type-struct-impl-nav-backward-1_M-to-x_value"}}}
    /// Navigate to [`XValue`] across R24(1-M)
    pub async fn r24_x_value<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<XValue>>> {
        span!("r24_x_value");
        let mut result = Vec::new();
        for x_value in store.iter_x_value().await {
            if x_value.read().await.ty == self.id() {
                result.push(x_value)
            }
        }
        result
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
