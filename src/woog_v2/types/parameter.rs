// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"parameter-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::woog_v2::UUID_NS;

// Referrer imports
use crate::sarzak_v2::types::ty::Ty;
use crate::woog_v2::types::mutability::Mutability;
use crate::woog_v2::types::object_method::ObjectMethod;
use crate::woog_v2::types::visibility::Visibility;

use crate::sarzak_v2::store::ObjectStore as SarzakV2Store;
use crate::woog_v2::store::ObjectStore as WoogV2Store;
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
        store: &mut WoogV2Store,
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
    pub fn r10_mutability<'a>(&'a self, store: &'a WoogV2Store) -> Vec<&Mutability> {
        vec![store.exhume_mutability(&self.mutability).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-forward-cond-to-method"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-forward-to-method"}}}
    /// Navigate to [`ObjectMethod`] across R5(1-?)
    pub fn r5_object_method<'a>(&'a self, store: &'a WoogV2Store) -> Vec<&ObjectMethod> {
        vec![store.exhume_object_method(&self.method).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`Parameter`] across R1(1-?c)
    pub fn r1_parameter<'a>(&'a self, store: &'a WoogV2Store) -> Vec<&Parameter> {
        match self.next {
            Some(ref next) => vec![store.exhume_parameter(next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-forward-to-ty"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-forward-cond-to-ty"}}}
    /// Navigate to [`Ty`] across R2(1-?)
    pub fn r2_ty<'a>(&'a self, store: &'a SarzakV2Store) -> Vec<&Ty> {
        vec![store.exhume_ty(&self.ty).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-forward-to-visibility"}}}
    /// Navigate to [`Visibility`] across R8(1-?)
    pub fn r8_visibility<'a>(&'a self, store: &'a WoogV2Store) -> Vec<&Visibility> {
        vec![store.exhume_visibility(&self.visibility).unwrap()]
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-backward-one-to-object_method"}}}
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-backward-one-bi-cond-to-parameter"}}}
    /// Navigate to [`Parameter`] across R1(1c-1c)
    pub fn r1c_parameter<'a>(&'a self, store: &'a WoogV2Store) -> Vec<&Parameter> {
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
