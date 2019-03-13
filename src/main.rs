extern crate wallpaper;
extern crate chrono;
extern crate structopt;
extern crate serde_json;
extern crate serde;

use structopt::StructOpt;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

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
    },
    #[structopt(name = "run")]
    Run
}

#[derive(Serialize, Deserialize)]
struct Wallpaper {
    path: String,
    #[serde(with = "time_format")]
    time: NaiveTime
}

#[derive(Serialize, Deserialize)]
struct Config {
    wallpapers: Vec<Wallpaper>
}

mod time_format {
    use chrono::{NaiveTime};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%H:%M";

    pub fn serialize<S>(
        date: &NaiveTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<NaiveTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let time = String::deserialize(deserializer)?;
        let time = &*time;
        NaiveTime::parse_from_str(time, "%H:%M").map_err(serde::de::Error::custom)
    }
}

fn main() { 
    let opt = Opts::from_args();

    let srcdir = PathBuf::from("./src/config.json");
    println!("{:?}", fs::canonicalize(&srcdir));
    let config = fs::read_to_string(srcdir)
        .expect("Something went wrong reading the file");

    let config: &str = &*config;

    let config: Config = serde_json::from_str(config).unwrap();

    match opt {
        Opts::Add { path, time } => {
            
        },
        Opts::Run => {
            let mut wallpapers = config.wallpapers;
            wallpapers.sort_by(|a, b| b.time.cmp(&a.time));
            let current_time = Local::now().naive_local().time();
            let wallpaper_to_change = wallpapers.iter().find(|x| x.time <= current_time).unwrap();
            let path: &str = &*wallpaper_to_change.path;
            match wallpaper::set_from_path(path) {
                Ok(_) => println!("nice"),
                Err(_) => println!("error")
            };
        }
    }
}