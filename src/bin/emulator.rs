use clap::Parser;
use yayachip8rsemu::state::Chip8State;

#[derive(Parser, Debug)]
#[command(author = "tabun dareka <tabun.dareka@protonmail.com>")]
#[command(about = "Chip8 emulator", long_about = None)]
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
    let a = Chip8State::new();

    println!("Hello, world!");
}
