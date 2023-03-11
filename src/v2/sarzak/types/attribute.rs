// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"attribute-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"attribute-use-statements"}}}
use uuid::Uuid;

use crate::v2::sarzak::types::object::Object;
use crate::v2::sarzak::types::ty::Ty;
use crate::v2::sarzak::UUID_NS;
use serde::{Deserialize, Serialize};

use crate::v2::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"attribute-struct-documentation"}}}
/// An `Attribute` represents a single value. Each value must have a
/// [`Type`], which constrains the values of data that may be assigned to
/// an `Attribute`.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"attribute-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Attribute {
    pub id: Uuid,
    pub name: String,
    /// R1: [`Attribute`] 'lives in an' [`Object`]
    pub obj_id: Option<Uuid>,
    /// R2: [`Attribute`] 'has a' [`Ty`]
    pub ty: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"attribute-implementation"}}}
impl Attribute {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"attribute-struct-impl-new"}}}
    /// Inter a new 'Attribute' in the store, and return it's `id`.
    pub fn new(
        name: String,
        obj_id: Option<&Object>,
        ty: &Ty,
        store: &mut SarzakStore,
    ) -> Attribute {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{}:{:?}:{:?}", name, obj_id, ty).as_bytes(),
        );
        let new = Attribute {
            id: id,
            name: name,
            obj_id: obj_id.map(|object| object.id),
            ty: ty.id(),
        };
        store.inter_attribute(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"attribute-struct-impl-nav-forward-cond-to-obj_id"}}}
    /// Navigate to [`Object`] across R1(1-*c)
    pub fn r1_object<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Object> {
        match self.obj_id {
            Some(ref obj_id) => vec![store.exhume_object(obj_id).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"attribute-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`Ty`] across R2(1-*)
    pub fn r2_ty<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Ty> {
        vec![store.exhume_ty(&self.ty).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
