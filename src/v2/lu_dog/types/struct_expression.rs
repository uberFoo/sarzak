// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"struct_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
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
///
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
    pub fn new(
        bug: Uuid,
        woog_struct: &Rc<RefCell<WoogStruct>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<StructExpression>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(StructExpression {
            bug,
            id,
            woog_struct: woog_struct.borrow().id,
        }));
        store.inter_struct_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-struct-impl-nav-forward-to-woog_struct"}}}
    /// Navigate to [`WoogStruct`] across R39(1-*)
    pub fn r39_woog_struct<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<WoogStruct>>> {
        span!("r39_woog_struct");
        vec![store.exhume_woog_struct(&self.woog_struct).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-struct-impl-nav-backward-1_M-to-field_expression"}}}
    /// Navigate to [`FieldExpression`] across R26(1-M)
    pub fn r26_field_expression<'a>(
        &'a self,
        store: &'a LuDogStore,
    ) -> Vec<Rc<RefCell<FieldExpression>>> {
        span!("r26_field_expression");
        store
            .iter_field_expression()
            .filter(|field_expression| field_expression.borrow().woog_struct == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Expression>>> {
        span!("r15_expression");
        vec![store.exhume_expression(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
