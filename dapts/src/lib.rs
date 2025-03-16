#![doc = include_str!("../README.md")]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]

pub mod event;
pub mod request;
mod types;

pub use crate::types::*;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// Request is a request, with associated command, and argument and response
/// types.
pub trait IRequest {
    const COMMAND: &'static str;
    type Arguments: DeserializeOwned + Serialize + Send + Sync + 'static;
    type Response: DeserializeOwned + Serialize + Send + Sync + 'static;
}

/// Event is an event, with associated name and body type.
pub trait IEvent {
    const EVENT: &'static str;
    type Body: DeserializeOwned + Serialize + Send + Sync + 'static;
}

/// Represents a request from the client.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Request {
    /// Sequence number for the Request.
    ///
    /// From the [specification](https://microsoft.github.io/debug-adapter-protocol/specification#Base_Protocol_ProtocolMessage):
    ///
    /// Sequence number of the message (also known as message ID). The `seq` for
    /// the first message sent by a client or debug adapter is 1, and for each
    /// subsequent message is 1 greater than the previous message sent by that
    /// actor. `seq` can be used to order requests, responses, and events, and
    /// to associate requests with their corresponding responses. For
    /// protocol messages of type `request` the sequence number can be used
    /// to cancel the request.
    pub seq: i64,
    /// The command to execute.
    pub command: String,
    /// The command to execute.
    #[serde(default = "serde_json::Value::default")]
    #[serde(skip_serializing_if = "serde_json::Value::is_null")]
    pub arguments: serde_json::Value,
}

impl Request {
    /// Creates a new request.
    pub fn new(seq: i64, command: String, arguments: impl serde::Serialize) -> Request {
        Request {
            seq,
            command,
            arguments: serde_json::to_value(arguments).unwrap(),
        }
    }
}

/// Represents response to the client.
///
/// The command field (which is a string) is used as a tag in the ResponseBody
/// enum, so users of this crate will control it by selecting the appropriate
/// enum variant for the body.
///
/// There is also no separate `ErrorResponse` struct. Instead, `Error` is just a
/// variant of the ResponseBody enum.
///
/// Specification: [Response](https://microsoft.github.io/debug-adapter-protocol/specification#Base_Protocol_Response)
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// Sequence number of the corresponding request.
    #[serde(rename = "request_seq")]
    pub request_seq: i64,
    /// Outcome of the request.
    /// If true, the request was successful and the `body` attribute may contain
    /// the result of the request.
    /// If the value is false, the attribute `message` contains the error in
    /// short form and the `body` may contain additional information (see
    /// `ErrorResponse.body.error`).
    pub success: bool,
    /// Contains the raw error in short form if `success` is false.
    /// This raw error might be interpreted by the client and is not shown in
    /// the UI.
    /// Some predefined values exist.
    /// Values:
    /// 'cancelled': request was cancelled.
    /// etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Contains request result if success is true and error details if success
    /// is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
}

impl Response {
    /// Creates a new response.
    pub fn new(
        request_seq: i64,
        success: bool,
        message: Option<String>,
        body: Option<impl serde::Serialize>,
    ) -> Response {
        Response {
            request_seq,
            success,
            message,
            body: body.map(|b| serde_json::to_value(b).unwrap()),
        }
    }

    /// Creates a new successful response.
    pub fn success(request_seq: i64, body: impl serde::Serialize) -> Response {
        Response {
            request_seq,
            success: true,
            message: None,
            body: Some(serde_json::to_value(body).unwrap()),
        }
    }

    /// Creates a new error response.
    pub fn error(request_seq: i64, message: Option<String>, detail: Option<Message>) -> Response {
        #[derive(Serialize)]
        struct ErrorResponseBody {
            /// A structured error message.
            error: Message,
        }

        Response {
            request_seq,
            success: false,
            message,
            body: detail.map(|error| serde_json::to_value(&ErrorResponseBody { error }).unwrap()),
        }
    }
}

/// Represents an event from the client.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    /// Sequence number for the Request.
    ///
    /// From the [specification](https://microsoft.github.io/debug-adapter-protocol/specification#Base_Protocol_ProtocolMessage):
    ///
    /// Sequence number of the message (also known as message ID). The `seq` for
    /// the first message sent by a client or debug adapter is 1, and for each
    /// subsequent message is 1 greater than the previous message sent by that
    /// actor. `seq` can be used to order requests, responses, and events, and
    /// to associate requests with their corresponding responses. For
    /// protocol messages of type `request` the sequence number can be used
    /// to cancel the request.
    pub seq: i64,
    /// Type of event.
    pub event: String,
    /// Event-specific information.
    #[serde(default = "serde_json::Value::default")]
    #[serde(skip_serializing_if = "serde_json::Value::is_null")]
    pub body: serde_json::Value,
}

impl Event {
    /// Creates a new event.
    pub fn new(seq: i64, event: String, body: impl serde::Serialize) -> Event {
        Event {
            seq,
            event,
            body: serde_json::to_value(body).unwrap(),
        }
    }
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
