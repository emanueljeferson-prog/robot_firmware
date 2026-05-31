use crate::middleware::broker::Broker;
use crate::middleware::message::Message;
use crate::middleware::message::Topics;
use crate::middleware::message::TaskDescriptor;
use std::sync::Arc;
use core::ffi::c_void;
use std::ffi::CString;

unsafe extern "C" {
    fn createTask(
        task_ptr: extern "C" fn(*mut c_void),
        task_name: *const i8,
        stack_size: usize,
        priority: u8,
        parameters: *mut c_void
    );
    fn schedulerStart();
}

pub struct RtosService;

impl RtosService {
    pub fn init(middleware: &Broker) {
        middleware.subscribe(
            Arc::new(|msg: &Message| {
                if let Message::RegisterTask(desc) = msg {
                    RtosService::crete_task(desc);
                }
            }),
            Topics::RegisterTask,
            true
        );
    }
    
    pub fn crete_task(desc: &TaskDescriptor) {
        println!("Descriptor: {:#?}", desc);

        let name = CString::new(desc.name).unwrap();

        let context_ptr = desc
            .context
            .map(|c| c.as_ptr())
            .unwrap_or(core::ptr::null_mut());

        unsafe {
            createTask(
                desc.task,
                name.as_ptr(),
                desc.stack_size,
                desc.priority,
                context_ptr
            );
        }
    }

    pub fn scheduler_start() {
        unsafe {
            schedulerStart();
        }
    }
}