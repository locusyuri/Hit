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

## Manifest 兼容性缺陷：大量真实 Scoop manifest 解析失败 ⭐⭐⭐⭐⭐

> **极为严重** —— 直接破坏"Scoop 兼容性"这一项目首要约束，导致 `search`/`info`/`install` 等核心命令在官方 bucket（main/extras/versions）上可用率大幅下降，且每条命令向 stderr 喷出 ~1500 行 WARN，淹没正常输出。

### 现象

对官方 bucket 执行任意需要扫描 manifest 的命令（`hit search`、`hit info`、`hit install`、`hit bucket list` 等），stderr 输出海量 WARN，形如：

```
WARN 跳过无效 manifest '...\buckets\main\bucket\perl.json': manifest JSON 解析失败：data did not match any variant of untagged enum HashField at line 48 column 12
WARN 跳过无效 manifest '...\buckets\extras\bucket\digital.json': manifest JSON 解析失败：invalid type: sequence, expected a string at line 10 column 8
WARN 跳过无效 manifest '...\buckets\extras\bucket\feem.json': manifest JSON 解析失败：data did not match any variant of untagged enum CheckverField at line 21 column 5
```

实测在已添加 `main`/`extras`/`versions` 三个官方 bucket 的情况下，单次 `hit search git` 喷出 **约 1500 行 WARN**，被跳过的 manifest 涵盖 chromium、firefox、vscode、calibre、postman、android-studio、dbeaver、thunderbird、eclipse 全家桶等主流软件。这些 manifest 在原版 Scoop 下均正常工作。

### 根因（三类解析失败）

1. **`HashField` 不支持对象形式的 hash（最严重）**
   Scoop manifest 的 `hash` 字段在 `autoupdate` 块里允许写成对象，用 `url` + `jsonpath`/`regex` 从远程取哈希。例如 `buckets/main/bucket/perl.json` 第 44-47 行：
   ```json
   "hash": {
       "url": "https://strawberryperl.com/releases.json",
       "jsonpath": "$[?(@.version == '$version')].edition.portable.sha256"
   }
   ```
   Hit 的 `HashField` enum 显然只接受 `String` 或 `{url, regex}` 形式，不接受 `{url, jsonpath}` 形式，导致整个 manifest 被跳过。同类失败：chromium、firefox、vscode、calibre、aimp、android-studio、anaconda3、postman、thunderbird、eclipse 全家桶、mysql、haskell、qemu、lxc、kapacitor、chronograf、mvndaemon、ossgadget、cherrytree、meshroom、f3d 等（数十个）。

2. **`suggest`/`depends`/`env_add_path`/`notes`/`post_install` 等数组字段被误判为单字符串**
   Scoop 中 `suggest`、`depends`、`env_add_path`、`notes`、`post_install`、`extract_dir` 等字段允许是字符串**或**字符串数组。Hit 把这些字段声明为 `String` 而非 `StringOrVec`，遇到数组形式就报 `invalid type: sequence, expected a string`。例如 `buckets/extras/bucket/digital.json` 第 6-11 行的 `suggest`：
   ```json
   "suggest": {
       "JDK": ["java/opendk", "java/oraclejdk"]
   }
   ```
   实际报错指向第 10 行第 8 列（数组结尾）。同类失败：digital、megabasterd、megasync、filezilla、bizhawk、flutter、godot-mono、composer、yarn、git-lfs、yt-dlp、spotdl、kibana、logstash、stirling-pdf、plantuml、springboot、ffmpegthumbnailer、wixtoolset、unbound、autoit、sapling、bbdown、twilio-cli、ani-cli、etlas、euler、flamelens 等（数十个）。

3. **`CheckverField` 不支持部分 checkver 变体**
   Scoop 的 `checkver` 字段支持 `github`+`jsonpath`、`regex`+`reverse`、多字段对象等变体。Hit 的 `CheckverField` enum 漏了若干形式，报 `data did not match any variant of untagged enum CheckverField`。例如 `buckets/extras/bucket/feem.json`、`warp-terminal.json`、`buckets/main/bucket/edgedriver.json`、`buckets/versions/bucket/inkscape-dev.json`、`edgedriver-canary.json`、`edgedriver-dev.json`。

### 影响

- **可用性**：主流软件（chromium/firefox/vscode/calibre/postman/android-studio/eclipse/mysql 等）的 manifest 全部被跳过，`hit install <这些包>` 会报"找不到包"，而原版 Scoop 可正常安装。
- **性能**：每条 search/info 命令扫描全部 bucket manifest，单次耗时 1-2 分钟，且向 stderr 喷出 ~1500 行 WARN，淹没正常输出，用户几乎无法使用。
- **项目定位**：Hit 首要约束是"Scoop 兼容性"，此 bug 直接违反该约束，使 Hit 在官方 bucket 上形同不可用。

### 修复方向（给修复 agent）

1. **`HashField` 扩展**：接受 `{ "url": String, "jsonpath": String }` 和 `{ "url": String, "regex": String }` 以及 `{ "jp": String, "url": String }` 等所有 Scoop 支持的取哈希对象形式。参考原版 Scoop 的 `manifest.ps1` 中 `hash` 参数解析逻辑，以及 `ref/Scoop/` 下 PowerShell 源码。
2. **字段多态**：把 `suggest`/`depends`/`env_add_path`/`notes`/`post_install`/`extract_dir`/`pre_install`/`installer`/`uninstaller`/`shortcuts` 等所有"字符串或字符串数组"或"字符串或对象数组"的字段，统一用 `StringOrVec`（`#[serde(untagged)]` enum）声明。参考 `docs/spec/MANIFEST_FORMAT.md` 与 `ref/Main/` 下真实 manifest。
3. **`CheckverField` 扩展**：补全 Scoop 支持的所有 checkver 变体（`github`+`jsonpath`、`regex`+`reverse`+`replace`、多字段对象等）。参考原版 Scoop 的 `checkver` 实现。
4. **WARN 抑制**：解析失败的 manifest 应该聚合计数后一次性输出摘要（如"跳过 N 个无效 manifest，详情见 --verbose"），而非逐条喷 stderr。但这是次要——首要是把上述三类解析 bug 修掉，让真实 manifest 能被解析。
5. **回归测试**：修复后，对 `ref/Main/`（1591 个 manifest）、`ref/extras`、`ref/versions` 全量解析，要求 0 WARN、0 跳过。可作为单元测试：`parse_all_manifests_in(ref/Main/bucket)` 应全部成功。

### 证据样本

- `buckets/main/bucket/perl.json` 第 44-47 行：`hash` 为 `{url, jsonpath}` 对象 → `HashField` 失败
- `buckets/extras/bucket/digital.json` 第 6-11 行：`suggest` 值为数组 → `invalid type: sequence, expected a string`
- `buckets/extras/bucket/feem.json` 第 21 行：`checkver` 变体 → `CheckverField` 失败
- 完整 WARN 日志（单次 `hit search git`，约 1500 行）：见本次测试会话输出

### 备注

此 bug 使原计划的手动测试无法继续（每条命令输出被 WARN 淹没，REPORT 会达数百 MB）。**需先修复此 bug，再重启测试。** 修完后请通知测试 agent。

## Hit 本身的 Shim 异常 ⭐⭐⭐
shim 文件夹下的 `hit.exe` 大小 10873 KB，和根目录下的 `hit.exe` 大小相同。事实上这里应该是轻量的代理文件，而不是真正的 hit 程序。


## 设计问题 ⭐
```markdown
| 2.1.6 | `hit bucket add unknownbucket`（未知名称且无 URL） | 报错"未知 bucket '...'，请提供 Git 仓库 URL" |
```
输出结果确实是这样, 但从用户体验角度来说，应该提示用户具体怎么做。
以及, 能不能统一一下输出格式。

## 格式问题 ⭐
所有输出都没有对齐
