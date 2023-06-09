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

#[derive(Debug, Serialize, Deserialize)]
pub struct DummyRequest {
    pub text: Option<String>,
    pub msg: String,
}

impl DummyRequest {
    pub fn create(t: Option<String>, m: Option<String>) -> Self {
        let msg = match m {
            Some(val) => val,
            None => String::from(""),
        };

        Self { text: t, msg }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DummyResponse {
    text: Option<String>,
    msg: String,
}

impl DummyResponse {
    pub fn create(t: Option<String>, m: Option<String>) -> Self {
        let msg = match m {
            Some(val) => val,
            None => String::from(""),
        };

        Self { text: t, msg }
    }
}
