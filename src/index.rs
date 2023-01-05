use std::{collections::HashMap, fs::File};
use std::io::{BufRead, BufReader};

pub struct Index {
    raw_index: HashMap<String, u32>,
}

impl Index {
    pub fn new() -> Self {
        Index { raw_index: HashMap::new() }
    }

    /// Initialize the HashMap index with all values found in a
    /// dump index file. Keep only the title and ID.
    pub fn init_from(&mut self, index_path: String) -> std::io::Result<()> {
        let index_file = File::open(index_path).unwrap(); 
        let reader = BufReader::new(index_file);

        println!("Generating index from file...");
        for line in reader.lines() {
            let l = line.expect("error reading line");
            let values: Vec<&str> = l.split(':').collect();
            let page_id: u32 = values[1].parse().unwrap();
            let page_title = values[2].to_string();
            self.raw_index.insert(page_title, page_id);
        }
        println!("Index len is {}", self.raw_index.len()); 
        Ok(())
    }

    /// Initialize the HashMap index with all values found in a
    /// dump index file. Keep only the title and ID. Return a
    /// vector with the offset for each stream.
    pub fn init_from_and_count(&mut self, index_path: String) -> std::io::Result<Vec<u64>> {
        let index_file = File::open(index_path).unwrap();
        let reader = BufReader::new(index_file);
        let mut offsets: Vec<u64> = vec![];

        println!("Generating index from file...");
        for line in reader.lines() {
            let l = line.expect("error reading line");
            let values: Vec<&str> = l.split(':').collect();
            // Values are encoded as OFFSET:PAGE_ID:PAGE_TITLE
            let offset: u64 = values[0].parse().unwrap();
            let page_id: u32 = values[1].parse().unwrap();
            let page_title = values[2].to_string();

            // Insert the new entry in the index
            self.raw_index.insert(page_title, page_id);

            match offsets.last() {
                Some(current) => if offset != *current {offsets.push(offset)},
                None => offsets.push(offset),
            }
        }
        println!("Offsets len is {}", offsets.len()); 
        Ok(offsets)
    }
}