use serde::{Serialize, Deserialize};

use crate::datasrc::prototype::{Message, MessageMeta};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RpcMessage {
    meta: RpcMessageMeta,
    payload: std::vec::Vec<u8>,
}

impl RpcMessage {
    pub fn new() -> Self {
        Self {
            meta: RpcMessageMeta {
                id: 0, 
                size: 0, 
                msg_type: 0,
            },
            payload: std::vec::Vec::new(),
        }
    }

    pub fn set_payload(&mut self, payload: std::vec::Vec<u8>) {
        self.payload = payload;
        self.meta.size = self.payload.len() as u64;
    }
}

impl Message<RpcMessageMeta> for RpcMessage {
    fn get_meta(&self) -> RpcMessageMeta {
        self.meta.clone()
    }

    fn get_payload(&self) -> std::vec::Vec<u8> {
        self.payload.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RpcMessageMeta {
    id: u64, 
    size: u64,
    msg_type: u64,
}

impl MessageMeta for RpcMessageMeta {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_size(&self) -> u64 {
        self.size
    }

    fn get_msg_type(&self) -> u64 {
        self.msg_type
    }
}
