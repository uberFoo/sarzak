// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"field_access-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-use-statements"}}}
use parking_lot::Mutex;
use std::sync::Arc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_pl_mutex::types::expression::Expression;
use crate::v2::lu_dog_pl_mutex::types::field_access_target::FieldAccessTarget;
use crate::v2::lu_dog_pl_mutex::types::woog_struct::WoogStruct;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_pl_mutex::store::ObjectStore as LuDogPlMutexStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-struct-documentation"}}}
/// A Struct Field Access
///
/// Think dotted notation.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FieldAccess {
    pub id: Uuid,
    /// R27: [`FieldAccess`] 'contains an' [`Expression`]
    pub expression: Uuid,
    /// R65: [`FieldAccess`] '' [`FieldAccessTarget`]
    pub field: Uuid,
    /// R66: [`FieldAccess`] '' [`WoogStruct`]
    pub woog_struct: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-implementation"}}}
impl FieldAccess {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-struct-impl-new"}}}
    /// Inter a new 'Field Access' in the store, and return it's `id`.
    pub fn new(
        expression: &Arc<Mutex<Expression>>,
        field: &Arc<Mutex<FieldAccessTarget>>,
        woog_struct: &Arc<Mutex<WoogStruct>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<FieldAccess>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(FieldAccess {
            id,
            expression: expression.lock().id(),
            field: field.lock().id(),
            woog_struct: woog_struct.lock().id,
        }));
        store.inter_field_access(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R27(1-*)
    pub fn r27_expression<'a>(
        &'a self,
        store: &'a LuDogPlMutexStore,
    ) -> Vec<Arc<Mutex<Expression>>> {
        span!("r27_expression");
        vec![store.exhume_expression(&self.expression).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-struct-impl-nav-forward-to-field"}}}
    /// Navigate to [`FieldAccessTarget`] across R65(1-*)
    pub fn r65_field_access_target<'a>(
        &'a self,
        store: &'a LuDogPlMutexStore,
    ) -> Vec<Arc<Mutex<FieldAccessTarget>>> {
        span!("r65_field_access_target");
        vec![store.exhume_field_access_target(&self.field).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-struct-impl-nav-forward-to-woog_struct"}}}
    /// Navigate to [`WoogStruct`] across R66(1-*)
    pub fn r66_woog_struct<'a>(
        &'a self,
        store: &'a LuDogPlMutexStore,
    ) -> Vec<Arc<Mutex<WoogStruct>>> {
        span!("r66_woog_struct");
        vec![store.exhume_woog_struct(&self.woog_struct).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogPlMutexStore,
    ) -> Vec<Arc<Mutex<Expression>>> {
        span!("r15_expression");
        vec![store.exhume_expression(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
