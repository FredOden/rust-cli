use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;
#[path = "instrument.rs"] mod instrument;


#[derive(Debug)]
enum OutType {
    File,
    StdOut
}

impl FromStr for OutType {
    type Err = String;
    fn from_str(file:&str) -> Result<Self, Self::Err> {
        match file {
            "-" | "stdout" => Ok(OutType::StdOut),
            "f" | "file" => Ok(OutType::File),
            _ => Err(std::format!("output type should be '-' or 'f', but is '{}'", file))
        }
    }

}

#[derive(StructOpt, Debug)]
#[structopt(name = "cli new")]
#[structopt(version = "0.1.2")]
#[structopt(about = "Pippo evaluates rust")]
struct Opt {
    #[structopt(short, long)]
    debug: bool,

    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Set rate avg update/sec
    #[structopt(short, long, default_value = "42")]
    rate: f64,

    /// list of instruments to subscrie
    #[structopt(short, long)]
    subscribe: Vec<String>,

    /// type of output : stdout or file
    #[structopt(short="t", long)]
    out_type: Option<OutType>,

    ///datafeed
    #[structopt(short, long)]
    feed: String
}

fn main() {
    match Opt::from_args_safe() {
        Ok(opt) => {
            println!("opt = {:#?}", opt);
            do_it(&opt);
        }
        Err(e) => {
            println!("problem: {}", e);
        }
    }
}

fn do_it(opt : &Opt) {  
    let mut reuters = instrument::DataFeed::new(opt.feed.to_string());

    let dictionary = vec![
        instrument::Instrument::new(instrument::Kind::Equity("TOTF.PA".to_string())),
        instrument::Instrument::new(instrument::Kind::Equity("MSFT.O".to_string())),
        instrument::Instrument::new(instrument::Kind::Currency("EUR=".to_string())),
        instrument::Instrument::new(instrument::Kind::Currency("CHF=".to_string())),
        instrument::Instrument::new(instrument::Kind::Currency("IDR=".to_string())),
        instrument::Instrument::new(instrument::Kind::Bond("46590XAR7=".to_string()))
    ];

    for i in dictionary.iter() {
        reuters.add(&i);
    }

    for ric in opt.subscribe.iter() {
        match reuters.subscribe(ric.to_string()) {
            Ok(&ref i) => {
                println!("subscribed {:?}", ric);
            }
            Err(e) => {
                println!("ERROR::{}", e);
            }
        }
    }


    reuters.start();

}


fn dummy<T: std::fmt::Debug>(d:T) {
    println!("dummy::{:?}", d);
}
