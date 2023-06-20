// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"span-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"span-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::dwarf_source_file::DwarfSourceFile;
use crate::v2::lu_dog_async::types::value_type::ValueType;
use crate::v2::lu_dog_async::types::x_value::XValue;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"span-struct-documentation"}}}
/// A span is just a two-tuple that specifies the start and end locations, in the source code
/// , for an entitiy.
///
/// Looking at this, I think that this should be a supertype, and then a subtype for each relationship
/// . I’m feeling lazy.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"span-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Span {
    pub end: i64,
    pub id: Uuid,
    pub start: i64,
    /// R64: [`Span`] '' [`DwarfSourceFile`]
    pub source: Uuid,
    /// R63: [`Span`] '' [`XValue`]
    pub x_value: Option<Uuid>,
    /// R62: [`Span`] '' [`ValueType`]
    pub ty: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"span-implementation"}}}
impl Span {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"span-struct-impl-new"}}}
    /// Inter a new 'Span' in the store, and return it's `id`.
    pub async fn new(
        end: i64,
        start: i64,
        source: &Arc<RwLock<DwarfSourceFile>>,
        x_value: Option<&Arc<RwLock<XValue>>>,
        ty: Option<&Arc<RwLock<ValueType>>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Span>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Span {
            end,
            id,
            start,
            source: source.read().await.id,
            x_value: futures::future::OptionFuture::from(
                x_value.map(|x_value| async { x_value.read().await.id }),
            )
            .await,
            ty: futures::future::OptionFuture::from(
                ty.map(|value_type| async { value_type.read().await.id() }),
            )
            .await,
        }));
        store.inter_span(new.clone()).await;
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"span-struct-impl-nav-forward-to-source"}}}
    /// Navigate to [`DwarfSourceFile`] across R64(1-*)
    pub async fn r64_dwarf_source_file<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<DwarfSourceFile>>> {
        span!("r64_dwarf_source_file");
        vec![store.exhume_dwarf_source_file(&self.source).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"span-struct-impl-nav-forward-cond-to-x_value"}}}
    /// Navigate to [`XValue`] across R63(1-*c)
    pub async fn r63_x_value<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<XValue>>> {
        span!("r63_x_value");
        match self.x_value {
            Some(ref x_value) => vec![store.exhume_x_value(x_value).await.unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"span-struct-impl-nav-forward-cond-to-ty"}}}
    /// Navigate to [`ValueType`] across R62(1-*c)
    pub async fn r62_value_type<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<ValueType>>> {
        span!("r62_value_type");
        match self.ty {
            Some(ref ty) => vec![store.exhume_value_type(ty).await.unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
