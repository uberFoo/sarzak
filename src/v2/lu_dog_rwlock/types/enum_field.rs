// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"enum_field-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::enumeration::Enumeration;
use crate::v2::lu_dog_rwlock::types::field_access_target::FieldAccessTarget;
use crate::v2::lu_dog_rwlock::types::field_access_target::FieldAccessTargetEnum;
use crate::v2::lu_dog_rwlock::types::struct_field::StructField;
use crate::v2::lu_dog_rwlock::types::tuple_field::TupleField;
use crate::v2::lu_dog_rwlock::types::unit::Unit;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-hybrid-documentation"}}}
/// A field on an Enumeration
///
/// Note that there are three sorts of fields. Tuple, Struct, and “plain?”.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EnumField {
    pub subtype: EnumFieldEnum,
    pub id: Uuid,
    pub name: String,
    /// R88: [`EnumField`] 'belongs to an' [`Enumeration`]
    pub woog_enum: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum EnumFieldEnum {
    StructField(Uuid),
    TupleField(Uuid),
    Unit(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-implementation"}}}
impl EnumField {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-struct-impl-new_plain"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-struct-impl-new_struct_field"}}}
    /// Inter a new EnumField in the store, and return it's `id`.
    pub fn new_struct_field(
        name: String,
        woog_enum: &Arc<RwLock<Enumeration>>,
        subtype: &Arc<RwLock<StructField>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<EnumField>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(EnumField {
            name: name,
            woog_enum: woog_enum.read().unwrap().id,
            subtype: EnumFieldEnum::StructField(subtype.read().unwrap().id), // b
            id,
        }));
        store.inter_enum_field(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-struct-impl-new_struct_field"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-struct-impl-new_tuple_field"}}}
    /// Inter a new EnumField in the store, and return it's `id`.
    pub fn new_tuple_field(
        name: String,
        woog_enum: &Arc<RwLock<Enumeration>>,
        subtype: &Arc<RwLock<TupleField>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<EnumField>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(EnumField {
            name: name,
            woog_enum: woog_enum.read().unwrap().id,
            subtype: EnumFieldEnum::TupleField(subtype.read().unwrap().id), // b
            id,
        }));
        store.inter_enum_field(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-struct-impl-new_tuple_field"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-struct-impl-new_unit"}}}
    /// Inter a new EnumField in the store, and return it's `id`.
    pub fn new_unit(
        name: String,
        woog_enum: &Arc<RwLock<Enumeration>>,
        subtype: &Arc<RwLock<Unit>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<EnumField>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(EnumField {
            name: name,
            woog_enum: woog_enum.read().unwrap().id,
            subtype: EnumFieldEnum::Unit(subtype.read().unwrap().id), // b
            id,
        }));
        store.inter_enum_field(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-struct-impl-nav-forward-to-woog_enum"}}}
    /// Navigate to [`Enumeration`] across R88(1-*)
    pub fn r88_enumeration<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<Enumeration>>> {
        vec![store.exhume_enumeration(&self.woog_enum).unwrap()]
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-impl-nav-subtype-to-supertype-expression"}}}
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-impl-nav-subtype-to-supertype-field_access_target"}}}
    // Navigate to [`FieldAccessTarget`] across R67(isa)
    pub fn r67_field_access_target<'a>(
        &'a self,
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<FieldAccessTarget>>> {
        vec![store
            .iter_field_access_target()
            .find(|field_access_target| {
                if let FieldAccessTargetEnum::EnumField(id) =
                    field_access_target.read().unwrap().subtype
                {
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
