fn main() {
    let v = vec![4, 20, 12, 8, 6];
    let mut itr = v.iter();

    while let Some(v) = itr.next() {
        println!("{}", v)
    }
}