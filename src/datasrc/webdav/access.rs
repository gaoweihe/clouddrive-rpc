use std::{io::Cursor, result};

use hyperdav::Client;

use crate::datasrc::{prototype::{DataSource, Message}, message::message::{RpcMessage, RpcMessageMeta}};

use super::webdav::WebDAV;

impl DataSource<RpcMessage, RpcMessageMeta> for WebDAV
{
    fn connect(&mut self, uri: &str, opt: Option<std::vec::Vec<String>>) -> anyhow::Result<()> {
        let opt = opt.unwrap();
        let username = opt[0].clone();
        let password = opt[1].clone();
        // hyperdav
        let client = Client::new()
            .credentials(username, password)
            .build(uri)
            .unwrap();
        let r = std::io::empty();
        let result = client.list(&["clouddrive-rpc"]);
        match result {
            Ok(res) => {},
            Err(e) => {
                client.mkcol(&["clouddrive-rpc"]).unwrap();
            },
        }

        self.client = Some(std::sync::Arc::new(std::sync::Mutex::new(client)));
        
        Ok(())
    }

    fn disconnect(&mut self) -> anyhow::Result<()> {
        self.client = None;

        Ok(())
    }

    fn put(&mut self, path: String, msg: &RpcMessage) -> anyhow::Result<()> {
        let binding = self.client.as_mut().unwrap();
        let client = binding.lock().unwrap();

        let full_path = self.work_dir.clone() + &path;
        client.put(Cursor::new(msg.get_payload()), &[full_path]).unwrap();

        Ok(())
    }

    fn get(&mut self, path: String) -> anyhow::Result<std::vec::Vec<u8>> {
        let binding = self.client.as_mut().unwrap();
        let client = binding.lock().unwrap();

        let full_path = self.work_dir.clone() + &path;
        let mut result = client.get(&[full_path]).unwrap();
        let result = result.text().unwrap().as_bytes().to_vec();

        Ok(result)
    }

    fn list(&mut self, path: String) -> anyhow::Result<std::vec::Vec<String>> {
        let binding = self.client.as_mut().unwrap();
        let client = binding.lock().unwrap();

        let full_path = self.work_dir.clone() + &path;
        let result = client.list(&[full_path]).unwrap();
        let mut result = result.iter().map(|x| x.href.clone()).collect::<Vec<String>>();

        Ok(result)
    }

    fn clear(&mut self, path: String) -> anyhow::Result<()> {
        todo!()
    }

    fn delete(&mut self, path: String) -> anyhow::Result<()> {
        let binding = self.client.as_mut().unwrap();
        let client = binding.lock().unwrap();

        let full_path = self.work_dir.clone() + &path;
        let result = client.request(reqwest::Method::DELETE, &[full_path]).send().unwrap();

        Ok(())
    }
}



