# Install

* `curl https://sh.rustup.rs -sSf | sh` - install rust
*  `rustup toolchain install nightly`  - install nightly version
* `rustup default nightly` - use nightly version by default

# Run

* `cargo test`  - run tests
* `cargo bench` - run benchmarks

```bash
$ cargo bench
    Finished release [optimized] target(s) in 0.00s
     Running target/release/deps/hashmap-897fff1c24678ae7

running 4 tests
test bench::read_bench       ... bench:     801,439 ns/iter (+/- 19,824)
test bench::read_write_bench ... bench:   1,788,829 ns/iter (+/- 43,840)
test bench::write_bench      ... bench:     993,139 ns/iter (+/- 48,839)

test result: ok. 0 passed; 0 failed; 1 ignored; 3 measured; 0 filtered out
```