use std::env;
use std::process::Command;
use std::str;

fn main() {
    // 获取命令行参数中的关键字
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: git_commit_stats <keyword>");
        return;
    }
    let keyword = &args[1];

    // 获取包含关键字的 commit 列表
    let output = Command::new("git")
        .arg("log")
        .arg(format!("--grep={}", keyword))
        .arg("--pretty=format:%H")
        .output()
        .expect("Failed to execute git log");

    let commits = String::from_utf8_lossy(&output.stdout);
    let commits: Vec<&str> = commits.lines().collect();

    println!("commits length {}", commits.len());

    let mut total_added = 0;
    let mut total_deleted = 0;

    for commit in commits {
        let output = Command::new("git")
            .arg("diff")
            .arg("--shortstat")
            .arg(commit)
            .output()
            .expect("Failed to execute git command");

        let diff_stats = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = diff_stats.lines().collect();

        if !lines.is_empty() {
            let last_line = lines.last().unwrap();
            let parts: Vec<&str> = last_line.split_whitespace().collect();

            if parts.len() >= 6 {
                // let added = usize::from_str(parts[3]).unwrap();
                let added: usize = parts[3].parse().unwrap();
                let deleted: usize = parts[5].parse().unwrap();
                // let deleted = usize::from_str(parts[5]).unwrap();

                println!(
                    "Commit {}: Added {} lines, Deleted {} lines",
                    commit, added, deleted
                );

                total_added += added;
                total_deleted += deleted;
            }
        }
    }

    println!("Total Added: {} lines", total_added);
    println!("Total Deleted: {} lines", total_deleted);
}
