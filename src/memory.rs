//! Memory structures
use crate::nal::TruthValue;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct MemoryItem {
    id: u64,
    timestamp: u64,
    pub term: String,
    pub tv: TruthValue,
    usage_count: u64,
    embed_id: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct Memory {
    items: Vec<MemoryItem>,
    last_id: u64,
    current_timestamp: u64,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            last_id: 0,
            current_timestamp: 0,
        }
    }
    pub fn add(&mut self, term: &str, tv: TruthValue, embed_id: Option<u64>) -> () {
        let id = self.last_id;
        self.items.push(MemoryItem {
            id,
            timestamp: self.current_timestamp,
            term: term.to_string(),
            tv,
            usage_count: 0,
            embed_id,
        });
        self.last_id += 1;
        self.current_timestamp += 1;
    }
}

pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Memory> {
    let file: File = File::open(&path)?;
    let memory: Memory = serde_json::from_reader(file)?;

    Ok(memory)
}

pub fn store<P: AsRef<Path>>(path: P, memory: &Memory) -> io::Result<()> {
    let file: File = File::create(&path)?;
    serde_json::to_writer(file, &memory)?;

    Ok(())
}
