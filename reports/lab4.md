# Chapter 6 实验报告

## 实验信息
- **章节**: Chapter 6 - 文件系统与I/O重定向
- **分支**: ch6
- **开始时间**: 2025-01-XX
- **结束时间**: 2025-01-XX

## 实验目标
实现硬链接相关的三个系统调用：
1. `sys_linkat` - 创建硬链接
2. `sys_unlinkat` - 删除硬链接
3. `sys_fstat` - 获取文件状态信息

## 实现内容

### 1. 文件系统层修改 (easy-fs)

#### 1.1 添加 nlink 字段到 DiskInode
在 `easy-fs/src/layout.rs` 中的 `DiskInode` 结构体添加 `nlink` 字段用于记录硬链接计数。

#### 1.2 实现 link 方法
在 `easy-fs/src/vfs.rs` 中实现 `Inode::link()` 方法：
- 在父目录中创建新的目录项，指向同一个 inode
- 增加目标 inode 的 nlink 计数
- 检查是否链接同名文件（返回错误）

#### 1.3 实现 unlink 方法
在 `easy-fs/src/vfs.rs` 中实现 `Inode::unlink()` 方法：
- 从父目录中删除目录项
- 减少 inode 的 nlink 计数
- 当 nlink 为 0 时，回收 inode 和数据块

#### 1.4 实现 get_inode_id 方法
添加 `get_inode_id()` 方法用于获取当前 inode 的 ID，供 fstat 使用。

#### 1.5 修改 EasyFileSystem
在 `easy-fs/src/efs.rs` 中添加 `dealloc_inode()` 方法用于回收 inode。

### 2. 内核层修改 (os)

#### 2.1 添加 stat 方法到 File trait
在 `os/src/fs/mod.rs` 中为 `File` trait 添加 `stat()` 方法，返回 `Stat` 结构体。

#### 2.2 实现 OSInode::stat
在 `os/src/fs/inode.rs` 中实现 `OSInode::stat()` 方法：
- 获取 inode ID
- 判断文件类型（目录或普通文件）
- 获取 nlink 计数
- 构造并返回 Stat 结构体

#### 2.3 实现 Stdin/Stdout::stat
在 `os/src/fs/stdio.rs` 中为 `Stdin` 和 `Stdout` 实现 `stat()` 方法，返回适当的默认值。

#### 2.4 实现 sys_linkat
在 `os/src/syscall/fs.rs` 中实现 `sys_linkat()`：
- 解析 oldpath 和 newpath
- 调用 `ROOT_INODE.link()` 创建硬链接
- 处理错误情况（文件不存在、链接同名文件等）

#### 2.5 实现 sys_unlinkat
在 `os/src/syscall/fs.rs` 中实现 `sys_unlinkat()`：
- 解析路径
- 调用 `ROOT_INODE.unlink()` 删除硬链接
- 处理错误情况（文件不存在等）

#### 2.6 实现 sys_fstat
在 `os/src/syscall/fs.rs` 中实现 `sys_fstat()`：
- 从 fd_table 获取文件描述符对应的文件
- 调用文件的 `stat()` 方法
- 将结果写入用户空间

## 调试过程

### 问题1: easy-fs 私有字段访问
**错误**: `field 'inode_area_start_block' of struct 'EasyFileSystem' is private`
**解决**: 在 `easy-fs/src/efs.rs` 中将 `inode_area_start_block` 字段改为 `pub`，或添加公共方法 `get_inode_area_start_block()`。

### 问题2: nlink 初始化
**问题**: 新创建的文件 nlink 应该初始化为 1
**解决**: 在 `DiskInode::initialize()` 中初始化 nlink 为 1。

### 问题3: unlink 时目录项查找
**问题**: 需要正确找到并删除目录中的对应项
**解决**: 遍历目录项，找到匹配的 inode_id，然后删除该目录项并调整目录大小。

### 问题4: 文件类型判断
**问题**: fstat 需要正确判断文件类型（目录或普通文件）
**解决**: 使用 `disk_inode.is_dir()` 判断，设置相应的 `StatMode`。

## 测试结果

运行 `make run BASE=2 APP=ch6_usertest` 测试所有 ch6 相关功能：

- ✅ ch6_file0: 基本文件读写测试
- ✅ ch6_file1: fstat 功能测试
- ✅ ch6_file2: link/unlink 功能测试
- ✅ ch6_file3: 大量 open/unlink 测试

所有测试通过，ch6 功能实现完整。

## 总结

本次实验成功实现了硬链接相关的三个系统调用：
1. `linkat` - 允许为同一文件创建多个目录项
2. `unlinkat` - 删除目录项，当最后一个链接被删除时回收文件
3. `fstat` - 获取文件元数据，包括 inode 号、文件类型、硬链接数等

实现过程中主要涉及文件系统层的 inode 管理和内核层的系统调用封装，需要正确处理硬链接计数和文件回收逻辑。

