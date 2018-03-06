extern crate petgraph;
use petgraph::Graph;

extern crate chrono;
use chrono::prelude::*;

#[macro_use]
extern crate serde_derive;
extern crate serde;

#[derive(Serialize, Deserialize, Debug)]
struct Mission {
    title: String,
    description: String,
    timestamp: DateTime<Utc>,
    entries: Vec<MissionEntry>,
    completion: Option<MissionEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MissionEntry {
    timestamp: DateTime<Utc>,
    entrytext: String,
}

fn main() {
    let mut graph = Graph::<Mission, Option<String>>::new();
    println!("{:?}", graph);
}
