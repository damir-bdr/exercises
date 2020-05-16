fn main() {
    let s = String::from("book");
    println!("I have one {}, you have two {}", s, pluralize(s.clone()));
}

fn pluralize(s: String) -> String {
    s + "s"
}
