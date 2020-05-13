fn main() {
    let x = 3;
    let r: &i32;
    {
        r = &x;
    }
    println!("{}", r);
}
