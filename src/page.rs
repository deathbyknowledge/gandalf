use capnp::{message::TypedBuilder, serialize_packed};

use crate::schema::page_capnp::page;

pub struct Page(TypedBuilder<page::Owned>);

impl Page {
    pub fn new() -> Self {
        let mut message = capnp::message::TypedBuilder::<page::Owned>::new_default();
        let mut page = message.init_root();
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
        &self.0.get_root_as_reader().unwrap().get_title().unwrap()
    }
    #[inline]
    pub fn set_title(&mut self, value: &str) {
        self.0.get_root().unwrap().set_title(value);
    }
    #[inline]
    pub fn get_content(&self) -> &str {
        &self.0.get_root_as_reader().unwrap().get_content().unwrap()
    }
    #[inline]
    pub fn set_content(&mut self, value: &str) {
        self.0.get_root().unwrap().set_content(value);
    }

    pub fn to_bytes_packed(self) -> Vec<u8>  {
        let mut buf = Vec::new();
        serialize_packed::write_message(&mut buf, &self.0.into_inner()).expect("Failed to serialize message");
        buf
    }
}