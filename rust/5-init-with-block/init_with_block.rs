fn f1() -> i32 {
    10
}

fn f2() -> i32 {
    20
}

fn main() {
    let v = {
        let v1 = f1();
        let v2 = f2();
        v1 + v2
    };
    println!("{}", v);
}
