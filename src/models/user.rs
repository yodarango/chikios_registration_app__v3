use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User{
    pub ID: u64,
    pub signature: String,
    // pub guardian_first_name: String,
    // pub guardian_last_name: String,
    // pub guardian_phone: String,
    // pub first_name: String,
    // pub last_name: String,
    // pub age: u8,
}
