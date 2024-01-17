// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"field_access_target-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog::types::enum_field::EnumField;
use crate::v2::lu_dog::types::field::Field;
use crate::v2::lu_dog::types::field_access::FieldAccess;
use crate::v2::lu_dog::types::function::Function;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-enum-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-hybrid-documentation"}}}
/// The target of a field access.
///
/// It may be either a [`Field`] or a [`Function`].
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-enum-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FieldAccessTarget {
    pub subtype: FieldAccessTargetEnum,
    pub bogus: bool,
    pub id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum FieldAccessTargetEnum {
    EnumField(Uuid),
    Field(Uuid),
    Function(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-implementation"}}}
impl FieldAccessTarget {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-new-impl"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-struct-impl-new_enum_field"}}}
    /// Inter a new FieldAccessTarget in the store, and return it's `id`.
    pub fn new_enum_field(
        bogus: bool,
        subtype: &Rc<RefCell<EnumField>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<FieldAccessTarget>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(FieldAccessTarget {
            bogus: bogus,
            subtype: FieldAccessTargetEnum::EnumField(subtype.borrow().id), // b
            id,
        }));
        store.inter_field_access_target(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-struct-impl-new_field"}}}
    /// Inter a new FieldAccessTarget in the store, and return it's `id`.
    pub fn new_field(
        bogus: bool,
        subtype: &Rc<RefCell<Field>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<FieldAccessTarget>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(FieldAccessTarget {
            bogus: bogus,
            subtype: FieldAccessTargetEnum::Field(subtype.borrow().id), // b
            id,
        }));
        store.inter_field_access_target(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-get-id-impl"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-struct-impl-new_function"}}}
    /// Inter a new FieldAccessTarget in the store, and return it's `id`.
    pub fn new_function(
        bogus: bool,
        subtype: &Rc<RefCell<Function>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<FieldAccessTarget>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(FieldAccessTarget {
            bogus: bogus,
            subtype: FieldAccessTargetEnum::Function(subtype.borrow().id), // b
            id,
        }));
        store.inter_field_access_target(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-struct-impl-nav-backward-1_M-to-field_access"}}}
    /// Navigate to [`FieldAccess`] across R65(1-M)
    pub fn r65_field_access<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<FieldAccess>>> {
        store
            .iter_field_access()
            .filter(|field_access| field_access.borrow().field == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
