extern crate wallpaper;
extern crate chrono;
extern crate structopt;

use structopt::StructOpt;
use chrono::prelude::*;

fn parse_time(time: &str) -> NaiveTime {
    NaiveTime::parse_from_str(time, "%H:%M").unwrap()
}

#[derive(Debug, StructOpt)]
#[structopt(name = "wallpaper", about = "An example of StructOpt usage.")]
enum Opts {
    #[structopt(name = "add")]
    Add {
        #[structopt(name = "path")]
        path: String,
        #[structopt(name = "time", parse(from_str = "parse_time"))]
        time: NaiveTime
    }
}

fn main() { 
    let opt = Opts::from_args();
    println!("{:?}", opt);
}