// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"span-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"span-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec_tracy::types::dwarf_source_file::DwarfSourceFile;
use crate::v2::lu_dog_vec_tracy::types::value_type::ValueType;
use crate::v2::lu_dog_vec_tracy::types::x_value::XValue;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec_tracy::store::ObjectStore as LuDogVecTracyStore;
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
#[derive(Clone, Debug, Deserialize, Serialize)]
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
        source: &Rc<RefCell<DwarfSourceFile>>,
        ty: Option<&Rc<RefCell<ValueType>>>,
        x_value: Option<&Rc<RefCell<XValue>>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<Span>> {
        store.inter_span(|id| {
            Rc::new(RefCell::new(Span {
                end,
                id,
                start,
                source: source.borrow().id,
                ty: ty.map(|value_type| value_type.borrow().id),
                x_value: x_value.map(|x_value| x_value.borrow().id),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"span-struct-impl-nav-forward-to-source"}}}
    /// Navigate to [`DwarfSourceFile`] across R64(1-*)
    pub fn r64_dwarf_source_file<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<DwarfSourceFile>>> {
        span!("r64_dwarf_source_file");
        vec![store.exhume_dwarf_source_file(&self.source).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"span-struct-impl-nav-forward-cond-to-ty"}}}
    /// Navigate to [`ValueType`] across R62(1-*c)
    pub fn r62_value_type<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<ValueType>>> {
        span!("r62_value_type");
        match self.ty {
            Some(ref ty) => vec![store.exhume_value_type(&ty).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"span-struct-impl-nav-forward-cond-to-x_value"}}}
    /// Navigate to [`XValue`] across R63(1-*c)
    pub fn r63_x_value<'a>(&'a self, store: &'a LuDogVecTracyStore) -> Vec<Rc<RefCell<XValue>>> {
        span!("r63_x_value");
        match self.x_value {
            Some(ref x_value) => vec![store.exhume_x_value(&x_value).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"span-implementation"}}}
impl PartialEq for Span {
    fn eq(&self, other: &Self) -> bool {
        self.end == other.end
            && self.start == other.start
            && self.source == other.source
            && self.ty == other.ty
            && self.x_value == other.x_value
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
