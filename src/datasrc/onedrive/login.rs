use onedrive_api::{OneDrive, FileName, DriveLocation, ItemLocation};
use hyperdav::Client;

use crate::datasrc::prototype::DataSource;

pub async fn connect() -> anyhow::Result<()> {
    let client = Client::new();
    let drive = OneDrive::new(
        "token", // Login token to Microsoft Graph.
        DriveLocation::me(),
    );
    
    let folder_item = drive
        .create_folder(
            ItemLocation::root(),
            FileName::new("test_folder").unwrap(),
        )
        .await?;
    
    drive
        .upload_small(
            folder_item.id.as_ref().unwrap(),
            &b"Hello, world"[..],
        )
        .await?;

    Ok(())
}

// impl DataSource for OneDrive {
//     fn connect(&mut self, url: &str, opt: Option<Vec<String>>) -> anyhow::Result<()> {
//         Ok(())
//     }

//     fn disconnect(&mut self) -> anyhow::Result<()> {
//         todo!()
//     }

//     fn put(&mut self, id: u64, msg: &SerMsg) -> anyhow::Result<()> {
//         todo!()
//     }

//     fn get(&mut self, id: u64) -> anyhow::Result<std::vec::Vec<u8>> {
//         todo!()
//     }

//     fn list(&mut self) -> anyhow::Result<std::vec::Vec<u64>> {
//         todo!()
//     }

//     fn clear(&mut self) -> anyhow::Result<()> {
//         todo!()
//     }

//     fn delete(&mut self, id: u64) -> anyhow::Result<()> {
//         todo!()
//     }
// }
