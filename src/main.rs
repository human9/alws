use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::env;

extern crate petgraph;
use petgraph::Graph;

extern crate chrono;
use chrono::prelude::*;

#[macro_use]
extern crate serde_derive;
extern crate serde;
use serde::de::DeserializeOwned;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Mission {
    title: String,
    description: String,
    timestamp: DateTime<Utc>,
    entries: Vec<MissionEntry>,
    completion: Option<MissionEntry>,
}

impl Mission {
    pub fn new(title: String, description: String) -> Self {
        Mission {
            title,
            description,
            timestamp: Utc::now(),
            entries: Vec::new(),
            completion: None,
        }
    }

    pub fn add_entry(&mut self, entry_text: String) {
        let entry = MissionEntry {
            timestamp: Utc::now(),
            entry_text,
        };
        self.entries.push(entry); 
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct MissionEntry {
    timestamp: DateTime<Utc>,
    entry_text: String,
}

fn main() {

    let path = default_path();
    let file = open_file(&path);
    let log = open_log(&file);

    write_to_file(&path, &log);
}

fn open_log(file: &File) -> Graph<Mission, Option<String>> {
    let graph = match serde_json::from_reader(file) {
        Ok(graph) => graph,
        Err(_) => create_log(),
    };
    graph
}

fn open_file(path: &PathBuf) -> File {
    match File::open(path) {
        Err(_) => match File::create(&path) {
            Ok(file) => file,
            Err(why) => panic!("failed to create or open {}: {}", path.display(), why.description()),
        }
        Ok(file) => file,
    }
}

fn default_path() -> PathBuf {
    let mut path = match env::home_dir() {
        Some(path) => path,
        None => panic!("Failed to find home dir!"),
    };
    path.push(".alws.json");
    path
}

fn write_to_file(path: &PathBuf, log: &Graph<Mission, Option<String>>) {
    let file = File::create(path).unwrap();
    serde_json::to_writer_pretty(&file, log).unwrap();
}

fn create_log() -> Graph<Mission, Option<String>> {
    let mut graph = Graph::<Mission, Option<String>>::new();
    let mut mission = Mission::new("A Life Well Spent".to_string(), "The winds of a new beginning blow".to_string());
    mission.add_entry("Something happened on this date".to_string());
    graph.add_node(mission);
    graph
}
