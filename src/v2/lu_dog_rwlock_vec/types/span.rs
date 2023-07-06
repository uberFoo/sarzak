// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"span-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"span-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock_vec::types::dwarf_source_file::DwarfSourceFile;
use crate::v2::lu_dog_rwlock_vec::types::value_type::ValueType;
use crate::v2::lu_dog_rwlock_vec::types::x_value::XValue;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock_vec::store::ObjectStore as LuDogRwlockVecStore;
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
    pub id: usize,
    pub start: i64,
    /// R64: [`Span`] '' [`DwarfSourceFile`]
    pub source: usize,
    /// R62: [`Span`] '' [`ValueType`]
    pub ty: Option<usize>,
    /// R63: [`Span`] '' [`XValue`]
    pub x_value: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"span-implementation"}}}
impl Span {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"span-struct-impl-new"}}}
    /// Inter a new 'Span' in the store, and return it's `id`.
    pub fn new(
        end: i64,
        start: i64,
        source: &Arc<RwLock<DwarfSourceFile>>,
        ty: Option<&Arc<RwLock<ValueType>>>,
        x_value: Option<&Arc<RwLock<XValue>>>,
        store: &mut LuDogRwlockVecStore,
    ) -> Arc<RwLock<Span>> {
        store.inter_span(|id| {
            Arc::new(RwLock::new(Span {
                end,
                id,
                start,
                source: source.read().unwrap().id,
                ty: ty.map(|value_type| value_type.read().unwrap().id),
                x_value: x_value.map(|x_value| x_value.read().unwrap().id),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"span-struct-impl-nav-forward-to-source"}}}
    /// Navigate to [`DwarfSourceFile`] across R64(1-*)
    pub fn r64_dwarf_source_file<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<DwarfSourceFile>>> {
        span!("r64_dwarf_source_file");
        vec![store.exhume_dwarf_source_file(&self.source).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"span-struct-impl-nav-forward-cond-to-ty"}}}
    /// Navigate to [`ValueType`] across R62(1-*c)
    pub fn r62_value_type<'a>(
        &'a self,
        store: &'a LuDogRwlockVecStore,
    ) -> Vec<Arc<RwLock<ValueType>>> {
        span!("r62_value_type");
        match self.ty {
            Some(ref ty) => vec![store.exhume_value_type(&ty).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"span-struct-impl-nav-forward-cond-to-x_value"}}}
    /// Navigate to [`XValue`] across R63(1-*c)
    pub fn r63_x_value<'a>(&'a self, store: &'a LuDogRwlockVecStore) -> Vec<Arc<RwLock<XValue>>> {
        span!("r63_x_value");
        match self.x_value {
            Some(ref x_value) => vec![store.exhume_x_value(&x_value).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
