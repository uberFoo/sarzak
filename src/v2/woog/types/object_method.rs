// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"object_method-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_method-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::v2::woog::UUID_NS;

// Referrer imports
use crate::v2::sarzak::types::object::Object;

// Referent imports
use crate::v2::woog::types::parameter::Parameter;

use crate::v2::sarzak::store::ObjectStore as SarzakStore;
use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_method-struct-documentation"}}}
/// Method
///
/// This represents a function's signature. We don't (yet) care about the body of the function
///. We are however very interested in it's type, which implies parameters and their types,
/// as well as our return type.
///
/// Looking at this more closely, I think that this should be related to a parameter list, and
/// the list related to the string of parameters. It may just be a nit, but it does bother me
/// a bit. I'll come back and fix it once it's less troublesome to generate this domain.
///
/// The function in question is a method, hanging off of an [`Object`][o].
///
/// [o][damn, now I need a documentation server].
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_method-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ObjectMethod {
    pub description: String,
    pub id: Uuid,
    pub name: String,
    /// R4: [`ObjectMethod`] 'is scoped to an' [`Object`]
    pub object: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_method-implementation"}}}
impl ObjectMethod {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_method-struct-impl-new"}}}
    /// Inter a new ObjectMethod in the store, and return it's `id`.
    pub fn new(
        description: String,
        name: String,
        object: &Object,
        store: &mut WoogStore,
    ) -> ObjectMethod {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{}:{}:{:?}", description, name, object).as_bytes(),
        );
        let new = ObjectMethod {
            description: description,
            name: name,
            object: object.id,
            id,
        };
        store.inter_object_method(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_method-struct-impl-nav-forward-to-object"}}}
    /// Navigate to [`Object`] across R4(1-*)
    pub fn r4_object<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Object> {
        vec![store.exhume_object(&self.object).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_method-struct-impl-nav-backward-cond-to-parameter"}}}
    /// Navigate to [`Parameter`] across R5(1-1c)
    pub fn r5c_parameter<'a>(&'a self, store: &'a WoogStore) -> Vec<&Parameter> {
        let parameter = store
            .iter_parameter()
            .find(|parameter| parameter.method == self.id);
        match parameter {
            Some(ref parameter) => vec![parameter],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
