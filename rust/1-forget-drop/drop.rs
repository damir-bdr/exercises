use std::mem;

struct A{
}

impl Drop for A {
    fn drop(&mut self) {
        println!("A is dropping.")
    }    
}

fn main() {
    let a = A{};
    mem::forget(a);
}