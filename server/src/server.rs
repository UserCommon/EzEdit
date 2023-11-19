#![allow(dead_code)]
use crate::config::{self, Config};
use crate::types::func_types::FuncTypes;
use crate::types::request::Request;
use crate::types::response::Response;
use crate::types::*;
use params::Params;
use serde_json::{Result, Value};
use std::collections::HashMap;
use std::fmt::format;
use std::io::{prelude::*, BufWriter};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{IpAddr, TcpListener, TcpStream};
use std::str;

/// TcpStream abstraction
pub struct Server {
    socket_listener: TcpListener,
    config: Config,
    methods: HashMap<String, FuncTypes>,
}

impl Server {
    pub fn new() -> std::io::Result<Self> {
        let config: Config = Config::from_config();
        println!("{}", config.get_url());
        Ok(Self {
            socket_listener: TcpListener::bind(config.get_url())?,
            config: config.clone(),
            methods: HashMap::new(),
        })
    }

    pub fn insert(&mut self, name: String, func: FuncTypes) {
        self.methods.insert(name, func);
    }

    pub fn handle(&mut self) -> std::io::Result<()> {
        loop {
            if let Ok((stream, addr)) = self.socket_listener.accept() {
                loop {
                    let mut buffered = BufReader::new(&stream);
                    let mut data: Vec<u8> = Vec::new();

                    let symbols = buffered.read_until(b'\n', &mut data)?; // May block! I need to do
                                                                          // something with it
                                                                          /*
                                                                                             if symbols == 0 {
                                                                                                 return Ok(());
                                                                                             }
                                                                          */
                    let req_str = str::from_utf8(&data).unwrap();
                    let req: Request = serde_json::from_str(req_str)?;
                    println!("{:#?}", req);

                    self.handle_request(req);
                }
            }
        }
        Ok(())
    }

    fn handle_request(&mut self, req: Request) -> Value {
        let Request { method, params } = req;
        match params {
            Some(type_of_params) => match type_of_params {
                Params::Positional(vec) => Value::Null,
                Params::None => {
                    let func = self.methods.get_mut(&method).unwrap(); // Change to Uwrap or Error, INVALID
                                                                       // METHOD!
                    match func {
                        FuncTypes::MutingFunction(f) => f(Params::None),
                        FuncTypes::ImmutingFunction(f) => f(Params::None),
                    }
                }
                _ => Value::Null,
            },
            None => {
                let func = self.methods.get_mut(&method).unwrap(); // Change to Uwrap or Error, INVALID
                                                                   // METHOD!
                match func {
                    FuncTypes::MutingFunction(f) => f(Params::None),
                    FuncTypes::ImmutingFunction(f) => f(Params::None),
                }
            }
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
        let request = Request::new(method.into(), None);

        let mut buf = BufWriter::new(&self.socket);

        println!(
            "Client-socket: {:?}, {:?}",
            self.socket,
            serde_json::json!(request)
        );

        let serialized = serde_json::to_string(&request).expect("Can't serialize request!");
        let serialized = format!("{}\n", serialized);
        println!("{}", serialized);
        buf.write_all(serialized.as_bytes())
            .expect("Failed to write bytes");
    }
}

#[cfg(test)]
mod socket_test {
    use super::*;

    #[test]
    fn test_rpc_1() {
        let s = std::thread::spawn(|| {
            let mut server = Server::new().expect("Can't create server!");
            server.insert(
                "hello_world".to_string(),
                FuncTypes::ImmutingFunction(Box::new(|params| {
                    println!("Hello world!");
                    Value::Null
                })),
            );
            server.insert(
                "hello_body".to_string(),
                FuncTypes::ImmutingFunction(Box::new(|params| {
                    println!("Hey body!");
                    Value::Null
                })),
            );
            server.handle().unwrap();
        });
        let mut client;
        loop {
            let connect = ClientForTesting::new();
            if let Ok(val) = connect {
                println!("Connected!");
                client = val;
                break;
            }
        }

        client.send_something("hello_world");
        client.send_something("hello_body");
        client.send_something("hello_world");
        client.send_something("hello_body");
        s.join().unwrap();
    }
}
