#![allow(dead_code)]

use crate::config::{self, Config};
use crate::types::func_types::FuncTypes;
use crate::types::request::Request;
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
        if let Ok((stream, addr)) = self.socket_listener.accept() {
            println!("Connected: {addr}");
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
                if req.method == "shutdown" {
                    break;
                }

                self.handle_request(req);
            }
        }
        Ok(())
    }

    fn handle_request(&mut self, req: Request) -> Value {
        // Lol, i can't do a method
        let Request { method, params } = req;
        let func = self.methods.get_mut(&method).unwrap();

        if let Some(args) = params {
            Self::handle_method(func, args)
        } else {
            Self::handle_method(func, Params::None) // TAFAK AAHAH
        }
    }

    fn handle_method(func: &mut FuncTypes, params: Params) -> Value {
        match func {
            FuncTypes::MutingFunction(ref mut f) => f(params),
            FuncTypes::ImmutingFunction(ref f) => f(params),
        }
    }

    pub fn shutdown() -> Request {
        Request::new("Shutdown".to_string(), None)
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
        println!("Client-socket: {:?}", serde_json::json!(request));

        let serialized = serde_json::to_string(&request).expect("Can't serialize request!");
        let serialized = format!("{}\n", serialized);
        println!("{}", serialized);
        buf.write_all(serialized.as_bytes())
            .expect("Failed to write bytes");
    }
    pub fn send_something_with_args(&mut self, method: &str, params: Params) {
        let request = Request::new(method.into(), Some(params));

        let mut buf = BufWriter::new(&self.socket);
        println!("Client-socket: {:?}", serde_json::json!(request));

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

            server.insert(
                "fibbonacci".to_string(),
                FuncTypes::ImmutingFunction(Box::new(|params| {
                    // Positional Params
                    let mut f;
                    match params {
                        Params::Positional(vec) => {
                            f = fib(serde_json::from_value(vec[0].clone()).unwrap())
                        }
                        _ => unreachable!(),
                    };
                    println!("{}", f);

                    Value::Null
                })),
            );
            // TODO! Test Muting Function..
            server.handle().unwrap();
        });

        fn fib(n: usize) -> i32 {
            let mut vals = (1, 0);
            for _ in 0..n {
                vals = (vals.1, vals.0 + vals.1);
            }
            vals.1
        }

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
        client
            .send_something_with_args("fibbonacci", Params::Positional(vec![serde_json::json!(5)]));
        client.send_something("shutdown");
        s.join().unwrap();
    }

    /*
    #[test]
    fn test_rpc_2() {
        let s = std::thread::spawn(|| {
            let mut server = Server::new().expect("Can't create server!");
            server.config.change_port("8081".to_string());
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
        client.send_something("shutdown");
        s.join().unwrap();
    }
    */
}
