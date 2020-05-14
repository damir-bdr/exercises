fn main() {
    let x = 1;
    let r: &i32;
    {
        let y = 2;
        r = f(&x, &y);
        // r = f(&y, &x)
    }
    println!("{}", r);
}

fn f<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    x
}
