mod middleware;
mod applications;
mod services;
mod hal;

use middleware::message::Message;
use middleware::message::Topics;
use middleware::broker::Broker;
use applications::app_motor_control::MotorControl;
use services::rtos::RtosService;

fn main() {
    let mut broker = Broker::new();
    RtosService::init(&broker);
    let app_motor = MotorControl::new("motor_left".to_string());
    app_motor.init(&broker);
    broker.run();
    RtosService::scheduler_start();
}
