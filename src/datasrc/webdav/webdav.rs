use anyhow::Ok;
use hyperdav::Client;

use crate::{datasrc::{message::message::{RpcMessage, RpcMessageMeta}}, rpc::channel::Channel};

pub struct WebDAV {
    pub client: Option<std::sync::Arc<std::sync::Mutex<Client>>>,
    pub work_dir: String, 
    pub msg_id_counter: std::sync::atomic::AtomicU64,
}

impl WebDAV {
    pub fn new() -> Self {
        WebDAV {
            client: None, 
            work_dir: String::from("clouddrive-rpc"),
            msg_id_counter: 0.into(),
        }
    }

    pub fn is_connected(&self) -> bool {
        self.client.is_some()
    }
}

impl Channel<RpcMessage, RpcMessageMeta> for WebDAV {
    fn poll(&mut self) -> anyhow::Result<std::vec::Vec<RpcMessage>> {
        todo!()
    }

    fn send(&mut self, msg_vec: std::vec::Vec<RpcMessage>) -> anyhow::Result<()> {
        for msg in msg_vec {
            todo!()
        }

        Ok(())
    }

    fn get_unique_msg_id(&self) -> u64 {
        self.msg_id_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }
}
