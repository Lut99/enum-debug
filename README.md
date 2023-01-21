# EnumDebug
Simple Rust crate that implements EnumDebug, which can automatically derive a formatter for enum variant names only.


## Installation
To use this crate, simply add the following to your `Cargo.toml`:
```toml
enum-debug = { git = "https://github.com/Lut99/enum-debug" }
```

This will use the latest version. You can also commit yourself to a specific version by using:
```toml
enum-debug = { git = "https://github.com/Lut99/enum-debug", tag="v<VERSION>" }
```
where you should replace `<VERSION>` with the version of your choice (check the [releases](https://github.com/Lut99/enum-debug/releases) to find possible options).


## Usage
This crate makes the `EnumDebug` trait available, which can be implemented on an enum of your choice.

Custom implementation is done as follows:
```rust
use enum_debug::EnumDebug;

enum Jedi {
    ObiWanKenobi,
    AnakinSkywalker,
    MaceWindu,
    MasterYoda,
}
impl EnumDebug for Jedi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Jedi::*;
        match self {
            // Simply map each variant to a string representation
            ObiWanKenobi    => write!(f, "ObiWanKenobi"),
            AnakinSkywalker => write!(f, "AnakinSkywalker"),
            MaceWindu       => write!(f, "MaceWindu"),
            MasterYoda      => write!(f, "MasterYoda"),
        }
    }

    // OPTIONAL: You can also implement `fmt_type_name` to override the default generated type name (see below)
    fn fmt_type_name(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Jedi")
    }
}
```

However, this is quite tedious. A faster way is to use the `derive` feature and the use the derive macro:
```toml
# Enable the feature
enum-debug = { git = "https://github.com/Lut99/enum-debug", features = ["derive"] }
```
```rust
use enum_debug::EnumDebug;

// Now it becomes as easy as
#[derive(EnumDebug)]
enum Jedi {
    ObiWanKenobi,
    AnakinSkywalker,
    MaceWindu,
    MasterYoda,
}
```

You can still set a manual name by using the `enum_debug` attribute:
```rust
use enum_debug::EnumDebug;

#[derive(EnumDebug)]
#[enum_debug(name = "Jedi")]
enum Jedi {
    ObiWanKenobi,
    AnakinSkywalker,
    MaceWindu,
    MasterYoda,
}
```

You can also specify `#[enum_debug(path)]` to generate the name as a full path, to help disambiguate if you have multiple classes with the same name in different files. Alternatively, for legacy reasons (when the path was the default behaviour), if you just specify `#[enum_debug(name)]`, then the exact same name as the literal identifier is used.

With the trait implemented, you can then use it to get the variant name:
```rust
use enum_debug::EnumDebug;
// Or
use enum_debug::prelude::*;

// `variant()` returns a struct that implement display, so you can simply use:
println!("Hello, {}!", Jedi::ObiWanKenobi.variant()); // Hello, ObiWanKenobi!
// If you use the debug formatter instead, then the type name is prepended (according to `fmt_type_name()`)
println!("Hello, {:?}!", Jedi::ObiWanKenobi.variant()); // Hello, Jedi::ObiWanKenobi!

// To get a string, use the default `to_string()` method
let jedi: String = Jedi::ObiWanKenobi.variant().to_string();
``` 


## Contribution
If you have any suggestions, comments, tip or bugs, please create an [issue](https://github.com/Lut99/enum-debug/issues) to let us know. Use appropriate tags to help speedup the process.
