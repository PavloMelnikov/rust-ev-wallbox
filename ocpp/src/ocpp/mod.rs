mod raw_ocpp_message;
mod typed_ocpp_message;
mod ocpp_event;
mod types;

/// Wrapper struct for CallId to not confuse it with any other string
#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CallId(String);

