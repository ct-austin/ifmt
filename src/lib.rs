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

extern crate proc_macro;
extern crate regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate quote;
use proc_macro2::Span;
use regex::Regex;
use syn::{Error, Token};

fn consume_expr(s: &str) -> (&str, String) {
    lazy_static! {
        // bad hack regex to look for string/character literals
        // we don't want to count braces contained within them
        static ref LITERAL: Regex = Regex::new(
            concat!(r#"^('(?:\\[\s\S]+?|\\'|[^\\'])'"#,
                    r##"|"(?:\\[\s\S]|[^\\])+?")"##)).unwrap();
        // have to handle raw strings separately due to no backrefs
        static ref RAW_STRING_START: Regex = Regex::new(
            r##"^r(#*)""##).unwrap();
    }

    let mut s = s;
    let mut expr = String::new();
    let mut brace_count = 1;
    while let Some(c) = s.chars().next() {
        match c {
            '{' => {
                brace_count += 1;
                s = &s[1..];
                expr.push('{');
            }
            '}' => {
                brace_count -= 1;
                if brace_count == 0 {
                    return (s, expr);
                } else {
                    expr.push('}');
                    s = &s[1..];
                }
            }
            _ => {
                let lit_match = LITERAL.find(s);
                if let Some(m) = lit_match {
                    s = &s[m.end()..];
                    expr.push_str(m.as_str());
                    // TODO this feels off
                    // should we just nest the second match?
                    continue;
                }
                let raw_caps = RAW_STRING_START.captures(s);
                match raw_caps {
                    // lazy hack to deal with the lack of backreferences
                    // generate the tail end of the regex
                    Some(c) => {
                        let m = c.get(0).unwrap();
                        s = &s[m.end()..];
                        expr.push_str(m.as_str());

                        let cap = c.get(1).unwrap();
                        let hash_count = cap.end() - cap.start();
                        let end_r =
                            Regex::new(&format!(r##"^[\s\S]+?"#{{{}}}"##, hash_count)).unwrap();
                        let em = end_r.find(s).expect("unclosed internal raw string");

                        expr.push_str(em.as_str());
                        s = &s[em.end()..];
                    }
                    None => {
                        expr.push(c);
                        s = &s[c.len_utf8()..];
                    }
                }
            }
        }
    }

    panic!("{}", "unbalanced {");
}

macro_rules! def_ifmt_macro {
    ($(#[$meta:meta])* $name:ident, $to_wrap:ident) => {
        #[proc_macro]
        $(#[$meta])*
        pub fn $name(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
            use syn::parse_macro_input;
            let FormatContents { fmt, args } = parse_macro_input!(tokens as FormatContents);
            (quote! {
                ::std::$to_wrap!(#fmt, #(#args),*)
            })
            .into()
        }
    };
}

def_ifmt_macro!(
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
    iformat,
    format
);

def_ifmt_macro!(
    /// Print an [`iformat!`][fn@iformat]-ed string to standard out.
    print,
    print
);

def_ifmt_macro!(
    /// Print an [`iformat!`][fn@iformat]-ed string to standard out, followed by `\n`.
    iprintln,
    println
);

def_ifmt_macro!(
    /// Print an [`iformat!`][fn@iformat]-ed string to standard error.
    ieprint,
    eprint
);

def_ifmt_macro!(
    /// Print an [`iformat!`][fn@iformat]-ed string to standard error, followed by `\n`.
    ieprintln,
    eprintln
);

def_ifmt_macro!(
    /// Create a `fmt::Arguments` value a la `format_args!` with inlined expressions (using the same syntax as [`iformat!`][fn@iformat]).
    iformat_args,
    format_args
);

def_ifmt_macro!(
    /// Panic with an [`iformat!`][fn@iformat]-ed message.
    ipanic,
    panic
);

struct FormatSpec {
    spec: String,
}

fn parse_format_type(
    id: &str,
    span: Span,
    input: syn::parse::ParseStream,
) -> syn::parse::Result<String> {
    let mut out = String::new();
    if id == "x" || id == "X" {
        out.push_str(id);
        if input.peek(Token![?]) {
            input.parse::<Token![?]>()?;
            out.push('?');
        }
    } else if id == "o" || id == "p" || id == "b" || id == "e" || id == "E" {
        out.push_str(id);
    } else if id == "s" {
        out.push('e');
    } else if id == "S" {
        out.push('E');
    } else {
        return Err(Error::new(
            span,
            format!("{} is not a valid format type", id),
        ));
    }
    Ok(out)
}

fn parse_precision_type(input: syn::parse::ParseStream) -> syn::parse::Result<String> {
    // ['.' precision][type]
    // TODO make it so these can be identifiers/exprs as well? maybe in parens?
    let mut spec = String::new();

    let span = input.cursor().span();

    // ['.']
    if input.peek(Token![.]) {
        input.parse::<Token![.]>()?;
        spec.push('.');
        // precision
        let lit = input.parse::<syn::LitInt>()?;
        let lit_str = lit.to_string();
        if lit.suffix() != "" {
            spec.push_str(&lit_str[..lit_str.len() - lit.suffix().len()]);
            // [type]
            spec.push_str(&parse_format_type(lit.suffix(), span, input)?);
        } else {
            spec.push_str(&lit_str);
        }
    }

    // [type]
    if input.peek(Token![?]) {
        input.parse::<Token![?]>()?;
        spec.push('?');
    // [type]
    } else if input.peek(syn::Ident) {
        let id = input.parse::<syn::Ident>()?.to_string();
        spec.push_str(&parse_format_type(&id, span, input)?);
    }

    Ok(spec)
}

impl syn::parse::Parse for FormatSpec {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        /*
        format_spec := [[fill]align][sign]['#']['0'][width]['.' precision][type]
        fill := character
        align := '<' | '^' | '>'
        sign := '+' | '-'
        width := count
        precision := count | '*'
        type := identifier | '?' | ''
        count := parameter | integer
        parameter := argument '$'
        */
        input.parse::<Token![;]>()?;
        let mut spec = String::new();
        // [[fill]align]
        if input.peek(syn::LitChar) {
            let pad = input.parse::<syn::LitChar>().unwrap();
            spec.push(pad.value());
            let lookahead = input.lookahead1();
            if lookahead.peek(Token![<]) {
                input.parse::<Token![<]>()?;
                spec.push('<');
            } else if lookahead.peek(Token![>]) {
                input.parse::<Token![>]>()?;
                spec.push('>');
            } else if lookahead.peek(Token![^]) {
                input.parse::<Token![^]>()?;
                spec.push('^');
            } else {
                return Err(lookahead.error());
            }
        } else if input.peek(Token![<]) {
            input.parse::<Token![<]>()?;
            spec.push('<');
        } else if input.peek(Token![>]) {
            input.parse::<Token![>]>()?;
            spec.push('>');
        } else if input.peek(Token![^]) {
            input.parse::<Token![^]>()?;
            spec.push('^');
        }

        // [sign]
        if input.peek(Token![+]) {
            input.parse::<Token![+]>()?;
            spec.push('+');
        } else if input.peek(Token![-]) {
            input.parse::<Token![-]>()?;
            spec.push('-');
        }

        // ['#']
        if input.peek(Token![#]) {
            input.parse::<Token![#]>()?;
            spec.push('#');
        }

        // TODO width as an expression
        //  will require some reworking - has to be passed by name
        //  FormatSpec either has to generate an incomplete spec which is templated later
        //  or it needs information on which number expression it's on
        // ['0'][width]['.' precision][type]
        if input.peek(syn::LitFloat) {
            // ['0'][width]['.' precision][type]
            let span = input.cursor().span();
            let lit = input.parse::<syn::LitFloat>()?;
            let lit_str = lit.to_string();
            if lit.suffix() != "" {
                // ['0'][width]['.' precision]
                spec.push_str(&lit_str[..lit_str.len() - lit.suffix().len()]);
                // [type]
                spec.push_str(&parse_format_type(lit.suffix(), span, input)?);
            } else {
                // ['0'][width]['.' precision]
                spec.push_str(&lit_str);
                // [type]
                if input.peek(syn::Ident) {
                    let id = input.parse::<syn::Ident>()?.to_string();
                    spec.push_str(&parse_format_type(&id, span, input)?);
                }
            }
        } else if input.peek(syn::LitInt) {
            // ['0'][width][type]
            let span = input.cursor().span();
            let lit = input.parse::<syn::LitInt>()?;
            let lit_str = lit.to_string();
            if lit.suffix() != "" {
                spec.push_str(&lit_str[..lit_str.len() - lit.suffix().len()]);
                // [type]
                spec.push_str(&parse_format_type(lit.suffix(), span, input)?);
            } else {
                // ['.' precision][type]
                spec.push_str(&lit_str);
                spec.push_str(&parse_precision_type(input)?);
            }
        } else {
            spec.push_str(&parse_precision_type(input)?);
        }

        Ok(FormatSpec { spec })
    }
}

struct FormatContents {
    fmt: String,
    args: Vec<syn::Expr>,
}

impl FormatContents {
    fn parse_inlit(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        lazy_static! {
            static ref SPEC: Regex =
                Regex::new(r":([\s\S]?[<^>])?([\+\-])?(#)?(0)?(\d+|\*)?(\.\d+)?(\?|\w+)?$")
                    .unwrap();
        }
        let format_str = input.parse::<syn::LitStr>()?.value();

        let mut format_lit = String::from("");
        let mut exprs = vec![];
        let mut s: &str = &format_str;
        while let Some(c) = s.chars().next() {
            match c {
                '{' => {
                    s = &s[1..];
                    match s.chars().next() {
                        Some('{') => format_lit.push_str("{{"),
                        _ => {
                            let (new_s, mut expr) = consume_expr(s);
                            s = new_s;
                            let spec_match = SPEC.find(&expr);
                            match spec_match {
                                Some(m) => {
                                    format_lit.push('{');
                                    format_lit.push_str(m.as_str());
                                    format_lit.push('}');
                                    let si = m.start();

                                    expr.truncate(si);
                                }
                                None => {
                                    format_lit.push_str("{}");
                                }
                            }

                            exprs.push(syn::parse_str::<syn::Expr>(&expr)?);
                        }
                    }
                }
                '}' => {
                    s = &s[1..];
                    if s.starts_with('}') {
                        format_lit.push_str("}}");
                    } else {
                        panic!("{}", "unmatched }");
                    }
                }
                _ => format_lit.push(c),
            }
            s = &s[c.len_utf8()..];
        }

        Ok(FormatContents {
            fmt: format_lit,
            args: exprs,
        })
    }

    fn parse_outlit(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let mut expect_expr = true; // state machine
        let mut fmt = String::new();
        let mut args = vec![];

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(syn::LitStr) {
                let ls = input.parse::<syn::LitStr>().unwrap();
                fmt += &ls.value().replace("{", "{{").replace("}", "}}");
                expect_expr = true;
            } else if expect_expr {
                // Bug in proc_macro(fake_call_site): iformat!("x:" 2 + );'s
                // "unexpected end of input" error span points at the first token in the macro
                // invocation rather than the end.
                // not going to try to work around - probably not a common mistake and
                // if proc_macro becomes a pass-through as mentioned by dtolnay
                // (or, barring that, if I remove it as a dependency) the issue goes away
                let expr = input.parse::<syn::Expr>()?;
                args.push(expr);
                fmt.push('{');
                if input.peek(Token![;]) {
                    let spec = &input.parse::<FormatSpec>()?.spec;
                    if !spec.is_empty() {
                        fmt.push(':');
                        fmt.push_str(spec);
                    }
                }
                fmt.push('}');
                expect_expr = false;
            } else {
                return Err(lookahead.error());
            }
        }
        Ok(FormatContents { fmt, args })
    }
}

impl syn::parse::Parse for FormatContents {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        // note: eventually one of these branches should go away
        let forked = input.fork();
        if forked.peek(syn::LitStr) && {
            forked.parse::<syn::LitStr>()?;
            forked.is_empty()
        } {
            FormatContents::parse_inlit(input)
        } else {
            FormatContents::parse_outlit(input)
        }
    }
}

struct WriteFormat {
    buf: syn::Expr,
    fmt_contents: FormatContents,
}

impl syn::parse::Parse for WriteFormat {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        Ok(WriteFormat {
            buf: input.parse::<syn::Expr>()?,
            fmt_contents: if !input.is_empty() {
                input.parse::<Token![,]>()?;
                input.parse::<FormatContents>()?
            } else {
                FormatContents {
                    fmt: "".to_string(),
                    args: vec![],
                }
            },
        })
    }
}

/// Print an [`iformat!`][fn@iformat]-ed string to the given buffer.
#[proc_macro]
pub fn iwrite(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use syn::parse_macro_input;
    let WriteFormat {
        buf,
        fmt_contents: FormatContents { fmt, args },
    } = parse_macro_input!(tokens as WriteFormat);
    (quote! {
        ::std::write!(#buf, #fmt, #(#args),*)
    })
    .into()
}

/// Print an [`iformat!`][fn@iformat]-ed string to the given buffer, followed by `\n`.
#[proc_macro]
pub fn iwriteln(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use syn::parse_macro_input;
    let WriteFormat {
        buf,
        fmt_contents: FormatContents { fmt, args },
    } = parse_macro_input!(tokens as WriteFormat);
    (quote! {
        ::std::writeln!(#buf, #fmt, #(#args),*)
    })
    .into()
}
