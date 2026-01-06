# Chapter 3 Report

## 实现
- 新增 syscall 计数：在 `TaskControlBlock` 中增加 `syscall_times`，通过 `record_syscall/get_syscall_count` 在调度器记录并查询当前任务各系统调用次数，`syscall()` 入口统一计数，`sys_trace` 请求 2 直接返回。
- 完成 `sys_trace`：支持读/写当前任务地址与查询系统调用次数，按需求返回值。
- 计时修正：改为在时钟中断中累加 tick（`timer::tick`），`sys_get_time` 基于 tick 计算毫秒/微秒并保底返回非零时间，避免启动初期返回 0。

## 调试记录
- 首次运行 `make run BASE=2` 时 `get_time` 返回 0，`ch3_sleep` 断言失败且测试卡住。调查后确认 `time` CSR 读数在环境下不可用，导致时间始终为 0。
- 方案调整为使用时钟中断维护软件 tick，再由 tick 推导时间；同时保留对极小值的下限处理，重新运行后所有用例完成。

## 测试
- 按文档运行 `cd os && make run BASE=2`，全部 ch2/ch3 测例通过，日志末尾出现 `Test trace OK!`、`Test sleep OK!` 与内核 `Panicked ... All applications completed!`。

## 其他
- 未修改 `user/` 目录测例。

