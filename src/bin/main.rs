use clap::Parser;
use zone_builder::BlogProcessor;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = ".")]
    path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let processor = BlogProcessor::new(args.path);
    processor.process_directory()?;

    println!("Blog directory processing completed.");
    Ok(())
}
