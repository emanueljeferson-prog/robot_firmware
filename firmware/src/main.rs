mod middleware;
mod applications;
mod services;
mod hal;
mod config;

extern crate alloc;

use applications::motor::MotorControl;
use services::rtos::RtosService;
use services::encoder::EncoderService; 
use crate::middleware::broker::Broker;
use crate::config::{init_global_config, get_config};

fn main() {
    init_global_config();
    if let Some(config) = get_config() {
        let broker = Broker::new();
        RtosService::init(&broker);
        let encoder = EncoderService::new();
        encoder.init(&broker);
        for (motor_id, motor_config) in &config.motors {
            let app_motor = MotorControl::new(*motor_id, &broker);
            app_motor.init();
        }
        broker.run();
        RtosService::scheduler_start();
    }
}
