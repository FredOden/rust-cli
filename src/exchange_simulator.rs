use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};

#[path="threadpool.rs"] mod threadpool;
use threadpool::ThreadPool;

#[derive(Serialize, Deserialize, Debug)]
pub struct Dictionary {
    symbols: std::collections::HashMap<String, DailyData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DailyData {
    name : String,
    kind : String,
    close : f32,
    #[serde(default = "default_volatility")]
    volatility : f32,
}

fn default_volatility() -> f32 {
    0.04
}

fn load(path: &str) {
    println!(">>>>>>>>>in load");
    let contents = fs::read_to_string(path).unwrap();
    match serde_json::from_str::<Dictionary>(&contents) {
        Ok(dictionary) => {
            println!("dictionary::{:#?}", dictionary);
            for(k, s) in dictionary.symbols {
                println!("{} -> {:#?}", k, s);
            }
        }
        Err(e) => {
            eprintln!("ERROR JSON::{}", e);
        }
    }
    println!("<<<<<<<<<out load");
}

pub fn start_exchange() {

    thread::spawn(|| {
        load("./data.json");
    });

    println!("start listenibg");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() //.take(2)
    {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.");
}
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    println!("handle connection");
    stream.read(&mut buffer).unwrap();
    let mut sb = std::str::from_utf8(&buffer);

    match sb {
        Ok(s) => {
            for line in 0..3 {
                let contents = format!("line ::{}<p>", line);
                let response = format!(
                    "{}->{}",
                    s,
                    contents
                );
                stream.write_all(response.as_bytes()).unwrap();
                println!("response::{}", response);
                stream.flush().unwrap();
                thread::sleep(Duration::from_secs(1));
            }
            println!("!!!!!!Exchange socket shutdown do");
            stream.shutdown(std::net::Shutdown::Both);
            println!("!!!!!!Exchange socket shutdown done");
        }
        _ => { return; }
    }
}
/*
   fn handle_connection(mut stream: TcpStream) {
   let mut buffer = [0; 1024];
   println!("handle connection");
   stream.read(&mut buffer).unwrap();
   let get = b"GET / HTTP/1.1\r\n";
   let sleep = b"GET /sleep HTTP/1.1\r\n";
   let (status_line, filename) = if buffer.starts_with(get) {
   ("HTTP/1.1 200 OK", "hello.html")
   } else if buffer.starts_with(sleep) {
   thread::sleep(Duration::from_secs(5));
   ("HTTP/1.1 200 OK", "hello.html")
   } else {
   ("HTTP/1.1 404 NOT FOUND", "404.html")
   };
//let contents = fs::read_to_string(filename).unwrap();
for line in 0..20 {
let contents = format!("line ::{}<p>", line);
let response = format!(
"{}\r\nContent-Length: {}\r\n\r\n{}",
status_line,
contents.len(),
contents
);
stream.write_all(response.as_bytes()).unwrap();
println!("response::{}", response);
thread::sleep(Duration::from_secs(1));
}
stream.flush().unwrap();
}
*/
/*
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn ooo() {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
}
*/
