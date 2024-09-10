use clap::{Arg, Command};
use std::process::Command as GitCommand;
use std::str;

fn get_commit_list(keyword: &str, exclude_merge: &bool) -> Vec<String> {
    let k = format!("--grep={}", keyword);
    let mut args = vec!["log", &k, "--pretty=format:%H"];

    if !exclude_merge {
        args.push("--no-merges");
    }

    let output = GitCommand::new("git")
        .args(&args)
        .output()
        .expect("Failed to execute git log");

    let commits = String::from_utf8_lossy(&output.stdout);
    commits.lines().map(String::from).collect()
}

fn calculate_diff(commit: &str) -> (usize, usize, usize) {
    let output = GitCommand::new("git")
        .arg("show")
        .arg(commit)
        .arg("--pretty=format:") // 只显示 diff，不显示 commit 信息
        .arg("--numstat") // 显示每个文件的新增/删除行数
        .output()
        .expect("Failed to execute git show");

    let diff_output = String::from_utf8_lossy(&output.stdout);
    let mut total_added = 0;
    let mut total_deleted = 0;
    let mut file_count = 0;

    // 遍历每一行输出，统计每个文件的行数变化
    for line in diff_output.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let added: usize = parts[0].parse().unwrap_or(0);
            let deleted: usize = parts[1].parse().unwrap_or(0);

            total_added += added;
            total_deleted += deleted;
            file_count += 1;
        }
    }

    (file_count, total_added, total_deleted)
}

fn print_commit_stats(commits: Vec<String>, should_sort: &bool) {
    let mut commit_stats = Vec::new();

    for commit in &commits {
        let (files_changed, added, deleted) = calculate_diff(commit);
        commit_stats.push((commit.clone(), files_changed, added, deleted));
    }

    if *should_sort {
        commit_stats.sort_by(|a, b| b.2.cmp(&a.2)); // 按插入行数降序排序
    }

    let mut total_files = 0;
    let mut total_added = 0;
    let mut total_deleted = 0;

    for (commit, files_changed, added, deleted) in commit_stats {

        println!(
            "Commit {}: {} insertions(+), {} deletions(-),{} files changed",
            commit, added, deleted, files_changed,
        );

        total_files += files_changed;
        total_added += added;
        total_deleted += deleted;
    }

    println!(
        "Total: {} insertions(+), {} deletions(-) , {} files changed",
        total_added, total_deleted, total_files
    );
}

fn main() {
    let matches = Command::new("git_commit_stats")
        .version("1.0")
        .about("Git commit statistics tool")
        .arg(
            Arg::new("keyword")
                .help("The keyword to search in commits")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("all")
                .help("Exclude merge commits")
                .long("all")
                .action(clap::ArgAction::SetTrue), // 默认值为 false，提供参数时为 true
        )
        .arg(
            Arg::new("sort")
                .help("Sort commits by commit hash")
                .long("sort")
                .action(clap::ArgAction::SetFalse), // 默认值为 true，提供参数时为 false
        )
        .get_matches();

    // 获取命令行参数的值
    let keyword = matches.get_one::<String>("keyword").unwrap();
    let exclude_merge = matches.get_one::<bool>("all").unwrap_or(&false); // 默认值为 false
    let should_sort = matches.get_one::<bool>("sort").unwrap_or(&true); //默认值为 true

    let commits = get_commit_list(keyword, exclude_merge);
    println!("Commits length: {}", commits.len());

    print_commit_stats(commits, should_sort);
}
