use serde::Deserialize;
use std::collections::HashMap;

pub type Domain = String;
pub type Medium = String;
pub type Source = String;

pub struct Referer {
    pub source: Source,
    pub medium: Medium,
    pub params: Vec<String>,
}

#[derive(Deserialize)]
pub struct Referers {
    #[serde(flatten)]
    pub data: HashMap<Medium, HashMap<Source, RefSource>>,
}

#[derive(Deserialize)]
pub struct RefSource {
    pub domains: Vec<String>,
    pub parameters: Option<Vec<String>>,
}
