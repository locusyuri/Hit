# Hit 实测报告（自动批量执行）
> 生成时间: 2026-06-27 22:19:05
> 按 TEST_FLOW.md 顺序，stdout 入本文件，stderr(WARN) 入 REPORT_warn.log。
> 跳过: §1/§15/§12.3.1/§20/§14.2/§14.4/§19.4


────────────────────────────────────────────────
## §0.3
────────────────────────────────────────────────
命令: hit --version
输出（原样）:
hit 0.1.0


────────────────────────────────────────────────
## §0.3
────────────────────────────────────────────────
命令: hit --help
输出（原样）:
Hit — Scoop 兼容的 Windows 包管理器

Usage: hit.exe [OPTIONS] <COMMAND>

Commands:
  install    安装软件包
  search     搜索软件包
  update     更新已安装软件
  uninstall  卸载软件
  list       列出已安装软件
  status     查看系统状态
  bucket     管理 Bucket 仓库
  info       查看软件包详情
  reset      切换软件版本
  cache      管理下载缓存
  home       打开软件主页
  cleanup    清理旧版本与缓存
  which      查找命令对应的 shim 路径
  prefix     显示安装路径
  hold       锁定软件版本（update 时跳过）
  unhold     解除版本锁定
  config     管理配置
  doctor     健康检查与修复
  si         交互式搜索并安装
  help       Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  日志级别（-v / -vv / -vvv）
  -h, --help        Print help
  -V, --version     Print version


────────────────────────────────────────────────
## §0.4
────────────────────────────────────────────────
命令: hit prefix
输出（原样）:
C:\Users\Violet\Downloads\test\hit


────────────────────────────────────────────────
## §0.4
────────────────────────────────────────────────
命令: hit config list
输出（原样）:
proxy                          (未设置)
mirror                         (未设置)
aria2_enabled                  false
no_junction                    false
root_path                      (未设置)
auto_cleanup_days              30
health_check_interval_days     7


────────────────────────────────────────────────
## §2.1.1
────────────────────────────────────────────────
命令: hit bucket add main
输出（原样）:

────────────────────────────────────────────────
## §2.1.4
────────────────────────────────────────────────
命令: hit bucket add main
输出（原样）:

────────────────────────────────────────────────
## §2.1.5
────────────────────────────────────────────────
命令: hit bucket add myrepo https://github.com/user/repo.git
输出（原样）:
添加 正在添加 bucket 'myrepo'...
正在克隆 bucket 'myrepo'...


────────────────────────────────────────────────
## §2.1.6
────────────────────────────────────────────────
命令: hit bucket add unknownbucket
输出（原样）:

────────────────────────────────────────────────
## §2.2.1
────────────────────────────────────────────────
命令: hit bucket list
输出（原样）:
名称                  Manifest    描述
extras                2319        
main                  1591        
versions              592         

共 3 个 Bucket


────────────────────────────────────────────────
## §2.2.2
────────────────────────────────────────────────
命令: hit b ls
输出（原样）:
名称                  Manifest    描述
extras                2319        
main                  1591        
versions              592         

共 3 个 Bucket


────────────────────────────────────────────────
## §2.3.1
────────────────────────────────────────────────
命令: hit bucket update
输出（原样）:
正在更新 bucket 'extras'...
正在克隆 bucket 'extras'...
  ✘ extras 失败: Bucket 'extras' 错误：克隆失败：Could not decode server reply
正在更新 bucket 'main'...
正在克隆 bucket 'main'...
bucket 'main'：检出文件中...
bucket 'main' 克隆完成
  ✔ main
正在更新 bucket 'versions'...
正在克隆 bucket 'versions'...
  ✘ versions 失败: Bucket 'versions' 错误：克隆失败：Could not decode server reply

✔ Bucket 更新完成（1/3）


────────────────────────────────────────────────
## §2.3.2
────────────────────────────────────────────────
命令: hit bucket update main
输出（原样）:
正在更新 bucket 'main'...
正在克隆 bucket 'main'...
  ✘ main 失败: Bucket 'main' 错误：克隆失败：An IO error occurred when talking to the server

✔ Bucket 更新完成（0/1）


────────────────────────────────────────────────
## §2.3.3
────────────────────────────────────────────────
命令: hit bucket update nonexistent
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
没有可更新的 Bucket


────────────────────────────────────────────────
## §2.4.1
────────────────────────────────────────────────
命令: hit bucket remove myrepo
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。


────────────────────────────────────────────────
## §2.4.2
────────────────────────────────────────────────
命令: hit bucket rm main
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。


────────────────────────────────────────────────
## §2.4.3
────────────────────────────────────────────────
命令: hit bucket remove nonexistent
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。


────────────────────────────────────────────────
## §2.4-restore
────────────────────────────────────────────────
命令: hit bucket add main
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
添加 正在添加 bucket 'main'...
正在克隆 bucket 'main'...
bucket 'main'：检出文件中...
bucket 'main' 克隆完成
✔ bucket 'main' 添加完成


────────────────────────────────────────────────
## §3.1
────────────────────────────────────────────────
命令: hit search git
输出（原样）:
[2m2026-06-27T14:21:02.477648Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

名称           版本         描述
act          0.2.89     Local Github actions runner.
actionlint   1.7.12     Static checker for GitHub Actions workflow files
argocd       3.4.4      CLI for ArgoCD - declarative, GitOps continuous delivery tool for Kubernetes
argocd-autopilot 0.4.20     A tool which offers an opinionated way of installing Argo-CD and managing GitOps repositories.
bat          0.26.1     A cat(1) clone with syntax highlighting and Git integration
bfg          1.15.0     BFG Repo-Cleaner removes large or troublesome blobs like git-filter-branch does, but faster
bit          1.1.2      Modernized git CLI
carvel-vendir 0.46.0     Carvel vendir is a tool that makes it easy to vendor portions of git repos, github releases, helm charts, docker image contents, etc. declaratively
claude-code  2.1.195    An agentic coding tool built by Anthropic that lives in your terminal, understands your codebase, and helps you code faster by executing routine tasks, explaining complex code, and handling git workflows.
cocogitto    7.0.0      The Conventional Commits toolbox.
codeowners-validator 0.7.4      The GitHub Codeowners file validator
codeql       2.25.6     Source code security analyzer from GitHub
delta        0.19.2     A syntax-highlighter for git and diff output
diffnav      0.11.0     A git diff pager based on delta but with a file tree.
doctl        1.163.0    A command line tool for DigitalOcean services
dolt         2.1.10     Dolt is a SQL database that you can fork, clone, branch, merge, push and pull just like a git repository.
gh           2.95.0     Official GitHub CLI
ghorg        1.11.11    Quickly clone an entire org/users repositories into one directory - Supports GitHub, GitLab, Bitbucket, and more
gibo         3.0.22     gibo (short for .gitignore boilerplates) is a shell script to help you easily access .gitignore boilerplates from github.com/github/gitignore
gig          0.8.3      Generate .gitignore files from your terminal (mostly) offline!
git          2.54.0     A free and open source distributed version control system.
git-absorb   0.9.0      git commit --fixup, but automatic
git-ai       1.6.3      An open source git extension that tracks AI-generated code in your repositories, linking every AI-written line to the agent, model, and transcripts that generated it.
git-annex    10.20260624 Manage files with git, without comitting them.
git-branchless 0.11.1     A suite of tools that enhances Git in several ways.
git-bug      0.10.1     Distributed, offline-first bug tracker embedded in git, with bridges
git-chglog   0.15.4     Changelog generator implemented in Go (Golang)
git-cliff    2.13.1     A highly customizable Changelog Generator that follows Conventional Commit specifications
git-credential-oauth 0.17.2     A Git credential helper that securely authenticates to GitHub, GitLab and BitBucket using OAuth.
git-crypt    0.7.0      Store encrypted data in git repository
git-filter-repo 2.47.0     git filter-branch replacement
git-flow-next 1.1.0      A modern reimplementation of git-flow in Go that offers greater flexibility while maintaining backward compatibility with the original git-flow and git-flow-avh.
git-interactive-rebase-tool 2.4.1      An improved sequence editor for Git
git-istage   0.3.193    A better git add -p
git-lfs      3.7.1      Git extension for versioning large files.
git-machete  3.43.0     Probably the sharpest git repository organizer & rebase/merge workflow automation tool you've ever seen
git-pkgs     0.16.2     Git subcommand for tracking package dependencies across git history. Analyzes your repository to show when dependencies were added, modified, or removed, who made those changes, and why.
git-quick-stats 2.11.0     Git quick statistics is a simple and efficient way to access various statistics in git repository.
git-sizer    1.5.0      Compute various size metrics for a Git repository, flagging those that might cause problems.
git-tfs      0.34.0     A Git/TFS bridge, similar to git-svn.
git-town     23.0.3     Git plugin that adds Git commands that make collaborative software development more efficient and safe.
git-up       2.4.0      A nicer 'git pull'
git-with-openssh 2.54.0     A free and open source distributed version control system.
git-xargs    0.1.16     A CLI tool for making updates across multiple Github repositories with a single command
git-xet      0.2.1      Git LFS custom transfer agent that implements upload and download of files using the Xet protocol.
gitea        1.26.4     A painless self-hosted Git service
gitignore    0.2018.07.25 Fetches .gitignore file templates from gitignore.io and writes them to standard output.
gitkube      0.3.0      Build and deploy docker images to Kubernetes using git push
gitlab-release-cli 0.24.0     Interacts with GitLab's Releases API through the command line and through GitLab CI/CD's configuration file, .gitlab-ci.yml
gitlab-runner 19.1.1     Run your jobs and send the results back to GitLab
gitleaks     8.30.1     SAST tool for detecting and preventing hardcoded secrets like passwords, api keys, and tokens in git repos
gitomatic    0.2        A tool to monitor git repositories and automatically pull & push changes.
gitoxide     0.55.0     An idiomatic, lean, fast & safe pure Rust implementation of Git
gitql        0.43.0     SQL like lanuage to perform queries on .git files
gitsign      0.16.1     Keyless Git signing with Sigstore!
gitui        0.28.1     Terminal client for Git
gitversion   6.6.2      Easy Semantic Versioning for projects using Git.
gk           3.1.68     GitKraken CLI
glab         1.105.0    GitLab CLI
glitter      1.6.6      Git tooling of the future
gmic-cli     3.7.6      A full-featured open-source framework for digital image processing.
gogs         0.13.3     A painless self-hosted Git service
gut          0.3.3      An easy to use Git client for the command line
helm-chart-releaser 1.8.1      Tool designed to help GitHub repos self-host their own chart repos by adding Helm chart artifacts to GitHub Releases named for the chart version and then creating an index.yaml file for those releases that can be hosted on GitHub Pages (or elsewhere!).
hk           1.48.0     A git hook manager and project linting tool with an emphasis on performance.
hub          2.14.2     An extension to command-line git that helps with everyday GitHub tasks without ever leaving the terminal.
jj           0.42.0     Jujutsu is a Git-compatible DVCS that is both simple and powerful
lab          0.25.1     An extension to command-line git that helps with everyday GitLab tasks.
lefthook     2.1.9      Fast and powerful Git hooks manager for any type of projects
legit        1.2.0      Complementary command-line interface for Git.
mergiraf     0.17.0     A syntax-aware git merge driver
mingit       2.54.0     Minimal Git for Windows (MinGit) is a lightweight distribution intended primarily for application-integration scenarios (such as integrated development environments and graphical visualization tools) where full interactive console capabilities, including colorization and pagination, are not required.
mingit-busybox 2.54.0     Minimal Git for Windows (MinGit) is a reduced sized package designed to support application integration (like integrated development environments, graph visualizers, etc.) where full console support (colorization, pagniation, etc.) is not needed.(BusyBox-backed MinGit, experimental, smaller version of MinGit)
mob          5.4.2      Smooth git handover with mob
multi-gitter 0.63.1     A command-line tool for updating multiple repositories with a single command.
nero-aac     1.5.4.0    Nero Digital AAC Encoder (Command Line Tools)
no-mistakes  1.30.1     A local Git proxy that AI-validates your code before push, forwarding only clean commits upstream.
paket        10.3.1     Dependency manager for .NET with support for NuGet and Git repositories
prs          0.5.1      Secure, fast & convenient password manager CLI with GPG & git sync
psgithub     0.15.240   PowerShell module to manage GitHub through its REST API.
ptags        0.3.5      A parallel universal-ctags wrapper for git repositories.
sapling      0.2.20250521.115337.25ed6ac4 Sapling SCM is a cross-platform, highly scalable, Git-compatible source control system.
sleuthkit    4.15.0     A collection of command line digital forensics tools that allow you to investigate volume and file system data.
smimesign    0.2.0      An S/MIME signing utility compatible with Git that allows developers to sign their commits and tags using X.509 certificates
stgit        2.6.1      Manage Git commits as a stack of patches
tea          0.14.2     Official command-line tool to interact with Gitea servers
tuicr        0.18.0     A code review TUI with vim keybindings. Export to GitHub or clipboard.
vmr          0.7.5      A general version manager for thousands of SDKs with TUI inspired by lazygit.
worktrunk    0.62.0     A CLI for Git worktree management, designed for parallel AI agent workflows.
y-cruncher   0.8.7.9547b Scalable multi-threaded benchmark calculating multiple mathematical constants to trillions of digits.
zizmor       1.26.1     A static analysis tool for GitHub Actions that finds common security issues such as template injection, credential leakage, excessive permissions, and impostor commits.

共 91 个结果


────────────────────────────────────────────────
## §3.2
────────────────────────────────────────────────
命令: hit s python
输出（原样）:
[2m2026-06-27T14:21:02.713773Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

名称           版本         描述
austin       4.0.0      Python frame stack sampler for CPython
hatch        1.17.0     A modern, extensible Python project manager.
mihomo       1.19.27    A simple Python Pydantic model for Honkai: Star Rail parsed data from the Mihomo API.
nim          2.2.10     A statically typed compiled systems programming language, which combines successful concepts from mature languages like Python, Ada and Modula.
oh-my-pi     16.2.2     AI Coding agent for the terminal — hash-anchored edits, optimized tool harness, LSP, Python, browser, subagents, and more (fork of pi).
pipx         1.15.0     Install and run Python applications in isolated environments
poetry       2.4.1      Dependency Management for Python
pyenv        3.1.1      Simple python version management tool for switching between multiple versions of Python.
pyflow       0.3.1      A modern Python installation and dependency manager
pyoxidizer   0.24.0     A modern Python application packaging and distribution tool
pypy2        7.3.23     A fast, compliant alternative implementation of the Python language.
pypy3        7.3.23     A fast, compliant alternative implementation of the Python language.
pyrefly      1.1.1      A fast type checker and language server for Python.
python       3.14.6     A programming language that lets you work quickly and integrate systems more effectively.
rcc          17.18.0    Allows you to create, manage, and distribute Python-based self-contained automation packages.
ruff         0.15.20    An extremely fast Python linter and code formatter, written in Rust.
rye          0.44.0     A comprehensive project and package management solution for Python
ty           0.0.55     An extremely fast Python type checker and language server, written in Rust.
upm          1.0        Universal package manager for Python, Node.js, Ruby and Emacs Lisp.
uv           0.11.25    An extremely fast Python package installer and resolver, written in Rust.
winpython    3.14.5.0   Free, open-source and portable Python distribution for Windows
yubikey-manager-cli 5.9.1      Python library and command line tool for configuring any YubiKey over all USB interfaces.

共 22 个结果


────────────────────────────────────────────────
## §3.3
────────────────────────────────────────────────
命令: hit search GIT
输出（原样）:
[2m2026-06-27T14:21:02.904923Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

名称           版本         描述
act          0.2.89     Local Github actions runner.
actionlint   1.7.12     Static checker for GitHub Actions workflow files
argocd       3.4.4      CLI for ArgoCD - declarative, GitOps continuous delivery tool for Kubernetes
argocd-autopilot 0.4.20     A tool which offers an opinionated way of installing Argo-CD and managing GitOps repositories.
bat          0.26.1     A cat(1) clone with syntax highlighting and Git integration
bfg          1.15.0     BFG Repo-Cleaner removes large or troublesome blobs like git-filter-branch does, but faster
bit          1.1.2      Modernized git CLI
carvel-vendir 0.46.0     Carvel vendir is a tool that makes it easy to vendor portions of git repos, github releases, helm charts, docker image contents, etc. declaratively
claude-code  2.1.195    An agentic coding tool built by Anthropic that lives in your terminal, understands your codebase, and helps you code faster by executing routine tasks, explaining complex code, and handling git workflows.
cocogitto    7.0.0      The Conventional Commits toolbox.
codeowners-validator 0.7.4      The GitHub Codeowners file validator
codeql       2.25.6     Source code security analyzer from GitHub
delta        0.19.2     A syntax-highlighter for git and diff output
diffnav      0.11.0     A git diff pager based on delta but with a file tree.
doctl        1.163.0    A command line tool for DigitalOcean services
dolt         2.1.10     Dolt is a SQL database that you can fork, clone, branch, merge, push and pull just like a git repository.
gh           2.95.0     Official GitHub CLI
ghorg        1.11.11    Quickly clone an entire org/users repositories into one directory - Supports GitHub, GitLab, Bitbucket, and more
gibo         3.0.22     gibo (short for .gitignore boilerplates) is a shell script to help you easily access .gitignore boilerplates from github.com/github/gitignore
gig          0.8.3      Generate .gitignore files from your terminal (mostly) offline!
git          2.54.0     A free and open source distributed version control system.
git-absorb   0.9.0      git commit --fixup, but automatic
git-ai       1.6.3      An open source git extension that tracks AI-generated code in your repositories, linking every AI-written line to the agent, model, and transcripts that generated it.
git-annex    10.20260624 Manage files with git, without comitting them.
git-branchless 0.11.1     A suite of tools that enhances Git in several ways.
git-bug      0.10.1     Distributed, offline-first bug tracker embedded in git, with bridges
git-chglog   0.15.4     Changelog generator implemented in Go (Golang)
git-cliff    2.13.1     A highly customizable Changelog Generator that follows Conventional Commit specifications
git-credential-oauth 0.17.2     A Git credential helper that securely authenticates to GitHub, GitLab and BitBucket using OAuth.
git-crypt    0.7.0      Store encrypted data in git repository
git-filter-repo 2.47.0     git filter-branch replacement
git-flow-next 1.1.0      A modern reimplementation of git-flow in Go that offers greater flexibility while maintaining backward compatibility with the original git-flow and git-flow-avh.
git-interactive-rebase-tool 2.4.1      An improved sequence editor for Git
git-istage   0.3.193    A better git add -p
git-lfs      3.7.1      Git extension for versioning large files.
git-machete  3.43.0     Probably the sharpest git repository organizer & rebase/merge workflow automation tool you've ever seen
git-pkgs     0.16.2     Git subcommand for tracking package dependencies across git history. Analyzes your repository to show when dependencies were added, modified, or removed, who made those changes, and why.
git-quick-stats 2.11.0     Git quick statistics is a simple and efficient way to access various statistics in git repository.
git-sizer    1.5.0      Compute various size metrics for a Git repository, flagging those that might cause problems.
git-tfs      0.34.0     A Git/TFS bridge, similar to git-svn.
git-town     23.0.3     Git plugin that adds Git commands that make collaborative software development more efficient and safe.
git-up       2.4.0      A nicer 'git pull'
git-with-openssh 2.54.0     A free and open source distributed version control system.
git-xargs    0.1.16     A CLI tool for making updates across multiple Github repositories with a single command
git-xet      0.2.1      Git LFS custom transfer agent that implements upload and download of files using the Xet protocol.
gitea        1.26.4     A painless self-hosted Git service
gitignore    0.2018.07.25 Fetches .gitignore file templates from gitignore.io and writes them to standard output.
gitkube      0.3.0      Build and deploy docker images to Kubernetes using git push
gitlab-release-cli 0.24.0     Interacts with GitLab's Releases API through the command line and through GitLab CI/CD's configuration file, .gitlab-ci.yml
gitlab-runner 19.1.1     Run your jobs and send the results back to GitLab
gitleaks     8.30.1     SAST tool for detecting and preventing hardcoded secrets like passwords, api keys, and tokens in git repos
gitomatic    0.2        A tool to monitor git repositories and automatically pull & push changes.
gitoxide     0.55.0     An idiomatic, lean, fast & safe pure Rust implementation of Git
gitql        0.43.0     SQL like lanuage to perform queries on .git files
gitsign      0.16.1     Keyless Git signing with Sigstore!
gitui        0.28.1     Terminal client for Git
gitversion   6.6.2      Easy Semantic Versioning for projects using Git.
gk           3.1.68     GitKraken CLI
glab         1.105.0    GitLab CLI
glitter      1.6.6      Git tooling of the future
gmic-cli     3.7.6      A full-featured open-source framework for digital image processing.
gogs         0.13.3     A painless self-hosted Git service
gut          0.3.3      An easy to use Git client for the command line
helm-chart-releaser 1.8.1      Tool designed to help GitHub repos self-host their own chart repos by adding Helm chart artifacts to GitHub Releases named for the chart version and then creating an index.yaml file for those releases that can be hosted on GitHub Pages (or elsewhere!).
hk           1.48.0     A git hook manager and project linting tool with an emphasis on performance.
hub          2.14.2     An extension to command-line git that helps with everyday GitHub tasks without ever leaving the terminal.
jj           0.42.0     Jujutsu is a Git-compatible DVCS that is both simple and powerful
lab          0.25.1     An extension to command-line git that helps with everyday GitLab tasks.
lefthook     2.1.9      Fast and powerful Git hooks manager for any type of projects
legit        1.2.0      Complementary command-line interface for Git.
mergiraf     0.17.0     A syntax-aware git merge driver
mingit       2.54.0     Minimal Git for Windows (MinGit) is a lightweight distribution intended primarily for application-integration scenarios (such as integrated development environments and graphical visualization tools) where full interactive console capabilities, including colorization and pagination, are not required.
mingit-busybox 2.54.0     Minimal Git for Windows (MinGit) is a reduced sized package designed to support application integration (like integrated development environments, graph visualizers, etc.) where full console support (colorization, pagniation, etc.) is not needed.(BusyBox-backed MinGit, experimental, smaller version of MinGit)
mob          5.4.2      Smooth git handover with mob
multi-gitter 0.63.1     A command-line tool for updating multiple repositories with a single command.
nero-aac     1.5.4.0    Nero Digital AAC Encoder (Command Line Tools)
no-mistakes  1.30.1     A local Git proxy that AI-validates your code before push, forwarding only clean commits upstream.
paket        10.3.1     Dependency manager for .NET with support for NuGet and Git repositories
prs          0.5.1      Secure, fast & convenient password manager CLI with GPG & git sync
psgithub     0.15.240   PowerShell module to manage GitHub through its REST API.
ptags        0.3.5      A parallel universal-ctags wrapper for git repositories.
sapling      0.2.20250521.115337.25ed6ac4 Sapling SCM is a cross-platform, highly scalable, Git-compatible source control system.
sleuthkit    4.15.0     A collection of command line digital forensics tools that allow you to investigate volume and file system data.
smimesign    0.2.0      An S/MIME signing utility compatible with Git that allows developers to sign their commits and tags using X.509 certificates
stgit        2.6.1      Manage Git commits as a stack of patches
tea          0.14.2     Official command-line tool to interact with Gitea servers
tuicr        0.18.0     A code review TUI with vim keybindings. Export to GitHub or clipboard.
vmr          0.7.5      A general version manager for thousands of SDKs with TUI inspired by lazygit.
worktrunk    0.62.0     A CLI for Git worktree management, designed for parallel AI agent workflows.
y-cruncher   0.8.7.9547b Scalable multi-threaded benchmark calculating multiple mathematical constants to trillions of digits.
zizmor       1.26.1     A static analysis tool for GitHub Actions that finds common security issues such as template injection, credential leakage, excessive permissions, and impostor commits.

共 91 个结果


────────────────────────────────────────────────
## §3.4
────────────────────────────────────────────────
命令: hit search git --bucket main
输出（原样）:
[2m2026-06-27T14:21:03.110520Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

名称           版本         描述
act          0.2.89     Local Github actions runner.
actionlint   1.7.12     Static checker for GitHub Actions workflow files
argocd       3.4.4      CLI for ArgoCD - declarative, GitOps continuous delivery tool for Kubernetes
argocd-autopilot 0.4.20     A tool which offers an opinionated way of installing Argo-CD and managing GitOps repositories.
bat          0.26.1     A cat(1) clone with syntax highlighting and Git integration
bfg          1.15.0     BFG Repo-Cleaner removes large or troublesome blobs like git-filter-branch does, but faster
bit          1.1.2      Modernized git CLI
carvel-vendir 0.46.0     Carvel vendir is a tool that makes it easy to vendor portions of git repos, github releases, helm charts, docker image contents, etc. declaratively
claude-code  2.1.195    An agentic coding tool built by Anthropic that lives in your terminal, understands your codebase, and helps you code faster by executing routine tasks, explaining complex code, and handling git workflows.
cocogitto    7.0.0      The Conventional Commits toolbox.
codeowners-validator 0.7.4      The GitHub Codeowners file validator
codeql       2.25.6     Source code security analyzer from GitHub
delta        0.19.2     A syntax-highlighter for git and diff output
diffnav      0.11.0     A git diff pager based on delta but with a file tree.
doctl        1.163.0    A command line tool for DigitalOcean services
dolt         2.1.10     Dolt is a SQL database that you can fork, clone, branch, merge, push and pull just like a git repository.
gh           2.95.0     Official GitHub CLI
ghorg        1.11.11    Quickly clone an entire org/users repositories into one directory - Supports GitHub, GitLab, Bitbucket, and more
gibo         3.0.22     gibo (short for .gitignore boilerplates) is a shell script to help you easily access .gitignore boilerplates from github.com/github/gitignore
gig          0.8.3      Generate .gitignore files from your terminal (mostly) offline!
git          2.54.0     A free and open source distributed version control system.
git-absorb   0.9.0      git commit --fixup, but automatic
git-ai       1.6.3      An open source git extension that tracks AI-generated code in your repositories, linking every AI-written line to the agent, model, and transcripts that generated it.
git-annex    10.20260624 Manage files with git, without comitting them.
git-branchless 0.11.1     A suite of tools that enhances Git in several ways.
git-bug      0.10.1     Distributed, offline-first bug tracker embedded in git, with bridges
git-chglog   0.15.4     Changelog generator implemented in Go (Golang)
git-cliff    2.13.1     A highly customizable Changelog Generator that follows Conventional Commit specifications
git-credential-oauth 0.17.2     A Git credential helper that securely authenticates to GitHub, GitLab and BitBucket using OAuth.
git-crypt    0.7.0      Store encrypted data in git repository
git-filter-repo 2.47.0     git filter-branch replacement
git-flow-next 1.1.0      A modern reimplementation of git-flow in Go that offers greater flexibility while maintaining backward compatibility with the original git-flow and git-flow-avh.
git-interactive-rebase-tool 2.4.1      An improved sequence editor for Git
git-istage   0.3.193    A better git add -p
git-lfs      3.7.1      Git extension for versioning large files.
git-machete  3.43.0     Probably the sharpest git repository organizer & rebase/merge workflow automation tool you've ever seen
git-pkgs     0.16.2     Git subcommand for tracking package dependencies across git history. Analyzes your repository to show when dependencies were added, modified, or removed, who made those changes, and why.
git-quick-stats 2.11.0     Git quick statistics is a simple and efficient way to access various statistics in git repository.
git-sizer    1.5.0      Compute various size metrics for a Git repository, flagging those that might cause problems.
git-tfs      0.34.0     A Git/TFS bridge, similar to git-svn.
git-town     23.0.3     Git plugin that adds Git commands that make collaborative software development more efficient and safe.
git-up       2.4.0      A nicer 'git pull'
git-with-openssh 2.54.0     A free and open source distributed version control system.
git-xargs    0.1.16     A CLI tool for making updates across multiple Github repositories with a single command
git-xet      0.2.1      Git LFS custom transfer agent that implements upload and download of files using the Xet protocol.
gitea        1.26.4     A painless self-hosted Git service
gitignore    0.2018.07.25 Fetches .gitignore file templates from gitignore.io and writes them to standard output.
gitkube      0.3.0      Build and deploy docker images to Kubernetes using git push
gitlab-release-cli 0.24.0     Interacts with GitLab's Releases API through the command line and through GitLab CI/CD's configuration file, .gitlab-ci.yml
gitlab-runner 19.1.1     Run your jobs and send the results back to GitLab
gitleaks     8.30.1     SAST tool for detecting and preventing hardcoded secrets like passwords, api keys, and tokens in git repos
gitomatic    0.2        A tool to monitor git repositories and automatically pull & push changes.
gitoxide     0.55.0     An idiomatic, lean, fast & safe pure Rust implementation of Git
gitql        0.43.0     SQL like lanuage to perform queries on .git files
gitsign      0.16.1     Keyless Git signing with Sigstore!
gitui        0.28.1     Terminal client for Git
gitversion   6.6.2      Easy Semantic Versioning for projects using Git.
gk           3.1.68     GitKraken CLI
glab         1.105.0    GitLab CLI
glitter      1.6.6      Git tooling of the future
gmic-cli     3.7.6      A full-featured open-source framework for digital image processing.
gogs         0.13.3     A painless self-hosted Git service
gut          0.3.3      An easy to use Git client for the command line
helm-chart-releaser 1.8.1      Tool designed to help GitHub repos self-host their own chart repos by adding Helm chart artifacts to GitHub Releases named for the chart version and then creating an index.yaml file for those releases that can be hosted on GitHub Pages (or elsewhere!).
hk           1.48.0     A git hook manager and project linting tool with an emphasis on performance.
hub          2.14.2     An extension to command-line git that helps with everyday GitHub tasks without ever leaving the terminal.
jj           0.42.0     Jujutsu is a Git-compatible DVCS that is both simple and powerful
lab          0.25.1     An extension to command-line git that helps with everyday GitLab tasks.
lefthook     2.1.9      Fast and powerful Git hooks manager for any type of projects
legit        1.2.0      Complementary command-line interface for Git.
mergiraf     0.17.0     A syntax-aware git merge driver
mingit       2.54.0     Minimal Git for Windows (MinGit) is a lightweight distribution intended primarily for application-integration scenarios (such as integrated development environments and graphical visualization tools) where full interactive console capabilities, including colorization and pagination, are not required.
mingit-busybox 2.54.0     Minimal Git for Windows (MinGit) is a reduced sized package designed to support application integration (like integrated development environments, graph visualizers, etc.) where full console support (colorization, pagniation, etc.) is not needed.(BusyBox-backed MinGit, experimental, smaller version of MinGit)
mob          5.4.2      Smooth git handover with mob
multi-gitter 0.63.1     A command-line tool for updating multiple repositories with a single command.
nero-aac     1.5.4.0    Nero Digital AAC Encoder (Command Line Tools)
no-mistakes  1.30.1     A local Git proxy that AI-validates your code before push, forwarding only clean commits upstream.
paket        10.3.1     Dependency manager for .NET with support for NuGet and Git repositories
prs          0.5.1      Secure, fast & convenient password manager CLI with GPG & git sync
psgithub     0.15.240   PowerShell module to manage GitHub through its REST API.
ptags        0.3.5      A parallel universal-ctags wrapper for git repositories.
sapling      0.2.20250521.115337.25ed6ac4 Sapling SCM is a cross-platform, highly scalable, Git-compatible source control system.
sleuthkit    4.15.0     A collection of command line digital forensics tools that allow you to investigate volume and file system data.
smimesign    0.2.0      An S/MIME signing utility compatible with Git that allows developers to sign their commits and tags using X.509 certificates
stgit        2.6.1      Manage Git commits as a stack of patches
tea          0.14.2     Official command-line tool to interact with Gitea servers
tuicr        0.18.0     A code review TUI with vim keybindings. Export to GitHub or clipboard.
vmr          0.7.5      A general version manager for thousands of SDKs with TUI inspired by lazygit.
worktrunk    0.62.0     A CLI for Git worktree management, designed for parallel AI agent workflows.
y-cruncher   0.8.7.9547b Scalable multi-threaded benchmark calculating multiple mathematical constants to trillions of digits.
zizmor       1.26.1     A static analysis tool for GitHub Actions that finds common security issues such as template injection, credential leakage, excessive permissions, and impostor commits.

共 91 个结果


────────────────────────────────────────────────
## §3.5
────────────────────────────────────────────────
命令: hit search nonexistent_xyz
输出（原样）:
[2m2026-06-27T14:21:03.312405Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

未找到匹配 'nonexistent_xyz' 的软件


────────────────────────────────────────────────
## §4.1
────────────────────────────────────────────────
命令: hit info git
输出（原样）:
[2m2026-06-27T14:21:03.493306Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......



────────────────────────────────────────────────
## §4.2
────────────────────────────────────────────────
命令: hit info git --bucket main
输出（原样）:
[2m2026-06-27T14:21:03.653130Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......



────────────────────────────────────────────────
## §4.3
────────────────────────────────────────────────
命令: hit info nonexistent
输出（原样）:
[2m2026-06-27T14:21:03.892817Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......



────────────────────────────────────────────────
## §4.4
────────────────────────────────────────────────
命令: hit info curl
输出（原样）:
[2m2026-06-27T14:21:04.119497Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......



────────────────────────────────────────────────
## §5.1
────────────────────────────────────────────────
命令: hit install curl
输出（原样）:
安装 curl ...
[2m2026-06-27T14:21:04.282282Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......



────────────────────────────────────────────────
## §5.2
────────────────────────────────────────────────
命令: hit i jq
输出（原样）:
安装 jq ...
[2m2026-06-27T14:21:04.485442Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......



────────────────────────────────────────────────
## §5.3
────────────────────────────────────────────────
命令: hit install curl
输出（原样）:
安装 curl ...
[2m2026-06-27T14:21:04.682799Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......



────────────────────────────────────────────────
## §5.4
────────────────────────────────────────────────
命令: hit install curl --force
输出（原样）:
安装 curl ...
[2m2026-06-27T14:21:04.894032Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......



────────────────────────────────────────────────
## §5.5
────────────────────────────────────────────────
命令: hit install main/git
输出（原样）:
安装 git ...
[2m2026-06-27T14:21:05.097224Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......



────────────────────────────────────────────────
## §5.6
────────────────────────────────────────────────
命令: hit install nonexistent_pkg
输出（原样）:
安装 nonexistent_pkg ...
[2m2026-06-27T14:21:05.301276Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......



────────────────────────────────────────────────
## §5.7
────────────────────────────────────────────────
命令: hit install jq --arch 64bit
输出（原样）:
安装 jq ...
[2m2026-06-27T14:21:05.434224Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......



────────────────────────────────────────────────
## §6.1
────────────────────────────────────────────────
命令: hit list
输出（原样）:
没有已安装的软件


────────────────────────────────────────────────
## §6.2
────────────────────────────────────────────────
命令: hit ls
输出（原样）:
没有已安装的软件


────────────────────────────────────────────────
## §6.3
────────────────────────────────────────────────
命令: hit list curl
输出（原样）:
没有匹配 'curl' 的已安装软件


────────────────────────────────────────────────
## §6.4
────────────────────────────────────────────────
命令: hit list nonexistent
输出（原样）:
没有匹配 'nonexistent' 的已安装软件


────────────────────────────────────────────────
## §7.1.1
────────────────────────────────────────────────
命令: hit install python@3.11.0
输出（原样）:

────────────────────────────────────────────────
## §7.1.2
────────────────────────────────────────────────
命令: hit install python@3.12.0
输出（原样）:

────────────────────────────────────────────────
## §7.1.3
────────────────────────────────────────────────
命令: hit reset python 3.11.0
输出（原样）:

────────────────────────────────────────────────
## §7.1.4
────────────────────────────────────────────────
命令: hit reset python 9.9.9
输出（原样）:

────────────────────────────────────────────────
## §7.2.1
────────────────────────────────────────────────
命令: hit hold curl
输出（原样）:

────────────────────────────────────────────────
## §7.2.2
────────────────────────────────────────────────
命令: hit hold curl
输出（原样）:

────────────────────────────────────────────────
## §7.2.3
────────────────────────────────────────────────
命令: hit update --all
输出（原样）:
刷新 正在更新 Bucket...
正在更新 bucket 'main'...
正在克隆 bucket 'main'...
bucket 'main'：检出文件中...
  ✔ main
✔ Bucket 更新完成（1/1）

bucket 'main' 克隆完成
[2m2026-06-27T14:21:36.597040Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

没有已安装的软件


────────────────────────────────────────────────
## §7.2.4
────────────────────────────────────────────────
命令: hit unhold curl
输出（原样）:

────────────────────────────────────────────────
## §7.2.5
────────────────────────────────────────────────
命令: hit unhold curl
输出（原样）:

────────────────────────────────────────────────
## §7.2.6
────────────────────────────────────────────────
命令: hit hold nonexistent
输出（原样）:

────────────────────────────────────────────────
## §8.1
────────────────────────────────────────────────
命令: hit update
输出（原样）:
刷新 正在更新 Bucket...
正在更新 bucket 'main'...
正在克隆 bucket 'main'...
bucket 'main'：检出文件中...
bucket 'main' 克隆完成
  ✔ main
✔ Bucket 更新完成（1/1）

[2m2026-06-27T14:22:02.779815Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

没有已安装的软件


────────────────────────────────────────────────
## §8.2
────────────────────────────────────────────────
命令: hit update --all
输出（原样）:
刷新 正在更新 Bucket...
正在更新 bucket 'main'...
正在克隆 bucket 'main'...
  ✘ main 失败: Bucket 'main' 错误：克隆失败：An IO error occurred when talking to the server
✔ Bucket 更新完成（0/1）

没有已安装的软件


────────────────────────────────────────────────
## §8.3
────────────────────────────────────────────────
命令: hit update curl
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
刷新 正在更新 Bucket...
✔ Bucket 更新完成（0/0）

  curl 未安装，跳过
所有软件已是最新版本


────────────────────────────────────────────────
## §8.4
────────────────────────────────────────────────
命令: hit update nonexistent
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
刷新 正在更新 Bucket...
✔ Bucket 更新完成（0/0）

  nonexistent 未安装，跳过
所有软件已是最新版本


────────────────────────────────────────────────
## §8.5
────────────────────────────────────────────────
命令: hit update --force
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
刷新 正在更新 Bucket...
✔ Bucket 更新完成（0/0）

没有已安装的软件


────────────────────────────────────────────────
## §9.1
────────────────────────────────────────────────
命令: hit uninstall jq
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。


────────────────────────────────────────────────
## §9.2
────────────────────────────────────────────────
命令: hit rm curl --purge
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。


────────────────────────────────────────────────
## §9.3
────────────────────────────────────────────────
命令: hit uninstall nonexistent
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。


────────────────────────────────────────────────
## §9.4
────────────────────────────────────────────────
命令: hit uninstall
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。


────────────────────────────────────────────────
## §9.5
────────────────────────────────────────────────
命令: hit uninstall jq curl
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。


────────────────────────────────────────────────
## §10-pre
────────────────────────────────────────────────
命令: hit install curl
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
安装 curl ...


────────────────────────────────────────────────
## §10.1
────────────────────────────────────────────────
命令: hit cache list
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
缓存为空


────────────────────────────────────────────────
## §10.2
────────────────────────────────────────────────
命令: hit cache dir
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
C:\Users\Violet\Downloads\test\hit\cache


────────────────────────────────────────────────
## §10.3
────────────────────────────────────────────────
命令: hit cache clean
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
没有可清理的缓存文件


────────────────────────────────────────────────
## §10.4
────────────────────────────────────────────────
命令: hit cache clean curl
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
没有可清理的缓存文件


────────────────────────────────────────────────
## §10.5
────────────────────────────────────────────────
命令: hit cache list
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
缓存为空


────────────────────────────────────────────────
## §11.1
────────────────────────────────────────────────
命令: hit cleanup python
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
没有需要清理的内容


────────────────────────────────────────────────
## §11.2
────────────────────────────────────────────────
命令: hit cleanup --all
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
没有需要清理的内容


────────────────────────────────────────────────
## §11.3
────────────────────────────────────────────────
命令: hit cleanup --cache
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。


────────────────────────────────────────────────
## §11.4
────────────────────────────────────────────────
命令: hit cleanup
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
没有需要清理的内容


────────────────────────────────────────────────
## §12.1.1
────────────────────────────────────────────────
命令: hit which curl
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。


────────────────────────────────────────────────
## §12.1.2
────────────────────────────────────────────────
命令: hit which nonexistent
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。


────────────────────────────────────────────────
## §12.2.1
────────────────────────────────────────────────
命令: hit prefix
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
C:\Users\Violet\Downloads\test\hit


────────────────────────────────────────────────
## §12.2.2
────────────────────────────────────────────────
命令: hit prefix curl
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。


────────────────────────────────────────────────
## §12.2.3
────────────────────────────────────────────────
命令: hit prefix nonexistent
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。


────────────────────────────────────────────────
## §12.3.1
────────────────────────────────────────────────
命令: hit home git
备注: 跳过：会打开浏览器
输出: [跳过]

────────────────────────────────────────────────
## §12.3.2
────────────────────────────────────────────────
命令: hit home nonexistent
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。


────────────────────────────────────────────────
## §13.1
────────────────────────────────────────────────
命令: hit config list
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
proxy                          (未设置)
mirror                         (未设置)
aria2_enabled                  false
no_junction                    false
root_path                      (未设置)
auto_cleanup_days              30
health_check_interval_days     7


────────────────────────────────────────────────
## §13.2
────────────────────────────────────────────────
命令: hit config set proxy http://127.0.0.1:7890
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
✔ 配置 'proxy' 已更新为 'http://127.0.0.1:7890'


────────────────────────────────────────────────
## §13.3
────────────────────────────────────────────────
命令: hit config set aria2_enabled true
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
✔ 配置 'aria2_enabled' 已更新为 'true'


────────────────────────────────────────────────
## §13.4
────────────────────────────────────────────────
命令: hit config set aria2_enabled yes
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
✔ 配置 'aria2_enabled' 已更新为 'yes'


────────────────────────────────────────────────
## §13.5
────────────────────────────────────────────────
命令: hit config set aria2_enabled maybe
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。


────────────────────────────────────────────────
## §13.6
────────────────────────────────────────────────
命令: hit config set auto_cleanup_days 60
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
✔ 配置 'auto_cleanup_days' 已更新为 '60'


────────────────────────────────────────────────
## §13.7
────────────────────────────────────────────────
命令: hit config set auto_cleanup_days abc
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。


────────────────────────────────────────────────
## §13.8
────────────────────────────────────────────────
命令: hit config set unknown_key value
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。


────────────────────────────────────────────────
## §13.9
────────────────────────────────────────────────
命令: hit config set proxy ""
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
✔ 配置 'proxy' 已更新为 ''


────────────────────────────────────────────────
## §13.10
────────────────────────────────────────────────
命令: hit config list
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
proxy                          (未设置)
mirror                         (未设置)
aria2_enabled                  false
no_junction                    false
root_path                      (未设置)
auto_cleanup_days              30
health_check_interval_days     7


────────────────────────────────────────────────
## §14.1
────────────────────────────────────────────────
命令: hit doctor
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
✔ 系统健康，无问题


────────────────────────────────────────────────
## §14.2
────────────────────────────────────────────────
命令: hit doctor
备注: 跳过：需手动删除 current junction 后测试
输出: [跳过]

────────────────────────────────────────────────
## §14.3
────────────────────────────────────────────────
命令: hit doctor --fix
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
✔ 系统健康，无问题


────────────────────────────────────────────────
## §14.4
────────────────────────────────────────────────
命令: hit doctor --fix
备注: 跳过：需手动创建损坏 .shim 后测试
输出: [跳过]

────────────────────────────────────────────────
## §16.1
────────────────────────────────────────────────
命令: hit status
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
Hit 0.1.0

  已安装软件:    0
  Bucket 数量:   0
  可用软件总数:  0
  缓存文件:      0 (0 B)
  根目录:        C:\Users\Violet\Downloads\test\hit


────────────────────────────────────────────────
## §16.2
────────────────────────────────────────────────
命令: hit st
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
Hit 0.1.0

  已安装软件:    0
  Bucket 数量:   0
  可用软件总数:  0
  缓存文件:      0 (0 B)
  根目录:        C:\Users\Violet\Downloads\test\hit


────────────────────────────────────────────────
## §17-i
────────────────────────────────────────────────
命令: hit i nonexistent_alias_test
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
安装 nonexistent_alias_test ...


────────────────────────────────────────────────
## §17-s
────────────────────────────────────────────────
命令: hit s nonexistent_alias_test
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
未找到匹配 'nonexistent_alias_test' 的软件


────────────────────────────────────────────────
## §17-u
────────────────────────────────────────────────
命令: hit u nonexistent
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
刷新 正在更新 Bucket...
✔ Bucket 更新完成（0/0）

  nonexistent 未安装，跳过
所有软件已是最新版本


────────────────────────────────────────────────
## §17-rm
────────────────────────────────────────────────
命令: hit rm nonexistent
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。


────────────────────────────────────────────────
## §17-ls
────────────────────────────────────────────────
命令: hit ls
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
没有已安装的软件


────────────────────────────────────────────────
## §17-st
────────────────────────────────────────────────
命令: hit st
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
Hit 0.1.0

  已安装软件:    0
  Bucket 数量:   0
  可用软件总数:  0
  缓存文件:      0 (0 B)
  根目录:        C:\Users\Violet\Downloads\test\hit


────────────────────────────────────────────────
## §17-b
────────────────────────────────────────────────
命令: hit b ls
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
没有已添加的 Bucket


────────────────────────────────────────────────
## §17-c
────────────────────────────────────────────────
命令: hit c
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
没有需要清理的内容


────────────────────────────────────────────────
## §17-r
────────────────────────────────────────────────
命令: hit r nonexistent 1.0.0
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。


────────────────────────────────────────────────
## §18.1
────────────────────────────────────────────────
命令: hit -v list
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
没有已安装的软件


────────────────────────────────────────────────
## §18.2
────────────────────────────────────────────────
命令: hit -vv list
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
没有已安装的软件


────────────────────────────────────────────────
## §18.3
────────────────────────────────────────────────
命令: hit -vvv list
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
没有已安装的软件


────────────────────────────────────────────────
## §19.1
────────────────────────────────────────────────
命令: hit
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。


────────────────────────────────────────────────
## §19.2
────────────────────────────────────────────────
命令: hit wrongcmd
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。


────────────────────────────────────────────────
## §19.3
────────────────────────────────────────────────
命令: hit install
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。


────────────────────────────────────────────────
## §19.4
────────────────────────────────────────────────
命令: hit bucket add main
备注: 跳过：需断网环境
输出: [跳过]

────────────────────────────────────────────────
## §19.5
────────────────────────────────────────────────
命令: hit list
输出（原样）:

  _    _       _
 | |  | |     | |
 | |__| | ___ | | ___  _   _  __ _  ___
 |  __  |/ _ \| |/ _ \| | | |/ _` |/ _ \
 | |  | | (_) | | (_) | |_| | (_| |  __/
 |_|  |_|\___/|_|\___/ \__, |\__,_|\___|
                         __/ |
                        |___/

首次使用 Hit？

  1) 快速开始 — 添加官方 Bucket（main, extras, versions）
  2) 自定义 — 手动选择要添加的 Bucket
  3) 跳过

请选择 [1/2/3]: 无效选择，已跳过。

已跳过。你可以稍后使用 hit bucket add 添加 Bucket。
没有已安装的软件

