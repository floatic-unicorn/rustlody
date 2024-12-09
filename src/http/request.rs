use serde::Serialize;


#[derive(Serialize)]
pub struct PostStartWorkRequest<'a> {
    #[serde(rename = "trackingNumber")]
    pub tracking_number: &'a str,
}


#[derive(Serialize)]
pub struct PostWorkerArrivedRequest<'a> {
    #[serde(rename = "")]
    pub picking_ids: Vec<&'a str>,
    pub worker_code: Option<&'a str>,
}
