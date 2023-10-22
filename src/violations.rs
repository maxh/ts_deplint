use std::collections::HashMap;

#[derive(Debug)]
pub struct Violation {
    pub file_path: String,
    pub disallowed_import: String,
}

pub fn pretty_print_violations(violations: Vec<Violation>) {
    // Cluster violations by file path
    let mut disallowed_imports_by_file_path: HashMap<String, Vec<String>> = HashMap::new();

    for violation in violations {
        disallowed_imports_by_file_path
            .entry(violation.file_path.clone())
            .or_default()
            .push(violation.disallowed_import);
    }

    // Print each cluster with the file path on one line
    // each disallowed import on a new line
    // and a blank line between each Cluster
    for (file_path, disallowed_imports) in disallowed_imports_by_file_path {
        println!("{}", file_path);
        for disallowed_import in disallowed_imports {
            println!("  {}", disallowed_import);
        }
    }
}
