use std::vec;

use anyhow::{Result, Ok};

use clouddrive_rpc::conf::conf::Conf;
use clouddrive_rpc::conf::conf::CONF;
use clouddrive_rpc::datasrc::message::message::RpcMessage;
use clouddrive_rpc::datasrc::message::message::RpcMessageMeta;
use clouddrive_rpc::rpc::channel::Channel;
use tracing_subscriber::{prelude::*, filter::LevelFilter};
use tracing_subscriber::layer::SubscriberExt;

use clouddrive_rpc::datasrc::prototype::DataSource;
use clouddrive_rpc::datasrc::webdav::{webdav::WebDAV};

pub fn launch_service() -> Result<()> {
    // clouddrive_rpc::datasrc::onedrive::login::connect().await?;

    let conf = CONF.get().unwrap();
    let username = conf.get_username().to_string();
    let password = conf.get_password().to_string();
    let uri = conf.get_uri();

    let mut webdav = WebDAV::new();
    let opt = Some(vec![username, password]);
    webdav.connect(
        uri,
        opt,
    )?;

    let mut msg = RpcMessage::new();
    msg.set_payload("dummy".as_bytes().to_vec());
    let msg = bincode::serialize(&msg).unwrap();
    webdav.put_by_rel_path("test.cdrpc".to_string(), &msg).unwrap();
    webdav.get_by_rel_path("test.cdrpc".to_string()).unwrap();
    let result = webdav.list("".to_string()).unwrap();
    tracing::trace!("result: {:?}", result);
    webdav.delete("test.cdrpc".to_string()).unwrap();
    webdav.clear("".to_string()).unwrap();

    let mut msg_vec = vec![];
    let mut msg = RpcMessage::new();

    let send_string = "hello".to_string();
    tracing::info!("send {:?}", send_string);

    let payload = send_string.as_bytes().to_vec();
    msg.set_payload(payload);
    msg_vec.push((msg, 0));
    webdav.send(msg_vec).unwrap();
    let msg_list = webdav.poll().unwrap();
    for msg in msg_list {
        let recv_string = String::from_utf8(msg.payload).unwrap();
        tracing::info!("recv {:?}", recv_string);
    }

    Ok(())
}

fn main() -> Result<()> {
    init_tracer();
    init_config();
    launch_service()?;

    Ok(())
}

fn init_tracer() {
    let level_filter = LevelFilter::INFO;
    let subscriber = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_filter(level_filter))
        .with(tracing_tracy::TracyLayer::new());
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
}

fn init_config() {
    Conf::parse_args();
}
