// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"struct_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-use-statements"}}}
use uuid::Uuid;

use crate::v2::woog::types::expression::Expression;
use crate::v2::woog::types::struct_expression_field::StructExpressionField;
use serde::{Deserialize, Serialize};

use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-struct-documentation"}}}
/// An instance
///
/// A struct expression creates an instance of a structure, e.g.,
///
/// ``rust
/// Foo {
///     bar: 0u8,
///     baz: true
/// }
/// ````
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StructExpression {
    pub id: Uuid,
    pub name: String,
    pub salt: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-implementation"}}}
impl StructExpression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-struct-impl-new"}}}
    /// Inter a new 'Struct Expression' in the store, and return it's `id`.
    pub fn new(name: String, salt: Uuid, store: &mut WoogStore) -> StructExpression {
        let id = Uuid::new_v4();
        let new = StructExpression {
            id: id,
            name: name,
            salt: salt,
        };
        store.inter_struct_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-struct-impl-new_"}}}
    /// Inter a new 'Struct Expression' in the store, and return it's `id`.
    pub fn new_(name: String, salt: Uuid) -> StructExpression {
        let id = Uuid::new_v4();
        let new = StructExpression {
            id: id,
            name: name,
            salt: salt,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-struct-impl-nav-backward-1_M-to-struct_expression_field"}}}
    /// Navigate to [`StructExpressionField`] across R27(1-M)
    pub fn r27_struct_expression_field<'a>(
        &'a self,
        store: &'a WoogStore,
    ) -> Vec<&StructExpressionField> {
        store
            .iter_struct_expression_field()
            .filter_map(|struct_expression_field| {
                if struct_expression_field.woog_struct == self.id {
                    Some(struct_expression_field)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R10(isa)
    pub fn r10_expression<'a>(&'a self, store: &'a WoogStore) -> Vec<&Expression> {
        vec![store.exhume_expression(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
