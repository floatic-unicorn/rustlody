/*
use crate::http::response::*;
use reqwest::header::{HeaderMap, HeaderValue};

pub trait PantosHttpClient {
    fn make_auth_headers(&self) -> HeaderMap<HeaderValue>;

    fn upload_excel(&self, file_path: &str) -> impl std::future::Future<Output = ()> + Send;

    fn command_robot_loading(&self) -> impl std::future::Future<Output = ()> + Send;

    fn identify_repesentative_invoice_barcode(
        &self,
        tracking_number: &str,
    ) -> impl std::future::Future<Output = GetIdentifyRepresentativeInvoiceBarcodeResponse> + Send;

    fn start_work(
        &self,
        robot_uid: &str,
        tracking_number: &str,
    ) -> impl std::future::Future<Output = ()> + Send;

    fn worker_arrived(
        &self,
        picking_ids: &Vec<String>,
    ) -> impl std::future::Future<Output = ()> + Send;

    fn get_same_location_pickings(
        &self,
        picking_id: &str,
    ) -> impl std::future::Future<Output = Vec<PickingDto>> + Send;

    fn complete_picking(&self, picking_id: &str) -> impl std::future::Future<Output = ()> + Send;

    fn complete_partial(
        &self,
        picking_id: &str,
        all: bool,
    ) -> impl std::future::Future<Output = ()> + Send;

    fn get_all_assigned_pickings(
        &self,
        robot_uid: &str,
    ) -> impl std::future::Future<Output = Vec<GetAllAssignedPickingsResponse>> + Send;

    fn get_total_unloadings(
        &self,
    ) -> impl std::future::Future<Output = GetUnloadingsTotalResponse> + Send;

    fn complete_unloading(
        &self,
        workgroup_ids: &[&str],
    ) -> impl std::future::Future<Output = ()> + Send;

    fn command_initial_pose_reset(
        &self,
        robot_uid: &str,
    ) -> impl std::future::Future<Output = ()> + Send;

    fn command_unpause(&self, robot_uid: &str) -> impl std::future::Future<Output = ()> + Send;

    fn set_robot_status_idle(
        &self,
        robot_uid: &str,
    ) -> impl std::future::Future<Output = ()> + Send;

    fn set_robot_status_fail(
        &self,
        robot_uid: &str,
    ) -> impl std::future::Future<Output = ()> + Send;
}
*/
