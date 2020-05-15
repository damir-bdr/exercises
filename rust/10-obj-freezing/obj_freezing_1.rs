struct Wrapper {
    value: Box<i32>,
}

fn main() {
    let mut w = Wrapper { value: Box::new(9) };
    let r: &i32 = &w.value;
    w.value = Box::new(7);  // dangling reference
    println!("{}", r);
}
