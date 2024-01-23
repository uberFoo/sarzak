// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"x_value-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::woog::types::access::Access;
use crate::v2::woog::types::expression::Expression;
use crate::v2::woog::types::grace_type::GraceType;
use crate::v2::woog::types::variable::Variable;
use serde::{Deserialize, Serialize};

use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-hybrid-documentation"}}}
/// A Value
///
/// I consider a value as an abstraction for a chunk of memory. By upgrading some bytes to
/// a `Value` you gain meaning and utility.
///
/// The meaning comes from assigning type information to the ones and zeros. From a modeling
///  perspective it is good enough to think in terms of [`Type`], which is just a general hint
///  about the domain of the value. When we get to generating code we require lower level information
/// , which is why we have [`GraceType`].
///
/// The utility are completely compiler/language level constructs. These are [`Mutability`]
///  and [`Visibility`].
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct XValue {
    pub subtype: XValueEnum,
    pub id: Uuid,
    /// R16: [`XValue`] 'is granted utility by' [`Access`]
    pub access: Uuid,
    /// R3: [`XValue`] 'is given meaning by a' [`GraceType`]
    pub ty: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum XValueEnum {
    Expression(Uuid),
    Variable(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-implementation"}}}
impl XValue {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-new_expression"}}}
    /// Inter a new XValue in the store, and return it's `id`.
    pub fn new_expression(
        access: &Arc<RwLock<Access>>,
        ty: &Arc<RwLock<GraceType>>,
        subtype: &Arc<RwLock<Expression>>,
        store: &mut WoogStore,
    ) -> Arc<RwLock<XValue>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(XValue {
            access: access.read().unwrap().id,
            ty: ty.read().unwrap().id(),
            subtype: XValueEnum::Expression(subtype.read().unwrap().id()), // b
            id,
        }));
        store.inter_x_value(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-new_variable"}}}
    /// Inter a new XValue in the store, and return it's `id`.
    pub fn new_variable(
        access: &Arc<RwLock<Access>>,
        ty: &Arc<RwLock<GraceType>>,
        subtype: &Arc<RwLock<Variable>>,
        store: &mut WoogStore,
    ) -> Arc<RwLock<XValue>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(XValue {
            access: access.read().unwrap().id,
            ty: ty.read().unwrap().id(),
            subtype: XValueEnum::Variable(subtype.read().unwrap().id), // b
            id,
        }));
        store.inter_x_value(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-nav-forward-to-access"}}}
    /// Navigate to [`Access`] across R16(1-*)
    pub fn r16_access<'a>(&'a self, store: &'a WoogStore) -> Vec<Arc<RwLock<Access>>> {
        vec![store.exhume_access(&self.access).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`GraceType`] across R3(1-*)
    pub fn r3_grace_type<'a>(&'a self, store: &'a WoogStore) -> Vec<Arc<RwLock<GraceType>>> {
        vec![store.exhume_grace_type(&self.ty).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
