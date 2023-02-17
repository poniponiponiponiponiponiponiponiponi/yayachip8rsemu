use clap::Parser;
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

fn main() {
    let args = Args::parse();

    println!("disassembler");
}
