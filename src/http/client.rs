use std::fs;
use reqwest::{header::{HeaderMap, HeaderValue, AUTHORIZATION}, Client};

use crate::http::token::make_token;


pub struct PantosHttpClient {
    pub url: String,
    pub client: Client
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
        return headers
    }

    // TOS
    pub async fn upload_excel(&mut self) {
        println!("[TOS] | [REQ] upload_excel");

        let headers = self.make_auth_headers();

        let file = fs::read("./resources/wave.excel").unwrap();
        let file_part = reqwest::multipart::Part::bytes(file)
            .file_name("bg.jpg")
            .mime_str("image/jpg")
            .unwrap();
        let form = reqwest::multipart::Form::new().part("img", file_part);

        match self.client
            .post(&self.url)
            .headers(headers)
            .multipart(form)
            .send()
            .await
        {
            Ok(_res) => {
                match _res.error_for_status() {
                    Ok(_res) => println!("[TOS] | [RES] upload_excel success"),
                    Err(_err) => println!("[TOS] | [RES] upload_excel failed due to: {}", _err)
                }
            },
            Err(_err) => println!("[TOS] | [RES] upload_excel failed due to: {}", _err),
        }
    }

    /*
    // Flody Console
    fn move_to_loading_zone();

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
