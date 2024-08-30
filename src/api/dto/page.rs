use crate::{Error, Result};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::Deserialize;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// Define a reasonable default page size.
const DEFAULT_PAGE_SIZE: i32 = 100;

// Page size limits
const MIN_PAGE_SIZE: i32 = 10;
const MAX_PAGE_SIZE: i32 = 1000;

/// The query parameters for getting a page of domain objects from a list endpoint.
#[derive(Debug, Deserialize, Default)]
pub struct PageParams {
    pub page_size: Option<i32>,
    pub page_token: Option<String>,
}

impl PageParams {
    pub fn page_size(&self) -> i32 {
        self.page_size
            .unwrap_or(DEFAULT_PAGE_SIZE)
            .clamp(MIN_PAGE_SIZE, MAX_PAGE_SIZE)
    }
}

/// A paging token for accessing previous, next pages of domain objects in a list call.
#[derive(BorshSerialize, BorshDeserialize)]
pub struct PageToken {
    cursor: i64,
    ts: u64,
}

impl PageToken {
    /// Encode a cursor seqno as a page token.
    pub fn encode(cursor: i64) -> Option<String> {
        if cursor <= 0 {
            return None;
        }
        match borsh::to_vec(&PageToken { cursor, ts: now() }) {
            Ok(bytes) => Some(URL_SAFE.encode(bytes)),
            Err(err) => {
                tracing::warn!("failed serializing page token: {}", err);
                None
            }
        }
    }

    /// Extract page cursor from encoded token param, falling back to a default value.
    pub fn decode_or(token_opt: &Option<String>, default: i64) -> Result<i64> {
        if default <= 0 {
            return Err(Error::invalid_args("default page cursor must be > 0"));
        }
        match token_opt {
            None => Ok(default),
            Some(token) => {
                let bytes = URL_SAFE.decode(token)?;
                let page_token: PageToken = borsh::from_slice(&bytes)?;
                Ok(page_token.cursor)
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
    fn encode_invalid_page_cursor() {
        assert!(PageToken::encode(0).is_none());
        assert!(PageToken::encode(-10).is_none());
    }

    #[test]
    fn decode_default() {
        // Should get default when page token is None
        let expect = i64::MAX;
        let output = PageToken::decode_or(&None, expect).unwrap();
        assert_eq!(output, expect);
        // Invalid defaults should produce an error
        assert!(PageToken::decode_or(&None, 0).is_err());
        assert!(PageToken::decode_or(&None, -10).is_err());
    }
}
