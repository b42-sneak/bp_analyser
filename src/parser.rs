use std::{fs::File, io::Read, path::PathBuf};

use anyhow::{anyhow, bail};
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::{Match, Regex};

lazy_static! {
    static ref BRA_REX: Regex =
        Regex::new(r"\{\{bra\|(?:(\d+)?\|)?(?:(\w+(?: metric)?)?)?(\|.*)*\}\}").unwrap();
}

pub fn parse(input: PathBuf, output: Option<PathBuf>) -> anyhow::Result<()> {
    let mut content = String::new();
    File::open(input)?.read_to_string(&mut content)?;

    let results: Vec<Bra> = content.par_lines().filter_map(parse_line).collect();

    if let Some(target) = output {
        todo!("Output writing is not yet implemented")
    } else {
        dbg!(results);
    }

    Ok(())
}

#[derive(Debug, Clone)]
pub struct Bra<'a> {
    pub whole_match: &'a str,
    pub band_size: Option<u32>,
    pub cup_size: CupSize,
    pub other_data: Option<&'a str>,
}

#[derive(Debug, Clone)]
pub enum CupSize {
    Single(char),
    Double(char),
    Triple(char),
    Metric(char),
}

impl TryFrom<&str> for CupSize {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let letter = value
            .chars()
            .next()
            .ok_or_else(|| anyhow!("Cannot parse CupSize"))?;

        let cup_size = match value.len() {
            1 => CupSize::Single(letter),
            2 => CupSize::Double(letter),
            3 => CupSize::Triple(letter),
            8 => CupSize::Metric(letter),
            _ => bail!("Invalid CupSize"),
        };

        Ok(cup_size)
    }
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

fn parse_line(text: &str) -> Option<Bra> {
    let captures = BRA_REX.captures(text)?;

    Some(Bra {
        // get(0) must be Some because of the successful capture above
        whole_match: captures.get(0).unwrap().as_str(),
        band_size: captures.get(1).and_then(|m| m.as_str().parse().ok()),
        cup_size: captures.get(2).and_then(|m| m.as_str().try_into().ok())?,
        other_data: captures.get(3).map(|m| m.as_str()),
    })
}
