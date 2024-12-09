use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client,
};

use crate::http::token::make_token;
use crate::http::request::*;
use crate::http::response::*;

pub struct PantosHttpClient {
    pub url: String,
    pub client: Client,
}

impl PantosHttpClient {
    pub fn new() -> Self {
        PantosHttpClient {
            url: String::from("http://localhost:8080"),
            client: Client::new(),
        }
    }

    fn make_auth_headers(&self) -> HeaderMap<HeaderValue> {
        let mut headers = HeaderMap::new();
        let bearer = String::from("Bearer ") + &make_token();
        let token = HeaderValue::from_str(&bearer).unwrap();

        headers.insert(AUTHORIZATION, token);
        headers
    }

    // TOS
    pub async fn upload_excel(&self) {
        println!("[TOS] | [REQ] upload_excel");

        let url = String::from(&self.url) + "/v1/waves/multiple";
        let headers = self.make_auth_headers();

        let form = reqwest::multipart::Form::new()
            .file("createWaves", "./resources/wave")
            .await
            .unwrap();

        match self
            .client
            .post(url)
            .headers(headers)
            .multipart(form)
            .send()
            .await
        {
            Ok(_res) => match _res.error_for_status() {
                Ok(_res) => println!("[TOS] | [RES] upload_excel success"),
                Err(_err) => panic!("[TOS] | [RES] upload_excel response error: {}", _err),
            },
            Err(_err) => panic!("[TOS] | [RES] upload_excel failed due to: {}", _err),
        }
    }

    // Flody Console
    pub async fn command_robot_loading(&self) {
        println!("[FLODY_CONSOLE] | [REQ] command_robot_loading");

        let robot_uid = "dBK39Eak";
        let url = String::from(&self.url) + "/v1/robots/" + robot_uid + "/load";
        let headers = self.make_auth_headers();

        match self.client.post(url).headers(headers).send().await {
            Ok(_res) => match _res.error_for_status() {
                Ok(_res) => println!("[FLODY_CONSOLE] | [RES] command_robot_loading success"),
                Err(_err) => panic!(
                    "[FLODY_CONSOLE] | [RES] command_robot_loading response error: {}",
                    _err
                ),
            },
            Err(_err) => panic!(
                "[FLODY_CONSOLE] | [RES] command_robot_loading failed due to: {}",
                _err
            ),
        }
    }

    pub async fn identify_repesentative_invoice_barcode(&self, tracking_number: &str) -> GetIdentifyRepresentativeInvoiceBarcodeResponse {
        println!("[FLODY_CONSOLE] | [REQ] 대표 송장 바코드 인식");

        let url = String::from(&self.url) + "/v1/pickingJobs/trackingNumber/" + tracking_number;
        let headers = self.make_auth_headers();

        let response = self.client.get(url).headers(headers).send().await.unwrap();
        if response.status().is_success() {
            let response_data: GetIdentifyRepresentativeInvoiceBarcodeResponse = response.json().await.unwrap();
            println!("[FLODY_CONSOLE] | [RES] 대표 송장 바코드 인식 = {}", response_data);
            return response_data
        } else {
            let status = response.status();
            match response.json::<ErrorResponse>().await {
                Ok(err_msg) => panic!(
                    "[FLODY_CONSOLE] | [REQ] 대표 송장 바코드 인식 error: {err_msg}",
                ),
                Err(err) => {
                    panic!(
                        "[FLODY_CONSOLE] | [REQ] 대표 송장 바코드 인식 failed with response: {status}, due to: {err}",
                    )
                }
            }
        }
    }

    pub async fn start_work(&self, robot_uid: &str, tracking_number: &str) {
        println!("[FLODY_CONSOLE] | [REQ] 작업 시작");

        let url = String::from(&self.url) + "/v1/robots/" + robot_uid + "/pickingJob/assign";
        let headers = self.make_auth_headers();
        let body = PostStartWorkRequest { tracking_number: &tracking_number };

        match self
            .client
            .post(url)
            .headers(headers)
            .json(&body)
            .send()
            .await
        {
            Ok(_res) => match _res.error_for_status() {
                Ok(_res) => println!("[FLODY_CONSOLE] | [RES] 작업 시작 성공"),
                Err(_err) => panic!("[FLODY_CONSOLE] | [RES] 작업 시작 error: {}", _err),
            },
            Err(_err) => panic!("[FLODY_CONSOLE] | [RES] 작업 시작 failed due to: {}", _err),
        }
    }

    pub async fn worker_arrived(&self) {
        println!("[FLODY_CONSOLE] | [REQ] 작업자 도착");

        //let url = String::from(&self.url) + "/v1/pickings/workerArrived";
        let url = String::from(&self.url) + "/v1/pickings/pickerArrived";
        let headers = self.make_auth_headers();
        let body = PostWorkerArrivedRequest {
            picking_ids: vec!["picking-id-1"],
            worker_code: None,
        };
    }
    /*

    fn get_same_location_pickings();

    fn complete_picking();

    fn complete_partial();

    fn get_total_unloadings();

    fn command_initial_pose_reset();

    fn command_unpause();

    fn set_robot_status_idle();

    fn set_robot_status_fail();

    // Admin Console
    fn get_warehouses();

    fn get_dashboard();

    fn get_robot_positions();

    fn get_worklist();
    */
}
