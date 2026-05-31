mod middleware;
mod applications;
mod services;
mod hal;

use applications::motor::MotorControl;
use services::rtos::RtosService;
use services::encoder::EncoderService; 
use crate::middleware::broker::Broker;


fn main() {
    let broker = Broker::new();
    RtosService::init(&broker);

    let encoder = EncoderService::new();
    encoder.init(&broker);

    let app_motor = MotorControl::new("motor_left".to_string(), &broker);
    app_motor.init();
    broker.run();
    RtosService::scheduler_start();
}
