use std::sync::Arc;
use core::ptr::NonNull;
use core::ffi::c_void;

pub type Callback = Arc<dyn Fn(&Message)+ Send + Sync>;
pub type TaskEntry = extern "C" fn(*mut core::ffi::c_void);

#[derive(Debug)]
pub struct TaskDescriptor {
    pub name: &'static str,
    pub stack_size: usize,
    pub priority: u8,
    pub task: TaskEntry,
    pub context: Option<NonNull<c_void>>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Topics {
    MotorControl,
    ReadSpeed,
    ReadImuData,
    ReadGpsData,
    RegisterTask
}

pub enum Message {
    MotorControl,
    ReadSpeed(f64),
    ReadImuData,
    ReadGpsData,
    RegisterTask(TaskDescriptor)
}

impl Message {
    pub fn topic(&self) -> Topics {
        match self {
            Message::MotorControl => {
                Topics::MotorControl
            }

            Message::ReadSpeed(_) => {
                Topics::ReadSpeed
            }

            Message::ReadImuData => {
                Topics::ReadImuData
            }

            Message::ReadGpsData => {
                Topics::ReadGpsData
            }

            Message::RegisterTask(_) => {
                Topics::RegisterTask
            }
        }
    }
}

pub struct Subscriber {
    pub callback: Callback,
    pub topic: Topics,
    pub delete_after: bool
}

impl Subscriber {
    pub fn new(cb: Callback, tp: Topics, delete: bool) -> Self {
        Self { callback: cb, topic: tp, delete_after: delete }
    }
}
