# nocash_gba_log

A logging implementation for no$gba.

Provides a logging implementation for the [`log`](https://docs.rs/log/latest/log/index.html) crate for logging when compiling for the Game Boy Advance and running within the [no$gba](https://problemkaputt.de/gba.htm) emulator.

## Usage

### In libraries
`nocash_gba_log` should be used in binaries only. Libraries should instead use the logging facade provided by the [`log`](https://docs.rs/log/latest/log/index.html) crate directly.

### In binaries
When logging in a binary, only one logger may be enabled. Therefore, `nocash_gba_log` cannot be used alongside any other logging implementations.

#### Installation
Add `nocash_gba_log` as a dependency in your `Cargo.toml`:

``` toml
[dependencies]
nocash_gba_log = "0.1.0"
```

Then call [`init()`](https://docs.rs/nocash_gba_log/latest/nocash_gba_log/fn.init.html) early in your binary. Any records logged before initialization will be silently dropped.

``` rust
fn main() {
    nocash_gba_log::init().expect("unable to initialize no$gba logger");

    log::info!("Hello, world!");
}
```

Note that you may want to handle the returned [`Error`](https://docs.rs/nocash_gba_log/latest/nocash_gba_log/struct.Error.html) message from [`init()`](https://docs.rs/nocash_gba_log/latest/nocash_gba_log/fn.init.html) more robustly, unless you only want your project to be run in no$gba.

## Compatibility
This logger uses memory mapped IO registers specific to the Game Boy Advance. It is therefore only safe to use this library when building to run on the Game Boy Advance or a Game Boy Advance emulator.

If this logger is attempted to be initialized when not running on no$gba, it will fail to initialize with an [`Error`](https://docs.rs/nocash_gba_log/latest/nocash_gba_log/struct.Error.html) identifying the failure.

## License
This project is licensed under either of

* Apache License, Version 2.0
([LICENSE-APACHE](https://github.com/Anders429/nocash_gba_log/blob/HEAD/LICENSE-APACHE) or
http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
([LICENSE-MIT](https://github.com/Anders429/nocash_gba_log/blob/HEAD/LICENSE-MIT) or
http://opensource.org/licenses/MIT)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
