struct S {
    value: i32
}

fn main() {
    let r: &i32;
    {
        let s = S { value: 9 };
        r = &s.value;   // <-- borrowed value does not live long enough
    }
    println!("{}", r);
}
