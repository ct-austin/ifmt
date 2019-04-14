extern crate proc_macro;
extern crate regex;
#[macro_use]
extern crate lazy_static;
use proc_macro::TokenStream;
use regex::Regex;
use syn::parse;

fn consume_expr(s: &str) -> (&str, String) {
    lazy_static! {
        // bad hack regex to look for string/character literals
        // we don't want to count braces contained within them
        static ref LITERAL: Regex = Regex::new(
            concat!(r#"^('(?:\\[\s\S]|[^\\])+?'"#,
                    r##"|"(?:\\[\s\S]|[^\\])+?)""##)).unwrap();
        // have to handle raw strings separately due to no backrefs
        static ref RAW_STRING_START: Regex = Regex::new(
            r##"^r(#*)""##).unwrap();
    }

    let mut s = s;
    let mut expr = String::new();
    let mut brace_count = 1;
    while s.len() > 0 {
        let c = s.chars().next().unwrap();
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

                        s = &s[em.end()..];
                        expr.push_str(em.as_str());
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

#[proc_macro]
pub fn formati(s_tokens: TokenStream) -> TokenStream {
    lazy_static! {
        static ref SPEC: Regex =
            Regex::new(r":([\s\S]?[<^>])?([\+\-])?(#)?(0)?(\d+|\*)?(\.\d+)?(\?|\w+)?$").unwrap();
    }

    let s_tree: syn::Lit = parse(s_tokens).unwrap();

    let s = match s_tree {
        syn::Lit::Str(ls) => ls.value(),
        _ => panic!("f! string applied to non-string token"),
    };

    let mut format_lit = String::from("");
    let mut exprs = Vec::new();
    let mut s: &str = &s;
    while s.len() > 0 {
        let c = s.chars().next().unwrap();

        match c {
            '{' => {
                s = &s[1..];
                match s.chars().next() {
                    Some('{') => format_lit.push_str("{{"),
                    _ => {
                        let (new_s, expr) = consume_expr(s);
                        s = new_s;
                        let spec_match = SPEC.find(&expr);
                        match spec_match {
                            Some(m) => {
                                format_lit.push('{');
                                format_lit.push_str(m.as_str());
                                format_lit.push('}');
                                exprs.push(format!("({})", &expr[..m.start()]));
                            }
                            None => {
                                format_lit.push_str("{}");
                                exprs.push(expr);
                            }
                        }
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
    format!(
        "::std::format!({}, {})",
        proc_macro::Literal::string(&format_lit).to_string(),
        exprs.join(",")
    )
    .parse()
    .unwrap()
}
