use std::time::SystemTime;

use uuid::{uuid, Uuid};

#[derive(Debug)]
pub struct TimeStamp(SystemTime);

pub const TIME_STAMP: Uuid = uuid!["52636bac-3f47-5792-8a32-166dbe8af74f"];
