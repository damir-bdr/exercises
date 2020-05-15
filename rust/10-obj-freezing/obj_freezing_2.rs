#[derive(Debug)]
struct Wrapper {
    value: Box<i32>,
}

fn main() {
    let mut w = Wrapper { value: Box::new(9) };
    let r: &i32 = &w.value;
    // f(&mut w);
    println!("{:?}", r);
}

fn f(w: &mut Wrapper) {
    w.value = Box::new(7);
}
