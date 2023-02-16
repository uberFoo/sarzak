// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"object-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-use-statements"}}}
use uuid::Uuid;

use serde::{Deserializa, Serialize};

use crate::woog::UUID_NS;

// Referent imports
use crate::woog::types::object_method::ObjectMethod;

use crate::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-documentation"}}}
/// An `Object` is a collection of related data. By creating `Object`s, and
/// connecting them with `Relationships` we build a powerful abstraction.
///
/// `Object`s contain [Attribute]s that represent the data that the
/// `Object`encapsulates. All `Object`s have an attribute called `id`, which
/// is a unique idenifier for each class of `Object`. The `id` attribute is a
/// version 5 UUID.
///
/// # Object imported from the sarzak Domain.
///
/// We don’t have a means of representing this as imported in Cuckoo, so I’m just adding
/// it here.
///
/// ❗️{ "imported_object": { "domain": "sarzak", "package": "sarzak", "model_path": "./
///" }}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Object {
    pub description: String,
    pub id: Uuid,
    pub key_letters: String,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-implementation"}}}
impl Object {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-new"}}}
    /// Inter a new Object in the store, and return it's `id`.
    pub fn new(
        description: String,
        key_letters: String,
        name: String,
        store: &mut WoogStore,
    ) -> Object {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{}:{}:{}", description, key_letters, name).as_bytes(),
        );
        let new = Object {
            description: description,
            key_letters: key_letters,
            name: name,
            id,
        };
        store.inter_object(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_Mc-to-object_method"}}}
    /// Navigate to [`ObjectMethod`] across R4(1-Mc)
    pub fn object_method<'a>(&'a self, store: &'a WoogStore) -> Vec<&ObjectMethod> {
        store
            .iter_object_method()
            .filter_map(|object_method| {
                if object_method.1.object == Some(self.id) {
                    Some(object_method.1)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
