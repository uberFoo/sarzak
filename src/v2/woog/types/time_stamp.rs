//! Time Stamp External Entity
//!
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"time_stamp-ee-use-statements"}}}
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

use crate::v2::woog::UUID_NS;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"time_stamp-ee-documentation"}}}
/// A timestamp with nanosecond precision
///
/// There are all sorts of caveats about using this in the documentation. I should include
///a pointer to it here.
///
/// 🐶 {"external_entity": {"name": "SystemTime", "ctor": "now", "path": "std::time"}}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"time_stamp-ee-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TimeStamp {
    pub id: Uuid,
    pub value: SystemTime,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"time_stamp-ee-impl"}}}
impl TimeStamp {
    pub fn new() -> Self {
        let value = SystemTime::now();
        Self {
            id: Uuid::new_v5(&UUID_NS, format!("{:?}", value).as_bytes()),
            value,
        }
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
