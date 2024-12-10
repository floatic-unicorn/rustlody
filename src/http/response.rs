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

#[derive(Debug, Deserialize)]
struct PickingSku {
    #[serde(rename = "wmsSkuId")]
    wms_sku_id: String,
    #[serde(rename = "locationCode")]
    location_code: String,
    #[serde(rename = "skuBarcode")]
    sku_barcode: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct PickingQuantity {
    #[serde(rename = "requestQuantity")]
    request_quantity: i32,
    #[serde(rename = "responseQuantity")]
    response_quantity: Option<i32>,
    #[serde(rename = "shortfallReason")]
    shortfall_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BoxLocation {
    floor: i32,
    sequence: i32,
}

#[derive(Debug, Deserialize)]
pub struct PickingDto{
    #[serde(rename = "pickingId")]
    pub picking_id: String,
    #[serde(rename = "workGroupId")]
    pub workgroup_id: String,
    #[serde(rename = "workId")]
    work_id: String,
    #[serde(rename = "robotId")]
    robot_id: String,
    #[serde(rename = "pickingSku")]
    picking_sku: PickingSku,
    #[serde(rename = "pickingQuantity")]
    picking_quantity: PickingQuantity,
    #[serde(rename = "boxLocation")]
    box_location: BoxLocation,
    #[serde(rename = "startedAt")]
    started_at: Option<f64>,
    #[serde(rename = "robotArrivedAt")]
    robot_arrived_at: Option<f64>,
    #[serde(rename = "workerArrivedAt")]
    worker_arrived_at: Option<f64>,
    #[serde(rename = "pickerArrivedAt")]
    pickier_arrived_at: Option<f64>,
    #[serde(rename = "pickedAt")]
    picked_at: Option<f64>,
    #[serde(rename = "createdAt")]
    created_at: Option<f64>,
    #[serde(rename = "updatedAt")]
    updated_at: Option<f64>,
    #[serde(rename = "canceledAt")]
    canceld_at: Option<f64>,
    priority: i64,
    status: String,
}

impl fmt::Display for PickingDto {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[RES] | [GET] same location pickings work.id= {}", self.work_id)
    }
}

#[derive(Deserialize)]
pub struct GetAllAssignedPickingsResponse{
    pub pickings: Vec<PickingDto>
}

#[derive(Debug, Deserialize)]
pub struct UnloadingCmd {
    #[serde(rename = "workGroupId")]
    workgroup_id: String,
    #[serde(rename = "shortfallReason")]
    shortfall_reason: Option<String>,
    #[serde(rename = "unloadingStationId")]
    unloading_station_id: String,
    #[serde(rename = "locationCode")]
    location_code: String,
    #[serde(rename = "isTamperEvident")]
    is_tamper_evident: bool,
    floor: i32,
    sequence: i32,
}

#[derive(Debug, Deserialize)]
pub struct GetUnloadingsTotalResponse{
    #[serde(rename = "inProgresses")]
    pub in_progresses: Vec<UnloadingCmd>,
    pub queue: Vec<UnloadingCmd>,
}
