#![doc = include_str!("../README.md")]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]

pub mod events;
pub mod requests;
mod types;

pub use crate::types::*;

use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

/// Request is a request, with associated command, and argument and response
/// types.
pub trait Request {
    const COMMAND: &'static str;
    type Arguments: Debug + Clone + Serialize + DeserializeOwned + Send + Sync;
    type Response: Debug + Clone + Serialize + DeserializeOwned + Send + Sync;
}

/// Event is an event, with associated name and body type.
pub trait Event {
    const EVENT: &'static str;
    type Body: Debug + Clone + Serialize + DeserializeOwned + Send + Sync;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_stopped_reason() {
        let reason = StoppedEventReason::Exception;
        let serialized = serde_json::to_string(&reason).unwrap();
        assert_eq!(serialized, r#""exception""#);

        let reason = StoppedEventReason::FunctionBreakpoint;
        let serialized = serde_json::to_string(&reason).unwrap();
        assert_eq!(serialized, r#""function breakpoint""#);
    }
}
