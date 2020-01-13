# ifmt
A small crate which brings inline string interpolation to rust's standard formatting macros.

## Getting started
To use ifmt in your project, add
```toml
[dependencies]
ifmt = "0.3.0"
```
to your Cargo.toml.

## Examples
```rust
let four = 4;
iprintln!("four plus four is: " four + 4);
// four plus four is: 8
iprintln!("here's a hex number: 0x" 0xb0bi64 * 1321517i64 ;x);
// here's a hex number: 0xdeadbeef
iprintln!("here's a debugging value: " Some(four);?);
// here's a debugging value: Some(4)
```

## Supported macros
```
format!      -> iformat!
print!       -> iprint!
println!     -> iprintln!
eprint!      -> ieprint!
eprintln!    -> ieprintln!
write!       -> iwrite!
writeln!     -> iwriteln!
panic!       -> ipanic!
format_args! -> iformat_args!
```

## License
This project is licensed under the MIT license or the Apache 2.0 license at your option.
