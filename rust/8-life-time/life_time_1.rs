fn main() {
    let x = 3;
    let r = &x;
    foo(r);
    println!("{}", r);
}

fn foo(r: &i32) {
    println!("{}", r)
}