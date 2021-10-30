use crate::{cli::Options, parser::parse};

mod cli;
mod parser;

fn main() -> anyhow::Result<()> {
    let opt = Options::get_matches();

    match opt.operation {
        cli::Operation::Crawl {} => todo!("Crawling capabilities still need to be implemented"),
        cli::Operation::Parse { input, output } => parse(input, output)?,
    }

    Ok(())
}
