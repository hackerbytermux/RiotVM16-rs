pub mod vm;

use vm::get_vm;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    filename: String,

    #[arg(short, long)]
    debug: bool,

    #[arg(short, long)]
    dump: bool,
}

// +2
fn main() {
    let args = Args::parse();

    let program = std::fs::read(args.filename).expect("Unable to read file");
    let mut  cpu = get_vm(&program);
    cpu.run();

    if args.debug {
        //write memory to file
        println!("{:?}", cpu.registers);
        println!("{:?}", cpu.stack);
    }

    if args.dump {
        std::fs::write("memory.bin", cpu.memory.memory).expect("Unable to write file");
    }
}   