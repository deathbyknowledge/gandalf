use std::fs::File;
use std::os::unix::prelude::FileExt;
use std::io::Cursor;
use std::io::{self, Read, prelude::*};

use roxmltree::Document;

use crate::page::Page;


/// Read a stream from a multi-stream dump file. Start at position `offset`
/// until `length` bytes have been read or EOF is reached.
pub fn read_stream_from_dump(file_path: &str, offset: u64, length: Option<&u64>) -> io::Result<Vec<u8>> {
    let file = File::open(file_path)?;
    let mut bytes: Vec<u8>;
    if let Some(len) = length {
        bytes = vec![0u8; (*len).try_into().unwrap()];
    } else {
        bytes = Vec::new();
    }
    file.read_exact_at(&mut bytes, offset)?;
    Ok(bytes)
}

/// Descompress a Vec<u8> as a bzip2 stream.
pub fn decompress_to_string(bytes: Vec<u8>) -> String {
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

pub struct DumpProcessor {
    dump_path: String,
    root_path: String,
}

impl DumpProcessor {
    pub fn new(dump_path: String ) -> Self {
        Self {dump_path, root_path: crate::DB_PATH.to_string()}
    }

    /// Iterate over the offsets.
    pub fn setup_db(&self, offsets: &[u64]) -> std::io::Result<()> {
        // The starting point for a stream is defined by its corresponding
        // offset value. Its lenght however, is defined by the next stream's offset
        // minus the current stream's offset.
        for (i, offset) in offsets.iter().enumerate() {
            // Get the next stream's offset
            let bytes = read_stream_from_dump(self.dump_path.as_str(), *offset, offsets.get(i +1)).unwrap();
            let mut contents = decompress_to_string(bytes);
            contents = "<document>".to_string() + contents.as_str() + "</document>";

            // XML parsing
            let doc = Document::parse(contents.as_str()).unwrap();
            for node in doc.descendants() {
                if let Ok(page) = node_to_page(node) {
                    let mut file = File::create(self.root_path.to_string() + page.get_id().to_string().as_str()).expect("Error db file");
                    let compressed = compress(&page.into_bytes_packed());
                    file.write_all(&compressed).unwrap();
                }
            }
            if i % 1000 == 0 {
                println!("Finished processing stream {i}");
            }
        }
        Ok(())
    }
} 

fn node_to_page(node: roxmltree::Node) -> Result<Page, String> {
    // XML parsing
    if node.is_element() && node.tag_name() == "page".into() {
        let mut page = Page::new();
        for child in node.children() {
            if child.tag_name() == "id".into() {
                page.set_id(child.text().unwrap().parse().unwrap());
            } else if child.tag_name() == "title".into() {
                page.set_title(child.text().unwrap());
            } else if child.tag_name() == "revision".into() {
                let txt = child.children().find(|ch| ch.tag_name() == "text".into()).unwrap();
                page.set_content(txt.text().unwrap());
            }
        }
        return Ok(page);
    }
    Err("this was no page".to_string())

}


fn compress(data: &[u8]) -> Vec<u8> {
    let mut e = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    e.write_all(data).expect("Failed to write data");
    e.finish().expect("Failed to finish encoding")
}