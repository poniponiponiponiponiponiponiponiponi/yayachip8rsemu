use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use yayachip8rsemu::disasm;

#[derive(Parser, Debug)]
#[command(author = "poni <poniponiponiponiponiponiponiponiponiponi@protonmail.com>")]
#[command(about = "yayachip8rsemu disassembler", long_about = None)]
#[command(version)]
struct Args {
   /// Print verbose information
   #[arg(short, long, action, default_value_t = false)]
   verbose: bool,

   /// File to disassemble
   #[arg(short, long)]
   file: String,

   /// File offset to start at
   #[arg(short, long, action, default_value_t = 0)]
   start: usize,

   /// Amount of instruction to print. Zero means to the end
   #[arg(short, long, action, default_value_t = 0)]
   instruction_amount: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    // https://github.com/rust-lang/rust/issues/46016
    #[cfg(unix)]
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
    
    let args = Args::parse();
    let mut file = File::open(args.file)?;
    let mut contents = Vec::<u8>::new();
    file.read_to_end(&mut contents)?;

    let mut instructions_printed = 0;
    for i in (args.start..contents.len()).step_by(2) {
        instructions_printed += 1;
        if instructions_printed == args.instruction_amount {
            break;
        }
        // Handle edge case when the file size is not even
        if i+1 == contents.len() {
            break;
        }
        let byte1 = contents[i];
        let byte2 = contents[i+1];
        let word = ((byte1 as u16) << 8) | byte2 as u16;
        let instruction = disasm::Instruction::from(word);
        println!("{:04x}:\t{:04x} {}", i, word, instruction);
    }

    Ok(())
}
