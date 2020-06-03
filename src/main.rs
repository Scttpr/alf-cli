use structopt::StructOpt;
use std::fs::File;

const OUTPUT_PATH: &str = "../src/commands"; // @todo edit this path to load env variable

#[derive(StructOpt)]
#[structopt(name = "discord_commandR", about = "A CLI tool to easily create Discord command templates")]
struct Cli {
    #[structopt(short = "n", long = "name")]
    name: String,
}

fn main() {
    let args = Cli::from_args();
    let name = args.name;
    let file_name: String = generate_file_name(name);
    
    
}

fn generate_file_name(name: String) -> String {
    name
    .split("-")
    .map(|slice| {
        let (first_letter, rest) = slice.split_at(1);
        first_letter.to_uppercase() + rest
    })
    .collect() 
}

fn create_file(file_name: String) {
    let mut file = File::create(&file_name);
}
