//! Types related to task management

use super::{TaskContext, MAX_SYSCALL_NUM};

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// Per-syscall invocation counters
    pub syscall_times: [usize; MAX_SYSCALL_NUM],
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}

impl TaskControlBlock {
    /// Increase count for a syscall id if within tracking range.
    pub fn increase_syscall_count(&mut self, syscall_id: usize) {
        if syscall_id < self.syscall_times.len() {
            self.syscall_times[syscall_id] += 1;
        }
    }

    /// Get count for a syscall id, returns 0 when out of range.
    pub fn get_syscall_count(&self, syscall_id: usize) -> usize {
        if syscall_id < self.syscall_times.len() {
            self.syscall_times[syscall_id]
        } else {
            0
        }
    }
}
