use serde::Serialize;

use crate::datasrc::prototype::{DataSource, Message, MessageMeta};

use super::context::RpcContext;

pub struct RpcManager<SerMsg, SerMeta>
where SerMsg: Message<SerMeta> + serde::Serialize,
    SerMeta: MessageMeta + serde::Serialize {
    pub context: RpcContext<SerMsg, SerMeta>,
} 

impl<SerMsg, SerMeta> RpcManager<SerMsg, SerMeta>
where SerMsg: Message<SerMeta> + serde::Serialize,
    SerMeta: MessageMeta + serde::Serialize {
    pub fn new() -> Self {
        RpcManager {
            context: RpcContext::new(),
        }
    }

    pub fn set_context(&mut self, context: RpcContext<SerMsg, SerMeta>) {
        self.context = context;
    }

    pub fn get_context(&self) -> &RpcContext<SerMsg, SerMeta> {
        &self.context
    }

    pub fn start(&self) {
    }
}
