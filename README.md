This is a rewrite of php-daydifference written in Rust.

## Development

Dev run the program with `cargo run 1024-06-28 2024-07-12 "1,2,3,4,5" "2021-01-10,2021-01-20"`

Compile with `cargo build --release`

Release run with `./target/release/daydif ...`

## Performance

`perf.php` can be dropped into the php-daydifference directory, and run with `php ./src/perf.php`.

That library runs the perf test in about 5.5 seconds.

That goal is to compile this Rust code as a library, and modify perf.php to call this library using FFI, comparing performance to php-daydifference.