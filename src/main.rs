mod processor;
mod memory;
mod bus;
mod loader;
mod tui;

use processor::Processor;
use loader::{Hex, Kind};

use tui::Tui;

use std::fs;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
    let mut processor = Processor::new();

    let raw = fs::read("examples/minimal/main.hex")?;

    let mut hex = Hex::new(&raw)?;

    while let Ok(record) = hex.next() {
        println!("record: {:#x?}", record);

        match record.kind {
            Kind::Data => {
                processor.flash(record.addr, &record.data);
            },
            Kind::ExtendSegmentAddress => {},
            Kind::StartSegmentAddress => {
            },
            Kind::ExtendLinearAddress => {},
            Kind::StartLinearAddress => {},
            Kind::Eof => {},
        }
    }

    processor.step();
    */

    let mut tui = Tui::new()?;

    tui.run()?;

    Ok(())
}


