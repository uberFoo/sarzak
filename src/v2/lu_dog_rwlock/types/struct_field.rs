// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"struct_field-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_field-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::enum_field::EnumField;
use crate::v2::lu_dog_rwlock::types::enum_field::EnumFieldEnum;
use crate::v2::lu_dog_rwlock::types::expression::Expression;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_field-struct-documentation"}}}
/// A field that is a structure.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_field-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StructField {
    pub id: Uuid,
    pub name: String,
    /// R89: [`StructField`] 'is composed with a' [`Expression`]
    pub expression: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_field-implementation"}}}
impl StructField {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_field-struct-impl-new"}}}
    /// Inter a new 'Struct Field' in the store, and return it's `id`.
    pub fn new(
        name: String,
        expression: Option<&Arc<RwLock<Expression>>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<StructField>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(StructField {
            id,
            name,
            expression: expression.map(|expression| expression.read().unwrap().id()),
        }));
        store.inter_struct_field(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_field-struct-impl-nav-forward-cond-to-expression"}}}
    /// Navigate to [`Expression`] across R89(1-*c)
    pub fn r89_expression<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r89_expression");
        match self.expression {
            Some(ref expression) => vec![store.exhume_expression(&expression).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_field-impl-nav-subtype-to-supertype-enum_field"}}}
    // Navigate to [`EnumField`] across R85(isa)
    pub fn r85_enum_field<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<EnumField>>> {
        span!("r85_enum_field");
        vec![store
            .iter_enum_field()
            .find(|enum_field| {
                if let EnumFieldEnum::StructField(id) = enum_field.read().unwrap().subtype {
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
