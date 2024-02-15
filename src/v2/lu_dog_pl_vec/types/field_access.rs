// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"field_access-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-use-statements"}}}
use parking_lot::RwLock;
use std::sync::Arc;
use uuid::Uuid;

use crate::v2::lu_dog_pl_vec::types::expression::Expression;
use crate::v2::lu_dog_pl_vec::types::expression::ExpressionEnum;
use crate::v2::lu_dog_pl_vec::types::field_access_target::FieldAccessTarget;
use crate::v2::lu_dog_pl_vec::types::woog_struct::WoogStruct;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_pl_vec::store::ObjectStore as LuDogPlVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-struct-documentation"}}}
/// A Struct Field Access
///
/// Think dotted notation.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FieldAccess {
    pub id: usize,
    /// R27: [`FieldAccess`] 'contains an' [`Expression`]
    pub expression: usize,
    /// R65: [`FieldAccess`] '' [`FieldAccessTarget`]
    pub field: usize,
    /// R66: [`FieldAccess`] '' [`WoogStruct`]
    pub woog_struct: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-implementation"}}}
impl FieldAccess {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-struct-impl-new"}}}
    /// Inter a new 'Field Access' in the store, and return it's `id`.
    pub fn new(
        expression: &Arc<RwLock<Expression>>,
        field: &Arc<RwLock<FieldAccessTarget>>,
        woog_struct: &Arc<RwLock<WoogStruct>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<FieldAccess>> {
        store.inter_field_access(|id| {
            Arc::new(RwLock::new(FieldAccess {
                id,
                expression: expression.read().id,
                field: field.read().id,
                woog_struct: woog_struct.read().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R27(1-*)
    pub fn r27_expression<'a>(
        &'a self,
        store: &'a LuDogPlVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store.exhume_expression(&self.expression).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-struct-impl-nav-forward-to-field"}}}
    /// Navigate to [`FieldAccessTarget`] across R65(1-*)
    pub fn r65_field_access_target<'a>(
        &'a self,
        store: &'a LuDogPlVecStore,
    ) -> Vec<Arc<RwLock<FieldAccessTarget>>> {
        vec![store.exhume_field_access_target(&self.field).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-struct-impl-nav-forward-to-woog_struct"}}}
    /// Navigate to [`WoogStruct`] across R66(1-*)
    pub fn r66_woog_struct<'a>(
        &'a self,
        store: &'a LuDogPlVecStore,
    ) -> Vec<Arc<RwLock<WoogStruct>>> {
        vec![store.exhume_woog_struct(&self.woog_struct).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogPlVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::FieldAccess(id) = expression.read().subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-implementation"}}}
impl PartialEq for FieldAccess {
    fn eq(&self, other: &Self) -> bool {
        self.expression == other.expression
            && self.field == other.field
            && self.woog_struct == other.woog_struct
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
