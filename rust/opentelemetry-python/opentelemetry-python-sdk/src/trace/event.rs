use std::collections::HashMap;

use crate::value::Value;

pub(super) struct Event {
    pub(super) name: String,
    pub(super) timestamp: i64,
    pub(super) attributes: HashMap<String, Value>,
}

impl Event {
    pub(super) fn new(
        name: String,
        timestamp: Option<i64>,
        attributes: HashMap<String, Value>,
    ) -> Self {
        Event {
            name,
            timestamp: timestamp
                .unwrap_or_else(|| std::time::UNIX_EPOCH.elapsed().unwrap().as_nanos() as i64),
            attributes,
        }
    }
}
