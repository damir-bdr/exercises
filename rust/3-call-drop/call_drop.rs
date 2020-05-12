#[derive(Debug)]
enum En{
    A1,
    A2,
}

fn main() {
    let mut e = En::A1;
    e = En::A2;
    let e1 = &e;

    drop(e);    // <-- move out of `e` occurs here
    println!("{:?}", e1); // <-- borrow later used here
}
