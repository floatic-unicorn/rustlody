use serde::Serialize;


#[derive(Serialize)]
pub struct PostStartWorkRequest<'a> {
    #[serde(rename = "trackingNumber")]
    pub tracking_number: &'a str,
}

#[derive(Serialize)]
pub struct PostWorkerArrivedRequest<'a> {
    #[serde(rename = "pickingIds")]
    pub picking_ids: &'a[&'a str],
    pub worker_code: Option<&'a str>,
}

#[derive(Serialize)]
pub struct PostCompleteUnloadingRequest<'a> {
    #[serde(rename = "workGroupIds")]
    pub workgroup_ids: &'a[&'a str],
}
