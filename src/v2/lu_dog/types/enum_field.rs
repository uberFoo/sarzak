// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"enum_field-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog::types::enumeration::Enumeration;
use crate::v2::lu_dog::types::expression::Expression;
use crate::v2::lu_dog::types::field_access_target::FieldAccessTarget;
use crate::v2::lu_dog::types::plain::Plain;
use crate::v2::lu_dog::types::struct_field::StructField;
use crate::v2::lu_dog::types::tuple_field::TupleField;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
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
    Plain(Uuid),
    StructField(Uuid),
    TupleField(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-implementation"}}}
impl EnumField {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-struct-impl-new_plain"}}}
    /// Inter a new EnumField in the store, and return it's `id`.
    pub fn new_plain(
        name: String,
        woog_enum: &Rc<RefCell<Enumeration>>,
        subtype: &Rc<RefCell<Plain>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<EnumField>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(EnumField {
            name: name,
            woog_enum: woog_enum.borrow().id,
            subtype: EnumFieldEnum::Plain(subtype.borrow().id),
            id,
        }));
        store.inter_enum_field(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-struct-impl-new_struct_field"}}}
    /// Inter a new EnumField in the store, and return it's `id`.
    pub fn new_struct_field(
        name: String,
        woog_enum: &Rc<RefCell<Enumeration>>,
        subtype: &Rc<RefCell<StructField>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<EnumField>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(EnumField {
            name: name,
            woog_enum: woog_enum.borrow().id,
            subtype: EnumFieldEnum::StructField(subtype.borrow().id),
            id,
        }));
        store.inter_enum_field(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-struct-impl-new_tuple_field"}}}
    /// Inter a new EnumField in the store, and return it's `id`.
    pub fn new_tuple_field(
        name: String,
        woog_enum: &Rc<RefCell<Enumeration>>,
        subtype: &Rc<RefCell<TupleField>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<EnumField>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(EnumField {
            name: name,
            woog_enum: woog_enum.borrow().id,
            subtype: EnumFieldEnum::TupleField(subtype.borrow().id),
            id,
        }));
        store.inter_enum_field(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-struct-impl-nav-forward-to-woog_enum"}}}
    /// Navigate to [`Enumeration`] across R88(1-*)
    pub fn r88_enumeration<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Enumeration>>> {
        span!("r88_enumeration");
        vec![store.exhume_enumeration(&self.woog_enum).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Expression>>> {
        span!("r15_expression");
        vec![store.exhume_expression(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-impl-nav-subtype-to-supertype-field_access_target"}}}
    // Navigate to [`FieldAccessTarget`] across R67(isa)
    pub fn r67_field_access_target<'a>(
        &'a self,
        store: &'a LuDogStore,
    ) -> Vec<Rc<RefCell<FieldAccessTarget>>> {
        span!("r67_field_access_target");
        vec![store.exhume_field_access_target(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
