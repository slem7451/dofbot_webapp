use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Servo {
    pub angle: String,
    pub servo: String,
}

#[derive(Serialize)]
pub struct AjaxResult {
    pub status: String,
    pub response: String,
}
