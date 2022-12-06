use hyperdav::Client;

pub struct WebDAV {
    pub client: Option<std::sync::Arc<std::sync::Mutex<Client>>>,
    pub work_dir: String,
}

impl WebDAV {
    pub fn new() -> Self {
        WebDAV {
            client: None, 
            work_dir: String::from("clouddrive-rpc/"),
        }
    }

    pub fn is_connected(&self) -> bool {
        self.client.is_some()
    }
}