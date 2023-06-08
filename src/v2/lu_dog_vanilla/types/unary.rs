// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"unary-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-use-statements"}}}
use crate::v2::lu_dog_vanilla::types::not::NOT;
use serde::{Deserialize, Serialize};
use crate::v2::lu_dog_vanilla::types::operator::Operator;
use crate::v2::lu_dog_vanilla::types::operator::OperatorEnum;
use uuid::Uuid;
use crate::v2::lu_dog_vanilla::types::negation::NEGATION;
use crate::v2::lu_dog_vanilla::store::ObjectStore as LuDogVanillaStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-enum-documentation"}}}
/// Unary Operators
/// 
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-enum-definition"}}}
#[derive(Clone,Debug,Deserialize,PartialEq,Serialize,)]
pub enum Unary {
Negation(Uuid),
Not(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-implementation"}}}
impl Unary {
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-new-impl"}}}
/// Create a new instance of Unary::Negation
pub fn new_negation() -> Self {
// This is already in the store, see associated function `new` above.
Self::Negation(NEGATION)
}

/// Create a new instance of Unary::Not
pub fn new_not() -> Self {
// This is already in the store, see associated function `new` above.
Self::Not(NOT)
}

// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-get-id-impl"}}}
pub fn id(&self) -> Uuid {
match self {
Unary::Negation(id) => *id,
Unary::Not(id) => *id,
}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"unary-impl-nav-subtype-to-supertype-operator"}}}
// Navigate to [`Operator`] across R47(isa)
pub fn r47_operator<'a>(&'a self, store: &'a LuDogVanillaStore) -> Vec<&Operator> {
vec![store.iter_operator().find(|operator| {if let OperatorEnum::Unary(id) = operator.subtype { id == self.id() } else { false } }).unwrap()]
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
