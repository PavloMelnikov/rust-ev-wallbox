//! OCCP (Open Charge Point Protocol) related structures and requests.

use crate::ocpp_mod::ocpp as ocpp_internal;
mod ocpp;
mod ocpp_mod;

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