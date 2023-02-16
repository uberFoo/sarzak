// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"object_method-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_method-use-statements"}}}
use uuid::Uuid;

use serde::{Deserializa, Serialize};

use crate::woog::UUID_NS;

// Referrer imports
use crate::woog::types::object::Object;
use crate::woog::types::parameter::Parameter;
use crate::woog::types::ty::Ty;
use crate::woog::types::visibility::Visibility;

use crate::woog::store::ObjectStore as WoogStore;
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
    /// R4: [`ObjectMethod`] 'may have many' [`Object`]
    pub object: Option<Uuid>,
    /// R5: [`ObjectMethod`] 'is used by an' [`Parameter`]
    pub param: Uuid,
    /// R3: [`ObjectMethod`] 'is the value of' [`Ty`]
    pub ty: Option<Uuid>,
    /// R7: [`ObjectMethod`] 'prescribes access to' [`Visibility`]
    pub visibility: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_method-implementation"}}}
impl ObjectMethod {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_method-struct-impl-new"}}}
    /// Inter a new ObjectMethod in the store, and return it's `id`.
    pub fn new(
        description: String,
        name: String,
        object: Option<&Object>,
        param: &Parameter,
        ty: Option<&Ty>,
        visibility: Option<&Visibility>,
        store: &mut WoogStore,
    ) -> ObjectMethod {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!(
                "{}:{}:{:?}:{:?}:{:?}:{:?}",
                description, name, object, param, ty, visibility
            )
            .as_bytes(),
        );
        let new = ObjectMethod {
            description: description,
            name: name,
            object: object.map(|object| object.id),
            param: param.id,
            ty: ty.map(|ty| ty.id),
            visibility: visibility.map(|visibility| visibility.id),
            id,
        };
        store.inter_object_method(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_method-struct-impl-nav-forward-cond-to-object"}}}
    /// Navigate to [`Object`] across R4(1-?c)
    pub fn object<'a>(&'a self, store: &'a WoogStore) -> Vec<&Object> {
        match self.object {
            Some(ref object) => vec![store.exhume_object(object).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_method-struct-impl-nav-forward-to-param"}}}
    /// Navigate to [`Parameter`] across R5(1-?)
    pub fn parameter_r5<'a>(&'a self, store: &'a WoogStore) -> Vec<&Parameter> {
        vec![store.exhume_parameter(&self.param).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_method-struct-impl-nav-forward-cond-to-ty"}}}
    /// Navigate to [`Ty`] across R3(1-?c)
    pub fn ty<'a>(&'a self, store: &'a WoogStore) -> Vec<&Ty> {
        match self.ty {
            Some(ref ty) => vec![store.exhume_ty(ty).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object_method-struct-impl-nav-forward-cond-to-visibility"}}}
    /// Navigate to [`Visibility`] across R7(1-?c)
    pub fn visibility<'a>(&'a self, store: &'a WoogStore) -> Vec<&Visibility> {
        match self.visibility {
            Some(ref visibility) => vec![store.exhume_visibility(visibility).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
