## ch4 实验报告（虚存与 mmap/munmap）

- 开始时间：2026-01-06（进入 `ch4` 分支并启动实现）
- 结束时间：2026-01-06（完成 `make run BASE=2` 全部测例通过）
- 分支基线：`origin/ch4`

### 实现内容
- `sys_get_time`：使用用户页表跨页拷贝 `TimeVal`，避免跨页/非法地址导致的访存错误。
- `sys_trace`：按照 U 权限检查读/写，读返回用户字节，写验证可写后修改；`Syscall` 统计基于每任务的调用计数。
- `sys_mmap`/`sys_munmap`：校验页对齐、`prot` 合法性及非空权限；检查映射区间冲突；创建/撤销映射时使用用户页表，权限位根据 `prot` 生成；错误场景返回 -1。
- 辅助：在 TCB 中增加 syscall 计数数组；在 `MemorySet` 中提供区间空闲检测与区间解除映射（用于 mmap/munmap 冲突检查）。

### 调试与问题
- mmap 权限与 PTE 匹配：确保 prot 的 R/W/X 分别映射到 `MapPermission`，并强制包含 `U`。
- 区间冲突：初版未检测已映射页，导致覆盖；增加逐页检查保证全空闲后才建图。
- trace 访问非法地址：通过 `translate` + PTE 校验 `V|U` 并检查读/写位，非法返回 -1。
- 跨页 get_time：通过 `translated_byte_buffer` 切片写入，避免跨页 panic。

### 测试
- 命令：`cd os && make run BASE=2`
- 结果：通过 ch4 全量官方测例（含 `trace_1`、`04_1~04_6`、`sbrk`、`sleep` 等），输出与文档期望一致，内核最终 panic 于 “All applications completed!” 符合用例结束语义。

### 后续改进
- mmap 失败后的回收目前按文档可忽略，如需健壮性可补全回收路径。
- syscall 统计未持久化到 ch5+ 分支，如需复用需在后续分支手动迁移。

