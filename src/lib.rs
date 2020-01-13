/*! A small crate which brings inline string interpolation to rust's standard formatting macros.

# Examples
```rust
use ifmt::iprintln;
let four = 4;
iprintln!("four plus four is: " four + 4);
// four plus four is: 8
iprintln!("here's a hex number: 0x" 0xb0bi64 * 1321517i64 ;x);
// here's a hex number: 0xdeadbeef
iprintln!("here's a debugging value: " Some(four);?);
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
panic!       -> ipanic!
format_args! -> iformat_args!
```
*/

use proc_macro_hack::proc_macro_hack;

/// Creates a String by interpolating inline expressions.
///
/// Works by expanding to `format!`.
///
/// ```
/// # let foo = Some(vec![1, 2, 3]);
/// # let bar = Some(vec![4, 5, 6]);
/// # let x = 6;
/// # let y = 20;
/// # use ifmt::iformat;
/// # assert!(
/// // Out-of-literal style (preferred)
/// iformat!("x plus y is " x + y ".") == format!("x plus y is {}.", x + y)
/// # &&
/// iformat!("foo: " foo;? ", bar: " bar;#?) == format!("foo: {:?}, bar: {:#?}", foo, bar)
/// # &&
///
/// // In-literal style (old)
/// iformat!("x plus y is {x + y}.") == format!("x plus y is {}.", x + y)
/// # &&
/// iformat!("foo: {foo:?}, bar: {bar:#?}") == format!("foo: {:?}, bar: {:#?}", foo, bar)
/// # );
/// ```
///
/// Out-of-literal format specs are roughly the same as in `std::fmt`, except:
///
/// * they are preceded by a `;` rather than a `:`
/// * format parameterization (for width/precision) is not yet supported, literals must be used
/// * since `3.4e`/`3.4E` (as in `format!("{:3.4e}", x)`) is invalid in rust (exponent is missing),
/// `3.4 e` (with a space) or `3.4s` must be used instead
/// * The fill character for padding must be a char literal:
/// `iformat!("padded: " inside_dashes;'-'^30)`
///
/// In-literal format specs are identical to those found in `std::fmt`, except that they also do not
/// support format parameterization.
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
