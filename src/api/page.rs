use crate::{Error, Result};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// The query parameters for getting a page of domain objects from a list endpoint.
#[derive(Debug, Deserialize, Default)]
pub struct PageParams {
    pub page_token: Option<String>,
}

/// A page of domain objects
#[derive(Debug, Serialize)]
pub struct Page<T: Serialize> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
    pub data: Vec<T>,
}

impl<T: Serialize> Page<T> {
    // Create a new page of domain objects
    pub fn new(next_page: Option<String>, data: Vec<T>) -> Self {
        Self { next_page, data }
    }
}

/// A paging token for accessing previous, next pages of domain objects in a list call.
#[derive(BorshSerialize, BorshDeserialize)]
pub struct PageToken {
    pub id: i32,
    pub ts: u64,
}

impl PageToken {
    /// Encode a cursor id as a page token.
    pub fn encode(id: i32) -> Option<String> {
        if id <= 0 {
            return None;
        }
        if let Ok(bytes) = borsh::to_vec(&PageToken { id, ts: now() }) {
            Some(URL_SAFE.encode(bytes))
        } else {
            tracing::warn!("failed serializing page token: {}", id);
            None
        }
    }

    /// Extract page id from encoded token param
    pub fn decode(token_opt: &Option<String>, or_val: i32) -> Result<i32> {
        match token_opt {
            None => Ok(or_val),
            Some(token) => {
                let bytes = URL_SAFE.decode(token)?;
                let page_token: PageToken = borsh::from_slice(&bytes)
                    .map_err(|_| Error::invalid_args("invalid page token encoding"))?;
                Ok(page_token.id)
            }
        }
    }
}

/// Calculate the number of seconds since the unix epoch.
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::MAX)
        .as_secs()
}
