trait Say {
    fn say(&self);
}

impl Say for i32 {
    fn say(&self) {
        println!("hm... int-int? {}", self + 1);
    }
}

fn main() {
    92.say();

    let x = 93;
    x.say();
}
