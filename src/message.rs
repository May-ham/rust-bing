use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct WebSocketMessage {
    pub arguments: Vec<Argument>,
    pub invocationId: String,
    pub target: String,
    pub type_: i32, // Note: `type` is a reserved keyword in Rust, so we use `type_`
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Argument {
    pub source: String,
    pub optionsSets: Vec<String>,
    pub allowedMessageTypes: Vec<String>,
    pub sliceIds: Vec<String>,
    pub verbosity: String,
    pub scenario: String,
    pub plugins: Vec<Plugin>,
    pub traceId: String,
    pub conversationHistoryOptionsSets: Vec<String>,
    pub isStartOfSession: bool,
    pub requestId: String,
    pub message: Message,
    pub tone: String,
    pub spokenTextMode: String,
    pub conversationId: String,
    pub participant: Participant,
}