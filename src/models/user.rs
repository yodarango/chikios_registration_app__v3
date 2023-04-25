use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User{
    pub ID: u64,
    pub signature: String,
    // guardian_first_name: String,
    // guardian_last_name: String,
    // guardian_phone: String,
    // first_name: String,
    // last_name: String,
    // age: u8,
}
