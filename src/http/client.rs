use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client,
};

use crate::http::request::*;
use crate::http::response::*;
use crate::http::token::make_token;

pub struct PantosHttpClient {
    pub url: String,
    pub client: Client,
}

impl Default for PantosHttpClient {
    fn default() -> Self {
        Self::new()
    }
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
    pub async fn upload_excel(&self, file_path: &str) {
        println!("[TOS] | [REQ] 엑셀 업로드 요청");

        let url = String::from(&self.url) + "/v1/waves/multiple";
        let headers = self.make_auth_headers();

        let form = reqwest::multipart::Form::new()
            .file("createWaves", file_path)
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
        println!("[FLODY_CONSOLE] | [REQ] 로딩존 이동 요청");

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

    pub async fn identify_repesentative_invoice_barcode(
        &self,
        tracking_number: &str,
    ) -> GetIdentifyRepresentativeInvoiceBarcodeResponse {
        println!("[FLODY_CONSOLE] | [REQ] 대표 송장 바코드 인식 요청");

        let url = String::from(&self.url) + "/v1/pickingJobs/trackingNumber/" + tracking_number;
        let headers = self.make_auth_headers();

        let response = self.client.get(url).headers(headers).send().await.unwrap();
        if response.status().is_success() {
            let response_data: GetIdentifyRepresentativeInvoiceBarcodeResponse =
                response.json().await.unwrap();
            println!(
                "[FLODY_CONSOLE] | [RES] 대표 송장 바코드 인식 = {}",
                response_data
            );
            response_data

        } else {
            let status = response.status();
            match response.json::<ErrorResponse>().await {
                Ok(err_msg) => {
                    panic!("[FLODY_CONSOLE] | [RES] 대표 송장 바코드 인식 error: {err_msg}",)
                }
                Err(err) => {
                    panic!(
                        "[FLODY_CONSOLE] | [RES] 대표 송장 바코드 인식 failed with response: {status}, due to: {err}",
                    )
                }
            }
        }
    }

    pub async fn start_work(&self, robot_uid: &str, tracking_number: &str) {
        println!("[FLODY_CONSOLE] | [REQ] 작업 시작 요청");

        let url = String::from(&self.url) + "/v1/robots/" + robot_uid + "/pickingJob/assign";
        let headers = self.make_auth_headers();
        let body = PostStartWorkRequest {
            tracking_number: &tracking_number,
        };

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

    pub async fn worker_arrived(&self, picking_ids: &Vec<String>) {
        println!(
            "[FLODY_CONSOLE] | [REQ] 작업자 도착 요청 picking_ids={:?}",
            picking_ids
        );

        //let url = String::from(&self.url) + "/v1/pickings/pickerArrived"; // api broken
        let url = String::from(&self.url) + "/v1/pickings/workerArrived"; // deprecated but in use
        let headers = self.make_auth_headers();
        let body = PostWorkerArrivedRequest {
            picking_ids,
            worker_code: None,
        };

        match self
            .client
            .post(url)
            .headers(headers)
            .json(&body)
            .send()
            .await
        {
            Ok(_res) => match _res.error_for_status() {
                Ok(_res) => println!("[FLODY_CONSOLE] | [RES] 작업자 도착 성공"),
                Err(_err) => {
                    panic!("[FLODY_CONSOLE] | [RES] 작업자 도착 error: {_err} | body: {body}")
                }
            },
            Err(_err) => {
                panic!("[FLODY_CONSOLE] | [RES] 작업자 도착 failed due to: {_err} | body: {body}")
            }
        }
    }

    pub async fn get_same_location_pickings(&self, picking_id: &str) -> Vec<PickingDto> {
        println!("[FLODY_CONSOLE] | [REQ] 동일 로케이션 피킹 요청");

        let url =
            String::from(&self.url) + "/v1/pickings/sameLocationCode" + "?pickingId=" + picking_id;
        let headers = self.make_auth_headers();

        let response = self.client.get(url).headers(headers).send().await.unwrap();
        if response.status().is_success() {
            let response_data: Vec<PickingDto> = response.json().await.unwrap();
            println!(
                "[FLODY_CONSOLE] | [RES] 동일 로케이션 피킹 size = {}",
                response_data.len()
            );
            response_data

        } else {
            let status = response.status();
            match response.json::<ErrorResponse>().await {
                Ok(err_msg) => {
                    panic!("[FLODY_CONSOLE] | [RES] 동일 로케이션 피킹 error: {err_msg}",)
                }
                Err(err) => {
                    panic!(
                        "[FLODY_CONSOLE] | [RES] 동일 로케이션 피킹 failed with response: {status}, due to: {err}",
                    )
                }
            }
        }
    }

    pub async fn complete_picking(&self, picking_id: &str) {
        println!("[FLODY_CONSOLE] | [REQ] 작업 완료 요청 picking_id = {picking_id}");

        let url = String::from(&self.url) + "/v1/pickings/" + picking_id + "/complete";
        let headers = self.make_auth_headers();

        match self.client.post(url).headers(headers).send().await {
            Ok(_res) => match _res.error_for_status() {
                Ok(_res) => println!("[FLODY_CONSOLE] | [RES] 작업 완료 성공"),
                Err(_err) => panic!("[FLODY_CONSOLE] | [RES] 작업 완료 error: {_err}"),
            },
            Err(_err) => panic!("[FLODY_CONSOLE] | [RES] 작업 완료 failed due to: {_err}"),
        }
    }

    pub async fn complete_partial(&self, picking_id: &str, all: bool) {
        println!("[FLODY_CONSOLE] | [REQ] 결품 작업 완료 요청");

        let mut url = String::from(&self.url) + "/v1/pickings/" + picking_id + "/partiallyComplete";
        if all == true {
            url = String::from(&self.url) + "/v1/pickings/" + "/partiallyComplete";
        }
        let headers = self.make_auth_headers();

        match self.client.post(url).headers(headers).send().await {
            Ok(_res) => match _res.error_for_status() {
                Ok(_res) => println!("[FLODY_CONSOLE] | [RES] 결품 작업 완료 성공"),
                Err(_err) => panic!("[FLODY_CONSOLE] | [RES] 결품 작업 완료 error: {_err}"),
            },
            Err(_err) => panic!("[FLODY_CONSOLE] | [RES] 결품 작업 완료 failed due to: {_err}"),
        }
    }

    pub async fn get_all_assigned_pickings(
        &self,
        robot_uid: &str,
    ) -> Vec<GetAllAssignedPickingsResponse> {
        println!("[FLODY_CONSOLE] | [REQ] 작업 목록 확인 요청");

        let url = String::from(&self.url) + "/v1/robots/" + robot_uid + "/pickings";
        let headers = self.make_auth_headers();

        let response = self.client.get(url).headers(headers).send().await.unwrap();
        if response.status().is_success() {
            let response_data: Vec<GetAllAssignedPickingsResponse> = response.json().await.unwrap();
            println!(
                "[FLODY_CONSOLE] | [RES] 작업 목록 확인 size= {}",
                response_data.len()
            );
            response_data

        } else {
            let status = response.status();
            match response.json::<ErrorResponse>().await {
                Ok(err_msg) => panic!("[FLODY_CONSOLE] | [RES] 작업 목록 확인 error: {err_msg}",),
                Err(err) => {
                    panic!(
                        "[FLODY_CONSOLE] | [RES] 작업 목록 확인 failed with response: {status}, due to: {err}",
                    )
                }
            }
        }
    }

    pub async fn get_total_unloadings(&self) -> GetUnloadingsTotalResponse {
        println!("[FLODY_CONSOLE] | [REQ] 전체 언로딩 정보 요청");

        let url = String::from(&self.url) + "/v1/unloadings/total";
        let headers = self.make_auth_headers();

        let response = self.client.get(url).headers(headers).send().await.unwrap();
        if response.status().is_success() {
            let response_data: GetUnloadingsTotalResponse = response.json().await.unwrap();
            println!(
                "[FLODY_CONSOLE] | [RES] 전체 언로딩 정보 in_progress = {}, queue = {}",
                response_data.in_progresses.len(),
                response_data.queue.len()
            );
            response_data

        } else {
            let status = response.status();
            match response.json::<ErrorResponse>().await {
                Ok(err_msg) => panic!("[FLODY_CONSOLE] | [RES] 전체 언로딩 정보 error: {err_msg}",),
                Err(err) => {
                    panic!(
                        "[FLODY_CONSOLE] | [RES] 전체 언로딩 정보 failed with response: {status}, due to: {err}",
                    )
                }
            }
        }
    }

    pub async fn complete_unloading(&self, workgroup_ids: &[&str]) {
        println!("[FLODY_CONSOLE] | [REQ] 언로딩 완료 요청");

        let url = String::from(&self.url) + "/v1/unloadings/complete";
        let headers = self.make_auth_headers();
        let body = PostCompleteUnloadingRequest { workgroup_ids };

        match self
            .client
            .post(url)
            .headers(headers)
            .json(&body)
            .send()
            .await
        {
            Ok(_res) => match _res.error_for_status() {
                Ok(_res) => println!("[FLODY_CONSOLE] | [RES] 언로딩 완료 success"),
                Err(_err) => panic!(
                    "[FLODY_CONSOLE] | [RES] 언로딩 완료 response error: {}",
                    _err
                ),
            },
            Err(_err) => panic!(
                "[FLODY_CONSOLE] | [RES] 언로딩 완료 failed due to: {}",
                _err
            ),
        }
    }

    pub async fn command_initial_pose_reset(&self, robot_uid: &str) {
        println!("[FLODY_CONSOLE] | [REQ] 로봇 위치 초기화 요청");

        let url = String::from(&self.url) + "/v1/robots/" + robot_uid + "/resetInitialPose";
        let headers = self.make_auth_headers();

        match self.client.post(url).headers(headers).send().await {
            Ok(_res) => match _res.error_for_status() {
                Ok(_res) => println!("[FLODY_CONSOLE] | [RES] 로봇 위치 초기화 success"),
                Err(_err) => panic!(
                    "[FLODY_CONSOLE] | [RES] 로봇 위치 초기화 response error: {}",
                    _err
                ),
            },
            Err(_err) => panic!(
                "[FLODY_CONSOLE] | [RES] 로봇 위치 초기화 failed due to: {}",
                _err
            ),
        }
    }

    pub async fn command_unpause(&self, robot_uid: &str) {
        println!("[FLODY_CONSOLE] | [REQ] 로봇 재활성화 요청");

        let url = String::from(&self.url) + "/v1/robots/" + robot_uid + "/unpause";
        let headers = self.make_auth_headers();

        match self.client.post(url).headers(headers).send().await {
            Ok(_res) => match _res.error_for_status() {
                Ok(_res) => println!("[FLODY_CONSOLE] | [RES] 로봇 재활성화 success"),
                Err(_err) => panic!(
                    "[FLODY_CONSOLE] | [RES] 로봇 재활성화 response error: {}",
                    _err
                ),
            },
            Err(_err) => panic!(
                "[FLODY_CONSOLE] | [RES] 로봇 재활성화 failed due to: {}",
                _err
            ),
        }
    }

    pub async fn set_robot_status_idle(&self, robot_uid: &str) {
        println!("[FLODY_CONSOLE] | [REQ] 로봇 대기 요청");

        let url = String::from(&self.url) + "/v1/robots/" + robot_uid + "/idle";
        let headers = self.make_auth_headers();

        match self.client.post(url).headers(headers).send().await {
            Ok(_res) => match _res.error_for_status() {
                Ok(_res) => println!("[FLODY_CONSOLE] | [RES] 로봇 대기 success"),
                Err(_err) => panic!("[FLODY_CONSOLE] | [RES] 로봇 대기 response error: {}", _err),
            },
            Err(_err) => panic!("[FLODY_CONSOLE] | [RES] 로봇 대기 failed due to: {}", _err),
        }
    }

    pub async fn set_robot_status_fail(&self, robot_uid: &str) {
        println!("[FLODY_CONSOLE] | [REQ] 로봇 작업 실패 요청");

        let url = String::from(&self.url) + "/v1/robots/" + robot_uid + "/fail";
        let headers = self.make_auth_headers();

        match self.client.post(url).headers(headers).send().await {
            Ok(_res) => match _res.error_for_status() {
                Ok(_res) => println!("[FLODY_CONSOLE] | [RES] 로봇 작업 실패 success"),
                Err(_err) => panic!(
                    "[FLODY_CONSOLE] | [RES] 로봇 작업 실패 response error: {}",
                    _err
                ),
            },
            Err(_err) => panic!(
                "[FLODY_CONSOLE] | [RES] 로봇 작업 실패 failed due to: {}",
                _err
            ),
        }
    }

    /*
    // Admin Console
    fn get_warehouses();

    fn get_dashboard();

    fn get_robot_positions();

    fn get_worklist();
    */
}
