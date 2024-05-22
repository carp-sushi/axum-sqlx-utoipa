use serde::Serialize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Story {
    pub id: i32,
    pub name: String,
}
