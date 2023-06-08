use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexResponse {
    msg: String,
}

impl IndexResponse {
    pub fn create(m: Option<String>) -> Self {
        let msg = match m {
            Some(val) => val,
            None => String::from(""),
        };

        Self { msg }
    }
}
