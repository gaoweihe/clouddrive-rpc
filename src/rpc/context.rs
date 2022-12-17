use crate::datasrc::{message::message::{RpcMessage}, prototype::{DataSource, Message, MessageMeta}};

use super::callback::CallBackBox;

pub struct RpcContext<SerMsg, SerMeta>
where SerMsg: Message<SerMeta> + serde::Serialize,
    SerMeta: MessageMeta + serde::Serialize {
    // send queue
    pub send_queue: std::sync::Arc<
        std::sync::Mutex<
            std::collections::VecDeque<
            SerMsg
    >>>,
    pub recv_queue: std::sync::Arc<
        std::sync::Mutex<
            std::collections::VecDeque<
            SerMsg
    >>>,
    pub cb_map: std::collections::HashMap<
        u64, 
        CallBackBox
    >,

    pub data_source: Option<Box<dyn DataSource<SerMsg, SerMeta>>>, 
}

impl<SerMsg, SerMeta> RpcContext<SerMsg, SerMeta>
where SerMsg: Message<SerMeta> + serde::Serialize,
    SerMeta: MessageMeta + serde::Serialize 
{
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
            data_source: None,
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
