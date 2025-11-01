// Tests for markdown file validation
use std::fs;
use std::path::Path;

/// Recursively finds all .md files in a directory
fn find_markdown_files(dir: &Path, files: &mut Vec<std::path::PathBuf>) {
    if dir.is_dir() {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    // Skip target and .git directories
                    if let Some(name) = path.file_name() {
                        if name != "target" && name != ".git" {
                            find_markdown_files(&path, files);
                        }
                    }
                } else if let Some(ext) = path.extension() {
                    if ext == "md" {
                        files.push(path);
                    }
                }
            }
        }
    }
}

#[test]
fn test_all_markdown_files_are_ascii_only() {
    // Get the workspace root (parent of matrix crate)
    let project_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let mut markdown_files = Vec::new();

    find_markdown_files(project_root, &mut markdown_files);

    assert!(!markdown_files.is_empty(), "No markdown files found in project");

    let mut non_ascii_files = Vec::new();

    for file in &markdown_files {
        if let Ok(content) = fs::read_to_string(file) {
            // Check if all characters are ASCII
            if !content.is_ascii() {
                // Find the first non-ASCII character for helpful error message
                for (line_num, line) in content.lines().enumerate() {
                    if !line.is_ascii() {
                        for (col, ch) in line.chars().enumerate() {
                            if !ch.is_ascii() {
                                non_ascii_files.push(format!(
                                    "{} (line {}, col {}): found '{}' (U+{:04X})",
                                    file.display(),
                                    line_num + 1,
                                    col + 1,
                                    ch,
                                    ch as u32
                                ));
                                break;
                            }
                        }
                        break;
                    }
                }
            }
        }
    }

    if !non_ascii_files.is_empty() {
        panic!(
            "Found {} markdown file(s) with non-ASCII characters:\n{}",
            non_ascii_files.len(),
            non_ascii_files.join("\n")
        );
    }
}

#[test]
fn test_readme_is_ascii() {
    let readme_path = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("README.md");
    let content = fs::read_to_string(&readme_path)
        .expect("Failed to read README.md");

    assert!(
        content.is_ascii(),
        "README.md contains non-ASCII characters"
    );
}

#[test]
fn test_claude_md_is_ascii() {
    let claude_path = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("CLAUDE.md");
    let content = fs::read_to_string(&claude_path)
        .expect("Failed to read CLAUDE.md");

    assert!(
        content.is_ascii(),
        "CLAUDE.md contains non-ASCII characters"
    );
}
