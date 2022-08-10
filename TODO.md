# Refactor cli

Instead of matching to string, use [strum](https://crates.io/crates/strum) to convert a string into an enum variant,
and [enum_dispatch](https://crates.io/crates/enum_dispatch) to implement different run methods on enum variants.
