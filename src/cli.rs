use std::path::PathBuf;

use structopt::StructOpt;

const AUTHOR_NAME: &'static str = "b42-sneak <GitHub @b42-sneak>";

#[derive(Debug, StructOpt)]
#[structopt(name ="BP analyser", about = "Analyses data from a wiki site", author = AUTHOR_NAME)]
pub struct Options {
    // /// Activate debug mode
    // // short and long flags (-d, --debug) will be deduced from the field's name
    // #[structopt(short, long)]
    // pub debug: bool,

    // /// Set speed
    // // we don't want to name it "speed", need to look smart
    // #[structopt(short = "v", long = "velocity", default_value = "42")]
    // pub speed: f64,

    // /// Input file
    // #[structopt(parse(from_os_str))]
    // pub input: PathBuf,

    // /// Where to write the output: to `stdout` or `file`
    // #[structopt(short, long)]
    // pub out_type: String,

    // /// File name: only required when `out-type` is set to `file`
    // #[structopt(name = "FILE", required_if("out-type", "file"))]
    // pub file_name: Option<String>,

    // The subcommands
    #[structopt(subcommand)]
    pub operation: Operation,
}

impl Options {
    pub fn get_matches() -> Self {
        structopt::StructOpt::from_clap(
            &Options::clap()
                .setting(structopt::clap::AppSettings::DisableHelpSubcommand)
                .get_matches(),
        )
    }
}

#[derive(StructOpt, Debug)]
#[structopt(about = "the stupid content tracker")]
pub enum Operation {
    /// Parse and analyse a wiki dump for information
    Parse {
        /// The file to be analysed
        input: PathBuf,

        /// Output file, stdout if not present
        #[structopt(parse(from_os_str))]
        output: Option<PathBuf>,
    },
    /// Download data from the wiki server and analyse that (TODO)
    Crawl {},
}
