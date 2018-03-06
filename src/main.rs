extern crate petgraph;
use petgraph::Graph;

extern crate chrono;
use chrono::prelude::*;

#[macro_use]
extern crate serde_derive;
extern crate serde;
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
    let mut graph = Graph::<Mission, Option<String>>::new();
    let mut mission = Mission::new("A Life Well Spent".to_string(), "The winds of a new beginning blow".to_string());
    mission.add_entry("Something happened on this date".to_string());
    graph.add_node(mission);
    println!("{}", serde_json::to_string_pretty(&graph).unwrap());
}
