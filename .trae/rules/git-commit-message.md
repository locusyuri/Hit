---
alwaysApply: true
scene: git_message
---

请使用中文编写提交信息, 并遵循 Conventional Commits 规范:

结构为: `<type>(<scope>): <description>`，可选 body/footer。
- type 枚举: feat/fix/docs/style/refactor/perf/test/chore/build/ci/revert。
- 规则: 现在时、小写开头、无句号。
- footer 用于 BREAKING CHANGE 或关闭 Issue。

示例:
- feat: 实现用户头像上传
- fix(auth): 解决登录后重定向失败，通过 localStorage 存储 returnUrl。Closes #1024
- feat(api): 重构用户信息接口。BREAKING CHANGE: /v1/user/profile 废弃，迁移至 /v2/user/info
