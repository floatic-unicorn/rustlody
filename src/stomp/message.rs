use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WsRobotInProgress {
    #[serde(rename = "robotId")]
    robot_id: String,
    status: String,
    #[serde(rename = "inProgressPickings")]
    pub in_progress_pickings: Vec<WsPickingCmd>,
    #[serde(rename = "inProgressUnloadings")]
    in_progress_unloadings: Vec<WsUnloadingCmd>,
    #[serde(rename = "waveMetadata")]
    wave_metadata: Option<WsWaveMetadata>,
    #[serde(rename = "emergencyError")]
    emergency_error: Option<String>,
    #[serde(rename = "batteryPercent")]
    battery_percent: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct WsPickingCmd {
    floor: i32,
    sequence: i32,
    #[serde(rename = "pickingId")]
    pub picking_id: String,
    #[serde(rename = "workGroupId")]
    work_group_id: String,
    #[serde(rename = "workId")]
    work_id: String,
    #[serde(rename = "wmsSkuId")]
    wms_sku_id: String,
    #[serde(rename = "skuName")]
    sku_name: String,
    //pickier: Optionn<String>,
    #[serde(rename = "locationCode")]
    location_code: String,
    #[serde(rename = "skuBarcode")]
    sku_barcode: String,
    #[serde(rename = "requestQuantity")]
    request_quantity: i32,
    #[serde(rename = "isTamperEvident")]
    is_tamper_evident: bool,
}

#[derive(Debug, Deserialize)]
struct WsUnloadingCmd {
    floor: i32,
    sequence: i32,
    #[serde(rename = "pickingId")]
    picking_id: String,
    #[serde(rename = "workGroupId")]
    work_group_id: String,
    #[serde(rename = "shortfallReasons")]
    shortfall_reasons: Vec<String>,
    #[serde(rename = "unloadingStationId")]
    unloading_station_id: i64,
    #[serde(rename = "locationCode")]
    location_code: String,
    #[serde(rename = "isTamperEvident")]
    is_tamper_evident: bool,
}

#[derive(Debug, Deserialize)]
struct WsWaveMetadata {
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
    selling_channel: String,
}

#[derive(Debug, Deserialize)]
struct WsEmergecyError {
    #[serde(rename = "recoveryStatus")]
    recovery_status: String,
    problems: Vec<String>,
    solutions: Vec<String>,
}
