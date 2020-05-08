fn main() {
    let r: &i32;
    let x = Box::new(92);
    {
        r = &x;
    }   
    use_x(r);
}

fn use_x(x: &i32) {
    println!("{}", x)
}
