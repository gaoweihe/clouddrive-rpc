use anyhow::Ok;
use hyperdav::Client;

use crate::{datasrc::{message::message::{RpcMessage, RpcMessageMeta}, prototype::{Message, MessageMeta, DataSource}}, rpc::channel::Channel, conf::conf::CONF};

pub struct WebDAV {
    pub client: Option<std::sync::Arc<std::sync::Mutex<Client>>>,
    pub work_dir: String, 
    pub msg_id_counter: std::sync::atomic::AtomicU64,
}

impl WebDAV {
    pub fn new() -> Self {
        let conf = CONF.get().unwrap();
        WebDAV {
            client: None, 
            work_dir: String::from("clouddrive-rpc"),
            msg_id_counter: conf.get_node_id().into(),
        }
    }

    pub fn is_connected(&self) -> bool {
        self.client.is_some()
    }
}

impl Channel<RpcMessage, RpcMessageMeta> for WebDAV {
    fn poll(&mut self) -> anyhow::Result<std::vec::Vec<RpcMessage>> {
        let conf = CONF.get().unwrap();

        let mut msg_list = std::vec::Vec::new();

        // list all unread messages 
        let unread_path = format!("{}/{}", conf.get_node_id(), "unread");
        let mut list_result = self.list(unread_path).unwrap();
        list_result.remove(0);
        for msg_full_path in list_result {
            // get message
            let msg = self.get(msg_full_path.clone()).unwrap();

            // mark message as read
            self.mark_as_read(msg_full_path).unwrap();
            
            // deserialize message
            let msg: RpcMessage = bincode::deserialize(&msg).unwrap();
            tracing::trace!("deserialized message: {:?}", msg);

            msg_list.push(msg);
        }

        Ok(msg_list)
    }

    ///
    /// Send message to destination
    /// 
    /// # Arguments
    /// * `msg_vec` - a vector of message and destination number
    /// 
    fn send(&mut self, msg_vec: std::vec::Vec<(RpcMessage, u64)>) -> anyhow::Result<()> {
        for msg_pair in msg_vec {
            let msg = msg_pair.0;
            // get directory 
            let dest_num = msg_pair.1;
            let msg_id = msg.get_meta().get_id();
            let dest_dir = format!("{}/{}/{}.cdrpc", dest_num, "unread", msg_id);

            // serialize message
            let msg = bincode::serialize(&msg).unwrap();
            tracing::trace!("serialized message: {:?}", msg);

            // put message
            self.put_by_rel_path(dest_dir, &msg).unwrap();
        }

        Ok(())
    }

    fn get_unique_msg_id(&self) -> u64 {
        self.msg_id_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }
}
