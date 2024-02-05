# rust-palindrome
Some dumb Rust CLI made to try the language
# Usage
## Dev usage
```sh
# The double dash is needed to pass args to cargo ru resulting thread instead of cargo itself
cargo run -- -w someword
```
## After release build
```sh
cd target/release/
./palindrome -w someword
```
# Release build
```sh
cargo build --release
```