# CHANGELOG for the `enum-debug` crate
This file keeps track of the changes done in each version of the `enum-debug` crate.

Note that this project uses [semantic versioning](https://semver.org). As such, breaking changes are indicated as **(BREAKING)**.


## 1.0.0 - 2024-07-22
IMPORTANT NOTICE: Now using the Apache 2.0 license. See [`LICENSE`](./LICENSE) for more details.

### Added
- `EnumDebug::variant_names()` to get a static slice of all variant names.
- `EnumDebug::variants()` to iterate over all variant names.

### Changed
- The project now uses [semantic versioning](https://semver.org) **(BREAKING)**.
- The license has been changed from GPLv3 to Apache 2.0. See [LICENSE](./LICENSE) for more information **(BREAKING)**.
- `EnumDebug::fmt` -> `EnumDebug::variant_name()`, which returns the static variant name instead of formatting it **(BREAKING)**.
- `EnumDebug::fmt_type_name` -> `EnumDebug::type_name()`, which returns the static type name instead of formatting it **(BREAKING)**.
- Now depending on `syn` 2.0 instead of 1.0.
- `enum-debug` is now the toplevel crate instead of this being a purely virtual workspace.

### Fixed
- `EnumDebug` derivation not working over enums with traits.
- `EnumDebug` derivation not working over enums with no variants.

## 0.2.0 - 2023-01-21
### Added
- An `enum_debug(path)` attribute that emulates the old behaviour to generate the full struct path as the name (see below).

### Changed
- The default behaviour for enum name derivation to use `enum_debug(name)`.

### Fixed
- `write!` and `stringify!` calls being local to scope, causing crashes if overridden.

## 0.1.0 - 2022-12-10
Initial release of the crate.
