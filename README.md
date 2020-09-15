# Ackermann function in Rust

This repository has an implementation of the Ackerman function in the programming language Rust. This was directly inspired by this Computerphile 

## Results

Computations where done on my laptop with a i7-6500U CPU @ 2.50GHz.

The furthest into a 0-6 double loop that I was able to get to was just ackerman (4,1) in a little under 22 seconds and then eventually ran out of stack size.

```bash
$ cargo run
   Compiling ackermann v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 1.03s
     Running `target/debug/ackermann`
  0.000001: ackermann (0,0) is: 1
  0.000016: ackermann (0,1) is: 2
  0.000021: ackermann (0,2) is: 3
  0.000023: ackermann (0,3) is: 4
  0.000026: ackermann (0,4) is: 5
  0.000032: ackermann (0,5) is: 6
  0.000038: ackermann (1,0) is: 2
  0.000043: ackermann (1,1) is: 3
  0.000047: ackermann (1,2) is: 4
  0.000051: ackermann (1,3) is: 5
  0.000054: ackermann (1,4) is: 6
  0.000056: ackermann (1,5) is: 7
  0.000060: ackermann (2,0) is: 3
  0.000062: ackermann (2,1) is: 5
  0.000065: ackermann (2,2) is: 7
  0.000069: ackermann (2,3) is: 9
  0.000074: ackermann (2,4) is: 11
  0.000077: ackermann (2,5) is: 13
  0.000081: ackermann (3,0) is: 5
  0.000084: ackermann (3,1) is: 13
  0.000091: ackermann (3,2) is: 29
  0.000110: ackermann (3,3) is: 61
  0.000183: ackermann (3,4) is: 125
  0.000466: ackermann (3,5) is: 253
  0.000470: ackermann (4,0) is: 13
 21.872629: ackermann (4,1) is: 65533

thread 'main' has overflowed its stack
fatal runtime error: stack overflow
[1]    20708 abort      cargo run
```

Running is release mode improved performance drastically, getting the time to calculate up to ackerman(4,1) down to 3.2 seconds:

```bash
$ cargo build --release
   Compiling ackermann v0.1.0
    Finished release [optimized] target(s) in 1.13s
$ ./target/release/ackermann       
  0.000001: ackermann (0,0) is: 1
  0.000071: ackermann (0,1) is: 2
  0.000100: ackermann (0,2) is: 3
  0.000129: ackermann (0,3) is: 4
  0.000153: ackermann (0,4) is: 5
  0.000176: ackermann (0,5) is: 6
  0.000198: ackermann (1,0) is: 2
  0.000221: ackermann (1,1) is: 3
  0.000244: ackermann (1,2) is: 4
  0.000268: ackermann (1,3) is: 5
  0.000291: ackermann (1,4) is: 6
  0.000315: ackermann (1,5) is: 7
  0.000341: ackermann (2,0) is: 3
  0.000364: ackermann (2,1) is: 5
  0.000438: ackermann (2,2) is: 7
  0.000462: ackermann (2,3) is: 9
  0.000486: ackermann (2,4) is: 11
  0.000511: ackermann (2,5) is: 13
  0.000534: ackermann (3,0) is: 5
  0.000558: ackermann (3,1) is: 13
  0.000589: ackermann (3,2) is: 29
  0.000645: ackermann (3,3) is: 61
  0.000790: ackermann (3,4) is: 125
  0.001297: ackermann (3,5) is: 253
  0.001334: ackermann (4,0) is: 13
  3.255726: ackermann (4,1) is: 65533

thread 'main' has overflowed its stack
fatal runtime error: stack overflow
[1]    21280 abort      ./target/release/ackermann
```

Additionally, I implemented a cached version called ackerman_cahced that stores the final result of a calculation in a HashMap. Then each time the the ackerman_cached function is called, it checks the cache to see if it has already computed this particular solution. If there is a hit in the HashMap, the solution can be immediately returned. This proved even faster, resulting in a time to calculate up to ackerman(4,1) to under 0.2 seconds.

```bash
$ ./target/release/ackermann cached
  0.000014: ackermann_cached (0,0) is: 1
  0.001096: ackermann_cached (0,1) is: 2
  0.004333: ackermann_cached (0,2) is: 3
  0.005098: ackermann_cached (0,3) is: 4
  0.005961: ackermann_cached (0,4) is: 5
  0.006953: ackermann_cached (0,5) is: 6
  0.008266: ackermann_cached (1,0) is: 2
  0.009799: ackermann_cached (1,1) is: 3
  0.011313: ackermann_cached (1,2) is: 4
  0.012869: ackermann_cached (1,3) is: 5
  0.014799: ackermann_cached (1,4) is: 6
  0.016699: ackermann_cached (1,5) is: 7
  0.019845: ackermann_cached (2,0) is: 3
  0.023556: ackermann_cached (2,1) is: 5
  0.026703: ackermann_cached (2,2) is: 7
  0.029998: ackermann_cached (2,3) is: 9
  0.035325: ackermann_cached (2,4) is: 11
  0.041130: ackermann_cached (2,5) is: 13
  0.046748: ackermann_cached (3,0) is: 5
  0.053220: ackermann_cached (3,1) is: 13
  0.055987: ackermann_cached (3,2) is: 29
  0.061947: ackermann_cached (3,3) is: 61
  0.071651: ackermann_cached (3,4) is: 125
  0.085331: ackermann_cached (3,5) is: 253
  0.108437: ackermann_cached (4,0) is: 13
  0.175021: ackermann_cached (4,1) is: 65533

thread 'main' has overflowed its stack
fatal runtime error: stack overflow
[1]    21870 abort      ./target/release/ackermann cached
```

Additionally, by having a cache, we can see all the different values that needed to be calculated in order to get to ackermann(4,1). This is shown in `data/cache_4-1`.


## Run it yourself

1. You will need rust and cargo for your platform which you can get from https://www.rust-lang.org/tools/install
2. Clone this repository
3. There are two ways to run the code.
    1. Run in debug mode with `cargo run`
    2. Build a release for faster performance with `cargo build --release` and then `./target/release/ackermann`

To run the cached version, run with "cached" as the first argument

```bash
$ cargo run cached
 - or -
$ ./target/release/ackermann cached
```

You can also specify arbitrary arguments to try computing ackermann function for. Ex:

```bash
$ ./target/release/ackermann 2 125000 
  0.151636: ackermann_cached (2,125000) is: 250003
```
