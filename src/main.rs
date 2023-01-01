use std::fs::File;
use std::io::{self, Read, Seek};
use std::io::{Cursor};
use std::os::unix::prelude::FileExt;
fn main() {
    let tmp = read_bytes_from_file("../new/enwiki-latest-pages-articles-multistream.xml.bz2", 602, 658112);
    let bytes = match tmp {
        Ok(b )=>  b,
        Err(err) => panic!("{}", err),
        
    };
    println!("{:x}..{:x}", bytes[0], bytes[bytes.len() - 1]);

    // A Cursor allows us to create a BufReader from a Vec<u8>
    let cursor = Cursor::new(bytes);
    let reader = std::io::BufReader::new(cursor);
    let mut decompressor = bzip2::read::BzDecoder::new(reader);

    let mut contents = String::new();
    match decompressor.read_to_string(&mut contents) {
        Ok(_) => println!("Went alright"),
        Err(err) => println!("{}", err),
        
    };
    print!("{}", contents);
}

fn read_bytes_from_file(file_path: &str, skip_bytes: u64, num_bytes: u64) -> io::Result<Vec<u8>> {
    let file = File::open(file_path)?;
    let mut bytes = vec![0u8; num_bytes.try_into().unwrap()];
    file.read_exact_at(&mut bytes, skip_bytes)?;
    Ok(bytes)
}