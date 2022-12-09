use crate::datasrc::{message::message::{RpcMessage}};

use super::callback::CallBackBox;

pub struct RpcContext {
    // send queue
    pub send_queue: std::sync::Arc<
        std::sync::Mutex<
            std::collections::VecDeque<
                RpcMessage
    >>>,
    pub recv_queue: std::sync::Arc<
        std::sync::Mutex<
            std::collections::VecDeque<
                RpcMessage
    >>>,
    pub cb_map: std::collections::HashMap<
        u64, 
        CallBackBox
    >,
}

impl RpcContext {
    pub fn new() -> Self {
        RpcContext {
            send_queue: std::sync::Arc::new(
                std::sync::Mutex::new(
                    std::collections::VecDeque::new()
            )),
            recv_queue: std::sync::Arc::new(
                std::sync::Mutex::new(
                    std::collections::VecDeque::new()
            )),
            cb_map: std::collections::HashMap::new(),
        }
    }

    pub fn reg_callback(&self, msg_type: u64, cb: CallBackBox) {
        unsafe { 
            let cb_map_mut = 
                get_mut_from_immut(&self.cb_map);
            cb_map_mut.insert(msg_type, cb); 
        }
    }

    pub fn get_callback(&self, msg_type: u64) -> Option<&CallBackBox> {
        self.cb_map.get(&msg_type)
    }
}

pub unsafe fn get_mut_from_immut<T>(immut: &T) -> &mut T {
    let immut_ptr = immut as *const T;
    let mut_ptr = immut_ptr as *mut T;
    &mut *mut_ptr
}
