///// Stage 2: Deserialize OCPP message in typed way /////
#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize)]
pub struct TypedOcppMessage {
    pub message_type: MessageType,
    pub call_id: CallId,
    pub action: String,
    pub payload: serde_json::Value,
}

use serde::de::{self, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;
use crate::ocpp::CallId;

impl<'de> Deserialize<'de> for TypedOcppMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TypedOcppMessageVisitor;

        impl<'de> Visitor<'de> for TypedOcppMessageVisitor {
            type Value = TypedOcppMessage;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an OCPP message array with 4 elements: [message_type, call_id, action, payload]")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<TypedOcppMessage, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let message_type = seq
                    .next_element::<u8>()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let call_id = seq
                    .next_element::<String>()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let action = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let payload = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                Ok(TypedOcppMessage {
                    message_type: match message_type {
                        2 => MessageType::Call,
                        3 => MessageType::CallResult,
                        4 => MessageType::CallError,
                        _ => {
                            return Err(de::Error::custom(format!(
                                "Unknown message type: {}",
                                message_type
                            )));
                        }
                    },
                    call_id: CallId(call_id),
                    action,
                    payload,
                })
            }
        }

        deserializer.deserialize_seq(TypedOcppMessageVisitor)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MessageType {
    Call = 2, //request
    CallResult = 3,
    CallError = 4,
}

#[allow(non_snake_case)]
#[cfg(test)]
mod typed_ocpp_message_tests {
    use super::*;

    #[test]
    fn given_valid_boot_notification_response__when_deserializing__then_ok() {
        let json = r#"
[
3,
"19223201",
"BootNotification",
{
"status": "Accepted",
"currentTime": "2019-08-24T14:15:22Z",
"interval": 0
}
]
"#;
        let result = serde_json::from_str::<TypedOcppMessage>(json);
        println!("{:#?}", result);
        assert!(result.is_ok());

        let expected = TypedOcppMessage {
            message_type: MessageType::CallResult,
            call_id: CallId("19223201".to_string()),
            action: "BootNotification".to_string(),
            payload: serde_json::json!({
                "status": "Accepted",
                "currentTime": "2019-08-24T14:15:22Z",
                "interval": 0
            }),
        };

        assert_eq!(result.unwrap(), expected);
    }
}
