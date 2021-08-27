use tonic::{transport::Server as TonicServer, Request, Response, Status};
use super::gprc::say_server::{Say, SayServer};

mod data_types {
    pub use super::super::gprc::{Item, Items, Query, Project,  ResType, Response};
}

pub struct Server {
    address: std::net::SocketAddr,
    tonic_server: TonicServer,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self {
            address: address.parse().unwrap(),
            tonic_server: TonicServer::builder(),
        }
    }

    pub async fn start(&mut self) -> Result<(), tonic::transport::Error> {
        self.tonic_server.add_service(SayServer::new(TagService::default())).serve(self.address).await
    }

}


#[derive(Default)]
struct TagService {}

#[tonic::async_trait]
impl Say for TagService {
    /// Add a project
    async fn add(&self, request: Request<data_types::Project>)->Result<Response<data_types::Response>, Status> {
       //TODO: Impliment add project functionality 
        unimplemented!()
    }
    /// Edit a project
    async fn edit(&self,request: Request<data_types::Project>)->Result<Response<data_types::Response>, Status> {
       //TODO: Impliment edit project functionality 
        unimplemented!()
    }

    /// Display a project
    async fn display(&self,request: Request<data_types::Project>)->Result<Response<data_types::Project>, Status> {
       //TODO: Impliment display project functionality 
        unimplemented!()
    }

    /// Update an item
    async fn write(&self,request: Request<data_types::Item>)->Result<Response<data_types::Response>, Status> {
       //TODO: Impliment write item functionality 
        unimplemented!()
    }
    /// Remove an item
    async fn remove(&self,request: Request<data_types::Item>)->Result<Response<data_types::Response>, Status> {
       //TODO: Impliment write item functionality 
        unimplemented!()
    }
    /// Show an item
    async fn show(&self,request: Request<data_types::Item>)->Result<Response<data_types::Item>, Status> {
       //TODO: Impliment write item functionality 
        unimplemented!()
    }
    /// Find matching items
    async fn find(&self,request: Request<data_types::Query>)->Result<Response<data_types::Items>, Status> {
       //TODO: Impliment write item functionality 
        unimplemented!()
    }
    // async fn send(&self,request:Request<SayRequest>)->Result<Response<SayResponse>,Status>{
    //     Ok(Response::new(SayResponse{
    //         message:format!("hello {}",request.get_ref().name),
    //     }))
    // }
}

