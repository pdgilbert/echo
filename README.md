# echo
Test example serial read/write using embedded-io traits.

## Building

The examples can be built using one of these lines:
```
cargo build --no-default-features --target thumbv7em-none-eabihf --features stm32f411,stm32f4xx --example echo_by_char
cargo build --no-default-features --target thumbv7em-none-eabihf --features stm32g474,stm32g4xx --example echo_by_char
cargo build --no-default-features --target thumbv7em-none-eabihf --features stm32h742,stm32h7xx --example echo_by_char
```
## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
