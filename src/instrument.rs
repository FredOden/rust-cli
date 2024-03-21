use std::fmt;
use std::vec;
use std::thread;
use std::mem::drop;
//use std::cell::Cell;
use std::sync::RwLock;
use std::time::Duration;
use rand::{Rng};

#[derive(Debug)]
pub enum Kind {
    Equity(String),
    Bond(String),
    Warrant(String),
    Currency(String)
}

#[derive(Debug)]
pub struct Data {
    last:RwLock<f64>,
    bid: f64,
    ask: f64,
    open: f64,
    close: f64
}

#[derive(Debug)]
pub struct Instrument {
    kind: Kind,
    data: Data,
    subscribers:u32
}


impl Instrument {
    
    pub fn new(kind:Kind) -> Instrument {
        Instrument {
            kind,
            data : Data {
                last:RwLock::new(0f64),
                bid: 0f64,
                ask: 0f64,
                open: 0f64,
                close: 0f64
            },
            subscribers :0u32
        }
    }

    pub fn get_name(&self) -> &String {
        match &self.kind {
            Kind::Equity(equity) => equity,
            Kind::Bond(bond) => bond,
            Kind::Warrant(warrant) => warrant,
            Kind::Currency(currency) => currency,
        }
    }

    pub fn inc_subscribers(&mut self) {
        self.subscribers += 1;
    }
    pub fn on_image(&self) {
        println!("Image for {} {:?}", &self.get_name(), &self);
    }
    pub fn on_update(&self) {
        println!("Update {} entering", &self.get_name());
        let last = &self.data.last.read().unwrap();
        println!("Update for {} last {} {:?}", &self.get_name(), *last, &self);
    }
}


use std::collections::HashMap;

pub struct DataFeed<'a> {
    registry: HashMap<&'a String, &'a Instrument>,
    name: String,
    subscribed: HashMap<&'a String, &'a Instrument>,
}

impl<'a> DataFeed<'a> {
    pub fn new(name:String) -> DataFeed<'a> {
       DataFeed {
           name,
           registry : HashMap::new(),
           subscribed : HashMap::new()
       }
    }
    pub fn add(&mut self, i:&'a Instrument) {
        self.registry.insert(i.get_name(),
            i
        );
    }
    pub fn flush(&self) {
       for (_, v) in &self.subscribed {
           v.on_image();
       }
    }
    pub fn subscribe(&mut self, name:String) -> Result<&Instrument, String> {
        match self.registry.get(&name) {
            Some(&instrument) => {
                &self.subscribed.insert(&(instrument.get_name()),&instrument);
                instrument.on_image();
                Ok(&instrument)
            }
            _ => Err(format!("{} instrument not found", name))
        }
    }

    pub fn start(&self) {
        thread::scope(|scope| {

            for (k, i) in &self.registry {
                scope.spawn(move || {
                    println!("Starting {}", k);
                    let mut r = rand::thread_rng();
                    for _ in 1..10 {
                        let mut ms =  r.gen_range(0..1000);
                        thread::sleep(Duration::from_millis(ms));
                        let mut tmp = i.data.last.write().unwrap();
                        *tmp = ms as f64;
                        drop(tmp);
                        match &self.subscribed.get(i.get_name()) {
                            Some(j) => {
                                j.on_update();
                            }
                            _ => {}
                        }
                    }
                    println!("ending {}", k);
                });
            }

        });
        println!("finished");
    }
}


