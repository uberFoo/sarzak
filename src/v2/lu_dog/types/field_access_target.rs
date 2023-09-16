// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"field_access_target-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-use-statements"}}}
use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
use crate::v2::lu_dog::types::enum_field::EnumField;
use crate::v2::lu_dog::types::field::Field;
use crate::v2::lu_dog::types::field_access::FieldAccess;
use crate::v2::lu_dog::types::function::Function;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-enum-documentation"}}}
/// The target of a field access.
///
/// It may be either a [`Field`] or a [`Function`].
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-enum-definition"}}}
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum FieldAccessTarget {
    EnumField(Uuid),
    Field(Uuid),
    Function(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-implementation"}}}
impl FieldAccessTarget {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-new-impl"}}}
    /// Create a new instance of FieldAccessTarget::EnumField
    pub fn new_enum_field(
        enum_field: &Rc<RefCell<EnumField>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Self>> {
        let id = enum_field.borrow().id;
        if let Some(enum_field) = store.exhume_field_access_target(&id) {
            enum_field
        } else {
            let new = Rc::new(RefCell::new(Self::EnumField(id)));
            store.inter_field_access_target(new.clone());
            new
        }
    } // wtf?

    /// Create a new instance of FieldAccessTarget::Field
    pub fn new_field(field: &Rc<RefCell<Field>>, store: &mut LuDogStore) -> Rc<RefCell<Self>> {
        let id = field.borrow().id;
        if let Some(field) = store.exhume_field_access_target(&id) {
            field
        } else {
            let new = Rc::new(RefCell::new(Self::Field(id)));
            store.inter_field_access_target(new.clone());
            new
        }
    } // wtf?

    /// Create a new instance of FieldAccessTarget::Function
    pub fn new_function(
        function: &Rc<RefCell<Function>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Self>> {
        let id = function.borrow().id;
        if let Some(function) = store.exhume_field_access_target(&id) {
            function
        } else {
            let new = Rc::new(RefCell::new(Self::Function(id)));
            store.inter_field_access_target(new.clone());
            new
        }
    } // wtf?

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Self::EnumField(id) => *id,
            Self::Field(id) => *id,
            Self::Function(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_access_target-struct-impl-nav-backward-1_M-to-field_access"}}}
    /// Navigate to [`FieldAccess`] across R65(1-M)
    pub fn r65_field_access<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<FieldAccess>>> {
        span!("r65_field_access");
        store
            .iter_field_access()
            .filter(|field_access| field_access.borrow().field == self.id())
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
