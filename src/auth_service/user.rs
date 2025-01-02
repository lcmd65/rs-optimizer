use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub struct User{
    pub (crate) username: String,
    pub (crate) password: String
}