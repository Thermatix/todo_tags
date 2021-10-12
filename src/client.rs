use tonic::{self, transport::Channel as TonicChannel};// Request, Response, Status};
use http::uri::Uri;

use super::gprc::{Action, tdt_client::TdtClient};
use super::envs;
use std::convert::From;

mod data_types {
    pub use super::super::gprc::{Item, Items, Query, Project,  ResType, Response};

}

#[derive(Debug)]
enum RequestResult {
    Response(data_types::Response),
    Project(data_types::Project),
    Item(data_types::Item),
    Items(data_types::Items),
}

impl From<data_types::Response> for RequestResult {
    fn from(v: data_types::Response) -> Self {
        Self::Response(v)
    }
}

impl From<RequestResult> for data_types::Response {
    fn from(v: RequestResult) -> Self {
        match v {
            RequestResult::Response(r) => r,
            _ => panic!("Expected RequestResult::, got {:#?}", v),
        }
    }
} 

impl From<data_types::Project> for RequestResult {
    fn from(v: data_types::Project) -> Self {
        Self::Project(v)

    }
}

impl From<RequestResult> for data_types::Project {
    fn from(v: RequestResult) -> Self {
        match v {
            RequestResult::Project(p) => p,
            _ => panic!("Expected RequestResult::, got {:#?}", v),
        }
    }
} 

impl From<data_types::Item> for RequestResult {
    fn from(v: data_types::Item) -> Self {
        Self::Item(v)
    }
}

impl From<RequestResult> for data_types::Item {
    fn from(v: RequestResult) -> Self {
        match v {
            RequestResult::Item(i) => i,
            _ => panic!("Expected RequestResult::, got {:#?}", v),
        }
    }
} 

impl From<data_types::Items> for RequestResult {
    fn from(v: data_types::Items) -> Self {
        Self::Items(v)
    }
}

impl From<RequestResult> for data_types::Items {
    fn from(v: RequestResult) -> Self {
        match v {
            RequestResult::Items(is) => is,
            _ => panic!("Expected RequestResult::, got {:#?}", v),
        }
    }
} 

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
            Action::Add(project) => Ok(self.client.add(data_types::Project::from(project)).await?.into_inner().into()),
            Action::Edit(project) => Ok(self.client.edit(data_types::Project::from(project)).await?.into_inner().into()),
            Action::Display(project) => Ok(self.client.display(data_types::Project::from(project)).await?.into_inner().into()),
            Action::Write(item) => Ok(self.client.write(data_types::Item::from(item)).await?.into_inner().into()),
            Action::Remove(item) => Ok(self.client.remove(data_types::Item::from(item)).await?.into_inner().into()),
            Action::Show(item) => Ok(self.client.show(data_types::Item::from(item)).await?.into_inner().into()),
            Action::Find(query) => Ok(self.client.find(data_types::Query::from(query)).await?.into_inner().into()),
            _ => panic!("Given action of {:#?} doesn't exist", action),
        }
    }

}
