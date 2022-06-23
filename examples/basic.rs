use std::cell::Cell;
use syscall_intercept::*;

fn main() {
    println!("enable interception ...");
    unsafe { set_hook_fn(hook) };

    println!("Hello world!");

    unsafe { unset_hook_fn() };
    println!("disabled interception");
}

extern "C" fn hook(
    num: isize,
    a0: isize,
    a1: isize,
    a2: isize,
    _a3: isize,
    _a4: isize,
    _a5: isize,
    result: &mut isize,
) -> InterceptResult {
    // detect and avoid recursive interception
    let _guard = match InterceptGuard::try_lock() {
        Some(g) => g,
        None => return InterceptResult::Forward,
    };
    // intercept interested functions
    if num == libc::SYS_write as _ && a0 == 1 {
        let buf = unsafe { std::slice::from_raw_parts(a1 as *const u8, a2 as usize) };
        let newbuf = buf
            .iter()
            .map(|&b| b.to_ascii_uppercase())
            .collect::<Vec<u8>>();
        let ret = unsafe {
            syscall_no_intercept(libc::SYS_write as _, a0, newbuf.as_ptr(), newbuf.len())
        };
        *result = ret;
        return InterceptResult::Hook;
    }
    InterceptResult::Forward
}

thread_local! {
    /// A flag indicating whether the current thread is in an intercept context.
    static INTERCEPTED: Cell<bool> = Cell::new(false);
}

struct InterceptGuard;

impl InterceptGuard {
    fn try_lock() -> Option<Self> {
        INTERCEPTED.with(|x| {
            if x.get() {
                None
            } else {
                x.set(true);
                Some(InterceptGuard)
            }
        })
    }
}

impl Drop for InterceptGuard {
    fn drop(&mut self) {
        INTERCEPTED.with(|x| x.set(false));
    }
}
