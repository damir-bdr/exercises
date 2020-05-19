trait Say {
    fn say(&self);
}

struct Cat;
impl Say for Cat {
    fn say(&self) { println!("meow!") }
}

struct Dog;
impl Say for Dog {
    fn say(&self) { println!("woof!") }
}

fn say_twice<T: Say>(t: &T) {
    t.say();
    t.say();
}

fn main(){
    let dog = Dog;
    say_twice(&dog);

    let cat = Cat;
    say_twice(&cat);
}
