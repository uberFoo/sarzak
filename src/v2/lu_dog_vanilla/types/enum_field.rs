// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"enum_field-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-use-statements"}}}
use uuid::Uuid;

use crate::v2::lu_dog_vanilla::types::enumeration::Enumeration;
use crate::v2::lu_dog_vanilla::types::field_access_target::FieldAccessTarget;
use crate::v2::lu_dog_vanilla::types::field_access_target::FieldAccessTargetEnum;
use crate::v2::lu_dog_vanilla::types::struct_field::StructField;
use crate::v2::lu_dog_vanilla::types::tuple_field::TupleField;
use crate::v2::lu_dog_vanilla::types::unit::Unit;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
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
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-struct-impl-new_struct_field"}}}
    /// Inter a new EnumField in the store, and return it's `id`.
    pub fn new_struct_field(
        name: String,
        woog_enum: &Enumeration,
        subtype: &StructField,
        store: &mut LuDogVanillaStore,
    ) -> EnumField {
        let id = Uuid::new_v4();
        let new = EnumField {
            name: name,
            woog_enum: woog_enum.id,
            subtype: EnumFieldEnum::StructField(subtype.id),
            id,
        };
        store.inter_enum_field(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-struct-impl-new_tuple_field"}}}
    /// Inter a new EnumField in the store, and return it's `id`.
    pub fn new_tuple_field(
        name: String,
        woog_enum: &Enumeration,
        subtype: &TupleField,
        store: &mut LuDogVanillaStore,
    ) -> EnumField {
        let id = Uuid::new_v4();
        let new = EnumField {
            name: name,
            woog_enum: woog_enum.id,
            subtype: EnumFieldEnum::TupleField(subtype.id),
            id,
        };
        store.inter_enum_field(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-struct-impl-new_unit"}}}
    /// Inter a new EnumField in the store, and return it's `id`.
    pub fn new_unit(
        name: String,
        woog_enum: &Enumeration,
        subtype: &Unit,
        store: &mut LuDogVanillaStore,
    ) -> EnumField {
        let id = Uuid::new_v4();
        let new = EnumField {
            name: name,
            woog_enum: woog_enum.id,
            subtype: EnumFieldEnum::Unit(subtype.id),
            id,
        };
        store.inter_enum_field(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-struct-impl-nav-forward-to-woog_enum"}}}
    /// Navigate to [`Enumeration`] across R88(1-*)
    pub fn r88_enumeration<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Enumeration> {
        vec![store.exhume_enumeration(&self.woog_enum).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-impl-nav-subtype-to-supertype-field_access_target"}}}
    // Navigate to [`FieldAccessTarget`] across R67(isa)
    pub fn r67_field_access_target<'a>(
        &'a self,
        store: &'a LuDogVanillaStore,
    ) -> Vec<&FieldAccessTarget> {
        vec![store
            .iter_field_access_target()
            .find(|field_access_target| {
                if let FieldAccessTargetEnum::EnumField(id) = field_access_target.subtype {
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
