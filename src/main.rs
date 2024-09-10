use std::env;
use std::process::Command;
use std::str;

fn get_commit_list(keyword: &str, include_merge: bool) -> Vec<String> {
    let k = format!("--grep={}", keyword);
    let mut args = vec!["log", &k, "--pretty=format:%H"];

    if !include_merge {
        args.push("--no-merges");
    }

    let output = Command::new("git")
        .args(&args)
        .output()
        .expect("Failed to execute git log");

    let commits = String::from_utf8_lossy(&output.stdout);
    commits.lines().map(String::from).collect()
}

fn calculate_diff(commit: &str) -> (usize, usize) {
    let output = Command::new("git")
        .arg("diff")
        .arg("--shortstat")
        .arg(commit)
        .output()
        .expect("Failed to execute git diff");

    let diff_stats = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = diff_stats.lines().collect();

    if let Some(last_line) = lines.last() {
        let parts: Vec<&str> = last_line.split_whitespace().collect();
        if parts.len() >= 6 {
            let added: usize = parts[3].parse().unwrap_or(0);
            let deleted: usize = parts[5].parse().unwrap_or(0);
            return (added, deleted);
        }
    }
    (0, 0)
}

fn print_commit_stats(commits: Vec<String>) {
    let mut total_added = 0;
    let mut total_deleted = 0;

    for commit in commits {
        let (added, deleted) = calculate_diff(&commit);

        println!(
            "Commit {}: Added {} lines, Deleted {} lines",
            commit, added, deleted
        );

        total_added += added;
        total_deleted += deleted;
    }

    println!("Total Added: {} lines", total_added);
    println!("Total Deleted: {} lines", total_deleted);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: git_commit_stats <keyword> [--merge | --no-merge]");
        return;
    }
    let keyword = &args[1];
    let include_merge = if args.len() > 2 {
        match args[2].as_str() {
            "--merge" => true,
            "--no-merge" => false,
            _ => {
                eprintln!("Invalid option: {}. Use --merge or --no-merge", args[2]);
                return;
            }
        }
    } else {
        true // 默认包含 merge
    };

    let commits = get_commit_list(keyword, include_merge);
    println!("Commits length: {}", commits.len());

    print_commit_stats(commits);
}
