use tonic::{transport::Channel as TonicChannel, Request, Response, Status};

mod data_types {
    pub use super::super::gprc::{Item, Items, Query, Project,  ResType, Response};
}

use super::gprc::tdt_client::TdtClient;
use super::envs;

use http::uri::Uri;

pub struct Client {
    client: TdtClient<TonicChannel>,
}

impl Client {
    pub async fn new() -> Self {
        {
            let channel = TonicChannel::builder(envs::server_addr().parse::<Uri>().unwrap())
                .connect()
                .await
                .expect("Can't create a channel");
            Self {
                client: TdtClient::new(channel)
            }
        }
    }
}
