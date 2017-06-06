# Quick start

## Compiling

To compile the crate simply get the sources and perform use `cargo` to build it:

```bash
git clone https://github.com/vityafx/toornament-rs.git
cd toornament-rs/
cargo update;
cargo build --release;
```

## Adding the crate as dependency

To add as dependency you must add the line into your `Cargo.toml`:

```toml
[dependencies]
toornament = "*"
```

It will get the latest available `toornament` crate. However, if you want to publish your crate
you must know the exact version and specify it in the `[dependencies]` section.

## Running

Check that you have everything installed correctly by compiling a minimal user-crate:


```rust
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Listing all the tournaments
    println!("Tournaments: {:?}\n", toornament.tournaments(None, true));
}
```

Change `API_TOKEN`, `CLIENT_ID`, `CLIENT_SECRET` to yours and run the source with `cargo run`.
If everything is good you will see all the tournaments available in the `toornament.com` service.
