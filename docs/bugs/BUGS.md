# Bug 记录

> 星星数量表示严重程度，越多越严重。
> 已解决的 Bug 见 [SOLVED_BUGS.md](./SOLVED_BUGS.md)。

---
## 格式 ⭐
`hit status` 展示的内容未对齐:
```bash
hit status
Hit 0.1.0

  已安装软件:           0
  Bucket 数量:       0
  可用软件总数:          0
  缓存文件:            0 (0 B)
  根目录:             C:\Users\Violet\Downloads\test\hit
```
最好使用表格。

## 欢迎页面仍未触发 ⭐
这次欢迎页面彻底无法触发了, 就算是执行 `hit bucket list` 也没有显示欢迎页面。(见[SOLVED_BUGS.md](./SOLVED_BUGS.md)中之前的描述)

## 搜索命令出现问题 ⭐⭐⭐⭐⭐
搜索命令出现问题，比如执行 `hit search zed` 出现以下错误：
```
hit search zed

2026-06-26T15:09:57.689096Z  WARN 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\antigravity-cli.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 39 column 12

                },

        ........^.......

2026-06-26T15:09:57.690250Z  WARN 跳过无效 manifest 'C:\Users\Violet\Downloads\test\hit\buckets\main\bucket\perl.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 48 column 12
....
....

名称           版本         描述
ack          3.10.0     A tool like grep, optimized for programmers
aws-copilot  1.34.1     A tool for developers to build, release and operate production ready containerized applications on Amazon ECS and AWS Fargate.
bit          1.1.2      Modernized git CLI
cloud-sql-proxy 2.22.1     Provides secure access to Cloud SQL Second Generation instances without having to add Authorized networks or configure SSL.
conan        2.29.1     The open-source, decentralized C/C++ package manager
crc          2.61.0     Manages a local OpenShift 4.x cluster optimized for testing and development purposes.
drmemory     2.6.20434  Memory monitoring tool capable of identifying memory-related programming errors such as accesses of uninitialized/unaddressable/freed memory, double frees, memory leaks, handle leaks, GDI API usage errors and accesses to un-reserved thread local storage slots.
eventstore   24.6.0     The stream database optimized for event sourcing
lore         0.8.3      An open source version control system designed for unprecedented scalability of both data and teams, optimized for projects that combine code with large binary assets, including games and entertainment.
loreserver   0.8.3      The server for Lore, an open source version control system designed for unprecedented scalability of both data and teams; it provides the centralized service that hosts Lore repositories.
mingit-busybox 2.54.0     Minimal Git for Windows (MinGit) is a reduced sized package designed to support application integration (like integrated development environments, graph visualizers, etc.) where full console support (colorization, pagniation, etc.) is not needed.(BusyBox-backed MinGit, experimental, smaller version of MinGit)
nomad        2.0.3      Easy-to-use, flexible, and performant workload orchestrator that can deploy a mix of microservice, batch, containerized, and non-containerized applications.
oh-my-pi     16.1.22    AI Coding agent for the terminal — hash-anchored edits, optimized tool harness, LSP, Python, browser, subagents, and more (fork of pi).
openshift-origin-client 3.11.0     OpenShift Origin is a distribution of Kubernetes optimized for continuous application development and multi-tenant deployment. OpenShift adds developer and operations-centric tools on top of Kubernetes to enable rapid application development, easy deployment and scaling, and long-term lifecycle maintenance for small and large teams.
oscdimg      2.56       Command-line tool to create an image (.iso) file of a customized 32-bit or 64-bit version of Windows Preinstallation Environment (Windows PE).
pwsh         7.6.3      Cross-platform automation and configuration tool/framework, known as Powershell Core, that works well with existing tools and is optimized for dealing with structured data.
tectonic     0.16.9     Tectonic is a modernized, complete, self-contained TeX/LaTeX engine, powered by XeTeX and TeXLive.
wasmedge     0.17.0     A lightweight, high-performance, and extensible WebAssembly runtime for cloud native, edge, and decentralized applications. It powers serverless apps, embedded functions, microservices, smart contracts, and IoT devices.
```

虽然最后显示了搜索结果，但中间出现了很多 manifest 解析失败的警告，说明在解析 bucket 中的 manifest 时出现了问题。
以及搜索结果格式不对，没有对齐，应该使用表格。

!!! 搜索结果似乎并不相关 !!!

## Hit 本身的 Shim 异常 ⭐⭐⭐
shim 文件夹下的 `hit.exe` 大小 10873 KB，和根目录下的 `hit.exe` 大小相同。事实上这里应该是轻量的代理文件，而不是真正的 hit 程序。