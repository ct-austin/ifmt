#![feature(proc_macro_hygiene)]

#[cfg(test)]
mod tests {
    use ifmt::{iformat, iwrite, iwriteln};
    #[test]
    fn simple_subst() {
        let x = 5.3;
        assert_eq!(
            format!("hello this is {{ a test {}", x),
            iformat!("hello this is {{ a test {x}")
        );
        let y = "420";
        assert_eq!(format!("{}", y), iformat!("{y}"));
        assert_eq!(format!("{}{}", x, y), iformat!("{x}{y}"));
    }

    #[test]
    fn expr_subst() {
        let x = 5.3;
        let y = 29.0;
        assert_eq!(
            format!("atest {} {}", x * y, x + y),
            iformat!("atest {x*y} {x+y}")
        );
    }

    #[test]
    fn spec_subst() {
        let x = 5.3;
        assert_eq!(
            format!("hello this is {{ a test {:.5}", x),
            iformat!("hello this is {{ a test {x:.5}")
        );
        let y = "420";
        let z = Some("debugged");
        let k = 420.0;
        assert_eq!(format!("{:?}", y), iformat!("{y:?}"));
        assert_eq!(format!("{:?}{:#?}", z, z), iformat!("{z:?}{z:#?}"));
        assert_eq!(
            format!(
                "atest {:o} {:X} {:.4}",
                (x * -k) as i32,
                (x + k) as i32,
                x * k
            ),
            iformat!("atest {(x*-k) as i32:o} {(x+k) as i32:X} {x*k:.4}")
        );
    }

    #[test]
    fn internal_strings() {
        let x = "s1";
        let y = "s2";
        assert_eq!(
            format!("str {}", format!("{} {} {} {} {{", 3 + 4, 90*53, x, y)),
            iformat!(r#"str {format!("{} {} {} {} {{", 3 + 4, 90*53, x, y)}"#));
        assert_eq!(
            format!("str {} {}", r#""a thing }"#, "another thing"),
            iformat!(r##"str {r#""a thing }"#} {"another thing"}"##));
    }

    #[test]
    fn write() {
        use std::io::Write;
        let mut buffer = Vec::new();
        // empty
        iwriteln!(&mut buffer).unwrap();
        iwriteln!(&mut buffer, "a boring string").unwrap();
        iwriteln!(&mut buffer, "some math 0x{2 + 9:x}").unwrap();
        iwrite!(&mut buffer, "the {1+0}st end").unwrap();
        assert_eq!(&buffer[..], "\na boring string\nsome math 0xb\nthe 1st end".as_bytes());
    }

    #[test]
    fn internal_lifetimes() {
        let test = "a test";
        assert_eq!("dirty a test",
            iformat!("dirty { let x: &'static str = test; { let y: &'static str = x; y } }"));
        assert_eq!("dirty static str = test; { let y: &",
            iformat!(r#"dirty { let x = "static str = test; { let y: &"; x }"#));
    }

    #[test]
    fn internal_chars() {
        assert_eq!("an open brace: {",
            iformat!("an open brace: {'{'}"));
        assert_eq!("a close brace: }",
            iformat!("a close brace: {'}'}"));
        assert_eq!("an open brace: {",
            iformat!(r#"an open brace: {'\u{007b}'}"#));
        assert_eq!("a close brace: }",
            iformat!(r#"a close brace: {'\u{007D}'}"#));
        assert_eq!("dirty dan: '",
            iformat!(r#"dirty dan: {'\''; { '\'' }}"#));
        assert_eq!("pinhead: \"",
            iformat!(r#"pinhead: {'\"'; { '\"' }}"#));
        assert_eq!("gary: '",
            iformat!(r#"gary: {"\'"; { "\'" }}"#));
        assert_eq!("val kilmer: \"",
            iformat!(r#"val kilmer: {"\""; { "\"" }}"#));
        assert_eq!("lelouch: \"",
            iformat!(r#"lelouch: {'"'; { '"' }}"#));
        assert_eq!("gon: '",
            iformat!(r#"gon: {"'"; { "'" }}"#));
    }
}
