// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"value-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-use-statements"}}}
use uuid::Uuid;

use crate::v2::woog::UUID_NS;

use serde::{Deserialize, Serialize};

// Subtype imports
use crate::v2::woog::types::expression::Expression;
use crate::v2::woog::types::variable::Variable;

// Referrer imports
use crate::v2::woog::types::access::Access;
use crate::v2::woog::types::grace_type::GraceType;

use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-hybrid-documentation"}}}
/// A Value
///
/// I consider a value as an abstraction for a chunk of memory. By upgrading some bytes to
///a `Value` you gain meaning and utility.
///
/// The meaning comes from assigning type information to the ones and zeros. From a modeling
/// perspective it is good enough to think in terms of [`Type`], which is just a general hint
/// about the domain of the value. When we get to generating code we require lower level information
///, which is why we have [`GraceType`].
///
/// The utility are completely compiler/language level constructs. These are [`Mutability`]
/// and [`Visibility`].
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Value {
    pub subtype: ValueEnum,
    pub id: Uuid,
    /// R16: [`Value`] 'is granted utility by' [`Access`]
    pub access: Uuid,
    /// R3: [`Value`] 'is given meaning by a' [`GraceType`]
    pub s_type: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ValueEnum {
    Expression(Uuid),
    Variable(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-implementation"}}}
impl Value {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-struct-impl-new"}}}
    /// Inter a new Value in the store, and return it's `id`.
    pub fn new_expression(
        access: &Access,
        s_type: &GraceType,
        subtype: &Expression,
        store: &mut WoogStore,
    ) -> Value {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{:?}:{:?}:{:?}", access, s_type, subtype).as_bytes(),
        );
        let new = Value {
            access: access.id,
            s_type: s_type.id(),
            subtype: ValueEnum::Expression(subtype.id()),
            id,
        };
        store.inter_value(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-struct-impl-new"}}}
    /// Inter a new Value in the store, and return it's `id`.
    pub fn new_variable(
        access: &Access,
        s_type: &GraceType,
        subtype: &Variable,
        store: &mut WoogStore,
    ) -> Value {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{:?}:{:?}:{:?}", access, s_type, subtype).as_bytes(),
        );
        let new = Value {
            access: access.id,
            s_type: s_type.id(),
            subtype: ValueEnum::Variable(subtype.id()),
            id,
        };
        store.inter_value(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-struct-impl-nav-forward-to-access"}}}
    /// Navigate to [`Access`] across R16(1-*)
    pub fn r16_access<'a>(&'a self, store: &'a WoogStore) -> Vec<&Access> {
        vec![store.exhume_access(&self.access).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-struct-impl-nav-forward-to-ty"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-struct-impl-nav-forward-to-s_type"}}}
    /// Navigate to [`GraceType`] across R3(1-*)
    pub fn r3_grace_type<'a>(&'a self, store: &'a WoogStore) -> Vec<&GraceType> {
        vec![store.exhume_grace_type(&self.s_type).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
