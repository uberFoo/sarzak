// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"object_method-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_method-use-statements"}}}
use uuid::Uuid;

use crate::v2::sarzak::types::object::Object;
use crate::v2::woog::types::block::Block;
use crate::v2::woog::types::call::Call;
use crate::v2::woog::types::function::Function;
use serde::{Deserialize, Serialize};

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
    pub id: Uuid,
    /// R23: [`ObjectMethod`] 'contains a' [`Block`]
    pub block: Uuid,
    /// R4: [`ObjectMethod`] 'is scoped to an' [`Object`]
    pub object: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_method-implementation"}}}
impl ObjectMethod {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_method-struct-impl-new"}}}
    /// Inter a new 'Object Method' in the store, and return it's `id`.
    pub fn new(block: &Block, object: &Object, store: &mut WoogStore) -> ObjectMethod {
        let id = Uuid::new_v4();
        let new = ObjectMethod {
            id: id,
            block: block.id,
            object: object.id,
        };
        store.inter_object_method(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_method-struct-impl-new_"}}}
    /// Inter a new 'Object Method' in the store, and return it's `id`.
    pub fn new_(block: &Block, object: &Object) -> ObjectMethod {
        let id = Uuid::new_v4();
        let new = ObjectMethod {
            id: id,
            block: block.id,
            object: object.id,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_method-struct-impl-nav-forward-to-block"}}}
    /// Navigate to [`Block`] across R23(1-*)
    pub fn r23_block<'a>(&'a self, store: &'a WoogStore) -> Vec<&Block> {
        vec![store.exhume_block(&self.block).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_method-struct-impl-nav-forward-to-object"}}}
    /// Navigate to [`Object`] across R4(1-*)
    pub fn r4_object<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Object> {
        vec![store.exhume_object(&self.object).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_method-struct-impl-nav-backward-1_M-to-call"}}}
    /// Navigate to [`Call`] across R19(1-M)
    pub fn r19_call<'a>(&'a self, store: &'a WoogStore) -> Vec<&Call> {
        store
            .iter_call()
            .filter_map(|call| {
                if call.method == self.id {
                    Some(call)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_method-struct-impl-nav-backward-cond-to-parameter"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_method-impl-nav-subtype-to-supertype-function"}}}
    // Navigate to [`Function`] across R25(isa)
    pub fn r25_function<'a>(&'a self, store: &'a WoogStore) -> Vec<&Function> {
        vec![store.exhume_function(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
