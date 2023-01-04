mod schema;
mod dump;
mod page;
mod index;

const DB_PATH: &str = "/opt/gandalf/";
const HELP_TEXT: &str = "Usage: `gandalf [subcommand] [..OPTIONS]`
Valid subcommands are 'init'";

use std::fs::File;
use std::io::{Read, prelude::*};
use std::io::Cursor;
use capnp::serialize_packed;
use roxmltree::Document;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Some(command) = args.get(1) {
      match command.as_str() {
        "init" => init(),
        "start" => start(),
        "get" => get(),
        "help" => println!("{HELP_TEXT}"),
        _ => println!("See usage with `gandalf help`"),
      }
    } else {
      println!("See usage with `gandalf help`")
    }
}

fn init() {
    let mut index = index::Index::new();
    let offsets = index.init_from_and_count("../new/enwiki-latest-pages-articles-multistream-index.txt".to_string()).expect("error init index");
    let dp = dump::DumpProcessor::new("../new/enwiki-latest-pages-articles-multistream.xml.bz2".to_string());
    dp.setup_db(&offsets).expect("error setup db");
}

fn get() {
    read_by_id(3821);
}

fn start() {
    let mut index = index::Index::new();
    index.init_from("../new/enwiki-latest-pages-articles-multistream-index.txt".to_string()).expect("error init index");
}

fn read_by_id(id: u32) {
  use schema::page_capnp::page;
  let mut file = File::open(DB_PATH.to_string() + &id.to_string()).expect("error opening file"); 
  let mut buffer = Vec::new();
  file.read_to_end(&mut buffer).expect("error reading file");
  let mut d = flate2::read::ZlibDecoder::new(&buffer[..]);
  let mut decompressed = Vec::new();
  d.read_to_end(&mut decompressed).expect("failed to decompress");
  let cursor = Cursor::new(decompressed);
  let reader = std::io::BufReader::new(cursor);
  let message_reader = serialize_packed::read_message(
      reader,
      ::capnp::message::ReaderOptions::new(),
  ).unwrap();
  if let Ok(page) = message_reader.get_root::<page::Reader>() {
    println!("ID {}:\n  Title is {}\n  Content is {}", page.get_id(), page.get_title().unwrap(), page.get_content().unwrap());
  }
}




#[derive(Debug)]
struct TestPage {
    id: u32,
    title: String,
    content: String,
}

impl TestPage {

    fn to_message(&self) -> Vec<u8> {
        use schema::page_capnp::page;

        let mut message = capnp::message::Builder::new_default();
        let mut page = message.init_root::<page::Builder>();
        page.set_id(self.id);
        page.set_title(self.title.as_str());
        page.set_content(self.content.as_str());
        let mut buf = Vec::new();
        serialize_packed::write_message(&mut buf, &message).expect("Failed to serialize message");
        buf
    }
}
