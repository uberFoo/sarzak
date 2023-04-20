// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"struct_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog::types::expression::Expression;
use crate::v2::lu_dog::types::field_expression::FieldExpression;
use crate::v2::lu_dog::types::woog_struct::WoogStruct;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-struct-documentation"}}}
/// A Structure Expression
///
/// This is how we create instances in dwarf.
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StructExpression {
    pub bug: Uuid,
    pub id: Uuid,
    /// R39: [`StructExpression`] '' [`WoogStruct`]
    pub woog_struct: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-implementation"}}}
impl StructExpression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-struct-impl-new"}}}
    /// Inter a new 'Struct Expression' in the store, and return it's `id`.
    pub fn new(bug: Uuid, woog_struct: &WoogStruct, store: &mut LuDogStore) -> StructExpression {
        let id = Uuid::new_v4();
        let new = StructExpression {
            bug: bug,
            id: id,
            woog_struct: woog_struct.id,
        };
        store.inter_struct_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-struct-impl-new_"}}}
    /// Inter a new 'Struct Expression' in the store, and return it's `id`.
    pub fn new_(bug: Uuid, woog_struct: &WoogStruct) -> StructExpression {
        let id = Uuid::new_v4();
        let new = StructExpression {
            bug: bug,
            id: id,
            woog_struct: woog_struct.id,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-struct-impl-nav-forward-to-woog_struct"}}}
    /// Navigate to [`WoogStruct`] across R39(1-*)
    pub fn r39_woog_struct<'a>(&'a self, store: &'a LuDogStore) -> Vec<&WoogStruct> {
        vec![store.exhume_woog_struct(&self.woog_struct).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-struct-impl-nav-backward-1_M-to-field_expression"}}}
    /// Navigate to [`FieldExpression`] across R26(1-M)
    pub fn r26_field_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<&FieldExpression> {
        store
            .iter_field_expression()
            .filter_map(|field_expression| {
                if field_expression.woog_struct == self.id {
                    Some(field_expression)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Expression> {
        vec![store.exhume_expression(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
