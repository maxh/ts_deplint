use std::{collections::HashMap, hash::Hash, hash::Hasher};

#[derive(Debug)]
pub struct Violation {
    pub file_path: String,
    pub disallowed_import: String,
}

impl Hash for Violation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file_path.hash(state);
        self.disallowed_import.hash(state);
    }
}

impl PartialEq for Violation {
    fn eq(&self, other: &Self) -> bool {
        self.file_path == other.file_path && self.disallowed_import == other.disallowed_import
    }
}

impl Eq for Violation {}

pub fn pretty_print_violations<I>(violations: I)
where
    I: IntoIterator<Item = Violation>,
{
    // Cluster violations by file path
    let mut disallowed_imports_by_file_path: HashMap<String, Vec<String>> = HashMap::new();

    for violation in violations {
        disallowed_imports_by_file_path
            .entry(violation.file_path.clone())
            .or_default()
            .push(violation.disallowed_import);
    }

    for (file_path, disallowed_imports) in disallowed_imports_by_file_path {
        println!("{}", file_path);
        for disallowed_import in disallowed_imports {
            println!("  {}", disallowed_import);
        }
    }
}
