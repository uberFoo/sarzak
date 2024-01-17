// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"field_access_target-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec_tracy::types::enum_field::EnumField;
use crate::v2::lu_dog_vec_tracy::types::field::Field;
use crate::v2::lu_dog_vec_tracy::types::field_access::FieldAccess;
use crate::v2::lu_dog_vec_tracy::types::function::Function;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec_tracy::store::ObjectStore as LuDogVecTracyStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-hybrid-documentation"}}}
/// The target of a field access.
///
/// It may be either a [`Field`] or a [`Function`].
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FieldAccessTarget {
    pub subtype: FieldAccessTargetEnum,
    pub bogus: bool,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum FieldAccessTargetEnum {
    EnumField(usize),
    Field(usize),
    Function(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-implementation"}}}
impl FieldAccessTarget {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-struct-impl-new_enum_field"}}}
    /// Inter a new FieldAccessTarget in the store, and return it's `id`.
    pub fn new_enum_field(
        bogus: bool,
        subtype: &Rc<RefCell<EnumField>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<FieldAccessTarget>> {
        store.inter_field_access_target(|id| {
            Rc::new(RefCell::new(FieldAccessTarget {
                bogus: bogus,
                subtype: FieldAccessTargetEnum::EnumField(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-struct-impl-new_field"}}}
    /// Inter a new FieldAccessTarget in the store, and return it's `id`.
    pub fn new_field(
        bogus: bool,
        subtype: &Rc<RefCell<Field>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<FieldAccessTarget>> {
        store.inter_field_access_target(|id| {
            Rc::new(RefCell::new(FieldAccessTarget {
                bogus: bogus,
                subtype: FieldAccessTargetEnum::Field(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-struct-impl-new_function"}}}
    /// Inter a new FieldAccessTarget in the store, and return it's `id`.
    pub fn new_function(
        bogus: bool,
        subtype: &Rc<RefCell<Function>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<FieldAccessTarget>> {
        store.inter_field_access_target(|id| {
            Rc::new(RefCell::new(FieldAccessTarget {
                bogus: bogus,
                subtype: FieldAccessTargetEnum::Function(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-struct-impl-nav-backward-1_M-to-field_access"}}}
    /// Navigate to [`FieldAccess`] across R65(1-M)
    pub fn r65_field_access<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<FieldAccess>>> {
        span!("r65_field_access");
        store
            .iter_field_access()
            .filter(|field_access| field_access.borrow().field == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-implementation"}}}
impl PartialEq for FieldAccessTarget {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype && self.bogus == other.bogus
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
