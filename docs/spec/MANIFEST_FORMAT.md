# Manifest 清单格式

Hit 完全兼容 Scoop Manifest 格式，并在此基础上扩展了增强字段。

## 基础结构（Scoop 兼容）

```json
{
  "name": "git",
  "version": "2.40.0",
  "description": "Distributed version control system",
  "homepage": "https://git-scm.com",
  "license": "GPL-2.0",

  "architecture": {
    "64bit": {
      "url": "https://github.com/git-for-windows/git/releases/download/v2.40.0.windows.1/PortableGit-2.40.0-64-bit.7z.exe",
      "hash": "sha256:abc123..."
    }
  },

  "bin": [
    "bin/git.exe",
    "bin/git-lfs.exe"
  ],

  "env_set": {
    "GIT_INSTALL_ROOT": "$dir"
  },

  "persist": [
    "etc/gitconfig",
    "share/git-core/templates"
  ],

  "checkver": {
    "github": "https://github.com/git-for-windows/git",
    "regex": "v([\\d.]+)\\.windows\\.1"
  },

  "autoupdate": {
    "architecture": {
      "64bit": {
        "url": "https://github.com/git-for-windows/git/releases/download/v$version.windows.1/PortableGit-$version-64-bit.7z.exe"
      }
    }
  }
}
```

## 扩展字段（Hit 新增）

```json
{
  "name": "python",
  "version": "3.12.0",
  "description": "Python programming language",

  "alias": ["py", "python3", "python3.12"],

  "bucket_priority": 50,
  "bucket_maintainer": "Python Official",
  "bucket_last_update": "2024-01-20T00:00:00Z",

  "dependencies": {
    "vc_redist": {"version": ">=14.0", "type": "runtime"},
    "openssl": {"version": "3.0", "type": "optional"}
  },

  "health_check": {
    "enabled": true,
    "interval_days": 7,
    "critical_files": ["python.exe", "DLLs/"],
    "verify_hash": true
  },

  "bundle": {
    "name": "dev-environment",
    "description": "Python development environment",
    "packages": [
      {"name": "python", "version": "3.12.0"},
      {"name": "git", "version": "2.40.0"},
      {"name": "vscode", "version": "1.85.0"}
    ]
  },

  "shadow": {
    "enabled": false,
    "persist_isolated": true,
    "env_isolated": ["PATH", "PYTHONPATH"]
  },

  "mirrors": [
    {"name": "official", "url": "https://www.python.org/ftp/python/"},
    {"name": "tuna", "url": "https://mirrors.tuna.tsinghua.edu.cn/python/"},
    {"name": "aliyun", "url": "https://mirrors.aliyun.com/python/"}
  ],

  "lifecycle": {
    "auto_archive": true,
    "keep_versions": 2,
    "dedup_enabled": true
  },

  "monitor": {
    "track_processes": true,
    "track_file_access": false,
    "stats_retention_days": 30
  },

  "dev": {
    "watch_paths": ["src/", "tests/"],
    "auto_reload": true
  }
}
```

## 字段说明

| 字段 | 类型 | 说明 | Scoop 兼容 |
|------|------|------|-----------|
| `name` | string | 软件名 | ✅ |
| `version` | string | 版本号 | ✅ |
| `description` | string | 描述 | ✅ |
| `homepage` | string | 主页 | ✅ |
| `license` | string | 许可证 | ✅ |
| `architecture` | object | 架构特定配置 | ✅ |
| `bin` | array | 可执行文件路径 | ✅ |
| `env_set` | object | 环境变量 | ✅ |
| `persist` | array | 持久化文件/目录 | ✅ |
| `checkver` | object | 版本检查配置 | ✅ |
| `autoupdate` | object | 自动更新 URL | ✅ |
| `alias` | array | 软件别名 | ❌ Hit 扩展 |
| `bucket_priority` | number | Bucket 优先级 | ❌ Hit 扩展 |
| `bucket_maintainer` | string | 维护者 | ❌ Hit 扩展 |
| `dependencies` | object | 依赖管理 | ❌ Hit 扩展 |
| `health_check` | object | 健康检查 | ❌ Hit 扩展 |
| `bundle` | object | 软件束定义 | ❌ Hit 扩展 |
| `shadow` | object | 沙盒配置 | ❌ Hit 扩展 |
| `mirrors` | array | 镜像源 | ❌ Hit 扩展 |
| `lifecycle` | object | 生命周期策略 | ❌ Hit 扩展 |
| `monitor` | object | 监控配置 | ❌ Hit 扩展 |
| `dev` | object | 开发模式 | ❌ Hit 扩展 |

---

> 详见 [PROJECT_STRUCTURE.md](../plan/PROJECT_STRUCTURE.md) — 项目结构与模块说明
