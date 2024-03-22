use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;
use std::error::Error;
#[path = "instrument.rs"] mod instrument;
#[path = "alphavantageapi.rs"] mod alphavantageapi;


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

#[tokio::main]
async fn main() {
    match Opt::from_args_safe() {
        Ok(opt) => {
            println!("opt = {:#?}", opt);
            do_it(&opt).await;
        }
        Err(e) => {
            println!("problem: {}", e);
        }
    }
}

async fn do_it(opt : &Opt) {  
    let mut reuters = instrument::DataFeed::new(opt.feed.to_string());

    let dictionary = vec![
        instrument::Instrument::new(instrument::Kind::Equity("AAPL".to_string())),
        instrument::Instrument::new(instrument::Kind::Equity("MSFT".to_string())),
        instrument::Instrument::new(instrument::Kind::Currency("EUR=".to_string())),
        instrument::Instrument::new(instrument::Kind::Currency("CHF=".to_string())),
        instrument::Instrument::new(instrument::Kind::Currency("IDR=".to_string())),
        instrument::Instrument::new(instrument::Kind::Bond("46590XAR7=".to_string()))
    ];

    for i in dictionary.iter() {
        reuters.add(&i);
    }

    let api_key = "votre_clé_api"; // Remplacez par votre clé API Marketstack.
    let api = alphavantageapi::AlphaVantageApi::new(api_key.to_string());
    for ric in opt.subscribe.iter() {

        match api.get(ric).await {

            Ok(financial_data) => {
                println!("{:?}", financial_data);

                match reuters.subscribe(ric.to_string()) {
                    Ok(&ref _i) => {
                        println!("subscribed {:?}", ric);
                    }
                    Err(e) => {
                        println!("ERROR::{}", e);
                    }
                }
            }
            Err(e) => {
                println!("AlphaVantageApi ERROR::{}", e);
            }
        }
    }


    reuters.start();
}


fn dummy<T: std::fmt::Debug>(d:T) {
    println!("dummy::{:?}", d);
}
