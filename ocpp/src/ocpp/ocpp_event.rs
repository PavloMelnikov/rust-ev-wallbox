use std::error::Error;
///// Stage 3: Deserialize whole OCPP response /////
use crate::ocpp::CallId;
use crate::ocpp::raw_ocpp_message::RawOcppMessage;
use serde::{Deserializer};
use super::types



#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize)]
pub struct OcppEvent {
    /// Call id, i.e `"19223201"`
    pub call_id: CallId,
    /// Combines `message_type`, `action` and `payload` into a single message
    pub message: OcppMessage,
}
/// converter function from Raw Format to OCPP Event Format
pub fn convert(ocpp_msg: RawOcppMessage) -> Result<OcppEvent, Box<dyn Error>> {
    println!("{:#?}", ocpp_msg);
    let tagged_payload = serde_json::json!({
        "action": ocpp_msg.action,
        "payload": ocpp_msg.payload,
    });
    let event = OcppEvent {
        call_id: CallId(ocpp_msg.call_id),
        message: match ocpp_msg.message_type {
            2 => OcppMessage::Request(serde_json::from_value(tagged_payload)?),
            3 => OcppMessage::Response(serde_json::from_value(tagged_payload)?),
            _ => {
                let err = format!("unexpected message type {}", ocpp_msg.message_type).into();
                return Err(err);
            }
        },
    };
    Ok(event)
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
    ChangeAvailability(ChangeAvailabilityResponse),
    ChangeConfiguration(ChangeConfigurationResponse),
    ClearChargingProfile(ClearChargingProfileResponse),
    DataTransfer(DataTransferResponse),
    DiagnosticsStatusNotification(DiagnosticsStatusNotificationResponse),
    FirmwareStatusNotification(FirmwareStatusNotificationResponse),
    GetCompositeSchedule(GetCompositeScheduleResponse),
    GetConfiguration(GetConfigurationResponse),
    GetDiagnostics(GetDiagnosticsResponse),
    GetLocalListVersion(GetLocalListVersionResponse),
    Heartbeat(HeartbeatResponse),
    MeterValues(MeterValuesResponse),
    RemoteStartTransaction(RemoteStartTransactionResponse),
    RemoteStopTransaction(RemoteStopTransactionResponse),
    ReserveNow(ReserveNowResponse),
    Reset(ResetResponse),
    SendLocalList(SendLocalListResponse),
    SetChargingProfile(SetChargingProfileResponse),
    StartTransaction(StartTransactionResponse),
    StatusNotification(StatusNotificationResponse),
    StopTransaction(StopTransactionResponse),
    TriggerMessage(TriggerMessageResponse),
    UnlockConnector(UnlockConnectorResponse),
    UpdateFirmware(UpdateFirmwareResponse)
    Other(String, serde_json::Value),
    () => {};
}

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

pub struct ChangeAvailabilityResponse {
    pub status: String,
    pub current_time: String,
    pub interval: u32,
}

pub struct ChangeConfigurationResponse {

}

pub struct ClearChargingProfileResponse {

}

pub struct DataTransferResponse {

}

pub struct DiagnosticsStatusNotificationResponse {

}

pub struct FirmwareStatusNotificationResponse {

}

pub struct GetCompositeScheduleResponse {

}

pub struct GetConfigurationResponse {

}

pub struct GetDiagnosticsResponse {

}

pub struct GetLocalListVersionResponse {

}

pub struct HeartbeatResponse {

}

pub struct MeterValuesResponse {

}

pub struct RemoteStartTransactionResponse {

}

pub struct RemoteStopTransactionResponse {

}

pub struct ReserveNowResponse {

}

pub struct ResetResponse {

}

pub struct SendLocalListResponse {

}

pub struct SetChargingProfileResponse {

}

pub struct StartTransactionResponse {

}

pub struct StatusNotificationResponse {

}

pub struct StopTransactionResponse {

}

pub struct TriggerMessageResponse {

}

pub struct UnlockConnectorResponse {
    status : String,
}

pub struct UpdateFirmwareResponse {

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
    use serde_json::{Error, json};

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

        let result = serde_json::from_str::<RawOcppMessage>(json).unwrap();
        let result = convert(result);
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

        let result = serde_json::from_str::<RawOcppMessage>(json).unwrap();
        let result = convert(result);
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

        let result = serde_json::from_str::<RawOcppMessage>(json).unwrap();
        let result = convert(result);
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

    #[test]
    fn given_valid_raw_ocpp_message_when_convert__then_ok() {
        // let mut map = serde_json::Map::new();
        // map.insert("status".to_string(),Value::String("Accepted".to_string()));
        // map.insert("currentTime".to_string(),Value::String("12345".to_string()));
        // map.insert("interval".to_string(),Value::Number(serde_json::Number::from(1234)));

        let rawOcppMessage = RawOcppMessage {
            message_type: 3,
            call_id: "192232".to_string(),
            action: "BootNotification".to_string(),
            // payload: Value::Object(map)
            payload: json!({
                    "status": "Accepted",
                    "currentTime": "2019-08-24T14:15:22Z",
                    "interval": 0,
            }),
        };

        let expected = OcppEvent {
            call_id: CallId("192232".to_string()),
            message: OcppMessage::Response(OcppResponse::BootNotification(
                BootNotificationResponse {
                    status: "Accepted".to_string(),
                    current_time: "2019-08-24T14:15:22Z".to_string(),
                    interval: 0,
                },
            )),
        };
        let ocppEvent = convert(rawOcppMessage).unwrap();

        assert_eq!(ocppEvent, expected);
    }
}
