# toornament-rs
[![Build status](https://travis-ci.org/vityafx/toornament-rs.svg?branch=master)](https://travis-ci.org/vityafx/toornament-rs)
[![Crates](https://img.shields.io/crates/v/toornament.svg)](https://crates.io/crates/toornament)
[![Docs](https://docs.rs/toornament/badge.svg)](https://docs.rs/toornament)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)



http://toornament.com api bindings.

## Status
Everything is implemented. Note, that the `toornament` service API is at early stages of development
and in beta stage, the library will try to be up-to-date.

## The book and the documentation
There is a [book](https://vityafx.github.io/toornament-rs) and the
[documentation](https://docs.rs/toornament) which may help you using this library.

## Implementation
- Non-mutable interface, no need to synchronize, thread-safe.
- No unsafe blocks
- No unwraps (except the tests)
- `reqwest` crate is used for performing requests

## Usage
Start by creating `Toornament` instance and perform needed operations after.

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let t = Toornament::with_application("API_TOKEN", "CLIENT_ID", "CLIENT_SECRET")
                       .unwrap();
    assert!(t.disciplines(None).is_ok());
}
```

More examples are in the  
[`examples/` subdirectory](./examples/)

## License

This project is [licensed under the MIT license](https://github.com/vityafx/toornament-rs/blob/master/LICENSE).
