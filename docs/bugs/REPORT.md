# Hit 实测报告（自动批量执行）
> 生成时间: 2026-06-28 12:31:29
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
  ✔ main

✔ Bucket 更新完成（1/1）
bucket 'main' 克隆完成


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
[2m2026-06-28T04:32:15.057082Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:32:15.061094Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

[2m2026-06-28T04:32:15.061737Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:32:15.068927Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:32:15.069856Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

[2m2026-06-28T04:32:15.095843Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......

名称           版本         描述
act          0.2.89     Local Github actions runner.
actionlint   1.7.12     Static checker for GitHub Actions workflow files
argocd       3.4.4      CLI for ArgoCD - declarative, GitOps continuous delivery tool for Kubernetes
argocd-autopilot 0.4.20     A tool which offers an opinionated way of installing Argo-CD and managing GitOps repositories.
bat          0.26.1     A cat(1) clone with syntax highlighting and Git integration
bfg          1.15.0     BFG Repo-Cleaner removes large or troublesome blobs like git-filter-branch does, but faster
bit          1.1.2      Modernized git CLI
caesium-image-compressor 2.8.5      Caesium is an image compression software that helps you store, send and share digital pictures, supporting JPG, PNG and WebP formats. You can quickly reduce the file size (and resolution, if you want) by preserving the overall quality of the image.
carvel-vendir 0.46.0     Carvel vendir is a tool that makes it easy to vendor portions of git repos, github releases, helm charts, docker image contents, etc. declaratively
cdex         2.24       Open-source Digital Audio CD Extractor
claude-code  2.1.195    An agentic coding tool built by Anthropic that lives in your terminal, understands your codebase, and helps you code faster by executing routine tasks, explaining complex code, and handling git workflows.
cocogitto    7.0.0      The Conventional Commits toolbox.
codeowners-validator 0.7.4      The GitHub Codeowners file validator
codeql       2.25.6     Source code security analyzer from GitHub
deepgit      26.1.003   Git Archaeology Tool.
deepgit402   4.0.2      Git Archaeology Tool.
delta        0.19.2     A syntax-highlighter for git and diff output
devhub       0.102.0    TweetDeck for GitHub
diffnav      0.11.0     A git diff pager based on delta but with a file tree.
digital      0.31       A digital logic designer and circuit simulator
doctl        1.163.0    A command line tool for DigitalOcean services
dolt         2.1.10     Dolt is a SQL database that you can fork, clone, branch, merge, push and pull just like a git repository.
dra          0.10.2     A command line tool to download release assets from GitHub
eget         1.3.4      Easily install prebuilt binaries from GitHub.
fork         2.20.1     A fast and friendly git client for Mac and Windows
gg           0.39.1     A GUI for Jujutsu, a Git-compatible DVCS
gh           2.95.0     Official GitHub CLI
ghorg        1.11.12    Quickly clone an entire org/users repositories into one directory - Supports GitHub, GitLab, Bitbucket, and more
gibo         3.0.22     gibo (short for .gitignore boilerplates) is a shell script to help you easily access .gitignore boilerplates from github.com/github/gitignore
gig          0.8.3      Generate .gitignore files from your terminal (mostly) offline!
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
gk           3.1.68     GitKraken CLI
glab         1.105.0    GitLab CLI
glitter      1.6.6      Git tooling of the future
gmic-cli     3.7.6      A full-featured open-source framework for digital image processing.
gmic-qt      3.7.6      A full-featured open-source framework for digital image processing.
gogs         0.13.3     A painless self-hosted Git service
gogs0        0.11.91    A painless self-hosted Git service
gut          0.3.3      An easy to use Git client for the command line
helm-chart-releaser 1.8.1      Tool designed to help GitHub repos self-host their own chart repos by adding Helm chart artifacts to GitHub Releases named for the chart version and then creating an index.yaml file for those releases that can be hosted on GitHub Pages (or elsewhere!).
hk           1.48.0     A git hook manager and project linting tool with an emphasis on performance.
hub          2.14.2     An extension to command-line git that helps with everyday GitHub tasks without ever leaving the terminal.
ignoreit     3.0.0      Quickly load .gitignore templates
jasper       1.1.2      A flexible and powerful issue reader for GitHub.
jcpicker     6.2        Just Color Picker - Free portable offline colour picker and colour editor for web designers and digital artists.
jj           0.42.0     Jujutsu is a Git-compatible DVCS that is both simple and powerful
kodi         21.3       Open source home theater/media center software and entertainment hub for digital media
kodi-dev     20260626   Open source home theater/media center software and entertainment hub for digital media
kodi-nightly 20250129   Open source home theater/media center software and entertainment hub for digital media
krita        5.3.2.1    A free digital painting application
lab          0.25.1     An extension to command-line git that helps with everyday GitLab tasks.
lazy-posh-git 0.2.0      PowerShell proxy command around Set-Location to defer import of posh-git module until one changes working directory to the root of a git directory.
lazygit      0.62.2     A simple terminal UI for git commands
lefthook     2.1.9      Fast and powerful Git hooks manager for any type of projects
legit        1.2.0      Complementary command-line interface for Git.
lepton       1.10.0     A lean code snippet manager based on GitHub Gist
logisim-evolution 4.1.0      Digital logic design tool and simulator
logitech-omm 2.6.1749   OnBoard Memory Manager (OMM) is a utility for pro gamers to quickly view, customize, and fine-tune the onboard memory of a compatible Logitech G mouse.
mergiraf     0.17.0     A syntax-aware git merge driver
metrogit     0.4.0      Git visualization tool that's more than just git.
mingit       2.54.0     Minimal Git for Windows (MinGit) is a lightweight distribution intended primarily for application-integration scenarios (such as integrated development environments and graphical visualization tools) where full interactive console capabilities, including colorization and pagination, are not required.
mingit-busybox 2.54.0     Minimal Git for Windows (MinGit) is a reduced sized package designed to support application integration (like integrated development environments, graph visualizers, etc.) where full console support (colorization, pagniation, etc.) is not needed.(BusyBox-backed MinGit, experimental, smaller version of MinGit)
mob          5.4.2      Smooth git handover with mob
mpv-git      20260610   Video player based on MPlayer/mplayer2 (builds by shinchiro)
multi-gitter 0.63.1     A command-line tool for updating multiple repositories with a single command.
nero-aac     1.5.4.0    Nero Digital AAC Encoder (Command Line Tools)
no-mistakes  1.31.2     A local Git proxy that AI-validates your code before push, forwarding only clean commits upstream.
onefetch     2.27.1     Git repository summary on terminal
oss-cad-suite-nightly 2026-06-27 Open source digital design and verification tools. Includes tools for RTL synthesis, formal hardware verification, place & route, FPGA programming, and testing with support for HDLs like Verilog, Migen and Amaranth.
paket        10.3.1     Dependency manager for .NET with support for NuGet and Git repositories
posh-git     1.1.0      A PowerShell module which provides Git/PowerShell integration.
posh-git-beta 1.1.0      A PowerShell module which provides Git/PowerShell integration. (beta version)
posh-git-nightly bbc5ac3800 A PowerShell module which provides Git/PowerShell integration. (nightly version)
prismlauncher-git 12.0.0-2451-475ab8a An open source Minecraft launcher with the ability to manage multiple instances, accounts and mods. Focused on user freedom and free redistributability.
prs          0.5.1      Secure, fast & convenient password manager CLI with GPG & git sync
psgithub     0.15.240   PowerShell module to manage GitHub through its REST API.
ptags        0.3.5      A parallel universal-ctags wrapper for git repositories.
reaper       7.75       Digital Audio Workstation
rtlutility   1.0.12     Tool for measuring the Round Trip Latency of your Digital Audio Workstation (DAW) and audio interface
sapling      0.2.20250521.115337.25ed6ac4 Sapling SCM is a cross-platform, highly scalable, Git-compatible source control system.
scoop-sd     0.3        A program to search for scoop packages. Powered by https://scoopsearch.github.io/
sleuthkit    4.15.0     A collection of command line digital forensics tools that allow you to investigate volume and file system data.
smartgit     26.1.038   A graphical Git client with support for SVN and Pull Requests for GitHub and Bitbucket.
smimesign    0.2.0      An S/MIME signing utility compatible with Git that allows developers to sign their commits and tags using X.509 certificates
sophia-script 7.1.6      Sophia Script for Windows is the largest PowerShell module on GitHub for Windows 10 and Windows 11 fine-tuning and automating the routine tasks.
sourcegit    2026.13    Open-source GUI client for git users.
sourcetree   3.4.31     Simple and powerful Git GUI
spotify      1.2.92.148.g882cc571 A digital music service that gives you access to millions of songs.
stgit        2.6.1      Manage Git commits as a stack of patches
sublime-merge 2125       A Git client with snappy UI, three-way merge tool, side-by-side diffs, syntax highlighting, and more.
super-productivity 18.12.0    To-do list & time tracker for programmers and other digital workers with Jira, Github, and Gitlab integration
tea          0.14.2     Official command-line tool to interact with Gitea servers
testdisk     7.2        TestDisk & PhotoRec. Data and digital picture recovery
tuicr        0.18.0     A code review TUI with vim keybindings. Export to GitHub or clipboard.
ungit        1.5.30     The easiest way to use git. On any platform. Anywhere.
vcxsrv       21.1.16.1  Windows X-server based on the xorg git sources (like xming or cygwin's xwin)
vibrance-gui 2.3        Automating NVIDIAs Digitial Vibrance Control and AMDs Saturation for any game
vmr          0.7.5      A general version manager for thousands of SDKs with TUI inspired by lazygit.
win-portacle 1.4        A Multi-platform Portable CLE (Common Lisp Environment), made up of Emacs, SBCL, QuickLisp, Git and more
winyl        3.3.1      A FOS digital audio player and music library application for organizing and playing audio on Windows.
worktrunk    0.62.0     A CLI for Git worktree management, designed for parallel AI agent workflows.
wslgit       1.3.1      A small executable that forwards all arguments to git running inside Bash on Windows/Windows Subsystem for Linux (WSL)
y-cruncher   0.8.7.9547b Scalable multi-threaded benchmark calculating multiple mathematical constants to trillions of digits.
zizmor       1.26.1     A static analysis tool for GitHub Actions that finds common security issues such as template injection, credential leakage, excessive permissions, and impostor commits.

共 162 个结果


────────────────────────────────────────────────
## §3.2
────────────────────────────────────────────────
命令: hit s python
输出（原样）:
[2m2026-06-28T04:32:15.333621Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:32:15.338968Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:32:15.345520Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:32:15.346476Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

[2m2026-06-28T04:32:15.353646Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

[2m2026-06-28T04:32:15.365232Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......

名称           版本         描述
anaconda2    2019.10    The most popular Python distribution for data science. (for Python 2 only)
anaconda3    2025.12-2  The most popular Python distribution for data science.
anaconda3-2022.05 2022.05    The most popular Python distribution for data science.
angr-management 9.2.222    The official GUI for angr, an open-source binary analysis platform for Python
austin       4.0.0      Python frame stack sampler for CPython
cudatext     1.234.4.0  A cross-platform, open-source text editor written in Object Pascal, available free of charge for both personal and commercial use, featuring fast startup and extensibility through Python-based add-ons.
gpodder      3.11.5     Simple open source podcast client written in Python using GTK+.
hatch        1.17.0     A modern, extensible Python project manager.
inbac        2.1.0      Python application for fast interactive image cropping
laragon      8.6.1      Universal development environment for PHP, Node.js, Python, Java, Go, Ruby
mihomo       1.19.27    A simple Python Pydantic model for Honkai: Star Rail parsed data from the Mihomo API.
miniconda2   4.8.3      A cross-platform, Python-agnostic binary package manager (for Python 2 only)
miniconda3   26.3.2-2   A cross-platform, Python-agnostic binary package manager
miniconda3-4.12.0 4.12.0     A cross-platform, Python-agnostic binary package manager
miniconda3-py310 26.3.2-2   A cross-platform, Python-agnostic binary package manager
miniconda3-py311 26.3.2-2   A cross-platform, Python-agnostic binary package manager
miniconda3-py37 23.1.0-1   A cross-platform, Python-agnostic binary package manager
miniconda3-py38 23.11.0-2  A cross-platform, Python-agnostic binary package manager
miniconda3-py39 25.9.1-3   A cross-platform, Python-agnostic binary package manager
mu-editor    1.2.0      A small, simple editor for beginner Python programmers.
nim          2.2.10     A statically typed compiled systems programming language, which combines successful concepts from mature languages like Python, Ada and Modula.
oh-my-pi     16.2.2     AI Coding agent for the terminal — hash-anchored edits, optimized tool harness, LSP, Python, browser, subagents, and more (fork of pi).
onthespot    1.1.4      OnTheSpot is an open-source user-friendly music downloader built with Qt and Python.
pdd          1.7        Date/time difference calculator and countdown timer. The name 'pdd' stands for Python Date Diff.
pipx         1.15.0     Install and run Python applications in isolated environments
poetry       2.4.1      Dependency Management for Python
propertree   0.2026.06.20 Cross platform GUI plist editor written in python.
pycharm      2026.1.3-261.25134.203 Cross-Platform IDE for Python by JetBrains.
pycharm-eap  2025.2-252.23892.194 Cross-Platform IDE for Python by JetBrains. (Community Edition, Early Access Program)
pycharm-latest 2026.2-262.8377.41 Cross-Platform IDE for Python by JetBrains (Latest).
pycharm-professional-eap 2026.2-262.8377.41 Cross-Platform IDE for Python by JetBrains. (Early Access Program)
pycharm-professional-rc 2026.1.4-261.26222.30 Cross-Platform IDE for Python by JetBrains. (Release Candidate)
pycharm-rc   2025.2.3-252.26830.27 Cross-Platform IDE for Python by JetBrains. (Community Edition, Release Candidate)
pyenv        3.1.1      Simple python version management tool for switching between multiple versions of Python.
pyflow       0.3.1      A modern Python installation and dependency manager
pyoxidizer   0.24.0     A modern Python application packaging and distribution tool
pypy2        7.3.23     A fast, compliant alternative implementation of the Python language.
pypy3        7.3.23     A fast, compliant alternative implementation of the Python language.
pyrefly      1.1.1      A fast type checker and language server for Python.
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
pyzo         4.21.0     The Interactive editor for scientific Python
rcc          17.18.0    Allows you to create, manage, and distribute Python-based self-contained automation packages.
renpy        8.5.3      Popular open source visual novel engine that uses Python for scripting
ruff         0.15.20    An extremely fast Python linter and code formatter, written in Rust.
rye          0.44.0     A comprehensive project and package management solution for Python
spyder       5.5.6      The Scientific Python Development Environment.
spyder-lite  5.5.6      The Scientific Python Development Environment, Lite version (lacks a number of optional but recommended dependencies).
thonny       4.1.7      Python IDE for beginners
ty           0.0.55     An extremely fast Python type checker and language server, written in Rust.
upm          1.0        Universal package manager for Python, Node.js, Ruby and Emacs Lisp.
uv           0.11.25    An extremely fast Python package installer and resolver, written in Rust.
wing-101     11.1.0.0   A very simple free Python IDE designed for teaching beginning programmers.
winpython    3.14.5.0   Free, open-source and portable Python distribution for Windows
winpython37  3.7.7.1    Free, open-source and portable Python distribution for Windows (3.7.x)
winpython3741 3.7.4.1    Free, open-source and portable Python distribution for Windows
winpython37cod 3.7.7.1    Free, open-source and portable Python distribution for Windows (3.7.x w/ VS Code)
winpython37ps2 3.7.6.0    Free, open-source and portable Python distribution for Windows (3.7.x w/ PySide2)
winpython38  3.8.9.0    Free, open-source and portable Python distribution for Windows (3.8.x)
winpython38cod 3.8.7.0    Free, open-source and portable Python distribution for Windows (3.8.x w/ VS Code)
winpython38ps2 3.8.1.0    Free, open-source and portable Python distribution for Windows (3.8.x w/ PySide2)
yasb         2.0.5      A highly configurable Windows status bar written in Python
youtube-dl-gui 1.8.5      A cross platform front-end GUI of the popular youtube-dl written in wxPython.
yubikey-manager-cli 5.9.1      Python library and command line tool for configuring any YubiKey over all USB interfaces.

共 78 个结果


────────────────────────────────────────────────
## §3.3
────────────────────────────────────────────────
命令: hit search GIT
输出（原样）:
[2m2026-06-28T04:32:15.603491Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:32:15.607993Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:32:15.611094Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:32:15.617444Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

[2m2026-06-28T04:32:15.626839Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

[2m2026-06-28T04:32:15.635882Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......

名称           版本         描述
act          0.2.89     Local Github actions runner.
actionlint   1.7.12     Static checker for GitHub Actions workflow files
argocd       3.4.4      CLI for ArgoCD - declarative, GitOps continuous delivery tool for Kubernetes
argocd-autopilot 0.4.20     A tool which offers an opinionated way of installing Argo-CD and managing GitOps repositories.
bat          0.26.1     A cat(1) clone with syntax highlighting and Git integration
bfg          1.15.0     BFG Repo-Cleaner removes large or troublesome blobs like git-filter-branch does, but faster
bit          1.1.2      Modernized git CLI
caesium-image-compressor 2.8.5      Caesium is an image compression software that helps you store, send and share digital pictures, supporting JPG, PNG and WebP formats. You can quickly reduce the file size (and resolution, if you want) by preserving the overall quality of the image.
carvel-vendir 0.46.0     Carvel vendir is a tool that makes it easy to vendor portions of git repos, github releases, helm charts, docker image contents, etc. declaratively
cdex         2.24       Open-source Digital Audio CD Extractor
claude-code  2.1.195    An agentic coding tool built by Anthropic that lives in your terminal, understands your codebase, and helps you code faster by executing routine tasks, explaining complex code, and handling git workflows.
cocogitto    7.0.0      The Conventional Commits toolbox.
codeowners-validator 0.7.4      The GitHub Codeowners file validator
codeql       2.25.6     Source code security analyzer from GitHub
deepgit      26.1.003   Git Archaeology Tool.
deepgit402   4.0.2      Git Archaeology Tool.
delta        0.19.2     A syntax-highlighter for git and diff output
devhub       0.102.0    TweetDeck for GitHub
diffnav      0.11.0     A git diff pager based on delta but with a file tree.
digital      0.31       A digital logic designer and circuit simulator
doctl        1.163.0    A command line tool for DigitalOcean services
dolt         2.1.10     Dolt is a SQL database that you can fork, clone, branch, merge, push and pull just like a git repository.
dra          0.10.2     A command line tool to download release assets from GitHub
eget         1.3.4      Easily install prebuilt binaries from GitHub.
fork         2.20.1     A fast and friendly git client for Mac and Windows
gg           0.39.1     A GUI for Jujutsu, a Git-compatible DVCS
gh           2.95.0     Official GitHub CLI
ghorg        1.11.12    Quickly clone an entire org/users repositories into one directory - Supports GitHub, GitLab, Bitbucket, and more
gibo         3.0.22     gibo (short for .gitignore boilerplates) is a shell script to help you easily access .gitignore boilerplates from github.com/github/gitignore
gig          0.8.3      Generate .gitignore files from your terminal (mostly) offline!
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
gk           3.1.68     GitKraken CLI
glab         1.105.0    GitLab CLI
glitter      1.6.6      Git tooling of the future
gmic-cli     3.7.6      A full-featured open-source framework for digital image processing.
gmic-qt      3.7.6      A full-featured open-source framework for digital image processing.
gogs         0.13.3     A painless self-hosted Git service
gogs0        0.11.91    A painless self-hosted Git service
gut          0.3.3      An easy to use Git client for the command line
helm-chart-releaser 1.8.1      Tool designed to help GitHub repos self-host their own chart repos by adding Helm chart artifacts to GitHub Releases named for the chart version and then creating an index.yaml file for those releases that can be hosted on GitHub Pages (or elsewhere!).
hk           1.48.0     A git hook manager and project linting tool with an emphasis on performance.
hub          2.14.2     An extension to command-line git that helps with everyday GitHub tasks without ever leaving the terminal.
ignoreit     3.0.0      Quickly load .gitignore templates
jasper       1.1.2      A flexible and powerful issue reader for GitHub.
jcpicker     6.2        Just Color Picker - Free portable offline colour picker and colour editor for web designers and digital artists.
jj           0.42.0     Jujutsu is a Git-compatible DVCS that is both simple and powerful
kodi         21.3       Open source home theater/media center software and entertainment hub for digital media
kodi-dev     20260626   Open source home theater/media center software and entertainment hub for digital media
kodi-nightly 20250129   Open source home theater/media center software and entertainment hub for digital media
krita        5.3.2.1    A free digital painting application
lab          0.25.1     An extension to command-line git that helps with everyday GitLab tasks.
lazy-posh-git 0.2.0      PowerShell proxy command around Set-Location to defer import of posh-git module until one changes working directory to the root of a git directory.
lazygit      0.62.2     A simple terminal UI for git commands
lefthook     2.1.9      Fast and powerful Git hooks manager for any type of projects
legit        1.2.0      Complementary command-line interface for Git.
lepton       1.10.0     A lean code snippet manager based on GitHub Gist
logisim-evolution 4.1.0      Digital logic design tool and simulator
logitech-omm 2.6.1749   OnBoard Memory Manager (OMM) is a utility for pro gamers to quickly view, customize, and fine-tune the onboard memory of a compatible Logitech G mouse.
mergiraf     0.17.0     A syntax-aware git merge driver
metrogit     0.4.0      Git visualization tool that's more than just git.
mingit       2.54.0     Minimal Git for Windows (MinGit) is a lightweight distribution intended primarily for application-integration scenarios (such as integrated development environments and graphical visualization tools) where full interactive console capabilities, including colorization and pagination, are not required.
mingit-busybox 2.54.0     Minimal Git for Windows (MinGit) is a reduced sized package designed to support application integration (like integrated development environments, graph visualizers, etc.) where full console support (colorization, pagniation, etc.) is not needed.(BusyBox-backed MinGit, experimental, smaller version of MinGit)
mob          5.4.2      Smooth git handover with mob
mpv-git      20260610   Video player based on MPlayer/mplayer2 (builds by shinchiro)
multi-gitter 0.63.1     A command-line tool for updating multiple repositories with a single command.
nero-aac     1.5.4.0    Nero Digital AAC Encoder (Command Line Tools)
no-mistakes  1.31.2     A local Git proxy that AI-validates your code before push, forwarding only clean commits upstream.
onefetch     2.27.1     Git repository summary on terminal
oss-cad-suite-nightly 2026-06-27 Open source digital design and verification tools. Includes tools for RTL synthesis, formal hardware verification, place & route, FPGA programming, and testing with support for HDLs like Verilog, Migen and Amaranth.
paket        10.3.1     Dependency manager for .NET with support for NuGet and Git repositories
posh-git     1.1.0      A PowerShell module which provides Git/PowerShell integration.
posh-git-beta 1.1.0      A PowerShell module which provides Git/PowerShell integration. (beta version)
posh-git-nightly bbc5ac3800 A PowerShell module which provides Git/PowerShell integration. (nightly version)
prismlauncher-git 12.0.0-2451-475ab8a An open source Minecraft launcher with the ability to manage multiple instances, accounts and mods. Focused on user freedom and free redistributability.
prs          0.5.1      Secure, fast & convenient password manager CLI with GPG & git sync
psgithub     0.15.240   PowerShell module to manage GitHub through its REST API.
ptags        0.3.5      A parallel universal-ctags wrapper for git repositories.
reaper       7.75       Digital Audio Workstation
rtlutility   1.0.12     Tool for measuring the Round Trip Latency of your Digital Audio Workstation (DAW) and audio interface
sapling      0.2.20250521.115337.25ed6ac4 Sapling SCM is a cross-platform, highly scalable, Git-compatible source control system.
scoop-sd     0.3        A program to search for scoop packages. Powered by https://scoopsearch.github.io/
sleuthkit    4.15.0     A collection of command line digital forensics tools that allow you to investigate volume and file system data.
smartgit     26.1.038   A graphical Git client with support for SVN and Pull Requests for GitHub and Bitbucket.
smimesign    0.2.0      An S/MIME signing utility compatible with Git that allows developers to sign their commits and tags using X.509 certificates
sophia-script 7.1.6      Sophia Script for Windows is the largest PowerShell module on GitHub for Windows 10 and Windows 11 fine-tuning and automating the routine tasks.
sourcegit    2026.13    Open-source GUI client for git users.
sourcetree   3.4.31     Simple and powerful Git GUI
spotify      1.2.92.148.g882cc571 A digital music service that gives you access to millions of songs.
stgit        2.6.1      Manage Git commits as a stack of patches
sublime-merge 2125       A Git client with snappy UI, three-way merge tool, side-by-side diffs, syntax highlighting, and more.
super-productivity 18.12.0    To-do list & time tracker for programmers and other digital workers with Jira, Github, and Gitlab integration
tea          0.14.2     Official command-line tool to interact with Gitea servers
testdisk     7.2        TestDisk & PhotoRec. Data and digital picture recovery
tuicr        0.18.0     A code review TUI with vim keybindings. Export to GitHub or clipboard.
ungit        1.5.30     The easiest way to use git. On any platform. Anywhere.
vcxsrv       21.1.16.1  Windows X-server based on the xorg git sources (like xming or cygwin's xwin)
vibrance-gui 2.3        Automating NVIDIAs Digitial Vibrance Control and AMDs Saturation for any game
vmr          0.7.5      A general version manager for thousands of SDKs with TUI inspired by lazygit.
win-portacle 1.4        A Multi-platform Portable CLE (Common Lisp Environment), made up of Emacs, SBCL, QuickLisp, Git and more
winyl        3.3.1      A FOS digital audio player and music library application for organizing and playing audio on Windows.
worktrunk    0.62.0     A CLI for Git worktree management, designed for parallel AI agent workflows.
wslgit       1.3.1      A small executable that forwards all arguments to git running inside Bash on Windows/Windows Subsystem for Linux (WSL)
y-cruncher   0.8.7.9547b Scalable multi-threaded benchmark calculating multiple mathematical constants to trillions of digits.
zizmor       1.26.1     A static analysis tool for GitHub Actions that finds common security issues such as template injection, credential leakage, excessive permissions, and impostor commits.

共 162 个结果


────────────────────────────────────────────────
## §3.4
────────────────────────────────────────────────
命令: hit search git --bucket main
输出（原样）:
[2m2026-06-28T04:32:15.878916Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:32:15.885436Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:32:15.890688Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:32:15.897522Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

[2m2026-06-28T04:32:15.898809Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

[2m2026-06-28T04:32:15.917022Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
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
ghorg        1.11.12    Quickly clone an entire org/users repositories into one directory - Supports GitHub, GitLab, Bitbucket, and more
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
no-mistakes  1.31.2     A local Git proxy that AI-validates your code before push, forwarding only clean commits upstream.
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
[2m2026-06-28T04:32:16.146411Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:32:16.151252Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:32:16.153349Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

[2m2026-06-28T04:32:16.158362Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

[2m2026-06-28T04:32:16.162147Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:32:16.179973Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......

未找到匹配 'nonexistent_xyz' 的软件


────────────────────────────────────────────────
## §4.1
────────────────────────────────────────────────
命令: hit info git
输出（原样）:
[2m2026-06-28T04:32:16.398804Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:32:16.401479Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:32:16.405733Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

[2m2026-06-28T04:32:16.410434Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

[2m2026-06-28T04:32:16.417060Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:32:16.423103Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......

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
[2m2026-06-28T04:32:16.674737Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:32:16.675081Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:32:16.677709Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:32:16.688071Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

[2m2026-06-28T04:32:16.688797Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

[2m2026-06-28T04:32:16.709209Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......

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
[2m2026-06-28T04:32:16.947025Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:32:16.951658Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:32:16.951929Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:32:16.956927Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

[2m2026-06-28T04:32:16.971480Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

[2m2026-06-28T04:32:16.975014Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......



────────────────────────────────────────────────
## §4.4
────────────────────────────────────────────────
命令: hit info curl
输出（原样）:
[2m2026-06-28T04:32:17.220730Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:32:17.222700Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:32:17.233004Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

[2m2026-06-28T04:32:17.233109Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:32:17.239558Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

[2m2026-06-28T04:32:17.250807Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......

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
[2m2026-06-28T04:32:17.487443Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:32:17.496890Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

[2m2026-06-28T04:32:17.507204Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:32:17.513729Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:32:17.514324Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......

[2m2026-06-28T04:32:17.518858Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

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
[2m2026-06-28T04:32:24.966019Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:32:24.971083Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

[2m2026-06-28T04:32:24.971697Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:32:24.975874Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:32:24.984361Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

[2m2026-06-28T04:32:25.006402Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......

▶ [解析] jq...
✔ [解析] jq 完成
▶ [下载] jq...
✔ [下载] jq 完成
▶ [校验] jq...
✔ [校验] jq 完成
解压 jq (jq#1.8.2#abde28e.exe)
▶ [解压] jq...
[2m2026-06-28T04:32:27.688175Z[0m [33m WARN[0m 事务回滚 [3mapp[0m[2m=[0mjq


────────────────────────────────────────────────
## §5.3
────────────────────────────────────────────────
命令: hit install curl
输出（原样）:
安装 curl ...
[2m2026-06-28T04:32:27.965118Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:32:27.967453Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:32:27.972956Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:32:27.975004Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

[2m2026-06-28T04:32:27.988053Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......

[2m2026-06-28T04:32:27.991372Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......



────────────────────────────────────────────────
## §5.4
────────────────────────────────────────────────
命令: hit install curl --force
输出（原样）:
安装 curl ...
[2m2026-06-28T04:32:28.220785Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:32:28.226001Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:32:28.226998Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:32:28.232560Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

[2m2026-06-28T04:32:28.238591Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......

[2m2026-06-28T04:32:28.242253Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

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
[2m2026-06-28T04:32:29.287214Z[0m [33m WARN[0m 事务回滚 [3mapp[0m[2m=[0mcurl


────────────────────────────────────────────────
## §5.5
────────────────────────────────────────────────
命令: hit install main/git
输出（原样）:
安装 git ...
[2m2026-06-28T04:32:29.564072Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:32:29.570963Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:32:29.572621Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

[2m2026-06-28T04:32:29.574719Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:32:29.577394Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

[2m2026-06-28T04:32:29.582404Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......

▶ [解析] git...
✔ [解析] git 完成
▶ [下载] git...
✔ [下载] git 完成
▶ [校验] git...
✔ [校验] git 完成
解压 git (git#2.54.0#b0a3d5f.7z)
▶ [解压] git...
[2m2026-06-28T04:32:36.828942Z[0m [33m WARN[0m 事务回滚 [3mapp[0m[2m=[0mgit


────────────────────────────────────────────────
## §5.6
────────────────────────────────────────────────
命令: hit install nonexistent_pkg
输出（原样）:
安装 nonexistent_pkg ...
[2m2026-06-28T04:32:37.115526Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:32:37.119658Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:32:37.125585Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:32:37.140491Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

[2m2026-06-28T04:32:37.143089Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......

[2m2026-06-28T04:32:37.160070Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......



────────────────────────────────────────────────
## §5.7
────────────────────────────────────────────────
命令: hit install jq --arch 64bit
输出（原样）:
安装 jq ...
[2m2026-06-28T04:32:37.414328Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:32:37.418886Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:32:37.427331Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:32:37.429257Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

[2m2026-06-28T04:32:37.436584Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......

[2m2026-06-28T04:32:37.438074Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

▶ [解析] jq...
✔ [解析] jq 完成
▶ [下载] jq...
✔ [下载] jq 完成
▶ [校验] jq...
✔ [校验] jq 完成
解压 jq (jq#1.8.2#abde28e.exe)
▶ [解压] jq...
[2m2026-06-28T04:32:37.462363Z[0m [33m WARN[0m 事务回滚 [3mapp[0m[2m=[0mjq


────────────────────────────────────────────────
## §6.1
────────────────────────────────────────────────
命令: hit list
输出（原样）:
名称           版本         架构       Bucket     安装时间
curl         8.21.0_1   64bit    main       2026-06-28T04:32:24Z

共 1 个软件


────────────────────────────────────────────────
## §6.2
────────────────────────────────────────────────
命令: hit ls
输出（原样）:
名称           版本         架构       Bucket     安装时间
curl         8.21.0_1   64bit    main       2026-06-28T04:32:24Z

共 1 个软件


────────────────────────────────────────────────
## §6.3
────────────────────────────────────────────────
命令: hit list curl
输出（原样）:
名称           版本         架构       Bucket     安装时间
curl         8.21.0_1   64bit    main       2026-06-28T04:32:24Z

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

[2m2026-06-28T04:33:04.906549Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:33:04.913687Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:33:04.915272Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:33:04.926129Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

[2m2026-06-28T04:33:04.933417Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

[2m2026-06-28T04:33:04.936242Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......

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

────────────────────────────────────────────────
## §8.1
────────────────────────────────────────────────
命令: hit update
输出（原样）:
刷新 正在更新 Bucket...
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

[2m2026-06-28T04:33:33.161012Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:33:33.164552Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:33:33.174908Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:33:33.180504Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

[2m2026-06-28T04:33:33.203470Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......

[2m2026-06-28T04:33:33.216460Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

所有软件已是最新版本


────────────────────────────────────────────────
## §8.2
────────────────────────────────────────────────
命令: hit update --all
输出（原样）:
刷新 正在更新 Bucket...
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

[2m2026-06-28T04:33:59.386348Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:33:59.402333Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

[2m2026-06-28T04:33:59.403889Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:33:59.421975Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:33:59.432081Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

[2m2026-06-28T04:33:59.442559Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......

所有软件已是最新版本


────────────────────────────────────────────────
## §8.3
────────────────────────────────────────────────
命令: hit update curl
输出（原样）:
刷新 正在更新 Bucket...
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

[2m2026-06-28T04:34:30.352101Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:34:30.360366Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:34:30.360482Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

[2m2026-06-28T04:34:30.363195Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:34:30.390599Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

[2m2026-06-28T04:34:30.408339Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......

所有软件已是最新版本


────────────────────────────────────────────────
## §8.4
────────────────────────────────────────────────
命令: hit update nonexistent
输出（原样）:
刷新 正在更新 Bucket...
正在更新 bucket 'extras'...
正在克隆 bucket 'extras'...
bucket 'extras'：检出文件中...
bucket 'extras' 克隆完成
  ✔ extras
正在更新 bucket 'main'...
正在克隆 bucket 'main'...
bucket 'main'：检出文件中...
  ✔ main
bucket 'main' 克隆完成
正在更新 bucket 'versions'...
正在克隆 bucket 'versions'...
bucket 'versions'：检出文件中...
bucket 'versions' 克隆完成
  ✔ versions
✔ Bucket 更新完成（3/3）

[2m2026-06-28T04:34:58.871257Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:34:58.874954Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:34:58.885739Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

[2m2026-06-28T04:34:58.887512Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

[2m2026-06-28T04:34:58.917898Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:34:58.925651Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......

  nonexistent 未安装，跳过
所有软件已是最新版本


────────────────────────────────────────────────
## §8.5
────────────────────────────────────────────────
命令: hit update --force
输出（原样）:
刷新 正在更新 Bucket...
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

[2m2026-06-28T04:35:27.519185Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:35:27.527389Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

[2m2026-06-28T04:35:27.532437Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:35:27.536270Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

[2m2026-06-28T04:35:27.538314Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:35:27.565078Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......

⬆ 可升级 1 个软件
升级 curl → 8.21.0_1
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
[2m2026-06-28T04:35:29.638227Z[0m [33m WARN[0m 事务回滚 [3mapp[0m[2m=[0mcurl
  ✘ 升级失败: IO 错误：创建 Junction: C:\Users\Violet\Downloads\test\hit\apps\curl\current -> C:\Users\Violet\Downloads\test\hit\apps\curl\8.21.0_1：Cannot create a file when that file already exists. (os error 183)

✔ 升级完成（0/1）


────────────────────────────────────────────────
## §9.1
────────────────────────────────────────────────
命令: hit uninstall jq
输出（原样）:

────────────────────────────────────────────────
## §9.2
────────────────────────────────────────────────
命令: hit rm curl --purge
输出（原样）:
卸载 curl ...


────────────────────────────────────────────────
## §9.3
────────────────────────────────────────────────
命令: hit uninstall nonexistent
输出（原样）:

────────────────────────────────────────────────
## §9.4
────────────────────────────────────────────────
命令: hit uninstall
输出（原样）:

────────────────────────────────────────────────
## §9.5
────────────────────────────────────────────────
命令: hit uninstall jq curl
输出（原样）:

────────────────────────────────────────────────
## §10-pre
────────────────────────────────────────────────
命令: hit install curl
输出（原样）:
安装 curl ...
[2m2026-06-28T04:35:30.787085Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:35:30.794289Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:35:30.798598Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:35:30.807209Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......

[2m2026-06-28T04:35:30.817414Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

[2m2026-06-28T04:35:30.820358Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......



────────────────────────────────────────────────
## §10.1
────────────────────────────────────────────────
命令: hit cache list
输出（原样）:
软件                   版本         大小         路径
curl                 8.21.0_1   4.7 MB     C:\Users\Violet\Downloads\test\hit\cache\curl#8.21.0_1#39c2972.xz
git                  2.54.0     56.3 MB    C:\Users\Violet\Downloads\test\hit\cache\git#2.54.0#b0a3d5f.7z
jq                   1.8.2      1011.0 KB  C:\Users\Violet\Downloads\test\hit\cache\jq#1.8.2#abde28e.exe

共 3 个文件（62.0 MB）


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
✔ 已清理 3 个缓存文件


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
  删除 git 2.54.0
  删除 jq 1.8.2

✔ 已清理 2 个旧版本


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

────────────────────────────────────────────────
## §12.1.2
────────────────────────────────────────────────
命令: hit which nonexistent
输出（原样）:

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
[2m2026-06-28T04:35:32.728319Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:35:32.733868Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:35:32.738655Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

[2m2026-06-28T04:35:32.741880Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

[2m2026-06-28T04:35:32.749711Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:35:32.763253Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......



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

────────────────────────────────────────────────
## §13.8
────────────────────────────────────────────────
命令: hit config set unknown_key value
输出（原样）:

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
⚠ 发现 3 个问题：

  ⚠ curl: current 链接损坏 (可修复)
  ✗ git: 未跟踪的应用目录
  ✗ jq: 未跟踪的应用目录

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
⚠ 发现 3 个问题：

  ⚠ curl: current 链接损坏 (可修复)
  ✗ git: 未跟踪的应用目录
  ✗ jq: 未跟踪的应用目录

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
[2m2026-06-28T04:35:34.628328Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:35:34.630022Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:35:34.637180Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

[2m2026-06-28T04:35:34.641385Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:35:34.642131Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

[2m2026-06-28T04:35:34.642848Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......

Hit 0.1.0

  已安装软件:    1
  Bucket 数量:   3
  可用软件总数:  4500
  缓存文件:      0 (0 B)
  根目录:        C:\Users\Violet\Downloads\test\hit


────────────────────────────────────────────────
## §16.2
────────────────────────────────────────────────
命令: hit st
输出（原样）:
[2m2026-06-28T04:35:34.899570Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:35:34.903215Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:35:34.909330Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

[2m2026-06-28T04:35:34.915839Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......

[2m2026-06-28T04:35:34.917271Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

[2m2026-06-28T04:35:34.920793Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

Hit 0.1.0

  已安装软件:    1
  Bucket 数量:   3
  可用软件总数:  4500
  缓存文件:      0 (0 B)
  根目录:        C:\Users\Violet\Downloads\test\hit


────────────────────────────────────────────────
## §17-i
────────────────────────────────────────────────
命令: hit i nonexistent_alias_test
输出（原样）:
安装 nonexistent_alias_test ...
[2m2026-06-28T04:35:35.182446Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:35:35.186161Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:35:35.186911Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

[2m2026-06-28T04:35:35.193344Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:35:35.199322Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

[2m2026-06-28T04:35:35.211176Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......



────────────────────────────────────────────────
## §17-s
────────────────────────────────────────────────
命令: hit s nonexistent_alias_test
输出（原样）:
[2m2026-06-28T04:35:35.458011Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:35:35.459906Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:35:35.466636Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:35:35.467029Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

[2m2026-06-28T04:35:35.467323Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

[2m2026-06-28T04:35:35.489013Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......

未找到匹配 'nonexistent_alias_test' 的软件


────────────────────────────────────────────────
## §17-u
────────────────────────────────────────────────
命令: hit u nonexistent
输出（原样）:
刷新 正在更新 Bucket...
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

[2m2026-06-28T04:36:03.743760Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:36:03.758137Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:36:03.759074Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:36:03.759781Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

[2m2026-06-28T04:36:03.760569Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

[2m2026-06-28T04:36:03.788127Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......

  nonexistent 未安装，跳过
所有软件已是最新版本


────────────────────────────────────────────────
## §17-rm
────────────────────────────────────────────────
命令: hit rm nonexistent
输出（原样）:

────────────────────────────────────────────────
## §17-ls
────────────────────────────────────────────────
命令: hit ls
输出（原样）:
名称           版本         架构       Bucket     安装时间
curl         8.21.0_1   64bit    main       2026-06-28T04:32:24Z

共 1 个软件


────────────────────────────────────────────────
## §17-st
────────────────────────────────────────────────
命令: hit st
输出（原样）:
[2m2026-06-28T04:36:04.344519Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\megasync.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 6 column 4

	1"
    ],
    
	........^.......

[2m2026-06-28T04:36:04.346882Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\filezilla.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 8 column 4

	0"
    ],
    
	........^.......

[2m2026-06-28T04:36:04.348925Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\irfanview.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 92 column 12

	        },
    
	........^.......

[2m2026-06-28T04:36:04.349248Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\bizhawk.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	s"
    ],
    
	........^.......

[2m2026-06-28T04:36:04.364707Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\qrencode.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 89 column 12

	        }
     
	........^.......

[2m2026-06-28T04:36:04.379327Z[0m [33m WARN[0m 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\extras\bucket\tablacus-explorer.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 5 column 4

	."
    ],
    
	........^.......

Hit 0.1.0

  已安装软件:    1
  Bucket 数量:   3
  可用软件总数:  4500
  缓存文件:      0 (0 B)
  根目录:        C:\Users\Violet\Downloads\test\hit


────────────────────────────────────────────────
## §17-b
────────────────────────────────────────────────
命令: hit b ls
输出（原样）:
名称                  Manifest    描述
extras                2321        
main                  1593        
versions              592         

共 3 个 Bucket


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

────────────────────────────────────────────────
## §18.1
────────────────────────────────────────────────
命令: hit -v list
输出（原样）:
名称           版本         架构       Bucket     安装时间
curl         8.21.0_1   64bit    main       2026-06-28T04:32:24Z

共 1 个软件


────────────────────────────────────────────────
## §18.2
────────────────────────────────────────────────
命令: hit -vv list
输出（原样）:
名称           版本         架构       Bucket     安装时间
curl         8.21.0_1   64bit    main       2026-06-28T04:32:24Z

共 1 个软件


────────────────────────────────────────────────
## §18.3
────────────────────────────────────────────────
命令: hit -vvv list
输出（原样）:
名称           版本         架构       Bucket     安装时间
curl         8.21.0_1   64bit    main       2026-06-28T04:32:24Z

共 1 个软件


────────────────────────────────────────────────
## §19.1
────────────────────────────────────────────────
命令: hit
输出（原样）:

────────────────────────────────────────────────
## §19.2
────────────────────────────────────────────────
命令: hit wrongcmd
输出（原样）:

────────────────────────────────────────────────
## §19.3
────────────────────────────────────────────────
命令: hit install
输出（原样）:

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
curl         8.21.0_1   64bit    main       2026-06-28T04:32:24Z

共 1 个软件

