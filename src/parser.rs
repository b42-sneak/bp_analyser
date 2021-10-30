use std::{fs::File, io::Read, path::PathBuf};

use anyhow::{anyhow, bail};
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;

lazy_static! {
    static ref BRA_REX: Regex =
        Regex::new(r"\{\{bra\|(?:(\d+)?\|)?(?:(\w+(?: metric)?)?)?(\|.*)*\}\}").unwrap();
}

pub fn parse(input: PathBuf, output: Option<PathBuf>) -> anyhow::Result<()> {
    let mut content = String::new();
    File::open(input)?.read_to_string(&mut content)?;

    let results: Vec<anyhow::Result<Bra>> =
        content.par_lines().map(|text| parse_line(text)).collect();

    Ok(())
}

pub struct Bra {
    pub whole_match: String,
    pub band_size: Option<u32>,
    pub cup_size: CupSize,
    pub other_data: Option<String>,
}

pub enum CupSize {
    Single(char),
    Double(char),
    Triple(char),
    Metric(char),
}

impl CupSize {
    pub fn get(&self) -> char {
        match self {
            CupSize::Single(size)
            | CupSize::Double(size)
            | CupSize::Triple(size)
            | CupSize::Metric(size) => size.to_owned(),
        }
    }
}

fn parse_line(text: &str) -> anyhow::Result<Bra> {
    dbg!(BRA_REX.captures(text));
    bail!("Kaboom")
}
