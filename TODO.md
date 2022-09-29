# Refactor cli

- Instead of matching to string, use [strum](https://crates.io/crates/strum) to convert a string into an enum variant,
and [enum_dispatch](https://crates.io/crates/enum_dispatch) to implement different run methods on enum variants.

- Use [thiserror](https://crates.io/crates/thiserror) instead of io::Errors for core

# Refactor core

It is ugly to have extras arguments on theme file. Maybe we could pass
the theme name to the postscript and have an storage inside postcript folder, for
key-values, where the key is the theme name and values are the arguments. We would need
a storage for each postscript, but passing only the theme name would complicate extras.

Idea: Extras are a folder, which have a script.sh and a storage.toml. In storage.toml,
we will have the mapping from theme name to args. Then, in rust we will parse that
storage and call the script with the theme associated args (map.get(theme-name))
This way, theme files will be a lot cleaner.
