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
    // NOTE: Not necessary, but otherwise it will use the Rust internal type name
    #[inline]
    fn type_name() -> &'static str { "Jedi" }

    #[inline]
    fn variant_names() -> &'static [&'static str] {
        &["ObiWanKenobi", "AnakinSkywalker", "MaceWindu", "MasterYoda"]
    }

    #[inline]
    fn variant_name(&self) -> &'static str {
        match self {
            Self::ObiWanKenobi => Self::variant_names()[0],
            Self::AnakinSkywalker => Self::variant_names()[1],
            Self::MaceWindu => Self::variant_names()[2],
            Self::MasterYoda => Self::variant_names()[3],
        }
    }
}

assert_eq!(format!("{}", Jedi::ObiWanKenobi.variant()), "ObiWanKenobi");
assert_eq!(format!("{:?}", Jedi::AnakinSkywalker.variant()), "Jedi::AnakinSkywalker");
assert_eq!(Jedi::MaceWindu.variant_name(), "MaceWindu");
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

assert_eq!(format!("{}", Jedi::ObiWanKenobi.variant()), "ObiWanKenobi");
assert_eq!(format!("{:?}", Jedi::AnakinSkywalker.variant()), "Jedi::AnakinSkywalker");
assert_eq!(Jedi::MaceWindu.variant_name(), "MaceWindu");
```

See the documentation on the `derive`-module for more information on the derive-macro.


## Contribution
If you have any suggestions, comments, tip or bugs, please create an [issue](https://github.com/Lut99/enum-debug/issues) to let us know! Or go ahead and create a [pull request](https://github.com/Lut99/enum-debug/pulls).


## License
This project is licensed under the Apache 2.0 license. See [`LICENSE`](./LICENSE) for more information.
