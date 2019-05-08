/*! A small crate which brings inline string interpolation to rust's standard formatting macros.

# Examples
```rust
use ifmt::iprintln;
let four = 4;
iprintln!("four plus four is: {four + 4}");
// four plus four is: 8
iprintln!("here's a hex number: 0x{0xb0bi64 * 1321517i64 :x}");
// here's a hex number: 0xdeadbeef
iprintln!("here's a debugging value: {Some(four):?}");
// here's a debugging value: Some(4)
```

# Supported macros
```ignore
format!      -> iformat!
print!       -> iprint!
println!     -> iprintln!
eprint!      -> ieprint!
eprintln!    -> ieprintln!
write!       -> iwrite!
writeln!     -> iwriteln!
format_args! -> iformat_args!
```
*/

use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack(fake_call_site)]
pub use ifmt_impl::{
    iformat,
    iprint,
    iprintln,
    ieprint,
    ieprintln,
    iwrite,
    iwriteln,
    iformat_args,
};
