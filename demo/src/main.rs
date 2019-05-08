fn main() {
    use ifmt::iprintln;
    let four = 4;
    iprintln!("four plus four is: {four + 4}");
    // four plus four is: 8
    iprintln!("here's a hex number: 0x{0xb0bi64 * 1321517i64 :x}");
    // here's a hex number: 0xdeadbeef
    iprintln!("here's a debugging value: {Some(four):?}");
    // here's a debugging value: Some(4)
}
