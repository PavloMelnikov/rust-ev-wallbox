//! OCCP (Open Charge Point Protocol) related structures and requests.

pub mod ocpp {
    use serde::{Deserialize, Serialize};

    // ----------- Request Structs -----------

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ConnectRequest {
        pub charge_point_id: String,
        pub charge_point_model: String,
        pub charge_point_vendor: String,
        pub firmware_version: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct AuthorizeRequest {
        pub id_tag: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct BootNotificationRequest {
        pub charge_point_model: String,
        pub charge_point_vendor: String,
        pub firmware_version: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct HeartbeatRequest;

    // ----------- Request Enum -----------

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(tag = "request_type", content = "payload")]
    pub enum OcppRequest {
        Connect(ConnectRequest),
        Authorize(AuthorizeRequest),
        BootNotification(BootNotificationRequest),
        Heartbeat(HeartbeatRequest),
    }

    // ----------- Handler -----------

    pub fn handle(request: OcppRequest) {
        match request {
            OcppRequest::Connect(v) => {
                println!("Handling Connect: {:?}", v);
            }
            OcppRequest::Authorize(v) => {
                println!("Handling Authorize: {:?}", v);
            }
            OcppRequest::BootNotification(v) => {
                println!("Handling BootNotification: {:?}", v);
            }
            OcppRequest::Heartbeat(v) => {
                println!("Handling Heartbeat: {:?}", v);
            }
        }
    }

    // pub fn do_request() {
    //     let auth_request_payload = AuthorizeRequest {
    //         id_tag: "123456".to_string(),
    //     };
    //     let auth_request = OcppRequest::Authorize(auth_request_payload);
    //     handle(auth_request);
    // }

    // ----------- Tests -----------

    #[cfg(test)]
    mod serde_tests {
        use super::*;
        use serde_json;

        #[test]
        fn test_serialize_connect_request() {
            let connect_request = ConnectRequest {
                charge_point_id: "CP123".to_string(),
                charge_point_model: "ModelX".to_string(),
                charge_point_vendor: "VendorY".to_string(),
                firmware_version: Some("1.0.0".to_string()),
            };

            let result = serde_json::to_string(&OcppRequest::Connect(connect_request));
            assert!(result.is_ok());
            let serialized = result.unwrap();
            println!("{}", serialized);
            assert!(serialized.contains("\"request_type\":\"Connect\""));
            assert!(serialized.contains("\"charge_point_id\":\"CP123\""));
        }

        #[test]
        fn test_deserialize_connect_request() {
            let raw = r#"{
                "request_type": "Connect",
                "payload": {
                    "charge_point_id": "CP123",
                    "charge_point_model": "ModelX",
                    "charge_point_vendor": "VendorY",
                    "firmware_version": "1.0.0"
                }
            }"#;

            let request: OcppRequest = serde_json::from_str(raw).unwrap();
            match request {
                OcppRequest::Connect(req) => {
                    assert_eq!(req.charge_point_id, "CP123");
                }
                _ => panic!("Expected Connect request"),
            }
        }
    }

    // ----------- Message Metadata -----------

    #[derive(Debug)]
    pub struct CallId(pub String);

    #[derive(Debug)]
    pub enum MessageType {
        Call = 2,
        CallResult = 3,
        CallError = 4,
    }
}
