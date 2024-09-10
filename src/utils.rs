use std::process::Command as GitCommand;
use std::str;

#[derive(Debug, Clone, PartialEq)]
pub struct Commit {
    commit_hash: String,
    files_changed: usize,
    added: usize,
    deleted: usize,
}

pub fn get_commits(keyword: &str, exclude_merge: &bool) -> Vec<Commit> {
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

    commits
        .lines()
        .map(|commit| {
            let (files_changed, added, deleted) = calculate_diff_info(commit);
            Commit {
                commit_hash: commit.to_string(),
                files_changed,
                added,
                deleted,
            }
        })
        .collect()
}

pub fn calculate_diff_info(commit: &str) -> (usize, usize, usize) {
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

pub fn sort_commits(commits: Vec<Commit>) -> Vec<Commit> {
    let mut commit_stats = commits;

    commit_stats.sort_by(|a, b| b.added.cmp(&a.added)); // 按插入行数降序排序

    commit_stats
}

pub fn print_commit_stats(commit_stats: Vec<Commit>) {
    println!("Commits length: {}", commit_stats.len());

    let mut total_added = 0;
    let mut total_deleted = 0;

    for commit in commit_stats {
        println!(
            "Commit {}: {} insertions(+), {} deletions(-),{} files changed",
            commit.commit_hash, commit.added, commit.deleted, commit.files_changed,
        );

        total_added += commit.added;
        total_deleted += commit.deleted;
    }

    println!(
        "Total: {} insertions(+), {} deletions(-)",
        total_added, total_deleted
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_commits() {
        let keyword = "README";
        let exclude_merge = true;
        let expected_commit_id = "cd13ae5251d6aaf12e4497d6285a6aa1b507eb42";

        let commits = get_commits(keyword, &exclude_merge);
        assert!(!commits.is_empty());

        let expected_commit = Commit {
            commit_hash: expected_commit_id.to_string(),
            files_changed: 1,
            added: 99,
            deleted: 0,
        };

        assert!(commits.contains(&expected_commit));
    }

    // 测试 calculate_diff_info 函数
    #[test]
    fn test_calculate_diff_info() {
        let commit_hash = "cd13ae5251d6aaf12e4497d6285a6aa1b507eb42";

        // 模拟 diff 输出
        let (files_changed, added, deleted) = calculate_diff_info(commit_hash);

        // 验证结果是否符合预期
        assert_eq!(files_changed, 1);
        assert_eq!(added, 99);
        assert_eq!(deleted, 0);
    }

    // 测试 sort_commits 函数
    #[test]
    fn test_sort_commits() {
        // 创建一些虚拟的 commit 数据
        let commits = vec![
            Commit {
                commit_hash: "commit1".to_string(),
                files_changed: 2,
                added: 10,
                deleted: 5,
            },
            Commit {
                commit_hash: "commit2".to_string(),
                files_changed: 3,
                added: 50,
                deleted: 10,
            },
            Commit {
                commit_hash: "commit3".to_string(),
                files_changed: 1,
                added: 30,
                deleted: 2,
            },
        ];

        // 按插入行数降序排序
        let sorted_commits = sort_commits(commits.clone());

        // 验证排序结果
        assert_eq!(sorted_commits[0].added, 50);
        assert_eq!(sorted_commits[1].added, 30);
        assert_eq!(sorted_commits[2].added, 10);
    }
}
