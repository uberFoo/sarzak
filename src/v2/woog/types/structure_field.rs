// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"structure_field-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure_field-use-statements"}}}
use uuid::Uuid;

use crate::v2::woog::types::expression::Expression;
use crate::v2::woog::types::struct_expression::StructExpression;
use crate::v2::woog::UUID_NS;
use serde::{Deserialize, Serialize};

use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure_field-struct-documentation"}}}
/// A Field in a Structure
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure_field-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StructureField {
    pub id: Uuid,
    pub name: String,
    /// R28: [`StructureField`] 'is initialized with' [`Expression`]
    pub expr: Uuid,
    /// R27: [`StructureField`] 'defines a' [`StructExpression`]
    pub woog_struct: Uuid,
    /// R30: [`StructureField`] 'came before' [`StructureField`]
    pub next: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure_field-implementation"}}}
impl StructureField {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure_field-struct-impl-new"}}}
    /// Inter a new 'Structure Field' in the store, and return it's `id`.
    pub fn new(
        name: String,
        expr: &Expression,
        woog_struct: &StructExpression,
        next: Option<&StructureField>,
        store: &mut WoogStore,
    ) -> StructureField {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{}:{:?}:{:?}:{:?}", name, expr, woog_struct, next).as_bytes(),
        );
        let new = StructureField {
            id: id,
            name: name,
            expr: expr.id(),
            woog_struct: woog_struct.id,
            next: next.map(|structure_field| structure_field.id),
        };
        store.inter_structure_field(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure_field-struct-impl-nav-forward-to-expr"}}}
    /// Navigate to [`Expression`] across R28(1-*)
    pub fn r28_expression<'a>(&'a self, store: &'a WoogStore) -> Vec<&Expression> {
        vec![store.exhume_expression(&self.expr).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure_field-struct-impl-nav-forward-to-woog_struct"}}}
    /// Navigate to [`StructExpression`] across R27(1-*)
    pub fn r27_struct_expression<'a>(&'a self, store: &'a WoogStore) -> Vec<&StructExpression> {
        vec![store.exhume_struct_expression(&self.woog_struct).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure_field-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`StructureField`] across R30(1-*c)
    pub fn r30_structure_field<'a>(&'a self, store: &'a WoogStore) -> Vec<&StructureField> {
        match self.next {
            Some(ref next) => vec![store.exhume_structure_field(next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure_field-struct-impl-nav-backward-one-bi-cond-to-structure_field"}}}
    /// Navigate to [`StructureField`] across R30(1c-1c)
    pub fn r30c_structure_field<'a>(&'a self, store: &'a WoogStore) -> Vec<&StructureField> {
        let structure_field = store
            .iter_structure_field()
            .find(|structure_field| structure_field.next == Some(self.id));
        match structure_field {
            Some(ref structure_field) => vec![structure_field],
            None => Vec::new(),
        }
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure_field-struct-impl-nav-forward-assoc-to-woog_struct"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure_field-struct-impl-nav-forward-assoc-to-field"}}}
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure_field-struct-impl-nav-forward-assoc-to-field"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"structure_field-struct-impl-nav-forward-assoc-to-woog_struct"}}}
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
