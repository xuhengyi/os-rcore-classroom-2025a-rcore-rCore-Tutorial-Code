# chapter4练习¶

> **章节**: 第 4 章
> 
> **源文件**: 7exercise.html

---

Toggle Light / Dark / Auto color theme

Toggle table of contents sidebar

__

# chapter4练习

## 编程作业

### 重写 sys_get_time 和 sys_trace

引入虚存机制后，原来内核的 `sys_get_time` 和 `sys_trace` 函数实现就无效了。**请你重写这两个系统调用的代码** ，恢复其正常功能。

此外，由于本章我们有了地址空间作为隔离机制，所以 `sys_trace` **需要考虑一些额外的情况** ：

  * 在读取（`trace_request` 为 0）时，如果对应地址用户不可见或不可读，则返回值应为 -1（`isize` 格式的 -1，而非 `u8`）。

  * 在写入（`trace_request` 为 1）时，如果对应地址用户不可见或不可写，则返回值应为 -1（`isize` 格式的 -1，而非 `u8`）。

### mmap 和 munmap 匿名映射

[mmap](https://man7.org/linux/man-pages/man2/mmap.2.html) 在 Linux 中主要用于在内存中映射文件， 本次实验简化它的功能，仅用于申请内存。

请实现 mmap 和 munmap 系统调用，mmap 定义如下：
    
    
    fn sys_mmap(start: usize, len: usize, prot: usize) -> isize
    

  * syscall ID：222

  * 申请长度为 len 字节的物理内存（不要求实际物理内存位置，可以随便找一块），将其映射到 start 开始的虚存，内存页属性为 prot

  * 参数：
    
    * start 需要映射的虚存起始地址，要求按页对齐

    * len 映射字节长度，可以为 0

    * prot：第 0 位表示是否可读，第 1 位表示是否可写，第 2 位表示是否可执行。其他位无效且必须为 0

  * 返回值：执行成功则返回 0，错误返回 -1

  * 说明：
    
    * 为了简单，目标虚存区间要求按页对齐，len 可直接按页向上取整，不考虑分配失败时的页回收。

  * 可能的错误：
    
    * start 没有按页大小对齐

    * prot & !0x7 != 0 (prot 其余位必须为0)

    * prot & 0x7 = 0 (这样的内存无意义)

    * [start, start + len) 中存在已经被映射的页

    * 物理内存不足

munmap 定义如下：
    
    
    fn sys_munmap(start: usize, len: usize) -> isize
    

  * syscall ID：215

  * 取消到 [start, start + len) 虚存的映射

  * 参数和返回值请参考 mmap

  * 说明：
    
    * 为了简单，参数错误时不考虑内存的恢复和回收。

  * 可能的错误：
    
    * [start, start + len) 中存在未被映射的虚存。

提示

  * 一定要注意 mmap 是的页表项，注意 riscv 页表项的格式与 prot 的区别。

  * 你增加 PTE_U 了吗？

### 实验要求

  * 实现分支：ch4。

  * 实现 mmap 和 munmap 两个系统调用，通过所有测例。

  * 实验目录请参考 ch3，报告命名 lab2.md/pdf

小心

注意 prot 参数的语义，它与内核定义的 MapPermission 有明显不同！


运行命令：
```
cd os
make run BASE=2
```
预期输出：

```
[rustsbi] RustSBI version 0.3.0-alpha.2, adapting to RISC-V SBI v1.0.0
.______       __    __      _______.___________.  _______..______   __
|   _  \     |  |  |  |    /       |           | /       ||   _  \ |  |
|  |_)  |    |  |  |  |   |   (----`---|  |----`|   (----`|  |_)  ||  |
|      /     |  |  |  |    \   \       |  |      \   \    |   _  < |  |
|  |\  \----.|  `--'  |.----)   |      |  |  .----)   |   |  |_)  ||  |
| _| `._____| \______/ |_______/       |__|  |_______/    |______/ |__|
[rustsbi] Implementation     : RustSBI-QEMU Version 0.2.0-alpha.2
[rustsbi] Platform Name      : riscv-virtio,qemu
[rustsbi] Platform SMP       : 1
[rustsbi] Platform Memory    : 0x80000000..0x88000000
[rustsbi] Boot HART          : 0
[rustsbi] Device Tree Region : 0x87000000..0x87000ef2
[rustsbi] Firmware Address   : 0x80000000
[rustsbi] Supervisor Address : 0x80200000
[rustsbi] pmp01: 0x00000000..0x80000000 (-wr)
[rustsbi] pmp02: 0x80000000..0x80200000 (---)
[rustsbi] pmp03: 0x80200000..0x88000000 (xwr)
[rustsbi] pmp04: 0x88000000..0x00000000 (-wr)
[kernel] Hello, world!
[kernel] back to world!
remap_test passed!
init TASK_MANAGER
num_app = 21
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x3a4, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
Hello, world from user mode program!
power_3 [10000/200000]
power_3 [20000/200000]
power_3 [30000/200000]
power_3 [40000/200000]
power_3 [50000/200000]
power_3 [60000/200000]
power_3 [70000/200000]
power_3 [80000/200000]
power_3 [90000/200000]
power_3 [100000/200000]
power_3 [110000/200000]
power_3 [120000/200000]
power_5 [10000/140000]
power_5 [20000/140000]
power_5 [30000/140000]
power_5 [40000/140000]
power_5 [50000/140000]
power_5 [60000/140000]
power_5 [70000/140000]
power_5 [80000/140000]
power_5 [90000/140000]
power_5 [100000/140000]
power_5 [110000/140000]
power_5 [120000/140000]
power_5 [130000/140000]
power_5 [140000/140000]
5^140000 = 386471875(MOD 998244353)
Test power_5 OK!
power_7 [10000/160000]
power_7 [20000/160000]
power_7 [30000/160000]
power_7 [40000/160000]
power_7 [50000/160000]
power_7 [60000/160000]
power_7 [70000/160000]
power_7 [80000/160000]
power_7 [90000/160000]
power_7 [100000/160000]
power_7 [110000/160000]
power_7 [120000/160000]
power_7 [130000/160000]
power_7 [140000/160000]
power_7 [150000/160000]
power_7 [160000/160000]
7^160000 = 667897727(MOD 998244353)
Test power_7 OK!
get_time OK! 46
current time_msec = 47
AAAAAAAAAA [1/5]
BBBBBBBBBB [1/5]
CCCCCCCCCC [1/5]
Test 04_1 OK!
[kernel] PageFault in application, bad addr = 0x10000000, bad instruction = 0x42e, kernel killed it.
[kernel] PageFault in application, bad addr = 0x10000000, bad instruction = 0x42c, kernel killed it.
Test 04_4 test OK!
Test trace_1 OK!
Test 04_5 ummap OK!
Test 04_6 ummap2 OK!
Test sbrk start.
origin break point = c000
power_3 [130000/200000]
power_3 [140000/200000]
power_3 [150000/200000]
power_3 [160000/200000]
power_3 [170000/200000]
power_3 [180000/200000]
power_3 [190000/200000]
power_3 [200000/200000]
3^200000 = 871008973(MOD 998244353)
Test power_3 OK!
AAAAAAAAAA [2/5]
BBBBBBBBBB [2/5]
CCCCCCCCCC [2/5]
one page allocated,  break point = d000
try write to allocated page
write ok
10 page allocated,  break point = 17000
11 page DEALLOCATED,  break point = c000
try DEALLOCATED more one page, should be failed.
Test sbrk almost OK!
now write to deallocated page, should cause page fault.
[kernel] PageFault in application, bad addr = 0xc000, bad instruction = 0x5c0, kernel killed it.
AAAAAAAAAA [3/5]
BBBBBBBBBB [3/5]
CCCCCCCCCC [3/5]
AAAAAAAAAA [4/5]
BBBBBBBBBB [4/5]
CCCCCCCCCC [4/5]
AAAAAAAAAA [5/5]
BBBBBBBBBB [5/5]
CCCCCCCCCC [5/5]
Test write A OK!
Test write B OK!
Test write C OK!
time_msec = 147 after sleeping 100 ticks, delta = 100ms!
Test sleep1 passed!
string from task trace test

Test trace OK!
Test sleep OK!
[kernel] Panicked at src/task/mod.rs:156 All applications completed!
```
