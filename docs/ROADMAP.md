# 路线图

## Phase 1：Scoop 基础能力实现（预计 3 个月）

**目标**：实现 Scoop 的所有基本命令和功能，完全兼容 Scoop bucket

- [ ] 项目工作区结构设计
- [ ] 核心模块划分
- [ ] **Scoop Manifest 格式完全兼容解析**
- [ ] **Scoop Bucket 仓库支持（Git 仓库克隆与更新）**
- [ ] **实现 `hit install/uninstall/list` 基本命令**
- [ ] **实现 `hit search` 搜索命令**
- [ ] **实现 `hit info` 软件详情命令**
- [ ] **实现 `hit update` 更新命令**
- [ ] **实现 `hit bucket add/remove/list/update` Bucket 管理命令**
- [ ] **Shim 代理机制（兼容 Scoop 的 shim 格式）**
- [ ] **Persist 持久化机制（兼容 Scoop 的 persist 目录结构）**
- [ ] **事务性安装（原子操作，失败回滚）**
- [ ] **权限检测与自动提权机制**
- [ ] **基础测试框架搭建**

## Phase 2：Scoop 高级功能实现（预计 2 个月）

**目标**：实现 Scoop 的高级功能

- [ ] **实现 `hit reset` 版本切换命令**
- [ ] **实现 `hit cleanup` 清理旧版本命令**
- [ ] **实现 `hit cache` 缓存管理命令**
- [ ] **实现 `hit status` 状态检查命令**
- [ ] **实现 `hit home` 打开主页命令**
- [ ] **实现 `hit uninstall --purge` 彻底卸载命令**
- [ ] **依赖自动解析与安装（Scoop 的 depends 字段）**
- [ ] **Bucket 全局索引与自动选择**

## Phase 3：Hit 增强功能（预计 3 个月）

**目标**：在 Scoop 基础上添加 Hit 独有的增强功能

- [ ] 健康检查（`hit check`）
- [ ] 修复损坏（`hit repair`）
- [ ] 镜像源管理与速度测试
- [ ] 交互式搜索安装（FuzzySelect）
- [ ] 软件别名（alias）支持
- [ ] Bucket 冲突检测与解决
- [ ] Bucket 统计与过时检测
- [ ] 安全扫描集成（VirusTotal）

## Phase 4：高级特性（预计 4 个月）

- [ ] SDK 版本管理（JDK/Python/Node.js）
- [ ] 深度卸载模块
- [ ] Bucket 自动化更新流水线
- [ ] 多线程下载加速
- [ ] 软件束（Bundle）管理
- [ ] 沙盒环境（Shadow）

## Phase 5：生态与跨平台（预计 6 个月）

- [ ] 官方 Bucket 仓库
- [ ] Shell 补全脚本
- [ ] 配置同步
- [ ] 环境诊断（`hit doctor`）
- [ ] Linux 支持
- [ ] macOS 支持

---

## 新增功能详解

### 1. 事务性安装（Transaction）

- **模块**：`hit-core/transaction/`
- **职责**：保证安装/卸载的原子性，失败自动回滚
- **工作流程**：
  1. 创建临时事务目录
  2. 下载文件到临时目录
  3. 校验哈希
  4. 解压到临时目录
  5. 执行预安装脚本（验证）
  6. 原子移动：`rename` 临时目录 → 正式目录
  7. 更新 `db.json`
  8. 生成 Shim
  9. 提交事务
- **回滚机制**：任一阶段失败，清理临时文件，恢复 `db.json` 快照
- **实现**：使用 `tempfile` 创建临时目录，`std::fs::rename` 原子移动

### 2. 依赖解析（Dependencies）

- **模块**：`hit-core/dependencies/`
- **职责**：自动解析并安装软件依赖
- **Manifest 字段**：

```json
{
  "dependencies": {
    "vc_redist": {
      "version": ">=14.0",
      "type": "runtime",
      "optional": false
    },
    "openssl": {
      "version": "3.0",
      "type": "optional"
    }
  }
}
```

- **依赖图**：检测循环依赖、版本冲突
- **安装策略**：先安装依赖，再安装主包；依赖已满足则跳过

### 3. Bucket 优化（Bucket Optimizations）

- **模块**：`hit-core/`（bucket 相关子模块）
- **职责**：解决多 bucket 冲突，提升搜索安装体验
- **核心设计**：
  - **全局索引**：内存缓存所有 bucket 的 manifest，合并为 `软件名 → [版本列表]`
  - **优先级系统**：bucket 按优先级排序（main=100 > sdk=50 > extras=30）
  - **自动选择**：安装时自动选最高版本 + 最高优先级 bucket
  - **交互式选择**：搜索后提供 FuzzySelect 界面，上下箭头选择，Enter 直接安装

**关键功能**：
1. **全局搜索**：显示所有 bucket 结果，标注来源
2. **版本约束语法**：`@latest`、`@stable`、`@^3.12`、`@3.12.0`
3. **冲突检测**：`hit bucket conflict list/resolve`
4. **软件别名**：Manifest `alias` 字段，支持 `py` → `python`
5. **安装前预览**：显示版本、大小、依赖、bucket 来源
6. **Bucket 统计**：`hit bucket stats/outdated`
7. **快速安装**：`hit install` 无参数时直接弹出选择框

**Bucket 元数据（bucket.json）**：

```json
{
  "priority": 100,
  "maintainer": "Hit Team",
  "package_count": 156,
  "auto_update": true
}
```

### 4. 健康检查（Health Check）

- **模块**：`hit-core/health/`
- **职责**：定期检查安装完整性，自动修复损坏
- **子命令**：
  - `hit check` - 检查所有软件
  - `hit check <package>` - 检查指定软件
  - `hit repair <package>` - 重新下载损坏文件
- **检查项**：
  - 文件是否存在
  - 哈希是否匹配
  - Shim 是否指向正确版本
  - 关键文件是否可执行
- **自动模式**：后台定时检查（默认 7 天），发现损坏自动修复

### 5. 软件束（Bundle）

- **模块**：`hit-core/bundle/`
- **职责**：一键安装多个软件，适合团队环境
- **Bundle 清单格式**：

```json
{
  "name": "dev-environment",
  "description": "Python 开发环境",
  "version": "1.0.0",
  "packages": [
    {"name": "python", "version": "3.12.0", "required": true},
    {"name": "git", "version": "latest", "required": true},
    {"name": "vscode", "version": "stable", "required": false}
  ],
  "post_install": [
    "git config --global user.name 'Your Name'",
    "pip install -U pip setuptools"
  ]
}
```

- **命令**：
  - `hit bundle create <name>` - 从当前环境创建束
  - `hit bundle install <bundle>` - 安装束
  - `hit bundle list` - 列出已安装束
  - `hit bundle export <name>` - 导出为 JSON

### 6. 沙盒环境（Shadow）

- **模块**：`hit-core/shadow/`
- **职责**：创建隔离环境，多版本软件互不干扰
- **应用场景**：
  - Python 多项目依赖隔离（类似 virtualenv）
  - 测试不同版本软件
  - 安全沙盒运行未知软件
- **实现**：
  - 独立 persist 目录：`~/.hit/persist/shadow/<name>/`
  - 独立环境变量：仅沙盒内可见
  - 通过 `hit shadow exec <name> <cmd>` 进入沙盒
- **命令**：
  - `hit shadow create <name> --base <package> --version <ver>`
  - `hit shadow list`
  - `hit shadow exec <name> <command>`
  - `hit shadow delete <name>`

### 7. 镜像源管理（Mirror）

- **模块**：`hit-core/mirror/`
- **职责**：多镜像支持，自动选择最快源
- **功能**：
  - 内置镜像：清华、阿里、UCloud、官方
  - 速度测试：`hit mirror speedtest`
  - 自动切换：下载失败时自动切换镜像
  - 区域感知：根据地理位置选择最近镜像
- **配置**：

```json
{
  "mirrors": {
    "python": [
      {"name": "tuna", "url": "https://mirrors.tuna.tsinghua.edu.cn/python/", "priority": 1},
      {"name": "aliyun", "url": "https://mirrors.aliyun.com/python/", "priority": 2}
    ]
  },
  "default_mirror": "tuna"
}
```

### 8. 生命周期管理（Lifecycle）

- **模块**：`hit-core/lifecycle/`
- **职责**：软件全生命周期自动化管理
- **功能**：
  - **归档**：`hit archive <package>` - 将旧版本移至外部存储
  - **孤立文件清理**：`hit orphan list/clean` - 扫描无主文件
  - **去重**：`hit dedup` - 跨软件重复文件硬链接化，节省空间
  - **自动清理**：配置 `auto_cleanup_days`，自动删除 N 天未使用的版本

### 9. 运行时监控（Monitor）

- **模块**：`hit-core/monitor/`
- **职责**：跟踪软件运行状态，收集资源统计
- **功能**：
  - `hit top` - 实时显示软件资源占用（类似 top）
  - `hit ps <package>` - 查看软件相关进程树
  - `hit trace <package>` - 跟踪软件文件访问（需管理员）
  - 统计信息：平均 CPU、内存、I/O
  - 识别冗余软件（长期未使用）

### 10. 配置同步（Sync）

- **模块**：`hit-core/sync/`
- **职责**：跨设备同步配置和已安装列表
- **功能**：
  - `hit config export` - 导出配置到文件
  - `hit config import` - 从文件导入配置
  - 云同步：GitHub Gist、OneDrive、Dropbox
  - 选择性同步：排除大型软件
  - 冲突处理：时间戳优先或手动合并

### 11. 开发模式（Dev）

- **模块**：`hit-core/dev/`
- **职责**：支持从本地目录安装，适合开发者
- **功能**：
  - `hit dev install <local-path>` - 从本地目录安装（不下载）
  - `hit dev watch <package>` - 监听文件变化，自动重载
  - 自动检测文件修改，更新 Shim 指向
  - 适合调试自己开发的软件

### 12. 备份与恢复（Backup）

- **模块**：`hit-core/backup/`
- **职责**：提供配置和已安装软件的备份与恢复功能
- **功能**：
  - `hit backup create` - 创建完整备份
  - `hit backup restore <backup-file>` - 从备份恢复
  - `hit backup list` - 列出所有备份
  - **备份内容**：配置文件、已安装软件列表、Shim 配置
  - **备份位置**：本地文件系统或云存储
  - **增量备份**：仅备份变化的文件，节省空间

### 13. 增量更新（Delta Update）

- **模块**：`hit-core/delta/`
- **职责**：实现软件的增量更新，提高更新速度
- **功能**：
  - **差异计算**：比较新旧版本，生成差异包
  - **增量下载**：仅下载变化的部分
  - **补丁应用**：将差异包应用到当前版本
  - **回滚机制**：更新失败时回滚到原版本
- **支持格式**：ZIP、7z、TAR.GZ
- **优势**：减少下载量，提高更新速度，节省带宽

---

> 详见 [PROJECT_STRUCTURE.md](./PROJECT_STRUCTURE.md) — 项目结构与模块说明
> 详见 [TODO.md](./TODO.md) — 实现任务清单
