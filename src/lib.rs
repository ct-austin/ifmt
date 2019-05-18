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


/// Creates a String by interpolating inlined expressions.
/// Takes one argument, which must be a string literal.
///
/// Works by expanding to `format!`:
///
/// `iformat!("two plus two: {2 + 2}") -> format!("two plus two: {}", {2 + 2})`
///
/// `iformat!("foo: {foo:?}") -> format!("foo: {:?}", {foo})`
#[proc_macro_hack(fake_call_site)]
pub use ifmt_impl::iformat;

/// Print an [`iformat!`][iformat]-ed string to standard out.
#[proc_macro_hack(fake_call_site)]
pub use ifmt_impl::iprint;

/// Print an [`iformat!`][iformat]-ed string to standard out, followed by `\n`.
#[proc_macro_hack(fake_call_site)]
pub use ifmt_impl::iprintln;

/// Print an [`iformat!`][iformat]-ed string to standard error.
#[proc_macro_hack(fake_call_site)]
pub use ifmt_impl::ieprint;

/// Print an [`iformat!`][iformat]-ed string to standard error, followed by `\n`.
#[proc_macro_hack(fake_call_site)]
pub use ifmt_impl::ieprintln;

/// Print an [`iformat!`][iformat]-ed string to the given buffer.
#[proc_macro_hack(fake_call_site)]
pub use ifmt_impl::iwrite;

/// Print an [`iformat!`][iformat]-ed string to the given buffer, followed by `\n`.
#[proc_macro_hack(fake_call_site)]
pub use ifmt_impl::iwriteln;

/// Create a `fmt::Arguments` value a la `format_args!` with inlined expressions (using the same syntax as [`iformat!`][iformat]).
#[proc_macro_hack(fake_call_site)]
pub use ifmt_impl::iformat_args;

/// Panic with an [`iformat!`][iformat]-ed message.
#[proc_macro_hack(fake_call_site)]
pub use ifmt_impl::ipanic;

