pub type Callback = Box<dyn Fn(&Message)>;

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
    RegisterTask
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

            Message::RegisterTask => {
                Topics::RegisterTask
            }
        }
    }
}

pub struct Subscriber {
    pub callback: Callback,
    pub topic: Topics
}

impl Subscriber {
    pub fn new(cb: Callback, tp: Topics) -> Self {
        Self { callback: cb, topic: tp }
    }
}
