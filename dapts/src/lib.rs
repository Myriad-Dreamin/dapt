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
