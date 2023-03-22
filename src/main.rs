use std::{path::Path, env};

use lib::{Args, find_in_dir};
use structopt::StructOpt;

fn main() {
    let args = Args::from_args();

    if env::args().len() == 1 {
        Args::clap().print_help().unwrap();
        return;
    }    

    let result = match args.filetype {
        Some(file_type) => {
            find_in_dir(Path::new(&args.path.unwrap()), &args.word.unwrap(), &Some(file_type))
        },
        None => {
            find_in_dir(Path::new(&args.path.unwrap()), &args.word.unwrap(), &None)
        },
    };

    if let Some(results) = result {
        for i in &results {
            for j in i {
                println!("{:?}", j);
            }
        }
    }
}