use serde::Serialize;
use argparse::{ArgumentParser, Store, StoreTrue, List};

use once_cell::sync::OnceCell;
pub static CONF: once_cell::sync::OnceCell<Conf> = OnceCell::new();

#[derive(Clone, Debug, Serialize)]
pub struct Conf {
    username: String,
    password: String,
    uri: String,
}

impl Conf {
    pub fn new() -> Self {
        Conf {
            username: String::new(),
            password: String::new(),
            uri: String::new(),
        }
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub fn get_uri(&self) -> &str {
        &self.uri
    }

    pub fn set_username(&mut self, username: &str) {
        self.username = username.to_string();
    }

    pub fn set_password(&mut self, password: &str) {
        self.password = password.to_string();
    }

    pub fn set_uri(&mut self, uri: &str) {
        self.uri = uri.to_string();
    }

    pub fn parse_args() {
        let mut conf = Conf::new();

        {
            let mut ap = ArgumentParser::new();
            ap.set_description("CloudDrive RPC");
            ap.refer(&mut conf.username)
                .add_option(&["-u", "--username"], Store, "Username");
            ap.refer(&mut conf.password)
                .add_option(&["-p", "--password"], Store, "Password");
            ap.refer(&mut conf.uri)
                .add_option(&["-U", "--uri"], Store, "URI");
            ap.parse_args_or_exit(); 
        }
    
        CONF.get_or_init(|| {
            tracing::info!("{:?}", conf);
            conf
        });
    }
}
