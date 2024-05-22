use crate::domain::Status;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Task {
    pub id: i32,
    pub story_id: i32,
    pub name: String,
    pub status: Status,
}
