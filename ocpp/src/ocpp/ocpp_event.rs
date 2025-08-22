///// Stage 3: Deserialize whole OCPP response /////
use crate::ocpp::CallId;
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use crate::ocpp::raw_ocpp_message::RawOcppMessage;

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize)]
pub struct OcppEvent {
    /// Call id, i.e `"19223201"`
    pub call_id: CallId,
    /// Combines `message_type`, `action` and `payload` into a single message
    pub message: OcppMessage,
}
// custom deserializer function for the format of OCPP Message
impl<'de> Deserialize<'de> for OcppEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize raw array
        let raw: Vec<Value> = Vec::deserialize(deserializer)?;

        if raw.len() != 4 {
            return Err(serde::de::Error::custom("expected 4-element OCPP array"));
        }

        let message_type = raw[0]
            .as_u64()
            .ok_or_else(|| serde::de::Error::custom("invalid message_type"))?
            as u8;

        let call_id = raw[1]
            .as_str()
            .ok_or_else(|| serde::de::Error::custom("invalid call_id"))?
            .to_string();

        let action = raw[2]
            .as_str()
            .ok_or_else(|| serde::de::Error::custom("invalid action"))?
            .to_string();

        let payload = &raw[3];

        // Normalize payload to a JSON value
        /*
        converting positional OCPP response like
        [
            ...,
            ...
            BootNotification,
            {
                "status": "Accepted",
                "currentTime": "2019-08-24T14:15:22Z",
                "interval": 0
            }
        ]

        into plain JSON object, which is supported by serde_json out of the box
        {
            "action": "BootNotification",
            "payload": {
                "status": "Accepted",
                "currentTime": "2019-08-24T14:15:22Z",
                "interval": 0
            }
        }
         */

        let tagged_payload = serde_json::json!({
            "action": action,
            "payload": payload,
        });

        // dispatch
        let message = match message_type {
            3 => {
                // Instead of writing match arms directly for each action
                // response
                // let response = match action.as_str() {
                //     "BootNotification" => {
                //         OcppResponse::BootNotification(serde_json::from_value(payload.clone()).map_err(serde::de::Error::custom)?)
                //     }
                //     "CancelReservation" => {
                //         OcppResponse::CancelReservation(serde_json::from_value(payload.clone()).map_err(serde::de::Error::custom)?)
                //     }
                //     _ => OcppResponse::Other(action, payload.clone()),
                // };
                // Instead, we can use normalized tagged payload to deserialize it automatically
                OcppMessage::Response(serde_json::from_value(tagged_payload).unwrap())
            }
            2 => {
                // call
                OcppMessage::Call(serde_json::from_value(tagged_payload).unwrap())
            }
            4 => {
                // request
                OcppMessage::Request(OcppRequest::Other(action, payload.clone()))
            }
            _ => {
                return Err(serde::de::Error::custom("unknown message_type"));
            }
        };

        Ok(OcppEvent {
            call_id: CallId(call_id),
            message,
        })
    }
}

pub fn convert(ocpp_msg : RawOcppMessage) -> OcppEvent
{
    let tagged_payload = serde_json::json!({
            "action": ocpp_msg.action,
            "payload": ocpp_msg.payload,
        });
    OcppEvent  {
        call_id: CallId(ocpp_msg.call_id),
        message: match ocpp_msg.message_type
        {
            2 => OcppMessage::Call(serde_json::from_value(tagged_payload).unwrap()),
            3 => OcppMessage::Request(serde_json::from_value(tagged_payload).unwrap()),
            4 => OcppMessage::Response(serde_json::from_value(tagged_payload).unwrap()),
            _ => { panic!("unexpected message type {}", ocpp_msg.message_type); }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum OcppMessage {
    Call(OcppCall),
    Request(OcppRequest),
    Response(OcppResponse),
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum OcppCall {
    Other(String, serde_json::Value),
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum OcppRequest {
    Other(String, serde_json::Value),
}

// TODO: Verify if it's possible to use two fields (action + payload) to deserizalize this enum automatically
#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "action", content = "payload")]
pub enum OcppResponse {
    BootNotification(BootNotificationResponse),
    CancelReservation(CancelReservationResponse),
    ClearCache(ClearCacheResponse),
    Other(String, serde_json::Value),
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationResponse {
    pub status: String,
    pub current_time: String,
    pub interval: u32,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelReservationResponse {
    pub custom_data: Option<serde_json::Value>, // Custom data can be any JSON value
    pub reservation_id: u32,
}

// [
//   3,
//   "19223201",
//   "ClearCache",
//   {
//     "customData": {
//       "vendorId": "string"
//     },
//     "status": "Accepted",
//     "statusInfo": {
//       "customData": {
//         "vendorId": "string"
//       },
//       "reasonCode": "string",
//       "additionalInfo": "string"
//     }
//   }
// ]

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearCacheResponse {
    pub custom_data: Option<serde_json::Value>, // Custom data can be any JSON value
    pub status: String,
    pub status_info: Option<StatusInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusInfo {
    pub custom_data: Option<serde_json::Value>, // Custom data can be any JSON value
    pub reason_code: String,
    pub additional_info: String,
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Error;

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

        let result = serde_json::from_str::<OcppEvent>(json);
        println!("{:#?}", result);
        assert!(result.is_ok());
        let expected = OcppEvent {
            call_id: CallId("19223201".to_string()),
            message: OcppMessage::Response(OcppResponse::BootNotification(
                BootNotificationResponse {
                    status: "Accepted".to_string(),
                    current_time: "2019-08-24T14:15:22Z".to_string(),
                    interval: 0,
                },
            )),
        };

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn given_valid_cancel_reservation_response__when_deserializing__then_ok() {
        let json = r#"
[
    3,
    "19223202",
    "CancelReservation",
    {
        "customData": null,
        "reservationId": 12345
    }
]
"#;

        let result = serde_json::from_str::<OcppEvent>(json);
        println!("{:#?}", result);
        assert!(result.is_ok());
        let expected = OcppEvent {
            call_id: CallId("19223202".to_string()),
            message: OcppMessage::Response(OcppResponse::CancelReservation(
                CancelReservationResponse {
                    custom_data: None,
                    reservation_id: 12345,
                },
            )),
        };

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn given_valid_clear_cache_response__when_deserializing__then_ok() {
        let json = r#"
[
  3,
  "19223201",
  "ClearCache",
  {
    "customData": {
      "vendorId": "string"
    },
    "status": "Accepted",
    "statusInfo": {
      "customData": {
        "vendorId": "string"
      },
      "reasonCode": "string",
      "additionalInfo": "string"
    }
  }
]
"#;

        let result = serde_json::from_str::<OcppEvent>(json);
        println!("{:#?}", result);
        assert!(result.is_ok());
        let expected = OcppEvent {
            call_id: CallId("19223201".to_string()),
            message: OcppMessage::Response(OcppResponse::ClearCache(ClearCacheResponse {
                custom_data: Some(serde_json::json!({"vendorId": "string"})),
                status: "Accepted".to_string(),
                status_info: Some(StatusInfo {
                    custom_data: Some(serde_json::json!({"vendorId": "string"})),
                    reason_code: "string".to_string(),
                    additional_info: "string".to_string(),
                }),
            })),
        };

        assert_eq!(result.unwrap(), expected);
    }

    type Datagram = String;

    /// Handles Responses and Calls.
    fn dispatcher(datagram: Datagram) -> Result<(), Box<dyn std::error::Error>> {
        // let result: Result<OcppEvent, _> = serde_json::from_str(&datagram);
        //
        // let event = match result {
        //     Ok(e) => e,
        //     Err(e) => return Err(Box::new(e)),
        // };
        let event: OcppEvent = serde_json::from_str(&datagram)?;

        let call_id = event.call_id;
        let message = event.message;

        match message {
            OcppMessage::Call(c) => handle_call(c),
            OcppMessage::Response(r) => handle_responses(r),
            OcppMessage::Request(r) => {
                return Err("Dispatch OCPP doesn't handle requets".into());
            }
        }

        Ok(())
    }

    /// Handles OCPP Call messages
    /// This function is called when a Call message is received.
    fn handle_call(c: OcppCall) {
        match c {
            OcppCall::Other(a, p) => {
                println!("TODO: Handle Call action {a}, payload: {p:?}");
            }
        }
    }

    /// Handles OCPP Response messages
    /// This function is called when a Response message is received.
    /// It matches the response type and prints the relevant information.
    /// If the response type is not recognized, it prints a TODO message.
    fn handle_responses(r: OcppResponse) {
        match r {
            OcppResponse::BootNotification(payload) => {
                println!(
                    "BootNotification response received: status: {}, current_time: {}, interval: {}",
                    payload.status, payload.current_time, payload.interval
                );
            }
            OcppResponse::CancelReservation(p) => {
                println!(
                    "CancelReservation response received: custom_data: {:?}, reservation_id: {}",
                    p.custom_data, p.reservation_id
                );
            }
            OcppResponse::ClearCache(p) => {
                println!(
                    "ClearCache response received: custom_data: {:?}, status: {}, status_info: {:?}",
                    p.custom_data, p.status, p.status_info
                );
            }
            OcppResponse::Other(a, p) => {
                println!("TODO: Handle Response action {a}, payload: {p:?}");
            }
        }
    }
}
