# rust-embed-panic

Adding `#[derive(RustEmbed)]` on a struct with doc comments (e.g. `/// foo`)
causes the proc macro to panic.

See the ticket at
[pyros2097/rust-embed#61](https://github.com/pyros2097/rust-embed/issues/61).

```
$ cargo build
   Compiling rust-embed-panic v0.1.0 (/Users/lopopolo/Desktop/rust-embed-panic)
error: proc-macro derive panicked
 --> src/main.rs:7:10
  |
7 | #[derive(RustEmbed)]
  |          ^^^^^^^^^
  |
  = help: message: #[derive(RustEmbed)] should contain one attribute like this #[folder = "examples/public/"]

error: aborting due to previous error

error: Could not compile `rust-embed-panic`.

To learn more, run the command again with --verbose.
```
