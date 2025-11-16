use colored::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct ProjectInsight {
    pub project_type: String,
    pub file_counts: HashMap<String, usize>,
    pub total_files: usize,
    pub dependencies: Vec<String>,
    pub structure: Vec<String>,
}

pub fn analyze_project(path: &str) -> ProjectInsight {
    let mut file_counts: HashMap<String, usize> = HashMap::new();
    let mut dependencies: Vec<String> = Vec::new();
    let mut structure: Vec<String> = Vec::new();
    let mut total_files = 0;

    let project_type = detect_project_type(path);
    
    scan_directory(Path::new(path), 0, &mut file_counts, &mut dependencies, &mut structure, &mut total_files, 3);

    ProjectInsight {
        project_type,
        file_counts,
        total_files,
        dependencies,
        structure,
    }
}

fn detect_project_type(path: &str) -> String {
    let path = Path::new(path);
    
    if path.join("Cargo.toml").exists() {
        return "🦀 Rust".to_string();
    }
    if path.join("package.json").exists() {
        return "📦 Node.js/JavaScript".to_string();
    }
    if path.join("requirements.txt").exists() || path.join("pyproject.toml").exists() {
        return "🐍 Python".to_string();
    }
    if path.join("go.mod").exists() {
        return "🐹 Go".to_string();
    }
    if path.join("pom.xml").exists() {
        return "☕ Java (Maven)".to_string();
    }
    if path.join("build.gradle").exists() {
        return "☕ Java (Gradle)".to_string();
    }
    if path.join("Gemfile").exists() {
        return "💎 Ruby".to_string();
    }
    if path.join("composer.json").exists() {
        return "🐘 PHP".to_string();
    }
    
    "📁 Mixed/Unknown".to_string()
}

fn scan_directory(
    dir: &Path,
    depth: usize,
    file_counts: &mut HashMap<String, usize>,
    dependencies: &mut Vec<String>,
    structure: &mut Vec<String>,
    total_files: &mut usize,
    max_depth: usize,
) {
    if depth > max_depth {
        return;
    }

    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        // Skip hidden files and common ignore directories
        if file_name_str.starts_with('.') {
            continue;
        }

        if file_name_str == "target" || file_name_str == "node_modules" || file_name_str == "__pycache__" {
            continue;
        }

        if path.is_dir() {
            let indent = "  ".repeat(depth);
            let dir_name = format!("{}/", file_name_str);
            structure.push(format!("{}{}", indent, dir_name.bright_blue()));
            
            scan_directory(
                &path,
                depth + 1,
                file_counts,
                dependencies,
                structure,
                total_files,
                max_depth,
            );
        } else {
            *total_files += 1;
            
            // Get file extension
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("no-ext")
                .to_string();

            *file_counts.entry(ext.clone()).or_insert(0) += 1;

            // Detect dependency files
            let file_name_str_ref: &str = &file_name_str;
            match file_name_str_ref {
                "Cargo.toml" | "package.json" | "requirements.txt" | "go.mod" | "pom.xml" | "Gemfile" | "composer.json" => {
                    dependencies.push(file_name_str.to_string());
                }
                _ => {}
            }

            if depth < 2 {
                let indent = "  ".repeat(depth);
                let file_name_colored = file_name_str.to_string().bright_white();
                structure.push(format!("{}{}", indent, file_name_colored));
            }
        }
    }
}

pub fn print_insight(insight: &ProjectInsight) {
    println!("\n{}", "=".repeat(60).bright_cyan());
    println!("{} {}", "🔍 PROJECT INSIGHT".bright_cyan().bold(), "=".repeat(40).bright_cyan());
    println!("{}\n", "=".repeat(60).bright_cyan());

    // Project Type
    println!("{} {}", "📌 Project Type:".bright_yellow(), insight.project_type);

    // Statistics
    println!("\n{} {}", "📊 Statistics:".bright_yellow(), format!("Total files: {}", insight.total_files).bright_white());

    // File Types
    if !insight.file_counts.is_empty() {
        println!("\n{}", "📁 File Types:".bright_yellow());
        let mut sorted_types: Vec<_> = insight.file_counts.iter().collect();
        sorted_types.sort_by(|a, b| b.1.cmp(a.1));
        
        for (ext, count) in sorted_types.iter().take(10) {
            let ext_display = if ext.as_str() == "no-ext" {
                "no extension".to_string()
            } else {
                format!(".{}", ext)
            };
            println!("  {} {}", ext_display.bright_cyan(), format!("({} files)", count).bright_white());
        }
    }

    // Dependencies
    if !insight.dependencies.is_empty() {
        println!("\n{}", "📦 Dependency Files:".bright_yellow());
        for dep in &insight.dependencies {
            println!("  {}", dep.bright_green());
        }
    }

    // Structure
    if !insight.structure.is_empty() {
        println!("\n{}", "🌳 Project Structure:".bright_yellow());
        for line in &insight.structure {
            println!("{}", line);
        }
    }

    println!("\n{}", "=".repeat(60).bright_cyan());
    println!();
}

