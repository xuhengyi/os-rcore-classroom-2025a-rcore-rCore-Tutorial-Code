## ch5 实验报告（spawn 与 stride 调度）

- 开始时间：2026-01-06（切换 `ch5` 分支并启动实现）
- 结束时间：2026-01-06（`make run BASE=2` 所有 ch5 官方测例通过）
- 分支基线：`origin/ch5`

### 实现内容
- `sys_get_time`：使用 `translated_refmut` 写回跨页 `TimeVal`，避免非法访存。
- `sys_mmap/sys_munmap`：校验页对齐与 `prot` 合法性；检查区间空闲后按 prot 映射；支持区间卸载并验证已映射，否则返回 -1。
- `spawn`：解析用户态字符串，按文件名加载 ELF，新建 TCB，设置 parent/children，并加入调度队列。
- `set_priority`：校验 `prio>=2`，更新当前任务优先级。
- stride 调度：在 TCB 记录 `priority/stride`，调度时遍历 ready 队列选取最小 stride，`pass = BIG_STRIDE/priority`，运行后累加 stride，实现时间片比例控制。

### 调试与问题
- 缺失类型导入导致编译失败：在 `sys_spawn` 补充 `TaskControlBlock` 引用。
- 早期 mmap 重用缺少区间检测：补充 `is_range_free/unmap_range`，避免覆盖或卸载未映射页。
- 优先级为 0/1 时导致除零：取 `max(prio,2)` 计算 pass，且 `set_priority` 对不合法返回 -1。

### 测试
- 命令：`cd os && make run BASE=2`
- 结果：`ch5_usertest` 及 `ch5_stride` 全部通过，且前置 ch2–ch4 测例在同次运行中均通过，最终输出 “ch5 Usertests passed!”。

### 后续改进
- stride 采用遍历 ready 队列，测例规模足够；若扩展可使用优先队列降复杂度。
- mmap/munmap 失败不回收分配的物理页按文档可忽略，如需健壮性可补充回滚。

