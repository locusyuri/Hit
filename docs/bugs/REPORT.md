# Hit 实测报告（自动批量执行）
> 生成时间: 2026-06-28 16:12:57
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
root_path                      C:\Users\Violet\Downloads\test\hit
auto_cleanup_days              30
health_check_interval_days     7


────────────────────────────────────────────────
## §2.1.1
────────────────────────────────────────────────
命令: hit bucket add main
输出（原样）:
错误: Bucket 'main' 已存在


────────────────────────────────────────────────
## §2.1.4
────────────────────────────────────────────────
命令: hit bucket add main
输出（原样）:
错误: Bucket 'main' 已存在


────────────────────────────────────────────────
## §2.1.5
────────────────────────────────────────────────
命令: hit bucket add myrepo https://github.com/user/repo.git
输出（原样）:
添加 正在添加 bucket 'myrepo'...
正在克隆 bucket 'myrepo'...
错误: Bucket 'myrepo' 错误：克隆失败：An IO error occurred when talking to the server


────────────────────────────────────────────────
## §2.1.6
────────────────────────────────────────────────
命令: hit bucket add unknownbucket
输出（原样）:
错误: 未知 bucket 'unknownbucket'，请提供 Git 仓库 URL
  示例：hit bucket add unknownbucket https://github.com/<user>/<bucket>.git


────────────────────────────────────────────────
## §2.2.1
────────────────────────────────────────────────
命令: hit bucket list
输出（原样）:
名称                  Manifest    描述
extras                2321        
main                  1593        
versions              592         

共 3 个 Bucket


────────────────────────────────────────────────
## §2.2.2
────────────────────────────────────────────────
命令: hit b ls
输出（原样）:
名称                  Manifest    描述
extras                2321        
main                  1593        
versions              592         

共 3 个 Bucket


────────────────────────────────────────────────
## §2.3.1
────────────────────────────────────────────────
命令: hit bucket update
输出（原样）:
正在更新 bucket 'extras'...
正在克隆 bucket 'extras'...
bucket 'extras'：检出文件中...
bucket 'extras' 克隆完成
  ✔ extras
正在更新 bucket 'main'...
正在克隆 bucket 'main'...
bucket 'main'：检出文件中...
bucket 'main' 克隆完成
  ✔ main
正在更新 bucket 'versions'...
正在克隆 bucket 'versions'...
bucket 'versions'：检出文件中...
bucket 'versions' 克隆完成
  ✔ versions

✔ Bucket 更新完成（3/3）


────────────────────────────────────────────────
## §2.3.2
────────────────────────────────────────────────
命令: hit bucket update main
输出（原样）:
正在更新 bucket 'main'...
正在克隆 bucket 'main'...
bucket 'main'：检出文件中...
bucket 'main' 克隆完成
  ✔ main

✔ Bucket 更新完成（1/1）


────────────────────────────────────────────────
## §2.3.3
────────────────────────────────────────────────
命令: hit bucket update nonexistent
输出（原样）:
没有可更新的 Bucket


────────────────────────────────────────────────
## §2.4.1
────────────────────────────────────────────────
命令: hit bucket remove myrepo
输出（原样）:
错误: Bucket 'myrepo' 不存在


────────────────────────────────────────────────
## §2.4.2
────────────────────────────────────────────────
命令: hit bucket rm main
输出（原样）:
移除 正在移除 bucket 'main'...
✔ bucket 'main' 已移除


────────────────────────────────────────────────
## §2.4.3
────────────────────────────────────────────────
命令: hit bucket remove nonexistent
输出（原样）:
错误: Bucket 'nonexistent' 不存在


────────────────────────────────────────────────
## §2.4-restore
────────────────────────────────────────────────
命令: hit bucket add main
输出（原样）:
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
名称           版本         描述
cocogitto    7.0.0      The Conventional Commits toolbox.
deepgit      26.1.003   Git Archaeology Tool.
deepgit402   4.0.2      Git Archaeology Tool.
digital      0.31       A digital logic designer and circuit simulator
git          2.54.0     A free and open source distributed version control system.
git-absorb   0.9.0      git commit --fixup, but automatic
git-ai       1.6.3      An open source git extension that tracks AI-generated code in your repositories, linking every AI-written line to the agent, model, and transcripts that generated it.
git-aliases  0.3.8      A PowerShell module that provides partial Git aliases from Oh My Zsh's git plugin.
git-annex    10.20260624 Manage files with git, without comitting them.
git-branchless 0.11.1     A suite of tools that enhances Git in several ways.
git-bug      0.10.1     Distributed, offline-first bug tracker embedded in git, with bridges
git-chglog   0.15.4     Changelog generator implemented in Go (Golang)
git-cliff    2.13.1     A highly customizable Changelog Generator that follows Conventional Commit specifications
git-cola     4.18.2     A highly-caffeinated, powerful, and intuitive graphical user interface for Git
git-credential-manager 2.8.0      Secure Git credential helper
git-credential-oauth 0.17.2     A Git credential helper that securely authenticates to GitHub, GitLab and BitBucket using OAuth.
git-crypt    0.7.0      Store encrypted data in git repository
git-filter-repo 2.47.0     git filter-branch replacement
git-flow-next 1.1.0      A modern reimplementation of git-flow in Go that offers greater flexibility while maintaining backward compatibility with the original git-flow and git-flow-avh.
git-flow-next-pre 1.1.0      A modern reimplementation of git-flow in Go that offers greater flexibility while maintaining backward compatibility with the original git-flow and git-flow-avh. (Pre-release)
git-graph    0.7.0      Command line tool to show clear git graphs arranged for your branching model
git-interactive-rebase-tool 2.4.1      An improved sequence editor for Git
git-istage   0.3.193    A better git add -p
git-lfs      3.7.1      Git extension for versioning large files.
git-machete  3.43.0     Probably the sharpest git repository organizer & rebase/merge workflow automation tool you've ever seen
git-pkgs     0.16.2     Git subcommand for tracking package dependencies across git history. Analyzes your repository to show when dependencies were added, modified, or removed, who made those changes, and why.
git-quick-stats 2.11.0     Git quick statistics is a simple and efficient way to access various statistics in git repository.
git-sizer    1.5.0      Compute various size metrics for a Git repository, flagging those that might cause problems.
git-tfs      0.34.0     A Git/TFS bridge, similar to git-svn.
git-tower    12.4.565   An easy to use GUI client for Git with powerful features.
git-town     23.0.3     Git plugin that adds Git commands that make collaborative software development more efficient and safe.
git-up       2.4.0      A nicer 'git pull'
git-with-openssh 2.54.0     A free and open source distributed version control system.
git-without-openssh 2.54.0     A free and open source distributed version control system.
git-xargs    0.1.16     A CLI tool for making updates across multiple Github repositories with a single command
git-xet      0.2.1      Git LFS custom transfer agent that implements upload and download of files using the Xet protocol.
git19        1.9.5-preview20150319 A free and open source distributed version control system designed to handle everything from small to very large projects with speed and efficiency.
gitahead     2.7.1      The elegant git gui for dev teams
gitas        0.0.8      A terminal UI to switch between multiple Git accounts and run git commands as any of them
gitbutler    0.20.4     A Git client for simultaneous branches on top of your existing workflow.
gitbutler-nightly 0.5.1705   A Git client for simultaneous branches on top of your existing workflow.
gitea        1.26.4     A painless self-hosted Git service
gitea-beta   1.26.4     Git with a cup of tea, painless self-hosted git service. (beta channel)
gitextensions 7.0.1.86   A graphical user interface for Git that allows you to control Git without using the commandline.
github       3.6.1      Extend your GitHub workflow beyond your browser.
github-beta  3.6.1-beta2 Extend your GitHub workflow beyond your browser.
gitify       6.20.0     GitHub notifications on menu bar
gitignore    0.2018.07.25 Fetches .gitignore file templates from gitignore.io and writes them to standard output.
gitkraken    12.2.1     A Git client which helps you track and manage changes to your code.
gitkube      0.3.0      Build and deploy docker images to Kubernetes using git push
gitlab-release-cli 0.24.0     Interacts with GitLab's Releases API through the command line and through GitLab CI/CD's configuration file, .gitlab-ci.yml
gitlab-runner 19.1.1     Run your jobs and send the results back to GitLab
gitleaks     8.30.1     SAST tool for detecting and preventing hardcoded secrets like passwords, api keys, and tokens in git repos
gitnuro      1.5.0      A FOSS Git multiplatform client for newbies and pros
gitomatic    0.2        A tool to monitor git repositories and automatically pull & push changes.
gitoxide     0.55.0     An idiomatic, lean, fast & safe pure Rust implementation of Git
gitql        0.43.0     SQL like lanuage to perform queries on .git files
gitsign      0.16.1     Keyless Git signing with Sigstore!
gittyup      1.4.0      A continuation of GitAhead client, a graphical Git client designed to help you understand and manage your source code history.
gitu         0.42.0     A TUI Git client inspired by Magit
gitui        0.28.1     Terminal client for Git
gitversion   6.6.2      Easy Semantic Versioning for projects using Git.
gitversion-beta 6.7.0      Easy Semantic Versioning for projects using Git.
lazy-posh-git 0.2.0      PowerShell proxy command around Set-Location to defer import of posh-git module until one changes working directory to the root of a git directory.
lazygit      0.62.2     A simple terminal UI for git commands
legit        1.2.0      Complementary command-line interface for Git.
logitech-omm 2.6.1749   OnBoard Memory Manager (OMM) is a utility for pro gamers to quickly view, customize, and fine-tune the onboard memory of a compatible Logitech G mouse.
metrogit     0.4.0      Git visualization tool that's more than just git.
mingit       2.54.0     Minimal Git for Windows (MinGit) is a lightweight distribution intended primarily for application-integration scenarios (such as integrated development environments and graphical visualization tools) where full interactive console capabilities, including colorization and pagination, are not required.
mingit-busybox 2.54.0     Minimal Git for Windows (MinGit) is a reduced sized package designed to support application integration (like integrated development environments, graph visualizers, etc.) where full console support (colorization, pagniation, etc.) is not needed.(BusyBox-backed MinGit, experimental, smaller version of MinGit)
mpv-git      20260610   Video player based on MPlayer/mplayer2 (builds by shinchiro)
multi-gitter 0.63.1     A command-line tool for updating multiple repositories with a single command.
posh-git     1.1.0      A PowerShell module which provides Git/PowerShell integration.
posh-git-beta 1.1.0      A PowerShell module which provides Git/PowerShell integration. (beta version)
posh-git-nightly bbc5ac3800 A PowerShell module which provides Git/PowerShell integration. (nightly version)
prismlauncher-git 12.0.0-2451-475ab8a An open source Minecraft launcher with the ability to manage multiple instances, accounts and mods. Focused on user freedom and free redistributability.
psgithub     0.15.240   PowerShell module to manage GitHub through its REST API.
smartgit     26.1.038   A graphical Git client with support for SVN and Pull Requests for GitHub and Bitbucket.
sourcegit    2026.13    Open-source GUI client for git users.
stgit        2.6.1      Manage Git commits as a stack of patches
ungit        1.5.30     The easiest way to use git. On any platform. Anywhere.
wslgit       1.3.1      A small executable that forwards all arguments to git running inside Bash on Windows/Windows Subsystem for Linux (WSL)

共 82 个结果


────────────────────────────────────────────────
## §3.2
────────────────────────────────────────────────
命令: hit s python
输出（原样）:
名称           版本         描述
python       3.14.6     A programming language that lets you work quickly and integrate systems more effectively.
python-alpha 3.15.0b3   A programming language that lets you work quickly and integrate systems more effectively.
python-beta  3.15.0b3   A programming language that lets you work quickly and integrate systems more effectively.
python-pre   3.15.0b3   A programming language that lets you work quickly and integrate systems more effectively.
python-rc    3.14.5rc1  A programming language that lets you work quickly and integrate systems more effectively.
python27     2.7.18     A programming language that lets you work quickly and integrate systems more effectively.
python310    3.10.11    A programming language that lets you work quickly and integrate systems more effectively.
python311    3.11.9     A programming language that lets you work quickly and integrate systems more effectively.
python312    3.12.10    A programming language that lets you work quickly and integrate systems more effectively.
python313    3.13.14    A programming language that lets you work quickly and integrate systems more effectively.
python314    3.14.6     A programming language that lets you work quickly and integrate systems more effectively.
python35     3.5.4      A programming language that lets you work quickly and integrate systems more effectively.
python36     3.6.8      A programming language that lets you work quickly and integrate systems more effectively.
python37     3.7.9      A programming language that lets you work quickly and integrate systems more effectively.
python38     3.8.10     A programming language that lets you work quickly and integrate systems more effectively.
python39     3.9.13     A programming language that lets you work quickly and integrate systems more effectively.
winpython    3.14.5.0   Free, open-source and portable Python distribution for Windows
winpython37  3.7.7.1    Free, open-source and portable Python distribution for Windows (3.7.x)
winpython3741 3.7.4.1    Free, open-source and portable Python distribution for Windows
winpython37cod 3.7.7.1    Free, open-source and portable Python distribution for Windows (3.7.x w/ VS Code)
winpython37ps2 3.7.6.0    Free, open-source and portable Python distribution for Windows (3.7.x w/ PySide2)
winpython38  3.8.9.0    Free, open-source and portable Python distribution for Windows (3.8.x)
winpython38cod 3.8.7.0    Free, open-source and portable Python distribution for Windows (3.8.x w/ VS Code)
winpython38ps2 3.8.1.0    Free, open-source and portable Python distribution for Windows (3.8.x w/ PySide2)

共 24 个结果


────────────────────────────────────────────────
## §3.3
────────────────────────────────────────────────
命令: hit search GIT
输出（原样）:
名称           版本         描述
cocogitto    7.0.0      The Conventional Commits toolbox.
deepgit      26.1.003   Git Archaeology Tool.
deepgit402   4.0.2      Git Archaeology Tool.
digital      0.31       A digital logic designer and circuit simulator
git          2.54.0     A free and open source distributed version control system.
git-absorb   0.9.0      git commit --fixup, but automatic
git-ai       1.6.3      An open source git extension that tracks AI-generated code in your repositories, linking every AI-written line to the agent, model, and transcripts that generated it.
git-aliases  0.3.8      A PowerShell module that provides partial Git aliases from Oh My Zsh's git plugin.
git-annex    10.20260624 Manage files with git, without comitting them.
git-branchless 0.11.1     A suite of tools that enhances Git in several ways.
git-bug      0.10.1     Distributed, offline-first bug tracker embedded in git, with bridges
git-chglog   0.15.4     Changelog generator implemented in Go (Golang)
git-cliff    2.13.1     A highly customizable Changelog Generator that follows Conventional Commit specifications
git-cola     4.18.2     A highly-caffeinated, powerful, and intuitive graphical user interface for Git
git-credential-manager 2.8.0      Secure Git credential helper
git-credential-oauth 0.17.2     A Git credential helper that securely authenticates to GitHub, GitLab and BitBucket using OAuth.
git-crypt    0.7.0      Store encrypted data in git repository
git-filter-repo 2.47.0     git filter-branch replacement
git-flow-next 1.1.0      A modern reimplementation of git-flow in Go that offers greater flexibility while maintaining backward compatibility with the original git-flow and git-flow-avh.
git-flow-next-pre 1.1.0      A modern reimplementation of git-flow in Go that offers greater flexibility while maintaining backward compatibility with the original git-flow and git-flow-avh. (Pre-release)
git-graph    0.7.0      Command line tool to show clear git graphs arranged for your branching model
git-interactive-rebase-tool 2.4.1      An improved sequence editor for Git
git-istage   0.3.193    A better git add -p
git-lfs      3.7.1      Git extension for versioning large files.
git-machete  3.43.0     Probably the sharpest git repository organizer & rebase/merge workflow automation tool you've ever seen
git-pkgs     0.16.2     Git subcommand for tracking package dependencies across git history. Analyzes your repository to show when dependencies were added, modified, or removed, who made those changes, and why.
git-quick-stats 2.11.0     Git quick statistics is a simple and efficient way to access various statistics in git repository.
git-sizer    1.5.0      Compute various size metrics for a Git repository, flagging those that might cause problems.
git-tfs      0.34.0     A Git/TFS bridge, similar to git-svn.
git-tower    12.4.565   An easy to use GUI client for Git with powerful features.
git-town     23.0.3     Git plugin that adds Git commands that make collaborative software development more efficient and safe.
git-up       2.4.0      A nicer 'git pull'
git-with-openssh 2.54.0     A free and open source distributed version control system.
git-without-openssh 2.54.0     A free and open source distributed version control system.
git-xargs    0.1.16     A CLI tool for making updates across multiple Github repositories with a single command
git-xet      0.2.1      Git LFS custom transfer agent that implements upload and download of files using the Xet protocol.
git19        1.9.5-preview20150319 A free and open source distributed version control system designed to handle everything from small to very large projects with speed and efficiency.
gitahead     2.7.1      The elegant git gui for dev teams
gitas        0.0.8      A terminal UI to switch between multiple Git accounts and run git commands as any of them
gitbutler    0.20.4     A Git client for simultaneous branches on top of your existing workflow.
gitbutler-nightly 0.5.1705   A Git client for simultaneous branches on top of your existing workflow.
gitea        1.26.4     A painless self-hosted Git service
gitea-beta   1.26.4     Git with a cup of tea, painless self-hosted git service. (beta channel)
gitextensions 7.0.1.86   A graphical user interface for Git that allows you to control Git without using the commandline.
github       3.6.1      Extend your GitHub workflow beyond your browser.
github-beta  3.6.1-beta2 Extend your GitHub workflow beyond your browser.
gitify       6.20.0     GitHub notifications on menu bar
gitignore    0.2018.07.25 Fetches .gitignore file templates from gitignore.io and writes them to standard output.
gitkraken    12.2.1     A Git client which helps you track and manage changes to your code.
gitkube      0.3.0      Build and deploy docker images to Kubernetes using git push
gitlab-release-cli 0.24.0     Interacts with GitLab's Releases API through the command line and through GitLab CI/CD's configuration file, .gitlab-ci.yml
gitlab-runner 19.1.1     Run your jobs and send the results back to GitLab
gitleaks     8.30.1     SAST tool for detecting and preventing hardcoded secrets like passwords, api keys, and tokens in git repos
gitnuro      1.5.0      A FOSS Git multiplatform client for newbies and pros
gitomatic    0.2        A tool to monitor git repositories and automatically pull & push changes.
gitoxide     0.55.0     An idiomatic, lean, fast & safe pure Rust implementation of Git
gitql        0.43.0     SQL like lanuage to perform queries on .git files
gitsign      0.16.1     Keyless Git signing with Sigstore!
gittyup      1.4.0      A continuation of GitAhead client, a graphical Git client designed to help you understand and manage your source code history.
gitu         0.42.0     A TUI Git client inspired by Magit
gitui        0.28.1     Terminal client for Git
gitversion   6.6.2      Easy Semantic Versioning for projects using Git.
gitversion-beta 6.7.0      Easy Semantic Versioning for projects using Git.
lazy-posh-git 0.2.0      PowerShell proxy command around Set-Location to defer import of posh-git module until one changes working directory to the root of a git directory.
lazygit      0.62.2     A simple terminal UI for git commands
legit        1.2.0      Complementary command-line interface for Git.
logitech-omm 2.6.1749   OnBoard Memory Manager (OMM) is a utility for pro gamers to quickly view, customize, and fine-tune the onboard memory of a compatible Logitech G mouse.
metrogit     0.4.0      Git visualization tool that's more than just git.
mingit       2.54.0     Minimal Git for Windows (MinGit) is a lightweight distribution intended primarily for application-integration scenarios (such as integrated development environments and graphical visualization tools) where full interactive console capabilities, including colorization and pagination, are not required.
mingit-busybox 2.54.0     Minimal Git for Windows (MinGit) is a reduced sized package designed to support application integration (like integrated development environments, graph visualizers, etc.) where full console support (colorization, pagniation, etc.) is not needed.(BusyBox-backed MinGit, experimental, smaller version of MinGit)
mpv-git      20260610   Video player based on MPlayer/mplayer2 (builds by shinchiro)
multi-gitter 0.63.1     A command-line tool for updating multiple repositories with a single command.
posh-git     1.1.0      A PowerShell module which provides Git/PowerShell integration.
posh-git-beta 1.1.0      A PowerShell module which provides Git/PowerShell integration. (beta version)
posh-git-nightly bbc5ac3800 A PowerShell module which provides Git/PowerShell integration. (nightly version)
prismlauncher-git 12.0.0-2451-475ab8a An open source Minecraft launcher with the ability to manage multiple instances, accounts and mods. Focused on user freedom and free redistributability.
psgithub     0.15.240   PowerShell module to manage GitHub through its REST API.
smartgit     26.1.038   A graphical Git client with support for SVN and Pull Requests for GitHub and Bitbucket.
sourcegit    2026.13    Open-source GUI client for git users.
stgit        2.6.1      Manage Git commits as a stack of patches
ungit        1.5.30     The easiest way to use git. On any platform. Anywhere.
wslgit       1.3.1      A small executable that forwards all arguments to git running inside Bash on Windows/Windows Subsystem for Linux (WSL)

共 82 个结果


────────────────────────────────────────────────
## §3.4
────────────────────────────────────────────────
命令: hit search git --bucket main
输出（原样）:
名称           版本         描述
cocogitto    7.0.0      The Conventional Commits toolbox.
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
legit        1.2.0      Complementary command-line interface for Git.
mingit       2.54.0     Minimal Git for Windows (MinGit) is a lightweight distribution intended primarily for application-integration scenarios (such as integrated development environments and graphical visualization tools) where full interactive console capabilities, including colorization and pagination, are not required.
mingit-busybox 2.54.0     Minimal Git for Windows (MinGit) is a reduced sized package designed to support application integration (like integrated development environments, graph visualizers, etc.) where full console support (colorization, pagniation, etc.) is not needed.(BusyBox-backed MinGit, experimental, smaller version of MinGit)
multi-gitter 0.63.1     A command-line tool for updating multiple repositories with a single command.
psgithub     0.15.240   PowerShell module to manage GitHub through its REST API.
stgit        2.6.1      Manage Git commits as a stack of patches

共 44 个结果


────────────────────────────────────────────────
## §3.5
────────────────────────────────────────────────
命令: hit search nonexistent_xyz
输出（原样）:
未找到匹配 'nonexistent_xyz' 的软件


────────────────────────────────────────────────
## §4.1
────────────────────────────────────────────────
命令: hit info git
输出（原样）:
名称:        git
版本:        2.54.0
描述:        A free and open source distributed version control system.
主页:      https://gitforwindows.org
许可证:      Detailed { identifier: "GPL-2.0-only", url: Some("https://github.com/git-for-windows/git/blob/HEAD/COPYING") }
架构:    64bit, arm64
依赖:      无
Bucket:      main


────────────────────────────────────────────────
## §4.2
────────────────────────────────────────────────
命令: hit info git --bucket main
输出（原样）:
名称:        git
版本:        2.54.0
描述:        A free and open source distributed version control system.
主页:      https://gitforwindows.org
许可证:      Detailed { identifier: "GPL-2.0-only", url: Some("https://github.com/git-for-windows/git/blob/HEAD/COPYING") }
架构:    64bit, arm64
依赖:      无
Bucket:      main


────────────────────────────────────────────────
## §4.3
────────────────────────────────────────────────
命令: hit info nonexistent
输出（原样）:
错误: 未找到软件 'nonexistent'


────────────────────────────────────────────────
## §4.4
────────────────────────────────────────────────
命令: hit info curl
输出（原样）:
名称:        curl
版本:        8.21.0_1
描述:        Command line tool and library for transferring data with URLs
主页:      https://curl.se/
许可证:      Identifier("MIT")
架构:    64bit, arm64
依赖:      无
Bucket:      main


────────────────────────────────────────────────
## §5.1
────────────────────────────────────────────────
命令: hit install curl
输出（原样）:
安装 curl ...
▶ [解析] curl...
✔ [解析] curl 完成
▶ [下载] curl...
✔ [下载] curl 完成
▶ [校验] curl...
✔ [校验] curl 完成
解压 curl (curl#8.21.0_1#39c2972.xz)
▶ [解压] curl...
✔ [解压] curl 完成
▶ [同步] curl...
▶ [提交] curl...
✔ [提交] curl 完成
✔ [同步] curl 完成
✔ curl 8.21.0_1 安装完成（1）


────────────────────────────────────────────────
## §5.2
────────────────────────────────────────────────
命令: hit i jq
输出（原样）:
安装 jq ...
▶ [解析] jq...
✔ [解析] jq 完成
▶ [下载] jq...
错误: 下载失败（https://github.com/jqlang/jq/releases/download/jq-1.8.2/jq-windows-amd64.exe#/jq.exe）：读取响应失败: error decoding response body


────────────────────────────────────────────────
## §5.3
────────────────────────────────────────────────
命令: hit install curl
输出（原样）:
错误: 'curl' 已安装，如需重装请使用 --force


────────────────────────────────────────────────
## §5.4
────────────────────────────────────────────────
命令: hit install curl --force
输出（原样）:
安装 curl ...
▶ [解析] curl...
✔ [解析] curl 完成
▶ [下载] curl...
✔ [下载] curl 完成
▶ [校验] curl...
✔ [校验] curl 完成
解压 curl (curl#8.21.0_1#39c2972.xz)
▶ [解压] curl...
✔ [解压] curl 完成
▶ [同步] curl...
▶ [提交] curl...
[2m2026-06-28T08:14:32.796753Z[0m [33m WARN[0m 事务回滚 [3mapp[0m[2m=[0mcurl
错误: IO 错误：创建 Junction: C:\Users\Violet\Downloads\test\hit\apps\curl\current -> C:\Users\Violet\Downloads\test\hit\apps\curl\8.21.0_1：Cannot create a file when that file already exists. (os error 183)
  原因: Cannot create a file when that file already exists. (os error 183)


────────────────────────────────────────────────
## §5.5
────────────────────────────────────────────────
命令: hit install main/git
输出（原样）:
安装 git ...
▶ [解析] git...
✔ [解析] git 完成
▶ [下载] git...
错误: 下载失败（https://github.com/git-for-windows/git/releases/download/v2.54.0.windows.1/PortableGit-2.54.0-64-bit.7z.exe#/dl.7z）：请求失败: error sending request for url (https://github.com/git-for-windows/git/releases/download/v2.54.0.windows.1/PortableGit-2.54.0-64-bit.7z.exe#/dl.7z)


────────────────────────────────────────────────
## §5.6
────────────────────────────────────────────────
命令: hit install nonexistent_pkg
输出（原样）:
安装 nonexistent_pkg ...
错误: 未找到软件 'nonexistent_pkg'


────────────────────────────────────────────────
## §5.7
────────────────────────────────────────────────
命令: hit install jq --arch 64bit
输出（原样）:
安装 jq ...
▶ [解析] jq...
✔ [解析] jq 完成
▶ [下载] jq...
错误: 下载失败（https://github.com/jqlang/jq/releases/download/jq-1.8.2/jq-windows-amd64.exe#/jq.exe）：请求失败: error sending request for url (https://github.com/jqlang/jq/releases/download/jq-1.8.2/jq-windows-amd64.exe#/jq.exe)


────────────────────────────────────────────────
## §6.1
────────────────────────────────────────────────
命令: hit list
输出（原样）:
名称           版本         架构       Bucket     安装时间
curl         8.21.0_1   64bit    main       2026-06-28T08:13:55Z

共 1 个软件


────────────────────────────────────────────────
## §6.2
────────────────────────────────────────────────
命令: hit ls
输出（原样）:
名称           版本         架构       Bucket     安装时间
curl         8.21.0_1   64bit    main       2026-06-28T08:13:55Z

共 1 个软件


────────────────────────────────────────────────
## §6.3
────────────────────────────────────────────────
命令: hit list curl
输出（原样）:
名称           版本         架构       Bucket     安装时间
curl         8.21.0_1   64bit    main       2026-06-28T08:13:55Z

共 1 个软件


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
错误: 版本约束暂不支持（'python@3.11.0' 中的 '@3.11.0' 部分）


────────────────────────────────────────────────
## §7.1.2
────────────────────────────────────────────────
命令: hit install python@3.12.0
输出（原样）:
错误: 版本约束暂不支持（'python@3.12.0' 中的 '@3.12.0' 部分）


────────────────────────────────────────────────
## §7.1.3
────────────────────────────────────────────────
命令: hit reset python 3.11.0
输出（原样）:
错误: 版本 '3.11.0' 不存在（python）


────────────────────────────────────────────────
## §7.1.4
────────────────────────────────────────────────
命令: hit reset python 9.9.9
输出（原样）:
错误: 版本 '9.9.9' 不存在（python）


────────────────────────────────────────────────
## §7.2.1
────────────────────────────────────────────────
命令: hit hold curl
输出（原样）:
🔒 'curl' 已锁定（update 时将跳过升级）


────────────────────────────────────────────────
## §7.2.2
────────────────────────────────────────────────
命令: hit hold curl
输出（原样）:
⏭ 'curl' 已经是锁定状态


────────────────────────────────────────────────
## §7.2.3
────────────────────────────────────────────────
命令: hit update --all
输出（原样）:
刷新 正在更新 Bucket...
正在更新 bucket 'extras'...
正在克隆 bucket 'extras'...
  ✘ extras 失败: Bucket 'extras' 错误：克隆失败：An IO error occurred when talking to the server
正在更新 bucket 'main'...
正在克隆 bucket 'main'...
  ✘ main 失败: Bucket 'main' 错误：克隆失败：An IO error occurred when talking to the server
正在更新 bucket 'versions'...
正在克隆 bucket 'versions'...
  ✘ versions 失败: Bucket 'versions' 错误：克隆失败：An IO error occurred when talking to the server
✔ Bucket 更新完成（0/3）

所有软件已是最新版本


────────────────────────────────────────────────
## §7.2.4
────────────────────────────────────────────────
命令: hit unhold curl
输出（原样）:
🔓 'curl' 已解除锁定


────────────────────────────────────────────────
## §7.2.5
────────────────────────────────────────────────
命令: hit unhold curl
输出（原样）:
⏭ 'curl' 未处于锁定状态


────────────────────────────────────────────────
## §7.2.6
────────────────────────────────────────────────
命令: hit hold nonexistent
输出（原样）:
错误: 'nonexistent' 未安装


────────────────────────────────────────────────
## §8.1
────────────────────────────────────────────────
命令: hit update
输出（原样）:
刷新 正在更新 Bucket...
✔ Bucket 更新完成（0/0）

所有软件已是最新版本


────────────────────────────────────────────────
## §8.2
────────────────────────────────────────────────
命令: hit update --all
输出（原样）:
刷新 正在更新 Bucket...
✔ Bucket 更新完成（0/0）

所有软件已是最新版本


────────────────────────────────────────────────
## §8.3
────────────────────────────────────────────────
命令: hit update curl
输出（原样）:
刷新 正在更新 Bucket...
✔ Bucket 更新完成（0/0）

所有软件已是最新版本


────────────────────────────────────────────────
## §8.4
────────────────────────────────────────────────
命令: hit update nonexistent
输出（原样）:
刷新 正在更新 Bucket...
✔ Bucket 更新完成（0/0）

  nonexistent 未安装，跳过
所有软件已是最新版本


────────────────────────────────────────────────
## §8.5
────────────────────────────────────────────────
命令: hit update --force
输出（原样）:
刷新 正在更新 Bucket...
✔ Bucket 更新完成（0/0）

所有软件已是最新版本


────────────────────────────────────────────────
## §9.1
────────────────────────────────────────────────
命令: hit uninstall jq
输出（原样）:
错误: 'jq' 未安装


────────────────────────────────────────────────
## §9.2
────────────────────────────────────────────────
命令: hit rm curl --purge
输出（原样）:
卸载 curl ...
错误: IO 错误：移除 Junction: C:\Users\Violet\Downloads\test\hit\apps\curl\current：The file or directory is not a reparse point. (os error 4390)
  原因: The file or directory is not a reparse point. (os error 4390)


────────────────────────────────────────────────
## §9.3
────────────────────────────────────────────────
命令: hit uninstall nonexistent
输出（原样）:
错误: 'nonexistent' 未安装


────────────────────────────────────────────────
## §9.4
────────────────────────────────────────────────
命令: hit uninstall
输出（原样）:
错误: 至少指定一个要卸载的软件名


────────────────────────────────────────────────
## §9.5
────────────────────────────────────────────────
命令: hit uninstall jq curl
输出（原样）:
错误: 'jq' 未安装


────────────────────────────────────────────────
## §10-pre
────────────────────────────────────────────────
命令: hit install curl
输出（原样）:
错误: 'curl' 已安装，如需重装请使用 --force


────────────────────────────────────────────────
## §10.1
────────────────────────────────────────────────
命令: hit cache list
输出（原样）:
软件                   版本         大小         路径
curl                 8.21.0_1   4.7 MB     C:\Users\Violet\Downloads\test\hit\cache\curl#8.21.0_1#39c2972.xz

共 1 个文件（4.7 MB）


────────────────────────────────────────────────
## §10.2
────────────────────────────────────────────────
命令: hit cache dir
输出（原样）:
C:\Users\Violet\Downloads\test\hit\cache


────────────────────────────────────────────────
## §10.3
────────────────────────────────────────────────
命令: hit cache clean
输出（原样）:
✔ 已清理 1 个缓存文件


────────────────────────────────────────────────
## §10.4
────────────────────────────────────────────────
命令: hit cache clean curl
输出（原样）:
没有可清理的缓存文件


────────────────────────────────────────────────
## §10.5
────────────────────────────────────────────────
命令: hit cache list
输出（原样）:
缓存为空


────────────────────────────────────────────────
## §11.1
────────────────────────────────────────────────
命令: hit cleanup python
输出（原样）:
没有需要清理的内容


────────────────────────────────────────────────
## §11.2
────────────────────────────────────────────────
命令: hit cleanup --all
输出（原样）:
没有需要清理的内容


────────────────────────────────────────────────
## §11.3
────────────────────────────────────────────────
命令: hit cleanup --cache
输出（原样）:

────────────────────────────────────────────────
## §11.4
────────────────────────────────────────────────
命令: hit cleanup
输出（原样）:
没有需要清理的内容


────────────────────────────────────────────────
## §12.1.1
────────────────────────────────────────────────
命令: hit which curl
输出（原样）:
错误: 未找到 'curl' 的 shim 文件


────────────────────────────────────────────────
## §12.1.2
────────────────────────────────────────────────
命令: hit which nonexistent
输出（原样）:
错误: 未找到 'nonexistent' 的 shim 文件


────────────────────────────────────────────────
## §12.2.1
────────────────────────────────────────────────
命令: hit prefix
输出（原样）:
C:\Users\Violet\Downloads\test\hit


────────────────────────────────────────────────
## §12.2.2
────────────────────────────────────────────────
命令: hit prefix curl
输出（原样）:
C:\Users\Violet\Downloads\test\hit\apps\curl


────────────────────────────────────────────────
## §12.2.3
────────────────────────────────────────────────
命令: hit prefix nonexistent
输出（原样）:
错误: 'nonexistent' 未安装


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
错误: 未找到软件 'nonexistent'


────────────────────────────────────────────────
## §13.1
────────────────────────────────────────────────
命令: hit config list
输出（原样）:
proxy                          (未设置)
mirror                         (未设置)
aria2_enabled                  false
no_junction                    false
root_path                      C:\Users\Violet\Downloads\test\hit
auto_cleanup_days              30
health_check_interval_days     7


────────────────────────────────────────────────
## §13.2
────────────────────────────────────────────────
命令: hit config set proxy http://127.0.0.1:7890
输出（原样）:
✔ 配置 'proxy' 已更新为 'http://127.0.0.1:7890'


────────────────────────────────────────────────
## §13.3
────────────────────────────────────────────────
命令: hit config set aria2_enabled true
输出（原样）:
✔ 配置 'aria2_enabled' 已更新为 'true'


────────────────────────────────────────────────
## §13.4
────────────────────────────────────────────────
命令: hit config set aria2_enabled yes
输出（原样）:
✔ 配置 'aria2_enabled' 已更新为 'yes'


────────────────────────────────────────────────
## §13.5
────────────────────────────────────────────────
命令: hit config set aria2_enabled maybe
输出（原样）:
错误: 'maybe' 不是有效的布尔值（应为 true/false/1/0/yes/no）


────────────────────────────────────────────────
## §13.6
────────────────────────────────────────────────
命令: hit config set auto_cleanup_days 60
输出（原样）:
✔ 配置 'auto_cleanup_days' 已更新为 '60'


────────────────────────────────────────────────
## §13.7
────────────────────────────────────────────────
命令: hit config set auto_cleanup_days abc
输出（原样）:
错误: 'abc' 不是有效的数字


────────────────────────────────────────────────
## §13.8
────────────────────────────────────────────────
命令: hit config set unknown_key value
输出（原样）:
错误: 未知配置项 'unknown_key'。支持的配置项：proxy, mirror, aria2_enabled, no_junction, root_path, auto_cleanup_days, health_check_interval_days


────────────────────────────────────────────────
## §13.9
────────────────────────────────────────────────
命令: hit config set proxy ""
输出（原样）:
✔ 配置 'proxy' 已更新为 ''


────────────────────────────────────────────────
## §13.10
────────────────────────────────────────────────
命令: hit config list
输出（原样）:
proxy                          (未设置)
mirror                         (未设置)
aria2_enabled                  true
no_junction                    false
root_path                      C:\Users\Violet\Downloads\test\hit
auto_cleanup_days              60
health_check_interval_days     7


────────────────────────────────────────────────
## §14.1
────────────────────────────────────────────────
命令: hit doctor
输出（原样）:
⚠ 发现 1 个问题：

  ⚠ curl: current 链接损坏 (可修复)

提示 使用 hit doctor --fix 自动修复可修复的问题


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
⚠ 发现 1 个问题：

  ⚠ curl: current 链接损坏 (可修复)

修复 正在修复 1 个问题...
  ✗ curl 修复失败: Cannot create a file when that file already exists. (os error 183)

✔ 已修复 0/1 个问题


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
Hit 0.1.0

  已安装软件:    1
  Bucket 数量:   0
  可用软件总数:  0
  缓存文件:      0 (0 B)
  根目录:        C:\Users\Violet\Downloads\test\hit


────────────────────────────────────────────────
## §16.2
────────────────────────────────────────────────
命令: hit st
输出（原样）:
Hit 0.1.0

  已安装软件:    1
  Bucket 数量:   0
  可用软件总数:  0
  缓存文件:      0 (0 B)
  根目录:        C:\Users\Violet\Downloads\test\hit


────────────────────────────────────────────────
## §17-i
────────────────────────────────────────────────
命令: hit i nonexistent_alias_test
输出（原样）:
安装 nonexistent_alias_test ...
错误: 未找到软件 'nonexistent_alias_test'


────────────────────────────────────────────────
## §17-s
────────────────────────────────────────────────
命令: hit s nonexistent_alias_test
输出（原样）:
未找到匹配 'nonexistent_alias_test' 的软件


────────────────────────────────────────────────
## §17-u
────────────────────────────────────────────────
命令: hit u nonexistent
输出（原样）:
刷新 正在更新 Bucket...
✔ Bucket 更新完成（0/0）

  nonexistent 未安装，跳过
所有软件已是最新版本


────────────────────────────────────────────────
## §17-rm
────────────────────────────────────────────────
命令: hit rm nonexistent
输出（原样）:
错误: 'nonexistent' 未安装


────────────────────────────────────────────────
## §17-ls
────────────────────────────────────────────────
命令: hit ls
输出（原样）:
名称           版本         架构       Bucket     安装时间
curl         8.21.0_1   64bit    main       2026-06-28T08:13:55Z

共 1 个软件


────────────────────────────────────────────────
## §17-st
────────────────────────────────────────────────
命令: hit st
输出（原样）:
Hit 0.1.0

  已安装软件:    1
  Bucket 数量:   0
  可用软件总数:  0
  缓存文件:      0 (0 B)
  根目录:        C:\Users\Violet\Downloads\test\hit


────────────────────────────────────────────────
## §17-b
────────────────────────────────────────────────
命令: hit b ls
输出（原样）:
没有已添加的 Bucket


────────────────────────────────────────────────
## §17-c
────────────────────────────────────────────────
命令: hit c
输出（原样）:
没有需要清理的内容


────────────────────────────────────────────────
## §17-r
────────────────────────────────────────────────
命令: hit r nonexistent 1.0.0
输出（原样）:
错误: 版本 '1.0.0' 不存在（nonexistent）


────────────────────────────────────────────────
## §18.1
────────────────────────────────────────────────
命令: hit -v list
输出（原样）:
名称           版本         架构       Bucket     安装时间
curl         8.21.0_1   64bit    main       2026-06-28T08:13:55Z

共 1 个软件


────────────────────────────────────────────────
## §18.2
────────────────────────────────────────────────
命令: hit -vv list
输出（原样）:
名称           版本         架构       Bucket     安装时间
curl         8.21.0_1   64bit    main       2026-06-28T08:13:55Z

共 1 个软件


────────────────────────────────────────────────
## §18.3
────────────────────────────────────────────────
命令: hit -vvv list
输出（原样）:
名称           版本         架构       Bucket     安装时间
curl         8.21.0_1   64bit    main       2026-06-28T08:13:55Z

共 1 个软件


────────────────────────────────────────────────
## §19.1
────────────────────────────────────────────────
命令: hit
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
## §19.2
────────────────────────────────────────────────
命令: hit wrongcmd
输出（原样）:
error: unrecognized subcommand 'wrongcmd'

  tip: a similar subcommand exists: 'r'

Usage: hit.exe [OPTIONS] <COMMAND>

For more information, try '--help'.


────────────────────────────────────────────────
## §19.3
────────────────────────────────────────────────
命令: hit install
输出（原样）:
错误: 至少指定一个要安装的软件名


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
名称           版本         架构       Bucket     安装时间
curl         8.21.0_1   64bit    main       2026-06-28T08:13:55Z

共 1 个软件

