use tonic::{transport::Server as TonicServer, Request, Response, Status};
use daemonize::Daemonize;
use fork::{fork, Fork};

use std::process::Command;
use std::fs::{self, File};


use super::gprc::tdt_server::{Tdt, TdtServer};
use super::envs;

mod data_types {
    pub use super::super::gprc::{Item, Items, Query, Project,  ResType, Response};
}

pub struct Server {
    address: String,
    tonic_server: TonicServer,
    child: bool,
}

impl Drop for Server {
     fn drop(&mut self) {
        if self.child {
            println!("Stopping TDT server...");
            fs::remove_file(envs::pid_file());
        }
    }
}

impl Server {
    pub fn new() -> Self {
        Self {
            address: envs::server_addr(),
            tonic_server: TonicServer::builder(),
            child: false,
        }
    }

    pub async fn start(&mut self) -> Option<i32> {
        let pid_file = envs::pid_file();
        if let Ok(pid) = fs::read_to_string(pid_file) {
            Some(i32::from_str_radix(&pid, 10).unwrap())
        } else {
            match fork() {
                Ok(Fork::Child) => {
                    self.child = true;
                    let std_out = File::create(envs::std_out()).unwrap();
                    let std_err = File::create(envs::std_err()).unwrap();
                    let daemonize = Daemonize::new()
                        .pid_file(envs::pid_file())
                        .chown_pid_file(envs::chown_pid() == "true") 
                        .working_directory(&envs::working_dir())
                        //.user(envs::user().as_str())
                        //.group(envs::group().as_str())
                        .umask(u32::from_str_radix(&envs::umask(), 8).unwrap())
                        .stdout(std_out)
                        .stderr(std_err);

                    match daemonize.start() {
                        Ok(_) => {
                            println!("Started TDT server");
                        },
                        Err(e) => {
                            eprintln!("Error starting tdt server, {}", e);
                        },
                    };
                    println!("Listening on {}", self.address);
                    match self.tonic_server
                        .add_service(
                            TdtServer::new(
                                TagService::default()
                                )
                            )
                        .serve(self.address.parse().unwrap()).await {
                        Ok(_) => (),
                        Err(e) => eprintln!("Unable to start listening, {}", e),
                    };
                    None
                    
                },
                Ok(Fork::Parent(child)) => {
                    println!("Started TDT server as it has not already been started with pid: {}", child);
                    Some(child)
                },
                _ => {
                    eprintln!("Unable to fork");
                    None
                },
            }
        }
    }
}


#[derive(Default)]
struct TagService {}

#[tonic::async_trait]
impl Tdt for TagService {
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
    // async fn send(&self,request:Request<TdtRequest>)->Result<Response<TdtResponse>,Status>{
    //     Ok(Response::new(TdtResponse{
    //         message:format!("hello {}",request.get_ref().name),
    //     }))
    // }
}

