//! Process management syscalls
use crate::config::PAGE_SIZE;
use crate::mm::{translated_byte_buffer, MapPermission, PageTable, PTEFlags, VirtAddr};
use crate::task::{
    change_program_brk, current_user_token, exit_current_and_run_next, suspend_current_and_run_next,
    syscall_times, with_current_task, MAX_SYSCALL_NUM,
};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    if _ts.is_null() {
        return -1;
    }
    let us = get_time_us();
    let time = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    let time_bytes = unsafe {
        core::slice::from_raw_parts(
            &time as *const _ as *const u8,
            core::mem::size_of::<TimeVal>(),
        )
    };
    let mut copied = 0;
    let buffers = translated_byte_buffer(
        current_user_token(),
        _ts as *const u8,
        core::mem::size_of::<TimeVal>(),
    );
    for buf in buffers {
        let len = buf.len().min(time_bytes.len() - copied);
        buf[..len].copy_from_slice(&time_bytes[copied..copied + len]);
        copied += len;
        if copied == time_bytes.len() {
            break;
        }
    }
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");
    let page_table = PageTable::from_token(current_user_token());
    match _trace_request {
        // Trace read
        0 => {
            let va = VirtAddr::from(_id);
            let vpn = va.floor();
            if let Some(pte) = page_table.translate(vpn) {
                let flags = pte.flags();
                if !pte.is_valid() || !flags.contains(PTEFlags::U) || !pte.readable() {
                    return -1;
                }
                let value = pte.ppn().get_bytes_array()[va.page_offset()];
                value as isize
            } else {
                -1
            }
        }
        // Trace write
        1 => {
            let va = VirtAddr::from(_id);
            let vpn = va.floor();
            if let Some(pte) = page_table.translate(vpn) {
                let flags = pte.flags();
                if !pte.is_valid() || !flags.contains(PTEFlags::U) || !pte.writable() {
                    return -1;
                }
                pte.ppn().get_bytes_array()[va.page_offset()] = _data as u8;
                0
            } else {
                -1
            }
        }
        // Count syscall
        2 => {
            if _id < MAX_SYSCALL_NUM {
                syscall_times(_id) as isize
            } else {
                -1
            }
        }
        _ => -1,
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap");
    if _start % PAGE_SIZE != 0 {
        return -1;
    }
    if _port & !0x7 != 0 || _port & 0x7 == 0 {
        return -1;
    }
    let len_aligned = (_len + PAGE_SIZE - 1) / PAGE_SIZE * PAGE_SIZE;
    let end = match _start.checked_add(len_aligned) {
        Some(end) => end,
        None => return -1,
    };
    if len_aligned == 0 {
        return 0;
    }
    with_current_task(|task| {
        let start_va = VirtAddr::from(_start);
        let end_va = VirtAddr::from(end);
        if !task.memory_set.is_range_free(start_va, end_va) {
            return -1;
        }
        task.memory_set
            .insert_framed_area(start_va, end_va, prot_to_perm(_port));
        0
    })
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap");
    if _start % PAGE_SIZE != 0 || _len % PAGE_SIZE != 0 {
        return -1;
    }
    if _len == 0 {
        return 0;
    }
    let end = match _start.checked_add(_len) {
        Some(end) => end,
        None => return -1,
    };
    with_current_task(|task| {
        let start_va = VirtAddr::from(_start);
        let end_va = VirtAddr::from(end);
        if task.memory_set.unmap_range(start_va, end_va) {
            0
        } else {
            -1
        }
    })
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}

fn prot_to_perm(prot: usize) -> MapPermission {
    let mut perm = MapPermission::U;
    if prot & 0x1 != 0 {
        perm |= MapPermission::R;
    }
    if prot & 0x2 != 0 {
        perm |= MapPermission::W;
    }
    if prot & 0x4 != 0 {
        perm |= MapPermission::X;
    }
    perm
}
