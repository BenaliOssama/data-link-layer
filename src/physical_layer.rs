use std::io::{self, Write};

pub struct PhysicalLayer;

impl PhysicalLayer {
    pub fn new() -> Self {
        PhysicalLayer
    }

    pub fn send(&self, frame: &[u8]) -> io::Result<()> {
        // Replace this with real emulator interface later
        println!("Sending frame: {:?}", frame);
        Ok(())
    }

    pub fn receive(&self) -> io::Result<Vec<u8>> {
        // Replace with emulator interface
        // For now, just simulate an empty frame
        Ok(vec![])
    }
}

