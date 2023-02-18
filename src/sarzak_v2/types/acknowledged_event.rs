// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"acknowledged_event-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"acknowledged_event-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::sarzak_v2::UUID_NS;

// Referent imports
use crate::sarzak_v2::types::event::Event;
use crate::sarzak_v2::types::state::State;

use crate::sarzak_v2::store::ObjectStore as SarzakV2Store;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"acknowledged_event-struct-documentation"}}}
/// An Event that Does Something
///
/// An acknowledged event is an event that a [`State`] knows how to handle.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"acknowledged_event-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AcknowledgedEvent {
    pub id: Uuid,
    /// R20: [`Event`] '🚧 Out of order — see sarzak#14.' [`Event`]
    pub event_id: Uuid,
    /// R20: [`State`] '🚧 Out of order — see sarzak#14.' [`State`]
    pub state_id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"acknowledged_event-implementation"}}}
impl AcknowledgedEvent {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"acknowledged_event-struct-impl-new"}}}
    /// Inter a new AcknowledgedEvent in the store, and return it's `id`.
    pub fn new(event_id: &Event, state_id: &State, store: &mut SarzakV2Store) -> AcknowledgedEvent {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{:?}:{:?}", event_id, state_id).as_bytes(),
        );
        let new = AcknowledgedEvent {
            event_id: event_id.id,
            state_id: state_id.id,
            id,
        };
        store.inter_acknowledged_event(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"acknowledged_event-struct-impl-nav-forward-assoc-to-event_id"}}}
    /// Navigate to [`Event`] across R20(1-?)
    pub fn r20_event<'a>(&'a self, store: &'a SarzakV2Store) -> Vec<&Event> {
        vec![store.exhume_event(&self.event_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"acknowledged_event-struct-impl-nav-forward-assoc-to-state_id"}}}
    /// Navigate to [`State`] across R20(1-?)
    pub fn r20_state<'a>(&'a self, store: &'a SarzakV2Store) -> Vec<&State> {
        vec![store.exhume_state(&self.state_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
