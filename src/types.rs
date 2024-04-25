use core::f32;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub enum EventType {
    Message,
    MessageEdit,
    MessageDelete,
    UserJoin,
    UserLeave,
    UserEdit
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct Event {
    pub event_type: EventType,
    pub id: Option<String>,
    pub message: Option<Message>,
    pub user: Option<String>
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct User {
    pub username: String,
    pub password: String,
    pub uuid: String,
    pub pfp: String,
    pub email: String,
    pub bio: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct Message {
    pub author: String,
    pub content: String,
    pub timestamp: String,
    pub channel: String,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct StatusResponse {
    pub db: bool,
    pub cpu: f32,
    pub mem: u64,
    pub redis: bool,
    pub tornado: bool
    // Add more things here as we need them
}
