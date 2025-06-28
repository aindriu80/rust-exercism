use anyhow::{Error, Result};
use std::fs;

#[derive(Debug)]
pub struct Flags {
    line_numbers: bool,     // -n
    files_only: bool,       // -l
    case_insensitive: bool, // -i
    invert: bool,           // -v
    entire_line: bool,      // -x
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Self {
            line_numbers: flags.contains(&"-n"),
            files_only: flags.contains(&"-l"),
            case_insensitive: flags.contains(&"-i"),
            invert: flags.contains(&"-v"),
            entire_line: flags.contains(&"-x"),
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut results = Vec::new();
    let multiple_files = files.len() > 1;

    // Prepare pattern for matching
    let search_pattern = if flags.case_insensitive {
        pattern.to_lowercase()
    } else {
        pattern.to_string()
    };

    for filename in files {
        // Read file content
        let content = fs::read_to_string(filename)
            .map_err(|e| anyhow::anyhow!("Failed to read file '{}': {}", filename, e))?;

        let mut file_matches = Vec::new();
        let mut has_match = false;

        for (line_num, line) in content.lines().enumerate() {
            let line_to_check = if flags.case_insensitive {
                line.to_lowercase()
            } else {
                line.to_string()
            };

            // Check if line matches based on flags
            let matches = if flags.entire_line {
                line_to_check == search_pattern
            } else {
                line_to_check.contains(&search_pattern)
            };

            // Apply invert flag
            let should_include = if flags.invert { !matches } else { matches };

            if should_include {
                has_match = true;

                // If files_only flag is set, we only need to know there's a match
                if flags.files_only {
                    break;
                }

                // Format the output line
                let formatted_line = if multiple_files && flags.line_numbers {
                    format!("{}:{}:{}", filename, line_num + 1, line)
                } else if multiple_files {
                    format!("{}:{}", filename, line)
                } else if flags.line_numbers {
                    format!("{}:{}", line_num + 1, line)
                } else {
                    line.to_string()
                };

                file_matches.push(formatted_line);
            }
        }

        // Handle output based on flags
        if flags.files_only {
            if has_match {
                results.push(filename.to_string());
            }
        } else {
            results.extend(file_matches);
        }
    }

    Ok(results)
}
