use std::thread;
use std::net::TcpStream;
use std::net::SocketAddr;
use std::io::Write;
use std::io::Read;

pub fn request<'a>(host: &'a str, port: u16, what: &'a str) {
    match TcpStream::connect(format!("{}:{}", host, port)) {

        Ok(mut stream) => {
            println!("connected stream::{:?}", stream);
            stream.write(what.as_bytes()).unwrap();
            stream.flush().unwrap();
            let mut buffer = [0; 1024];
            println!("handle connection");
            for i in 0..5 {
                println!("::::::::Reading for {}", what);
                match stream.read(&mut buffer) {
                    Err(e) => {
                        eprintln!("Error reading from exchange {}", e);
                        break;
                    }
                    Ok(n) => {
                        if n < 1 {
                            println!("connectopn closed");
                            break;
                        }
                        let mut sb = std::str::from_utf8(&buffer).unwrap();
                        println!("from exchange({}) <- {}", what, sb);
                    }
                }
            }
        }
        Err(e) => {
            println!("client Error::{}", e);
        }
    }
}
