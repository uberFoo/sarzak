// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"import-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"import-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog::types::item::Item;
use crate::v2::lu_dog::types::item::ItemEnum;
use crate::v2::lu_dog::types::value_type::ValueType;
use crate::v2::lu_dog::types::value_type::ValueTypeEnum;
use crate::v2::sarzak::types::object::Object;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
use crate::v2::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"import-struct-documentation"}}}
/// An Import of a foreign ObjectStore
///
/// This indicates to the downstream model compiler that it needs to emit code to load the imported
///  ObjectStore.
///
/// I've got this has_alias boolean here because I don't have `Option<String>`. I never needed
///  it until now, because you get an option with a 1c relationship. Not proud of this, but it's
///  the best alternative. Makes me wonder about adding an `Option` type to the primitives though
/// .
///
/// I suppose if there were a way to signify a null string. Or I could check if it's length
///  is 0. I think adding the bool is cleaner.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"import-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Import {
    pub alias: String,
    pub has_alias: bool,
    pub id: Uuid,
    pub name: String,
    pub x_path: String,
    /// R40: [`Import`] '' [`Object`]
    pub object: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"import-implementation"}}}
impl Import {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"import-struct-impl-new"}}}
    /// Inter a new 'Import' in the store, and return it's `id`.
    pub fn new(
        alias: String,
        has_alias: bool,
        name: String,
        x_path: String,
        object: Option<&Object>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<Import>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(Import {
            alias,
            has_alias,
            id,
            name,
            x_path,
            object: object.as_ref().map(|object| object.id),
        }));
        store.inter_import(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"import-struct-impl-nav-forward-cond-to-object"}}}
    /// Navigate to [`Object`] across R40(1-*c)
    pub fn r40_object<'a>(
        &'a self,
        store: &'a SarzakStore,
    ) -> Vec<std::sync::Arc<std::sync::RwLock<Object>>> {
        match self.object {
            Some(ref object) => vec![store.exhume_object(&object).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"import-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R6(isa)
    pub fn r6_item<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Item>>> {
        vec![store
            .iter_item()
            .find(|item| {
                if let ItemEnum::Import(id) = item.borrow().subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"import-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<ValueType>>> {
        vec![store
            .iter_value_type()
            .find(|value_type| {
                if let ValueTypeEnum::Import(id) = value_type.borrow().subtype {
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
