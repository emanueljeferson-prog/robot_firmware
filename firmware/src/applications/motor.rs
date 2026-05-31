use crate::middleware::message::Message;
use crate::middleware::message::TaskDescriptor;
use crate::middleware::publisher::PublisherManager;

use core::ffi::c_void;
use core::ptr::NonNull;

pub struct MotorControl<'a> {
    id: u8,
    signal: f32,
    speed: f32,
    publisher: &'a dyn PublisherManager
}

impl<'a> MotorControl<'a> {
    pub fn new(id: u8, publisher: &'a dyn PublisherManager) -> Self {
        Self { id, signal: 0.0, speed: 0.0, publisher }
    }

    pub fn init(&self) {
        self.publisher.publish(
            &mut Message::RegisterTask(
                TaskDescriptor {
                    name: "motor_control_task",
                    stack_size: 1024,
                    priority: 1,
                    task: motor_control_task,
                    context: Some(NonNull::from(self).cast::<c_void>())
                }
            )
        );

        self.publisher.publish(
            &mut Message::RegisterTask(
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
        println!("Motor control '{}'...", self.id);
    }

    pub fn read_speed(&mut self) {
        self.publisher.publish(&mut Message::ReadSpeed(self.id ,&mut self.speed));
        println!("Read speed: {}", self.speed);
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
        motor.publisher.publish(&mut Message::DelayTask(1000));
    }
}

extern "C" fn read_speed_task(ctx: *mut c_void)
{
    let motor =
        unsafe {
            &mut *(ctx as *mut MotorControl)
        };

    loop {
        motor.read_speed();
        motor.publisher.publish(&mut Message::DelayTask(1000));
    }
}