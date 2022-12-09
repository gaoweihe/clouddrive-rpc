use std::{io::Cursor};

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
        let result = client.list(&["clouddrive-rpc"]);
        match result {
            Ok(_res) => {},
            Err(_e) => {
                self.create_work_dir_no_check().unwrap();
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
        let full_path = self.get_full_path(&path);
        let binding = self.client.as_mut().unwrap();
        let client = binding.lock().unwrap();

        client.put(Cursor::new(msg.get_payload()), &[full_path]).unwrap();

        Ok(())
    }

    fn get(&mut self, path: String) -> anyhow::Result<std::vec::Vec<u8>> {
        let full_path = self.get_full_path(&path);
        let binding = self.client.as_mut().unwrap();
        let client = binding.lock().unwrap();

        let mut result = client.get(&[full_path]).unwrap();
        let result = result.text().unwrap().as_bytes().to_vec();

        Ok(result)
    }

    fn list(&mut self, path: String) -> anyhow::Result<std::vec::Vec<String>> {
        let full_path = self.get_full_path(&path);
        let binding = self.client.as_mut().unwrap();
        let client = binding.lock().unwrap();
        
        let result = client.list(&[full_path]).unwrap();
        let result = result.iter().map(|x| x.href.clone()).collect::<Vec<String>>();

        Ok(result)
    }

    fn clear(&mut self, _path: String) -> anyhow::Result<()> {
        self.list("".to_string()).unwrap().iter().for_each(|x| {
            tracing::info!("delete {}", x);
            self.delete(x.clone()).unwrap();
        });
        
        self.create_work_dir().unwrap();

        Ok(())
    }

    fn delete(&mut self, path: String) -> anyhow::Result<()> {
        let full_path = self.get_full_path(&path);

        let binding = self.client.as_mut().unwrap();
        let client = binding.lock().unwrap();
        let _result = client.request(reqwest::Method::DELETE, &[full_path]).send().unwrap();

        Ok(())
    }

    fn mv(&mut self, src: String, dst: String) -> anyhow::Result<()> {
        let src_fp = self.get_full_path(&src);
        let dst_fp = self.get_full_path(&dst);

        let binding = self.client.as_mut().unwrap();
        let client = binding.lock().unwrap();
        let _result = client.mv(&[src_fp], &[dst_fp]).unwrap();

        Ok(())
    }

    fn exist(&mut self, path: String) -> bool {
        let full_path = self.get_full_path(&path);
        let binding = self.client.as_mut().unwrap();
        let client = binding.lock().unwrap();
        let result = client.list(&[full_path]);

        match result {
            Ok(res) => {
                if res.len() == 0 {
                    false
                } else {
                    true
                }
            },
            Err(_e) => {
                false
            },
        }
    }
}

impl WebDAV {
    fn get_full_path(&self, path: &String) -> String {
        let full_path = self.work_dir.clone() + "/" + &path;
        full_path
    }

    fn get_read_path(&self, path: &String) -> String {
        let read_path = "read/".to_string() + &path;
        read_path
    }

    fn get_read_full_path(&self, path: &String) -> String {
        let read_path = self.get_read_path(path);
        let full_path = self.get_full_path(&read_path);
        full_path
    }

    fn create_work_dir(&mut self) -> anyhow::Result<()> {
        let binding = self.client.as_mut().unwrap();
        let client = binding.lock().unwrap();
        let result = client.list(&["clouddrive-rpc"]);
        drop(client);

        match result {
            Ok(_res) => {},
            Err(_e) => {
                self.create_work_dir_no_check().unwrap();
            },
        }

        Ok(())
    }

    fn create_work_dir_no_check(&mut self) -> anyhow::Result<()> {
        let binding = self.client.as_mut().unwrap();
        let client = binding.lock().unwrap();
        client.mkcol(&["clouddrive-rpc"]).unwrap();

        Ok(())
    }

    fn mark_as_read_no_check(&mut self, path: String) -> anyhow::Result<()> {
        let read_full_path = self.get_read_full_path(&path);
        let full_path = self.get_full_path(&path);
        
        self.mv(full_path, read_full_path).unwrap();

        Ok(())
    }

    fn mark_as_read(&mut self, path: String) -> anyhow::Result<()> {
        self.mark_as_read_no_check(path).unwrap();

        Ok(())
    }
}

