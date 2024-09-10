<p align="center">
  <img src="https://raw.githubusercontent.com/PKief/vscode-material-icon-theme/ec559a9f6bfd399b82bb44393651661b08aaf7ba/icons/folder-markdown-open.svg" width="20%" alt="<code>❯ REPLACE-ME</code>-logo">
</p>
<p align="center">
    <h1 align="center"><code>❯ GitCommitStats</code></h1>
</p>
<p align="center">
	<!-- local repository, no metadata badges. --></p>
<p align="center">
		<em>Built with the tools and technologies:</em>
</p>
<p align="center">
	<img src="https://img.shields.io/badge/Rust-000000.svg?style=default&logo=Rust&logoColor=white" alt="Rust">
</p>

<br>

#####  Table of Contents

- [Overview](#overview)
- [Repository Structure](#repository-structure)
- [Modules](#modules)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Usage](#usage)

---

##  Overview

Gitcommit_stats is a valuable tool enabling users to track and analyze historical Git repository commits efficiently. By providing an effective summary of recent commit transactions based on configurable keyword searches, it helps reveal essential information about coding changes. The system seamlessly excludes non-committer actions by default, allowing for focused tracking and evaluation of actual development progress.

---

##  Repository Structure

```sh
└── /
    ├── Cargo.lock
    ├── Cargo.toml
    └── src
        └── main.rs
```

---

##  Modules

<details closed><summary>.</summary>

| File | Summary |
| --- | --- |
| [Cargo.toml](Cargo.toml) | Configuring package attributes, defining dependencies, ensuring version control integrity and compatibility through careful manipulation of crates like Clap, ultimately harmonizing functionality within an efficient build workflow for the git_commit_stats repository. |

</details>

<details closed><summary>src</summary>

| File | Summary |
| --- | --- |
| [main.rs](src\main.rs) | Lists recent commits based on keyword searches+ Excludes merge commits by default unless flag specified* Tracks commitment metrics (insertions, deletions, files changed) per commit+ Generates a formatted summary output |

</details>

---

##  Getting Started

###  Prerequisites

**Rust**: `version x.y.z`

###  Installation

Build the project from source:

1. Clone the  repository:
```sh
❯ git clone https://github.com/Lemonnnnnnnnnnn/git-commit-stats
```

2. Navigate to the project directory:
```sh
❯ cd git-commit-stats
```

3. Install the required dependencies:
```sh
❯ cargo build
```

###  Usage

To run the project, execute the following command:

```sh
❯ cargo run "keyword"
```
