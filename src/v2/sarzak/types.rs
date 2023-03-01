//! The _Metamodel_
//!
//! This is the model of the model. From here all is generated...
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"v2::sarzak-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::sarzak-module-definition"}}}
pub mod acknowledged_event;
pub mod associative;
pub mod associative_referent;
pub mod associative_referrer;
pub mod attribute;
pub mod binary;
pub mod boolean;
pub mod cardinality;
pub mod conditional;
pub mod conditionality;
pub mod event;
pub mod external;
pub mod float;
pub mod integer;
pub mod isa;
pub mod many;
pub mod object;
pub mod one;
pub mod referent;
pub mod referrer;
pub mod relationship;
pub mod state;
pub mod string;
pub mod subtype;
pub mod supertype;
pub mod ty;
pub mod unconditional;
pub mod uuid;

pub use crate::v2::sarzak::acknowledged_event::AcknowledgedEvent;
pub use crate::v2::sarzak::associative::Associative;
pub use crate::v2::sarzak::associative_referent::AssociativeReferent;
pub use crate::v2::sarzak::associative_referrer::AssociativeReferrer;
pub use crate::v2::sarzak::attribute::Attribute;
pub use crate::v2::sarzak::binary::Binary;
pub use crate::v2::sarzak::boolean::BOOLEAN;
pub use crate::v2::sarzak::cardinality::Cardinality;
pub use crate::v2::sarzak::conditional::CONDITIONAL;
pub use crate::v2::sarzak::conditionality::Conditionality;
pub use crate::v2::sarzak::event::Event;
pub use crate::v2::sarzak::external::External;
pub use crate::v2::sarzak::float::FLOAT;
pub use crate::v2::sarzak::integer::INTEGER;
pub use crate::v2::sarzak::isa::Isa;
pub use crate::v2::sarzak::many::MANY;
pub use crate::v2::sarzak::object::Object;
pub use crate::v2::sarzak::one::ONE;
pub use crate::v2::sarzak::referent::Referent;
pub use crate::v2::sarzak::referrer::Referrer;
pub use crate::v2::sarzak::relationship::Relationship;
pub use crate::v2::sarzak::state::State;
pub use crate::v2::sarzak::string::STRING;
pub use crate::v2::sarzak::subtype::Subtype;
pub use crate::v2::sarzak::supertype::Supertype;
pub use crate::v2::sarzak::ty::Ty;
pub use crate::v2::sarzak::unconditional::UNCONDITIONAL;
pub use crate::v2::sarzak::uuid::UUID;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
