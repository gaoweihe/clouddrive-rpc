use serde::Serialize;

pub trait MessageMeta {
    fn get_id(&self) -> u64;
    fn get_size(&self) -> u64;
    fn get_msg_type(&self) -> u64;
}

pub trait Message<SerMeta>
where SerMeta: MessageMeta + Serialize {
    fn get_meta(&self) -> SerMeta;
    fn get_payload(&self) -> std::vec::Vec<u8>;
}

pub trait DataSource<SerMsg, SerMeta>
where SerMsg: Message<SerMeta> + Serialize,
      SerMeta: MessageMeta + Serialize {
    fn connect(&mut self, uri: &str, opt: Option<std::vec::Vec<String>>) -> anyhow::Result<()>;
    fn disconnect(&mut self) -> anyhow::Result<()>;
    fn put(&mut self, path: String, msg: &SerMsg) -> anyhow::Result<()>;
    fn get(&mut self, path: String) -> anyhow::Result<std::vec::Vec<u8>>;
    fn list(&mut self, path: String) -> anyhow::Result<std::vec::Vec<String>>;
    fn clear(&mut self, path: String) -> anyhow::Result<()>;
    fn delete(&mut self, path: String) -> anyhow::Result<()>;
    fn mv(&mut self, src: String, dst: String) -> anyhow::Result<()>;
    fn exist(&mut self, path: String) -> bool;
} 
