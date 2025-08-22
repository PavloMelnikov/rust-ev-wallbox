//! OCCP (Open Charge Point Protocol) related structures and requests.

use crate::ocpp_mod::ocpp as ocpp_internal;
mod ocpp;
mod ocpp_mod;

#[cfg(test)]
mod serde_tests {
    use super::*;

    #[test]
    fn test_serialize_connect_request() {
        let connect_request = ocpp_internal::ConnectRequest {
            charge_point_id: "CP123".to_string(),
            charge_point_model: "".to_string(),
            charge_point_vendor: "".to_string(),
            firmware_version: None,
        };

        let result = serde_json::to_string(&ocpp_internal::OcppRequest::Connect(connect_request));
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

        let request: ocpp_internal::OcppRequest = serde_json::from_str(raw).unwrap();
    }
}



fn main() {
    let connect = ocpp_internal::ConnectRequest {
        charge_point_id: "CP001".into(),
        charge_point_model: "ModelZ".into(),
        charge_point_vendor: "VendorX".into(),
        firmware_version: None,
    };

    let request = ocpp_internal::OcppRequest::Connect(connect);
    // ocpp::handle(request);
}