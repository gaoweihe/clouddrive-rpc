use crate::datasrc::prototype::Message;
use crate::datasrc::prototype::MessageMeta;

pub trait Channel<SerMsg, SerMeta>
where SerMsg: Message<SerMeta> + serde::Serialize,
    SerMeta: MessageMeta + serde::Serialize {
    fn poll(&mut self) -> anyhow::Result<std::vec::Vec<SerMsg>>;
    fn send(&mut self, msg_vec: std::vec::Vec<(SerMsg, u64)>) -> anyhow::Result<()>;
    fn get_unique_msg_id(&self) -> u64;
}
