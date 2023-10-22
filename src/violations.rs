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
        let disallowed_imports_parts = disallowed_imports
            .iter()
            .map(|disallowed_import| {
                let mut common_prefix = file_path
                    .chars()
                    .zip(disallowed_import.chars())
                    .take_while(|(a, b)| a == b)
                    .map(|(a, _)| a)
                    .collect::<String>();
                // strip any chars after the last /
                if let Some(last_slash_idx) = common_prefix.rfind('/') {
                    common_prefix.truncate(last_slash_idx + 1);
                }
                let dir_after_common_prefix = file_path
                    .chars()
                    .skip(common_prefix.len())
                    .take_while(|c| *c != '/')
                    .collect::<String>();
                let disallowed_after_common_prefix = disallowed_import
                    .chars()
                    .skip(common_prefix.len())
                    .take_while(|c| *c != '/')
                    .collect::<String>();
                (
                    common_prefix,
                    dir_after_common_prefix,
                    disallowed_after_common_prefix,
                )
            })
            .collect::<Vec<(String, String, String)>>();
        let parts_by_common_prefix = disallowed_imports_parts
            .iter()
            .map(
                |(common_prefix, dir_after_common_prefix, disallowed_after_common_prefix)| {
                    (
                        common_prefix.clone(),
                        vec![(
                            dir_after_common_prefix.clone(),
                            disallowed_after_common_prefix.clone(),
                        )],
                    )
                },
            )
            .collect::<HashMap<String, Vec<(String, String)>>>();
        let mut common_prefixes = parts_by_common_prefix.keys().collect::<Vec<&String>>();
        common_prefixes.sort();
        for common_prefix in common_prefixes {
            let parts = &parts_by_common_prefix[common_prefix];
            let mut dirs = parts.iter().map(|(dir, _)| dir).collect::<Vec<&String>>();
            dirs.sort();
            for dir in dirs {
                let disallowed_imports = parts
                    .iter()
                    .filter(|(d, _)| d == dir)
                    .map(|(_, disallowed_import)| disallowed_import)
                    .collect::<Vec<&String>>();
                for disallowed_import in disallowed_imports {
                    println!("  {} is disallowed", disallowed_import);
                }
                println!("    per rules in {}", common_prefix);
            }
        }
    }
}
