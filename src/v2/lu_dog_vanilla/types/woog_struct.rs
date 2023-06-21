// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"woog_struct-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog_vanilla::types::field::Field;
use crate::v2::lu_dog_vanilla::types::field_access::FieldAccess;
use crate::v2::lu_dog_vanilla::types::implementation::Implementation;
use crate::v2::lu_dog_vanilla::types::item::Item;
use crate::v2::lu_dog_vanilla::types::item::ItemEnum;
use crate::v2::lu_dog_vanilla::types::struct_expression::StructExpression;
use crate::v2::lu_dog_vanilla::types::value_type::ValueType;
use crate::v2::sarzak::types::object::Object;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
use crate::v2::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-documentation"}}}
/// A Type from the Model
///
/// This is really just an alias for `[Object]`.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct WoogStruct {
    pub id: Uuid,
    pub name: String,
    /// R4: [`WoogStruct`] 'represents an' [`Object`]
    pub object: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-implementation"}}}
impl WoogStruct {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-impl-new"}}}
    /// Inter a new 'Struct' in the store, and return it's `id`.
    pub fn new(name: String, object: Option<&Object>, store: &mut LuDogVanillaStore) -> WoogStruct {
        let id = Uuid::new_v4();
        let new = WoogStruct {
            id,
            name,
            object: object.map(|object| object.id),
        };
        store.inter_woog_struct(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-impl-nav-forward-cond-to-object"}}}
    /// Navigate to [`Object`] across R4(1-*c)
    pub fn r4_object<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Object> {
        match self.object {
            Some(ref object) => vec![store.exhume_object(object).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-impl-nav-backward-1_M-to-field"}}}
    /// Navigate to [`Field`] across R7(1-M)
    pub fn r7_field<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Field> {
        store
            .iter_field()
            .filter(|field| field.x_model == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-impl-nav-backward-1_M-to-field_access"}}}
    /// Navigate to [`FieldAccess`] across R66(1-M)
    pub fn r66_field_access<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&FieldAccess> {
        store
            .iter_field_access()
            .filter(|field_access| field_access.woog_struct == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-impl-nav-backward-cond-to-implementation"}}}
    /// Navigate to [`Implementation`] across R8(1-1c)
    pub fn r8c_implementation<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Implementation> {
        let implementation = store
            .iter_implementation()
            .find(|implementation| implementation.model_type == self.id);
        match implementation {
            Some(implementation) => vec![implementation],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-struct-impl-nav-backward-1_M-to-struct_expression"}}}
    /// Navigate to [`StructExpression`] across R39(1-M)
    pub fn r39_struct_expression<'a>(
        &'a self,
        store: &'a LuDogVanillaStore,
    ) -> Vec<&StructExpression> {
        store
            .iter_struct_expression()
            .filter(|struct_expression| struct_expression.woog_struct == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R6(isa)
    pub fn r6_item<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Item> {
        vec![store
            .iter_item()
            .find(|item| {
                if let ItemEnum::WoogStruct(id) = item.subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"woog_struct-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&ValueType> {
        vec![store.exhume_value_type(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
