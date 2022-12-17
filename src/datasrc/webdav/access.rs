use std::{io::Cursor};

use hyperdav::Client;

use crate::{datasrc::{prototype::{DataSource, Message}, message::message::{RpcMessage, RpcMessageMeta}}, conf::conf::CONF};

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

    fn put(&mut self, path: String, msg: &Vec<u8>) -> anyhow::Result<()> {
        let binding = self.client.as_mut().unwrap();
        let client = binding.lock().unwrap();

        tracing::trace!("putting {:?} into {:?}", msg, path);
        client.put(Cursor::new(msg.clone()), &[path]).unwrap();

        Ok(())
    }

    fn get(&mut self, path: String) -> anyhow::Result<std::vec::Vec<u8>> {
        let binding = self.client.as_mut().unwrap();
        let client = binding.lock().unwrap();

        tracing::trace!("getting {:?}", path);
        let mut result = client.get(&[path]).unwrap();
        let result = result.text().unwrap().as_bytes().to_vec();

        Ok(result)
    }

    fn list(&mut self, path: String) -> anyhow::Result<std::vec::Vec<String>> {
        let full_path = self.get_full_path(&path);
        let binding = self.client.as_mut().unwrap();
        let client = binding.lock().unwrap();
        
        let result = client.list(&[full_path]).unwrap();
        let result = result.iter().map(|x| {
            let conf = CONF.get().unwrap();
            let href = x.href.clone();
            let work_dir = conf.workdir.clone() + "/";
            let href = href.replace(&work_dir, "");
            href
        }).collect::<Vec<String>>();

        tracing::trace!("list result: {:?}", result);

        Ok(result)
    }

    fn clear(&mut self, _path: String) -> anyhow::Result<()> {
        self.list("".to_string()).unwrap().iter().for_each(|x| {
            self.delete(x.clone()).unwrap();
        });
        
        self.create_work_dir().unwrap();

        Ok(())
    }

    fn delete(&mut self, path: String) -> anyhow::Result<()> {
        let full_path = self.get_full_path(&path);
        tracing::trace!("deleting {:?}", full_path);

        let binding = self.client.as_mut().unwrap();
        let client = binding.lock().unwrap();
        let _result = client.request(reqwest::Method::DELETE, &[full_path]).send().unwrap();

        Ok(())
    }

    fn mv(&mut self, src: String, dst: String) -> anyhow::Result<()> {
        let binding = self.client.as_mut().unwrap();
        let client = binding.lock().unwrap();

        tracing::trace!("moving {:?} to {:?}", src, dst);
        let _result = client.mv(&[src], &[dst]).unwrap();

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
    pub fn get_by_rel_path(&mut self, path: String) -> anyhow::Result<std::vec::Vec<u8>> {
        let full_path = self.get_full_path(&path);
        let result = self.get(full_path).unwrap();

        Ok(result)
    }

    pub fn put_by_rel_path(&mut self, path: String, msg: &Vec<u8>) -> anyhow::Result<()> {
        let full_path = self.get_full_path(&path);
        self.put(full_path, msg).unwrap();

        Ok(())
    }
}

impl WebDAV {
    fn get_full_path(&self, path: &String) -> String {
        let full_path = self.work_dir.clone() + "/" + &path;
        full_path
    }

    fn get_read_path(&self, path: &String) -> String {
        let read_path = path.replace("unread", "read");
        read_path
    }

    fn get_read_full_path(&self, path: &String) -> String {
        let read_path = self.get_read_path(path);
        let full_path = self.get_full_path(&read_path);
        full_path
    }

    fn create_work_dir(&mut self) -> anyhow::Result<()> {
        let conf = CONF.get().unwrap();

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

        let binding = self.client.as_mut().unwrap();
        let client = binding.lock().unwrap();
        let path = format!("clouddrive-rpc/{}", conf.get_node_id());
        let result = client.list(&[&path]);
        match result {
            Ok(_res) => {},
            Err(_e) => {
                client.mkcol(&[&path]).unwrap();
            },
        }
        drop(client);

        let binding = self.client.as_mut().unwrap();
        let client = binding.lock().unwrap();
        let path = format!("clouddrive-rpc/{}/read", conf.get_node_id());
        let result = client.list(&[&path]);
        match result {
            Ok(_res) => {},
            Err(_e) => {
                client.mkcol(&[&path]).unwrap();
            },
        }
        drop(client);

        let binding = self.client.as_mut().unwrap();
        let client = binding.lock().unwrap();
        let path = format!("clouddrive-rpc/{}/unread", conf.get_node_id());
        let result = client.list(&[&path]);
        match result {
            Ok(_res) => {},
            Err(_e) => {
                client.mkcol(&[&path]).unwrap();
            },
        }
        drop(client);

        Ok(())
    }

    fn create_work_dir_no_check(&mut self) -> anyhow::Result<()> {
        let binding = self.client.as_mut().unwrap();
        let client = binding.lock().unwrap();
        client.mkcol(&["clouddrive-rpc"]).unwrap();

        Ok(())
    }

    fn mark_as_read_no_check(&mut self, path: String) -> anyhow::Result<()> {
        let read_path = self.get_read_path(&path);
        
        self.mv(path, read_path).unwrap();

        Ok(())
    }

    pub fn mark_as_read(&mut self, path: String) -> anyhow::Result<()> {
        tracing::trace!("marking {:?} as read", path);
        self.mark_as_read_no_check(path).unwrap();

        Ok(())
    }

    fn mark_as_read_by_rel_path_no_check(&mut self, path: String) -> anyhow::Result<()> {
        let read_full_path = self.get_read_full_path(&path);
        let full_path = self.get_full_path(&path);
        
        self.mv(full_path, read_full_path).unwrap();

        Ok(())
    }

    pub fn mark_as_read_by_rel_path(&mut self, path: String) -> anyhow::Result<()> {
        self.mark_as_read_by_rel_path_no_check(path).unwrap();

        Ok(())
    }
}

