use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client,
};

use crate::http::token::make_token;

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
                Err(_err) => println!("[TOS] | [RES] upload_excel response error: {}", _err),
            },
            Err(_err) => println!("[TOS] | [RES] upload_excel failed due to: {}", _err),
        }
    }

    // Flody Console
    pub async fn command_robot_loading(&self) {
        println!("[TOS] | [REQ] command_robot_loading");

        let robot_uid = "dBK39Eak";
        let url = String::from(&self.url) + "/v1/robots/" + robot_uid + "/load";
        let headers = self.make_auth_headers();

        match self.client.post(url).headers(headers).send().await {
            Ok(_res) => match _res.error_for_status() {
                Ok(_res) => println!("[TOS] | [RES] command_robot_loading success"),
                Err(_err) => println!(
                    "[TOS] | [RES] command_robot_loading response error: {}",
                    _err
                ),
            },
            Err(_err) => println!(
                "[TOS] | [RES] command_robot_loading failed due to: {}",
                _err
            ),
        }
    }

    /*
    fn identify_repesentative_invoice_barcode();

    fn start_work();

    fn worker_arrived();

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
