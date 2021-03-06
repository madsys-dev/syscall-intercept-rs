# syscall-intercept-rs

[![Crate](https://img.shields.io/crates/v/syscall-intercept.svg)](https://crates.io/crates/syscall-intercept)
[![Docs](https://docs.rs/syscall-intercept/badge.svg)](https://docs.rs/syscall-intercept)
[![CI](https://github.com/madsys-dev/syscall-intercept-rs/workflows/CI/badge.svg?branch=main)](https://github.com/madsys-dev/syscall-intercept-rs/actions)

A Rust wrapper of [pmem/syscall_intercept](https://github.com/pmem/syscall_intercept), a system call intercepting library on x86_64 Linux.

## Usage

Install dependencies:

```sh
sudo apt install cmake libcapstone-dev
```

Add the following lines to your Cargo.toml:

```toml
[dependencies]
syscall-intercept = "0.1"
```

Define your syscall hook function:

```rust
use syscall_intercept::*;

extern "C" fn hook(
    num: isize,
    a0: isize,
    a1: isize,
    a2: isize,
    a3: isize,
    a4: isize,
    a5: isize,
    result: &mut isize,
) -> InterceptResult {
    ...
}
```

Enable or disable interception:

```rust
unsafe { set_hook_fn(hook) };
unsafe { unset_hook_fn() };
```

Issue syscall without being intercepted:

```rust
let ret = unsafe { syscall_no_intercept(libc::SYS_exit as _, 0) };
```

## License

MIT License
