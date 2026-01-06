use crate::checksum;
use crate::physical_layer::PhysicalLayer;
use std::time::{Duration, Instant};
use std::thread;

pub struct StopAndWait {
    physical: PhysicalLayer,
    timeout: Duration,
}

impl StopAndWait {
    pub fn new(physical: PhysicalLayer, timeout_ms: u64) -> Self {
        StopAndWait {
            physical,
            timeout: Duration::from_millis(timeout_ms),
        }
    }

    pub fn send(&self, data: &[u8]) {
        let mut seq: u8 = 0;

        for chunk in data.chunks(64) { // example chunk size
            let checksum = checksum::crc16(chunk);
            let mut frame = vec![seq];
            frame.extend_from_slice(chunk);
            frame.push((checksum >> 8) as u8);
            frame.push((checksum & 0xFF) as u8);

            loop {
                let start = Instant::now();
                self.physical.send(&frame).unwrap();

                // Simplified: assume ACK arrives immediately
                // Replace with real receive logic
                let ack = self.physical.receive().unwrap_or_default();
                if !ack.is_empty() && ack[0] == seq {
                    break;
                }

                if start.elapsed() > self.timeout {
                    println!("Timeout, resending frame {}", seq);
                }
            }

            seq ^= 1; // toggle sequence number 0 <-> 1
        }
    }
}

