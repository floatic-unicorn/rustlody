use crate::http::client::PantosHttpClient;


pub async fn run_success_flow(
    mut http_client: PantosHttpClient,
) {
    http_client.upload_excel().await;
}
