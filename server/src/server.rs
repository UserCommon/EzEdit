#![allow(dead_code)]

use crate::config::{self, Config};
use crate::types::func_types::FuncTypes;
use crate::types::request::Request;
use crate::types::response::Response;
use crate::types::*;
use params::Params;
use serde_json::Result;
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{IpAddr, TcpStream, TcpListener};

/// TcpStream abstraction
pub struct Server {
    socket_listener: TcpListener,
    socket_stream: TcpStream,
    config: Config,
    methods: HashMap<String, FuncTypes>,
}

impl Server {
    pub fn new() -> std::io::Result<Self> {
        let config: Config = Config::from_config();
        println!("{}", config.get_url());
        Ok(Self {
            socket_listener: TcpListener::bind(config.get_url())?,
            socket_stream: TcpStream::connect(config.get_url())?,
            config: config.clone(),
            methods: HashMap::new(),
        })
    }

    pub fn insert(&mut self, name: String, func: FuncTypes) {
        self.methods.insert(name, func);
    }

    pub fn handle(&mut self) -> std::io::Result<()> {
        let req: Request = serde_json::Deserializer::from_reader(&self.socket_stream); // Stuck
        self.handle_request(req);
        Ok(())
    }

    fn handle_request(&mut self, req: Request) -> Response {
        let Request { method, params } = req;
        match params {
            Some(type_of_params) => match type_of_params {
                Params::Positional(vec) => Response {
                    result: None,
                    error: None,
                },
                Params::None => {
                    let func = self.methods.get_mut(&method).unwrap(); // Change to Uwrap or Error, INVALID
                                                                       // METHOD!
                    match func {
                        FuncTypes::MutingFunction(f) => f(Params::None),
                        FuncTypes::ImmutingFunction(f) => f(Params::None),
                    }
                }
                _ => Response {
                    result: None,
                    error: None,
                },
            },
            None => {
                let func = self.methods.get_mut(&method).unwrap(); // Change to Uwrap or Error, INVALID
                                                                       // METHOD!
                match func {
                    FuncTypes::MutingFunction(f) => f(Params::None),
                    FuncTypes::ImmutingFunction(f) => f(Params::None),
                }
            },
        }
    }
}

struct ClientForTesting {
    socket: TcpStream,
    config: Config,
}

impl ClientForTesting {
    pub fn new() -> std::io::Result<Self> {
        let config: Config = Config::from_config();
        Ok(Self {
            socket: TcpStream::connect(config.get_url())?,
            config: config.clone(),
        })
    }

    pub fn send_something(&mut self, method: &str) {
        let request = format!(
            r#"
            {{
                "method": "{}"
            }}
            "#,
            method
        );
        let req: Request = serde_json::from_str(&request).expect("cant deserialize!");

        serde_json::to_writer(&self.socket, &req);
    }
}

#[cfg(test)]
mod socket_test {
    use super::*;

    #[test]
    fn test_rpc_1() {
        let mut server = Server::new().expect("Can't create server!");
        let mut client = ClientForTesting::new().expect("Can't create client!");
        server.insert("hello_world".to_string(), FuncTypes::ImmutingFunction(Box::new(|Params| { 
            println!("Hello world!");
            Response {
                result: None,
                error: None
            }
        })));
        client.send_something("hello_world");
        server.handle().unwrap();

    }
}
