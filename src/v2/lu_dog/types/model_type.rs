// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"model_type-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"model_type-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog::types::field::Field;
use crate::v2::lu_dog::types::implementation::Implementation;
use crate::v2::lu_dog::types::item::Item;
use crate::v2::sarzak::types::object::Object;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
use crate::v2::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"model_type-struct-documentation"}}}
/// A Type from the Model
///
/// This is really just an alias for `[Object]`.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"model_type-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ModelType {
    pub id: Uuid,
    /// R4: [`ModelType`] 'represents an' [`Object`]
    pub object: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"model_type-implementation"}}}
impl ModelType {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"model_type-struct-impl-new"}}}
    /// Inter a new 'Model Type' in the store, and return it's `id`.
    pub fn new(object: &Object, store: &mut LuDogStore) -> ModelType {
        let id = Uuid::new_v4();
        let new = ModelType {
            id: id,
            object: object.id,
        };
        store.inter_model_type(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"model_type-struct-impl-new_"}}}
    /// Inter a new 'Model Type' in the store, and return it's `id`.
    pub fn new_(object: &Object) -> ModelType {
        let id = Uuid::new_v4();
        let new = ModelType {
            id: id,
            object: object.id,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"model_type-struct-impl-nav-forward-to-object"}}}
    /// Navigate to [`Object`] across R4(1-*)
    pub fn r4_object<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Object> {
        vec![store.exhume_object(&self.object).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"model_type-struct-impl-nav-backward-1_M-to-field"}}}
    /// Navigate to [`Field`] across R7(1-M)
    pub fn r7_field<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Field> {
        store
            .iter_field()
            .filter_map(|field| {
                if field.model == self.id {
                    Some(field)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"model_type-struct-impl-nav-backward-cond-to-implementation"}}}
    /// Navigate to [`Implementation`] across R8(1-1c)
    pub fn r8c_implementation<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Implementation> {
        let implementation = store
            .iter_implementation()
            .find(|implementation| implementation.model_type == self.id);
        match implementation {
            Some(ref implementation) => vec![implementation],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"model_type-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R6(isa)
    pub fn r6_item<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Item> {
        vec![store.exhume_item(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
