# chapter5练习¶

> **章节**: 第 5 章
> 
> **源文件**: 4exercise.html

---

Toggle Light / Dark / Auto color theme

Toggle table of contents sidebar

__

# chapter5练习

## 编程作业

### 关于之前的 syscall

你仍需要迁移上一章的 `sys_get_time` `sys_mmap` `sys_munmap` 以适应新的进程结构。不过，**从本章节开始，不再要求维护 ``sys_trace`` 这一系统调用** 。

### 进程创建

大家一定好奇过为啥进程创建要用 fork + exec 这么一个奇怪的系统调用，就不能直接搞一个新进程吗？ 思而不学则殆，我们就来试一试！这章的编程练习请大家实现一个完全 DIY 的系统调用 spawn，用以创建一个新进程。

spawn 系统调用定义( [标准spawn看这里](https://man7.org/linux/man-pages/man3/posix_spawn.3.html) )：
    
    
    fn sys_spawn(path: *const u8) -> isize
    

  * syscall ID: 400

  * 功能：新建子进程，使其执行目标程序。

  * 说明：成功返回子进程id，否则返回 -1。

  * 可能的错误：
    
    * 无效的文件名。

小心

虽然测例很简单，但提醒读者 spawn **不必** 像 fork 一样复制父进程的地址空间。

### stride 调度算法

ch3 中我们实现的调度算法十分简单。现在我们要为我们的 os 实现一种带优先级的调度算法：stride 调度算法。

算法描述如下:

(1) 为每个进程设置一个当前 stride，表示该进程当前已经运行的“长度”。另外设置其对应的 pass 值（只与进程的优先权有关系），表示对应进程在调度后，stride 需要进行的累加值。

  2. 每次需要调度时，从当前 runnable 态的进程中选择 stride 最小的进程调度。对于获得调度的进程 P，将对应的 stride 加上其对应的步长 pass。

  3. 一个时间片后，回到上一步骤，重新调度当前 stride 最小的进程。

可以证明，如果令 P.pass = BigStride / P.priority 其中 P.priority 表示进程的优先权（大于 1），而 BigStride 表示一个预先定义的大常数，则该调度方案为每个进程分配的时间将与其优先级成正比。证明过程我们在这里略去，有兴趣的同学可以在网上查找相关资料。

其他实验细节：

  * stride 调度要求进程优先级 \\(\geq 2\\)，所以设定进程优先级 \\(\leq 1\\) 会导致错误。

  * 进程初始 stride 设置为 0 即可。

  * 进程初始优先级设置为 16。

为了实现该调度算法，内核还要增加 set_prio 系统调用
    
    
    // syscall ID：140
    // 设置当前进程优先级为 prio
    // 参数：prio 进程优先级，要求 prio >= 2
    // 返回值：如果输入合法则返回 prio，否则返回 -1
    fn sys_set_priority(prio: isize) -> isize;
    

提示

  * 你可以在TCB加入新的字段来支持优先级等。

  * 为了减少整数除的误差，BIG_STRIDE 一般需要很大，但为了不至于发生反转现象（详见问答作业），或许选择一个适中的数即可，当然能进行溢出处理就更好了。

  * stride 算法要找到 stride 最小的进程，使用优先级队列是效率不错的办法，但是我们的实验测例很简单，所以效率完全不是问题。事实上，很推荐使用暴力扫一遍的办法找最小值。

  * 注意设置进程的初始优先级。

注意

为了让大家能在本编程作业中使用 `Vec` 等数据结构，我们利用第三方库 `buddy_system_allocator` 为大家实现了堆内存分配器，相关代码位于 `mm/heap_allocator` 模块。

背景知识： [Rust 中的动态内存分配](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter4/1rust-dynamic-allocation.html)

### 实验要求

  * 实现分支：ch5。

  * 实验目录请参考 ch3。注意在reports中放入lab1-3的所有报告。

  * 通过所有测例。

在 os 目录下 `make run BASE=2` 加载所有测例， `ch5_usertest` 打包了所有你需要通过的测例。 `ch5_stride` 检查 stride 调度算法是否满足公平性要求，六个子程序运行的次数应该大致与其优先级呈正比，测试通过标准是 \\(\max{\frac{runtimes}{prio}}/ \min{\frac{runtimes}{prio}} < 1.5\\).


从本章开始，你的内核必须前向兼容，能通过前一章的所有测例。
