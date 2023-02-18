// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"parameter-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::v2::woog::UUID_NS;

// Referrer imports
use crate::v2::sarzak::types::ty::Ty;
use crate::v2::woog::types::mutability::Mutability;
use crate::v2::woog::types::object_method::ObjectMethod;
use crate::v2::woog::types::visibility::Visibility;

use crate::v2::sarzak::store::ObjectStore as SarzakStore;
use crate::v2::woog::store::ObjectStore as WoogStore;
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
    /// R10: [`Parameter`] 'has' [`Mutability`]
    pub mutability: Uuid,
    /// R5: [`Parameter`] 'is used by an' [`ObjectMethod`]
    pub method: Uuid,
    /// R1: [`Parameter`] 'came before' [`Parameter`]
    pub next: Option<Uuid>,
    /// R2: [`Parameter`] 'evaluates to' [`Ty`]
    pub ty: Uuid,
    /// R8: [`Parameter`] 'has a' [`Visibility`]
    pub visibility: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-implementation"}}}
impl Parameter {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-new"}}}
    /// Inter a new Parameter in the store, and return it's `id`.
    pub fn new(
        name: String,
        mutability: &Mutability,
        method: &ObjectMethod,
        next: Option<&Parameter>,
        ty: &Ty,
        visibility: &Visibility,
        store: &mut WoogStore,
    ) -> Parameter {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!(
                "{}:{:?}:{:?}:{:?}:{:?}:{:?}",
                name, mutability, method, next, ty, visibility
            )
            .as_bytes(),
        );
        let new = Parameter {
            name: name,
            mutability: mutability.id(),
            method: method.id,
            next: next.map(|parameter| parameter.id),
            ty: ty.id(),
            visibility: visibility.id(),
            id,
        };
        store.inter_parameter(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-forward-to-mutability"}}}
    /// Navigate to [`Mutability`] across R10(1-?)
    pub fn r10_mutability<'a>(&'a self, store: &'a WoogStore) -> Vec<&Mutability> {
        vec![store.exhume_mutability(&self.mutability).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-forward-to-method"}}}
    /// Navigate to [`ObjectMethod`] across R5(1-?)
    pub fn r5_object_method<'a>(&'a self, store: &'a WoogStore) -> Vec<&ObjectMethod> {
        vec![store.exhume_object_method(&self.method).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`Parameter`] across R1(1-?c)
    pub fn r1_parameter<'a>(&'a self, store: &'a WoogStore) -> Vec<&Parameter> {
        match self.next {
            Some(ref next) => vec![store.exhume_parameter(next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`Ty`] across R2(1-?)
    pub fn r2_ty<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Ty> {
        vec![store.exhume_ty(&self.ty).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-forward-to-visibility"}}}
    /// Navigate to [`Visibility`] across R8(1-?)
    pub fn r8_visibility<'a>(&'a self, store: &'a WoogStore) -> Vec<&Visibility> {
        vec![store.exhume_visibility(&self.visibility).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-backward-one-bi-cond-to-parameter"}}}
    /// Navigate to [`Parameter`] across R1(1c-1c)
    pub fn r1c_parameter<'a>(&'a self, store: &'a WoogStore) -> Vec<&Parameter> {
        let parameter = store
            .iter_parameter()
            .find(|parameter| parameter.1.next == Some(self.id));
        match parameter {
            Some(ref parameter) => vec![parameter.1],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
