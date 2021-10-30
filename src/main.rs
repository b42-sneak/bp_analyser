use crate::cli::Options;

mod cli;

fn main() {
    let opt = Options::get_matches();

    match opt.operation {
        cli::Operation::Parse { input, output } => todo!(),
        cli::Operation::Crawl {} => todo!("Crawling capabilities still need to be implemented"),
    }
}
