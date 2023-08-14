use clap::{command,Arg,Command};

pub fn construct_parser() -> Command {
    let input_dir = Arg::new("output_directory")
        .short('i')
        .long("indir")
        .value_name("DIRECTORY")
        .help("Specify the directory containing the source zip file to be included in the optimization process.");

    let output_dir = Arg::new("input_directory")
        .short('o')
        .long("outdir")
        .value_name("DIRECTORY")
        .help("Specify the destination directory where the optimized .kt theme file will be saved. The optimized file will be generated based on the compressed zip file.");
        
    command!()
        .arg(input_dir)
        .arg(output_dir)
}