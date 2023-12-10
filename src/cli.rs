use crate::reflector::ReflectorKind;
use crate::rotor::RotorKind;
use crate::utils::try_to_alphabet_index;
use clap::Parser;
use std::str::FromStr;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(long, value_enum)]
    pub reflector: ReflectorKind,
    #[arg(long, required = true, value_name = "ROTOR", value_enum, num_args = 3)]
    pub rotors: Vec<RotorKind>,
    #[arg(long, required = true, value_name = "NUM", value_parser = clap::value_parser!(u8).range(1..=26), num_args = 3)]
    /// [possible values: 1 to 26]
    pub rings: Vec<u8>,
    #[arg(long, value_parser = parse_plug_pair, num_args = 0..=13)]
    /// [possible values: two unique alphabet characters]
    pub plugs: Vec<(usize, usize)>,
    #[arg(long, required = true, value_name = "CHAR", value_parser = parse_alphabet_index, num_args = 3)]
    pub key: Vec<usize>,
    #[arg(long)]
    pub input: String,
}

// Converts a string of a single alphabet character into its relative index
fn parse_alphabet_index(s: &str) -> Result<usize, String> {
    let char = char::from_str(&s.to_ascii_uppercase())
        .map_err(|_| "must be a single alphabet character")?;

    let index =
        try_to_alphabet_index(char as u8).map_err(|_| "must be a character between A and Z")?;

    Ok(index)
}

// Converts a string of a two alphabet character into an array of their respective indices
fn parse_plug_pair(s: &str) -> Result<(usize, usize), String> {
    let chars: [char; 2] = s
        .chars()
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| "must be a pair of 2 characters between A and Z")?;

    let left = try_to_alphabet_index(chars[0] as u8)?;
    let right = try_to_alphabet_index(chars[1] as u8)?;

    Ok((left, right))
}
