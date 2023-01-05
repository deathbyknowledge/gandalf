use std::{fs::File, io::Read};

use capnp::{message::TypedBuilder, serialize_packed};

use crate::{schema::page_capnp::page, DB_PATH};

pub struct Page(TypedBuilder<page::Owned>);

impl Page {
    pub fn new() -> Self {
        let mut message = capnp::message::TypedBuilder::<page::Owned>::new_default();
        message.init_root();
        Self(message)
    }
    #[inline]
    pub fn get_id(&self) -> u32 {
        self.0.get_root_as_reader().unwrap().get_id()
    }
    #[inline]
    pub fn set_id(&mut self, value: u32) {
        self.0.get_root().unwrap().set_id(value);
    }
    #[inline]
    pub fn get_title(&self) -> &str {
        self.0.get_root_as_reader().unwrap().get_title().unwrap()
    }
    #[inline]
    pub fn set_title(&mut self, value: &str) {
        self.0.get_root().unwrap().set_title(value);
    }
    #[inline]
    pub fn get_content(&self) -> &str {
        self.0.get_root_as_reader().unwrap().get_content().unwrap()
    }
    #[inline]
    pub fn set_content(&mut self, value: &str) {
        self.0.get_root().unwrap().set_content(value);
    }

    pub fn into_bytes_packed(self) -> Vec<u8>  {
        let mut buf = Vec::new();
        serialize_packed::write_message(&mut buf, &self.0.into_inner()).expect("Failed to serialize message");
        buf
    }

    pub fn from_compressed_file(id: u32) -> Self {
        let mut file = File::open(DB_PATH.to_string() + &id.to_string()).expect("error opening file"); 
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("error reading file");
        let mut d = flate2::read::ZlibDecoder::new(&buffer[..]);
        let mut decompressed = Vec::new();
        d.read_to_end(&mut decompressed).expect("failed to decompress");
        let reader = std::io::BufReader::new(&decompressed[..]);
        let message_reader = serialize_packed::read_message(
            reader,
            ::capnp::message::ReaderOptions::new(),
        ).expect("couldnt read message");

        let reader = message_reader.into_typed::<page::Owned>();
        let mut builder = capnp::message::TypedBuilder::new_default();
        builder.set_root(reader.get().unwrap()).expect("error on casting");
        Page(builder)
    }

}