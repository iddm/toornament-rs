# toornament-rs
[![](https://meritbadge.herokuapp.com/toornament)](https://crates.io/crates/toornament) [![](https://travis-ci.org/vityafx/toornament-rs.svg?branch=master)](https://travis-ci.org/vityafx/toornament-rs) [![](https://img.shields.io/badge/docs-online-2020ff.svg)](https://vityafx.github.io/toornament-rs/master/toornament/)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)


http://toornament.com api bindings.

## Status
Everything is implemented.

## Implementation
- No unsafe blocks (but in the tests:))
- `reqwest` crate is used for performing requests

## Usage
Start by creating `Toornament` instance and perform needed operations after.
```rust
let t = Toornament::with_application("API_TOKEN", "CLIENT_ID", "CLIENT_SECRET").unwrap();
assert!(t.disciplines(None).is_ok());
```

## License

This project is [licensed under the MIT license](https://github.com/vityafx/toornament-rs/blob/master/LICENSE).
