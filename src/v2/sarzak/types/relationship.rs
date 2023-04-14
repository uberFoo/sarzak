// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"relationship-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship-use-statements"}}}
use crate::v2::sarzak::store::ObjectStore as SarzakStore;
use crate::v2::sarzak::types::associative::Associative;
use crate::v2::sarzak::types::binary::Binary;
use crate::v2::sarzak::types::isa::Isa;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship-enum-documentation"}}}
/// A `Relationship` indicates that a set of objects are connected to each other in some manner
///. Typically it is a _real world_ relationship. In the
/// case of this model it is strictly an abstraction.
///
/// There are three types of `Relationship`: [`Isa`], [`Binary`], and [`Associative`]. Thus
/// `Relationship` is itself the *supertype* in an [`Isa`] relationship. It is a partitioning
/// *supertype-subtype* relationship, rather one of inheritance. As such, it’s  perfectly
/// suited to a rust `enum`! 😃
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship-enum-definition"}}}
#[derive(Clone, Debug, Eq, Deserialize, Hash, PartialEq, Serialize)]
pub enum Relationship {
    Associative(Uuid),
    Binary(Uuid),
    Isa(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship-implementation"}}}
impl Relationship {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship-new-impl"}}}
    /// Create a new instance of Relationship::Associative
    pub fn new_associative(associative: &Associative, store: &mut SarzakStore) -> Self {
        let new = Self::Associative(associative.id);
        store.inter_relationship(new.clone());
        new
    }

    pub fn new_associative_(associative: &Associative) -> Self {
        let new = Self::Associative(associative.id);
        new
    }

    /// Create a new instance of Relationship::Binary
    pub fn new_binary(binary: &Binary, store: &mut SarzakStore) -> Self {
        let new = Self::Binary(binary.id);
        store.inter_relationship(new.clone());
        new
    }

    pub fn new_binary_(binary: &Binary) -> Self {
        let new = Self::Binary(binary.id);
        new
    }

    /// Create a new instance of Relationship::Isa
    pub fn new_isa(isa: &Isa, store: &mut SarzakStore) -> Self {
        let new = Self::Isa(isa.id);
        store.inter_relationship(new.clone());
        new
    }

    pub fn new_isa_(isa: &Isa) -> Self {
        let new = Self::Isa(isa.id);
        new
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Relationship::Associative(id) => *id,
            Relationship::Binary(id) => *id,
            Relationship::Isa(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
