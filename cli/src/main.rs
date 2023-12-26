use std::error::Error;
use material3_optimzer::Material3BuilderExtractor;

fn main() -> Result<(),Box<dyn Error>> {
    let command = Cli::parse();

    let extractor = Material3BuilderExtractor::try_from_file(command.input_dir)?;
    extractor.extract_and_write_to_file(&command.output_dir)?;

    Ok(())
}

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short,help = "Specify the directory containing the source zip file to be included in the optimization process.")]
    input_dir : String,
    #[arg(short,help = "Specify the destination directory where the optimized .kt theme file will be saved. The optimized file will be generated based on the compressed zip file.")]
    output_dir : String
}