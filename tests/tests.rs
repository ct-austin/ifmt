#![feature(proc_macro_hygiene)]

#[cfg(test)]
mod tests {
    use istr::formati;
    #[test]
    fn simple_subst() {
        let x = 5.3;
        assert_eq!(
            format!("hello this is {{ a test {}", x),
            formati!("hello this is {{ a test {x}")
        );
        let y = "420";
        assert_eq!(format!("{}", y), formati!("{y}"));
        assert_eq!(format!("{}{}", x, y), formati!("{x}{y}"));
    }

    #[test]
    fn expr_subst() {
        let x = 5.3;
        let y = 29.0;
        assert_eq!(
            format!("atest {} {}", x * y, x + y),
            formati!("atest {x*y} {x+y}")
        );
    }

    #[test]
    fn spec_subst() {
        let x = 5.3;
        assert_eq!(
            format!("hello this is {{ a test {:.5}", x),
            formati!("hello this is {{ a test {x:.5}")
        );
        let y = "420";
        let z = Some("debugged");
        let k = 420.0;
        assert_eq!(format!("{:?}", y), formati!("{y:?}"));
        assert_eq!(format!("{:?}{:#?}", z, z), formati!("{z:?}{z:#?}"));
        assert_eq!(
            format!(
                "atest {:o} {:X} {:.4}",
                (x * -k) as i32,
                (x + k) as i32,
                x * k
            ),
            formati!("atest {(x*-k) as i32:o} {(x+k) as i32:X} {x*k:.4}")
        );
    }
}
