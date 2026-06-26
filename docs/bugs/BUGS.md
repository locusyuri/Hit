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