///// Stage 1: Deserialize a raw OCPP message /////
#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RawOcppMessage {
    pub message_type: u8,
    pub call_id: String,
    pub action: String,
    pub payload: serde_json::Value,
}

// macro is used for all the tests
macro_rules! ocpp_test {
    ($name:ident, $action:expr, $payload:expr) => {
        #[test]
        fn $name() {
            let json = serde_json::json!([
                3,
                "19223201",
                $action,
                $payload
            ])
            .to_string();

            let result = serde_json::from_str::<RawOcppMessage>(&json);
            assert!(result.is_ok());

            let expected = RawOcppMessage {
                message_type: 3,
                call_id: "19223201".to_string(),
                action: $action.to_string(),
                payload: $payload,
            };

            assert_eq!(result.unwrap(), expected);
        }
    };
}


//test serialization of all event types



#[allow(non_snake_case)]
#[cfg(test)]
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

        let result = serde_json::from_str::<RawOcppMessage>(json);
        println!("{:#?}", result);
        assert!(result.is_ok());
        let expected = RawOcppMessage {
            message_type: 3,
            call_id: "19223201".to_string(),
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
