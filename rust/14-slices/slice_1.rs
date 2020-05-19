fn main() {
    let a = [0.0, 3.14, -8.79];    
    let a_slice = &a[..4];  // panicked at 'index 4 out of range for slice

    println!("{:?}", a_slice);
}