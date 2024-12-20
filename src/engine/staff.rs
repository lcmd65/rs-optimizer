

use serde_json::Deserializer;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Eq)]
pub struct Coverage {
    pub(crate) id: String,
    pub(crate) staff_groups: Vec<String>,
    pub(crate) shift: Vec<String>,
    pub(crate) desire_value: i8,
    pub(crate) day: i8,
    pub(crate) day_type: String,
    pub(crate) priority: i8,
    pub(crate) penalty: i8,
    pub(crate) types: Vec<String>,
}

impl Coverage {
    pub fn shift_random(&self) -> String {
        let mut rng = rand::thread_rng(); // Create an internal RNG
        self.shift.choose(&mut rng).unwrap().to_string()
    }
}