use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone, Hash)]
pub struct User{
    pub (crate) id: u32,
    pub (crate) username: String,
    pub (crate) password: String
}