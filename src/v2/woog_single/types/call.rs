// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"call-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-use-statements"}}}
use uuid::Uuid;

use crate::v2::woog_single::types::expression::Expression;
use crate::v2::woog_single::types::object_method::ObjectMethod;
use serde::{Deserialize, Serialize};

use crate::v2::woog_single::store::ObjectStore as WoogSingleStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-documentation"}}}
/// A Function Call
///
/// This is [formally](https://doc.rust-lang.org/reference/expressions/call-expr.html) a call
///  expression.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Call {
    pub id: Uuid,
    /// R19: [`Call`] 'invokes' [`ObjectMethod`]
    pub method: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-implementation"}}}
impl Call {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-new"}}}
    /// Inter a new 'Call' in the store, and return it's `id`.
    pub fn new(method: &ObjectMethod, store: &mut WoogSingleStore) -> Call {
        let id = Uuid::new_v4();
        let new = Call {
            id,
            method: method.id,
        };
        store.inter_call(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-struct-impl-nav-forward-to-method"}}}
    /// Navigate to [`ObjectMethod`] across R19(1-*)
    pub fn r19_object_method<'a>(&'a self, store: &'a WoogSingleStore) -> Vec<&ObjectMethod> {
        vec![store.exhume_object_method(&self.method).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"call-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R10(isa)
    pub fn r10_expression<'a>(&'a self, store: &'a WoogSingleStore) -> Vec<&Expression> {
        vec![store.exhume_expression(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
