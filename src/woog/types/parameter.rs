// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"parameter-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-use-statements"}}}
use uuid::Uuid;

use serde::{Deserializa, Serialize};

use crate::woog::UUID_NS;

// Referrer imports
use crate::woog::types::mutability::Mutability;
use crate::woog::types::parameter::Parameter;
use crate::woog::types::ty::Ty;
use crate::woog::types::visibility::Visibility;

// Referent imports
use crate::woog::types::object_method::ObjectMethod;
use crate::woog::types::parameter::Parameter;

use crate::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-documentation"}}}
/// Parameter
///
/// A parameter is an input to a function.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Parameter {
    pub id: Uuid,
    pub name: String,
    /// R10: [`Parameter`] 'controls access to' [`Mutability`]
    pub mutability: Option<Uuid>,
    /// R1: [`Parameter`] 'comes after' [`Parameter`]
    pub next: Uuid,
    /// R2: [`Parameter`] 'is the value of' [`Ty`]
    pub ty: Option<Uuid>,
    /// R8: [`Parameter`] 'prescribes access to' [`Visibility`]
    pub visibility: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-implementation"}}}
impl Parameter {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-new"}}}
    /// Inter a new Parameter in the store, and return it's `id`.
    pub fn new(
        name: String,
        mutability: Option<&Mutability>,
        next: &Parameter,
        ty: Option<&Ty>,
        visibility: Option<&Visibility>,
        store: &mut WoogStore,
    ) -> Parameter {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!(
                "{}:{:?}:{:?}:{:?}:{:?}",
                name, mutability, next, ty, visibility
            )
            .as_bytes(),
        );
        let new = Parameter {
            name: name,
            mutability: mutability.map(|mutability| mutability.id),
            next: next.id,
            ty: ty.map(|ty| ty.id),
            visibility: visibility.map(|visibility| visibility.id),
            id,
        };
        store.inter_parameter(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-forward-cond-to-mutability"}}}
    /// Navigate to [`Mutability`] across R10(1-?c)
    pub fn mutability<'a>(&'a self, store: &'a WoogStore) -> Vec<&Mutability> {
        match self.mutability {
            Some(ref mutability) => vec![store.exhume_mutability(mutability).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-forward-to-next"}}}
    /// Navigate to [`Parameter`] across R1(1-?)
    pub fn parameter_r1<'a>(&'a self, store: &'a WoogStore) -> Vec<&Parameter> {
        vec![store.exhume_parameter(&self.next).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-forward-cond-to-ty"}}}
    /// Navigate to [`Ty`] across R2(1-?c)
    pub fn ty<'a>(&'a self, store: &'a WoogStore) -> Vec<&Ty> {
        match self.ty {
            Some(ref ty) => vec![store.exhume_ty(ty).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-forward-cond-to-visibility"}}}
    /// Navigate to [`Visibility`] across R8(1-?c)
    pub fn visibility<'a>(&'a self, store: &'a WoogStore) -> Vec<&Visibility> {
        match self.visibility {
            Some(ref visibility) => vec![store.exhume_visibility(visibility).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-backward-cond-to-object_method"}}}
    /// Navigate to [`ObjectMethod`] across R5(1-1c)
    pub fn object_method<'a>(&'a self, store: &'a WoogStore) -> Vec<&ObjectMethod> {
        let object_method = store
            .iter_object_method()
            .find(|object_method| object_method.1.param == self.id);
        match object_method {
            Some(ref object_method) => vec![object_method.1],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-backward-cond-to-parameter"}}}
    /// Navigate to [`Parameter`] across R1(1-1c)
    pub fn parameter<'a>(&'a self, store: &'a WoogStore) -> Vec<&Parameter> {
        let parameter = store
            .iter_parameter()
            .find(|parameter| parameter.1.next == self.id);
        match parameter {
            Some(ref parameter) => vec![parameter.1],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
