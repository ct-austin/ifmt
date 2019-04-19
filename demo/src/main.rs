#![feature(proc_macro_hygiene)]

fn main() {
    use ifmt::iprintln;
    let four = 4;
    iprintln!("four plus four is: {four + 4}");
    // four plus four is: 8
    iprintln!("here's a hex number: 0x{7+7 :X}");
    // here's a hex number: 0xE
    iprintln!("here's a debugging value: {Some(4):?}");
    // here's a debugging value: Some(4)
}
