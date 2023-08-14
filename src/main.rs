mod cli;
mod optimize;

fn main() { 
    let parser = cli::construct_parser();

    let matches = parser.get_matches();

    let input = matches.get_one::<String>("input_directory");
    let output = matches.get_one::<String>("output_directory");

    match (input,output) {
        (Some(i),Some(o)) => {
            match optimize::OptimzeResult::try_from(i.as_str()){
                Err(error) => {
                    println!("{error}");
                    return
                },
                Ok(value) => value.write_result_to_file(o)
            }
            println!("Optimization Successful");
        },
        _ => println!("Error: Both input and output directories are required."),
    }
}