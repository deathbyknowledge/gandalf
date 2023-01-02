use std::fs::File;
use std::io::{self, Read, prelude::*};
use std::io::Cursor;
use std::os::unix::prelude::FileExt;
use capnp::serialize_packed;
use roxmltree::Document;

pub mod schema {
  pub mod page_capnp {
    include!(concat!(env!("OUT_DIR"), "/page_capnp.rs"));
  }
}


fn main() {
    let bytes = read_bytes_from_file( "../new/enwiki-latest-pages-articles-multistream.xml.bz2", 602, 658112).unwrap();
    let mut contents = decompress_to_string(bytes);
    contents = "<document>".to_string() + contents.as_str() + "</document>";

    // XML parsing
    let doc = Document::parse(contents.as_str()).unwrap();
    let mut pages: Vec<TestPage> = Vec::new();
    for node in doc.descendants() {
        if let Ok(page) = node_to_page(node) {
            pages.push(page);
        }
    }
    let page = &pages[1];
    let xml = page.to_xml();
    let msg = page.to_message();
    let compressed = compress(&msg);
    println!("{} bytes in XML and {} bytes in Capn'Proto", xml.len(), compressed.len());
}

fn node_to_page(node: roxmltree::Node) -> Result<TestPage, String> {
    if node.is_element() {
        if node.tag_name() == "page".into() {
            let mut page = TestPage{id: 0, title: String::new(), content: String::new()};
            for child in node.children() {
                if child.tag_name() == "id".into() {
                    page.id = child.text().unwrap().parse().unwrap();
                } else if child.tag_name() == "title".into() {
                    page.title = child.text().unwrap().to_string();
                } else if child.tag_name() == "revision".into() {
                    let txt = child.children().find(|ch| ch.tag_name() == "text".into()).unwrap();
                    page.content = txt.text().unwrap().to_string();
                }
            }
            return Ok(page);
        }
    }
    Err("Node is not page".to_string())

}

fn read_bytes_from_file(file_path: &str, skip_bytes: u64, num_bytes: u64) -> io::Result<Vec<u8>> {
    let file = File::open(file_path)?;
    let mut bytes = vec![0u8; num_bytes.try_into().unwrap()];
    file.read_exact_at(&mut bytes, skip_bytes)?;
    Ok(bytes)
}

fn decompress_to_string(bytes: Vec<u8>) -> String {
    // A Cursor allows us to create a BufReader from a Vec<u8>
    let cursor = Cursor::new(bytes);
    let reader = std::io::BufReader::new(cursor);
    let mut decompressor = bzip2::read::BzDecoder::new(reader);
    let mut contents = String::new();
    decompressor.read_to_string(&mut contents).unwrap();
    
    // store the uncompressed contents in a file
    if cfg!(debug_assertions) {
        let mut file = File::create("/tmp/wiki-stream.xml").unwrap();
        file.write_all(contents.as_bytes()).unwrap();
    }
    contents
}

fn compress(data: &[u8]) -> Vec<u8> {
    let mut e = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    e.write_all(data).expect("Failed to write data");
    e.finish().expect("Failed to finish encoding")
}

#[derive(Debug)]
struct TestPage {
    id: u32,
    title: String,
    content: String,
}

impl TestPage {

    fn to_xml(&self) -> String {
        format!("<page>\n<id>{}</id>\n<title>{}</title>\n<content>{}</content>",
                self.id, self.title, self.content)
    }

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
