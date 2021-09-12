use std::{fs::File, io, path::Path};

fn main() {
    println!("Hello, world!");
}

pub struct DiskManager {
    heap_file: File,
    next_page: u64,
}
impl DiskManager {
    pub fn new(data_file: File) -> io::Result<Self> {
        unimplemented!()
    }
    pub fn open(data_file_path: impl AsRef<Path>) -> io::Result<Self> {
        unimplemented!()
    }
    pub fn allocate_page(&mut self) -> PageId {
        unimplemented!()
    }
    pub fn read_page_data(&mut self, page_id: PageId, data: &mut [u8]) -> io::Result<()> {
        unimplemented!()
    }
    pub fn write_page_data(&mut self, page_id: PageId, data: &mut [u8]) -> io::Result<()> {
        unimplemented!()
    }
}
