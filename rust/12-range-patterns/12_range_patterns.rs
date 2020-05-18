fn main() {
    // let x = 128u8;
    let x = 129u8;
    let s = match x {
        0 ..= 128 => "small",
        129 ..= 255 => "big",
    };

    println!("{}", s);
}
