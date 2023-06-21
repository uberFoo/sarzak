//! Time Stamp External Entity
//!
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"time_stamp-ee-use-statements"}}}
use crate::v2::woog::store::ObjectStore as WoogStore;
use crate::v2::woog::UUID_NS;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"time_stamp-ee-documentation"}}}
/// A timestamp with nanosecond precision
///
/// There are all sorts of caveats about using this in the documentation. I should include
/// a pointer to it here.
///
/// 🐶 {"external_entity": {"name": "SystemTime", "ctor": "now", "path": "std::time"}}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"time_stamp-ee-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TimeStamp {
    pub id: Uuid,
    inner: SystemTime,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"time_stamp-ee-impl"}}}
impl TimeStamp {
    pub fn new(store: &mut WoogStore) -> TimeStamp {
        let inner = SystemTime::now();
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}", inner).as_bytes());
        let new = TimeStamp {
            id,
            inner,
        };
        store.inter_time_stamp(new.clone());
        new
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
