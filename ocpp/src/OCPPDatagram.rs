#[derive(Subcommand)]
enum MessageType {
    Call,
    CallResult,
    CallError
}

struct OCPPDatagram
{

    /*
    public:
    QString m_Id; ///< Unique message ID, maximum length of 36 characters, to allow for UUIDs/GUIDs
    MessageType m_Type; ///< Enum to identify the type of message
    nlohmann::json m_Payload; ///< Payload of the Action or Result; in case of an CALLERROR it contains the ErrorDetails.
    QString m_Action; ///< The Action field in the CALL message MUST is the OCPP message name without the "Request" suffix. Case-sensitive. For
                      ///< CALLRESULT and CALLERROR the action is set by the proxy that relayed the request.
    QString m_ErrorCode; ///< Only for CALLERROR
    QString m_ErrorDescription; ///< Only for CALLERROR
    */
}

pub fn new() -> Self {
    Self {
        id: String::new(),
        msg_type: MessageType::Call,
        payload: Value::Null,
        action: String::new(),
        error_code: String::new(),
        error_description: String::new(),
    }
}