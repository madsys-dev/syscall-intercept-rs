#[link(name = "syscall_intercept")]
extern "C" {
    static mut intercept_hook_point: Option<HookFn>;

    pub fn syscall_no_intercept(num: isize, ...) -> i32;
}

/// Set syscall intercept hook function.
pub fn set_hook_fn(f: HookFn) {
    unsafe { intercept_hook_point = Some(f) };
}

/// Clear syscall intercept hook function.
pub fn unset_hook_fn() {
    unsafe { intercept_hook_point = None };
}

/// The type of hook function.
pub type HookFn = extern "C" fn(
    num: isize,
    a0: isize,
    a1: isize,
    a2: isize,
    a3: isize,
    a4: isize,
    a5: isize,
    result: &mut isize,
) -> InterceptResult;

/// The return value of hook function.
#[repr(i32)]
pub enum InterceptResult {
    /// The user takes over the system call.
    Hook = 0,
    /// The specific system call was ignored by the user and the original syscall should be executed.
    Forward = 1,
}
