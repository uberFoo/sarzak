//! Sarzak Domain
//!
use ::uuid::{uuid, Uuid};

pub mod from;
pub mod store;
pub mod types;

pub use store::ObjectStore;
pub use types::*;

// sarzak
pub const UUID_NS: Uuid = uuid!("daccabb9-eb3a-5cde-ba7c-19a3f22ab649");

pub fn init_instances(store: &mut ObjectStore) {
    // Create instances of primitives missing from nut::sarzak that
    // the extrusion process depends upon.
    store.inter_cardinality(Cardinality::One(ONE));
    store.inter_cardinality(Cardinality::Many(MANY));

    store.inter_conditionality(Conditionality::Conditional(CONDITIONAL));
    store.inter_conditionality(Conditionality::Unconditional(UNCONDITIONAL));

    store.inter_ty(Ty::Integer(INTEGER));
    store.inter_ty(Ty::Boolean(BOOLEAN));
    store.inter_ty(Ty::Float(FLOAT));
    store.inter_ty(Ty::String(STRING));
    store.inter_ty(Ty::Uuid(UUID));
}
