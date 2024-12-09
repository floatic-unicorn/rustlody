use std:: fmt;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct ErrorResponse{ 
    message: String
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error message = {}", self.message)
    }
}

#[derive(Deserialize)]
pub struct GetIdentifyRepresentativeInvoiceBarcodeResponse {
    #[serde(rename = "waveId")]
    wave_id: String,
    #[serde(rename = "waveName")]
    wave_name: String,
    #[serde(rename = "boxType")]
    box_type: String,
    #[serde(rename = "maxBoxCount")]
    max_box_count: i32,
    #[serde(rename = "desiredBoxCount")]
    desired_box_count: i32,
    affinity: String,
    #[serde(rename = "sellingChannel")]
    selling_channel: String
}

impl fmt::Display for GetIdentifyRepresentativeInvoiceBarcodeResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[RES] | [GET] identify representative invoice barcode wave.name = {}", self.wave_name)
    }
}
