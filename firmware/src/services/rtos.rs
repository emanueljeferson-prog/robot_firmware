use crate::middleware::message::Message;
use crate::middleware::message::Topics;
use crate::middleware::message::TaskDescriptor;
use crate::middleware::subscriber::SubscriberManager;

use alloc::sync::Arc;
use core::ffi::c_void;
use alloc::ffi::CString;

unsafe extern "C" {
    fn createTask(
        task_ptr: extern "C" fn(*mut c_void),
        task_name: *const i8,
        stack_size: usize,
        priority: u8,
        parameters: *mut c_void
    );
    fn schedulerStart();
    fn delayTask(delay: u32);
}

pub struct RtosService;

impl RtosService {
    pub fn init(middleware: &dyn SubscriberManager) {
        middleware.subscribe(
            Arc::new(|msg: &mut Message| {
                if let Message::RegisterTask(desc) = msg {
                    RtosService::crete_task(desc);
                }
            }),
            Topics::RegisterTask,
            true
        );

        middleware.subscribe(
            Arc::new(|msg: &mut Message| {
                if let Message::DelayTask(delay) = msg {
                    RtosService::delay_task(*delay);
                }
            }),
            Topics::DelayTask,
            false
        )
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

    pub fn delay_task(delay: u32) {
        unsafe {
            delayTask(delay);
        }
    }
}