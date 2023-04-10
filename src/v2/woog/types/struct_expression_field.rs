// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"struct_expression_field-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression_field-use-statements"}}}
use uuid::Uuid;

use crate::v2::woog::types::expression::Expression;
use crate::v2::woog::types::struct_expression::StructExpression;
use serde::{Deserialize, Serialize};

use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression_field-struct-documentation"}}}
/// A Field in a Structure
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression_field-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StructExpressionField {
    pub id: Uuid,
    pub name: String,
    /// R28: [`StructExpressionField`] 'is initialized with' [`Expression`]
    pub expr: Uuid,
    /// R27: [`StructExpressionField`] 'defines a' [`StructExpression`]
    pub woog_struct: Uuid,
    /// R30: [`StructExpressionField`] 'came before' [`StructExpressionField`]
    pub next: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression_field-implementation"}}}
impl StructExpressionField {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression_field-struct-impl-new"}}}
    /// Inter a new 'Struct Expression Field' in the store, and return it's `id`.
    pub fn new(
        name: String,
        expr: &Expression,
        woog_struct: &StructExpression,
        next: Option<&StructExpressionField>,
        store: &mut WoogStore,
    ) -> StructExpressionField {
        let id = Uuid::new_v4();
        let new = StructExpressionField {
            id: id,
            name: name,
            expr: expr.id(),
            woog_struct: woog_struct.id,
            next: next.map(|struct_expression_field| struct_expression_field.id),
        };
        store.inter_struct_expression_field(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression_field-struct-impl-new_"}}}
    /// Inter a new 'Struct Expression Field' in the store, and return it's `id`.
    pub fn new_(
        name: String,
        expr: &Expression,
        woog_struct: &StructExpression,
        next: Option<&StructExpressionField>,
    ) -> StructExpressionField {
        let id = Uuid::new_v4();
        let new = StructExpressionField {
            id: id,
            name: name,
            expr: expr.id(),
            woog_struct: woog_struct.id,
            next: next.map(|struct_expression_field| struct_expression_field.id),
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression_field-struct-impl-nav-forward-to-expr"}}}
    /// Navigate to [`Expression`] across R28(1-*)
    pub fn r28_expression<'a>(&'a self, store: &'a WoogStore) -> Vec<&Expression> {
        vec![store.exhume_expression(&self.expr).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression_field-struct-impl-nav-forward-to-woog_struct"}}}
    /// Navigate to [`StructExpression`] across R27(1-*)
    pub fn r27_struct_expression<'a>(&'a self, store: &'a WoogStore) -> Vec<&StructExpression> {
        vec![store.exhume_struct_expression(&self.woog_struct).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression_field-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`StructExpressionField`] across R30(1-*c)
    pub fn r30_struct_expression_field<'a>(
        &'a self,
        store: &'a WoogStore,
    ) -> Vec<&StructExpressionField> {
        match self.next {
            Some(ref next) => vec![store.exhume_struct_expression_field(next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression_field-struct-impl-nav-backward-one-bi-cond-to-struct_expression_field"}}}
    /// Navigate to [`StructExpressionField`] across R30(1c-1c)
    pub fn r30c_struct_expression_field<'a>(
        &'a self,
        store: &'a WoogStore,
    ) -> Vec<&StructExpressionField> {
        let struct_expression_field = store
            .iter_struct_expression_field()
            .find(|struct_expression_field| struct_expression_field.next == Some(self.id));
        match struct_expression_field {
            Some(ref struct_expression_field) => vec![struct_expression_field],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
