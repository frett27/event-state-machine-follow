
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Evaluation result types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventValue {
    String(String),
    Integer(i64),
    Boolean(bool),
    Float(f64),
    Null,
}


// this structure is the generic definition of an informing element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub is_state: bool,
    pub attributes: HashMap<String, EventValue>,
    pub timestamp: u64,
}

impl Event {
    pub fn new(
        name: String,
        is_state: bool,
        attributes: HashMap<String, EventValue>,
        timestamp: u64,
    ) -> Self {
        Self {
            name,
            is_state,
            attributes,
            timestamp,
        }
    }
}
