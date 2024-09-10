pub mod utils;
use clap::{Arg, Command};
use utils::{get_commits, print_commit_stats, sort_commits};

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

    let commits = if *should_sort {
        sort_commits(get_commits(keyword, exclude_merge))
    } else {
        get_commits(keyword, exclude_merge)
    };

    print_commit_stats(commits);
}
