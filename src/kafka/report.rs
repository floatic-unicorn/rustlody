use serde::Serialize;

#[derive(Debug, Serialize)]
struct ReportPayload<'a> {
    state: &'a str,
    //reason: String,
    code: &'a str,
}

#[derive(Debug, Serialize)]
pub struct ReportHeader<'a> {
    r#type: &'a str,
    #[serde(rename = "robotId")]
    robot_id: &'a str,
    #[serde(rename = "commandId")]
    command_id: &'a str,
    timestamp: f64,
}

#[derive(Debug, Serialize)]
pub struct ReportMessage<'a> {
    header: ReportHeader<'a>,
    payload: ReportPayload<'a>,
}

impl<'a> ReportMessage<'a> {
    pub fn new(state: &'a str, timestamp: f64, robot_id: &'a str) -> Self {
        ReportMessage {
            header: ReportHeader {
                r#type: "REPORT",
                robot_id,
                command_id: "",
                timestamp,
            },
            payload: ReportPayload {
                state,
                //reason: "test-reason".to_string(),
                code: "test-code"
            },
        }
    }
}
