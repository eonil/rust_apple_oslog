`eonil_apple_oslog`
==================
Eonil, 2020.

Dead-simple call to Apple's `os_log` function. (system level log facility)
- BSD level `syslog` doesn't work well on macOS. (at least on my machine)
- Apple is providing their own version of logging facility called `OSLog`.
- `OSLog` is complex facility.
- This just provides single simple logging function with default settings.

How to Use
----------

Add dependency to your `Cargo.toml`.

[dependency]
eonil_apple_oslog = { git = "https://github.com/eonil/rust_apple_oslog", tag = "0.1.0" }

Call the function.

    use eonil_apple_oslog::oslog_default;

    fn main() {
        oslog_default("Hello, world!");
    }

Open Console.app and run your code. See how it comes up.

License
-------
"MIT License".

- 

