#[macro_use]
extern crate rustling;

use std::result;

#[macro_use]
mod macros;
mod helpers;
mod dimension;
mod examples;
mod en;
mod fr;
mod parser;

pub use rustling::{ ParserMatch, Range, DucklingResult };
pub use dimension::{Dimension, IntegerValue, NumberValue, FloatValue, OrdinalValue, TemperatureValue};

#[derive(Copy,Clone,Debug)]
pub enum Lang { EN, FR }

impl std::str::FromStr for Lang {
    type Err=String;
    fn from_str(it: &str) -> result::Result<Lang,Self::Err> {
        match &*it.to_lowercase() {
            "en" => Ok(Lang::EN),
            "fr" => Ok(Lang::FR),
            _ => Err(format!("Unknown language {}", it))
        }
    }
}

impl ::std::string::ToString for Lang {
    fn to_string(&self) -> String {
        match self {
            &Lang::EN => "en".to_string(),
            &Lang::FR => "fr".to_string(),
        }
    }
}

pub type Parser = rustling::Parser<Dimension, parser::Feat, parser::FeatureExtractor>;

pub fn build_parser(lang:Lang) -> DucklingResult<Parser> {
    match lang {
        Lang::EN => build_parser_en(),
        Lang::FR => build_parser_fr(),
    }
}

fn build_parser_en() -> DucklingResult<Parser> {
    let rules = en::rules_numbers()?;
    let exs = en::examples_numbers();
    let model = rustling::train::train(&rules, exs, parser::FeatureExtractor())?;
    Ok(rustling::Parser::new(rules, model, parser::FeatureExtractor()))
}

fn build_parser_fr() -> DucklingResult<Parser> {
    let rules = fr::rules_numbers()?;
    let exs = fr::examples_numbers();
    let model = rustling::train::train(&rules, exs, parser::FeatureExtractor())?;
    Ok(rustling::Parser::new(rules, model, parser::FeatureExtractor()))
}

