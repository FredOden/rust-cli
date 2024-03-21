use std::thread;
use std::mem::drop;
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
    last:f64,
    bid: f64,
    ask: f64,
    open: f64,
    close: f64,
    tick: usize
}

/// because DataFeed is multithreaded
/// instrument data is managed
/// with RwLock read and write
#[derive(Debug)]
pub struct RwData {
    rw:RwLock<Data>
}

#[derive(Debug)]
pub struct Instrument {
    kind: Kind,
    data: RwData,
    subscribers:RwLock<usize>
}


impl Instrument {

    pub fn new(kind:Kind) -> Instrument {
        Instrument {
            kind,
            data : RwData { rw:RwLock::new(Data {
                last:0f64,
                bid: 0f64,
                ask: 0f64,
                open: 0f64,
                close: 0f64,
                tick: 0
            })
            },
            subscribers : RwLock::new(0usize)
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

    pub fn get_subscribers(&self) -> usize {
        let s = self.subscribers.read().unwrap();
        *s
    }

    pub fn on_image(&self) {
        println!("Image for {} {:?}", &self.get_name(), &self);
    }
    pub fn on_update(&self) {
        let data = self.data.rw.read().unwrap();
        println!("Update for {} {:?}", &self.get_name(), *data);
        drop(data);
    }
}


use std::collections::HashMap;

pub struct DataFeed<'a> {
    registry: HashMap<&'a String, &'a Instrument>,
    name: String,
}

impl<'a> DataFeed<'a> {
    pub fn new(name:String) -> DataFeed<'a> {
        DataFeed {
            name,
            registry : HashMap::new(),
        }
    }
    pub fn add(&mut self, i:&'a Instrument) {
        self.registry.insert(i.get_name(),
        i
        );
    }
    pub fn flush(&self) {
        for (_, v) in &self.registry {
            if v.get_subscribers() > 0 {
                v.on_image();
            }
        }
    }
    pub fn subscribe(&mut self, name:String) -> Result<&Instrument, String> {
        match self.registry.get(&name) {
            Some(&instrument) => {
                let mut s = instrument.subscribers.write().unwrap();
                *s += 1;
                drop(s);
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
                        let ms =  r.gen_range(0..1000);
                        thread::sleep(Duration::from_millis(ms));
                        let mut tmp = i.data.rw.write().unwrap();
                        (*tmp).last = ms as f64;
                        (*tmp).tick += 1;
                        drop(tmp);
                        let s = i.subscribers.read().unwrap();
                        let subscribers = *s;
                        drop(s);
                        if subscribers > 0 {
                            i.on_update();
                        }
                    }
                    println!("ending {}", k);
                });
            }

        });
        println!("finished");
    }
}


