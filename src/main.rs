#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate stopwatch;
#[macro_use]
extern crate lazy_static;

use std::env;
use stopwatch::{Stopwatch};

mod configuration;
mod database;
mod metric;

fn main() {
    let args: Vec<String> = env::args().collect();

    let conf = configuration::Configuration::new(&args).expect("lol");

    println!("conf {:?}", conf);

    for source in conf.sources {
        let sw = Stopwatch::start_new();
        let mut connection = database::Connection::new(&source.url).expect("lol 2");

        connection.query(&source.query);

        println!("Connection to {} took {} secs ({:?})", source.name, sw.elapsed().as_secs(), sw.elapsed());
    }
}



