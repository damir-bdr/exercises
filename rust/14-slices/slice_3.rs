fn main() {
    let s = "🥝🍅";

    // let s_slice = &s[0..1]; // panicked at 'byte index 1 is not a char boundary; it is inside '🥝' (bytes 0..4
    let s_slice = &s[0..4];
    println!("{}", s_slice);
}
