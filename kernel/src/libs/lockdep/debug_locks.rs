use core::sync::atomic::{AtomicBool, Ordering};

static DEBUG_LOCKS: AtomicBool = AtomicBool::new(true);
static DEBUG_LOCKS_SLIENT: AtomicBool = AtomicBool::new(false);

// 在特定条件满足时发出警告消息。
macro_rules! debug_locks_warn_on{
    ($cond:expr) => {
        {
            let mut ret =0;
            if !$crate::oops_in_progress && $cond{
                $crate::instrumentation_begin();
                if $crate::debug_locks_off() && !crate::debug_locks_slient {
                    eprintln!("DEBUG_LOCKS_WARN_ON({})",stringify!($cond));
                }
                $crate::instrumentation_end();
                ret = 1;
            }
            ret
        }
    };
}

fn debug_locks_off() -> bool{
    if DEBUG_LOCKS.load(Ordering::Relaxed){
        if !DEBUG_LOCKS_SLIENT.load(Ordering::Relaxed) {
            // TODO console_verbose();
            return true;
        }
    }
    false
}