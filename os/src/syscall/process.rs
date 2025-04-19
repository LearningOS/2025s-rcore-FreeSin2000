//! Process management syscalls
use crate::task::{change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, copy_from_user, copy_to_user, current_syscall_cnt};
use crate::mm::{VirtAddr};
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
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    /*
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    */
    let res =  TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    let ptr = &res as *const TimeVal as *const u8;
    let len = core::mem::size_of::<TimeVal>();
    let buf = unsafe{core::slice::from_raw_parts(ptr, len)};
    let va: VirtAddr = (ts as usize).into();
    copy_to_user(va, len, buf);
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {

    trace!("kernel: sys_trace");
    match trace_request {
        0 => {
            let mut res: u8 = 0;
            let len = core::mem::size_of::<u8>();
            let ptr = &mut res as * mut u8;
            let buf = unsafe{core::slice::from_raw_parts_mut(ptr, len)};
            if copy_from_user(id.into(), len, buf) >= 0 {res as isize} else {-1}
            //unsafe{ *(id as *const u8) as isize}
        },
        1 => {
            let len = core::mem::size_of::<u8>();
            let ptr = &data as *const usize as * const u8;
            let buf = unsafe{core::slice::from_raw_parts(ptr, len)};
            //unsafe{ *(id as *mut u8) = (data & 0xff) as u8};
            if copy_to_user(id.into(), len, buf) >= 0 {0} else {-1}

        },
        2 => {
            current_syscall_cnt(id) as isize
        },
        _ => {
            -1
        },
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    let start_va: VirtAddr = start.into();
    let _end_va: VirtAddr = (start_va.0 + len).into();
    if !start_va.aligned() {
        return -1;
    }
    if prot & !0x7 != 0 || prot & 0x7 == 0 {
        return -1;
    }
    /*let start_vpn = start_va.floor();
    let end_vpn = end_va.ceil();
    let mut map_perm = MapPermission::U;
    map_perm = map_perm | (prot << 1);
    */
    -1 
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    -1
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
