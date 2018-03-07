use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::env;

extern crate petgraph;
use petgraph::Graph;

extern crate chrono;
use chrono::prelude::*;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Mission {
    pub title: String,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub entries: Vec<MissionEntry>,
    pub completion: Option<MissionEntry>,
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
pub struct MissionEntry {
    pub timestamp: DateTime<Utc>,
    pub entry_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    pub graph: Graph<Mission, Option<String>>,
}

impl Log {

    pub fn mission_list(&mut self) -> Vec<&mut Mission> {
        self.graph.node_weights_mut().collect()
    }
}

pub fn open_log(file: &File) -> Log {
    let log = match serde_json::from_reader(file) {
        Ok(log) => log,
        Err(_) => create_log(),
    };
    log
}

pub fn open_file(path: &PathBuf) -> File {
    match File::open(path) {
        Err(_) => match File::create(&path) {
            Ok(file) => file,
            Err(why) => panic!("failed to create or open {}: {}", path.display(), why.description()),
        }
        Ok(file) => file,
    }
}

pub fn default_path() -> PathBuf {
    let mut path = match env::home_dir() {
        Some(path) => path,
        None => panic!("Failed to find home dir!"),
    };
    path.push(".alws.json");
    path
}

pub fn write_to_file(path: &PathBuf, log: &Log) {
    let file = File::create(path).unwrap();
    serde_json::to_writer_pretty(&file, log).unwrap();
}

fn create_log() -> Log {
    let mut graph = Graph::<Mission, Option<String>>::new();
    let mut mission = Mission::new("A Life Well Spent".to_string(), "The winds of a new beginning blow".to_string());
    mission.add_entry("Something happened on this date".to_string());
    let mut mission2 = Mission::new("A Test of Courage".to_string(), "Many Bothans died to bring us this information".to_string());
    graph.add_node(mission);
    graph.add_node(mission2);
    Log { graph }
}
