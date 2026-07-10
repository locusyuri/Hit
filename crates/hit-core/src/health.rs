//! 健康检查模块
//!
//! 检查已安装软件的完整性（目录、junction、shim）并报告问题。

use std::path::PathBuf;

use hit_common::Session;

/// 健康检查问题类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IssueType {
    /// 应用目录不存在
    MissingAppDir,
    /// current junction 不存在
    MissingCurrent,
    /// current 不是有效 junction
    BrokenJunction,
    /// current 指向的版本目录不存在
    MissingVersion,
    /// db.json 有记录但 app 目录不存在（孤立记录）
    OrphanDbRecord,
    /// app 目录存在但 db.json 无记录（未跟踪）
    StaleDbRecord,
    /// .shim 文件指向不存在的 exe
    BrokenShim,
}

impl std::fmt::Display for IssueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingAppDir => write!(f, "应用目录不存在"),
            Self::MissingCurrent => write!(f, "current 链接不存在"),
            Self::BrokenJunction => write!(f, "current 链接损坏"),
            Self::MissingVersion => write!(f, "版本目录不存在"),
            Self::OrphanDbRecord => write!(f, "孤立数据库记录"),
            Self::StaleDbRecord => write!(f, "未跟踪的应用目录"),
            Self::BrokenShim => write!(f, "shim 目标不存在"),
        }
    }
}

/// 健康检查问题
#[derive(Debug, Clone)]
pub struct HealthIssue {
    /// 相关应用名
    pub app: String,
    /// 问题类型
    pub issue: IssueType,
    /// 相关路径
    pub path: PathBuf,
    /// 是否可自动修复
    pub fixable: bool,
}

/// 检查所有已安装应用的健康状态
pub fn check_installed_apps(session: &Session) -> Vec<HealthIssue> {
    let mut issues = Vec::new();
    let db = match hit_common::error::Result::ok(
        crate::store::Db::load(&crate::store::db_path(session)),
    ) {
        Some(db) => db,
        None => return issues,
    };

    let apps_path = session.apps_path();

    // 检查 db.json 中的每个已安装包
    for name in db.list_packages().keys() {
        let app_dir = apps_path.join(name);

        // 检查应用目录
        if !app_dir.exists() {
            issues.push(HealthIssue {
                app: name.clone(),
                issue: IssueType::MissingAppDir,
                path: app_dir,
                fixable: true,
            });
            continue;
        }

        // 检查 current 链接
        let current = app_dir.join("current");
        if !current.exists() {
            issues.push(HealthIssue {
                app: name.clone(),
                issue: IssueType::MissingCurrent,
                path: current,
                fixable: true,
            });
            continue;
        }

        // 检查 current 是否为有效 junction
        if !is_valid_junction(&current) {
            issues.push(HealthIssue {
                app: name.clone(),
                issue: IssueType::BrokenJunction,
                path: current,
                fixable: true,
            });
            continue;
        }

        // 检查 current 指向的版本目录是否存在
        let target = std::fs::read_link(&current).unwrap_or_default();
        if !target.exists() {
            issues.push(HealthIssue {
                app: name.clone(),
                issue: IssueType::MissingVersion,
                path: target,
                fixable: false,
            });
        }
    }

    // 检查孤立记录（目录存在但 db.json 无记录）
    if let Ok(entries) = std::fs::read_dir(apps_path) {
        for entry in entries.flatten() {
            if !entry.path().is_dir() {
                continue;
            }
            let dir_name = entry.file_name().to_string_lossy().into_owned();
            if !db.is_installed(&dir_name) {
                issues.push(HealthIssue {
                    app: dir_name,
                    issue: IssueType::StaleDbRecord,
                    path: entry.path(),
                    fixable: false,
                });
            }
        }
    }

    issues
}

/// 检查所有已安装应用的健康问题（不含 shim 检查）
pub fn check_all(session: &Session) -> Vec<HealthIssue> {
    check_installed_apps(session)
}

/// 检查路径是否为有效的 junction（通过 read_link 测试）
fn is_valid_junction(path: &std::path::Path) -> bool {
    std::fs::read_link(path).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use hit_common::config::HitConfig;

    fn test_session(dir: &std::path::Path) -> Session {
        let config = HitConfig {
            root_path: Some(dir.to_string_lossy().into()),
            ..Default::default()
        };
        Session::with_config(config)
    }

    #[test]
    fn check_installed_apps_empty_db() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("buckets").join("main")).unwrap();
        let session = test_session(dir.path());
        let issues = check_installed_apps(&session);
        assert!(issues.is_empty());
    }

    #[test]
    fn check_detects_orphan_db_record() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("buckets").join("main")).unwrap();
        let session = test_session(dir.path());

        // 创建 app 目录但无 db 记录
        std::fs::create_dir_all(session.apps_path().join("orphan_app")).unwrap();

        let issues = check_installed_apps(&session);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|i| i.issue == IssueType::StaleDbRecord));
    }

    #[test]
    fn check_shims_empty() {
        // shim 检查已移至 hit-cli/doctor.rs，此处仅验证 health 模块编译
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("buckets").join("main")).unwrap();
        let session = test_session(dir.path());
        let issues = check_installed_apps(&session);
        assert!(issues.is_empty());
    }
}
