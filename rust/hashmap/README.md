# Install

* `curl https://sh.rustup.rs -sSf | sh` - install rust
*  `rustup toolchain install nightly`  - install nightly version
* `rustup default nightly` - use nightly version by default

# Run

* `cargo test`  - run tests
* `cargo bench` - run benchmarks


N = 1000000
```bash
$ cargo bench
    Finished release [optimized] target(s) in 0.00s
     Running target/release/deps/hashmap-897fff1c24678ae7

running 3 tests
test bench::fx_read_bench              ... bench: 162,922,243 ns/iter (+/- 3,903,465)
test bench::fx_read_write_bench        ... bench: 349,671,155 ns/iter (+/- 32,493,617)
test bench::fx_write_bench             ... bench: 186,306,450 ns/iter (+/- 3,371,861)
test bench::hashbrown_read_bench       ... bench: 163,083,437 ns/iter (+/- 3,613,079)
test bench::hashbrown_read_write_bench ... bench: 349,662,093 ns/iter (+/- 6,257,600)
test bench::hashbrown_write_bench      ... bench: 186,594,249 ns/iter (+/- 5,291,077)
test bench::read_bench                 ... bench: 191,496,536 ns/iter (+/- 2,796,407)
test bench::read_write_bench           ... bench: 405,558,283 ns/iter (+/- 5,273,230)
test bench::write_bench                ... bench: 214,778,187 ns/iter (+/- 2,918,149)

test result: ok. 0 passed; 0 failed; 1 ignored; 3 measured; 0 filtered out
```