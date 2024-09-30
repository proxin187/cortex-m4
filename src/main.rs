mod processor;
mod memory;
mod bus;
mod tui;

use processor::Processor;
use tui::Tui;

use clap::{Parser, Subcommand};

use std::fs;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,

    #[arg(long, short, action)]
    debug: bool,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// a interactive emulator interface
    Interactive {
        path: String
    },

    /// a minimal emulator interface with no tui
    Minimal {
        path: String
    },
}

// TODO: project idea: write a sandbox without emulating, overwrite syscalls

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.command {
        Command::Interactive { path } => {
            let rom = fs::read(path)?;

            let mut tui = Tui::new()?;

            tui.flash(&rom)?;

            tui.run()?;
        },
        Command::Minimal { path } => {
            let rom = fs::read(path)?;

            let mut processor = Processor::new();

            processor.flash(&rom)?;

            processor.reset();

            for _ in 0..12 {
                processor.step();
            }

            // TODO: finish the minimal interface
        },
    }

    Ok(())
}


