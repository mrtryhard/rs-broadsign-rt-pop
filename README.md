# Broadsign Real-Time Pop server sample
This is an unofficial server example. It is discouraged to use it as-is for production uses.
It comes with no warranty.

This implementation was done off hour, for hobby only.

## About Real-Time Pop server implementation
This is an implementation of version 13.2's real-time pop protocol:
https://docs.broadsign.com/broadsign-control/13-2/real-time-pop-api.html

## Using VS Code
1. Install `rust-analyzer` and  `Rust` extensions.
2. Recommended: Change `Rust` extension settings to use `rust-analyzer` instead of `rls` (`ctrl + ,`).
3. Test using `cargo test`.
3. Run using `cargo run`. Use `cargo run --release` for optimizations.

## Using Insomnia
An Insomnia file is available (`api_insomnia.json`). You may import it to help you debug or comprehend
how to use this real-time pop server implementation.

## License
Unless otherwise specified by Broadsign International LLC, this code is MIT licensed.