use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Servo {
    pub angle: String,
    pub servo: String,
}

#[derive(Deserialize)]
pub struct Pose {
    pub pose: String,
    pub servo6: String,
}

#[derive(Deserialize)]
pub struct State {
    pub state: String,
}

#[derive(Serialize)]
pub struct AjaxResult {
    pub status: String,
    pub response: String,
}
