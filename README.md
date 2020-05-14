# bur

Bar text updater that uses `xsetroot` to update the text of your window manager's bar.

NOTE: Only tested on dwm's bar.

As default bur only comes with a `Time` module, however it is intended for you to add more modules easily. See `Adding modules` for more info.

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

## Adding modules:

This example will be a module that prints out "Hello, World!".

First we need to create a new rust module `hello_world` inside of `src/modules/mod.rs`:

```rust
pub mod hello_world;
```

Then a new file called `hello_world.rs` will need to be made under the `src/modules/` folder, containing:

```rust
use super::{*, Module};

pub struct HelloWorld;
```

All this does is define an empty struct called `HelloWorld`, and imports types we will need later. To use values defined in the
config file (`config.rs`), then here you need to import `crate::config`.

Now implement the `Module` trait for your struct. The `Module` trait is a simple trait that only contains an `update` function (as of writing).

```rust
// -- snip ---
#[async_trait]
impl Module for HelloWorld {
    async fn update(&mut self, _update_counter: usize) -> GenResult<bool> {
        Ok(false)
    }
}
```

The `update` function returns `true` if it has updated. `update_counter` is used to tell if the module
needs updating. This needs to be decided in the `update` function since each module will have different update rates.
For the HelloWorld example, we don't need to update anything since all we are doing is printing the text "Hello, World!".

Now the trait `std::fmt::Display` has to be implemented for the `HelloWorld` struct, so it can be displayed on the bar.

```rust
// -- snip --
impl fmt::Display for HelloWorld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hello, World!")
    }
}
```

Finally, we need to add it to the bar. In `src/main.rs`:

```rust
// -- Inside fn main() --
let mut bur = Bur::new(vec![
    // Put our new `HelloWorld` module here:
    Box::new( modules::hello_world::HelloWorld ),
    Box::new( modules::time::Time::default() ),
]);
```

The order at which the modules are drawn from left to right in the order of this list.

Now recompile with `cargo build --release`, and that's the module done! Of course, this can be taken as far as you want.
For example, in my personal `bur` I have a module that shows the current Ethereum price.
See the included `Time` module for how to start making a module that is slightly more complex.

