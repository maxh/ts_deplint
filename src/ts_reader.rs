use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const IGNORE_COMMENT: &str = "// ts_deplint ignore";

pub fn read_ts_imports(ts_path: &Path) -> io::Result<Vec<String>> {
    let ts_file = File::open(ts_path)?;
    let reader = io::BufReader::new(ts_file);

    let mut ts_imports = Vec::new();

    let mut curr_line: String = "".to_string();
    let mut prev_line: String;
    for line in reader.lines() {
        prev_line = curr_line;
        curr_line = line?;

        if prev_line.contains(IGNORE_COMMENT) {
            continue;
        }

        if let Some(ts_import) = extract_import(&curr_line) {
            ts_imports.push(ts_import);
        }
    }

    Ok(ts_imports)
}

fn extract_import(line: &str) -> Option<String> {
    if let Some(start) = line.find("from ") {
        let end = line[start + 6..]
            .find(";")
            .map(|i| start + 6 + i - 1)
            .unwrap_or(line.len());
        let path = &line[start + 6..end];
        return Some(path.to_string());
    }
    None
}
