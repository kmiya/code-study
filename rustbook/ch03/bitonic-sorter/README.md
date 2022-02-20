# bitonic-sorter

```bash
$ cargo run --release --example benchmark -- 26
Finished release [optimized] target(s) in 0.01s
Running `target/release/examples/benchmark 26`
sorting 67108864 integers (256.0 MB)
cpu info: 6 physical cores, 12 logical cores
seq_sort: sorted 67108864 integers in 26.7219839 seconds
par_sort: sorted 67108864 integers in 5.0381538 seconds
speed up: 5.30x
```
