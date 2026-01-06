mod checksum;
mod physical_layer;
mod protocols;

use physical_layer::PhysicalLayer;
use protocols::stop_and_wait::StopAndWait;

fn main() {
    let physical = PhysicalLayer::new();
    let sender = StopAndWait::new(physical, 1000);

    let data = b"Hello, this is a test message for Stop-and-Wait protocol!";
    sender.send(data);
}

