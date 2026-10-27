use clap::Parser;


/// The args that the program will take for the mmap to be able to iterate over the items
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args{

    /// The file path which will open and iterate over
    #[arg(short, long)]
    file_path: String,

    /// The offset which will start to iterate
    #[arg(short, long, default_value_t = 0)]
    offset: u64,

    /// The length of the bytes to iterate
    #[arg(short, long, default_value_t = 64)]
    length: u64,

    /// The counter to repeat
    #[arg(short, long, default_value_t = 1)]
    counter: u64,
}
