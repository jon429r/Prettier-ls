use clap::Parser;
use colored::*;
use std::fs;
use std::path::Path;

/// A program to display directories and files in a tree-like format
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the directory to display
    #[arg(short, long, default_value = ".")]
    path: String,

    /// Maximum number of files to show in the root directory
    #[arg(short = 'r', long, default_value_t = 10)]
    root_limit: usize,

    /// Maximum number of files to show in each subdirectory
    #[arg(short = 's', long, default_value_t = 4)]
    sub_limit: usize,

    /// Number of levels to show in the tree
    #[arg(short = 'l', long, default_value_t = 3)]
    levels: usize,

    /// Whether to show hidden files (starting with '.')
    #[arg(short = 'a', long, default_value_t = false)]
    show_hidden: bool,
}

fn print_tree(
    path: &Path,
    prefix: &str,
    is_last: bool,
    max_files: usize,
    show_hidden: bool,
) -> std::io::Result<()> {
    let symbol = if is_last { "└── " } else { "├── " };

    if let Some(file_name) = path.file_name() {
        let styled_name = if path.is_dir() {
            file_name.to_string_lossy().blue().bold()
        } else if let Some(extension) = path.extension() {
            match extension.to_string_lossy().as_ref() {
                "rs" => file_name.to_string_lossy().yellow(),
                "txt" => file_name.to_string_lossy().cyan(),
                _ => file_name.to_string_lossy().green(),
            }
        } else {
            file_name.to_string_lossy().green() // Files in green
        };
        println!("{}{}{}", prefix, symbol, styled_name);
    }

    if path.is_dir() {
        let mut entries: Vec<_> = fs::read_dir(path)?
            .filter_map(|e| e.ok())
            .filter(|entry| show_hidden || !entry.file_name().to_string_lossy().starts_with('.'))
            .collect();

        entries.sort_by_key(|e| e.file_name());
        let total_entries = entries.len();
        let limited_entries = entries.iter().take(max_files);

        for (i, entry) in limited_entries.enumerate() {
            let child_path = entry.path();
            let new_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };

            print_tree(
                &child_path,
                &new_prefix,
                i == max_files - 1 || i == total_entries - 1,
                max_files,
                show_hidden,
            )?;
        }

        if total_entries > max_files {
            println!(
                "{}{}... ({} more entries)",
                prefix,
                if is_last { "    " } else { "│   " },
                total_entries - max_files
            );
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let root_path = Path::new(&args.path);

    println!("{}", root_path.display());
    print_tree(
        root_path,
        "",
        true,
        args.sub_limit, // Pass subdirectory limit
        args.show_hidden,
    )?;

    Ok(())
}
