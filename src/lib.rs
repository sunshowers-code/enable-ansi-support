// Copyright (c) The enable-ansi-support Contributors
// SPDX-License-Identifier: MIT

//! Enable ANSI code support on Windows 10 and above.
//!
//! This crate provides one function, `enable_ansi_support`, which allows ANSI escape codes
//! to work on Windows 10 and above.
//!
//! Call `enable_ansi_support` *once*, early on in `main()`, to enable ANSI escape codes generated
//! by crates like
//! [`ansi_term`](https://docs.rs/ansi_term) or [`owo-colors`](https://docs.rs/owo-colors)
//! to work on Windows just like they do on Unix platforms.
//!
//! This uses Windows API calls to alter the properties of the console that
//! the program is running in. See the
//! [Windows documentation](https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences)
//! for more information.
//!
//! On non-Windows platforms, `enable_ansi_support` is a no-op.

/// Enables ANSI code support on Windows 10.
///
/// Returns the Windows error code if unsuccessful.
///
/// On non-Windows platforms, this is a no-op that always returns `Ok(())`.
#[cfg(windows)]
pub fn enable_ansi_support() -> Result<(), u32> {
    // ref: https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#EXAMPLE_OF_ENABLING_VIRTUAL_TERMINAL_PROCESSING @@ https://archive.is/L7wRJ#76%

    use std::{ffi::OsStr, iter::once, os::windows::ffi::OsStrExt, ptr::null_mut};
    use winapi::um::{
        consoleapi::{GetConsoleMode, SetConsoleMode},
        errhandlingapi::GetLastError,
        fileapi::{CreateFileW, OPEN_EXISTING},
        handleapi::INVALID_HANDLE_VALUE,
        winnt::{FILE_SHARE_WRITE, GENERIC_READ, GENERIC_WRITE},
    };

    const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;

    unsafe {
        // ref: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew
        // Using `CreateFileW("CONOUT$", ...)` to retrieve the console handle works correctly even if STDOUT and/or STDERR are redirected
        let console_out_name: Vec<u16> =
            OsStr::new("CONOUT$").encode_wide().chain(once(0)).collect();
        let console_handle = CreateFileW(
            console_out_name.as_ptr(),
            GENERIC_READ | GENERIC_WRITE,
            FILE_SHARE_WRITE,
            null_mut(),
            OPEN_EXISTING,
            0,
            null_mut(),
        );
        if console_handle == INVALID_HANDLE_VALUE {
            return Err(GetLastError());
        }

        // ref: https://docs.microsoft.com/en-us/windows/console/getconsolemode
        let mut console_mode: u32 = 0;
        if 0 == GetConsoleMode(console_handle, &mut console_mode) {
            return Err(GetLastError());
        }

        // VT processing not already enabled?
        if console_mode & ENABLE_VIRTUAL_TERMINAL_PROCESSING == 0 {
            // https://docs.microsoft.com/en-us/windows/console/setconsolemode
            if 0 == SetConsoleMode(
                console_handle,
                console_mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING,
            ) {
                return Err(GetLastError());
            }
        }
    }

    Ok(())
}

/// Enables ANSI code support on Windows 10.
///
/// Returns the Windows error code if unsuccessful.
///
/// On non-Windows platforms, this is a no-op that always returns `Ok(())`.
#[cfg(not(windows))]
#[inline]
pub fn enable_ansi_support() -> Result<(), u32> {
    Ok(())
}
