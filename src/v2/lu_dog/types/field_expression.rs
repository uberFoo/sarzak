// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"field_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog::types::struct_expression::StructExpression;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-struct-documentation"}}}
/// A Struct Field Expression
///
/// This assigns a value to a field in a structure.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FieldExpression {
    pub id: Uuid,
    pub name: String,
    /// R25: [`FieldExpression`] 'follows' [`FieldExpression`]
    pub next: Option<Uuid>,
    /// R26: [`FieldExpression`] 'belongs to a' [`StructExpression`]
    pub woog_struct: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-implementation"}}}
impl FieldExpression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-struct-impl-new"}}}
    /// Inter a new 'Field Expression' in the store, and return it's `id`.
    pub fn new(
        name: String,
        next: Option<&FieldExpression>,
        woog_struct: &StructExpression,
        store: &mut LuDogStore,
    ) -> FieldExpression {
        let id = Uuid::new_v4();
        let new = FieldExpression {
            id: id,
            name: name,
            next: next.map(|field_expression| field_expression.id),
            woog_struct: woog_struct.id,
        };
        store.inter_field_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-struct-impl-new_"}}}
    /// Inter a new 'Field Expression' in the store, and return it's `id`.
    pub fn new_(
        name: String,
        next: Option<&FieldExpression>,
        woog_struct: &StructExpression,
    ) -> FieldExpression {
        let id = Uuid::new_v4();
        let new = FieldExpression {
            id: id,
            name: name,
            next: next.map(|field_expression| field_expression.id),
            woog_struct: woog_struct.id,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`FieldExpression`] across R25(1-*c)
    pub fn r25_field_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<&FieldExpression> {
        match self.next {
            Some(ref next) => vec![store.exhume_field_expression(next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-struct-impl-nav-forward-to-woog_struct"}}}
    /// Navigate to [`StructExpression`] across R26(1-*)
    pub fn r26_struct_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<&StructExpression> {
        vec![store.exhume_struct_expression(&self.woog_struct).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-struct-impl-nav-backward-one-bi-cond-to-field_expression"}}}
    /// Navigate to [`FieldExpression`] across R25(1c-1c)
    pub fn r25c_field_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<&FieldExpression> {
        let field_expression = store
            .iter_field_expression()
            .find(|field_expression| field_expression.next == Some(self.id));
        match field_expression {
            Some(ref field_expression) => vec![field_expression],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
