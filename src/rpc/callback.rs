use std::boxed::Box;

use crate::datasrc::message::message::RpcMessage;

pub trait CallBack: Fn(RpcMessage) { }
impl<F> CallBack for F where F: Fn(RpcMessage) { }
impl std::fmt::Debug for dyn CallBack {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "dyn CallBack")
    }
}

pub type CallBackBox = Box<dyn CallBack>;
