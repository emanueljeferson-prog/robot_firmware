use crate::middleware::broker::Broker;
use crate::middleware::message::Message;
use crate::middleware::message::TaskDescriptor;

use core::ffi::c_void;
use core::ptr::NonNull;

pub struct MotorControl {
    signal: f32,
    speed: f32,
    motor: String
}

impl MotorControl {
    pub fn new(name: String) -> Self {
        Self { signal: 0.0, speed: 0.0, motor: name }
    }

    pub fn init(&self, middleware: &Broker) {
        middleware.publish(
            &Message::RegisterTask(
                TaskDescriptor {
                    name: "motor_control_task",
                    stack_size: 1024,
                    priority: 1,
                    task: motor_control_task,
                    context: Some(NonNull::from(self).cast::<c_void>())
                }
            )
        );

        middleware.publish(
            &Message::RegisterTask(
                TaskDescriptor {
                    name: "read_speed_task",
                    stack_size: 1024,
                    priority: 1,
                    task: read_speed_task,
                    context: Some(NonNull::from(self).cast::<c_void>())
                }
            )
        );
    }

    pub fn control(&self) {
        println!("Motor control '{}'...", self.motor);
    }

    pub fn read_speed(&self) {
        println!("Read speed '{}'...", self.motor);
    }
}

extern "C" fn motor_control_task(ctx: *mut c_void)
{
    let motor =
        unsafe {
            &*(ctx as *const MotorControl)
        };

    loop {
        motor.control();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

extern "C" fn read_speed_task(ctx: *mut c_void)
{
    let motor =
        unsafe {
            &*(ctx as *const MotorControl)
        };

    loop {
        motor.read_speed();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}