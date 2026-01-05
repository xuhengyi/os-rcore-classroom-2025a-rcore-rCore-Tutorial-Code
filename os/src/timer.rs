//! RISC-V timer-related functionality

use crate::config::CLOCK_FREQ;
use crate::sbi::set_timer;
use crate::sync::UPSafeCell;
use lazy_static::*;
use riscv::register::time;
/// The number of ticks per second
const TICKS_PER_SEC: usize = 100;
#[allow(dead_code)]
/// The number of milliseconds per second
const MSEC_PER_SEC: usize = 1000;
/// The number of microseconds per second
#[allow(dead_code)]
const MICRO_PER_SEC: usize = 1_000_000;

/// Get the current time in ticks
pub fn get_time() -> usize {
    time::read()
}

lazy_static! {
    static ref TICKS: UPSafeCell<usize> = unsafe { UPSafeCell::new(0) };
}

/// Increase global tick counter. Should be called every timer interrupt.
pub fn tick() {
    let mut ticks = TICKS.exclusive_access();
    *ticks += 1;
}

/// get current time in milliseconds based on tick count
#[allow(dead_code)]
pub fn get_time_ms() -> usize {
    let ticks = *TICKS.exclusive_access();
    ticks * MSEC_PER_SEC / TICKS_PER_SEC
}

/// get current time in microseconds based on tick count
#[allow(dead_code)]
pub fn get_time_us() -> usize {
    let ticks = *TICKS.exclusive_access();
    ticks * MICRO_PER_SEC / TICKS_PER_SEC
}

/// Set the next timer interrupt
pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}
