pub mod utils {
    use crate::models::user::User;
    
    pub fn vector_to_json(user: &Vec<User>) -> Result<String, std::io::Error> {
        let serialized_data = serde_json::to_string(&user)?;

        Ok(serialized_data)
    }
}