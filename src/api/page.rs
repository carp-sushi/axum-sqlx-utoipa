use crate::{Error, Result};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// Define a reasonable default page size.
const DEFAULT_PAGE_SIZE: i32 = 100;

// Page size limits
const MIN_PAGE_SIZE: i32 = 10;
const MAX_PAGE_SIZE: i32 = 1000;

/// The query parameters for getting a page of domain objects from a list endpoint.
#[derive(Debug, Deserialize, Default)]
pub(crate) struct PageParams {
    pub page_token: Option<String>,
    pub page_size: Option<i32>,
}

impl PageParams {
    pub fn page_size(&self) -> i32 {
        self.page_size
            .unwrap_or(DEFAULT_PAGE_SIZE)
            .clamp(MIN_PAGE_SIZE, MAX_PAGE_SIZE)
    }
}

/// A page of domain objects
#[derive(Debug, Serialize)]
pub(crate) struct Page<T: Serialize> {
    #[serde(skip_serializing_if = "Option::is_none")]
    next_page: Option<String>,
    data: Vec<T>,
}

impl<T: Serialize> Page<T> {
    // Create a new page of domain objects
    pub fn new(next_page: Option<String>, data: Vec<T>) -> Self {
        Self { next_page, data }
    }
}

/// A paging token for accessing previous, next pages of domain objects in a list call.
#[derive(BorshSerialize, BorshDeserialize)]
pub(crate) struct PageToken {
    id: i32,
    ts: u64,
}

impl PageToken {
    /// Encode a cursor id as a page token.
    pub fn encode(id: i32) -> Option<String> {
        if id <= 0 {
            return None;
        }
        match borsh::to_vec(&PageToken { id, ts: now() }) {
            Ok(bytes) => Some(URL_SAFE.encode(bytes)),
            Err(err) => {
                tracing::warn!("failed serializing page token: {}", err);
                None
            }
        }
    }

    /// Extract page id from encoded token param, falling back to a default value.
    pub fn decode_or(token_opt: &Option<String>, default: i32) -> Result<i32> {
        if default <= 0 {
            return Err(Error::invalid_args("default page id must be > 0"));
        }
        match token_opt {
            None => Ok(default),
            Some(token) => {
                let bytes = URL_SAFE.decode(token)?;
                let page_token: PageToken = borsh::from_slice(&bytes)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_page_token() {
        let input = 5201;
        let pt = PageToken::encode(input);
        assert!(pt.is_some());
        let output = PageToken::decode_or(&pt, 1).unwrap();
        assert_eq!(input, output);
    }

    #[test]
    fn encode_invalid_page_id() {
        assert!(PageToken::encode(0).is_none());
        assert!(PageToken::encode(-10).is_none());
    }

    #[test]
    fn decode_default() {
        let fallback = i32::MAX;
        let output = PageToken::decode_or(&None, fallback).unwrap();
        assert_eq!(fallback, output);
    }
}
