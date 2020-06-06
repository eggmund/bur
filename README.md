# bur

Bar text updater that uses `xsetroot` to update the text of your window manager's bar.

NOTE: Only tested on dwm's bar.

As default bur only comes with a `Time` and a `Network` module, however it is intended for you to add more modules easily. See the `hello_world.rs` example in
the `examples` folder for more info.

![Example](example.png)

The aim of `bur` is to make a bar status updater that is almost infinitely extensible, whilst also being easy to extend. It can be made to be compatible
with any setup, as the idea is that the source code can be edited to your specific needs.

You should not need extensive Rust knowledge to create good modules, as the aim is to make it simple to create modules.

## Building:

Build with `cargo` using:

```
cargo build --release
```

The binary will then be found in `target/release/bur`.

To run in debug mode, run the executable with the environment variable:

`RUST_LOG=debug ./bur`

## Config:

For configuration, you can define constants in `config.rs`.
