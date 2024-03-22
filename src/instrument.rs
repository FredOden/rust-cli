// Import necessary modules from the standard library.
use std::thread;
use std::mem::drop;
use std::sync::RwLock;
use std::time::Duration;
use rand::{Rng}; // Import the Rng trait from the rand crate for random number generation.

// Define an enumeration to represent different kinds of financial instruments.
#[derive(Debug)]
pub enum Kind {
    Equity(String),
    Bond(String),
    Warrant(String),
    Currency(String)
}

// Define a struct to hold data related to financial instruments.
#[derive(Debug)]
pub struct Data {
    last: f64,   // The last traded price.
    bid: f64,    // The current bid price.
    ask: f64,    // The current ask price.
    open: f64,   // The opening price.
    close: f64,  // The closing price.
    tick: usize  // A counter for the number of ticks (price changes).
}

// Define a struct to hold financial instrument data with thread-safe read/write access.
#[derive(Debug)]
pub struct RwData {
    rw: RwLock<Data> // Use a read-write lock to manage concurrent access to the data.
}

// Define a struct to represent a financial instrument.
#[derive(Debug)]
pub struct Instrument {
    kind: Kind, // The kind of instrument (Equity, Bond, etc.).
    data: RwData, // The data associated with the instrument.
    subscribers: RwLock<usize> // A count of subscribers interested in updates for this instrument.
}

// Implement methods for the Instrument struct.
impl Instrument {
    // Constructor method to create a new Instrument instance.
    pub fn new(kind: Kind) -> Instrument {
        Instrument {
            kind,
            data: RwData {
                rw: RwLock::new(Data {
                    last: 0f64,
                    bid: 0f64,
                    ask: 0f64,
                    open: 0f64,
                    close: 0f64,
                    tick: 0
                })
            },
            subscribers: RwLock::new(0usize)
        }
    }

    // Method to retrieve the name of the instrument based on its kind.
    pub fn get_name(&self) -> &String {
        match &self.kind {
            Kind::Equity(equity) => equity,
            Kind::Bond(bond) => bond,
            Kind::Warrant(warrant) => warrant,
            Kind::Currency(currency) => currency,
        }
    }

    // Method to get the number of subscribers.
    pub fn get_subscribers(&self) -> usize {
        let s = self.subscribers.read().unwrap();
        *s
    }

    // Method to simulate sending an image update to subscribers.
    pub fn on_image(&self) {
        println!("Image for {} {:?}", &self.get_name(), &self);
    }

    // Method to simulate sending a data update to subscribers.
    pub fn on_update(&self) {
        let data = self.data.rw.read().unwrap();
        println!("Update for {} {:?}", &self.get_name(), *data);
        drop(data); // Explicitly drop the read lock to release it.
    }
}

// Import the HashMap collection from the standard library.
use std::collections::HashMap;

// Define a struct to manage a collection of instruments and their updates.
pub struct DataFeed<'a> {
    registry: HashMap<&'a String, &'a Instrument>, // A registry mapping instrument names to their references.
    name: String, // The name of the data feed.
}

// Implement methods for the DataFeed struct.
impl<'a> DataFeed<'a> {
    // Constructor method to create a new DataFeed instance.
    pub fn new(name: String) -> DataFeed<'a> {
        DataFeed {
            name,
            registry: HashMap::new(),
        }
    }

    // Method to add an instrument to the registry.
    pub fn add(&mut self, i: &'a Instrument) {
        self.registry.insert(i.get_name(), i);
    }

    // Method to simulate sending image updates to all subscribed instruments.
    pub fn flush(&self) {
        for (_, v) in &self.registry {
            if v.get_subscribers() > 0 {
                v.on_image();
            }
        }
    }

    // Method to subscribe to an instrument by name and receive updates.
    pub fn subscribe(&mut self, name: String) -> Result<&Instrument, String> {
        match self.registry.get(&name) {
            Some(&instrument) => {
                let mut s = instrument.subscribers.write().unwrap();
                *s += 1;
                drop(s); // Explicitly drop the write lock to release it.
                instrument.on_image();
                Ok(&instrument)
            }
            _ => Err(format!("{} instrument not found", name))
        }
    }

    // Method to start the data feed and simulate instrument updates.
    pub fn start(&self) {
        thread::scope(|scope| {
            for (k, i) in &self.registry {
                scope.spawn(move || {
                    println!("Starting {}", k);
                    let mut r = rand::thread_rng();
                    for _ in 1..10 {
                        let ms = r.gen_range(0..1000);
                        thread::sleep(Duration::from_millis(ms));
                        let mut tmp = i.data.rw.write().unwrap();
                        (*tmp).last = ms as f64;
                        (*tmp).tick += 1;
                        drop(tmp); // Explicitly drop the write lock to release it.
                        let s = i.subscribers.read().unwrap();
                        let subscribers = *s;
                        drop(s); // Explicitly drop the read lock to release it.
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

