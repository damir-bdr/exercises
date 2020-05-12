fn main() {
    let op: Option<i32> = Some(5);

    if let Some(x) = op {
        println!("{}", x);
    }
}
