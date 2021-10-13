use tonic::{self, transport::Channel as TonicChannel};// Request, Response, Status};
use http::uri::Uri;

use super::gprc::{Action, tdt_client::TdtClient, data_types};
use super::envs;
use std::convert::From;

define_container!(RequestResult: {
    Response: data_types::Response,
    Project: data_types::Project,
    Items: data_types::Items,
    Item: data_types::Item,
});

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
                client: TdtClient::new(channel),
            }
        }
    }

    pub async fn request(&mut self, action: Action) -> Result<RequestResult, tonic::Status> {
        match action {
            Action::Add(project) => Ok(self.client.add(project).await?.into_inner().into()),
            Action::Edit(project) => Ok(self.client.edit(project).await?.into_inner().into()),
            Action::Display(project) => Ok(self.client.display(project).await?.into_inner().into()),
            Action::Write(item) => Ok(self.client.write(item).await?.into_inner().into()),
            Action::Remove(item) => Ok(self.client.remove(item).await?.into_inner().into()),
            Action::Show(item) => Ok(self.client.show(item).await?.into_inner().into()),
            Action::Find(query) => Ok(self.client.find(query).await?.into_inner().into()),
            _ => panic!("Given action of {:#?} doesn't exist", action),
        }
    }

}
