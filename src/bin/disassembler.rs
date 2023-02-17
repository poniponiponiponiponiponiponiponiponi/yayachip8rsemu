use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use yayachip8rsemu::disasm;

#[derive(Parser, Debug)]
#[command(author = "tabun dareka <tabun.dareka@protonmail.com>")]
#[command(about = "Chip8 disassembler", long_about = None)]
#[command(version)]
struct Args {
   /// Print verbose information
   #[arg(short, long, action, default_value_t = false)]
   verbose: bool,

   /// File to run
   #[arg(short, long)]
   file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("sstart");
    let args = Args::parse();

    let mut file = File::open(args.file)?;
    let mut contents = Vec::<u8>::new();
    file.read_to_end(&mut contents)?;

    for i in (0..contents.len()).step_by(2) {
        if i+1 == contents.len() {
            break;
        }
        let byte1 = contents[i];
        let byte2 = contents[i+1];
        let word = ((byte1 as u16) << 8) | byte2 as u16;
        let instruction = disasm::Instruction::from(word);
        println!("{:04x}:\t{}", word, instruction);
    }

    Ok(())
}
