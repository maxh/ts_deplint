use std::fs::File;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;

#[derive(Serialize, Deserialize)]
pub struct Rules {
    allow: HashMap<String, Vec<String>>,
}

pub fn parse_rules_file(file_path: &str) -> Result<Rules, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut json_content = String::new();
    file.read_to_string(&mut json_content)?;

    let rules: Rules = serde_json::from_str(&json_content)?;

    Ok(rules)
}
