//! The _Metamodel_
//!
//! This is the model of the model. From here all is generated...
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"sarzak_v2-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"sarzak_v2-module-definition"}}}
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

pub use crate::sarzak_v2::acknowledged_event::AcknowledgedEvent;
pub use crate::sarzak_v2::associative::Associative;
pub use crate::sarzak_v2::associative_referent::AssociativeReferent;
pub use crate::sarzak_v2::associative_referrer::AssociativeReferrer;
pub use crate::sarzak_v2::attribute::Attribute;
pub use crate::sarzak_v2::binary::Binary;
pub use crate::sarzak_v2::boolean::BOOLEAN;
pub use crate::sarzak_v2::cardinality::Cardinality;
pub use crate::sarzak_v2::conditional::CONDITIONAL;
pub use crate::sarzak_v2::conditionality::Conditionality;
pub use crate::sarzak_v2::event::Event;
pub use crate::sarzak_v2::external::External;
pub use crate::sarzak_v2::float::FLOAT;
pub use crate::sarzak_v2::integer::INTEGER;
pub use crate::sarzak_v2::isa::Isa;
pub use crate::sarzak_v2::many::MANY;
pub use crate::sarzak_v2::object::Object;
pub use crate::sarzak_v2::one::ONE;
pub use crate::sarzak_v2::referent::Referent;
pub use crate::sarzak_v2::referrer::Referrer;
pub use crate::sarzak_v2::relationship::Relationship;
pub use crate::sarzak_v2::state::State;
pub use crate::sarzak_v2::string::STRING;
pub use crate::sarzak_v2::subtype::Subtype;
pub use crate::sarzak_v2::supertype::Supertype;
pub use crate::sarzak_v2::ty::Ty;
pub use crate::sarzak_v2::unconditional::UNCONDITIONAL;
pub use crate::sarzak_v2::uuid::UUID;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
