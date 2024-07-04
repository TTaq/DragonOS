use core::{
    fmt::Debug,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};


#[cfg(CONFIG_PROVE_LOCKING)]
static prove_locking: i32 = 1;
#[cfg(not(CONFIG_PROVE_LOCKING))]
static prove_locking:i32 =0;

#[cfg(CONFIG_LOCK_STAT)]
static lock_stat:i32 =1;
#[cfg(not(CONFIG_LOCK_STAT))]
static lock_stat:i32 =0;

#[cfg(CONFIG_SYSCTL)]
// TODO 结构体ctl_table如何定义

// TODO 如何将下面函数声明为static
// debug_locks如何获取
// this_cpu_read(lock_recursion)如何获取
// current.lock_recursion如何获取
#[inline(always)]
fn lockdep_enable() -> bool{
    if !debug_locks{
        return false;
    }
    if this_cpu_read(lock_recursion){
        return fasle;
    }
    if current.lock_recursion{
        return false;
    }
    true;
    
}

// lockdep_lock：保护锁定图，哈希和类/哈希分配器。
lazy_static!{
    static _owner: *mut ProcessControlBlock;
    static  lockdep_selftest_task_struct: *mut ProcessControlBlock;
    static ref NR_LIST_ENTRIES:AtomicUsize = AtomicUsize::new(0);
    static ref LIST_ENTRIES: Mutex
}

pub struct LockList {
    entry: LockHead,
    
} 




