# Chapter 8 实验报告：死锁检测

## 实验信息
- **章节**: Chapter 8
- **实验内容**: 实现死锁检测机制
- **开始时间**: 2025-01-XX
- **完成时间**: 2025-01-XX

## 实验目标

实现死锁检测功能，当 mutex 和 semaphore 相关的系统调用可能引发死锁时，拒绝相应的资源获取请求并返回错误码 `-0xDEAD`。

## 实现内容

### 1. 数据结构扩展

在 `ProcessControlBlockInner` 中添加了死锁检测相关的数据结构：

```rust
/// deadlock detection switch
pub deadlock_detect: bool,
/// owner tid of each mutex
pub mutex_owner: Vec<Option<usize>>,
/// semaphore state tracking
pub semaphore_state: Vec<Option<SemaphoreState>>,
/// waiting mutex for each tid
pub wait_mutex_for_tid: Vec<Option<usize>>,
/// waiting semaphore for each tid
pub wait_sem_for_tid: Vec<Option<usize>>,
```

其中 `SemaphoreState` 结构体用于跟踪信号量的状态：

```rust
pub struct SemaphoreState {
    /// available resources
    pub available: isize,
    /// current holders and their counts
    pub holders: BTreeMap<usize, usize>,
}
```

### 2. 辅助方法实现

实现了以下辅助方法来管理跟踪数据结构：

- `ensure_tid_entry(tid: usize)`: 确保等待跟踪向量覆盖指定的 tid
- `ensure_mutex_entry(id: usize)`: 确保 mutex 跟踪覆盖指定的 id
- `ensure_sem_entry(id: usize, init_available: Option<isize>)`: 确保信号量跟踪覆盖指定的 id，并可选择初始化可用资源数

### 3. 死锁检测算法

实现了 `would_deadlock` 方法，使用深度优先搜索（DFS）检测是否存在循环等待：

```rust
pub fn would_deadlock(&self, from: usize, deps: &[usize]) -> bool {
    // 构建等待图
    // 1. 添加现有的 mutex 等待边
    // 2. 添加现有的 semaphore 等待边
    // 3. 添加提议的新边
    // 使用 DFS 检测是否存在环
}
```

算法思路：
1. 构建等待图：将线程和资源的关系建模为有向图
2. 对于 mutex：如果线程 A 等待 mutex M，而 M 被线程 B 持有，则添加边 A -> B
3. 对于 semaphore：如果线程 A 等待 semaphore S，而 S 的资源被线程 B 持有，则添加边 A -> B
4. 使用 DFS 检测从当前线程出发是否存在环，如果存在环则说明会发生死锁

### 4. 系统调用实现

#### 4.1 `sys_enable_deadlock_detect`

```rust
pub fn sys_enable_deadlock_detect(enabled: usize) -> isize {
    let process = current_process();
    let mut inner = process.inner_exclusive_access();
    match enabled {
        0 => {
            inner.deadlock_detect = false;
            0
        }
        1 => {
            inner.deadlock_detect = true;
            0
        }
        _ => -1,
    }
}
```

功能：启用或禁用当前进程的死锁检测功能。

#### 4.2 `sys_mutex_lock` 集成死锁检测

在获取 mutex 锁之前，如果启用了死锁检测：
1. 检查 mutex 是否已被其他线程持有
2. 如果被持有，检查获取该锁是否会导致死锁
3. 如果会导致死锁，返回 `-0xDEAD`
4. 否则记录等待关系，然后尝试获取锁
5. 获取锁成功后，更新 mutex 的所有者信息

#### 4.3 `sys_mutex_unlock` 更新状态

释放 mutex 时，清除所有者信息：

```rust
inner.mutex_owner[mutex_id] = None;
```

#### 4.4 `sys_semaphore_create` 初始化跟踪

创建信号量时，初始化跟踪状态：

```rust
process_inner.ensure_sem_entry(id, Some(res_count as isize));
```

#### 4.5 `sys_semaphore_down` 集成死锁检测

在获取信号量资源之前，如果启用了死锁检测：
1. 检查信号量是否有可用资源
2. 如果没有可用资源，检查当前持有资源的线程
3. 检查获取资源是否会导致死锁
4. 如果会导致死锁，返回 `-0xDEAD`
5. 否则记录等待关系，然后尝试获取资源
6. 获取资源成功后，更新信号量状态（减少可用资源，增加持有者计数）

#### 4.6 `sys_semaphore_up` 更新状态

释放信号量资源时，更新状态：
1. 增加可用资源数
2. 减少当前线程的持有计数

## 调试过程

### 问题1: 重复导入 Arc

**错误信息**:
```
error[E0252]: the name `Arc` is defined multiple times
```

**原因**: 在 `sync.rs` 中重复导入了 `Arc`。

**解决**: 删除重复的导入语句。

### 问题2: 模块可见性问题

**错误信息**:
```
error[E0603]: module `process` is private
```

**原因**: `SemaphoreState` 需要在 `syscall` 模块中使用，但 `process` 模块是私有的。

**解决**: 在 `task/mod.rs` 中导出 `SemaphoreState`：

```rust
pub use process::SemaphoreState;
```

### 问题3: 借用检查错误

**错误信息**:
```
error[E0502]: cannot borrow `process_inner` as immutable because it is also borrowed as mutable
```

**原因**: 在持有可变借用的情况下尝试不可变借用。

**解决**: 调整代码结构，先完成可变操作，释放借用后再进行不可变操作。

### 问题4: 缺少文档注释

**错误信息**:
```
error: missing documentation for a struct field
```

**原因**: `SemaphoreState` 结构体的字段缺少文档注释。

**解决**: 为字段添加文档注释：

```rust
pub struct SemaphoreState {
    /// available resources
    pub available: isize,
    /// current holders and their counts
    pub holders: BTreeMap<usize, usize>,
}
```

## 测试结果

### 测试用例

1. **ch8_deadlock_mutex1**: 测试 mutex 的自锁死锁检测
   - 预期：检测到死锁，返回 `-0xDEAD`
   - 结果：✓ 通过

2. **ch8_deadlock_sem1**: 测试 semaphore 的循环等待死锁检测
   - 预期：检测到死锁，至少有一个线程返回非0
   - 结果：✓ 通过

3. **ch8_deadlock_sem2**: 测试无死锁情况
   - 预期：未检测到死锁，所有线程正常完成
   - 结果：✓ 通过

4. **ch8_usertest**: 综合测试所有 ch8 相关功能
   - 结果：✓ 通过

## 实现细节

### 死锁检测时机

死锁检测在以下时机进行：
1. **mutex_lock**: 当 mutex 已被其他线程持有时，在尝试获取锁之前检测
2. **semaphore_down**: 当信号量没有可用资源时，在尝试获取资源之前检测

### 等待图构建

等待图的构建包括：
1. **Mutex 等待边**: `wait_mutex_for_tid[tid] -> mutex_owner[mid]`
2. **Semaphore 等待边**: `wait_sem_for_tid[tid] -> semaphore_state[sid].holders` 中的所有持有者
3. **提议边**: 当前请求的资源的所有者

### 环检测算法

使用 DFS 进行环检测：
- `visited`: 记录已访问的节点
- `stack`: 记录当前 DFS 路径上的节点
- 如果在 DFS 过程中遇到 `stack` 中的节点，说明存在环

## 总结

本次实验成功实现了死锁检测机制，主要包括：

1. ✅ 扩展了 `ProcessControlBlockInner` 数据结构，添加死锁检测相关的跟踪信息
2. ✅ 实现了 `would_deadlock` 方法，使用 DFS 算法检测循环等待
3. ✅ 在 `mutex_lock` 和 `semaphore_down` 中集成了死锁检测
4. ✅ 实现了 `sys_enable_deadlock_detect` 系统调用
5. ✅ 正确处理了资源获取和释放时的状态更新

通过本次实验，深入理解了死锁检测的原理和实现方法，掌握了如何通过等待图检测循环等待关系，从而预防死锁的发生。

## 代码统计

- 新增代码行数：约 200 行
- 修改文件：
  - `os/src/task/process.rs`: 添加死锁检测数据结构和算法
  - `os/src/syscall/sync.rs`: 集成死锁检测到系统调用
  - `os/src/task/mod.rs`: 导出 `SemaphoreState`
  - `os/src/syscall/thread.rs`: 修复 tid 跟踪问题

