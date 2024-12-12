use super::report::ReportMessage;

pub trait PantosKafkaClient {
    fn consume_desired_topic(&self) -> impl std::future::Future<Output = String> + Send;

    fn publish_reported_message(&self, robot_uid: &str, message: ReportMessage);

    fn publish_started_loading(
        &self,
        robot_uid: &str,
    ) -> impl std::future::Future<Output = ()> + Send;

    fn publish_loading(&self, robot_uid: &str) -> impl std::future::Future<Output = ()> + Send;

    fn publish_started_picking(
        &self,
        robot_uid: &str,
    ) -> impl std::future::Future<Output = ()> + Send;

    //fn publish_waiting_worker_to_pick(&self, robot_uid: &str) {}

    fn publish_picking(&self, robot_uid: &str) -> impl std::future::Future<Output = ()> + Send;

    fn publish_started_unloading(
        &self,
        robot_uid: &str,
    ) -> impl std::future::Future<Output = ()> + Send;

    fn publish_unloading(&self, robot_uid: &str) -> impl std::future::Future<Output = ()> + Send;

    fn publish_arrived_at_emergency_position(
        &self,
        robot_uid: &str,
    ) -> impl std::future::Future<Output = ()> + Send;

    fn publish_arrived_at_recovered_position(
        &self,
        robot_uid: &str,
    ) -> impl std::future::Future<Output = ()> + Send;
}
