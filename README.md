# cargo-llvm-codecov-converter
Convert from llvm-cov format to codecov format

installation: `cargo install cargo-llvm-codecov-converter`

converts llvm-cov json formatted output from the [llvm-cov export tool](http://llvm.org/docs/CommandGuide/llvm-cov.html#llvm-cov-export) into the [codecov custom coverage format](https://docs.codecov.io/docs/codecov-custom-coverage-format) via `Stdin` and `Stdout`

for example to calculate coverage for rust code:

```bash
export LLVM_PROFILE_FILE="target/debug/coverage/<my-crate>-%m.profraw"

# build the test binary with coverage instrumentation
executables=$(RUSTFLAGS="-Zinstrument-coverage" cargo test --tests --no-run --message-format=json | jq -r "select(.profile.test == true) | .executable")

# run instrumented tests
$executables

# combine profraw files
cargo profdata -- merge -sparse target/debug/coverage/<my-crate>-*.profraw -o target/debug/coverage/<my-crate>.profdata

# collect coverage
cargo cov -- export $executables \
    --instr-profile=target/debug/coverage/<my-crate>.profdata \
    --ignore-filename-regex="(.*\.cargo/registry/.*)|(.*\.rustup/.*)|(.*test.*)" \
    --skip-functions \
    | cargo llvm-codecov-converter > target/debug/coverage/<my-crate>.json
```

note the pipe into `cargo llvm-codecov-converter`
