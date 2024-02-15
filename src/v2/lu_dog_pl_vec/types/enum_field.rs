// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"enum_field-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-use-statements"}}}
use parking_lot::RwLock;
use std::sync::Arc;
use uuid::Uuid;

use crate::v2::lu_dog_pl_vec::types::enumeration::Enumeration;
use crate::v2::lu_dog_pl_vec::types::field_access_target::FieldAccessTarget;
use crate::v2::lu_dog_pl_vec::types::field_access_target::FieldAccessTargetEnum;
use crate::v2::lu_dog_pl_vec::types::struct_field::StructField;
use crate::v2::lu_dog_pl_vec::types::tuple_field::TupleField;
use crate::v2::lu_dog_pl_vec::types::unit::Unit;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_pl_vec::store::ObjectStore as LuDogPlVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-hybrid-documentation"}}}
/// A field on an Enumeration
///
/// Note that there are three sorts of fields. Tuple, Struct, and “plain?”.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EnumField {
    pub subtype: EnumFieldEnum,
    pub id: usize,
    pub name: String,
    /// R88: [`EnumField`] 'belongs to an' [`Enumeration`]
    pub woog_enum: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum EnumFieldEnum {
    StructField(usize),
    TupleField(usize),
    Unit(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-implementation"}}}
impl EnumField {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-struct-impl-new_struct_field"}}}
    /// Inter a new EnumField in the store, and return it's `id`.
    pub fn new_struct_field(
        name: String,
        woog_enum: &Arc<RwLock<Enumeration>>,
        subtype: &Arc<RwLock<StructField>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<EnumField>> {
        store.inter_enum_field(|id| {
            Arc::new(RwLock::new(EnumField {
                name: name.to_owned(),
                woog_enum: woog_enum.read().id,
                subtype: EnumFieldEnum::StructField(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-struct-impl-new_tuple_field"}}}
    /// Inter a new EnumField in the store, and return it's `id`.
    pub fn new_tuple_field(
        name: String,
        woog_enum: &Arc<RwLock<Enumeration>>,
        subtype: &Arc<RwLock<TupleField>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<EnumField>> {
        store.inter_enum_field(|id| {
            Arc::new(RwLock::new(EnumField {
                name: name.to_owned(),
                woog_enum: woog_enum.read().id,
                subtype: EnumFieldEnum::TupleField(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-struct-impl-new_unit"}}}
    /// Inter a new EnumField in the store, and return it's `id`.
    pub fn new_unit(
        name: String,
        woog_enum: &Arc<RwLock<Enumeration>>,
        subtype: &Arc<RwLock<Unit>>,
        store: &mut LuDogPlVecStore,
    ) -> Arc<RwLock<EnumField>> {
        store.inter_enum_field(|id| {
            Arc::new(RwLock::new(EnumField {
                name: name.to_owned(),
                woog_enum: woog_enum.read().id,
                subtype: EnumFieldEnum::Unit(subtype.read().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-struct-impl-nav-forward-to-woog_enum"}}}
    /// Navigate to [`Enumeration`] across R88(1-*)
    pub fn r88_enumeration<'a>(
        &'a self,
        store: &'a LuDogPlVecStore,
    ) -> Vec<Arc<RwLock<Enumeration>>> {
        vec![store.exhume_enumeration(&self.woog_enum).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-impl-nav-subtype-to-supertype-field_access_target"}}}
    // Navigate to [`FieldAccessTarget`] across R67(isa)
    pub fn r67_field_access_target<'a>(
        &'a self,
        store: &'a LuDogPlVecStore,
    ) -> Vec<Arc<RwLock<FieldAccessTarget>>> {
        vec![store
            .iter_field_access_target()
            .find(|field_access_target| {
                if let FieldAccessTargetEnum::EnumField(id) = field_access_target.read().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-implementation"}}}
impl PartialEq for EnumField {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype
            && self.name == other.name
            && self.woog_enum == other.woog_enum
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
