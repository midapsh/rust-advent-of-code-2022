# Intro

Problems are taken from this [link](https://adventofcode.com/2022).

# Running tests

Use the following command

```shell
# For example
cargo test --lib day1
# cargo test --lib day2
# ...
#
# Run specific test (functions)
cargo test --lib day1::tests::test_part_1_dummy # --lib <mod::to::test_function>
# cargo test --lib day2::tests::test_part_1_dummy
# ...
#
# Run test with print
cargo test --lib day1 -- --nocapture
# cargo test --lib day2 -- --nocapture
```

# Running benchmarks

Use the following command

```shell
# For example
cargo bench -- 1-1
# cargo bench -- 1-2
# cargo bench -- 2-1
# ...
```