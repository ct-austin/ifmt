#[cfg(test)]
mod outlittests {
    use ifmt::{iformat, ipanic, iwrite, iwriteln};
    #[test]
    fn simple_subst() {
        let x = 5.3;
        assert_eq!(
            format!("hello this is {{ a test {}", x),
            iformat!("hello this is { a test " x)
        );
        let y = "420";
        assert_eq!(format!("{}", y), iformat!(y));
        assert_eq!(format!("{}{}", x, y), iformat!(x "" y));
    }

    #[test]
    fn expr_subst() {
        let x = 5.3;
        let y = 29.0;
        assert_eq!(
            format!("atest {} {}", x * y, x + y),
            iformat!("atest " x*y " " x+y)
        );
    }

    #[test]
    fn spec_subst() {
        let x = 5.3;
        assert_eq!(
            format!("hello this is {{ a test {:.5}", x),
            iformat!("hello this is { a test " x;.5)
        );
        let y = "420";
        let z = Some("debugged");
        let k = 420.0;
        assert_eq!(format!("{:?}", y), iformat!(y;?));
        assert_eq!(format!("{:?}{:#?}", z, z), iformat!(z;? "" z;#?));
        assert_eq!(
            format!(
                "atest {:o} {:X} {:.4}",
                (x * -k) as i32,
                (x + k) as i32,
                x * k
            ),
            iformat!("atest " (x*-k) as i32;o " " (x+k) as i32;X " " x*k;.4)
        );

        assert_eq!(format!("{:+4}", 420),
                   iformat!(420;+4));
        assert_eq!(format!("{:-4}", 420),
                   iformat!(420;-4));
        assert_eq!(format!("{:>10.4}", "BBBBBBBBBBBBBBBBBBBB"),
                   iformat!(("BBBBBBBBBBBBBBBBBBBB");>10 .4));
        assert_eq!(format!("{:0>5}", "B"),
                   iformat!(("B");'0'>5));
        assert_eq!(format!("{:L>5}", "B"),
                   iformat!(("B");'L'>5));
        assert_eq!(format!("{:.3}", 3.1415e9f32),
                   iformat!(3.1415e9f32;.3));
        assert_eq!(format!("{:11.3}", 3.1415e9f32),
                   iformat!(3.1415e9f32;11 .3));
        assert_eq!(format!("{:11.3}", 3.1415e9f32),
                   iformat!(3.1415e9f32;11.3));
        assert_eq!(format!("{:+11.3}", 3.1415e9f32),
                   iformat!(3.1415e9f32;+11 .3));
        assert_eq!(format!("{:+11.3}", 3.1415e9f32),
                   iformat!(3.1415e9f32;+11.3));
        assert_eq!(format!("{:+11.3e}", 3.1415e9f32),
                   iformat!(3.1415e9f32;+11 .3 e));
        assert_eq!(format!("{:+11.3e}", 3.1415e9f32),
                   iformat!(3.1415e9f32;+11 .3s));
        assert_eq!(format!("{:+11.3E}", 3.1415e9f32),
                   iformat!(3.1415e9f32;+11.3S));
        assert_eq!(format!("{:+11.3e}", 3.1415e9f32),
                   iformat!(3.1415e9f32;+11.3 e));
        assert_eq!(format!("{:+11.3E}", 3.1415e9f32),
                   iformat!(3.1415e9f32;+11 .3 E));
        assert_eq!(format!("{:<6}", 4.20),
                   iformat!(4.20;<6));
        assert_eq!(format!("{:06}", 4.20),
                   iformat!(4.20;06));
        assert_eq!(format!("{:^06}", 4.20),
                   iformat!(4.20;^06));
        assert_eq!(format!("{:+06}", 4.20),
                   iformat!(4.20;+06));
        assert_eq!(format!("{:<+06}", 4.20),
                   iformat!(4.20;<+06));
        assert_eq!(format!("{:#06x}", 420),
                   iformat!(420;#06x));
        assert_eq!(format!("{:>#06x}", 420),
                   iformat!(420;>#06x));
        assert_eq!(format!("{:^#06x}", 420),
                   iformat!(420;^#06 x));
        assert_eq!(format!("{:<#06x}", 420),
                   iformat!(420;<#06 x));
    }

     #[test]
     fn write() {
         use std::io::Write;
         let mut buffer = Vec::new();
         // empty
         iwriteln!(&mut buffer).unwrap();
         iwriteln!(&mut buffer, "a boring string").unwrap();
         iwriteln!(&mut buffer, "some math 0x" 2 + 9;x).unwrap();
         iwrite!(&mut buffer, "the " 1+0 "st end").unwrap();
         println!("{:?}", String::from_utf8(buffer[..].to_vec()).unwrap());
         assert_eq!(
             &buffer[..],
             "\na boring string\nsome math 0xb\nthe 1st end".as_bytes()
         );
     }

    #[test]
    fn internal_lifetimes() {
        let test = "a test";
        assert_eq!(
            "dirty a test",
            iformat!("dirty " { let x: &'static str = test; { let y: &'static str = x; y } })
        );
        assert_eq!(
            "dirty static str = test; { let y: &",
            iformat!(r#"dirty "# { let x = "static str = test; { let y: &"; x })
        );
    }

    #[test]
    fn internal_chars() {
        assert_eq!("an open brace: {", iformat!("an open brace: " '{'));
        assert_eq!("a close brace: }", iformat!("a close brace: " '}'));
        assert_eq!("an open brace: {", iformat!("an open brace: " '\u{007b}'));
        assert_eq!(
            "a close brace: }",
            iformat!(r#"a close brace: "# '\u{007D}')
        );
        assert_eq!("dirty dan: '", iformat!("dirty dan: " { '\''; { '\'' }}));
        assert_eq!("pinhead: \"", iformat!("pinhead: " {'\"'; { '\"' }}));
        assert_eq!("gary: '", iformat!("gary: " {"\'"; { "\'" }}));
        assert_eq!("val kilmer: \"", iformat!("val kilmer: " {"\""; { "\"" }}));
        assert_eq!("lelouch: \"", iformat!("lelouch: " {'"'; { '"' }}));
        assert_eq!("gon: '", iformat!("gon: " {"'"; { "'" }}));
    }

    #[test]
    fn borrow() {
        let s = String::from("asdf");
        let v = vec![s];
        iformat!("borrow: " v[0]);
    }

    #[test]
    #[should_panic(expected = "2B || !2b")]
    fn panic() {
        let tobe = 0x2B;
        ipanic!(tobe;X " || !" tobe;x);
    }
}
