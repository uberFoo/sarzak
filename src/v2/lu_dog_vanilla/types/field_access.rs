// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"field_access-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog_vanilla::types::expression::Expression;
use crate::v2::lu_dog_vanilla::types::expression::ExpressionEnum;
use crate::v2::lu_dog_vanilla::types::field_access_target::FieldAccessTarget;
use crate::v2::lu_dog_vanilla::types::woog_struct::WoogStruct;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
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
        expression: &Expression,
        field: &FieldAccessTarget,
        woog_struct: &WoogStruct,
        store: &mut LuDogVanillaStore,
    ) -> FieldAccess {
        let id = Uuid::new_v4();
        let new = FieldAccess {
            id,
            expression: expression.id,
            field: field.id,
            woog_struct: woog_struct.id,
        };
        store.inter_field_access(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R27(1-*)
    pub fn r27_expression<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Expression> {
        vec![store.exhume_expression(&self.expression).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-struct-impl-nav-forward-to-field"}}}
    /// Navigate to [`FieldAccessTarget`] across R65(1-*)
    pub fn r65_field_access_target<'a>(
        &'a self,
        store: &'a LuDogVanillaStore,
    ) -> Vec<&FieldAccessTarget> {
        vec![store.exhume_field_access_target(&self.field).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-struct-impl-nav-forward-to-woog_struct"}}}
    /// Navigate to [`WoogStruct`] across R66(1-*)
    pub fn r66_woog_struct<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&WoogStruct> {
        vec![store.exhume_woog_struct(&self.woog_struct).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Expression> {
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::FieldAccess(id) = expression.subtype {
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
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
