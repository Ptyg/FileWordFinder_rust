use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(short, long, requires="word")]
    path: String,

    #[structopt(short, long, requires="path")]
    word: String,

    #[structopt(short="t", long)]
    filetype: Option<String>
}

fn main() {
    
}