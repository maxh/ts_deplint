extern crate serde_yaml;

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

use crate::rules::read_rules_file;

fn get_allows(yaml_path: &Path) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let yaml_rules = read_rules_file(yaml_path)?;
    return Ok(yaml_rules.allow);
}

fn get_other_readme_lines(readme_path: &Path) -> io::Result<(Vec<String>, Vec<String>)> {
    match fs::read_to_string(readme_path) {
        Ok(readme) => {
            let readme_lines: Vec<String> = readme.lines().map(|s| s.to_string()).collect();
            let dep_sigil_index = readme_lines
                .iter()
                .position(|line| line.starts_with("%%dep"));
            match dep_sigil_index {
                Some(dep_sigil_idx) => {
                    let block_start_idx = dep_sigil_idx.saturating_sub(1);
                    let block_end_idx = readme_lines[block_start_idx + 1..]
                        .iter()
                        .position(|line| line.starts_with("```"))
                        .map(|idx| idx + block_start_idx + 1)
                        .unwrap_or_else(|| readme_lines.len());

                    let before_dep_diagram_block = readme_lines[0..block_start_idx].to_vec();
                    let after_dep_diagram_block = readme_lines[block_end_idx + 1..].to_vec();

                    Ok((before_dep_diagram_block, after_dep_diagram_block))
                }
                None => Ok((readme_lines.clone(), vec![])),
            }
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok((vec![], vec![])),
        Err(e) => Err(e),
    }
}

pub fn update_readme_with_diagram(
    yaml_path: &Path,
    readme_path: &Path,
) -> Result<(), Box<dyn Error>> {
    let allows = get_allows(yaml_path)?;

    if allows.is_empty() {
        println!("No allows found in the YAML file.");
        return Ok(());
    }

    let (before_dep_diagram_block, after_dep_diagram_block) = get_other_readme_lines(readme_path)?;
    let mermaid_edges = allows
        .iter()
        .flat_map(|(source, targets)| {
            targets
                .iter()
                .map(move |target| format!("  {} --> {}", source, target))
        })
        .collect::<Vec<String>>();

    let mut output_lines = Vec::new();
    output_lines.extend(before_dep_diagram_block);
    output_lines.push("```mermaid".to_string());
    output_lines.push("%%dep".to_string());
    output_lines.push("graph TD".to_string());
    output_lines.push("  subgraph \" \"".to_string());
    output_lines.extend(mermaid_edges);
    output_lines.push("  end".to_string());
    output_lines.push("```".to_string());
    output_lines.extend(after_dep_diagram_block);

    let output_content = output_lines.join("\n");

    let mut file = fs::File::create(readme_path)?;
    file.write_all(output_content.as_bytes())?;

    Ok(())
}
