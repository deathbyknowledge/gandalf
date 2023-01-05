mod schema;
mod dump;
mod page;
mod index;

const DB_PATH: &str = "/opt/gandalf/";
const HELP_TEXT: &str = "Usage: `gandalf [subcommand] [..OPTIONS]`
Valid subcommands are 'init'";

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
    let page = crate::page::Page::from_compressed_file(3821);
    println!("ID {}:\n  Title is {}\n  Content is {}", page.get_id(), page.get_title(), page.get_content());
}

fn start() {
    let mut index = index::Index::new();
    index.init_from("../new/enwiki-latest-pages-articles-multistream-index.txt".to_string()).expect("error init index");
}