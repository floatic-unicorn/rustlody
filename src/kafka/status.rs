use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct StatusMessage {
    header: StatusHeader,
    payload: StatusPayload,
}

impl StatusMessage {
    pub fn new(
        robot_id: String,
        is_localized: bool,
        is_paused: bool,
        is_collisioned: bool,
    ) -> Self {
        StatusMessage {
            header: StatusHeader {
                r#type: "STATUS".to_string(),
                robot_id,
                command_id: "".to_string(),
                timestamp: 0.0,
            },
            payload: StatusPayload {
                battery: StatusBattery {
                    isCharing: false,
                    isFull: false,
                    isLow: false,
                },
                emergency: StatusEmergency {
                    isPaused: is_paused,
                    isCollisioned: is_collisioned,
                },
                navigation: StatusNavigation {
                    isMoving: false,
                    isLocalized: is_localized,
                },
                opStatus: String::from("LOCALIZATION"),
                opMode: String::from(""),
                session: vec![
                    StatusSession {
                        errors: vec![],
                        module: String::from(""),
                        status: String::from(""),
                    }
                ]
            }
        }
    }

    pub fn operation_error(robot_id: String) -> Self {
        StatusMessage {
            header: StatusHeader {
                r#type: "STATUS".to_string(),
                robot_id,
                command_id: "".to_string(),
                timestamp: 0.0,
            },
            payload: StatusPayload {
                battery: StatusBattery {
                    isCharing: false,
                    isFull: false,
                    isLow: false,
                },
                emergency: StatusEmergency {
                    isCollisioned: true,
                    isPaused: false,
                },
                navigation: StatusNavigation {
                    isMoving: false,
                    isLocalized: true,
                },
                opMode: String::from("OPERATION"),
                opStatus: String::from("ERROR"),
                session: vec![
                    StatusSession {
                        errors: vec![],
                        module: String::from(""),
                        status: String::from(""),
                    }
                ]
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub struct StatusHeader {
    r#type: String,
    #[serde(rename = "robotId")]
    robot_id: String,
    #[serde(rename = "commandId")]
    command_id: String,
    timestamp: f64,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
struct StatusPayload {
    battery: StatusBattery,
    emergency: StatusEmergency,
    navigation: StatusNavigation,
    opStatus: String,
    opMode: String,
    session: Vec<StatusSession>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
struct StatusBattery {
    isCharing: bool,
    isFull: bool,
    isLow: bool,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
struct StatusEmergency{
    isCollisioned: bool,
    isPaused: bool,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
struct StatusNavigation{
    isMoving: bool,
    isLocalized: bool,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
struct StatusSession{
    errors: Vec<String>,
    module: String,
    status: String,
}
