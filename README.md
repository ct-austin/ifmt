# ifmt
A small crate which provides expression-interpolating versions of rust's standard formatting macros.

## Usage
### Nightly
Since procedural macros currently cannot be used in expression position on stable, this crate requires you to use nightly rust and enable `#![feature(proc_macro_hygiene)]`.

### Examples
```rust
let four = 4;
iprintln!("four plus four is: {four + 4}");
// four plus four is: 8
iprintln!("here's a hex number: 0x{7+7 :X}");
// here's a hex number: 0xE
iprintln!("here's a debugging value: {Some(4):?}");
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
format_args! -> iformat_args!

```
