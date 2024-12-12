use std::fmt;
use serde::Serialize;


#[derive(Serialize)]
pub struct PostStartWorkRequest<'a> {
    #[serde(rename = "trackingNumber")]
    pub tracking_number: &'a str,
}

#[derive(Serialize)]
pub struct PostWorkerArrivedRequest<'a> {
    #[serde(rename = "pickingIds")]
    pub picking_ids: &'a Vec<String>,
    pub worker_code: Option<&'a str>,
}

impl<'a> fmt::Display for PostWorkerArrivedRequest<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "picking_ids=[{}], worker_code={}",
            self.picking_ids.join(", "),
            self.worker_code.unwrap_or("none")
        )
    }
}

#[derive(Serialize)]
pub struct PostCompleteUnloadingRequest<'a> {
    #[serde(rename = "workGroupIds")]
    pub workgroup_ids: &'a[&'a str],
}
