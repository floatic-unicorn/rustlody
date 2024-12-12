use serde::Deserialize;


#[derive(Deserialize)]
struct DesiredHeader {
    r#type: String,
    #[serde(rename = "robotId")]
    robot_id: String,
    #[serde(rename = "commandId")]
    command_id: String,
    timestamp: i64,
}

#[derive(Deserialize)]
struct DesiredDestination {
    r#type: String,
    value: String
}

#[derive(Deserialize)]
pub struct DesiredPayload {
    pub state: String,
    destination: DesiredDestination,
}

#[derive(Deserialize)]
pub struct DesiredMessage {
    header: DesiredHeader,
    pub payload: DesiredPayload,
}
