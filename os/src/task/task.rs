//! Types related to task management

use super::TaskContext;
const SYS_MAXID: usize = 500;
/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
		/// The syscall counter
		pub task_syscall_cnt: TaskSyscallCounter,
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

/// The counter of calling of syscalls.
#[derive(Copy, Clone)]
pub struct TaskSyscallCounter {
	syscall_cnt: [isize;SYS_MAXID],
}

impl TaskSyscallCounter {

	/// Get the number of calling of syscalls.
	pub fn get_cnt(&self, id: usize) -> isize{
			self.syscall_cnt[id]
	}

	/// Accumulate the number of callings of syscalls.
	pub fn acc_cnt(&mut self, id: usize) {
			self.syscall_cnt[id] = self.syscall_cnt[id] + 1;
	}

	/// Zero init of TaskSyscallCounter.
	pub fn zero_init() -> Self {
			Self { syscall_cnt: [0; SYS_MAXID]}
	}
}
