# enable-ansi-support: Enable ANSI escape code support on Windows 10

[![enable-ansi-support on crates.io](https://img.shields.io/crates/v/enable-ansi-support)](https://crates.io/crates/enable-ansi-support)
[![Documentation (latest release)](https://img.shields.io/badge/docs-latest%20release-brightgreen.svg)](https://docs.rs/enable-ansi-support/)
[![Documentation (main)](https://img.shields.io/badge/documentation-main-purple.svg)](https://sunshowers-code.github.io/enable-ansi-support/rustdoc/enable_ansi_support/)
[![License](https://img.shields.io/badge/license-Apache-green.svg)](LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

## About

This crate provides one function, `enable_ansi_support`, which allows ANSI escape codes to work on Windows 10 and above.
Call `enable_ansi_support` *once*, early on in `main()`, to enable ANSI escape codes generated by crates like
[`ansi_term`](https://docs.rs/ansi_term) or [`owo-colors`](https://docs.rs/owo-colors) to work on Windows just like they
do on Unix platforms.

On non-Windows platforms, `enable_ansi_support` is a no-op.

## Example

```rust
fn main() {
    match enable_ansi_support::enable_ansi_support() {
        Ok(()) => {
            // ANSI escape codes were successfully enabled, or this is a non-Windows platform.
            // Use your terminal color library of choice here.
            println!("\x1b[31mHello, world\x1b[0m");
        }
        Err(_) => {
            // The operation was unsuccessful, typically because it's running on an older
            // version of Windows. The program may choose to disable ANSI color code output in
            // this case.
        }
    }
}
```

## Minimum supported Rust version

The minimum supported Rust version (MSRV) is **1.49**. The MSRV will be updated sparingly, and any
update to it will be considered a breaking change.

## License and credits

This project is available under the terms of the [MIT license](LICENSE). It is a derivative of `ansi_term`'s
[`enable_ansi_support`](https://github.com/ogham/rust-ansi-term/blob/master/src/windows.rs)
(as of [this snapshot](https://github.com/ogham/rust-ansi-term/blob/ff7eba98d55ad609c7fcc8c7bb0859b37c7545cc/src/windows.rs))
with minor modifications. The upstream code is used under the terms of the MIT license:

> Copyright (c) 2014 Benjamin Sago
>
> Permission is hereby granted, free of charge, to any person obtaining a copy
> of this software and associated documentation files (the "Software"), to deal
> in the Software without restriction, including without limitation the rights
> to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
> copies of the Software, and to permit persons to whom the Software is
> furnished to do so, subject to the following conditions:
>
> The above copyright notice and this permission notice shall be included in all
> copies or substantial portions of the Software.
> 
> THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
> IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
> FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
> AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
> LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
> OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
> SOFTWARE.
