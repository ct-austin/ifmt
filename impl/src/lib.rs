extern crate proc_macro;
extern crate proc_macro2;
extern crate regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate quote;
use proc_macro2::TokenStream;
use proc_macro_hack::proc_macro_hack;
use regex::Regex;

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
        // TODO:
        // match here doubles the indentation
        // either way this might be nested a bit deep
        // probably should either refactor out clauses
        // or change back to if/else
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
                match lit_match {
                    Some(m) => {
                        s = &s[m.end()..];
                        expr.push_str(m.as_str());
                        // TODO this feels off
                        // should we just nest the second match?
                        continue;
                    }
                    None => (),
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

    panic!("unbalanced {");
}

fn extract_exprs(s_tokens: TokenStream) -> (String, TokenStream) {
    lazy_static! {
        static ref SPEC: Regex =
            Regex::new(r":([\s\S]?[<^>])?([\+\-])?(#)?(0)?(\d+|\*)?(\.\d+)?(\?|\w+)?$").unwrap();
    }

    if s_tokens.is_empty() {
        return (String::new(), TokenStream::new());
    }
    let s_tree: syn::Lit = syn::parse2(s_tokens).expect("expected string literal");

    let format_str = match s_tree {
        syn::Lit::Str(ls) => ls.value(),
        _ => panic!("expected string literal"),
    };

    let mut format_lit = String::from("");
    let mut exprs = TokenStream::new();
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
                        
                        // TODO: errors 'expecting <X> got }' don't show correctly
                        // the span is pointing at the last value in the macro invocation and not the whole thing (which is already wrong but not quite as wrong)
                        let tt = match expr.parse() {
                            Ok(x) => proc_macro2::TokenTree::Group(proc_macro2::Group::new(proc_macro2::Delimiter::Brace, x)),
                            Err(e) => {
                                let msg = format!("{:?}", e);
                                return (String::new(), quote! {
                                    compile_error!(#msg);
                                })
                            }
                        };

                        exprs.extend(std::iter::once(tt));
                        exprs.extend(std::iter::once(proc_macro2::TokenTree::Punct(proc_macro2::Punct::new(',', proc_macro2::Spacing::Alone))));
                    }
                }
            }
            '}' => {
                s = &s[1..];
                if s.chars().next() == Some('}') {
                    format_lit.push_str("}}");
                } else {
                    panic!("unmatched }");
                }
            }
            _ => format_lit.push(c),
        }
        s = &s[c.len_utf8()..];
    }

    (format_lit, exprs)
}

macro_rules! def_ifmt_macro {
    ($name:ident, $to_wrap:ident) => {
        #[proc_macro_hack]
        pub fn $name (tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
            let (s, e) = extract_exprs(tokens.into());
            let expanded = quote! {
                ::std::$to_wrap!(#s, #e)
            };

            expanded.into()
        }
    };
}

def_ifmt_macro!(iformat, format);

def_ifmt_macro!(iprint, print);

def_ifmt_macro!(iprintln, println);

def_ifmt_macro!(ieprint, eprint);

def_ifmt_macro!(ieprintln, eprintln);

def_ifmt_macro!(iformat_args, format_args);

def_ifmt_macro!(ipanic, panic);

#[proc_macro_hack]
pub fn iwrite(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use std::iter::FromIterator;
    let mut tokens = tokens.into_iter();
    // go until we hit a ,
    // probably should be done with macro_rules but oh well
    let writer = proc_macro2::TokenStream::from(
        proc_macro::TokenStream::from_iter(
            tokens.by_ref().take_while(|x| match x { proc_macro::TokenTree::Punct(c) => c.as_char() != ',', _ => true })));
    let n = match tokens.next() {
        Some(x) => x,
        None => return (quote! { ::std::write!(#writer); }).into()
    };
    let (s, e) = extract_exprs(proc_macro::TokenStream::from(n).into());
    let expanded = quote! {
        ::std::write!(#writer, #s, #e)
    };
    expanded.into()
}

#[proc_macro_hack]
pub fn iwriteln(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use std::iter::FromIterator;
    let mut tokens = tokens.into_iter();
    let writer = proc_macro2::TokenStream::from(
        proc_macro::TokenStream::from_iter(
            tokens.by_ref().take_while(|x| match x { proc_macro::TokenTree::Punct(c) => c.as_char() != ',', _ => true })));
    let n = match tokens.next() {
        Some(x) => x,
        None => return (quote! { ::std::writeln!(#writer) }).into()
    };
    let (s, e) = extract_exprs(proc_macro::TokenStream::from(n).into());
    let expanded = quote! {
        ::std::writeln!(#writer, #s, #e)
    };
    expanded.into()
}
