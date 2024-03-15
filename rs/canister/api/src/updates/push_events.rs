use crate::IdempotentEventPrevious;
use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct PushEventsArgsPrevious {
    pub events: Vec<IdempotentEventPrevious>,
}
