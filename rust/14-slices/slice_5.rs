fn main() {
    let s = String::from("book");
    foo(&s);
    foo(&s[1..]);

    bar(&s);
}

fn foo(s: &str) {
    println!("foo: {}", s);        
}

fn bar(s: &String) {
    println!("bar: {}", s);        
}