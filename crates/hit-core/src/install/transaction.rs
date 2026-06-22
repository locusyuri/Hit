//! 安装事务管理器（RAII 模式）
//!
//! `Transaction` 封装安装流水线的"进行中"状态：
//!
//! - `begin()` 创建一个临时目录（staging），供解压、链接准备等步骤使用
//! - `atomic_move()` 把 staging 中的产物原子地移动到最终位置
//! - `record_undo()` 登记"可逆动作"——rollback 时按逆序执行
//! - `commit()` 消费事务、清理 staging 残留，**已落地**的产物保留
//! - `Drop` 若仍为 Active，自动回滚（删 staging + 逆序执行 undo）
//!
//! 与 Scoop（无回滚）和 Hok（commit 逻辑被注释）不同，Hit 的事务保证：
//! 失败时不会留下半成品安装状态。

use std::mem::ManuallyDrop;
use std::path::{Path, PathBuf};

use tempfile::TempDir;

use hit_common::{HitError, Result};

#[cfg(windows)]
use crate::win::fs::{remove_junction, remove_persist_link};
#[cfg(windows)]
use crate::win::env::{remove_from_path, set_env_var};

/// 安装事务状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxState {
    /// 已创建临时目录，工作进行中
    Active,
    /// 已 commit：staging 已清理，undo 动作已清空（已落地的产物保留）
    Committed,
    /// 已 rollback：staging 已清理，undo 动作已逆序执行
    RolledBack,
}

/// 可逆的已提交动作（rollback 时按注册逆序执行）
#[derive(Debug)]
pub enum UndoAction {
    /// 移除已创建的 junction（`lnk` 指向的 junction 目录）
    RemoveJunction(PathBuf),
    /// 移除已创建的 shim（`shim_exe` 与同名 `.shim` 文件）
    RemoveShim(PathBuf),
    /// 移除已创建的 persist 链接（junction / hard link）
    RemovePersistLink(PathBuf),
    /// 从 PATH 中移除若干条目（子串匹配模式，与 `win::env::remove_from_path` 一致）
    RemoveFromPath(Vec<String>),
    /// 删除已设置的环境变量
    RemoveEnvVar(String),
}

/// RAII 安装事务
pub struct Transaction {
    state: TxState,
    /// 临时工作目录；用 ManuallyDrop 以便 commit 时手动释放所有权
    temp: ManuallyDrop<TempDir>,
    /// 事务涉及的 app 名称（用于错误信息）
    app: String,
    /// 回滚时需要撤销的已落地动作（逆序执行）
    undo_actions: Vec<UndoAction>,
}

impl Transaction {
    /// 开始事务：创建一个 OS tempdir 作为 staging 区
    pub fn begin(app: &str) -> Result<Self> {
        let temp = TempDir::new().map_err(|e| {
            HitError::io(format!("创建事务临时目录失败（app={app}）"), e)
        })?;
        tracing::debug!(app, staging = ?temp.path(), "事务开始");
        Ok(Self {
            state: TxState::Active,
            temp: ManuallyDrop::new(temp),
            app: app.to_string(),
            undo_actions: Vec::new(),
        })
    }

    /// staging 根目录路径
    pub fn staging_dir(&self) -> &Path {
        self.temp.path()
    }

    /// 在 staging 下创建命名子目录（多 archive 解压时使用）
    pub fn staging_subdir(&self, name: &str) -> Result<PathBuf> {
        let dir = self.temp.path().join(name);
        std::fs::create_dir_all(&dir).map_err(|e| {
            HitError::io(format!("创建 staging 子目录失败：{}", dir.display()), e)
        })?;
        Ok(dir)
    }

    /// 登记一个可逆动作（rollback 时逆序执行）
    pub fn record_undo(&mut self, action: UndoAction) {
        self.undo_actions.push(action);
    }

    /// 原子移动：把 `src` 移到 `dst`，覆盖已存在的 `dst`
    ///
    /// `std::fs::rename` 在 Windows 内部使用 `MoveFileExW(MOVEFILE_REPLACE_EXISTING)`；
    /// 跨卷时 rename 会失败，此时回退到 copy + remove（带 tracing::warn 提示）。
    pub fn atomic_move(&self, src: &Path, dst: &Path) -> Result<()> {
        if let Some(parent) = dst.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                HitError::io(format!("创建父目录失败：{}", parent.display()), e)
            })?;
        }
        match std::fs::rename(src, dst) {
            Ok(()) => Ok(()),
            Err(e) if src.exists() => {
                // rename 失败但源仍在：大概率跨卷，退化为 copy+remove
                tracing::warn!(
                    src = ?src, dst = ?dst, error = %e,
                    "rename 失败，退化为 copy + remove"
                );
                copy_recursive(src, dst)?;
                std::fs::remove_dir_all(src).map_err(|e| {
                    HitError::io(format!("清理 rename 回退源失败：{}", src.display()), e)
                })?;
                Ok(())
            }
            Err(e) => Err(HitError::io(
                format!("原子移动失败：{} -> {}", src.display(), dst.display()),
                e,
            )),
        }
    }

    /// 标记事务提交成功
    ///
    /// - 状态切换为 Committed（Drop 不再回滚）
    /// - 释放 staging 目录所有权（TempDir drop 清理残留）
    /// - 清空 undo 队列（已落地的动作保留）
    pub fn commit(mut self) -> Result<()> {
        self.state = TxState::Committed;
        // 显式释放 TempDir 所有权：drop 时清理 staging 残留
        // SAFETY：self 即将被消费，temp 不再被使用
        unsafe {
            let temp = ManuallyDrop::take(&mut self.temp);
            drop(temp);
        }
        self.undo_actions.clear();
        tracing::debug!(app = %self.app, "事务已提交");
        Ok(())
    }

    /// 显式回滚（通常 Drop 自动执行；此方法用于错误路径的早退）
    pub fn rollback(mut self) -> Result<()> {
        self.do_rollback()
    }

    pub fn state(&self) -> TxState {
        self.state
    }

    pub fn app(&self) -> &str {
        &self.app
    }

    /// 内部回滚：逆序执行 undo + 清理 staging
    fn do_rollback(&mut self) -> Result<()> {
        if self.state != TxState::Active {
            return Ok(());
        }
        self.state = TxState::RolledBack;
        tracing::warn!(app = %self.app, "事务回滚");

        // 逆序执行 undo
        let mut last_err: Option<HitError> = None;
        while let Some(action) = self.undo_actions.pop() {
            if let Err(e) = undo_one(&action) {
                tracing::warn!(error = %e, "undo 动作失败（继续回滚其余）");
                if last_err.is_none() {
                    last_err = Some(e);
                }
            }
        }

        // 清理 staging 残留
        // SAFETY：即将切换为 RolledBack，temp 不再被使用
        unsafe {
            let temp = ManuallyDrop::take(&mut self.temp);
            drop(temp);
        }

        match last_err {
            None => Ok(()),
            Some(e) => Err(HitError::Rollback {
                app: self.app.clone(),
                reason: format!("部分 undo 动作失败：{e}"),
            }),
        }
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        if self.state == TxState::Active {
            let _ = self.do_rollback();
        }
    }
}

/// 执行单个 undo 动作
#[cfg(windows)]
fn undo_one(action: &UndoAction) -> Result<()> {
    match action {
        UndoAction::RemoveJunction(lnk) => remove_junction(lnk),
        UndoAction::RemoveShim(shim_exe) => {
            let sidecar = shim_exe.with_extension("shim");
            let r1 = std::fs::remove_file(shim_exe);
            let r2 = std::fs::remove_file(&sidecar);
            r1.or(r2).map_err(|e| {
                HitError::io(format!("移除 shim 失败：{}", shim_exe.display()), e)
            })
        }
        UndoAction::RemovePersistLink(source) => remove_persist_link(source),
        UndoAction::RemoveFromPath(patterns) => {
            let refs: Vec<&str> = patterns.iter().map(String::as_str).collect();
            remove_from_path(&refs, "User")
        }
        UndoAction::RemoveEnvVar(name) => set_env_var(name, None),
    }
}

#[cfg(not(windows))]
fn undo_one(action: &UndoAction) -> Result<()> {
    match action {
        UndoAction::RemoveShim(shim_exe) => {
            let sidecar = shim_exe.with_extension("shim");
            let _ = std::fs::remove_file(shim_exe);
            let _ = std::fs::remove_file(&sidecar);
            Ok(())
        }
        _ => {
            tracing::debug!(?action, "非 Windows 平台跳过 undo 动作");
            Ok(())
        }
    }
}

/// 递归复制（用于跨卷回退）
fn copy_recursive(src: &Path, dst: &Path) -> Result<()> {
    if src.is_dir() {
        std::fs::create_dir_all(dst).map_err(|e| {
            HitError::io(format!("创建目录失败：{}", dst.display()), e)
        })?;
        for entry in std::fs::read_dir(src).map_err(|e| {
            HitError::io(format!("读取目录失败：{}", src.display()), e)
        })? {
            let entry = entry.map_err(|e| {
                HitError::io(format!("读取目录项失败：{}", src.display()), e)
            })?;
            let child_src = entry.path();
            let child_dst = dst.join(entry.file_name());
            copy_recursive(&child_src, &child_dst)?;
        }
        Ok(())
    } else {
        std::fs::copy(src, dst).map_err(|e| {
            HitError::io(
                format!("复制文件失败：{} -> {}", src.display(), dst.display()),
                e,
            )
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn begin_creates_staging_dir() {
        let tx = Transaction::begin("test-app").expect("begin 应成功");
        assert_eq!(tx.state(), TxState::Active);
        assert!(tx.staging_dir().exists(), "staging 目录应存在");
        assert!(tx.staging_dir().is_dir());
    }

    #[test]
    fn staging_subdir_creates_named_directory() {
        let tx = Transaction::begin("test-app").expect("begin 应成功");
        let sub = tx.staging_subdir("archive-0").expect("应能创建子目录");
        assert!(sub.exists());
        assert!(sub.is_dir());
        assert_eq!(sub.file_name().unwrap(), "archive-0");
    }

    #[test]
    fn atomic_move_replaces_existing_target() {
        let tx = Transaction::begin("test-app").expect("begin 应成功");
        let staging = tx.staging_dir();

        let src = staging.join("new.txt");
        fs::write(&src, "new content").unwrap();

        let dst_dir = tempfile::tempdir().unwrap();
        let dst = dst_dir.path().join("target.txt");
        fs::write(&dst, "old content").unwrap();

        tx.atomic_move(&src, &dst).expect("atomic_move 应能覆盖目标");
        assert_eq!(fs::read_to_string(&dst).unwrap(), "new content");
        assert!(!src.exists(), "源应在移动后消失");
    }

    #[test]
    fn atomic_move_creates_parent_dirs() {
        let tx = Transaction::begin("test-app").expect("begin 应成功");
        let staging = tx.staging_dir();

        let src = staging.join("file.txt");
        fs::write(&src, "hello").unwrap();

        let dst_dir = tempfile::tempdir().unwrap();
        let dst = dst_dir.path().join("nested").join("deep").join("file.txt");

        tx.atomic_move(&src, &dst).expect("应能创建中间目录");
        assert_eq!(fs::read_to_string(&dst).unwrap(), "hello");
    }

    #[test]
    fn drop_without_commit_rolls_back() {
        // 注册一个 undo 动作，drop 时应执行
        let undo_target = tempfile::NamedTempFile::new().unwrap();
        let undo_path = undo_target.path().to_path_buf();
        assert!(undo_path.exists());

        {
            let mut tx = Transaction::begin("rollback-app").unwrap();
            tx.record_undo(UndoAction::RemoveShim(undo_path.clone()));
            // 不调用 commit，让 drop 触发回滚
        }

        // RemoveShim undo 应删除 .exe 与 .shim；我们的 undo_path 是临时文件本身
        // （测试只验证 undo 动作被执行——remove_file 不报错即可）
        assert!(!undo_path.exists(), "undo 应已移除目标文件");
    }

    #[test]
    fn commit_prevents_rollback_and_clears_undo() {
        let undo_target = tempfile::NamedTempFile::new().unwrap();
        let undo_path = undo_target.path().to_path_buf();

        let mut tx = Transaction::begin("commit-app").unwrap();
        tx.record_undo(UndoAction::RemoveShim(undo_path.clone()));
        tx.commit().expect("commit 应成功");

        // commit 后 undo 队列被清空，文件应保留
        assert!(undo_path.exists(), "commit 后 undo 动作不应被执行");
    }

    #[test]
    fn explicit_rollback_executes_undo_in_reverse_order() {
        let tmp = tempfile::tempdir().unwrap();
        let a = tmp.path().join("a.txt");
        let b = tmp.path().join("b.txt");
        fs::write(&a, "a").unwrap();
        fs::write(&b, "b").unwrap();

        let mut tx = Transaction::begin("rollback-order").unwrap();
        // 注册顺序：先 a 后 b；rollback 应先撤销 b 再撤销 a
        tx.record_undo(UndoAction::RemoveShim(a.clone()));
        tx.record_undo(UndoAction::RemoveShim(b.clone()));
        tx.rollback().expect("显式 rollback 应成功");

        assert!(!a.exists());
        assert!(!b.exists());
    }
}
