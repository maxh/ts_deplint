use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
};

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Violation {
    DisallowedImportViolation(DisallowedImportViolation),
    ReferenceToNonexistentDirectory(ReferenceToNonexistentDirectory),
}

#[derive(Debug)]
pub struct DisallowedImportViolation {
    pub file_path: String,
    pub disallowed_import: String,
    pub full_disallowed_import: String,
}

impl Hash for DisallowedImportViolation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file_path.hash(state);
        self.disallowed_import.hash(state);
        self.full_disallowed_import.hash(state);
    }
}

impl PartialEq for DisallowedImportViolation {
    fn eq(&self, other: &Self) -> bool {
        self.file_path == other.file_path
            && self.disallowed_import == other.disallowed_import
            && self.full_disallowed_import == other.full_disallowed_import
    }
}

impl Eq for DisallowedImportViolation {}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct ReferenceToNonexistentDirectory {
    pub rules_file_path: String,
    pub directory_name: String,
    pub user_message: String,
}

pub fn pretty_print_violations<I>(violations: I)
where
    I: IntoIterator<Item = Violation>,
{
    // Cluster disallowed import violations by file path
    let mut disallowed_imports_by_file_path: HashMap<String, HashSet<String>> = HashMap::new();
    let mut full_disallowed_imports_by_file_path_plus_disallowed_import: HashMap<
        String,
        HashSet<String>,
    > = HashMap::new();

    for violation in violations {
        match violation {
            Violation::DisallowedImportViolation(violation) => {
                let key = format!("{}:{}", violation.file_path, violation.disallowed_import);
                disallowed_imports_by_file_path
                    .entry(violation.file_path)
                    .or_default()
                    .insert(violation.disallowed_import);
                full_disallowed_imports_by_file_path_plus_disallowed_import
                    .entry(key)
                    .or_default()
                    .insert(violation.full_disallowed_import);
            }
            Violation::ReferenceToNonexistentDirectory(issue) => {
                println!("{}", issue.user_message);
            }
        }
    }

    for (file_path, disallowed_imports) in disallowed_imports_by_file_path {
        println!("{}", file_path);
        for disallowed_import in disallowed_imports {
            let key = format!("{}:{}", file_path, disallowed_import);
            println!("  imports {}", disallowed_import);
            let full_disallowed_imports =
                full_disallowed_imports_by_file_path_plus_disallowed_import
                    .get(&key)
                    .expect("full_disallowed_imports_by_file_path_plus_disallowed_imports");
            let sorted_full_disallowed_imports = {
                let mut sorted_full_disallowed_imports = Vec::from_iter(full_disallowed_imports);
                sorted_full_disallowed_imports.sort();
                sorted_full_disallowed_imports
            };
            for full_disallowed_import in sorted_full_disallowed_imports {
                println!("     {}", full_disallowed_import);
            }
        }
        println!();
    }
}
