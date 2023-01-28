# CHANGELOG for the `enum-debug` crate
This file keeps track of the changes done in each version of the `enum-debug` crate.

## 0.2.1 - 2023-01-28
### Fixed
- `EnumDebug` derivation not working over enums with traits.

## 0.2.0 - 2023-01-21
### Added
- An `enum_debug(path)` attribute that emulates the old behaviour to generate the full struct path as the name (see below).

### Changed
- The default behaviour for enum name derivation to use `enum_debug(name)`.

### Fixed
- `write!` and `stringify!` calls being local to scope, causing crashes if overridden.

## 0.1.0 - 2022-12-10
Initial release of the crate.
