// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"attribute-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"attribute-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::sarzak_v2::UUID_NS;

// Referrer imports
use crate::sarzak_v2::types::object::Object;
use crate::sarzak_v2::types::ty::Ty;

use crate::sarzak_v2::store::ObjectStore as SarzakV2Store;
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
    /// Inter a new Attribute in the store, and return it's `id`.
    pub fn new(
        name: String,
        obj_id: Option<&Object>,
        ty: &Ty,
        store: &mut SarzakV2Store,
    ) -> Attribute {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{}:{:?}:{:?}", name, obj_id, ty).as_bytes(),
        );
        let new = Attribute {
            name: name,
            obj_id: obj_id.map(|object| object.id),
            ty: ty.id(),
            id,
        };
        store.inter_attribute(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"attribute-struct-impl-nav-forward-cond-to-obj_id"}}}
    /// Navigate to [`Object`] across R1(1-?c)
    pub fn r1_object<'a>(&'a self, store: &'a SarzakV2Store) -> Vec<&Object> {
        match self.obj_id {
            Some(ref obj_id) => vec![store.exhume_object(obj_id).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"attribute-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`Ty`] across R2(1-?)
    pub fn r2_ty<'a>(&'a self, store: &'a SarzakV2Store) -> Vec<&Ty> {
        vec![store.exhume_ty(&self.ty).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
